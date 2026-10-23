//! `requestPasswordReset` (always 202), `confirmPasswordReset` (ends ALL sessions), and the
//! admin fallback `adminCreateResetLink` (Q20, auth policy §3).

use chrono::{DateTime, Utc};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::PasswordResetId;
use identity_enterprise::password_reset::PasswordReset;
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::EndReason;
use identity_enterprise::user::User;
use shared_kernel::UserId;

use crate::auth::AuthContext;
use crate::changes::{Actor, AuditAction, ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::{IdentityDeps, IssuedToken, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{StoreConflict, StoreError};
use crate::throttle::ThrottleDecision;
use crate::usecases::common::{
    admin_audit, check_public_throttle, fragment_link, user_audit, validate_new_password,
};
use crate::views::ResetLink;

fn issue_reset(
    deps: &IdentityDeps,
    user_id: UserId,
    now: DateTime<Utc>,
) -> (PasswordReset, IssuedToken) {
    let token = deps.issue_token();
    (
        PasswordReset::issue(PasswordResetId::new_v7(), user_id, token.hash, now),
        token,
    )
}

pub struct RequestPasswordReset {
    deps: IdentityDeps,
}

impl RequestPasswordReset {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Never reveals whether the e-mail exists or the request was throttled: every outcome is
    /// `Ok(())`. Only internal failures surface (500).
    pub async fn handle(&self, email: &str, meta: &RequestMeta) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        if deps.public_throttle.admit(meta.ip, now).await != ThrottleDecision::Allow {
            return Ok(());
        }
        let Ok(email) = EmailAddress::parse(email) else {
            return Ok(());
        };
        let Some(user) = deps
            .store
            .user_for_login(email.normalized())
            .await?
            .filter(User::is_active)
        else {
            return Ok(());
        };
        let (reset, token) = issue_reset(deps, user.id, now);
        deps.store
            .commit(ChangeSet::new(user.id).write(Write::InsertPasswordReset(reset)))
            .await?;
        let url = fragment_link(deps, user.locale.as_str(), "reset-password", &token);
        // Delivery failures are the mailer's to log; the response stays 202.
        let _ = deps
            .mailer
            .send_password_reset(user.email.as_entered(), user.locale, &url)
            .await;
        Ok(())
    }
}

pub struct ConfirmPasswordReset {
    deps: IdentityDeps,
}

impl ConfirmPasswordReset {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        token: &str,
        new_password: &SecretString,
        meta: &RequestMeta,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        check_public_throttle(deps, meta.ip, now).await?;
        let reset = match deps.hash_presented_token(token) {
            Some(hash) => deps.store.password_reset_by_token(&hash).await?,
            None => None,
        };
        let Some(reset) = reset.filter(|r| r.is_usable(now)) else {
            return Err(IdentityError::ResetTokenInvalid);
        };
        let user = deps
            .store
            .user(reset.user_id)
            .await?
            .filter(User::is_active)
            .ok_or(IdentityError::ResetTokenInvalid)?;
        let password = validate_new_password(deps, new_password, &user.email, "/new_password")?;
        let password_hash = deps
            .passwords
            .hash(password.as_secret())
            .await
            .map_err(IdentityError::Internal)?;
        let changes = ChangeSet::new(user.id)
            .write(Write::UsePasswordReset {
                reset_id: reset.id,
                at: now,
            })
            .write(Write::SetPasswordHash {
                user_id: user.id,
                password_hash,
            })
            .write(Write::EndUserSessions {
                user_id: user.id,
                except: None,
                reason: EndReason::PasswordReset,
                at: now,
            })
            .audit(user_audit(
                user.id,
                Actor::TokenHolder,
                AuditAction::PasswordResetCompleted,
                now,
                Some(meta.ip),
            ))
            .event(IdentityEvent::SessionsEnded {
                user_id: user.id,
                sessions: EndedSessions::AllExcept(Vec::new()),
                reason: EndReason::PasswordReset,
                at: now,
            });
        match deps.store.commit(changes).await {
            Ok(()) => Ok(()),
            Err(StoreError::Conflict(StoreConflict::ResetGone)) => {
                Err(IdentityError::ResetTokenInvalid)
            }
            Err(err) => Err(err.into()),
        }
    }
}

/// Admin fallback when e-mail is unavailable. Audited in the affected user's log (admin id and
/// time, never the link) and in the admin trail.
pub struct AdminCreateResetLink {
    deps: IdentityDeps,
}

impl AdminCreateResetLink {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        target: UserId,
        meta: &RequestMeta,
    ) -> Result<ResetLink, IdentityError> {
        ctx.require_admin()?;
        let deps = &self.deps;
        let now = deps.clock.now();
        let user = deps
            .store
            .user(target)
            .await?
            .ok_or(IdentityError::NotFound)?;
        ctx.require_step_up(now)?;
        let (reset, token) = issue_reset(deps, user.id, now);
        let expires_at = reset.expires_at;
        let action = AuditAction::PasswordResetLinkIssued { target: user.id };
        let changes = ChangeSet::new(user.id)
            .write(Write::InsertPasswordReset(reset))
            .audit(user_audit(
                user.id,
                Actor::Admin(ctx.user.id),
                action.clone(),
                now,
                Some(meta.ip),
            ))
            .audit(admin_audit(ctx.user.id, action, now, Some(meta.ip)));
        deps.store.commit(changes).await?;
        Ok(ResetLink {
            reset_url: fragment_link(deps, user.locale.as_str(), "reset-password", &token),
            expires_at,
        })
    }
}
