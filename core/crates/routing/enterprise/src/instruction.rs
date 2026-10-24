//! Accepted instruction identity and timing.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

macro_rules! id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
            ::serde::Serialize, ::serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(::uuid::Uuid);

        impl $name {
            pub fn new_v7() -> Self {
                Self(::uuid::Uuid::now_v7())
            }

            pub const fn from_uuid(id: ::uuid::Uuid) -> Self {
                Self(id)
            }

            pub const fn as_uuid(&self) -> ::uuid::Uuid {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}
pub(crate) use id;

id!(
    /// Core-assigned id of an accepted instruction (UUIDv7).
    InstructionId
);
id!(
    /// Instruction source owned by `identity`; routing keeps the plain id.
    SourceId
);

/// Source-assigned id, unique per user forever (ADR-0003 §5).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct MessageId(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid message_id")]
pub struct InvalidMessageId;

impl MessageId {
    pub fn parse(value: impl Into<String>) -> Result<Self, InvalidMessageId> {
        let value = value.into();
        let valid = (1..=128).contains(&value.len())
            && value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b':' | b'-'));
        if valid {
            Ok(Self(value))
        } else {
            Err(InvalidMessageId)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for MessageId {
    type Error = InvalidMessageId;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<MessageId> for String {
    fn from(value: MessageId) -> Self {
        value.0
    }
}

/// When the source says it sent the instruction and when the Core received it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntakeTimes {
    pub sent_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
}

impl IntakeTimes {
    /// `min(sent_at, received_at)` (ADR-0003 §7): a delayed webhook expires like a
    /// delayed delivery, and a future `sent_at` (clock skew) never extends the TTL.
    pub fn expiry_base(&self) -> DateTime<Utc> {
        self.sent_at.min(self.received_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn at(s: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 23, 12, 0, s).unwrap()
    }

    #[test]
    fn expiry_base_is_the_earlier_instant() {
        let late = IntakeTimes {
            sent_at: at(0),
            received_at: at(5),
        };
        assert_eq!(late.expiry_base(), at(0));
        let skewed = IntakeTimes {
            sent_at: at(9),
            received_at: at(5),
        };
        assert_eq!(skewed.expiry_base(), at(5));
    }

    #[test]
    fn message_id_follows_the_contract_pattern() {
        assert!(MessageId::parse("tv-MNQ-trigger-1792685000000").is_ok());
        assert!(MessageId::parse("a.b_c:d-e").is_ok());
        assert!(MessageId::parse("").is_err());
        assert!(MessageId::parse("has space").is_err());
        assert!(MessageId::parse("x".repeat(129)).is_err());
    }
}
