//! Instruction sources (S3 AC-3, Q17): named, several per user, one active key each.

use chrono::{DateTime, Utc};
use shared_kernel::UserId;
use thiserror::Error;

use crate::ids::SourceId;
use crate::tokens::KeyId;

pub const DISPLAY_NAME_MAX_CHARS: usize = 60;

/// Name shown in lists (`DisplayName`, Esquadro on Crivo L1): trimmed, then 1–60 characters, no
/// control character and no token-shaped part. The name is copied into audit details, which
/// refuse a token shape; accepting one here would let a source silence its own audit trail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayName(String);

#[derive(Debug, Error, PartialEq, Eq)]
#[error("display name must have 1 to 60 characters, no control character and no token")]
pub struct InvalidDisplayName;

impl DisplayName {
    pub fn parse(raw: &str) -> Result<Self, InvalidDisplayName> {
        let name = raw.trim();
        let chars = name.chars().count();
        if chars == 0
            || chars > DISPLAY_NAME_MAX_CHARS
            || name.chars().any(char::is_control)
            || contains_token_shape(name)
        {
            return Err(InvalidDisplayName);
        }
        Ok(Self(name.to_owned()))
    }

    pub fn from_stored(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Length of the `<key_id>` and `<secret>` parts of a token (ADR-0003 §7).
const TOKEN_KEY_LEN: usize = 12;
const TOKEN_SECRET_LEN: usize = 43;

/// True when `s` contains `aw(k|d)_[A-Za-z0-9]{12}_[A-Za-z0-9_-]{43}` anywhere (source or device
/// token shape).
fn contains_token_shape(s: &str) -> bool {
    let b = s.as_bytes();
    let key = |c: &u8| c.is_ascii_alphanumeric();
    let secret = |c: &u8| c.is_ascii_alphanumeric() || *c == b'_' || *c == b'-';
    let len = 4 + TOKEN_KEY_LEN + 1 + TOKEN_SECRET_LEN;
    b.windows(len).any(|w| {
        (w.starts_with(b"awk_") || w.starts_with(b"awd_"))
            && w[4..4 + TOKEN_KEY_LEN].iter().all(key)
            && w[4 + TOKEN_KEY_LEN] == b'_'
            && w[5 + TOKEN_KEY_LEN..].iter().all(secret)
    })
}

/// Input platform of a source. Only the type string reaches the Core's generic code; the
/// adapter that understands it lives in `core/adapters/*`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceType {
    TradingView,
}

impl SourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TradingView => "tradingview",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "tradingview" => Some(Self::TradingView),
            _ => None,
        }
    }

    /// Derived, never stored (contract `Source.is_automated`): instructions from automated sources
    /// require the per-account `automation` opt-in (O1).
    pub fn is_automated(self) -> bool {
        match self {
            Self::TradingView => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceStatus {
    Active,
    Revoked,
}

impl SourceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Revoked => "revoked",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub id: SourceId,
    pub user_id: UserId,
    pub source_type: SourceType,
    pub name: DisplayName,
    pub created_at: DateTime<Utc>,
    /// Revocation time (`identity.source.archived_at`).
    pub revoked_at: Option<DateTime<Utc>>,
    /// Key in force, or the last key of a revoked source.
    pub key_id: KeyId,
    /// Creation time of the current key when the source was rotated at least once.
    pub rotated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("the source is revoked")]
pub struct SourceRevoked;

impl Source {
    pub fn status(&self) -> SourceStatus {
        if self.revoked_at.is_some() {
            SourceStatus::Revoked
        } else {
            SourceStatus::Active
        }
    }

    /// A revoked source is a read-only history row: it keeps its name and cannot be rotated.
    pub fn ensure_active(&self) -> Result<(), SourceRevoked> {
        match self.status() {
            SourceStatus::Active => Ok(()),
            SourceStatus::Revoked => Err(SourceRevoked),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_name_counts_characters() {
        assert!(DisplayName::parse("").is_err());
        assert!(DisplayName::parse(&"é".repeat(60)).is_ok());
        assert!(DisplayName::parse(&"é".repeat(61)).is_err());
    }

    const TOKEN: &str = "awk_Abc123Def456_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNO-_";

    #[test]
    fn display_name_is_trimmed_and_never_blank() {
        assert_eq!(DisplayName::parse("  Main  ").unwrap().as_str(), "Main");
        assert!(DisplayName::parse("   ").is_err());
        assert!(DisplayName::parse(&format!(" {} ", "a".repeat(60))).is_ok());
    }

    #[test]
    fn display_name_refuses_control_characters() {
        assert!(DisplayName::parse("Main\nfeed").is_err());
        assert!(DisplayName::parse("Main\u{7f}").is_err());
        assert!(DisplayName::parse("Ma\u{85}in").is_err());
    }

    #[test]
    fn display_name_refuses_any_token_shaped_part() {
        assert_eq!(TOKEN.len(), 60);
        assert!(DisplayName::parse(TOKEN).is_err());
        assert!(DisplayName::parse(&TOKEN.replacen("awk_", "awd_", 1)).is_err());
        let inside = format!("x{}", &TOKEN[..59]);
        assert!(
            DisplayName::parse(&inside).is_ok(),
            "one char short is no token"
        );
        assert!(contains_token_shape(&format!("feed {TOKEN} copy")));
        assert!(!contains_token_shape("awk_Abc123Def456 is my key id"));
    }

    #[test]
    fn tradingview_is_automated() {
        assert!(SourceType::TradingView.is_automated());
        assert_eq!(
            SourceType::parse("tradingview"),
            Some(SourceType::TradingView)
        );
        assert_eq!(SourceType::parse("TradingView"), None);
    }
}
