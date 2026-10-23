//! Passwordless flows through fakes (auth policy rev. 2 §11): login and sign-up codes, the
//! anti-enumeration rules, devices, step-up binding and channel changes.

use chrono::Duration;
use identity_enterprise::channel::Channel;
use identity_enterprise::channel_change::ChangeStatus;
use identity_enterprise::ids::{ConnectionId, InvitationId};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::otp::{CodeRole, Purpose, StepUpAction};
use identity_enterprise::profile::Locale;
use identity_enterprise::session::EndReason;
use identity_enterprise::source::SourceType;
use shared_kernel::Clock;
use uuid::Uuid;

use crate::changes::{AuditAction, IdentityEvent, Notice};
use crate::error::IdentityError;
use crate::testing::World;
use crate::tests::{add_user, ctx, login, login_on, meta, proof, signed_in};
use crate::usecases::channel_change::*;
use crate::usecases::connections::*;
use crate::usecases::devices::*;
use crate::usecases::signin::*;
use crate::usecases::signup::*;
use crate::usecases::sources::*;
use crate::usecases::step_up::{RequestStepUp, RequestStepUpCommand, StepUpProof};
use crate::views::{ChallengeIssued, DeliveryView, SignedIn};

fn audits(w: &World) -> Vec<AuditAction> {
    w.store
        .committed()
        .into_iter()
        .flat_map(|c| c.audit)
        .map(|a| a.action)
        .collect()
}

async fn request_login(
    w: &World,
    email: &str,
    device: Option<&str>,
) -> Result<ChallengeIssued, IdentityError> {
    RequestLoginCode::new(w.deps.clone())
        .handle(RequestLoginCodeCommand {
            email: email.into(),
            channel: Channel::Email,
            device_cookie: device.map(str::to_owned),
            meta: meta(),
        })
        .await
}

async fn verify_login(
    w: &World,
    challenge: &ChallengeIssued,
    code: &str,
    device: Option<&str>,
) -> Result<SignedIn, IdentityError> {
    VerifyLoginCode::new(w.deps.clone())
        .handle(VerifyLoginCodeCommand {
            challenge_id: challenge.challenge_id.expose().to_owned(),
            code: code.into(),
            remember_me: false,
            device_cookie: device.map(str::to_owned),
            meta: meta(),
        })
        .await
}

/// Full sign-in by code; returns the session and device cookies.
async fn sign_in_by_code(w: &World, email: &str, device: Option<&str>) -> SignedIn {
    w.clock.advance(Duration::seconds(61));
    let ch = request_login(w, email, device).await.unwrap();
    let code = w.delivery.last_code_to(email).unwrap();
    verify_login(w, &ch, &code, device).await.unwrap()
}

// ---------- login ----------

#[tokio::test]
async fn login_answers_the_same_for_unknown_and_inactive_identifiers() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    let known = request_login(&w, "ana@example.com", None).await.unwrap();
    let unknown = request_login(&w, "nobody@example.com", None).await.unwrap();
    assert_eq!(known.resend_after_s, unknown.resend_after_s);
    assert_eq!(known.expires_at, unknown.expires_at);
    assert_eq!(
        known.challenge_id.expose().len(),
        unknown.challenge_id.expose().len()
    );
    assert_eq!(
        w.delivery.jobs().len(),
        1,
        "nothing is sent to an unknown e-mail"
    );
    assert_eq!(*w.store.placeholders.lock().unwrap(), 1);
    // WhatsApp is not offered yet: same answer, nothing sent.
    w.clock.advance(Duration::seconds(61));
    RequestLoginCode::new(w.deps.clone())
        .handle(RequestLoginCodeCommand {
            email: "ana@example.com".into(),
            channel: Channel::Whatsapp,
            device_cookie: None,
            meta: meta(),
        })
        .await
        .unwrap();
    assert_eq!(w.delivery.jobs().len(), 1);
    // A wrong code on an unknown challenge is the same single error.
    assert!(matches!(
        verify_login(&w, &unknown, "K7Q2-0111", None).await,
        Err(IdentityError::CodeInvalid)
    ));
}

#[tokio::test]
async fn sign_in_by_code_registers_a_new_device_then_recognizes_it() {
    let w = World::new();
    let user = add_user(&w, "ana@example.com", false);
    let first = sign_in_by_code(&w, "ana@example.com", None).await;
    assert!(first.new_device);
    assert_eq!(first.user.id, user.id);
    assert!(ctx(&w, first.session.value.expose()).await.is_ok());
    let notices = w
        .store
        .committed()
        .into_iter()
        .flat_map(|c| c.events)
        .filter(|e| {
            matches!(
                e,
                IdentityEvent::NoticeRequested {
                    notice: Notice::NewDevice { .. },
                    ..
                }
            )
        })
        .count();
    assert_eq!(notices, 1);
    let cookie = first.device.value.expose().to_owned();
    let second = sign_in_by_code(&w, "ana@example.com", Some(&cookie)).await;
    assert!(!second.new_device, "known browser");
    assert_eq!(
        second.device.value.expose(),
        cookie,
        "cookie kept and renewed"
    );
    // The code is single use.
    let ch = request_login(&w, "ana@example.com", Some(&cookie)).await;
    assert!(
        matches!(ch, Err(IdentityError::RateLimited { .. })),
        "60-s cooldown"
    );
}

#[tokio::test]
async fn five_wrong_attempts_invalidate_and_resend_keeps_the_count() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    let ch = request_login(&w, "ana@example.com", None).await.unwrap();
    let good = w.delivery.last_code_to("ana@example.com").unwrap();
    for _ in 0..3 {
        assert!(matches!(
            verify_login(&w, &ch, "0000-0000", None).await,
            Err(IdentityError::CodeInvalid)
        ));
    }
    // Resend: new code, attempts kept (3 of 5).
    w.clock.advance(Duration::seconds(61));
    ResendCode::new(w.deps.clone())
        .handle(
            None,
            ResendCodeCommand {
                challenge_id: ch.challenge_id.expose().to_owned(),
                meta: meta(),
            },
        )
        .await
        .unwrap();
    let resent = w.delivery.last_code_to("ana@example.com").unwrap();
    assert_ne!(resent, good);
    assert!(
        matches!(
            verify_login(&w, &ch, &good, None).await,
            Err(IdentityError::CodeInvalid)
        ),
        "the previous code died with the resend"
    );
    assert!(matches!(
        verify_login(&w, &ch, "o000 0000", None).await,
        Err(IdentityError::CodeInvalid)
    ));
    // 5th wrong attempt reached: even the right code fails now.
    assert!(matches!(
        verify_login(&w, &ch, &resent, None).await,
        Err(IdentityError::CodeInvalid)
    ));
    let failed = audits(&w)
        .iter()
        .filter(|a| matches!(a, AuditAction::SignInFailed { .. }))
        .count();
    assert_eq!(failed, 5);
    assert!(
        *w.code_throttle.wrong.lock().unwrap() >= 5,
        "counted per IP too"
    );
    let row = &w.store.one_time_codes()[0];
    assert_eq!(row.state.failed_attempts, 5);
    assert_eq!(
        row.state.invalidated_reason,
        Some(identity_enterprise::otp::InvalidReason::Attempts)
    );
}

#[tokio::test]
async fn codes_expire_after_ten_minutes_and_forged_tokens_fail() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    let ch = request_login(&w, "ana@example.com", None).await.unwrap();
    let code = w.delivery.last_code_to("ana@example.com").unwrap();
    let mut forged = ch.clone();
    let mut t = ch.challenge_id.expose().to_owned();
    t.replace_range(4..5, if &t[4..5] == "0" { "1" } else { "0" });
    forged.challenge_id = identity_enterprise::secret::SecretString::new(t);
    assert!(matches!(
        verify_login(&w, &forged, &code, None).await,
        Err(IdentityError::CodeInvalid)
    ));
    w.clock.advance(Duration::minutes(10));
    assert!(matches!(
        verify_login(&w, &ch, &code, None).await,
        Err(IdentityError::CodeInvalid)
    ));
}

#[tokio::test]
async fn resend_cooldown_is_identical_for_registered_and_unknown_identifiers() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    let known = request_login(&w, "ana@example.com", None).await.unwrap();
    let unknown = request_login(&w, "zz@example.com", None).await.unwrap();
    for ch in [&known, &unknown] {
        let r = ResendCode::new(w.deps.clone())
            .handle(
                None,
                ResendCodeCommand {
                    challenge_id: ch.challenge_id.expose().to_owned(),
                    meta: meta(),
                },
            )
            .await;
        assert!(matches!(r, Err(IdentityError::RateLimited { .. })));
    }
    // Over the 15-min budget: 5 sends per identifier and device class.
    for _ in 0..4 {
        w.clock.advance(Duration::seconds(61));
        request_login(&w, "zz@example.com", None).await.unwrap();
    }
    w.clock.advance(Duration::seconds(61));
    assert!(matches!(
        request_login(&w, "zz@example.com", None).await,
        Err(IdentityError::RateLimited { .. })
    ));
    // A forged token is accepted with the same body and sends nothing.
    w.clock.advance(Duration::seconds(61));
    let sent = w.delivery.jobs().len();
    ResendCode::new(w.deps.clone())
        .handle(
            None,
            ResendCodeCommand {
                challenge_id: "deadbeef".repeat(10),
                meta: meta(),
            },
        )
        .await
        .unwrap();
    assert_eq!(w.delivery.jobs().len(), sent);
}

#[tokio::test]
async fn new_challenges_never_invalidate_an_open_one() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    let mine = request_login(&w, "ana@example.com", Some("x"))
        .await
        .unwrap();
    let code = w.delivery.last_code_to("ana@example.com").unwrap();
    for _ in 0..3 {
        w.clock.advance(Duration::seconds(61));
        request_login(&w, "ana@example.com", None).await.unwrap();
    }
    assert!(verify_login(&w, &mine, &code, None).await.is_ok());
}

#[tokio::test]
async fn login_delivery_is_never_reported() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    let ch = request_login(&w, "ana@example.com", None).await.unwrap();
    assert_eq!(
        GetChallengeDelivery::new(w.deps.clone())
            .handle(None, ch.challenge_id.expose())
            .await
            .unwrap(),
        DeliveryView::NotReported
    );
}

// ---------- sign-up ----------

async fn invitation(w: &World, email: &str) -> String {
    let token = w.deps.issue_token();
    let admin = add_user(
        w,
        &format!("admin-{}@example.com", Uuid::now_v7().simple()),
        true,
    );
    let inv = Invitation::issue(
        InvitationId::new_v7(),
        email.into(),
        Locale::En,
        admin.id,
        w.clock.now(),
    );
    w.store
        .state
        .lock()
        .unwrap()
        .invitations
        .push((inv, token.hash));
    token.value().expose().to_owned()
}

async fn complete(w: &World, ch: &ChallengeIssued, code: &str) -> Result<SignedIn, IdentityError> {
    CompleteSignup::new(w.deps.clone())
        .handle(CompleteSignupCommand {
            challenge_id: ch.challenge_id.expose().to_owned(),
            code: code.into(),
            locale: Locale::PtBr,
            time_zone: "America/Sao_Paulo".into(),
            remember_me: true,
            device_cookie: None,
            meta: meta(),
        })
        .await
}

#[tokio::test]
async fn sign_up_creates_the_account_only_when_the_code_is_confirmed() {
    let w = World::new();
    let token = invitation(&w, "new@example.com").await;
    let ch = StartSignup::new(w.deps.clone())
        .handle(&token, &meta())
        .await
        .unwrap();
    assert!(
        w.store.user_by_email("new@example.com").is_none(),
        "nothing yet"
    );
    let code = w.delivery.last_code_to("new@example.com").unwrap();
    assert_eq!(
        GetChallengeDelivery::new(w.deps.clone())
            .handle(None, ch.challenge_id.expose())
            .await
            .unwrap(),
        DeliveryView::Pending
    );
    let bad_tz = CompleteSignup::new(w.deps.clone())
        .handle(CompleteSignupCommand {
            challenge_id: ch.challenge_id.expose().to_owned(),
            code: code.clone(),
            locale: Locale::En,
            time_zone: "Mars/Base".into(),
            remember_me: false,
            device_cookie: None,
            meta: meta(),
        })
        .await;
    assert!(matches!(bad_tz, Err(IdentityError::InvalidTimeZone)));
    let done = complete(&w, &ch, &code).await.unwrap();
    assert!(
        !done.new_device,
        "the sign-up session sends no new-device notice"
    );
    assert!(done.session.persistent);
    assert!(ctx(&w, done.session.value.expose()).await.is_ok());
    assert!(
        audits(&w)
            .iter()
            .any(|a| matches!(a, AuditAction::AccountCreated { .. }))
    );
    // Second confirmation and a new start on the consumed invitation both fail.
    assert!(matches!(
        complete(&w, &ch, &code).await,
        Err(IdentityError::CodeInvalid)
    ));
    assert!(matches!(
        StartSignup::new(w.deps.clone())
            .handle(&token, &meta())
            .await,
        Err(IdentityError::InvitationInvalid)
    ));
}

#[tokio::test]
async fn sign_up_code_dies_with_its_invitation() {
    let w = World::new();
    let token = invitation(&w, "late@example.com").await;
    let ch = StartSignup::new(w.deps.clone())
        .handle(&token, &meta())
        .await
        .unwrap();
    let code = w.delivery.last_code_to("late@example.com").unwrap();
    w.clock.advance(Duration::days(8));
    assert!(matches!(
        complete(&w, &ch, &code).await,
        Err(IdentityError::CodeInvalid)
    ));
    assert!(w.store.user_by_email("late@example.com").is_none());
}

// ---------- step-up ----------

#[tokio::test]
async fn step_up_proof_is_bound_to_session_action_and_target() {
    let w = World::new();
    let (_, cookie, c) = signed_in(&w, "ana@example.com", false).await;
    let create = CreateSource::new(w.deps.clone());
    assert!(matches!(
        create
            .handle(&c, "Main", SourceType::TradingView, None, &meta())
            .await,
        Err(IdentityError::StepUpRequired)
    ));
    // A proof for another action does not work.
    let other = proof(&w, &c, StepUpAction::ConnectionCreate, None).await;
    assert!(matches!(
        create
            .handle(&c, "Main", SourceType::TradingView, Some(&other), &meta())
            .await,
        Err(IdentityError::CodeInvalid)
    ));
    // Another session of the same user cannot use this session's code.
    let p = proof(&w, &c, StepUpAction::SourceCreate, None).await;
    let other_session = ctx(&w, &login(&w, "ana@example.com").await).await.unwrap();
    assert!(matches!(
        create
            .handle(
                &other_session,
                "Main",
                SourceType::TradingView,
                Some(&p),
                &meta()
            )
            .await,
        Err(IdentityError::CodeInvalid)
    ));
    let made = create
        .handle(&c, "Main", SourceType::TradingView, Some(&p), &meta())
        .await
        .unwrap();
    // Single use.
    assert!(matches!(
        create
            .handle(&c, "Other", SourceType::TradingView, Some(&p), &meta())
            .await,
        Err(IdentityError::CodeInvalid)
    ));
    // Rotation needs a proof for that source.
    let rotate = RotateSource::new(w.deps.clone());
    let wrong_target = proof(
        &w,
        &c,
        StepUpAction::SourceRotate,
        Some(made.source.source.id.as_uuid()),
    )
    .await;
    let second = create
        .handle(
            &c,
            "Second",
            SourceType::TradingView,
            Some(&proof(&w, &c, StepUpAction::SourceCreate, None).await),
            &meta(),
        )
        .await
        .unwrap();
    assert!(matches!(
        rotate
            .handle(&c, second.source.source.id, Some(&wrong_target), &meta())
            .await,
        Err(IdentityError::CodeInvalid)
    ));
    let _ = cookie;
    assert!(
        audits(&w)
            .iter()
            .any(|a| matches!(a, AuditAction::StepUpFailed { .. }))
    );
    assert!(audits(&w).iter().any(|a| matches!(
        a,
        AuditAction::StepUpSucceeded {
            action: StepUpAction::SourceCreate,
            ..
        }
    )));
}

#[tokio::test]
async fn request_step_up_checks_target_rules_and_ownership() {
    let w = World::new();
    let (_, _, a) = signed_in(&w, "a@example.com", false).await;
    let (_, _, b) = signed_in(&w, "b@example.com", false).await;
    let ask = |c: &crate::auth::AuthContext, action, target| {
        let w = &w;
        let c = c.clone();
        async move {
            RequestStepUp::new(w.deps.clone())
                .handle(
                    &c,
                    RequestStepUpCommand {
                        action,
                        target_id: target,
                        channel: Channel::Email,
                        meta: meta(),
                    },
                )
                .await
        }
    };
    assert!(matches!(
        ask(&a, StepUpAction::SourceRotate, None).await,
        Err(IdentityError::InvalidRequest {
            pointer: "/target_id"
        })
    ));
    assert!(matches!(
        ask(&a, StepUpAction::SourceCreate, Some(Uuid::now_v7())).await,
        Err(IdentityError::InvalidRequest {
            pointer: "/target_id"
        })
    ));
    let conn = CreateConnection::new(w.deps.clone())
        .handle(
            &a,
            "Desk",
            Some(&proof(&w, &a, StepUpAction::ConnectionCreate, None).await),
            &meta(),
        )
        .await
        .unwrap()
        .connection
        .connection
        .id;
    let sent = w.delivery.jobs().len();
    assert!(matches!(
        ask(&b, StepUpAction::ConnectionUnpair, Some(conn.as_uuid())).await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        ask(&b, StepUpAction::AccountOptInAccept, Some(Uuid::now_v7())).await,
        Err(IdentityError::NotFound)
    ));
    assert_eq!(
        w.delivery.jobs().len(),
        sent,
        "nothing sent for a foreign target"
    );
    let account = Uuid::now_v7();
    w.accounts.0.lock().unwrap().insert((b.user.id, account));
    ask(&b, StepUpAction::AccountOptInAccept, Some(account))
        .await
        .unwrap();
    assert!(matches!(
        ask(&a, StepUpAction::AdminInvitationCreate, None).await,
        Err(IdentityError::Forbidden)
    ));
    assert!(matches!(
        RequestStepUp::new(w.deps.clone())
            .handle(
                &a,
                RequestStepUpCommand {
                    action: StepUpAction::SourceCreate,
                    target_id: None,
                    channel: Channel::Whatsapp,
                    meta: meta(),
                },
            )
            .await,
        Err(IdentityError::ChannelUnavailable)
    ));
}

#[tokio::test]
async fn q35_off_waives_conditional_step_ups_but_never_unpair() {
    let w = World::with_settings(false);
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let id: ConnectionId = CreateConnection::new(w.deps.clone())
        .handle(&c, "Desk", None, &meta())
        .await
        .unwrap()
        .connection
        .connection
        .id;
    assert!(matches!(
        UnpairConnection::new(w.deps.clone())
            .handle(&c, id, true, None, &meta())
            .await,
        Err(IdentityError::StepUpRequired)
    ));
}

#[tokio::test]
async fn access_port_consumes_the_code_before_the_action() {
    use access::{RequestOrigin, StepUpGate};
    let w = World::new();
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let account = Uuid::now_v7();
    w.accounts.0.lock().unwrap().insert((c.user.id, account));
    let p = proof(&w, &c, StepUpAction::AccountOptInAccept, Some(account)).await;
    let gate = crate::usecases::access_port::IdentityAccess::new(w.deps.clone());
    let caller = crate::usecases::access_port::caller_of(&c);
    let origin = RequestOrigin { ip: meta().ip };
    let grant = gate
        .authorize(
            &caller,
            StepUpAction::AccountOptInAccept,
            Some(account),
            Some(p.clone()),
            &origin,
        )
        .await
        .unwrap();
    assert!(
        grant.proven() && grant.covers(&caller, StepUpAction::AccountOptInAccept, Some(account))
    );
    // Already consumed: a parallel or later request with the same code is refused.
    assert_eq!(
        gate.authorize(
            &caller,
            StepUpAction::AccountOptInAccept,
            Some(account),
            Some(p),
            &origin
        )
        .await,
        Err(access::AccessError::CodeInvalid)
    );
    assert_eq!(
        gate.authorize(
            &caller,
            StepUpAction::AccountOptInAccept,
            Some(account),
            None,
            &origin
        )
        .await,
        Err(access::AccessError::StepUpRequired)
    );
}

// ---------- devices and sessions ----------

#[tokio::test]
async fn forget_device_and_sign_out_everywhere() {
    let w = World::new();
    let (_, cookie, c) = signed_in(&w, "ana@example.com", false).await;
    let other = login(&w, "ana@example.com").await;
    let devices = ListDevices::new(w.deps.clone()).handle(&c).await.unwrap();
    assert_eq!(devices.len(), 2);
    assert_eq!(devices.iter().filter(|d| d.current).count(), 1);
    let other_ctx = ctx(&w, &other).await.unwrap();
    ForgetDevice::new(w.deps.clone())
        .handle(&c, other_ctx.session.device_id, &meta())
        .await
        .unwrap();
    assert!(ctx(&w, &other).await.is_err(), "its session ended");
    assert!(ctx(&w, &cookie).await.is_ok());
    let (_, _, b) = signed_in(&w, "b@example.com", false).await;
    assert!(matches!(
        ForgetDevice::new(w.deps.clone())
            .handle(&b, c.session.device_id, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    SignOutEverywhere::new(w.deps.clone())
        .handle(&c, &meta())
        .await
        .unwrap();
    assert!(ctx(&w, &cookie).await.is_err());
    assert!(
        ListDevices::new(w.deps.clone())
            .handle(&b)
            .await
            .unwrap()
            .len()
            == 1
    );
    let ended = w
        .store
        .committed()
        .into_iter()
        .flat_map(|c| c.events)
        .any(|e| {
            matches!(
                e,
                IdentityEvent::SessionsEnded {
                    reason: EndReason::SignedOutEverywhere,
                    ..
                }
            )
        });
    assert!(ended);
}

#[tokio::test]
async fn revoked_device_cookie_counts_as_none() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    let first = sign_in_by_code(&w, "ana@example.com", None).await;
    let c = ctx(&w, first.session.value.expose()).await.unwrap();
    SignOutEverywhere::new(w.deps.clone())
        .handle(&c, &meta())
        .await
        .unwrap();
    let cookie = first.device.value.expose().to_owned();
    let again = sign_in_by_code(&w, "ana@example.com", Some(&cookie)).await;
    assert!(again.new_device);
    assert_ne!(
        again.device.value.expose(),
        cookie,
        "a new value replaces the revoked one"
    );
}

#[tokio::test]
async fn shared_browser_keeps_the_cookie_for_each_user() {
    let w = World::new();
    add_user(&w, "a@example.com", false);
    add_user(&w, "b@example.com", false);
    let a = sign_in_by_code(&w, "a@example.com", None).await;
    let cookie = a.device.value.expose().to_owned();
    let b = sign_in_by_code(&w, "b@example.com", Some(&cookie)).await;
    assert!(b.new_device, "new for this user");
    assert_eq!(
        b.device.value.expose(),
        cookie,
        "kept: valid for the other user"
    );
    let a2 = sign_in_by_code(&w, "a@example.com", Some(&cookie)).await;
    assert!(!a2.new_device);
}

// ---------- channel change ----------

async fn start_change(
    w: &World,
    c: &crate::auth::AuthContext,
    value: &str,
) -> Result<crate::views::ChannelChangeView, IdentityError> {
    StartChannelChange::new(w.deps.clone())
        .handle(
            c,
            StartChannelChangeCommand {
                channel: Channel::Email,
                new_value: value.into(),
                meta: meta(),
            },
        )
        .await
}

async fn confirm(
    w: &World,
    c: &crate::auth::AuthContext,
    view: &crate::views::ChannelChangeView,
    role: CodeRole,
    to: &str,
) -> Result<crate::views::ChannelChangeVerified, IdentityError> {
    let code = view.codes.iter().find(|x| x.role == role).unwrap();
    VerifyChannelChangeCode::new(w.deps.clone())
        .handle(
            c,
            VerifyChannelChangeCodeCommand {
                change_id: view.id,
                challenge_id: code.challenge_id.expose().to_owned(),
                code: w.delivery.last_code_to(to).unwrap(),
                meta: meta(),
            },
        )
        .await
}

#[tokio::test]
async fn channel_change_completes_rotates_and_freezes_for_24_hours() {
    let w = World::new();
    let (user, cookie, c) = signed_in(&w, "ana@example.com", false).await;
    let other = login(&w, "ana@example.com").await;
    let view = start_change(&w, &c, "Ana.New@Example.com").await.unwrap();
    assert_eq!(view.codes.len(), 2);
    assert_eq!(view.masked_new_value, "a***@e***.com");
    assert!(matches!(
        start_change(&w, &c, "x@example.com").await,
        Err(IdentityError::ChannelChangePending)
    ));
    let one = confirm(&w, &c, &view, CodeRole::CurrentValue, "ana@example.com")
        .await
        .unwrap();
    assert_eq!(one.change.status, ChangeStatus::AwaitingCodes);
    assert!(one.session.is_none());
    let done = confirm(&w, &c, &view, CodeRole::NewValue, "ana.new@example.com")
        .await
        .unwrap();
    assert_eq!(done.change.status, ChangeStatus::Completed);
    let rotated = done.session.unwrap();
    assert!(ctx(&w, rotated.value.expose()).await.is_ok());
    assert!(ctx(&w, &cookie).await.is_err(), "rotated away");
    assert!(ctx(&w, &other).await.is_err(), "other sessions end");
    assert_eq!(
        w.store.user_by_email("ana.new@example.com").unwrap().id,
        user.id
    );
    let c2 = ctx(&w, rotated.value.expose()).await.unwrap();
    assert!(matches!(
        start_change(&w, &c2, "third@example.com").await,
        Err(IdentityError::ChannelChangeTooSoon)
    ));
    assert!(matches!(
        start_change(&w, &c2, "ana.new@example.com").await,
        Err(IdentityError::ChannelUnchanged)
    ));
}

#[tokio::test]
async fn channel_change_from_a_new_device_waits_72_hours_and_can_be_cancelled() {
    let w = World::new();
    let (user, _, _) = signed_in(&w, "ana@example.com", false).await;
    let fresh = login_on(&w, "ana@example.com", Duration::days(1)).await;
    let c = ctx(&w, &fresh).await.unwrap();
    let view = start_change(&w, &c, "new@example.com").await.unwrap();
    confirm(&w, &c, &view, CodeRole::CurrentValue, "ana@example.com")
        .await
        .unwrap();
    let held = confirm(&w, &c, &view, CodeRole::NewValue, "new@example.com")
        .await
        .unwrap();
    assert_eq!(held.change.status, ChangeStatus::OnHold);
    assert!(held.session.is_none());
    assert_eq!(
        w.store.user_by_email("ana@example.com").unwrap().id,
        user.id,
        "unchanged during the hold"
    );
    // Not due yet.
    w.clock.advance(Duration::hours(71));
    assert_eq!(
        ApplyDueChannelHolds::new(w.deps.clone())
            .handle()
            .await
            .unwrap(),
        0
    );
    // Cancelled from another session of the user.
    let mine = ctx(&w, &login(&w, "ana@example.com").await).await.unwrap();
    CancelChannelChange::new(w.deps.clone())
        .handle(&mine, view.id, &meta())
        .await
        .unwrap();
    CancelChannelChange::new(w.deps.clone())
        .handle(&mine, view.id, &meta())
        .await
        .unwrap();
    w.clock.advance(Duration::hours(2));
    assert_eq!(
        ApplyDueChannelHolds::new(w.deps.clone())
            .handle()
            .await
            .unwrap(),
        0
    );
    assert!(w.store.user_by_email("new@example.com").is_none());
    // A new request after a cancellation raises an admin alert.
    start_change(&w, &mine, "again@example.com").await.unwrap();
    assert!(matches!(
        w.alerts.0.lock().unwrap().last(),
        Some(crate::ports::SecurityAlert::ChannelChangeAfterCancel { .. })
    ));
}

#[tokio::test]
async fn due_hold_applies_ends_every_session_and_revokes_devices() {
    let w = World::new();
    let (user, cookie, _) = signed_in(&w, "ana@example.com", false).await;
    let fresh = login_on(&w, "ana@example.com", Duration::days(1)).await;
    let c = ctx(&w, &fresh).await.unwrap();
    let view = start_change(&w, &c, "new@example.com").await.unwrap();
    confirm(&w, &c, &view, CodeRole::CurrentValue, "ana@example.com")
        .await
        .unwrap();
    confirm(&w, &c, &view, CodeRole::NewValue, "new@example.com")
        .await
        .unwrap();
    w.clock.advance(Duration::hours(72));
    assert_eq!(
        ApplyDueChannelHolds::new(w.deps.clone())
            .handle()
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        w.store.user_by_email("new@example.com").unwrap().id,
        user.id
    );
    assert!(ctx(&w, &cookie).await.is_err());
    assert!(ctx(&w, &fresh).await.is_err());
    assert!(
        w.store
            .state
            .lock()
            .unwrap()
            .known_devices
            .iter()
            .all(|d| d.revoked_at.is_some())
    );
}

#[tokio::test]
async fn channel_clash_is_never_revealed() {
    let w = World::new();
    add_user(&w, "taken@example.com", false);
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let view = start_change(&w, &c, "taken@example.com").await.unwrap();
    let new_code = view
        .codes
        .iter()
        .find(|x| x.role == CodeRole::NewValue)
        .unwrap();
    let job = w
        .delivery
        .jobs()
        .into_iter()
        .find(|j| j.delivery.destination.normalized() == "taken@example.com")
        .unwrap();
    assert!(job.suppressed, "nothing is sent to the other account");
    // The challenge exists like any other; its verification fails with the generic error.
    let r = VerifyChannelChangeCode::new(w.deps.clone())
        .handle(
            &c,
            VerifyChannelChangeCodeCommand {
                change_id: view.id,
                challenge_id: new_code.challenge_id.expose().to_owned(),
                code: "0000-0000".into(),
                meta: meta(),
            },
        )
        .await;
    assert!(matches!(r, Err(IdentityError::CodeInvalid)));
}

#[tokio::test]
async fn change_codes_are_bound_to_the_session_and_die_when_the_change_expires() {
    let w = World::new();
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let other = ctx(&w, &login(&w, "ana@example.com").await).await.unwrap();
    let view = start_change(&w, &c, "new@example.com").await.unwrap();
    assert!(matches!(
        confirm(&w, &other, &view, CodeRole::CurrentValue, "ana@example.com").await,
        Err(IdentityError::CodeInvalid)
    ));
    let (_, _, b) = signed_in(&w, "b@example.com", false).await;
    assert!(matches!(
        confirm(&w, &b, &view, CodeRole::CurrentValue, "ana@example.com").await,
        Err(IdentityError::NotFound)
    ));
    w.clock.advance(Duration::minutes(10));
    assert!(matches!(
        confirm(&w, &c, &view, CodeRole::CurrentValue, "ana@example.com").await,
        Err(IdentityError::ChannelChangeClosed)
    ));
    // A stale change does not block a new start.
    w.clock.advance(Duration::seconds(61));
    start_change(&w, &c, "new2@example.com").await.unwrap();
}

#[tokio::test]
async fn security_view_shows_the_pending_change() {
    let w = World::new();
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    start_change(&w, &c, "new@example.com").await.unwrap();
    let sec = crate::usecases::me::GetMySecurity::new(w.deps.clone())
        .handle(&c)
        .await
        .unwrap();
    assert_eq!(sec.email, "ana@example.com");
    let pending = sec.pending_channel_change.unwrap();
    assert_eq!(pending.codes.len(), 2);
    assert!(pending.codes.iter().all(|c| !c.confirmed));
    let job = w
        .delivery
        .jobs()
        .into_iter()
        .find(|j| j.purpose == Purpose::ChannelChange)
        .unwrap();
    assert!(
        !format!("{job:?}").contains(job.delivery.code.expose()),
        "code redacted in Debug"
    );
}

#[tokio::test]
async fn step_up_proof_type_is_redacted() {
    let p = StepUpProof {
        challenge_id: "abc".into(),
        code: "K7Q2-0111".into(),
    };
    assert!(!format!("{p:?}").contains("K7Q2"));
}
