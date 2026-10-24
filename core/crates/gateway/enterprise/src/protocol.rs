//! Protocol version negotiation (ADR-0004 §3 step 4, §9). The Core speaks 1.2–1.3; an adapter
//! that offers nothing in that range is refused with 4003 and shown as outdated.

use std::fmt;

/// `major.minor` of the adapter protocol. Only major 1 exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtocolVersion {
    major: u16,
    minor: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VersionError {
    #[error("invalid protocol version")]
    Invalid,
    /// Close 4003; the connection shows as `outdated`.
    #[error("no supported protocol version in common")]
    Unsupported,
}

impl ProtocolVersion {
    pub const V1_0: Self = Self::new(1, 0);
    pub const V1_1: Self = Self::new(1, 1);
    pub const V1_2: Self = Self::new(1, 2);
    pub const V1_3: Self = Self::new(1, 3);

    /// Lowest version the Core accepts (Esquadro B3): `reduce` needs the 1.2 guard.
    pub const MIN_SUPPORTED: Self = Self::V1_2;
    /// Highest version the Core speaks.
    pub const MAX_SUPPORTED: Self = Self::V1_3;

    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Parses `^[0-9]+\.[0-9]+$` without leading zeros.
    pub fn parse(value: &str) -> Result<Self, VersionError> {
        let (major, minor) = value.split_once('.').ok_or(VersionError::Invalid)?;
        Ok(Self::new(part(major)?, part(minor)?))
    }

    /// Highest version listed by the adapter that the Core also supports.
    pub fn negotiate(offered: &[Self]) -> Result<Self, VersionError> {
        offered
            .iter()
            .copied()
            .filter(|v| (Self::MIN_SUPPORTED..=Self::MAX_SUPPORTED).contains(v))
            .max()
            .ok_or(VersionError::Unsupported)
    }

    /// A received envelope is accepted when its minor is ≤ the negotiated one (retransmitted
    /// outbox frames keep their original bytes, §9).
    pub fn accepts_envelope(self, envelope: Self) -> bool {
        envelope.major == self.major && envelope.minor <= self.minor
    }

    /// Same major: a frame of another minor is acceptable when its type exists in the
    /// negotiated version and the receiver's schema validates it (§9 rollback rule).
    pub fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// `reduce` (with `refs`) is sent only on sessions ≥ 1.2.
    pub fn supports_reduce(self) -> bool {
        self >= Self::V1_2
    }

    /// `account_state` exists from 1.1.
    pub fn supports_account_state(self) -> bool {
        self >= Self::V1_1
    }

    /// Notifications, `trade_completed`, `notification_read` and welcome display fields: 1.3.
    pub fn supports_notifications(self) -> bool {
        self >= Self::V1_3
    }
}

fn part(value: &str) -> Result<u16, VersionError> {
    let canonical = !value.is_empty()
        && value.bytes().all(|b| b.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'));
    if !canonical {
        return Err(VersionError::Invalid);
    }
    value.parse().map_err(|_| VersionError::Invalid)
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> ProtocolVersion {
        ProtocolVersion::parse(s).unwrap()
    }

    #[test]
    fn picks_highest_common_version() {
        assert_eq!(
            ProtocolVersion::negotiate(&[v("1.0"), v("1.1"), v("1.2")]),
            Ok(v("1.2"))
        );
        assert_eq!(
            ProtocolVersion::negotiate(&[v("1.2"), v("1.3"), v("1.4")]),
            Ok(v("1.3"))
        );
    }

    #[test]
    fn refuses_adapter_below_minimum() {
        assert_eq!(
            ProtocolVersion::negotiate(&[v("1.0"), v("1.1")]),
            Err(VersionError::Unsupported)
        );
        assert_eq!(
            ProtocolVersion::negotiate(&[]),
            Err(VersionError::Unsupported)
        );
        assert_eq!(
            ProtocolVersion::negotiate(&[v("2.0")]),
            Err(VersionError::Unsupported)
        );
    }

    #[test]
    fn parse_is_strict_and_display_round_trips() {
        for bad in ["1", "1.", ".2", "1.02", "01.2", "1.2.3", "a.b", "1.-2"] {
            assert!(ProtocolVersion::parse(bad).is_err(), "{bad}");
        }
        assert_eq!(v("1.12").to_string(), "1.12");
        assert!(v("1.10") > v("1.9"));
    }

    #[test]
    fn older_envelopes_are_accepted_newer_are_not() {
        let session = v("1.2");
        assert!(session.accepts_envelope(v("1.0")));
        assert!(session.accepts_envelope(v("1.2")));
        assert!(!session.accepts_envelope(v("1.3")));
        assert!(!session.accepts_envelope(v("2.0")));
    }

    #[test]
    fn features_follow_the_negotiated_minor() {
        assert!(v("1.2").supports_reduce() && !v("1.1").supports_reduce());
        assert!(v("1.1").supports_account_state() && !v("1.0").supports_account_state());
        assert!(v("1.3").supports_notifications() && !v("1.2").supports_notifications());
    }
}
