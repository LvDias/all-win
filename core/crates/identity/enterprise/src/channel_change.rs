//! A change of the e-mail or WhatsApp value (auth policy rev. 2 §7, HTTP 2.0.0 `ChannelChange`,
//! data model §4.3). Codes go to the current and to the new value under one flow; the change applies
//! only when every code is confirmed within 10 minutes. Requested from a device first seen less
//! than 7 days ago, it waits 72 hours on hold (M2, approved), cancellable from any session.

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;

use crate::channel::{Channel, Destination};
use crate::ids::{ChannelChangeId, SessionId};

/// Every code must be confirmed within this window.
pub const CHANGE_LIFETIME: Duration = Duration::minutes(10);
/// M2 hold (policy §7) and the §9 admin-recovery hold.
pub const HOLD: Duration = Duration::hours(72);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeStatus {
    AwaitingCodes,
    OnHold,
    Completed,
    Cancelled,
    Expired,
}

impl ChangeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingCodes => "awaiting_codes",
            Self::OnHold => "on_hold",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "awaiting_codes" => Self::AwaitingCodes,
            "on_hold" => Self::OnHold,
            "completed" => Self::Completed,
            "cancelled" => Self::Cancelled,
            "expired" => Self::Expired,
            _ => return None,
        })
    }

    /// Open changes block a new one (`channel_change_one_open`).
    pub fn is_open(self) -> bool {
        matches!(self, Self::AwaitingCodes | Self::OnHold)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelChange {
    pub id: ChannelChangeId,
    pub user_id: UserId,
    pub new_value: Destination,
    pub requested_session_id: SessionId,
    pub status: ChangeStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub hold_until: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    /// `None` with `cancelled_at` set: cancelled by the Core (generic reason, policy §3).
    pub cancelled_by_session_id: Option<SessionId>,
}

impl ChannelChange {
    pub fn start(
        id: ChannelChangeId,
        user_id: UserId,
        new_value: Destination,
        session: SessionId,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            new_value,
            requested_session_id: session,
            status: ChangeStatus::AwaitingCodes,
            created_at: now,
            expires_at: now + CHANGE_LIFETIME,
            hold_until: None,
            completed_at: None,
            cancelled_at: None,
            cancelled_by_session_id: None,
        }
    }

    pub fn channel(&self) -> Channel {
        self.new_value.channel()
    }

    /// Status as seen at `now`: an `awaiting_codes` change past its expiry is expired (set lazily
    /// in the database by the next writer).
    pub fn status_at(&self, now: DateTime<Utc>) -> ChangeStatus {
        if self.status == ChangeStatus::AwaitingCodes && now >= self.expires_at {
            ChangeStatus::Expired
        } else {
            self.status
        }
    }

    /// When a hold applies (a worker applies due holds).
    pub fn hold_is_due(&self, now: DateTime<Utc>) -> bool {
        self.status == ChangeStatus::OnHold && self.hold_until.is_some_and(|h| now >= h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::email::EmailAddress;
    use chrono::TimeZone;

    #[test]
    fn expiry_is_lazy_and_holds_become_due() {
        let t0 = Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap();
        let mut c = ChannelChange::start(
            ChannelChangeId::new_v7(),
            UserId::new_v7(),
            Destination::Email(EmailAddress::parse("new@example.com").unwrap()),
            SessionId::new_v7(),
            t0,
        );
        assert_eq!(c.status_at(t0), ChangeStatus::AwaitingCodes);
        assert_eq!(
            c.status_at(t0 + Duration::minutes(10)),
            ChangeStatus::Expired
        );
        c.status = ChangeStatus::OnHold;
        c.hold_until = Some(t0 + HOLD);
        assert!(!c.hold_is_due(t0 + Duration::hours(71)));
        assert!(c.hold_is_due(t0 + Duration::hours(72)));
        assert!(c.status.is_open());
        assert_eq!(c.channel(), Channel::Email);
    }
}
