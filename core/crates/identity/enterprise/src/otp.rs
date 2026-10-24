//! One-time codes (auth policy rev. 2 §2–§4, data model §4.1/§4.3): sign-up, login, step-up and
//! channel-change challenges. A challenge is identified by a sealed token that only the requester
//! holds; the code is 8 Crockford Base32 characters (U16 format 1, see [`crate::codes`]), stored
//! only as an HMAC, valid 10 minutes, invalidated after 5 wrong attempts.

use std::fmt;

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;
use uuid::Uuid;

use crate::channel::Destination;
use crate::ids::{ChannelChangeId, CodeId, InvitationId, SessionId};
use crate::tokens::CodeHmac;

pub const CODE_LIFETIME: Duration = Duration::minutes(10);
pub const MAX_WRONG_ATTEMPTS: i16 = 5;
/// `resend_after_s` after every send; also the per-identifier cooldown window.
pub const RESEND_COOLDOWN: Duration = Duration::seconds(60);
/// More than this many characters is a schema error (400), never a counted attempt.
pub const MAX_CODE_INPUT_CHARS: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Purpose {
    Signup,
    Login,
    StepUp,
    ChannelChange,
}

impl Purpose {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Signup => "signup",
            Self::Login => "login",
            Self::StepUp => "step_up",
            Self::ChannelChange => "channel_change",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "signup" => Self::Signup,
            "login" => Self::Login,
            "step_up" => Self::StepUp,
            "channel_change" => Self::ChannelChange,
            _ => return None,
        })
    }

    pub fn as_byte(self) -> u8 {
        match self {
            Self::Signup => 1,
            Self::Login => 2,
            Self::StepUp => 3,
            Self::ChannelChange => 4,
        }
    }

    fn from_byte(b: u8) -> Option<Self> {
        Some(match b {
            1 => Self::Signup,
            2 => Self::Login,
            3 => Self::StepUp,
            4 => Self::ChannelChange,
            _ => return None,
        })
    }
}

/// HTTP 2.0.0 `StepUpAction`, shared with other modules through the access port (ADR-0007).
pub use access::StepUpAction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CodeRole {
    CurrentValue,
    NewValue,
    OtherChannel,
}

impl CodeRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CurrentValue => "current_value",
            Self::NewValue => "new_value",
            Self::OtherChannel => "other_channel",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "current_value" => Self::CurrentValue,
            "new_value" => Self::NewValue,
            "other_channel" => Self::OtherChannel,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryState {
    Pending,
    Sent,
    Failed,
}

impl DeliveryState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Sent => "sent",
            Self::Failed => "failed",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "pending" => Self::Pending,
            "sent" => Self::Sent,
            "failed" => Self::Failed,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidReason {
    Attempts,
    ChannelChanged,
    ChangeClosed,
    UserClosed,
    InvitationClosed,
}

impl InvalidReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Attempts => "attempts",
            Self::ChannelChanged => "channel_changed",
            Self::ChangeClosed => "change_closed",
            Self::UserClosed => "user_closed",
            Self::InvitationClosed => "invitation_closed",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "attempts" => Self::Attempts,
            "channel_changed" => Self::ChannelChanged,
            "change_closed" => Self::ChangeClosed,
            "user_closed" => Self::UserClosed,
            "invitation_closed" => Self::InvitationClosed,
            _ => return None,
        })
    }
}

/// Lifecycle shared by `one_time_code` and `signup_code`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeState {
    pub code_hmac: CodeHmac,
    pub failed_attempts: i16,
    pub created_at: DateTime<Utc>,
    pub sent_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub consumed_at: Option<DateTime<Utc>>,
    pub invalidated_at: Option<DateTime<Utc>>,
    pub invalidated_reason: Option<InvalidReason>,
    pub delivery_state: DeliveryState,
}

impl CodeState {
    pub fn issue(code_hmac: CodeHmac, now: DateTime<Utc>) -> Self {
        Self {
            code_hmac,
            failed_attempts: 0,
            created_at: now,
            sent_at: now,
            expires_at: now + CODE_LIFETIME,
            consumed_at: None,
            invalidated_at: None,
            invalidated_reason: None,
            delivery_state: DeliveryState::Pending,
        }
    }

    /// Neither consumed nor invalidated and not expired at `now`.
    pub fn is_open(&self, now: DateTime<Utc>) -> bool {
        self.consumed_at.is_none() && self.invalidated_at.is_none() && now < self.expires_at
    }

    /// Resend: a new code for the same challenge. Wrong attempts are kept (policy §2.2).
    pub fn resent(&self, code_hmac: CodeHmac, now: DateTime<Utc>) -> Self {
        Self {
            code_hmac,
            sent_at: now,
            expires_at: now + CODE_LIFETIME,
            delivery_state: DeliveryState::Pending,
            ..self.clone()
        }
    }
}

/// Login, step-up and channel-change code (`identity.one_time_code`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OneTimeCode {
    pub id: CodeId,
    pub user_id: UserId,
    pub purpose: Purpose,
    pub destination: Destination,
    /// Step-up: what the code authorizes, and on which resource.
    pub action: Option<StepUpAction>,
    pub target_id: Option<Uuid>,
    /// Step-up and channel change: only this session may use the code.
    pub session_id: Option<SessionId>,
    pub channel_change_id: Option<ChannelChangeId>,
    pub role: Option<CodeRole>,
    pub request_ip: Option<std::net::IpAddr>,
    pub state: CodeState,
}

/// Sign-up code (`identity.signup_code`): no user exists yet; the destination is the invited e-mail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignupCode {
    pub id: CodeId,
    pub invitation_id: InvitationId,
    pub request_ip: Option<std::net::IpAddr>,
    pub state: CodeState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceClass {
    Known,
    Unknown,
}

impl DeviceClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Known => "known",
            Self::Unknown => "unknown",
        }
    }
}

/// What the sealed `challenge_id` carries (policy §3): the code row id, the purpose, the identifier
/// HMAC and device class used by the send limits, and when it was issued. The Core MACs these bytes
/// with `k_challenge`; an unknown identifier gets the same token with no row behind it.
#[derive(Clone, PartialEq, Eq)]
pub struct ChallengeClaims {
    pub code_id: CodeId,
    pub purpose: Purpose,
    pub identifier_hmac: [u8; 32],
    pub pepper_version: i16,
    pub device_class: DeviceClass,
    pub issued_at: DateTime<Utc>,
}

const CLAIMS_VERSION: u8 = 1;
/// version ‖ purpose ‖ uuid ‖ identifier HMAC ‖ pepper version ‖ device class ‖ issued_at.
pub const CLAIMS_LEN: usize = 1 + 1 + 16 + 32 + 2 + 1 + 8;

impl ChallengeClaims {
    pub fn to_bytes(&self) -> [u8; CLAIMS_LEN] {
        let mut out = [0u8; CLAIMS_LEN];
        out[0] = CLAIMS_VERSION;
        out[1] = self.purpose.as_byte();
        out[2..18].copy_from_slice(self.code_id.as_uuid().as_bytes());
        out[18..50].copy_from_slice(&self.identifier_hmac);
        out[50..52].copy_from_slice(&self.pepper_version.to_be_bytes());
        out[52] = match self.device_class {
            DeviceClass::Known => 1,
            DeviceClass::Unknown => 0,
        };
        out[53..61].copy_from_slice(&self.issued_at.timestamp().to_be_bytes());
        out
    }

    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() != CLAIMS_LEN || b[0] != CLAIMS_VERSION {
            return None;
        }
        let uuid = Uuid::from_slice(&b[2..18]).ok()?;
        let mut identifier_hmac = [0u8; 32];
        identifier_hmac.copy_from_slice(&b[18..50]);
        let device_class = match b[52] {
            1 => DeviceClass::Known,
            0 => DeviceClass::Unknown,
            _ => return None,
        };
        let mut ts = [0u8; 8];
        ts.copy_from_slice(&b[53..61]);
        Some(Self {
            code_id: CodeId::from_uuid(uuid),
            purpose: Purpose::from_byte(b[1])?,
            identifier_hmac,
            pepper_version: i16::from_be_bytes([b[50], b[51]]),
            device_class,
            issued_at: DateTime::from_timestamp(i64::from_be_bytes(ts), 0)?,
        })
    }
}

impl fmt::Debug for ChallengeClaims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChallengeClaims({:?}, REDACTED)", self.purpose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn t0() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap()
    }

    #[test]
    fn claims_round_trip_and_reject_garbage() {
        let c = ChallengeClaims {
            code_id: CodeId::new_v7(),
            purpose: Purpose::Login,
            identifier_hmac: [9; 32],
            pepper_version: 3,
            device_class: DeviceClass::Known,
            issued_at: t0(),
        };
        let b = c.to_bytes();
        assert_eq!(ChallengeClaims::from_bytes(&b), Some(c.clone()));
        let mut bad = b;
        bad[0] = 9;
        assert!(ChallengeClaims::from_bytes(&bad).is_none());
        assert!(ChallengeClaims::from_bytes(&b[1..]).is_none());
        assert!(!format!("{c:?}").contains("9, 9"));
    }

    #[test]
    fn lifecycle_and_resend_keep_attempts() {
        let s = CodeState::issue(
            CodeHmac {
                bytes: [1; 32],
                pepper_version: 1,
            },
            t0(),
        );
        assert!(s.is_open(t0() + Duration::minutes(9)));
        assert!(!s.is_open(t0() + Duration::minutes(10)));
        let mut tried = s.clone();
        tried.failed_attempts = 3;
        let r = tried.resent(
            CodeHmac {
                bytes: [2; 32],
                pepper_version: 1,
            },
            t0() + Duration::minutes(5),
        );
        assert_eq!(r.failed_attempts, 3);
        assert_eq!(r.expires_at, t0() + Duration::minutes(15));
        assert_eq!(r.created_at, t0());
    }

    #[test]
    fn step_up_action_table() {
        for a in StepUpAction::ALL {
            assert_eq!(StepUpAction::parse(a.as_str()), Some(a));
        }
        assert!(StepUpAction::SourceRotate.requires_target());
        assert!(!StepUpAction::SourceCreate.requires_target());
        assert!(!StepUpAction::ConnectionUnpair.is_q35_conditional());
        assert!(!StepUpAction::AccountOptInAccept.is_q35_conditional());
        assert!(StepUpAction::RouteEnable.is_q35_conditional());
    }
}
