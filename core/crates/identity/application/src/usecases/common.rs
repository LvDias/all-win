//! Helpers shared by the use cases (throttles, sessions, audit records).

use std::net::IpAddr;

use chrono::{DateTime, Utc};
use identity_enterprise::secret::SecretString;
use shared_kernel::UserId;

use crate::changes::{Actor, AuditAction, AuditRecord, Trail};
use crate::deps::{IdentityDeps, IssuedToken};
use crate::error::IdentityError;
use crate::throttle::ThrottleDecision;

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
