//! `startTotpSetup` (step-up), `confirmTotpSetup`, `disableTotp`, `regenerateRecoveryCodes`
//! (A5, Q22). Enabling or disabling TOTP ends every other session of the user.

use chrono::{DateTime, Utc};
use identity_enterprise::codes::{CodeKind, NormalizedCode};
use identity_enterprise::ids::RecoveryBatchId;
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::EndReason;
use identity_enterprise::tokens::CodeHmac;
use identity_enterprise::totp::{self, TotpCredential};

use crate::auth::AuthContext;
use crate::changes::{Actor, AuditAction, ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{StoreConflict, StoreError};
use crate::throttle::LoginAttempt;
use crate::usecases::common::{check_throttle, user_attempt, user_audit};
use crate::usecases::signin::proof_of_possession;
use crate::views::{RecoveryCodesView, TotpSetupView};

pub const RECOVERY_CODES_PER_BATCH: usize = 10;

/// A fresh batch: display forms for the user, HMACs for storage (distinct within the batch).
pub(crate) fn recovery_batch(
    deps: &IdentityDeps,
) -> Result<(Vec<SecretString>, Vec<CodeHmac>), IdentityError> {
    let mut shown = Vec::with_capacity(RECOVERY_CODES_PER_BATCH);
    let mut stored: Vec<CodeHmac> = Vec::with_capacity(RECOVERY_CODES_PER_BATCH);
    while stored.len() < RECOVERY_CODES_PER_BATCH {
        let random: [u8; 7] = deps.random_bytes();
        let code = NormalizedCode::generate(CodeKind::Recovery, &random)
            .ok_or_else(|| IdentityError::internal("recovery code generation"))?;
        let hmac = deps.hasher.code_hmac(&code);
        if !stored.contains(&hmac) {
            stored.push(hmac);
            shown.push(code.display());
        }
    }
    Ok((shown, stored))
}

fn totp_changed_event(ctx: &AuthContext, at: DateTime<Utc>) -> IdentityEvent {
    IdentityEvent::SessionsEnded {
        user_id: ctx.user.id,
        sessions: EndedSessions::AllExcept(vec![ctx.session.id]),
        reason: EndReason::TotpChanged,
        at,
    }
}

fn end_other_sessions(ctx: &AuthContext, at: DateTime<Utc>) -> Write {
    Write::EndUserSessions {
        user_id: ctx.user.id,
        except: Some(ctx.session.id),
        reason: EndReason::TotpChanged,
        at,
    }
}

pub struct StartTotpSetup {
    deps: IdentityDeps,
}

impl StartTotpSetup {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Returns the secret once; an unconfirmed enrolment expires after 10 minutes and a new
    /// start replaces it.
    pub async fn handle(&self, ctx: &AuthContext) -> Result<TotpSetupView, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        ctx.require_step_up(now)?;
        if deps
            .store
            .totp(ctx.user.id)
            .await?
            .is_some_and(|c| c.is_enabled())
        {
            return Err(IdentityError::TotpAlreadyEnabled);
        }
        let secret: [u8; totp::SECRET_BYTES] = deps.random_bytes();
        let (secret_encrypted, key_version) = deps
            .cipher
            .encrypt(&secret)
            .map_err(IdentityError::Internal)?;
        let credential = TotpCredential {
            secret_encrypted,
            key_version,
            created_at: now,
            enabled_at: None,
            last_used_step: None,
        };
        let expires_at = credential.setup_expires_at();
        deps.store
            .commit(ChangeSet::new(ctx.user.id).write(Write::UpsertTotpSetup {
                user_id: ctx.user.id,
                credential,
            }))
            .await?;
        let base32 = totp::base32_secret(&secret);
        Ok(TotpSetupView {
            otpauth_uri: SecretString::new(totp::otpauth_uri(ctx.user.email.as_entered(), &base32)),
            secret: SecretString::new(base32),
            expires_at,
        })
    }
}

pub struct ConfirmTotpSetup {
    deps: IdentityDeps,
}

impl ConfirmTotpSetup {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        code: &str,
        meta: &RequestMeta,
    ) -> Result<RecoveryCodesView, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let attempt = user_attempt(deps, &ctx.user, meta.ip, now).await?;
        check_throttle(deps, &attempt, now).await?;
        let Some(credential) = deps
            .store
            .totp(ctx.user.id)
            .await?
            .filter(|c| c.setup_open(now))
        else {
            return Err(IdentityError::TotpSetupExpired);
        };
        let secret = deps
            .cipher
            .decrypt(&credential.secret_encrypted, credential.key_version)
            .map_err(IdentityError::Internal)?;
        let Some(step) = totp::verify(&secret, code, now, None) else {
            deps.login_throttle.record_failure(&attempt, now).await;
            return Err(IdentityError::InvalidCode);
        };
        let (shown, stored) = recovery_batch(deps)?;
        let changes = ChangeSet::new(ctx.user.id)
            .write(Write::EnableTotp {
                user_id: ctx.user.id,
                at: now,
                used_step: step,
            })
            .write(Write::ReplaceRecoveryCodes {
                user_id: ctx.user.id,
                batch_id: RecoveryBatchId::new_v7(),
                codes: stored,
                at: now,
            })
            .write(end_other_sessions(ctx, now))
            .audit(user_audit(
                ctx.user.id,
                Actor::User(ctx.user.id),
                AuditAction::TotpEnabled,
                now,
                Some(meta.ip),
            ))
            .event(totp_changed_event(ctx, now));
        match deps.store.commit(changes).await {
            Ok(()) => {
                deps.login_throttle.record_success(&attempt, now).await;
                Ok(RecoveryCodesView { codes: shown })
            }
            Err(StoreError::Conflict(StoreConflict::TotpNotPending)) => {
                Err(IdentityError::TotpSetupExpired)
            }
            Err(err) => Err(err.into()),
        }
    }
}

/// Password + current code for TOTP users. Returns the attempt so callers can record outcomes.
async fn verified_possession(
    deps: &IdentityDeps,
    ctx: &AuthContext,
    password: &SecretString,
    code: &str,
    meta: &RequestMeta,
) -> Result<(LoginAttempt, Option<Write>), IdentityError> {
    let now = deps.clock.now();
    let attempt = user_attempt(deps, &ctx.user, meta.ip, now).await?;
    check_throttle(deps, &attempt, now).await?;
    match proof_of_possession(deps, ctx, password, Some(code), now).await {
        Ok(consume) => Ok((attempt, consume)),
        Err(err) => {
            if matches!(
                err,
                IdentityError::ReauthFailed | IdentityError::InvalidCode
            ) {
                deps.login_throttle.record_failure(&attempt, now).await;
            }
            Err(err)
        }
    }
}

async fn commit_with_step(
    deps: &IdentityDeps,
    attempt: &LoginAttempt,
    changes: ChangeSet,
) -> Result<(), IdentityError> {
    let now = deps.clock.now();
    match deps.store.commit(changes).await {
        Ok(()) => {
            deps.login_throttle.record_success(attempt, now).await;
            Ok(())
        }
        Err(StoreError::Conflict(StoreConflict::TotpStepReplayed)) => {
            deps.login_throttle.record_failure(attempt, now).await;
            Err(IdentityError::InvalidCode)
        }
        Err(err) => Err(err.into()),
    }
}

pub struct DisableTotp {
    deps: IdentityDeps,
}

impl DisableTotp {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// With TOTP not enabled there is no code to check: `auth.invalid_code`.
    pub async fn handle(
        &self,
        ctx: &AuthContext,
        password: &SecretString,
        code: &str,
        meta: &RequestMeta,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let (attempt, consume) = verified_possession(deps, ctx, password, code, meta).await?;
        let Some(consume) = consume else {
            deps.login_throttle
                .record_failure(&attempt, deps.clock.now())
                .await;
            return Err(IdentityError::InvalidCode);
        };
        let now = deps.clock.now();
        let changes = ChangeSet::new(ctx.user.id)
            .write(consume)
            .write(Write::DeleteTotp {
                user_id: ctx.user.id,
            })
            .write(end_other_sessions(ctx, now))
            .audit(user_audit(
                ctx.user.id,
                Actor::User(ctx.user.id),
                AuditAction::TotpDisabled,
                now,
                Some(meta.ip),
            ))
            .event(totp_changed_event(ctx, now));
        commit_with_step(deps, &attempt, changes).await
    }
}

pub struct RegenerateRecoveryCodes {
    deps: IdentityDeps,
}

impl RegenerateRecoveryCodes {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        password: &SecretString,
        code: &str,
        meta: &RequestMeta,
    ) -> Result<RecoveryCodesView, IdentityError> {
        let deps = &self.deps;
        let (attempt, consume) = verified_possession(deps, ctx, password, code, meta).await?;
        let Some(consume) = consume else {
            deps.login_throttle
                .record_failure(&attempt, deps.clock.now())
                .await;
            return Err(IdentityError::InvalidCode);
        };
        let now = deps.clock.now();
        let (shown, stored) = recovery_batch(deps)?;
        let changes = ChangeSet::new(ctx.user.id)
            .write(consume)
            .write(Write::ReplaceRecoveryCodes {
                user_id: ctx.user.id,
                batch_id: RecoveryBatchId::new_v7(),
                codes: stored,
                at: now,
            })
            .audit(user_audit(
                ctx.user.id,
                Actor::User(ctx.user.id),
                AuditAction::RecoveryCodesRegenerated,
                now,
                Some(meta.ip),
            ));
        commit_with_step(deps, &attempt, changes).await?;
        Ok(RecoveryCodesView { codes: shown })
    }
}
