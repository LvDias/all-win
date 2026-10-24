//! `requestStepUp` and the step-up proof carried by sensitive actions (auth policy rev. 2 §6,
//! HTTP 2.0.0 `StepUpProof`). A code is bound to the session, the action and the target, and is
//! consumed in the same transaction as the action, so no "elevated" state exists.

use identity_enterprise::channel::Channel;
use identity_enterprise::ids::{CodeId, ConnectionId, SourceId};
use identity_enterprise::otp::{CodeState, DeviceClass, OneTimeCode, Purpose, StepUpAction};
use uuid::Uuid;

use crate::auth::AuthContext;
use crate::changes::{Actor, AuditAction, ChangeSet, CodeTable, StepUpFailure, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{CodeDelivery, CodeMessage};
use crate::usecases::codes::{self, Check};
use crate::usecases::common::user_audit;
use crate::usecases::delivery::DeliveryJob;
use crate::views::ChallengeIssued;

pub use access::StepUpProof;

/// Whether `action` needs a proof under the current settings.
pub fn step_up_required(deps: &IdentityDeps, action: StepUpAction) -> bool {
    !action.is_q35_conditional() || deps.settings.step_up_q35_persistence_actions
}

pub struct RequestStepUpCommand {
    pub action: StepUpAction,
    pub target_id: Option<Uuid>,
    pub channel: Channel,
    pub meta: RequestMeta,
}

pub struct RequestStepUp {
    deps: IdentityDeps,
}

impl RequestStepUp {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        cmd: RequestStepUpCommand,
    ) -> Result<ChallengeIssued, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let user = &ctx.user;
        if cmd.action.is_operator_only() {
            // `admin_recovery`: only the operator command asks for it (HTTP 5.4.0).
            return Err(IdentityError::InvalidRequest { pointer: "/action" });
        }
        if cmd.action == StepUpAction::AdminInvitationCreate {
            ctx.require_admin()?;
        }
        if cmd.action.requires_target() != cmd.target_id.is_some() {
            return Err(IdentityError::InvalidRequest {
                pointer: "/target_id",
            });
        }
        if let Some(target) = cmd.target_id {
            self.check_target(ctx, cmd.action, target).await?;
        }
        if cmd.channel == Channel::Whatsapp && !deps.settings.whatsapp_enabled {
            return Err(IdentityError::ChannelUnavailable);
        }
        let Some(destination) = user.destination(cmd.channel) else {
            return Err(IdentityError::ChannelUnavailable);
        };
        // M3: from a new device, persistence actions need the e-mail code.
        let device_is_new = deps
            .store
            .device(user.id, ctx.session.device_id)
            .await?
            .is_none_or(|d| d.is_new(now));
        if device_is_new && cmd.action.is_persistence() && cmd.channel != Channel::Email {
            return Err(IdentityError::StepUpChannelNotAllowed);
        }
        let ident = codes::identifier(deps, destination.normalized());
        let limits = async {
            codes::admit_user_sends(deps, user.id, 1, now).await?;
            codes::admit_send(deps, cmd.meta.ip, &ident, DeviceClass::Known, now).await
        };
        if let Err(e) = limits.await {
            if matches!(e, IdentityError::RateLimited { .. }) {
                codes::audit_rate_limited(deps, user.id, Purpose::StepUp, cmd.meta.ip, now).await?;
            }
            return Err(e);
        }
        let code_id = CodeId::new_v7();
        let fresh = codes::fresh_code(deps, code_id)?;
        let row = OneTimeCode {
            id: code_id,
            user_id: user.id,
            purpose: Purpose::StepUp,
            destination: destination.clone(),
            action: Some(cmd.action),
            target_id: cmd.target_id,
            session_id: Some(ctx.session.id),
            channel_change_id: None,
            role: None,
            request_ip: Some(cmd.meta.ip),
            state: CodeState::issue(fresh.hmac, now),
        };
        deps.store
            .commit(ChangeSet::new(user.id).write(Write::InsertOneTimeCode(row)))
            .await?;
        deps.delivery.dispatch(DeliveryJob {
            table: CodeTable::OneTime,
            code_id,
            user: Some(user.id),
            purpose: Purpose::StepUp,
            delivery: CodeDelivery {
                destination,
                locale: user.locale,
                code: fresh.plain,
                message: CodeMessage::StepUp(cmd.action),
            },
            suppressed: false,
            ip: Some(cmd.meta.ip),
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

    /// The target exists and is the caller's (404 otherwise; nothing sent or counted).
    async fn check_target(
        &self,
        ctx: &AuthContext,
        action: StepUpAction,
        target: Uuid,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let user = ctx.user.id;
        let found = match action {
            StepUpAction::SourceRotate => deps
                .store
                .source(user, SourceId::from_uuid(target))
                .await?
                .is_some_and(|s| s.revoked_at.is_none()),
            StepUpAction::PairingCodeNew | StepUpAction::ConnectionUnpair => deps
                .store
                .connection(user, ConnectionId::from_uuid(target))
                .await?
                .is_some_and(|c| c.connection.ensure_not_unpaired().is_ok()),
            StepUpAction::AccountOptInAccept
            | StepUpAction::RouteEnable
            | StepUpAction::AccountBindingChange => deps
                .account_targets
                .owned_by(user, target)
                .await
                .map_err(IdentityError::internal)?,
            _ => false,
        };
        if found {
            Ok(())
        } else {
            Err(IdentityError::NotFound)
        }
    }
}

/// Checks the proof of a step-up operation. `Ok(None)`: no proof needed. `Ok(Some(set))`: the
/// proof is valid; the caller commits `set` (consume + audit) in the action's transaction, so a
/// commit conflict there means the code was used meanwhile (`auth.code_invalid`). Failures are
/// audited and committed here.
pub(crate) async fn authorize(
    deps: &IdentityDeps,
    ctx: &AuthContext,
    action: StepUpAction,
    target_id: Option<Uuid>,
    proof: Option<&StepUpProof>,
    meta: &RequestMeta,
) -> Result<Option<ChangeSet>, IdentityError> {
    if !step_up_required(deps, action) {
        return Ok(None);
    }
    let now = deps.clock.now();
    let user = ctx.user.id;
    let failed = |reason| {
        user_audit(
            user,
            Actor::User(user),
            AuditAction::StepUpFailed {
                action,
                target_id,
                reason,
            },
            now,
            Some(meta.ip),
        )
    };
    let Some(proof) = proof else {
        deps.store
            .commit(ChangeSet::new(user).audit(failed(StepUpFailure::Missing)))
            .await?;
        return Err(IdentityError::StepUpRequired);
    };
    codes::admit_verify(deps, meta.ip, now).await?;
    let row = match codes::open(deps, &proof.challenge_id) {
        Some(claims) if claims.purpose == Purpose::StepUp => {
            deps.store.one_time_code(user, claims.code_id).await?
        }
        _ => None,
    };
    let bound = row.filter(|r| {
        r.purpose == Purpose::StepUp
            && r.session_id == Some(ctx.session.id)
            && r.action == Some(action)
            && r.target_id == target_id
            && ctx.user.has_destination(&r.destination)
    });
    let Some(row) = bound else {
        deps.code_throttle.record_wrong(meta.ip, now).await;
        deps.store
            .commit(ChangeSet::new(user).audit(failed(StepUpFailure::CodeInvalid)))
            .await?;
        return Err(IdentityError::CodeInvalid);
    };
    match codes::check(deps, row.id, &row.state, &proof.code, now) {
        Check::Match(hmac) => Ok(Some(
            ChangeSet::new(user)
                .write(Write::ConsumeCode {
                    table: CodeTable::OneTime,
                    code_id: row.id,
                    code_hmac: hmac,
                    at: now,
                })
                .audit(user_audit(
                    user,
                    Actor::User(user),
                    AuditAction::StepUpSucceeded { action, target_id },
                    now,
                    Some(meta.ip),
                )),
        )),
        Check::Closed => {
            deps.code_throttle.record_wrong(meta.ip, now).await;
            deps.store
                .commit(ChangeSet::new(user).audit(failed(StepUpFailure::CodeInvalid)))
                .await?;
            Err(IdentityError::CodeInvalid)
        }
        Check::Wrong => {
            codes::record_wrong(
                deps,
                user,
                CodeTable::OneTime,
                row.id,
                Some(failed(StepUpFailure::CodeInvalid)),
                meta.ip,
                now,
            )
            .await?;
            Err(IdentityError::CodeInvalid)
        }
    }
}

/// Starts the action's change set with the step-up consume, when there is one.
pub(crate) fn with_grant(scope: shared_kernel::UserId, grant: Option<ChangeSet>) -> ChangeSet {
    grant.unwrap_or_else(|| ChangeSet::new(scope))
}
