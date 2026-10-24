//! Admin-assisted recovery of a user who lost every channel (S3 AC-12, auth policy §9, Q28 a).
//! Only the operator command `allwin-core admin recovery start|confirm|status|cancel` runs these
//! use cases; there is no HTTP route (Esquadro).
//!
//! - The operator is a platform admin named on the command line. Every action that changes state
//!   needs a fresh step-up code sent to the admin's own confirmed channel (target: the user on
//!   `start`, the change on `confirm` and `cancel`). Break-glass (I3, an admin recovering
//!   themself) is SSH only: no code, `break_glass = true` in the audit.
//! - `start` opens a change of kind `admin_recovery` in the AC-11 table and sends a code to the
//!   new e-mail. The user reads it to the admin, who types it in `confirm`; that opens the 72-h
//!   hold with notices to every old channel. The hold worker applies it like an M2 hold: every
//!   session ends and every device is revoked.
//! - The admin's step-up is consumed in the same transaction as the target's change
//!   ([`ChangeSet::then_in`]). The target's audit names the actor `platform_admin`, never the
//!   admin; the admin's own trail holds the step-up entries.

use std::net::{IpAddr, Ipv4Addr};

use chrono::{DateTime, Utc};
use identity_enterprise::channel::{Channel, Destination};
use identity_enterprise::channel_change::{
    ChangeKind, ChangeStatus, ChannelChange, HOLD, Recovery, VerificationRef, VerifiedBy,
};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::{ChannelChangeId, CodeId};
use identity_enterprise::otp::{
    CodeRole, CodeState, DeviceClass, InvalidReason, OneTimeCode, Purpose, StepUpAction,
};
use identity_enterprise::user::User;
use shared_kernel::UserId;
use thiserror::Error;
use uuid::Uuid;

use crate::changes::{
    Actor, AuditAction, ChangeAudit, ChangeSet, CodeTable, HostReport, IdentityEvent, Notice,
    OperatorOutcome, OperatorRecord, OperatorStep, StepUpFailure, Write,
};
use crate::deps::IdentityDeps;
use crate::error::IdentityError;
use crate::ports::{CodeDelivery, CodeMessage, SecurityAlert, StoreConflict, StoreError};
use crate::usecases::channel_change::{not_completed, notice_to_all, value_taken};
use crate::usecases::codes::{self, Check};
use crate::usecases::common::{admin_audit, user_audit};
use crate::usecases::delivery::DeliveryJob;
use crate::views::ChallengeIssued;

pub use access::StepUpProof;

/// The operator command has no client address; the per-IP memory limits use the loopback.
const OPERATOR_IP: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

/// Failures of the operator command. Messages are for the operator's terminal: they never hold
/// a code or an unmasked channel value.
#[derive(Debug, Error)]
pub enum RecoveryError {
    #[error("{0}")]
    Identity(#[from] IdentityError),
    #[error("no active user matches --user")]
    TargetNotFound,
    #[error("--admin is not an active platform admin")]
    NotAnAdmin,
    #[error(
        "an admin cannot recover their own account with --admin; the operator uses --break-glass"
    )]
    SelfRecovery,
    #[error("--break-glass recovers only a platform admin's own account")]
    BreakGlassNotAdmin,
    #[error(
        "another active platform admin exists: they run the normal recovery with their step-up"
    )]
    OtherAdminAvailable,
    #[error("--break-glass runs only through the host wrapper, which reports the SSH session")]
    HostReportMissing,
    #[error("the new e-mail belongs to another account")]
    EmailTaken,
    #[error("this change is not an admin recovery")]
    NotARecovery,
    #[error("only the admin who started this recovery, in the same mode, can confirm it")]
    OtherAdmin,
    #[error("wrong code; {attempts_left} attempt(s) left")]
    CodeWrong { attempts_left: i16 },
    #[error("wrong code; the attempts are used up and the recovery was closed")]
    AttemptsExhausted,
}

impl RecoveryError {
    /// Stable reason code of a refusal, for the operator log.
    fn refusal(&self) -> Option<&'static str> {
        Some(match self {
            Self::NotAnAdmin => "not_an_admin",
            Self::SelfRecovery => "self_recovery",
            Self::BreakGlassNotAdmin => "break_glass_not_admin",
            Self::OtherAdminAvailable => "other_admin_available",
            Self::HostReportMissing => "host_report_missing",
            Self::EmailTaken => "email_taken",
            Self::NotARecovery => "not_a_recovery",
            Self::OtherAdmin => "other_admin",
            Self::Identity(IdentityError::InvalidChannelValue) => "invalid_new_email",
            Self::Identity(IdentityError::ChannelUnchanged) => "unchanged",
            Self::Identity(IdentityError::ChannelChangeTooSoon) => "too_soon",
            Self::Identity(IdentityError::ChannelChangePending) => "pending",
            Self::Identity(IdentityError::ChannelChangeClosed) => "closed",
            Self::Identity(IdentityError::NotFound) => "not_found",
            Self::Identity(IdentityError::RateLimited { .. }) => "rate_limited",
            // Step-up failures, wrong or exhausted codes and store errors are logged where they
            // happen, or are not refusals.
            _ => return None,
        })
    }
}

impl From<StoreError> for RecoveryError {
    fn from(e: StoreError) -> Self {
        Self::Identity(e.into())
    }
}

/// How the command names the user: the current e-mail or the user id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserRef {
    Email(String),
    Id(UserId),
}

/// A platform admin acting through the operator command, resolved by [`ResolveOperator`].
#[derive(Clone, Debug)]
pub struct Operator {
    admin: User,
    break_glass: bool,
    host: HostReport,
}

impl Operator {
    pub fn admin_id(&self) -> UserId {
        self.admin.id
    }

    pub fn break_glass(&self) -> bool {
        self.break_glass
    }

    pub fn host(&self) -> &HostReport {
        &self.host
    }

    /// `identity.operator_recovery` for one action on `user` (Esquadro N-1).
    fn record(
        &self,
        user: UserId,
        change: Option<ChannelChangeId>,
        step: OperatorStep,
        outcome: OperatorOutcome,
        now: DateTime<Utc>,
    ) -> OperatorRecord {
        OperatorRecord {
            user_id: user,
            admin_user_id: Some(self.admin.id),
            change_id: change,
            step,
            outcome,
            break_glass: self.break_glass,
            after_cancel: false,
            host: self.host.clone(),
            at: now,
        }
    }

    /// Masked e-mail of the admin's channel the step-up code goes to.
    pub fn masked_step_up_channel(&self) -> String {
        Destination::Email(self.admin.email.clone()).masked()
    }
}

pub struct ResolveOperatorCommand {
    /// `--admin`: the admin's e-mail; absent on break-glass.
    pub admin_email: Option<String>,
    pub break_glass: bool,
    pub user: UserRef,
    /// What the host wrapper reported; break-glass needs a valid login.
    pub host: HostReport,
}

/// Commits one operator event on its own (a refusal or a failure: there is no action to share
/// the transaction with). Best effort (Brasa R3-5): if this commit fails, the caller still
/// returns the refusal or code error it had, which the operator needs to see.
async fn log_operator(deps: &IdentityDeps, record: OperatorRecord) {
    let set = ChangeSet::new(record.user_id).event(IdentityEvent::OperatorRecovery(record));
    // A store failure here surfaces on the command's next call; the caller's error wins.
    let _ = deps.store.commit(set).await;
}

/// A refusal of `start`, `confirm` or `cancel` before or around the admin's step-up (Brasa
/// R3-3): it goes to the operator log with its reason code. Step-up failures and exhausted
/// confirms are logged where they happen.
async fn log_refusal(
    deps: &IdentityDeps,
    op: &Operator,
    user: UserId,
    change: Option<ChannelChangeId>,
    step: OperatorStep,
    e: &RecoveryError,
) {
    if let Some(reason) = e.refusal() {
        let now = deps.clock.now();
        log_operator(
            deps,
            op.record(user, change, step, OperatorOutcome::Refused(reason), now),
        )
        .await;
    }
}

/// Finds the target user and the acting admin, and applies who may act on whom.
pub struct ResolveOperator {
    deps: IdentityDeps,
}

impl ResolveOperator {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Refusals for a known target are written to the operator log (N-3).
    pub async fn handle(
        &self,
        cmd: ResolveOperatorCommand,
    ) -> Result<(Operator, User), RecoveryError> {
        let target = self
            .find(&cmd.user)
            .await?
            .ok_or(RecoveryError::TargetNotFound)?;
        match self.check(&cmd, &target).await {
            Ok(op) => Ok((op, target)),
            Err(e) => {
                if let Some(reason) = e.refusal() {
                    log_operator(
                        &self.deps,
                        OperatorRecord {
                            user_id: target.id,
                            admin_user_id: cmd.break_glass.then_some(target.id),
                            change_id: None,
                            step: OperatorStep::Resolve,
                            outcome: OperatorOutcome::Refused(reason),
                            break_glass: cmd.break_glass,
                            after_cancel: false,
                            host: cmd.host.clone(),
                            at: self.deps.clock.now(),
                        },
                    )
                    .await;
                }
                Err(e)
            }
        }
    }

    async fn check(
        &self,
        cmd: &ResolveOperatorCommand,
        target: &User,
    ) -> Result<Operator, RecoveryError> {
        if cmd.break_glass {
            // I3: only an admin recovering themself, with no admin credential at all.
            let names_other = match &cmd.admin_email {
                Some(e) => {
                    self.find(&UserRef::Email(e.clone())).await?.map(|u| u.id) != Some(target.id)
                }
                None => false,
            };
            if !target.is_platform_admin || names_other {
                return Err(RecoveryError::BreakGlassNotAdmin);
            }
            if cmd.host.login().is_none() {
                return Err(RecoveryError::HostReportMissing);
            }
            // Policy 3.3 §9.1 (a)(i): only without another step-up factor, i.e. when no other
            // active admin can run the normal recovery. After S3b a confirmed WhatsApp of the
            // target also refuses it (a)(ii).
            if !other_admins(&self.deps, target).await?.is_empty() {
                return Err(RecoveryError::OtherAdminAvailable);
            }
            return Ok(Operator {
                admin: target.clone(),
                break_glass: true,
                host: cmd.host.clone(),
            });
        }
        let Some(email) = &cmd.admin_email else {
            return Err(RecoveryError::NotAnAdmin);
        };
        let admin = self
            .find(&UserRef::Email(email.clone()))
            .await?
            .filter(|u| u.is_platform_admin)
            .ok_or(RecoveryError::NotAnAdmin)?;
        if admin.id == target.id {
            return Err(RecoveryError::SelfRecovery);
        }
        Ok(Operator {
            admin,
            break_glass: false,
            host: cmd.host.clone(),
        })
    }

    async fn find(&self, user: &UserRef) -> Result<Option<User>, RecoveryError> {
        let id = match user {
            UserRef::Id(id) => *id,
            UserRef::Email(raw) => {
                let Ok(email) = EmailAddress::parse(raw) else {
                    return Ok(None);
                };
                match self.deps.store.login_target(email.normalized()).await? {
                    Some(t) => t.user_id,
                    None => return Ok(None),
                }
            }
        };
        Ok(self.deps.store.user(id).await?.filter(User::is_active))
    }
}

/// The active platform admins other than `admin` (answered only for an active admin's scope).
async fn other_admins(deps: &IdentityDeps, admin: &User) -> Result<Vec<User>, RecoveryError> {
    let listed = deps.store.users(admin.id).await?;
    // Fail closed (Brasa N-4): the listing answers only an active admin's scope, so an empty or
    // foreign answer means it could not be checked, never "no other admin".
    if !listed
        .iter()
        .any(|s| s.id == admin.id && s.is_platform_admin)
    {
        return Err(IdentityError::internal("admin listing did not include the admin").into());
    }
    let mut found = Vec::new();
    for s in listed {
        if s.is_platform_admin
            && s.id != admin.id
            && let Some(u) = deps.store.user(s.id).await?.filter(User::is_active)
        {
            found.push(u);
        }
    }
    Ok(found)
}

/// Sends the admin a fresh step-up code for one recovery action. `target` is the user id for
/// `start` and the change id for `confirm` and `cancel`. Not used on break-glass.
pub struct RequestOperatorStepUp {
    deps: IdentityDeps,
}

impl RequestOperatorStepUp {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        op: &Operator,
        target: Uuid,
    ) -> Result<ChallengeIssued, RecoveryError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let admin = &op.admin;
        if op.break_glass {
            return Err(IdentityError::InvalidRequest {
                pointer: "/break_glass",
            }
            .into());
        }
        let destination = Destination::Email(admin.email.clone());
        let ident = codes::identifier(deps, destination.normalized());
        let limits = async {
            codes::admit_user_sends(deps, admin.id, 1, now).await?;
            codes::admit_send(deps, OPERATOR_IP, &ident, DeviceClass::Known, now).await
        };
        if let Err(e) = limits.await {
            if matches!(e, IdentityError::RateLimited { .. }) {
                rate_limited(deps, admin.id, Purpose::StepUp, now).await?;
            }
            return Err(e.into());
        }
        let code_id = CodeId::new_v7();
        let fresh = codes::fresh_code(deps, code_id)?;
        let row = OneTimeCode {
            id: code_id,
            user_id: admin.id,
            purpose: Purpose::StepUp,
            destination: destination.clone(),
            action: Some(StepUpAction::AdminRecovery),
            target_id: Some(target),
            // No session: the challenge stays in the command's memory (data model rev. 16.8).
            session_id: None,
            channel_change_id: None,
            role: None,
            request_ip: None,
            state: CodeState::issue(fresh.hmac, now),
        };
        deps.store
            .commit(ChangeSet::new(admin.id).write(Write::InsertOneTimeCode(row)))
            .await?;
        deps.delivery.dispatch(DeliveryJob {
            table: CodeTable::OneTime,
            code_id,
            user: Some(admin.id),
            purpose: Purpose::StepUp,
            delivery: CodeDelivery {
                destination,
                locale: admin.locale,
                code: fresh.plain,
                message: CodeMessage::StepUp(StepUpAction::AdminRecovery),
            },
            suppressed: false,
            ip: None,
        });
        let challenge = codes::seal(
            deps,
            code_id,
            Purpose::StepUp,
            &ident,
            DeviceClass::Known,
            now,
        );
        Ok(codes::issued(challenge, now))
    }
}

/// `security.code_rate_limited` with no client address.
async fn rate_limited(
    deps: &IdentityDeps,
    user: UserId,
    purpose: Purpose,
    now: DateTime<Utc>,
) -> Result<(), IdentityError> {
    let changes = ChangeSet::new(user).audit(user_audit(
        user,
        Actor::System,
        AuditAction::CodeRateLimited { purpose },
        now,
        None,
    ));
    deps.store.commit(changes).await?;
    Ok(())
}

/// Checks the admin's step-up for `target`. Returns the admin's change set (consume plus
/// `security.step_up_succeeded` in the admin's trail), which the caller commits together with
/// the target's change; on break-glass an empty set. Failures are audited and committed here.
async fn authorize(
    deps: &IdentityDeps,
    op: &Operator,
    on: (UserId, Option<ChannelChangeId>, OperatorStep),
    proof: Option<&StepUpProof>,
    now: DateTime<Utc>,
) -> Result<ChangeSet, RecoveryError> {
    let admin = op.admin.id;
    if op.break_glass {
        return Ok(ChangeSet::new(admin));
    }
    let (user, change, step) = on;
    // `start` binds the code to the user; `confirm` and `cancel` to the change.
    let target = change.map_or(user.as_uuid(), |c| c.as_uuid());
    let failure = |e: IdentityError| async move {
        log_operator(
            deps,
            op.record(user, change, step, OperatorOutcome::StepUpFailed, now),
        )
        .await;
        Err::<ChangeSet, RecoveryError>(e.into())
    };
    let action = StepUpAction::AdminRecovery;
    let failed = |reason| {
        admin_audit(
            admin,
            AuditAction::StepUpFailed {
                action,
                target_id: Some(target),
                reason,
            },
            now,
            None,
        )
    };
    let Some(proof) = proof else {
        deps.store
            .commit(ChangeSet::new(admin).audit(failed(StepUpFailure::Missing)))
            .await?;
        return failure(IdentityError::StepUpRequired).await;
    };
    let row = match codes::open(deps, &proof.challenge_id) {
        Some(claims) if claims.purpose == Purpose::StepUp => {
            deps.store.one_time_code(admin, claims.code_id).await?
        }
        _ => None,
    };
    let bound = row.filter(|r| {
        r.purpose == Purpose::StepUp
            && r.session_id.is_none()
            && r.action == Some(action)
            && r.target_id == Some(target)
            && op.admin.has_destination(&r.destination)
    });
    let Some(row) = bound else {
        deps.store
            .commit(ChangeSet::new(admin).audit(failed(StepUpFailure::CodeInvalid)))
            .await?;
        return failure(IdentityError::CodeInvalid).await;
    };
    match codes::check(deps, row.id, &row.state, &proof.code, now) {
        Check::Match(hmac) => Ok(ChangeSet::new(admin)
            .write(Write::ConsumeCode {
                table: CodeTable::OneTime,
                code_id: row.id,
                code_hmac: hmac,
                at: now,
            })
            .audit(admin_audit(
                admin,
                AuditAction::StepUpSucceeded {
                    action,
                    target_id: Some(target),
                },
                now,
                None,
            ))),
        Check::Closed => {
            deps.store
                .commit(ChangeSet::new(admin).audit(failed(StepUpFailure::CodeInvalid)))
                .await?;
            failure(IdentityError::CodeInvalid).await
        }
        Check::Wrong => {
            codes::record_wrong(
                deps,
                admin,
                CodeTable::OneTime,
                row.id,
                Some(failed(StepUpFailure::CodeInvalid)),
                OPERATOR_IP,
                now,
            )
            .await?;
            failure(IdentityError::CodeInvalid).await
        }
    }
}

/// Policy 3.3 §9.1 (c): a break-glass start tells the target's current channels and every other
/// active admin (none while (a)(i) holds; the rule stays for when it changes). Notices go through
/// the outbox in the same transaction; an admin's notice row is written in that admin's scope.
async fn break_glass_notices(
    deps: &IdentityDeps,
    mut set: ChangeSet,
    target: &User,
    now: DateTime<Utc>,
) -> Result<ChangeSet, RecoveryError> {
    let masked_account = Destination::Email(target.email.clone()).masked();
    for e in notice_to_all(target, now, |to| Notice::BreakGlassStarted {
        to,
        masked_account: masked_account.clone(),
        at: now,
    }) {
        set = set.event(e);
    }
    for admin in other_admins(deps, target).await? {
        set = set.event(IdentityEvent::NoticeRequested {
            user_id: admin.id,
            locale: admin.locale,
            notice: Notice::AdminBreakGlassStarted {
                to: Destination::Email(admin.email.clone()),
                masked_account: masked_account.clone(),
                at: now,
            },
            at: now,
        });
    }
    Ok(set)
}

/// Every committed break-glass action raises an operational alert (Brasa HIGH-1): the hold is
/// the protection, and operators must see that someone used SSH alone.
fn alert_break_glass(deps: &IdentityDeps, op: &Operator, change: &ChannelChange) {
    if op.break_glass {
        deps.alerts.raise(SecurityAlert::AdminRecoveryBreakGlass {
            user_id: change.user_id,
            change_id: change.id,
        });
    }
}

/// What the command prints: masked values only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryView {
    pub change_id: ChannelChangeId,
    pub kind: ChangeKind,
    pub status: ChangeStatus,
    pub masked_new_value: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub hold_until: Option<DateTime<Utc>>,
    pub break_glass: bool,
}

impl RecoveryView {
    fn of(change: &ChannelChange, now: DateTime<Utc>) -> Self {
        Self {
            change_id: change.id,
            kind: change.kind(),
            status: change.status_at(now),
            masked_new_value: change.new_value.masked(),
            created_at: change.created_at,
            expires_at: change.expires_at,
            hold_until: change.hold_until,
            break_glass: change.recovery.as_ref().is_some_and(|r| r.break_glass),
        }
    }
}

pub struct StartRecoveryCommand {
    pub new_email: String,
    pub verified_by: VerifiedBy,
    pub verification_ref: VerificationRef,
    /// The admin's step-up for target = the user id; `None` on break-glass.
    pub step_up: Option<StepUpProof>,
}

/// `admin recovery start`: opens the recovery and sends a code to the new e-mail. An open user
/// change of the target is closed first by the Core (`not_completed`, with notices).
pub struct StartAdminRecovery {
    deps: IdentityDeps,
}

impl StartAdminRecovery {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Refusals go to the operator log (R3-3).
    pub async fn handle(
        &self,
        op: &Operator,
        target: &User,
        cmd: StartRecoveryCommand,
    ) -> Result<RecoveryView, RecoveryError> {
        let r = self.run(op, target, cmd).await;
        if let Err(e) = &r {
            log_refusal(&self.deps, op, target.id, None, OperatorStep::Start, e).await;
        }
        r
    }

    async fn run(
        &self,
        op: &Operator,
        target: &User,
        cmd: StartRecoveryCommand,
    ) -> Result<RecoveryView, RecoveryError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let new_value = EmailAddress::parse(&cmd.new_email)
            .map(Destination::Email)
            .map_err(|_| IdentityError::InvalidChannelValue)?;
        if target.has_destination(&new_value) {
            return Err(IdentityError::ChannelUnchanged.into());
        }
        if target.channel_change_allowed_at() > now {
            return Err(IdentityError::ChannelChangeTooSoon.into());
        }
        let open = deps
            .store
            .open_channel_change(target.id)
            .await?
            .filter(|c| c.status_at(now).is_open());
        if open.as_ref().is_some_and(|c| c.recovery.is_some()) {
            return Err(IdentityError::ChannelChangePending.into());
        }
        if value_taken(deps, target.id, &new_value).await? {
            return Err(RecoveryError::EmailTaken);
        }
        let grant = authorize(
            deps,
            op,
            (target.id, None, OperatorStep::Start),
            cmd.step_up.as_ref(),
            now,
        )
        .await?;
        let ident = codes::identifier(deps, new_value.normalized());
        let limits = async {
            codes::admit_user_sends(deps, target.id, 1, now).await?;
            codes::admit_send(deps, OPERATOR_IP, &ident, DeviceClass::Known, now).await
        };
        if let Err(e) = limits.await {
            if matches!(e, IdentityError::RateLimited { .. }) {
                rate_limited(deps, target.id, Purpose::ChannelChange, now).await?;
            }
            return Err(e.into());
        }
        let after_cancel = deps
            .store
            .latest_channel_change(target.id)
            .await?
            .is_some_and(|c| c.status == ChangeStatus::Cancelled);
        let change = ChannelChange::start_recovery(
            ChannelChangeId::new_v7(),
            target.id,
            new_value.clone(),
            Recovery {
                admin_user_id: op.admin.id,
                verified_by: cmd.verified_by,
                verification_ref: cmd.verification_ref,
                break_glass: op.break_glass,
            },
            now,
        );
        let code_id = CodeId::new_v7();
        let fresh = codes::fresh_code(deps, code_id)?;
        let code = OneTimeCode {
            id: code_id,
            user_id: target.id,
            purpose: Purpose::ChannelChange,
            destination: new_value.clone(),
            action: None,
            target_id: None,
            session_id: None,
            channel_change_id: Some(change.id),
            role: Some(CodeRole::NewValue),
            request_ip: None,
            state: CodeState::issue(fresh.hmac, now),
        };
        let mut target_set = ChangeSet::new(target.id).write(Write::ExpireStaleChannelChange {
            user_id: target.id,
            at: now,
        });
        if let Some(user_change) = &open {
            target_set = target_set.extend(not_completed(target, user_change, now));
        }
        target_set = target_set
            .write(Write::InsertChannelChange(change.clone()))
            .write(Write::InsertOneTimeCode(code))
            .audit(user_audit(
                target.id,
                Actor::PlatformAdmin(op.admin.id),
                AuditAction::ChannelChangeRequested {
                    change_id: change.id,
                    kind: ChangeAudit::of(&change),
                    channel: Channel::Email,
                    masked_new_value: new_value.masked(),
                },
                now,
                None,
            ));
        if op.break_glass {
            target_set = break_glass_notices(deps, target_set, target, now).await?;
        }
        // The operator log and the repeat alert travel with the commit (Esquadro N-1): the
        // command's own stdout is the operator's terminal, never collected.
        target_set = target_set.event(IdentityEvent::OperatorRecovery(OperatorRecord {
            after_cancel,
            ..op.record(
                target.id,
                Some(change.id),
                OperatorStep::Start,
                OperatorOutcome::Done,
                now,
            )
        }));
        match deps.store.commit(grant.then_in(target_set)).await {
            Ok(()) => {}
            Err(StoreError::Conflict(StoreConflict::ChannelChangeOpen)) => {
                return Err(IdentityError::ChannelChangePending.into());
            }
            Err(StoreError::Conflict(StoreConflict::ChannelChangeGone)) => {
                // The user's change closed meanwhile; nothing was written.
                return Err(IdentityError::ChannelChangePending.into());
            }
            Err(e) => return Err(codes::consume_conflict(e).into()),
        }
        if after_cancel {
            deps.alerts
                .raise(SecurityAlert::ChannelChangeAfterCancel { user_id: target.id });
        }
        alert_break_glass(deps, op, &change);
        deps.delivery.dispatch(DeliveryJob {
            table: CodeTable::OneTime,
            code_id,
            user: Some(target.id),
            purpose: Purpose::ChannelChange,
            delivery: CodeDelivery {
                destination: new_value,
                locale: target.locale,
                code: fresh.plain,
                message: CodeMessage::AdminRecovery,
            },
            suppressed: false,
            ip: None,
        });
        Ok(RecoveryView::of(&change, now))
    }
}

/// The target's recovery `change_id`, checked to be an admin recovery.
async fn recovery(
    deps: &IdentityDeps,
    target: &User,
    change_id: ChannelChangeId,
) -> Result<ChannelChange, RecoveryError> {
    let change = deps
        .store
        .channel_change(target.id, change_id)
        .await?
        .ok_or(IdentityError::NotFound)?;
    if change.recovery.is_none() {
        return Err(RecoveryError::NotARecovery);
    }
    Ok(change)
}

pub struct ConfirmRecoveryCommand {
    pub change_id: ChannelChangeId,
    /// The code the user read from the new e-mail.
    pub code: String,
    /// The admin's step-up for target = the change id; `None` on break-glass.
    pub step_up: Option<StepUpProof>,
}

/// `admin recovery confirm`: the admin types the code the user received at the new e-mail; the
/// recovery goes on hold for 72 h with notices to every old channel. Five wrong codes close it
/// (`security.code_rate_limited`, then `not_completed`). A wrong code does not spend the admin's
/// step-up, which is consumed only with the confirmation.
pub struct ConfirmAdminRecovery {
    deps: IdentityDeps,
}

impl ConfirmAdminRecovery {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Refusals go to the operator log (R3-3).
    pub async fn handle(
        &self,
        op: &Operator,
        target: &User,
        cmd: ConfirmRecoveryCommand,
    ) -> Result<RecoveryView, RecoveryError> {
        let change = Some(cmd.change_id);
        let r = self.run(op, target, cmd).await;
        if let Err(e) = &r {
            log_refusal(&self.deps, op, target.id, change, OperatorStep::Confirm, e).await;
        }
        r
    }

    async fn run(
        &self,
        op: &Operator,
        target: &User,
        cmd: ConfirmRecoveryCommand,
    ) -> Result<RecoveryView, RecoveryError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let change = recovery(deps, target, cmd.change_id).await?;
        let started_by = change
            .recovery
            .as_ref()
            .map(|r| (r.admin_user_id, r.break_glass));
        if started_by != Some((op.admin.id, op.break_glass)) {
            return Err(RecoveryError::OtherAdmin);
        }
        if change.status_at(now) != ChangeStatus::AwaitingCodes {
            return Err(IdentityError::ChannelChangeClosed.into());
        }
        let Some(row) = deps
            .store
            .change_codes(target.id, change.id)
            .await?
            .into_iter()
            .find(|c| {
                c.role == Some(CodeRole::NewValue)
                    && c.session_id.is_none()
                    && c.destination.normalized() == change.new_value.normalized()
            })
        else {
            log_operator(
                deps,
                op.record(
                    target.id,
                    Some(change.id),
                    OperatorStep::Confirm,
                    OperatorOutcome::Refused("code_missing"),
                    now,
                ),
            )
            .await;
            return Err(IdentityError::CodeInvalid.into());
        };
        let grant = authorize(
            deps,
            op,
            (target.id, Some(change.id), OperatorStep::Confirm),
            cmd.step_up.as_ref(),
            now,
        )
        .await?;
        let hmac = match codes::check(deps, row.id, &row.state, &cmd.code, now) {
            Check::Match(h) => h,
            Check::Closed => {
                log_operator(
                    deps,
                    op.record(
                        target.id,
                        Some(change.id),
                        OperatorStep::Confirm,
                        OperatorOutcome::Refused("code_closed"),
                        now,
                    ),
                )
                .await;
                return Err(IdentityError::CodeInvalid.into());
            }
            Check::Wrong => return Err(self.wrong(op, target, &change, &row, now).await?),
        };
        let hold_until = now + HOLD;
        let mut target_set = ChangeSet::new(target.id)
            .write(Write::ConsumeCode {
                table: CodeTable::OneTime,
                code_id: row.id,
                code_hmac: hmac,
                at: now,
            })
            .write(Write::HoldChannelChange {
                change_id: change.id,
                hold_until,
                at: now,
            })
            .audit(user_audit(
                target.id,
                Actor::PlatformAdmin(op.admin.id),
                AuditAction::ChannelChangeOnHold {
                    change_id: change.id,
                    kind: ChangeAudit::of(&change),
                    channel: change.channel(),
                    hold_until,
                },
                now,
                None,
            ));
        // The target's current values are the old ones: every one of them is told.
        for e in notice_to_all(target, now, |to| Notice::ChannelChangeOnHold {
            to,
            channel: change.channel(),
            masked_new_value: change.new_value.masked(),
            hold_until,
        }) {
            target_set = target_set.event(e);
        }
        target_set = target_set.event(IdentityEvent::OperatorRecovery(op.record(
            target.id,
            Some(change.id),
            OperatorStep::Confirm,
            OperatorOutcome::Done,
            now,
        )));
        deps.store
            .commit(grant.then_in(target_set))
            .await
            .map_err(codes::consume_conflict)?;
        alert_break_glass(deps, op, &change);
        let mut held = change;
        held.status = ChangeStatus::OnHold;
        held.hold_until = Some(hold_until);
        Ok(RecoveryView::of(&held, now))
    }

    /// Counts a wrong code; the attempt that uses the last one closes the recovery.
    async fn wrong(
        &self,
        op: &Operator,
        target: &User,
        change: &ChannelChange,
        row: &OneTimeCode,
        now: DateTime<Utc>,
    ) -> Result<RecoveryError, RecoveryError> {
        let deps = &self.deps;
        codes::record_wrong(
            deps,
            target.id,
            CodeTable::OneTime,
            row.id,
            None,
            OPERATOR_IP,
            now,
        )
        .await?;
        let after = deps.store.one_time_code(target.id, row.id).await?;
        let exhausted = after
            .as_ref()
            .is_some_and(|c| c.state.invalidated_reason == Some(InvalidReason::Attempts));
        if !exhausted {
            let used = after.map_or(row.state.failed_attempts + 1, |c| c.state.failed_attempts);
            return Ok(RecoveryError::CodeWrong {
                attempts_left: identity_enterprise::otp::MAX_WRONG_ATTEMPTS - used,
            });
        }
        let record = op.record(
            target.id,
            Some(change.id),
            OperatorStep::Confirm,
            OperatorOutcome::AttemptsExhausted,
            now,
        );
        let set = not_completed(target, change, now)
            .audit(user_audit(
                target.id,
                Actor::System,
                AuditAction::CodeRateLimited {
                    purpose: Purpose::ChannelChange,
                },
                now,
                None,
            ))
            .event(IdentityEvent::OperatorRecovery(record.clone()));
        match deps.store.commit(set).await {
            Ok(()) => Ok(RecoveryError::AttemptsExhausted),
            // Closed meanwhile (cancelled by the user, or expired): nothing more to close, but
            // the operator log still gets the exhausted confirm.
            Err(StoreError::Conflict(StoreConflict::ChannelChangeGone)) => {
                log_operator(deps, record).await;
                Ok(RecoveryError::AttemptsExhausted)
            }
            Err(e) => Err(e.into()),
        }
    }
}

/// `admin recovery cancel`: any active admin (or the break-glass operator for their own
/// recovery) closes an open recovery; audited as `cancelled` by `system` with the actor
/// `platform_admin`, with notices to every channel.
pub struct CancelAdminRecovery {
    deps: IdentityDeps,
}

impl CancelAdminRecovery {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Refusals go to the operator log (R3-3).
    pub async fn handle(
        &self,
        op: &Operator,
        target: &User,
        change_id: ChannelChangeId,
        step_up: Option<StepUpProof>,
    ) -> Result<RecoveryView, RecoveryError> {
        let r = self.run(op, target, change_id, step_up).await;
        if let Err(e) = &r {
            log_refusal(
                &self.deps,
                op,
                target.id,
                Some(change_id),
                OperatorStep::Cancel,
                e,
            )
            .await;
        }
        r
    }

    async fn run(
        &self,
        op: &Operator,
        target: &User,
        change_id: ChannelChangeId,
        step_up: Option<StepUpProof>,
    ) -> Result<RecoveryView, RecoveryError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let change = recovery(deps, target, change_id).await?;
        match change.status_at(now) {
            ChangeStatus::AwaitingCodes | ChangeStatus::OnHold => {}
            _ => return Err(IdentityError::ChannelChangeClosed.into()),
        }
        let grant = authorize(
            deps,
            op,
            (target.id, Some(change.id), OperatorStep::Cancel),
            step_up.as_ref(),
            now,
        )
        .await?;
        let mut target_set = ChangeSet::new(target.id)
            .write(Write::CancelChannelChange {
                change_id: change.id,
                by_session: None,
                at: now,
            })
            .audit(user_audit(
                target.id,
                Actor::PlatformAdmin(op.admin.id),
                AuditAction::ChannelChangeCancelled {
                    change_id: change.id,
                    kind: ChangeAudit::of(&change),
                    channel: change.channel(),
                    by_user: false,
                },
                now,
                None,
            ));
        for e in notice_to_all(target, now, |to| Notice::ChannelChangeCancelled {
            to,
            channel: change.channel(),
        }) {
            target_set = target_set.event(e);
        }
        target_set = target_set.event(IdentityEvent::OperatorRecovery(op.record(
            target.id,
            Some(change.id),
            OperatorStep::Cancel,
            OperatorOutcome::Done,
            now,
        )));
        match deps.store.commit(grant.then_in(target_set)).await {
            Ok(()) => {}
            Err(StoreError::Conflict(StoreConflict::ChannelChangeGone)) => {
                return Err(IdentityError::ChannelChangeClosed.into());
            }
            Err(e) => return Err(codes::consume_conflict(e).into()),
        }
        alert_break_glass(deps, op, &change);
        let mut cancelled = change;
        cancelled.status = ChangeStatus::Cancelled;
        cancelled.cancelled_at = Some(now);
        Ok(RecoveryView::of(&cancelled, now))
    }
}

/// `admin recovery status`: the target's latest channel change, of either kind (read only, no
/// step-up).
pub struct RecoveryStatus {
    deps: IdentityDeps,
}

impl RecoveryStatus {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        _op: &Operator,
        target: &User,
    ) -> Result<Option<RecoveryView>, RecoveryError> {
        let now = self.deps.clock.now();
        Ok(self
            .deps
            .store
            .latest_channel_change(target.id)
            .await?
            .map(|c| RecoveryView::of(&c, now)))
    }
}
