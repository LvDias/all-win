//! Helpers shared by the use cases (throttled credential checks, sessions, audit records).

use std::net::IpAddr;

use chrono::{DateTime, Duration, Utc};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::SessionId;
use identity_enterprise::password::{self, NewPassword};
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::{ClientInfo, NewSession};
use identity_enterprise::totp;
use identity_enterprise::user::User;
use shared_kernel::UserId;

use crate::changes::{Actor, AuditAction, AuditRecord, Trail};
use crate::deps::{IdentityDeps, IssuedToken, RequestMeta};
use crate::error::IdentityError;
use crate::ports::PasswordCheck;
use crate::throttle::{LoginAttempt, ThrottleDecision};
use crate::views::SessionCookie;

/// "Usual IPs": a completed sign-in from the IP within this window (data model §4.1).
const USUAL_IP_WINDOW: Duration = Duration::days(30);

pub(crate) async fn login_attempt(
    deps: &IdentityDeps,
    email_normalized: String,
    user: Option<&User>,
    ip: IpAddr,
    now: DateTime<Utc>,
) -> Result<LoginAttempt, IdentityError> {
    let ip_is_usual = match user {
        Some(user) => {
            deps.store
                .signed_in_from(user.id, ip, now - USUAL_IP_WINDOW)
                .await?
        }
        None => false,
    };
    Ok(LoginAttempt {
        email_normalized,
        ip,
        user: user.map(|u| u.id),
        ip_is_usual,
    })
}

pub(crate) async fn user_attempt(
    deps: &IdentityDeps,
    user: &User,
    ip: IpAddr,
    now: DateTime<Utc>,
) -> Result<LoginAttempt, IdentityError> {
    login_attempt(
        deps,
        user.email.normalized().to_owned(),
        Some(user),
        ip,
        now,
    )
    .await
}

pub(crate) async fn check_throttle(
    deps: &IdentityDeps,
    attempt: &LoginAttempt,
    now: DateTime<Utc>,
) -> Result<(), IdentityError> {
    match deps.login_throttle.check(attempt, now).await {
        ThrottleDecision::Allow => Ok(()),
        ThrottleDecision::Limited { retry_after } => {
            Err(IdentityError::RateLimited { retry_after })
        }
    }
}

pub(crate) async fn check_public_throttle(
    deps: &IdentityDeps,
    ip: IpAddr,
    now: DateTime<Utc>,
) -> Result<(), IdentityError> {
    match deps.public_throttle.admit(ip, now).await {
        ThrottleDecision::Allow => Ok(()),
        ThrottleDecision::Limited { retry_after } => {
            Err(IdentityError::RateLimited { retry_after })
        }
    }
}

/// Verifies the user's password (NFKC-normalized like at hashing time).
pub(crate) async fn password_matches(
    deps: &IdentityDeps,
    user: &User,
    raw: &SecretString,
) -> Result<PasswordCheck, IdentityError> {
    deps.passwords
        .verify(&user.password_hash, &password::normalize(raw))
        .await
        .map_err(IdentityError::Internal)
}

/// Checks a TOTP code against the enabled credential; returns the matched step to consume.
pub(crate) async fn totp_step(
    deps: &IdentityDeps,
    user_id: UserId,
    code: &str,
    now: DateTime<Utc>,
) -> Result<Option<i64>, IdentityError> {
    let Some(credential) = deps.store.totp(user_id).await?.filter(|c| c.is_enabled()) else {
        return Ok(None);
    };
    let secret = deps
        .cipher
        .decrypt(&credential.secret_encrypted, credential.key_version)
        .map_err(IdentityError::Internal)?;
    Ok(totp::verify(&secret, code, now, credential.last_used_step))
}

pub(crate) fn validate_new_password(
    deps: &IdentityDeps,
    raw: &SecretString,
    email: &EmailAddress,
    pointer: &'static str,
) -> Result<NewPassword, IdentityError> {
    NewPassword::validate(raw, email, |candidate| deps.breached.contains(candidate))
        .map_err(|rejection| IdentityError::PasswordRejected { rejection, pointer })
}

pub(crate) fn new_session(
    deps: &IdentityDeps,
    user_id: UserId,
    persistent: bool,
    meta: &RequestMeta,
) -> (NewSession, IssuedToken) {
    let token = deps.issue_token();
    let new = NewSession {
        id: SessionId::new_v7(),
        user_id,
        token_hash: token.hash,
        persistent,
        client: ClientInfo {
            ip: Some(meta.ip),
            user_agent: meta.user_agent.clone(),
        },
    };
    (new, token)
}

pub(crate) fn cookie(
    token: &IssuedToken,
    persistent: bool,
    expires_at: DateTime<Utc>,
) -> SessionCookie {
    SessionCookie {
        value: token.value(),
        persistent,
        expires_at,
    }
}

pub(crate) fn user_audit(
    user_id: UserId,
    actor: Actor,
    action: AuditAction,
    at: DateTime<Utc>,
    ip: Option<IpAddr>,
) -> AuditRecord {
    AuditRecord {
        trail: Trail::User(user_id),
        actor,
        action,
        at,
        ip,
    }
}

pub(crate) fn admin_audit(
    admin: UserId,
    action: AuditAction,
    at: DateTime<Utc>,
    ip: Option<IpAddr>,
) -> AuditRecord {
    AuditRecord {
        trail: Trail::Admin(admin),
        actor: Actor::Admin(admin),
        action,
        at,
        ip,
    }
}

/// `https://<host>/<locale>/<path>#t=<token>`: the token travels in the fragment only.
pub(crate) fn fragment_link(
    deps: &IdentityDeps,
    locale: &str,
    path: &str,
    token: &IssuedToken,
) -> SecretString {
    SecretString::new(format!(
        "{}/{locale}/{path}#t={}",
        deps.settings.public_origin,
        token.value().expose()
    ))
}
