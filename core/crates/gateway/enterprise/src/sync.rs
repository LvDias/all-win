//! Out-of-sync flag per account (ADR-0004 §6, Q14): sticky until the user clears it.
//!
//! Single writer = gateway. `version` is bumped on every change and is the compare-and-set
//! token of the persisted row (S11 clear answers 409 on a stale version).

use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Closed set of `sync.*` reasons (ADR-0004 §6/§6.1; HTTP 5.3.0 draft registry, Esquadro).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum SyncReason {
    BracketIncomplete,
    BracketOrphanLeg,
    BracketDoubleFill,
    BracketIntegrityUnknown,
    PositionChanged,
    CommandUndelivered,
    SimulationReset,
    DisconnectedByUser,
    AccountNotFoundAfterRepair,
    DisconnectedInTrade,
    CommandRejected,
    CommandExpired,
    CommandPartiallyFilled,
    CommandUnknown,
    SnapshotMismatch,
    ExternalActivity,
    FillCorrected,
    ConnectionUnpaired,
    AmbiguousReference,
    CancelFailed,
}

impl SyncReason {
    pub const ALL: [Self; 20] = [
        Self::BracketIncomplete,
        Self::BracketOrphanLeg,
        Self::BracketDoubleFill,
        Self::BracketIntegrityUnknown,
        Self::PositionChanged,
        Self::CommandUndelivered,
        Self::SimulationReset,
        Self::DisconnectedByUser,
        Self::AccountNotFoundAfterRepair,
        Self::DisconnectedInTrade,
        Self::CommandRejected,
        Self::CommandExpired,
        Self::CommandPartiallyFilled,
        Self::CommandUnknown,
        Self::SnapshotMismatch,
        Self::ExternalActivity,
        Self::FillCorrected,
        Self::ConnectionUnpaired,
        Self::AmbiguousReference,
        Self::CancelFailed,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BracketIncomplete => "sync.bracket_incomplete",
            Self::BracketOrphanLeg => "sync.bracket_orphan_leg",
            Self::BracketDoubleFill => "sync.bracket_double_fill",
            Self::BracketIntegrityUnknown => "sync.bracket_integrity_unknown",
            Self::PositionChanged => "sync.position_changed",
            Self::CommandUndelivered => "sync.command_undelivered",
            Self::SimulationReset => "sync.simulation_reset",
            Self::DisconnectedByUser => "sync.disconnected_by_user",
            Self::AccountNotFoundAfterRepair => "sync.account_not_found_after_repair",
            Self::DisconnectedInTrade => "sync.disconnected_in_trade",
            Self::CommandRejected => "sync.command_rejected",
            Self::CommandExpired => "sync.command_expired",
            Self::CommandPartiallyFilled => "sync.command_partially_filled",
            Self::CommandUnknown => "sync.command_unknown",
            Self::SnapshotMismatch => "sync.snapshot_mismatch",
            Self::ExternalActivity => "sync.external_activity",
            Self::FillCorrected => "sync.fill_corrected",
            Self::ConnectionUnpaired => "sync.connection_unpaired",
            Self::AmbiguousReference => "sync.ambiguous_reference",
            Self::CancelFailed => "sync.cancel_failed",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.as_str() == code)
    }
}

impl TryFrom<String> for SyncReason {
    type Error = UnknownSyncReason;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value).ok_or(UnknownSyncReason)
    }
}

impl From<SyncReason> for String {
    fn from(value: SyncReason) -> Self {
        value.as_str().to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown sync reason code")]
pub struct UnknownSyncReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncState {
    InSync,
    OutOfSync,
}

/// What a mutation did; `None` from a method means nothing changed (no version bump, no event).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncChange {
    pub state: SyncState,
    pub added: Vec<SyncReason>,
    pub removed: Vec<SyncReason>,
    pub version: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ClearError {
    /// Optimistic concurrency: the user saw an older state (HTTP 409).
    #[error("stale sync version")]
    StaleVersion { current: i64 },
}

/// Invariant: `OutOfSync` ⇔ at least one reason. A reason whose condition resolves is removed,
/// but the last reason is kept: only the user returns the account to `InSync` (Q14).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountSync {
    reasons: BTreeSet<SyncReason>,
    version: i64,
    flagged_at: Option<DateTime<Utc>>,
}

impl AccountSync {
    pub fn new_in_sync() -> Self {
        Self {
            reasons: BTreeSet::new(),
            version: 0,
            flagged_at: None,
        }
    }

    /// Rehydrates a persisted row; an `out_of_sync` row without reasons is refused.
    pub fn restore(
        reasons: impl IntoIterator<Item = SyncReason>,
        version: i64,
        flagged_at: Option<DateTime<Utc>>,
    ) -> Self {
        let reasons: BTreeSet<_> = reasons.into_iter().collect();
        let flagged_at = if reasons.is_empty() { None } else { flagged_at };
        Self {
            reasons,
            version,
            flagged_at,
        }
    }

    pub fn state(&self) -> SyncState {
        if self.reasons.is_empty() {
            SyncState::InSync
        } else {
            SyncState::OutOfSync
        }
    }

    pub fn reasons(&self) -> impl Iterator<Item = SyncReason> + '_ {
        self.reasons.iter().copied()
    }

    pub fn version(&self) -> i64 {
        self.version
    }

    pub fn flagged_at(&self) -> Option<DateTime<Utc>> {
        self.flagged_at
    }

    /// Flags the account with `reason`. Idempotent: an already present reason changes nothing.
    pub fn raise(&mut self, reason: SyncReason, now: DateTime<Utc>) -> Option<SyncChange> {
        if !self.reasons.insert(reason) {
            return None;
        }
        if self.flagged_at.is_none() {
            self.flagged_at = Some(now);
        }
        Some(self.bump(vec![reason], vec![]))
    }

    /// The condition behind `reason` resolved. The last reason of a flagged account stays:
    /// the account remains out of sync until the user clears it.
    pub fn resolve(&mut self, reason: SyncReason) -> Option<SyncChange> {
        if self.reasons.len() <= 1 || !self.reasons.remove(&reason) {
            return None;
        }
        Some(self.bump(vec![], vec![reason]))
    }

    /// The user clears the flag (S11 AC-3), with the version they saw.
    pub fn clear(&mut self, expected_version: i64) -> Result<Option<SyncChange>, ClearError> {
        if expected_version != self.version {
            return Err(ClearError::StaleVersion {
                current: self.version,
            });
        }
        if self.reasons.is_empty() {
            return Ok(None);
        }
        let removed = std::mem::take(&mut self.reasons).into_iter().collect();
        self.flagged_at = None;
        Ok(Some(self.bump(vec![], removed)))
    }

    fn bump(&mut self, added: Vec<SyncReason>, removed: Vec<SyncReason>) -> SyncChange {
        self.version += 1;
        SyncChange {
            state: self.state(),
            added,
            removed,
            version: self.version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn t(s: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(1_800_000_000 + s, 0).unwrap()
    }

    #[test]
    fn codes_round_trip_and_match_the_wire_pattern() {
        for r in SyncReason::ALL {
            assert_eq!(SyncReason::parse(r.as_str()), Some(r));
            let code = r.as_str().strip_prefix("sync.").unwrap();
            assert!(code.bytes().all(|b| b.is_ascii_lowercase() || b == b'_'));
        }
        let unique: BTreeSet<_> = SyncReason::ALL.iter().map(|r| r.as_str()).collect();
        assert_eq!(unique.len(), 20);
        assert_eq!(SyncReason::parse("sync.something_new"), None);
    }

    #[test]
    fn raise_flags_once_and_bumps_version() {
        let mut s = AccountSync::new_in_sync();
        let c = s.raise(SyncReason::CommandRejected, t(0)).unwrap();
        assert_eq!((c.state, c.version), (SyncState::OutOfSync, 1));
        assert_eq!(s.raise(SyncReason::CommandRejected, t(1)), None);
        assert_eq!(s.version(), 1);
        assert_eq!(s.flagged_at(), Some(t(0)));
    }

    #[test]
    fn resolving_never_returns_to_in_sync() {
        let mut s = AccountSync::new_in_sync();
        s.raise(SyncReason::DisconnectedInTrade, t(0));
        s.raise(SyncReason::BracketIntegrityUnknown, t(0));
        assert!(s.resolve(SyncReason::BracketIntegrityUnknown).is_some());
        assert_eq!(s.resolve(SyncReason::DisconnectedInTrade), None);
        assert_eq!(s.state(), SyncState::OutOfSync);
    }

    #[test]
    fn only_the_user_clears_with_the_current_version() {
        let mut s = AccountSync::new_in_sync();
        s.raise(SyncReason::SnapshotMismatch, t(0));
        s.raise(SyncReason::CommandExpired, t(1));
        assert_eq!(s.clear(1), Err(ClearError::StaleVersion { current: 2 }));
        let c = s.clear(2).unwrap().unwrap();
        assert_eq!(
            (c.state, c.version, c.removed.len()),
            (SyncState::InSync, 3, 2)
        );
        assert_eq!(s.flagged_at(), None);
        assert_eq!(s.clear(3), Ok(None));
    }

    #[test]
    fn restore_keeps_the_invariant() {
        let s = AccountSync::restore([], 4, Some(t(0)));
        assert_eq!((s.state(), s.flagged_at()), (SyncState::InSync, None));
    }
}
