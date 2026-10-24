//! Shared mechanics of one-time codes (auth policy rev. 2 §2–§4): generation, the sealed challenge
//! token, the send limits and the single verification path used by every purpose.

use std::net::IpAddr;
use std::time::Duration as StdDuration;

use chrono::{DateTime, Duration, Utc};
use identity_enterprise::codes::{CodeKind, NormalizedCode};
use identity_enterprise::ids::CodeId;
use identity_enterprise::otp::{
    CODE_LIFETIME, ChallengeClaims, CodeState, DeviceClass, MAX_CODE_INPUT_CHARS, Purpose,
    RESEND_COOLDOWN,
};
use identity_enterprise::secret::SecretString;
use identity_enterprise::tokens::CodeHmac;
use shared_kernel::UserId;

use crate::changes::{Actor, AuditAction, ChangeSet, CodeTable, Write};
use crate::deps::IdentityDeps;
use crate::error::IdentityError;
use crate::ports::WindowCount;
use crate::throttle::ThrottleDecision;
use crate::usecases::common::user_audit;
use crate::views::ChallengeIssued;

/// Durable per-identifier windows and their limits (policy §4, separate per device class):
/// one send per 60 s, 5 per 15 min, 20 per 24 h.
pub(crate) const SEND_WINDOWS: [(i32, i32); 3] = [(60, 1), (900, 5), (86_400, 20)];
/// Step-up and channel-change sends per signed-in user per hour.
pub(crate) const USER_SENDS_PER_HOUR: i32 = 10;
const USER_SEND_WINDOW: i32 = 3600;

/// A fresh code: the plaintext for the delivery and its HMAC for the row.
pub(crate) struct FreshCode {
    pub plain: SecretString,
    pub hmac: CodeHmac,
}

pub(crate) fn fresh_code(deps: &IdentityDeps, code_id: CodeId) -> Result<FreshCode, IdentityError> {
    let random: [u8; 5] = deps.random_bytes();
    let code = NormalizedCode::generate(CodeKind::OneTime, &random)
        .ok_or_else(|| IdentityError::internal("code generation"))?;
    Ok(FreshCode {
        plain: code.display(),
        hmac: deps.hasher.otp_hmac(code_id, &code),
    })
}

/// The identifier HMAC used by the send limits and carried in the challenge token.
pub(crate) fn identifier(deps: &IdentityDeps, normalized: &str) -> CodeHmac {
    deps.hasher.identifier_hmac(normalized)
}

pub(crate) fn seal(
    deps: &IdentityDeps,
    code_id: CodeId,
    purpose: Purpose,
    identifier: &CodeHmac,
    device_class: DeviceClass,
    issued_at: DateTime<Utc>,
) -> SecretString {
    deps.hasher.seal_challenge(&ChallengeClaims {
        code_id,
        purpose,
        identifier_hmac: identifier.bytes,
        pepper_version: identifier.pepper_version,
        device_class,
        issued_at,
    })
}

pub(crate) fn issued(challenge_id: SecretString, now: DateTime<Utc>) -> ChallengeIssued {
    ChallengeIssued {
        challenge_id,
        expires_at: now + CODE_LIFETIME,
        resend_after_s: RESEND_COOLDOWN.num_seconds(),
    }
}

fn retry_after(seconds: i64) -> StdDuration {
    StdDuration::from_secs(u64::try_from(seconds.max(1)).unwrap_or(1))
}

/// `Retry-After` for a window over its limit: until the window ends.
fn retry_after_window(c: &WindowCount, now: DateTime<Utc>) -> StdDuration {
    let end = c.window_start + Duration::seconds(i64::from(c.window_seconds));
    retry_after((end - now).num_seconds())
}

/// One send against every limit that applies to it: per IP and prefix (memory), then per
/// identifier and device class (durable, identical for unknown identifiers). Counts the send
/// only when it is allowed, atomically with the decision.
pub(crate) async fn admit_send(
    deps: &IdentityDeps,
    ip: IpAddr,
    identifier: &CodeHmac,
    class: DeviceClass,
    now: DateTime<Utc>,
) -> Result<(), IdentityError> {
    if let ThrottleDecision::Limited { retry_after } = deps.code_throttle.admit_send(ip, now).await
    {
        return Err(IdentityError::RateLimited { retry_after });
    }
    match deps
        .store
        .count_sends_within(identifier, class, &SEND_WINDOWS, 1, now)
        .await?
    {
        Some(over) => Err(IdentityError::RateLimited {
            retry_after: retry_after_window(&over, now),
        }),
        None => Ok(()),
    }
}

/// Per-user hourly limit of step-up and channel-change sends (`needed` sends at once), counted
/// like the per-identifier limits under a key no identifier can produce (a NUL never appears in
/// a normalized e-mail or phone number). Fixed hourly windows, as the per-identifier ones.
pub(crate) async fn admit_user_sends(
    deps: &IdentityDeps,
    user: UserId,
    needed: i32,
    now: DateTime<Utc>,
) -> Result<(), IdentityError> {
    let key = identifier(deps, &format!("\0user\0{}", user.as_uuid()));
    match deps
        .store
        .count_sends_within(
            &key,
            DeviceClass::Known,
            &[(USER_SEND_WINDOW, USER_SENDS_PER_HOUR)],
            needed,
            now,
        )
        .await?
    {
        Some(over) => Err(IdentityError::RateLimited {
            retry_after: retry_after_window(&over, now),
        }),
        None => Ok(()),
    }
}

/// Audits a rate-limited code request of a registered user (policy §11 audit tests).
pub(crate) async fn audit_rate_limited(
    deps: &IdentityDeps,
    user: UserId,
    purpose: Purpose,
    ip: IpAddr,
    now: DateTime<Utc>,
) -> Result<(), IdentityError> {
    let changes = ChangeSet::new(user).audit(user_audit(
        user,
        Actor::User(user),
        AuditAction::CodeRateLimited { purpose },
        now,
        Some(ip),
    ));
    deps.store.commit(changes).await?;
    Ok(())
}

/// Opens a presented challenge token. `None` for anything forged, altered or malformed.
pub(crate) fn open(deps: &IdentityDeps, challenge_id: &str) -> Option<ChallengeClaims> {
    deps.hasher.open_challenge(challenge_id)
}

/// Outcome of comparing a typed code with a challenge row.
pub(crate) enum Check {
    /// Matches; consume with this HMAC (it guards against a concurrent resend).
    Match(CodeHmac),
    /// Wrong or malformed: one counted attempt.
    Wrong,
    /// The row cannot accept codes (closed, expired): no attempt is counted, same answer.
    Closed,
}

/// Normalizes the input (U16 format 1) and compares it in constant time with the row's HMAC under
/// the row's pepper version.
pub(crate) fn check(
    deps: &IdentityDeps,
    code_id: CodeId,
    state: &CodeState,
    typed: &str,
    now: DateTime<Utc>,
) -> Check {
    if !state.is_open(now) {
        return Check::Closed;
    }
    if typed.chars().count() > MAX_CODE_INPUT_CHARS {
        return Check::Closed;
    }
    let Some(code) = NormalizedCode::parse(CodeKind::OneTime, typed) else {
        return Check::Wrong;
    };
    match deps
        .hasher
        .otp_hmac_with(code_id, &code, state.code_hmac.pepper_version)
    {
        Some(h) if deps.hasher.mac_eq(&h, &state.code_hmac) => Check::Match(state.code_hmac),
        _ => Check::Wrong,
    }
}

/// Commits one wrong attempt on its own (never together with a success) and counts it per IP.
pub(crate) async fn record_wrong(
    deps: &IdentityDeps,
    scope: UserId,
    table: CodeTable,
    code_id: CodeId,
    audit: Option<crate::changes::AuditRecord>,
    ip: IpAddr,
    now: DateTime<Utc>,
) -> Result<(), IdentityError> {
    deps.code_throttle.record_wrong(ip, now).await;
    let mut changes = ChangeSet::new(scope).write(Write::FailCode { table, code_id });
    if let Some(a) = audit {
        changes = changes.audit(a);
    }
    deps.store.commit(changes).await?;
    Ok(())
}

/// Per-IP wrong-attempt limit before trying a code.
pub(crate) async fn admit_verify(
    deps: &IdentityDeps,
    ip: IpAddr,
    now: DateTime<Utc>,
) -> Result<(), IdentityError> {
    match deps.code_throttle.admit_verify(ip, now).await {
        ThrottleDecision::Allow => Ok(()),
        ThrottleDecision::Limited { retry_after } => {
            Err(IdentityError::RateLimited { retry_after })
        }
    }
}

/// Maps commit conflicts of a code consume to the single verification error.
pub(crate) fn consume_conflict(e: crate::ports::StoreError) -> IdentityError {
    use crate::ports::{StoreConflict as C, StoreError as E};
    match e {
        E::Conflict(C::CodeGone | C::InvitationGone | C::EmailTaken | C::WhatsappTaken) => {
            IdentityError::CodeInvalid
        }
        E::Conflict(C::ChannelChangeGone) => IdentityError::ChannelChangeClosed,
        other => other.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn retry_after_comes_from_the_full_window() {
        let now = Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 30).unwrap();
        let start = Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap();
        let over = WindowCount {
            window_seconds: 60,
            window_start: start,
            sends: 2,
        };
        assert_eq!(retry_after_window(&over, now), StdDuration::from_secs(30));
    }
}
