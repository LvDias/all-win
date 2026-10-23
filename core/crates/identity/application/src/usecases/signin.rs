//! `login`, `verifyLoginTotp`, `verifyLoginRecoveryCode`, `logout`, `stepUp`.

use identity_enterprise::codes::{CodeKind, NormalizedCode};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::{EndReason, MfaState, Session};
use identity_enterprise::user::User;

use crate::auth::{AuthContext, AuthenticateSession, activity_write};
use crate::changes::{Actor, AuditAction, ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{PasswordCheck, StoreConflict, StoreError};
use crate::usecases::common::{
    check_throttle, cookie, login_attempt, new_session, password_matches, totp_step, user_attempt,
    user_audit,
};
use crate::views::LoginOutcome;

pub struct LoginCommand {
    pub email: String,
    pub password: SecretString,
    pub remember_me: bool,
    pub meta: RequestMeta,
}

/// Same response and comparable timing for unknown e-mail, wrong password and closed user
/// (S3 AC-5): every failure runs one argon2id verification.
pub struct Login {
    deps: IdentityDeps,
}

impl Login {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cmd: LoginCommand) -> Result<LoginOutcome, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let email = EmailAddress::parse(&cmd.email).ok();
        let user = match &email {
            Some(email) => deps.store.user_for_login(email.normalized()).await?,
            None => None,
        };
        let normalized = email.as_ref().map_or_else(
            || cmd.email.trim().to_lowercase(),
            |e| e.normalized().to_owned(),
        );
        let attempt = login_attempt(deps, normalized, user.as_ref(), cmd.meta.ip, now).await?;
        check_throttle(deps, &attempt, now).await?;

        let check = match &user {
            Some(user) => password_matches(deps, user, &cmd.password).await?,
            None => {
                deps.passwords.verify_dummy(&cmd.password).await;
                PasswordCheck::Invalid
            }
        };
        let (user, needs_rehash) = match (user, check) {
            (Some(user), PasswordCheck::Valid { needs_rehash }) if user.is_active() => {
                (user, needs_rehash)
            }
            _ => {
                deps.login_throttle.record_failure(&attempt, now).await;
                return Err(IdentityError::InvalidCredentials);
            }
        };
        deps.login_throttle.record_success(&attempt, now).await;

        let totp_enabled = deps
            .store
            .totp(user.id)
            .await?
            .is_some_and(|c| c.is_enabled());
        let (new, token) = new_session(deps, user.id, cmd.remember_me, &cmd.meta);
        let session = Session::start(new, totp_enabled, now);
        let mut changes = ChangeSet::new(user.id).write(Write::InsertSession(session.clone()));
        if needs_rehash {
            let rehashed = deps
                .passwords
                .hash(&identity_enterprise::password::normalize(&cmd.password))
                .await
                .map_err(IdentityError::Internal)?;
            changes = changes.write(Write::SetPasswordHash {
                user_id: user.id,
                password_hash: rehashed,
            });
        }
        deps.store.commit(changes).await?;

        let cookie = cookie(&token, session.persistent, session.absolute_expires_at);
        Ok(if session.mfa_state == MfaState::Pending {
            LoginOutcome::Pending2fa {
                cookie,
                pending_expires_at: session.absolute_expires_at,
            }
        } else {
            LoginOutcome::Authenticated {
                user,
                cookie,
                recovery_codes_remaining: None,
            }
        })
    }
}

/// Completes a pending sign-in: consumes the second factor, ends the pending session and starts
/// the full one (session id rotated) in one transaction.
async fn finish_second_factor(
    deps: &IdentityDeps,
    ctx: &AuthContext,
    meta: &RequestMeta,
    consume: Write,
    extra: impl FnOnce(ChangeSet) -> ChangeSet,
) -> Result<(User, crate::views::SessionCookie), StoreError> {
    let now = deps.clock.now();
    let (new, token) = new_session(deps, ctx.user.id, ctx.session.persistent, meta);
    let full = ctx.session.rotate(new, MfaState::Verified, now);
    let changes = ChangeSet::new(ctx.user.id)
        .write(consume)
        .write(Write::EndSession {
            session_id: ctx.session.id,
            reason: EndReason::Rotated,
            at: now,
        })
        .write(Write::InsertSession(full.clone()));
    deps.store.commit(extra(changes)).await?;
    Ok((
        ctx.user.clone(),
        cookie(&token, full.persistent, full.absolute_expires_at),
    ))
}

pub struct VerifyLoginTotp {
    deps: IdentityDeps,
}

impl VerifyLoginTotp {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// `ctx` must come from [`AuthenticateSession`] with `Requirement::Pending`.
    pub async fn handle(
        &self,
        ctx: &AuthContext,
        code: &str,
        meta: &RequestMeta,
    ) -> Result<LoginOutcome, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let attempt = user_attempt(deps, &ctx.user, meta.ip, now).await?;
        check_throttle(deps, &attempt, now).await?;
        let Some(step) = totp_step(deps, ctx.user.id, code, now).await? else {
            deps.login_throttle.record_failure(&attempt, now).await;
            return Err(IdentityError::InvalidCode);
        };
        let consume = Write::ConsumeTotpStep {
            user_id: ctx.user.id,
            step,
        };
        match finish_second_factor(deps, ctx, meta, consume, |c| c).await {
            Ok((user, cookie)) => {
                deps.login_throttle.record_success(&attempt, now).await;
                Ok(LoginOutcome::Authenticated {
                    user,
                    cookie,
                    recovery_codes_remaining: None,
                })
            }
            Err(StoreError::Conflict(StoreConflict::TotpStepReplayed)) => {
                deps.login_throttle.record_failure(&attempt, now).await;
                Err(IdentityError::InvalidCode)
            }
            Err(err) => Err(err.into()),
        }
    }
}

pub struct VerifyLoginRecoveryCode {
    deps: IdentityDeps,
}

impl VerifyLoginRecoveryCode {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        code: &str,
        meta: &RequestMeta,
    ) -> Result<LoginOutcome, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let attempt = user_attempt(deps, &ctx.user, meta.ip, now).await?;
        check_throttle(deps, &attempt, now).await?;
        let Some(normalized) = NormalizedCode::parse(CodeKind::Recovery, code) else {
            deps.login_throttle.record_failure(&attempt, now).await;
            return Err(IdentityError::InvalidCode);
        };
        let remaining = deps
            .store
            .recovery_codes_remaining(ctx.user.id)
            .await?
            .saturating_sub(1);
        let consume = Write::ConsumeRecoveryCode {
            user_id: ctx.user.id,
            candidates: deps.hasher.code_hmac_candidates(&normalized),
            at: now,
        };
        let audit = user_audit(
            ctx.user.id,
            Actor::User(ctx.user.id),
            AuditAction::RecoveryCodeUsed { remaining },
            now,
            Some(meta.ip),
        );
        match finish_second_factor(deps, ctx, meta, consume, |c| c.audit(audit)).await {
            Ok((user, cookie)) => {
                deps.login_throttle.record_success(&attempt, now).await;
                Ok(LoginOutcome::Authenticated {
                    user,
                    cookie,
                    recovery_codes_remaining: Some(remaining),
                })
            }
            Err(StoreError::Conflict(StoreConflict::RecoveryCodeInvalid)) => {
                deps.login_throttle.record_failure(&attempt, now).await;
                Err(IdentityError::InvalidCode)
            }
            Err(err) => Err(err.into()),
        }
    }
}

/// Idempotent: accepts a full or pending session, or none.
pub struct Logout {
    deps: IdentityDeps,
}

impl Logout {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cookie: Option<&str>) -> Result<(), IdentityError> {
        let Some(session) = AuthenticateSession::new(self.deps.clone())
            .find(cookie)
            .await?
        else {
            return Ok(());
        };
        let now = self.deps.clock.now();
        let changes = ChangeSet::new(session.user_id)
            .write(Write::EndSession {
                session_id: session.id,
                reason: EndReason::Logout,
                at: now,
            })
            .event(IdentityEvent::SessionsEnded {
                user_id: session.user_id,
                sessions: EndedSessions::One(session.id),
                reason: EndReason::Logout,
                at: now,
            });
        self.deps.store.commit(changes).await?;
        Ok(())
    }
}

pub struct StepUpCommand {
    pub password: SecretString,
    pub code: Option<String>,
    pub meta: RequestMeta,
}

/// Refreshes `authenticated_at` (15-minute rule). With TOTP enabled the code is mandatory.
pub struct StepUp {
    deps: IdentityDeps,
}

impl StepUp {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, ctx: &AuthContext, cmd: StepUpCommand) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let attempt = user_attempt(deps, &ctx.user, cmd.meta.ip, now).await?;
        check_throttle(deps, &attempt, now).await?;
        let mut changes = ChangeSet::new(ctx.user.id);
        let verified =
            proof_of_possession(deps, ctx, &cmd.password, cmd.code.as_deref(), now).await;
        match verified {
            Ok(consume) => changes.writes.extend(consume),
            Err(err) => {
                if matches!(
                    err,
                    IdentityError::ReauthFailed | IdentityError::InvalidCode
                ) {
                    deps.login_throttle.record_failure(&attempt, now).await;
                }
                return Err(err);
            }
        }
        let mut session = ctx.session.clone();
        session.reauthenticated(now);
        match deps
            .store
            .commit(changes.write(activity_write(&session)))
            .await
        {
            Ok(()) => {
                deps.login_throttle.record_success(&attempt, now).await;
                Ok(())
            }
            Err(StoreError::Conflict(StoreConflict::TotpStepReplayed)) => {
                deps.login_throttle.record_failure(&attempt, now).await;
                Err(IdentityError::InvalidCode)
            }
            Err(err) => Err(err.into()),
        }
    }
}

/// Password, plus the TOTP code when enabled. Returns the write that consumes the TOTP step.
/// A missing code for a TOTP user is `auth.invalid_code` (contract `stepUp`).
pub(crate) async fn proof_of_possession(
    deps: &IdentityDeps,
    ctx: &AuthContext,
    password: &SecretString,
    code: Option<&str>,
    now: chrono::DateTime<chrono::Utc>,
) -> Result<Option<Write>, IdentityError> {
    if password_matches(deps, &ctx.user, password).await? == PasswordCheck::Invalid {
        return Err(IdentityError::ReauthFailed);
    }
    let totp_enabled = deps
        .store
        .totp(ctx.user.id)
        .await?
        .is_some_and(|c| c.is_enabled());
    if !totp_enabled {
        return Ok(None);
    }
    let step = match code {
        Some(code) => totp_step(deps, ctx.user.id, code, now).await?,
        None => None,
    };
    let step = step.ok_or(IdentityError::InvalidCode)?;
    Ok(Some(Write::ConsumeTotpStep {
        user_id: ctx.user.id,
        step,
    }))
}
