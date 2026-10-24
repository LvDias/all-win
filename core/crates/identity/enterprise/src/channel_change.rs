//! A change of the e-mail or WhatsApp value (auth policy rev. 2 §7, HTTP 2.0.0 `ChannelChange`,
//! data model §4.3). Codes go to the current and to the new value under one flow; the change applies
//! only when every code is confirmed within 10 minutes. Requested from a device first seen less
//! than 7 days ago, it waits 72 hours on hold (M2, approved), cancellable from any session.
//! An admin recovery (AC-12, policy §9) uses the same row: started and confirmed by a platform
//! admin with the operator command, it always waits 72 hours on hold.

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;

use crate::channel::{Channel, Destination};
use crate::ids::{ChannelChangeId, SessionId};
use crate::source::contains_token_shape;

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

/// HTTP 5.4.0 `ChannelChange.kind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeKind {
    UserChange,
    AdminRecovery,
}

impl ChangeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserChange => "user_change",
            Self::AdminRecovery => "admin_recovery",
        }
    }
}

/// How the admin verified the user's identity out of band (policy §9).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerifiedBy {
    VideoCall,
    PhoneKnownNumber,
    InPerson,
    Other,
}

impl VerifiedBy {
    pub const ALL: [Self; 4] = [
        Self::VideoCall,
        Self::PhoneKnownNumber,
        Self::InPerson,
        Self::Other,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::VideoCall => "video_call",
            Self::PhoneKnownNumber => "phone_known_number",
            Self::InPerson => "in_person",
            Self::Other => "other",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|v| v.as_str() == raw)
    }
}

pub const VERIFICATION_REF_MAX_CHARS: usize = 200;
/// A digit run this long looks like a phone or document number.
const MAX_DIGIT_RUN: usize = 6;

/// The admin's reference for the verification (a ticket, a call time): trimmed, 1–200 characters
/// of plain text. It is shown in the user's audit, so it refuses control characters, token-shaped
/// parts, e-mail addresses, `+` before a digit and runs of more than 6 digits (Lastro: never the
/// new e-mail or phone, never a document number). Names or other personal data of third parties
/// cannot be detected here; the operator command's help forbids them (Crivo n4).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationRef(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("verification reference must be 1 to 200 characters of plain text without contact data")]
pub struct InvalidVerificationRef;

impl VerificationRef {
    pub fn parse(raw: &str) -> Result<Self, InvalidVerificationRef> {
        let text = raw.trim();
        let chars = text.chars().count();
        let mut run = 0;
        let mut longest = 0;
        for c in text.chars() {
            run = if c.is_ascii_digit() { run + 1 } else { 0 };
            longest = longest.max(run);
        }
        if chars == 0
            || chars > VERIFICATION_REF_MAX_CHARS
            || text.chars().any(char::is_control)
            || text.contains('@')
            || text
                .as_bytes()
                .windows(2)
                .any(|w| w[0] == b'+' && w[1].is_ascii_digit())
            || longest > MAX_DIGIT_RUN
            || contains_token_shape(text)
        {
            return Err(InvalidVerificationRef);
        }
        Ok(Self(text.to_owned()))
    }

    pub fn from_stored(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// What an admin recovery adds to the change (data model rev. 16.8).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recovery {
    /// The admin who acted; the target user themself on break-glass.
    pub admin_user_id: UserId,
    pub verified_by: VerifiedBy,
    pub verification_ref: VerificationRef,
    /// The operator's break-glass path (I3): SSH only, no admin step-up.
    pub break_glass: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelChange {
    pub id: ChannelChangeId,
    pub user_id: UserId,
    pub new_value: Destination,
    /// The user's session that started it; `None` for an admin recovery.
    pub requested_session_id: Option<SessionId>,
    /// `Some` for an admin recovery.
    pub recovery: Option<Recovery>,
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
            requested_session_id: Some(session),
            recovery: None,
            status: ChangeStatus::AwaitingCodes,
            created_at: now,
            expires_at: now + CHANGE_LIFETIME,
            hold_until: None,
            completed_at: None,
            cancelled_at: None,
            cancelled_by_session_id: None,
        }
    }

    /// An admin recovery of the e-mail: the code goes to the new value only, and the admin
    /// confirms it with the operator command.
    pub fn start_recovery(
        id: ChannelChangeId,
        user_id: UserId,
        new_value: Destination,
        recovery: Recovery,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            new_value,
            requested_session_id: None,
            recovery: Some(recovery),
            status: ChangeStatus::AwaitingCodes,
            created_at: now,
            expires_at: now + CHANGE_LIFETIME,
            hold_until: None,
            completed_at: None,
            cancelled_at: None,
            cancelled_by_session_id: None,
        }
    }

    pub fn kind(&self) -> ChangeKind {
        if self.recovery.is_some() {
            ChangeKind::AdminRecovery
        } else {
            ChangeKind::UserChange
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
        assert_eq!(c.kind(), ChangeKind::UserChange);
    }

    #[test]
    fn a_recovery_has_no_session_and_its_kind() {
        let t0 = Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap();
        let admin = UserId::new_v7();
        let c = ChannelChange::start_recovery(
            ChannelChangeId::new_v7(),
            UserId::new_v7(),
            Destination::Email(EmailAddress::parse("new@example.com").unwrap()),
            Recovery {
                admin_user_id: admin,
                verified_by: VerifiedBy::VideoCall,
                verification_ref: VerificationRef::parse("ticket 4521").unwrap(),
                break_glass: false,
            },
            t0,
        );
        assert_eq!(c.kind(), ChangeKind::AdminRecovery);
        assert_eq!(c.requested_session_id, None);
        assert_eq!(c.status_at(t0 + CHANGE_LIFETIME), ChangeStatus::Expired);
    }

    #[test]
    fn verified_by_round_trips() {
        for v in VerifiedBy::ALL {
            assert_eq!(VerifiedBy::parse(v.as_str()), Some(v));
        }
        assert_eq!(VerifiedBy::parse("email"), None);
    }

    #[test]
    fn verification_ref_is_plain_text_without_contact_data() {
        assert_eq!(
            VerificationRef::parse("  call 2026-10-23 14:00, ticket 4521 ")
                .unwrap()
                .as_str(),
            "call 2026-10-23 14:00, ticket 4521"
        );
        assert!(VerificationRef::parse(&"a".repeat(200)).is_ok());
        let token = format!("x awk_{}_{}", "A".repeat(12), "b".repeat(43));
        for bad in [
            "",
            "   ",
            &"a".repeat(201),
            "line\nbreak",
            "tab\there",
            "j@example.com",
            "+5511",
            "cpf 1234567",
            token.as_str(),
        ] {
            assert_eq!(
                VerificationRef::parse(bad),
                Err(InvalidVerificationRef),
                "{bad:?}"
            );
        }
    }
}
