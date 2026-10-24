//! Admin recovery through fakes (S3 AC-12, auth policy §9): the admin's factors, the code to the
//! new e-mail, the 72-h hold, cancel, the alert after a cancel and the effect.

use chrono::Duration;
use identity_enterprise::channel_change::{ChangeKind, ChangeStatus, VerificationRef, VerifiedBy};
use identity_enterprise::otp::{CodeRole, DeviceClass, Purpose, StepUpAction};
use identity_enterprise::user::User;
use shared_kernel::Clock;
use uuid::Uuid;

use crate::changes::{
    Actor, AuditAction, ChangeAudit, HostReport, IdentityEvent, Notice, OperatorOutcome,
    OperatorStep, Trail,
};
use crate::error::IdentityError;
use crate::ports::{CodeMessage, SecurityAlert};
use crate::testing::World;
use crate::tests::{add_user, ctx, login, login_on, meta, signed_in};
use crate::usecases::admin_recovery::*;
use crate::usecases::channel_change::{
    ApplyDueChannelHolds, CancelChannelChange, VerifyChannelChangeCode,
    VerifyChannelChangeCodeCommand,
};
use crate::usecases::codes;
use crate::usecases::devices::ForgetDevice;
use crate::usecases::signin::{ResendCode, ResendCodeCommand, SignOutEverywhere};
use crate::usecases::step_up::{RequestStepUp, RequestStepUpCommand};

const ADMIN: &str = "root@example.com";
const USER: &str = "ana@example.com";
const NEW: &str = "ana.new@example.com";

async fn resolve(
    w: &World,
    admin: Option<&str>,
    break_glass: bool,
    user: &str,
) -> Result<(Operator, User), RecoveryError> {
    ResolveOperator::new(w.deps.clone())
        .handle(ResolveOperatorCommand {
            admin_email: admin.map(str::to_owned),
            break_glass,
            user: UserRef::Email(user.into()),
            host: wrapper(),
        })
        .await
}

/// A fresh admin step-up for `target`, after the per-identifier cooldown.
async fn step_up(w: &World, op: &Operator, admin_email: &str, target: Uuid) -> StepUpProof {
    w.clock.advance(Duration::seconds(61));
    let ch = RequestOperatorStepUp::new(w.deps.clone())
        .handle(op, target)
        .await
        .unwrap();
    StepUpProof {
        challenge_id: ch.challenge_id.expose().to_owned(),
        code: w.delivery.last_code_to(admin_email).unwrap(),
    }
}

async fn start(
    w: &World,
    op: &Operator,
    target: &User,
    proof: Option<StepUpProof>,
) -> Result<RecoveryView, RecoveryError> {
    StartAdminRecovery::new(w.deps.clone())
        .handle(
            op,
            target,
            StartRecoveryCommand {
                new_email: NEW.into(),
                verified_by: VerifiedBy::VideoCall,
                verification_ref: VerificationRef::parse("ticket 4521").unwrap(),
                step_up: proof,
            },
        )
        .await
}

async fn confirm(
    w: &World,
    op: &Operator,
    target: &User,
    view: &RecoveryView,
    code: &str,
    proof: Option<StepUpProof>,
) -> Result<RecoveryView, RecoveryError> {
    ConfirmAdminRecovery::new(w.deps.clone())
        .handle(
            op,
            target,
            ConfirmRecoveryCommand {
                change_id: view.change_id,
                code: code.into(),
                step_up: proof,
            },
        )
        .await
}

/// Admin, user with two sessions (one on an old device) and the resolved operator.
async fn setup(w: &World) -> (Operator, User, String) {
    add_user(w, ADMIN, true);
    let (_, cookie, _) = signed_in(w, USER, false).await;
    let (op, target) = resolve(w, Some(ADMIN), false, USER).await.unwrap();
    (op, target, cookie)
}

/// Started and confirmed: the recovery is on hold.
async fn held(w: &World, op: &Operator, target: &User) -> RecoveryView {
    let p = step_up(w, op, ADMIN, target.id.as_uuid()).await;
    let view = start(w, op, target, Some(p)).await.unwrap();
    let p = step_up(w, op, ADMIN, view.change_id.as_uuid()).await;
    let code = w.delivery.last_code_to(NEW).unwrap();
    confirm(w, op, target, &view, &code, Some(p)).await.unwrap()
}

fn add_source(w: &World, user: shared_kernel::UserId, name: &str) {
    use identity_enterprise::source::{DisplayName, Source, SourceType};
    let source = Source {
        id: identity_enterprise::ids::SourceId::new_v7(),
        user_id: user,
        source_type: SourceType::TradingView,
        name: DisplayName::parse(name).unwrap(),
        created_at: w.clock.now(),
        revoked_at: None,
        key_id: identity_enterprise::tokens::KeyId::parse("AbCdEf123456").unwrap(),
        rotated_at: None,
    };
    w.store
        .state
        .lock()
        .unwrap()
        .sources
        .insert(source.id, (source, Vec::new()));
}

/// A connection, paired to an NT8 instance or still awaiting pairing.
fn add_connection(w: &World, user: shared_kernel::UserId, name: &str, paired: bool) {
    use identity_enterprise::connection::{Connection, ConnectionStatus, PairedInstance};
    let now = w.clock.now();
    let c = Connection {
        id: identity_enterprise::ids::ConnectionId::new_v7(),
        user_id: user,
        name: identity_enterprise::source::DisplayName::parse(name).unwrap(),
        created_at: now,
        instance: paired.then(|| PairedInstance {
            instance_id: Uuid::now_v7(),
            instance_name: name.into(),
            adapter: crate::tests::adapter("1.0.0"),
            paired_at: now,
        }),
        status: if paired {
            ConnectionStatus::Disconnected
        } else {
            ConnectionStatus::AwaitingPairing
        },
        status_changed_at: now,
        last_seen_at: None,
        unpaired_at: None,
    };
    w.store.state.lock().unwrap().connections.insert(c.id, c);
}

/// What the host wrapper reports for the tests' operator.
fn wrapper() -> HostReport {
    HostReport::parse(
        Some("ops"),
        Some("203.0.113.5 51234 10.0.0.2 22"),
        Some(&format!("SHA256:{}", "A".repeat(43))),
    )
}

/// Every `identity.operator_recovery` record committed, with the writes of its change set.
fn operator_events(w: &World) -> Vec<(crate::changes::OperatorRecord, usize)> {
    w.store
        .committed()
        .into_iter()
        .flat_map(|c| {
            let writes = c.writes.len();
            c.events.into_iter().filter_map(move |e| match e {
                IdentityEvent::OperatorRecovery(r) => Some((r, writes)),
                _ => None,
            })
        })
        .collect()
}

fn records(w: &World) -> Vec<crate::changes::AuditRecord> {
    w.store
        .committed()
        .into_iter()
        .flat_map(|c| c.audit)
        .collect()
}

fn notices(w: &World) -> Vec<Notice> {
    w.store
        .committed()
        .into_iter()
        .flat_map(|c| c.events)
        .filter_map(|e| match e {
            IdentityEvent::NoticeRequested { notice, .. } => Some(notice),
            _ => None,
        })
        .collect()
}

#[tokio::test]
async fn recovery_waits_72_hours_then_ends_sessions_and_revokes_devices() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    let fresh = login_on(&w, USER, Duration::days(30)).await;
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    let view = start(&w, &op, &target, Some(p)).await.unwrap();
    assert_eq!(view.kind, ChangeKind::AdminRecovery);
    assert_eq!(view.status, ChangeStatus::AwaitingCodes);
    assert_eq!(view.masked_new_value, "a***@e***.com");
    let job = w.delivery.jobs().into_iter().last().unwrap();
    assert_eq!(job.delivery.destination.normalized(), NEW);
    assert!(matches!(job.delivery.message, CodeMessage::AdminRecovery));
    // The user's session sees the recovery without any code to confirm.
    let c = ctx(&w, &cookie).await.unwrap();
    let pending = crate::usecases::me::GetMySecurity::new(w.deps.clone())
        .handle(&c)
        .await
        .unwrap()
        .pending_channel_change
        .unwrap();
    assert_eq!(pending.kind, ChangeKind::AdminRecovery);
    assert!(pending.codes.is_empty());

    let p = step_up(&w, &op, ADMIN, view.change_id.as_uuid()).await;
    let code = w.delivery.last_code_to(NEW).unwrap();
    let held = confirm(&w, &op, &target, &view, &code, Some(p))
        .await
        .unwrap();
    assert_eq!(held.status, ChangeStatus::OnHold);
    let hold_until = held.hold_until.unwrap();
    assert_eq!(hold_until, w.clock.now() + Duration::hours(72));
    assert!(notices(&w).iter().any(|n| matches!(
        n,
        Notice::ChannelChangeOnHold { to, .. } if to.normalized() == USER
    )));
    assert_eq!(w.store.user_by_email(USER).unwrap().id, target.id);

    w.clock.advance(Duration::hours(71));
    let holds = ApplyDueChannelHolds::new(w.deps.clone());
    assert_eq!(holds.handle().await.unwrap(), 0);
    w.clock.advance(Duration::hours(1));
    assert_eq!(holds.handle().await.unwrap(), 1);
    assert_eq!(w.store.user_by_email(NEW).unwrap().id, target.id);
    assert!(ctx(&w, &cookie).await.is_err());
    assert!(ctx(&w, &fresh).await.is_err());
    assert!(
        w.store
            .state
            .lock()
            .unwrap()
            .known_devices
            .iter()
            .filter(|d| d.user_id == target.id)
            .all(|d| d.revoked_at.is_some())
    );
    // Esquadro: the old e-mail learns only the fact; the new one gets what to review.
    let ns = notices(&w);
    assert!(ns.iter().any(|n| matches!(
        n,
        Notice::RecoveredByAdmin { to, masked_new_value, .. }
            if to.normalized() == USER && masked_new_value == "a***@e***.com"
    )));
    assert!(ns.iter().any(|n| matches!(
        n,
        Notice::AccountRecovered { to, sources: 0, pairings: 0 } if to.normalized() == NEW
    )));
    assert!(!ns.iter().any(|n| matches!(
        n,
        Notice::AccountRecovered { to, .. } if to.normalized() == USER
    )));
    assert!(
        !ns.iter()
            .any(|n| matches!(n, Notice::ChannelChanged { .. }))
    );
}

#[tokio::test]
async fn the_new_email_is_told_how_many_sources_and_pairings_stayed() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    let c = ctx(&w, &cookie).await.unwrap();
    for name in ["Main", "Backup"] {
        add_source(&w, target.id, name);
    }
    add_connection(&w, target.id, "PC", true);
    add_connection(&w, target.id, "Laptop", false);
    drop(c);
    held(&w, &op, &target).await;
    w.clock.advance(Duration::hours(72));
    ApplyDueChannelHolds::new(w.deps.clone())
        .handle()
        .await
        .unwrap();
    assert!(notices(&w).iter().any(|n| matches!(
        n,
        Notice::AccountRecovered { to, sources: 2, pairings: 1 } if to.normalized() == NEW
    )));
}

#[tokio::test]
async fn break_glass_raises_an_alert_on_every_committed_action() {
    let w = World::new();
    add_user(&w, ADMIN, true);
    let (op, me) = resolve(&w, None, true, ADMIN).await.unwrap();
    let breaks = |w: &World| {
        w.alerts
            .0
            .lock()
            .unwrap()
            .iter()
            .filter(|a| matches!(a, SecurityAlert::AdminRecoveryBreakGlass { user_id, .. } if *user_id == me.id))
            .count()
    };
    let view = start(&w, &op, &me, None).await.unwrap();
    assert_eq!(breaks(&w), 1);
    // A failed confirm raises nothing.
    assert!(
        confirm(&w, &op, &me, &view, "AAAA-0000", None)
            .await
            .is_err()
    );
    assert_eq!(breaks(&w), 1);
    let code = w.delivery.last_code_to(NEW).unwrap();
    confirm(&w, &op, &me, &view, &code, None).await.unwrap();
    assert_eq!(breaks(&w), 2);
    CancelAdminRecovery::new(w.deps.clone())
        .handle(&op, &me, view.change_id, None)
        .await
        .unwrap();
    assert_eq!(breaks(&w), 3);
}

#[tokio::test]
async fn the_normal_path_raises_no_break_glass_alert_and_a_refused_start_no_cancel_alert() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    let view = held(&w, &op, &target).await;
    let c = ctx(&w, &cookie).await.unwrap();
    CancelChannelChange::new(w.deps.clone())
        .handle(&c, view.change_id, &meta())
        .await
        .unwrap();
    // Refused (no admin code): the repeat alert waits for a committed request.
    assert!(start(&w, &op, &target, None).await.is_err());
    let alerts = w.alerts.0.lock().unwrap().clone();
    assert!(alerts.iter().all(|a| !matches!(
        a,
        SecurityAlert::AdminRecoveryBreakGlass { .. }
            | SecurityAlert::ChannelChangeAfterCancel { .. }
    )));
}

#[tokio::test]
async fn forgetting_a_device_does_not_cancel_the_hold() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    held(&w, &op, &target).await;
    let c = ctx(&w, &cookie).await.unwrap();
    ForgetDevice::new(w.deps.clone())
        .handle(&c, c.session.device_id, &meta())
        .await
        .unwrap();
    assert_eq!(
        w.store.state.lock().unwrap().changes[0].status,
        ChangeStatus::OnHold
    );
}

#[tokio::test]
async fn an_expired_admin_code_is_refused() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    w.clock.advance(Duration::minutes(10));
    assert!(matches!(
        start(&w, &op, &target, Some(p)).await,
        Err(RecoveryError::Identity(IdentityError::CodeInvalid))
    ));
    assert!(w.store.state.lock().unwrap().changes.is_empty());
}

#[tokio::test]
async fn no_session_can_resend_a_recovery_or_admin_code() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    start(&w, &op, &target, Some(p.clone())).await.unwrap();
    let c = ctx(&w, &cookie).await.unwrap();
    let admin_c = c_admin(&w).await;
    let row = w
        .store
        .one_time_codes()
        .into_iter()
        .find(|c| c.role == Some(CodeRole::NewValue))
        .unwrap();
    let challenge = codes::seal(
        &w.deps,
        row.id,
        Purpose::ChannelChange,
        &codes::identifier(&w.deps, NEW),
        DeviceClass::Known,
        row.state.created_at,
    );
    let sends = w.delivery.jobs().len();
    w.clock.advance(Duration::seconds(61));
    for (who, ch) in [
        (&c, challenge.expose().to_owned()),
        (&admin_c, p.challenge_id.clone()),
    ] {
        let _ = ResendCode::new(w.deps.clone())
            .handle(
                Some(who),
                ResendCodeCommand {
                    challenge_id: ch,
                    meta: meta(),
                },
            )
            .await;
    }
    assert_eq!(w.delivery.jobs().len(), sends, "nothing was resent");
}

#[tokio::test]
async fn the_target_trail_names_platform_admin_and_the_admin_trail_holds_the_step_ups() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    let view = held(&w, &op, &target).await;
    let recs = records(&w);
    let mine: Vec<_> = recs
        .iter()
        .filter(|r| r.trail == Trail::User(target.id))
        .collect();
    let requested = mine
        .iter()
        .find(|r| matches!(r.action, AuditAction::ChannelChangeRequested { .. }))
        .unwrap();
    assert_eq!(requested.actor, Actor::PlatformAdmin(op.admin_id()));
    assert!(matches!(
        &requested.action,
        AuditAction::ChannelChangeRequested {
            kind: ChangeAudit::AdminRecovery {
                verified_by: VerifiedBy::VideoCall,
                break_glass: false,
                ..
            },
            ..
        }
    ));
    assert!(mine.iter().any(
        |r| matches!(r.action, AuditAction::ChannelChangeOnHold { .. })
            && r.actor == Actor::PlatformAdmin(op.admin_id())
    ));
    let admin_steps: Vec<_> = recs
        .iter()
        .filter(|r| r.trail == Trail::Admin(op.admin_id()))
        .map(|r| r.action.clone())
        .collect();
    assert_eq!(
        admin_steps,
        vec![
            AuditAction::StepUpSucceeded {
                action: StepUpAction::AdminRecovery,
                target_id: Some(target.id.as_uuid()),
            },
            AuditAction::StepUpSucceeded {
                action: StepUpAction::AdminRecovery,
                target_id: Some(view.change_id.as_uuid()),
            },
        ]
    );
}

#[tokio::test]
async fn without_a_valid_admin_code_nothing_happens() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    // No code.
    assert!(matches!(
        start(&w, &op, &target, None).await,
        Err(RecoveryError::Identity(IdentityError::StepUpRequired))
    ));
    // Wrong code.
    let mut p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    let good = p.code.clone();
    p.code = "AAAA-0000".into();
    assert!(matches!(
        start(&w, &op, &target, Some(p.clone())).await,
        Err(RecoveryError::Identity(IdentityError::CodeInvalid))
    ));
    // A code for another target.
    let other = add_user(&w, "bia@example.com", false);
    let q = step_up(&w, &op, ADMIN, other.id.as_uuid()).await;
    assert!(matches!(
        start(&w, &op, &target, Some(q)).await,
        Err(RecoveryError::Identity(IdentityError::CodeInvalid))
    ));
    assert!(w.store.state.lock().unwrap().changes.is_empty());
    assert!(w.delivery.last_code_to(NEW).is_none());
    let failures = records(&w)
        .into_iter()
        .filter(|r| {
            r.trail == Trail::Admin(op.admin_id())
                && matches!(r.action, AuditAction::StepUpFailed { .. })
        })
        .count();
    assert_eq!(failures, 3);
    // The right code still works once.
    p.code = good;
    start(&w, &op, &target, Some(p.clone())).await.unwrap();
}

#[tokio::test]
async fn break_glass_is_ssh_only_with_the_same_hold() {
    let w = World::new();
    let (_, cookie, _) = signed_in(&w, ADMIN, true).await;
    let (op, me) = resolve(&w, None, true, ADMIN).await.unwrap();
    assert!(op.break_glass());
    let view = start(&w, &op, &me, None).await.unwrap();
    assert!(view.break_glass);
    let sent_to_admin = w
        .delivery
        .jobs()
        .iter()
        .filter(|j| j.purpose == Purpose::StepUp)
        .count();
    assert_eq!(sent_to_admin, 0, "no admin step-up on break-glass");
    let code = w.delivery.last_code_to(NEW).unwrap();
    let held = confirm(&w, &op, &me, &view, &code, None).await.unwrap();
    assert_eq!(held.status, ChangeStatus::OnHold);
    assert!(records(&w).iter().any(|r| matches!(
        &r.action,
        AuditAction::ChannelChangeOnHold {
            kind: ChangeAudit::AdminRecovery {
                break_glass: true,
                ..
            },
            ..
        }
    )));
    assert!(notices(&w).iter().any(|n| matches!(
        n,
        Notice::ChannelChangeOnHold { to, .. } if to.normalized() == ADMIN
    )));
    // The admin can still cancel it from a session.
    let c = ctx(&w, &cookie).await.unwrap();
    CancelChannelChange::new(w.deps.clone())
        .handle(&c, view.change_id, &meta())
        .await
        .unwrap();
}

#[tokio::test]
async fn who_may_act_on_whom() {
    let w = World::new();
    add_user(&w, ADMIN, true);
    add_user(&w, "other.admin@example.com", true);
    add_user(&w, USER, false);
    assert!(matches!(
        resolve(&w, Some(USER), false, ADMIN).await,
        Err(RecoveryError::NotAnAdmin)
    ));
    assert!(matches!(
        resolve(&w, None, false, USER).await,
        Err(RecoveryError::NotAnAdmin)
    ));
    assert!(matches!(
        resolve(&w, Some(ADMIN), false, ADMIN).await,
        Err(RecoveryError::SelfRecovery)
    ));
    assert!(matches!(
        resolve(&w, None, true, USER).await,
        Err(RecoveryError::BreakGlassNotAdmin)
    ));
    assert!(matches!(
        resolve(&w, Some("other.admin@example.com"), true, ADMIN).await,
        Err(RecoveryError::BreakGlassNotAdmin)
    ));
    assert!(matches!(
        resolve(&w, Some(ADMIN), false, "nobody@example.com").await,
        Err(RecoveryError::TargetNotFound)
    ));
    let (op, _) = resolve(&w, Some(ADMIN), false, USER).await.unwrap();
    assert!(!op.break_glass());
    // (a)(i): with another active admin, break-glass is refused; that admin recovers normally.
    assert!(matches!(
        resolve(&w, None, true, ADMIN).await,
        Err(RecoveryError::OtherAdminAvailable)
    ));
}

#[tokio::test]
async fn break_glass_needs_no_other_active_admin_and_tells_the_account_channels() {
    let w = World::new();
    add_user(&w, ADMIN, true);
    let mut gone = add_user(&w, "old.admin@example.com", true);
    gone.status = identity_enterprise::user::UserStatus::Closed;
    w.store.insert_user(gone);
    // A closed admin account is no other factor: break-glass is allowed.
    let (op, me) = resolve(&w, None, true, ADMIN).await.unwrap();
    assert!(
        RequestOperatorStepUp::new(w.deps.clone())
            .handle(&op, Uuid::now_v7())
            .await
            .is_err(),
        "no admin step-up on break-glass"
    );
    start(&w, &op, &me, None).await.unwrap();
    let ns = notices(&w);
    assert!(ns.iter().any(|n| matches!(
        n,
        Notice::BreakGlassStarted { to, masked_account, .. }
            if to.normalized() == ADMIN && masked_account == "r***@e***.com"
    )));
    assert!(
        !ns.iter()
            .any(|n| matches!(n, Notice::AdminBreakGlassStarted { .. })),
        "no active admin to tell"
    );
    // The normal path sends no break-glass notice.
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    held(&w, &op, &target).await;
    assert!(!notices(&w).iter().any(|n| matches!(
        n,
        Notice::BreakGlassStarted { .. } | Notice::AdminBreakGlassStarted { .. }
    )));
}

#[tokio::test]
async fn wrong_codes_count_and_the_fifth_closes_the_recovery() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    let view = start(&w, &op, &target, Some(p)).await.unwrap();
    let p = step_up(&w, &op, ADMIN, view.change_id.as_uuid()).await;
    for left in (1..=4).rev() {
        assert!(matches!(
            confirm(&w, &op, &target, &view, "AAAA-0000", Some(p.clone())).await,
            Err(RecoveryError::CodeWrong { attempts_left }) if attempts_left == left
        ));
    }
    assert!(matches!(
        confirm(&w, &op, &target, &view, "AAAA-0000", Some(p.clone())).await,
        Err(RecoveryError::AttemptsExhausted)
    ));
    let change = w.store.state.lock().unwrap().changes[0].clone();
    assert_eq!(change.status, ChangeStatus::Cancelled);
    let recs = records(&w);
    assert!(recs.iter().any(|r| r.action
        == AuditAction::CodeRateLimited {
            purpose: Purpose::ChannelChange
        }
        && r.trail == Trail::User(target.id)));
    assert!(
        recs.iter()
            .any(|r| matches!(r.action, AuditAction::ChannelChangeNotCompleted { .. }))
    );
    // The right code no longer helps.
    let code = w.delivery.last_code_to(NEW).unwrap();
    assert!(
        confirm(&w, &op, &target, &view, &code, Some(p))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn a_wrong_code_keeps_the_admin_step_up_for_the_next_try() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    let view = start(&w, &op, &target, Some(p)).await.unwrap();
    let p = step_up(&w, &op, ADMIN, view.change_id.as_uuid()).await;
    assert!(
        confirm(&w, &op, &target, &view, "AAAA-0000", Some(p.clone()))
            .await
            .is_err()
    );
    let code = w.delivery.last_code_to(NEW).unwrap();
    let held = confirm(&w, &op, &target, &view, &code, Some(p.clone()))
        .await
        .unwrap();
    assert_eq!(held.status, ChangeStatus::OnHold);
    // The step-up is spent with the confirmation.
    let again = step_up(&w, &op, ADMIN, view.change_id.as_uuid()).await;
    assert!(matches!(
        CancelAdminRecovery::new(w.deps.clone())
            .handle(&op, &target, view.change_id, Some(p))
            .await,
        Err(RecoveryError::Identity(IdentityError::CodeInvalid))
    ));
    CancelAdminRecovery::new(w.deps.clone())
        .handle(&op, &target, view.change_id, Some(again))
        .await
        .unwrap();
}

#[tokio::test]
async fn confirm_after_ten_minutes_or_by_another_admin_is_refused() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    add_user(&w, "other.admin@example.com", true);
    let (other, _) = resolve(&w, Some("other.admin@example.com"), false, USER)
        .await
        .unwrap();
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    let view = start(&w, &op, &target, Some(p)).await.unwrap();
    let code = w.delivery.last_code_to(NEW).unwrap();
    let q = step_up(
        &w,
        &other,
        "other.admin@example.com",
        view.change_id.as_uuid(),
    )
    .await;
    assert!(matches!(
        confirm(&w, &other, &target, &view, &code, Some(q)).await,
        Err(RecoveryError::OtherAdmin)
    ));
    w.clock.advance(Duration::minutes(10));
    let p = step_up(&w, &op, ADMIN, view.change_id.as_uuid()).await;
    assert!(matches!(
        confirm(&w, &op, &target, &view, &code, Some(p)).await,
        Err(RecoveryError::Identity(IdentityError::ChannelChangeClosed))
    ));
}

#[tokio::test]
async fn the_user_cancels_during_the_hold_and_a_new_request_raises_the_alert() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    let view = held(&w, &op, &target).await;
    let c = ctx(&w, &cookie).await.unwrap();
    // Signing out everywhere does not cancel the hold (Q88).
    SignOutEverywhere::new(w.deps.clone())
        .handle(&c, &meta())
        .await
        .unwrap();
    let mine = ctx(&w, &login(&w, USER).await).await.unwrap();
    assert_eq!(
        w.store.state.lock().unwrap().changes[0].status,
        ChangeStatus::OnHold
    );
    CancelChannelChange::new(w.deps.clone())
        .handle(&mine, view.change_id, &meta())
        .await
        .unwrap();
    assert!(records(&w).iter().any(|r| matches!(
        r.action,
        AuditAction::ChannelChangeCancelled {
            by_user: true,
            kind: ChangeAudit::AdminRecovery { .. },
            ..
        }
    )));
    w.clock.advance(Duration::hours(73));
    assert_eq!(
        ApplyDueChannelHolds::new(w.deps.clone())
            .handle()
            .await
            .unwrap(),
        0
    );
    assert!(w.store.user_by_email(NEW).is_none());
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    start(&w, &op, &target, Some(p)).await.unwrap();
    assert!(matches!(
        w.alerts.0.lock().unwrap().last(),
        Some(SecurityAlert::ChannelChangeAfterCancel { user_id }) if *user_id == target.id
    ));
}

#[tokio::test]
async fn the_admin_cancels_with_a_fresh_step_up() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    let view = held(&w, &op, &target).await;
    assert!(matches!(
        CancelAdminRecovery::new(w.deps.clone())
            .handle(&op, &target, view.change_id, None)
            .await,
        Err(RecoveryError::Identity(IdentityError::StepUpRequired))
    ));
    let p = step_up(&w, &op, ADMIN, view.change_id.as_uuid()).await;
    let done = CancelAdminRecovery::new(w.deps.clone())
        .handle(&op, &target, view.change_id, Some(p))
        .await
        .unwrap();
    assert_eq!(done.status, ChangeStatus::Cancelled);
    assert!(records(&w).iter().any(|r| matches!(
        r.action,
        AuditAction::ChannelChangeCancelled { by_user: false, .. }
    ) && r.actor == Actor::PlatformAdmin(op.admin_id())));
    assert!(notices(&w).iter().any(|n| matches!(
        n,
        Notice::ChannelChangeCancelled { to, .. } if to.normalized() == USER
    )));
}

#[tokio::test]
async fn no_session_can_confirm_a_recovery_code() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    let view = start(&w, &op, &target, Some(p)).await.unwrap();
    // Even with a correctly sealed challenge for the code, the HTTP verify refuses it.
    let row = w
        .store
        .one_time_codes()
        .into_iter()
        .find(|c| c.role == Some(CodeRole::NewValue))
        .unwrap();
    assert_eq!(row.session_id, None);
    let ident = codes::identifier(&w.deps, NEW);
    let challenge = codes::seal(
        &w.deps,
        row.id,
        Purpose::ChannelChange,
        &ident,
        DeviceClass::Known,
        row.state.created_at,
    );
    let c = ctx(&w, &cookie).await.unwrap();
    let code = w.delivery.last_code_to(NEW).unwrap();
    assert!(matches!(
        VerifyChannelChangeCode::new(w.deps.clone())
            .handle(
                &c,
                VerifyChannelChangeCodeCommand {
                    change_id: view.change_id,
                    challenge_id: challenge.expose().to_owned(),
                    code: code.clone(),
                    meta: meta(),
                },
            )
            .await,
        Err(IdentityError::CodeInvalid)
    ));
    let p = step_up(&w, &op, ADMIN, view.change_id.as_uuid()).await;
    confirm(&w, &op, &target, &view, &code, Some(p))
        .await
        .unwrap();
    // And the HTTP step-up request refuses the operator action.
    assert!(matches!(
        RequestStepUp::new(w.deps.clone())
            .handle(
                &c_admin(&w).await,
                RequestStepUpCommand {
                    action: StepUpAction::AdminRecovery,
                    target_id: Some(target.id.as_uuid()),
                    channel: identity_enterprise::channel::Channel::Email,
                    meta: meta(),
                },
            )
            .await,
        Err(IdentityError::InvalidRequest { pointer: "/action" })
    ));
}

async fn c_admin(w: &World) -> crate::auth::AuthContext {
    ctx(w, &login(w, ADMIN).await).await.unwrap()
}

#[tokio::test]
async fn an_open_user_change_is_replaced_and_only_the_target_is_touched() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    let (bia, bia_cookie, _) = signed_in(&w, "bia@example.com", false).await;
    let c = ctx(&w, &cookie).await.unwrap();
    crate::usecases::channel_change::StartChannelChange::new(w.deps.clone())
        .handle(
            &c,
            crate::usecases::channel_change::StartChannelChangeCommand {
                channel: identity_enterprise::channel::Channel::Email,
                new_value: "attacker@example.com".into(),
                meta: meta(),
            },
        )
        .await
        .unwrap();
    let before_bia = w.store.state.lock().unwrap().clone();
    let view = held(&w, &op, &target).await;
    let st = w.store.state.lock().unwrap().clone();
    let user_change = st.changes.iter().find(|c| c.recovery.is_none()).unwrap();
    assert_eq!(user_change.status, ChangeStatus::Cancelled);
    assert_eq!(user_change.cancelled_by_session_id, None);
    assert!(st.changes.iter().any(|c| c.id == view.change_id));
    // Nothing of another user changed; every write ran under the admin's or the target's scope.
    assert_eq!(
        before_bia
            .sessions
            .values()
            .filter(|s| s.user_id == bia.id)
            .count(),
        st.sessions.values().filter(|s| s.user_id == bia.id).count()
    );
    assert!(ctx(&w, &bia_cookie).await.is_ok());
    for set in w.store.committed() {
        assert!(set.scope == op.admin_id() || set.scope == target.id || set.scope == bia.id);
        for wr in &set.writes {
            if let crate::changes::Write::SwitchScope(u) = wr {
                assert_eq!(*u, target.id);
            }
        }
    }
}

#[tokio::test]
async fn a_new_email_of_another_account_is_refused_before_any_code() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    add_user(&w, NEW, false);
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    assert!(matches!(
        start(&w, &op, &target, Some(p)).await,
        Err(RecoveryError::EmailTaken)
    ));
    assert!(w.store.state.lock().unwrap().changes.is_empty());
}

#[tokio::test]
async fn break_glass_on_a_regular_user_is_refused_before_anything_is_written() {
    let w = World::new();
    add_user(&w, ADMIN, true);
    add_user(&w, USER, false);
    let committed = w.store.committed().len();
    assert!(matches!(
        resolve(&w, None, true, USER).await,
        Err(RecoveryError::BreakGlassNotAdmin)
    ));
    // Only the refusal's operator event is written: no change, no code, no send.
    let after = w.store.committed();
    assert!(after[committed..].iter().all(|c| {
        c.writes.is_empty()
            && c.audit.is_empty()
            && c.events
                .iter()
                .all(|e| matches!(e, IdentityEvent::OperatorRecovery(_)))
    }));
    assert!(w.store.state.lock().unwrap().changes.is_empty());
    assert!(w.store.one_time_codes().is_empty());
    assert!(w.delivery.jobs().is_empty());
}

#[test]
fn every_verified_by_needs_a_non_empty_reference() {
    // `other` included (Vigia L1): the reference is required for every method.
    assert!(VerifiedBy::parse("other").is_some());
    for blank in ["", "   ", "\t"] {
        assert!(VerificationRef::parse(blank).is_err());
    }
}

#[test]
fn recovery_view_and_errors_never_show_values() {
    let e = RecoveryError::CodeWrong { attempts_left: 2 };
    assert_eq!(e.to_string(), "wrong code; 2 attempt(s) left");
    assert!(!format!("{:?}", RecoveryError::EmailTaken).contains('@'));
}

#[test]
fn host_report_keeps_only_well_formed_values() {
    let key = format!("SHA256:{}", "Ab+/".repeat(10) + "xyz");
    let h = HostReport::parse(
        Some(" ops_1.a-b "),
        Some("2001:db8::1 51234 10.0.0.2 22"),
        Some(&key),
    );
    assert_eq!(h.login(), Some("ops_1.a-b"));
    assert_eq!(h.ssh_connection(), Some("2001:db8::1 51234 10.0.0.2 22"));
    assert_eq!(h.key_fingerprint(), Some(key.as_str()));
    for bad in ["", "Root", "1ops", "op s", &"a".repeat(33), "o\tps"] {
        assert_eq!(
            HostReport::parse(Some(bad), None, None).login(),
            None,
            "{bad:?}"
        );
    }
    for bad in [
        "203.0.113.5 51234 10.0.0.2",
        "203.0.113.5 99999 10.0.0.2 22",
        "host 1 10.0.0.2 22",
        "203.0.113.5 1 10.0.0.2 22 extra",
    ] {
        assert_eq!(
            HostReport::parse(None, Some(bad), None).ssh_connection(),
            None,
            "{bad:?}"
        );
    }
    for bad in [
        "MD5:aa".to_owned(),
        format!("SHA256:{}", "a".repeat(42)),
        format!("SHA256:{}=", "a".repeat(43)),
        format!("SHA256:{}-", "a".repeat(42)),
    ] {
        assert_eq!(
            HostReport::parse(None, None, Some(&bad)).key_fingerprint(),
            None,
            "{bad:?}"
        );
    }
}

#[tokio::test]
async fn break_glass_without_the_wrapper_report_is_refused_and_logged() {
    let w = World::new();
    add_user(&w, ADMIN, true);
    let r = ResolveOperator::new(w.deps.clone())
        .handle(ResolveOperatorCommand {
            admin_email: None,
            break_glass: true,
            user: UserRef::Email(ADMIN.into()),
            host: HostReport::parse(Some("Not A Login"), None, None),
        })
        .await;
    assert!(matches!(r, Err(RecoveryError::HostReportMissing)));
    assert!(w.store.state.lock().unwrap().changes.is_empty());
    let ev = operator_events(&w);
    assert_eq!(ev.len(), 1);
    assert_eq!(ev[0].0.step, OperatorStep::Resolve);
    assert_eq!(
        ev[0].0.outcome,
        OperatorOutcome::Refused("host_report_missing")
    );
    assert!(ev[0].0.break_glass);
}

#[tokio::test]
async fn refused_break_glass_attempts_reach_the_operator_log() {
    let w = World::new();
    add_user(&w, ADMIN, true);
    add_user(&w, "other.admin@example.com", true);
    add_user(&w, USER, false);
    assert!(resolve(&w, None, true, ADMIN).await.is_err());
    assert!(resolve(&w, None, true, USER).await.is_err());
    let outcomes: Vec<_> = operator_events(&w)
        .into_iter()
        .map(|(r, _)| r.outcome)
        .collect();
    assert_eq!(
        outcomes,
        [
            OperatorOutcome::Refused("other_admin_available"),
            OperatorOutcome::Refused("break_glass_not_admin"),
        ]
    );
    let (r, _) = operator_events(&w).remove(0);
    assert_eq!(
        r.host.login(),
        Some("ops"),
        "host-reported values travel with it"
    );
}

#[tokio::test]
async fn every_committed_action_carries_its_operator_event_in_the_same_commit() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    let view = held(&w, &op, &target).await;
    let p = step_up(&w, &op, ADMIN, view.change_id.as_uuid()).await;
    CancelAdminRecovery::new(w.deps.clone())
        .handle(&op, &target, view.change_id, Some(p))
        .await
        .unwrap();
    let done: Vec<_> = operator_events(&w)
        .into_iter()
        .filter(|(r, _)| r.outcome == OperatorOutcome::Done)
        .collect();
    assert_eq!(
        done.iter().map(|(r, _)| r.step).collect::<Vec<_>>(),
        [
            OperatorStep::Start,
            OperatorStep::Confirm,
            OperatorStep::Cancel
        ]
    );
    for (r, writes) in &done {
        assert!(*writes > 0, "{:?} rides on the action's commit", r.step);
        assert_eq!(r.user_id, target.id);
        assert_eq!(r.admin_user_id, Some(op.admin_id()));
        assert_eq!(r.change_id, Some(view.change_id));
        assert!(!r.break_glass);
    }
}

#[tokio::test]
async fn failed_step_ups_exhausted_confirms_and_repeats_are_logged() {
    let w = World::new();
    let (op, target, cookie) = setup(&w).await;
    assert!(start(&w, &op, &target, None).await.is_err());
    let view = held(&w, &op, &target).await;
    let c = ctx(&w, &cookie).await.unwrap();
    CancelChannelChange::new(w.deps.clone())
        .handle(&c, view.change_id, &meta())
        .await
        .unwrap();
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    let again = start(&w, &op, &target, Some(p)).await.unwrap();
    let p = step_up(&w, &op, ADMIN, again.change_id.as_uuid()).await;
    for _ in 0..5 {
        let _ = confirm(&w, &op, &target, &again, "AAAA-0000", Some(p.clone())).await;
    }
    let ev: Vec<_> = operator_events(&w).into_iter().map(|(r, _)| r).collect();
    assert_eq!(
        (ev[0].step, ev[0].outcome),
        (OperatorStep::Start, OperatorOutcome::StepUpFailed)
    );
    let repeat = ev
        .iter()
        .find(|r| r.change_id == Some(again.change_id) && r.step == OperatorStep::Start)
        .unwrap();
    assert!(repeat.after_cancel, "the repeat alert reaches the server");
    assert_eq!(
        ev.last().map(|r| (r.step, r.outcome)),
        Some((OperatorStep::Confirm, OperatorOutcome::AttemptsExhausted))
    );
}

#[tokio::test]
async fn refusals_around_the_step_up_reach_the_operator_log() {
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    add_user(&w, "other.admin@example.com", true);
    add_user(&w, NEW, false);
    // start: the new e-mail is taken.
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    assert!(matches!(
        start(&w, &op, &target, Some(p)).await,
        Err(RecoveryError::EmailTaken)
    ));
    let refused = |w: &World| -> Vec<(OperatorStep, &'static str)> {
        operator_events(w)
            .into_iter()
            .filter_map(|(r, _)| match r.outcome {
                OperatorOutcome::Refused(why) => Some((r.step, why)),
                _ => None,
            })
            .collect()
    };
    assert_eq!(refused(&w), [(OperatorStep::Start, "email_taken")]);
    // confirm: by another admin, then after the code window.
    let w = World::new();
    let (op, target, _) = setup(&w).await;
    add_user(&w, "other.admin@example.com", true);
    let (other, _) = resolve(&w, Some("other.admin@example.com"), false, USER)
        .await
        .unwrap();
    let p = step_up(&w, &op, ADMIN, target.id.as_uuid()).await;
    let view = start(&w, &op, &target, Some(p)).await.unwrap();
    assert!(
        confirm(&w, &other, &target, &view, "AAAA-0000", None)
            .await
            .is_err()
    );
    w.clock.advance(Duration::minutes(11));
    assert!(
        confirm(&w, &op, &target, &view, "AAAA-0000", None)
            .await
            .is_err()
    );
    assert_eq!(
        refused(&w),
        [
            (OperatorStep::Confirm, "other_admin"),
            (OperatorStep::Confirm, "closed"),
        ]
    );
}
