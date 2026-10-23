//! `getMe`, `updateMe` (Q23), `getMySecurity`, `changePassword`.

use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::{self, EndReason, MfaState};
use identity_enterprise::user::User;

use crate::auth::AuthContext;
use crate::changes::{Actor, AuditAction, ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::PasswordCheck;
use crate::usecases::common::{
    check_throttle, cookie, new_session, password_matches, user_attempt, user_audit,
    validate_new_password,
};
use crate::views::{SecurityView, SessionCookie, SessionInfo};

pub struct GetMe;

impl GetMe {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&self, ctx: &AuthContext) -> User {
        ctx.user.clone()
    }
}

impl Default for GetMe {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UpdateMe {
    deps: IdentityDeps,
}

impl UpdateMe {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        locale: Option<Locale>,
        time_zone: Option<&str>,
    ) -> Result<User, IdentityError> {
        let time_zone = time_zone
            .map(TimeZoneName::parse)
            .transpose()
            .map_err(|_| IdentityError::InvalidTimeZone)?;
        let mut user = ctx.user.clone();
        if let Some(locale) = locale {
            user.locale = locale;
        }
        if let Some(tz) = &time_zone {
            user.time_zone = tz.clone();
        }
        let changes = ChangeSet::new(user.id).write(Write::UpdateProfile {
            user_id: user.id,
            locale,
            time_zone,
        });
        self.deps.store.commit(changes).await?;
        Ok(user)
    }
}

pub struct GetMySecurity {
    deps: IdentityDeps,
}

impl GetMySecurity {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, ctx: &AuthContext) -> Result<SecurityView, IdentityError> {
        let totp_enabled = self
            .deps
            .store
            .totp(ctx.user.id)
            .await?
            .is_some_and(|c| c.is_enabled());
        let recovery_codes_remaining = if totp_enabled {
            self.deps
                .store
                .recovery_codes_remaining(ctx.user.id)
                .await?
        } else {
            0
        };
        let s = &ctx.session;
        Ok(SecurityView {
            totp_enabled,
            recovery_codes_remaining,
            session: SessionInfo {
                persistent: s.persistent,
                idle_timeout_s: session::idle_timeout(s.persistent).num_seconds(),
                absolute_timeout_s: session::absolute_timeout(s.persistent).num_seconds(),
                idle_expires_at: s.idle_expires_at,
                absolute_expires_at: s.absolute_expires_at,
                step_up_valid_until: s.step_up_valid_until(),
            },
        })
    }
}

pub struct ChangePasswordCommand {
    pub current_password: SecretString,
    pub new_password: SecretString,
    pub meta: RequestMeta,
}

/// Step-up; rotates the current session and ends every other one.
pub struct ChangePassword {
    deps: IdentityDeps,
}

impl ChangePassword {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        cmd: ChangePasswordCommand,
    ) -> Result<SessionCookie, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        ctx.require_step_up(now)?;
        let attempt = user_attempt(deps, &ctx.user, cmd.meta.ip, now).await?;
        check_throttle(deps, &attempt, now).await?;
        if password_matches(deps, &ctx.user, &cmd.current_password).await? == PasswordCheck::Invalid
        {
            deps.login_throttle.record_failure(&attempt, now).await;
            return Err(IdentityError::ReauthFailed);
        }
        deps.login_throttle.record_success(&attempt, now).await;
        let password =
            validate_new_password(deps, &cmd.new_password, &ctx.user.email, "/new_password")?;
        let password_hash = deps
            .passwords
            .hash(password.as_secret())
            .await
            .map_err(IdentityError::Internal)?;

        let (new, token) = new_session(deps, ctx.user.id, ctx.session.persistent, &cmd.meta);
        let mfa = match ctx.session.mfa_state {
            MfaState::Verified => MfaState::Verified,
            _ => MfaState::NotRequired,
        };
        let rotated = ctx.session.rotate(new, mfa, now);
        let changes = ChangeSet::new(ctx.user.id)
            .write(Write::SetPasswordHash {
                user_id: ctx.user.id,
                password_hash,
            })
            .write(Write::EndSession {
                session_id: ctx.session.id,
                reason: EndReason::Rotated,
                at: now,
            })
            .write(Write::EndUserSessions {
                user_id: ctx.user.id,
                except: Some(rotated.id),
                reason: EndReason::PasswordChanged,
                at: now,
            })
            .write(Write::InsertSession(rotated.clone()))
            .audit(user_audit(
                ctx.user.id,
                Actor::User(ctx.user.id),
                AuditAction::PasswordChanged,
                now,
                Some(cmd.meta.ip),
            ))
            .event(IdentityEvent::SessionsEnded {
                user_id: ctx.user.id,
                sessions: EndedSessions::AllExcept(vec![ctx.session.id, rotated.id]),
                reason: EndReason::PasswordChanged,
                at: now,
            });
        deps.store.commit(changes).await?;
        Ok(cookie(
            &token,
            rotated.persistent,
            rotated.absolute_expires_at,
        ))
    }
}
