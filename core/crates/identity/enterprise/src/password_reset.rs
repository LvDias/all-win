//! Password reset tokens (Q20, auth policy §3): single use, 30 minutes, newest wins.

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;

use crate::ids::PasswordResetId;
use crate::tokens::TokenHash;

pub const RESET_LIFETIME: Duration = Duration::minutes(30);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PasswordReset {
    pub id: PasswordResetId,
    pub user_id: UserId,
    pub token_hash: TokenHash,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub invalidated_at: Option<DateTime<Utc>>,
}

impl PasswordReset {
    pub fn issue(
        id: PasswordResetId,
        user_id: UserId,
        token_hash: TokenHash,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            token_hash,
            created_at: now,
            expires_at: now + RESET_LIFETIME,
            used_at: None,
            invalidated_at: None,
        }
    }

    /// Unknown, used, expired and superseded all collapse into "not usable" (one error code).
    pub fn is_usable(&self, now: DateTime<Utc>) -> bool {
        self.used_at.is_none() && self.invalidated_at.is_none() && now < self.expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn usable_only_once_and_within_30_minutes() {
        let t0 = Utc.with_ymd_and_hms(2026, 10, 23, 0, 0, 0).unwrap();
        let mut r = PasswordReset::issue(
            PasswordResetId::new_v7(),
            UserId::new_v7(),
            TokenHash::from_bytes([1; 32]),
            t0,
        );
        assert!(r.is_usable(t0 + Duration::minutes(29)));
        assert!(!r.is_usable(t0 + Duration::minutes(30)));
        r.used_at = Some(t0);
        assert!(!r.is_usable(t0));
        r.used_at = None;
        r.invalidated_at = Some(t0);
        assert!(!r.is_usable(t0));
    }
}
