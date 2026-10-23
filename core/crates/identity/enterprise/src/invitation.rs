//! Invitations (Q6/Q19, auth policy §3): admin-issued, bound to one e-mail, single use, 7 days.

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;

use crate::ids::InvitationId;
use crate::profile::Locale;

pub const INVITATION_LIFETIME: Duration = Duration::days(7);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Revoked,
    Expired,
}

impl InvitationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invitation {
    pub id: InvitationId,
    /// Normalized e-mail the invitation is bound to.
    pub email: String,
    pub locale: Locale,
    pub issued_by: UserId,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub accepted_user_id: Option<UserId>,
    pub revoked_at: Option<DateTime<Utc>>,
    /// Closed because a new invitation was issued for the same e-mail after this one expired.
    pub superseded_at: Option<DateTime<Utc>>,
}

impl Invitation {
    pub fn issue(
        id: InvitationId,
        email: String,
        locale: Locale,
        issued_by: UserId,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            email,
            locale,
            issued_by,
            created_at: now,
            expires_at: now + INVITATION_LIFETIME,
            accepted_at: None,
            accepted_user_id: None,
            revoked_at: None,
            superseded_at: None,
        }
    }

    /// A superseded invitation had already expired, so it keeps showing as expired.
    pub fn status(&self, now: DateTime<Utc>) -> InvitationStatus {
        if self.accepted_at.is_some() {
            InvitationStatus::Accepted
        } else if self.revoked_at.is_some() {
            InvitationStatus::Revoked
        } else if now >= self.expires_at || self.superseded_at.is_some() {
            InvitationStatus::Expired
        } else {
            InvitationStatus::Pending
        }
    }

    /// Open = not accepted, revoked or superseded (what the unique index
    /// `invitation_open_email_key` covers), even when expired.
    pub fn is_open(&self) -> bool {
        self.accepted_at.is_none() && self.revoked_at.is_none() && self.superseded_at.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn status_lifecycle() {
        let t0 = Utc.with_ymd_and_hms(2026, 10, 23, 0, 0, 0).unwrap();
        let mut inv = Invitation::issue(
            InvitationId::new_v7(),
            "a@b.c".into(),
            Locale::En,
            UserId::new_v7(),
            t0,
        );
        assert_eq!(
            inv.status(t0 + Duration::days(6)),
            InvitationStatus::Pending
        );
        assert_eq!(
            inv.status(t0 + Duration::days(7)),
            InvitationStatus::Expired
        );
        assert!(inv.is_open());
        let mut superseded = inv.clone();
        superseded.superseded_at = Some(t0 + Duration::days(8));
        assert_eq!(
            superseded.status(t0 + Duration::days(8)),
            InvitationStatus::Expired
        );
        assert!(!superseded.is_open());
        inv.revoked_at = Some(t0);
        assert_eq!(inv.status(t0), InvitationStatus::Revoked);
        assert!(!inv.is_open());
    }
}
