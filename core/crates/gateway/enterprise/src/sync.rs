//! Out-of-sync flag per account (ADR-0004 §6, Q14): sticky until the user clears it.
//!
//! Single writer = gateway. `version` is bumped on every change and is the compare-and-set
//! token of the persisted row (S11 clear answers 409 on a stale version).

use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ids::{CommandId, InstructionId};
use crate::instrument::Instrument;

/// Closed set of `sync.*` reasons (ADR-0004 §6/§6.1; HTTP 5.3.0 draft `SyncReasonCode`, Esquadro).
/// Declaration order is the display order, most severe first (derived `Ord`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum SyncReason {
    BracketDoubleFill,
    BracketIncomplete,
    BracketIntegrityUnknown,
    CommandUnknown,
    PositionChanged,
    SnapshotMismatch,
    CommandPartiallyFilled,
    CommandRejected,
    CommandExpired,
    CommandUndelivered,
    CancelFailed,
    AmbiguousReference,
    ExternalActivity,
    FillCorrected,
    SimulationReset,
    DisconnectedInTrade,
    DisconnectedByUser,
    ConnectionUnpaired,
    AccountNotFoundAfterRepair,
    BracketOrphanLeg,
}

impl SyncReason {
    pub const ALL: [Self; 20] = [
        Self::BracketDoubleFill,
        Self::BracketIncomplete,
        Self::BracketIntegrityUnknown,
        Self::CommandUnknown,
        Self::PositionChanged,
        Self::SnapshotMismatch,
        Self::CommandPartiallyFilled,
        Self::CommandRejected,
        Self::CommandExpired,
        Self::CommandUndelivered,
        Self::CancelFailed,
        Self::AmbiguousReference,
        Self::ExternalActivity,
        Self::FillCorrected,
        Self::SimulationReset,
        Self::DisconnectedInTrade,
        Self::DisconnectedByUser,
        Self::ConnectionUnpaired,
        Self::AccountNotFoundAfterRepair,
        Self::BracketOrphanLeg,
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

/// What one occurrence is about; with the code it forms the dedupe key (one active reason per
/// code and instrument or command).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReasonScope {
    Account,
    Instrument(Instrument),
    Command(CommandId),
}

/// Facts kept with a reason (HTTP `SyncReason`; never platform text).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasonDetails {
    /// Protocol symbol and `YYYY-MM` of the contract (HTTP 5.3.0, Compasso); filled from an
    /// instrument scope by `ReasonEntry::new`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_id: Option<InstructionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<CommandId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled_qty: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_trigger: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasonEntry {
    pub code: SyncReason,
    pub scope: ReasonScope,
    pub detected_at: DateTime<Utc>,
    pub details: ReasonDetails,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl ReasonEntry {
    pub fn new(
        code: SyncReason,
        scope: ReasonScope,
        detected_at: DateTime<Utc>,
        mut details: ReasonDetails,
    ) -> Self {
        if let ReasonScope::Instrument(i) = &scope {
            details.instrument_symbol = Some(i.symbol().to_owned());
            details.contract_month = Some(i.contract_month().to_owned());
        }
        Self {
            code,
            scope,
            detected_at,
            details,
            resolved_at: None,
        }
    }

    /// `account_sync_reason.dedupe_key` (≤ 200 chars).
    pub fn dedupe_key(&self) -> String {
        dedupe_key(self.code, &self.scope)
    }
}

pub fn dedupe_key(code: SyncReason, scope: &ReasonScope) -> String {
    match scope {
        ReasonScope::Account => code.as_str().to_owned(),
        ReasonScope::Instrument(i) => {
            format!("{}:{}:{}", code.as_str(), i.symbol(), i.contract_month())
        }
        ReasonScope::Command(c) => format!("{}:{c}", code.as_str()),
    }
}

/// Inverse of [`dedupe_key`], for rows read back from `account_sync_reason`.
pub fn parse_dedupe_key(key: &str) -> Option<(SyncReason, ReasonScope)> {
    let mut parts = key.splitn(3, ':');
    let code = SyncReason::parse(parts.next()?)?;
    let scope = match (parts.next(), parts.next()) {
        (None, _) => ReasonScope::Account,
        (Some(a), None) => {
            ReasonScope::Command(CommandId::from_uuid(uuid::Uuid::parse_str(a).ok()?))
        }
        (Some(symbol), Some(month)) => {
            ReasonScope::Instrument(Instrument::new(symbol, month).ok()?)
        }
    };
    Some((code, scope))
}

/// Core rule: at most 32 active and 32 resolved reasons per account (HTTP `AccountSync`).
pub const MAX_REASONS: usize = 32;

/// What a mutation did; `None` from a method means nothing changed (no version bump, no event).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncChange {
    pub state: SyncState,
    pub added: Vec<ReasonEntry>,
    pub resolved: Vec<ReasonEntry>,
    /// Removed by the user's clear (active and resolved).
    pub cleared: usize,
    pub version: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ClearError {
    /// The user saw an older state: 409 `accounts.sync_state_changed`, nothing changes.
    #[error("stale sync version")]
    StaleVersion { current: i64 },
    /// 409 `accounts.not_out_of_sync`.
    #[error("account is not out of sync")]
    NotOutOfSync,
}

/// Q14 flag of one account. `InSync` ⇔ no active and no resolved reasons. An account can stay
/// `OutOfSync` with every reason resolved: only the user clears.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountSync {
    state: SyncState,
    active: BTreeMap<String, ReasonEntry>,
    resolved: Vec<ReasonEntry>,
    version: i64,
    flagged_at: Option<DateTime<Utc>>,
    /// The last change was the user's clear (makes a repeated clear a no-op, not a 409).
    last_change_was_clear: bool,
}

impl AccountSync {
    pub fn new_in_sync() -> Self {
        Self {
            state: SyncState::InSync,
            active: BTreeMap::new(),
            resolved: Vec::new(),
            version: 1,
            flagged_at: None,
            last_change_was_clear: false,
        }
    }

    /// Rehydrates a persisted row (`account_sync` + its reason rows).
    pub fn restore(
        state: SyncState,
        reasons: impl IntoIterator<Item = ReasonEntry>,
        version: i64,
        flagged_at: Option<DateTime<Utc>>,
        cleared_at: Option<DateTime<Utc>>,
    ) -> Self {
        let mut active = BTreeMap::new();
        let mut resolved = Vec::new();
        for r in reasons {
            if r.resolved_at.is_some() {
                resolved.push(r);
            } else {
                active.insert(r.dedupe_key(), r);
            }
        }
        let in_sync = state == SyncState::InSync;
        Self {
            state,
            active: if in_sync { BTreeMap::new() } else { active },
            resolved: if in_sync { Vec::new() } else { resolved },
            version,
            flagged_at: if in_sync { None } else { flagged_at },
            last_change_was_clear: in_sync && cleared_at.is_some(),
        }
    }

    pub fn state(&self) -> SyncState {
        self.state
    }

    /// Active reasons, most severe first.
    pub fn active(&self) -> Vec<&ReasonEntry> {
        let mut v: Vec<_> = self.active.values().collect();
        v.sort_by(|a, b| a.code.cmp(&b.code).then(a.detected_at.cmp(&b.detected_at)));
        v
    }

    pub fn resolved(&self) -> &[ReasonEntry] {
        &self.resolved
    }

    pub fn has_active(&self, code: SyncReason) -> bool {
        self.active.values().any(|r| r.code == code)
    }

    pub fn version(&self) -> i64 {
        self.version
    }

    pub fn flagged_at(&self) -> Option<DateTime<Utc>> {
        self.flagged_at
    }

    /// Flags the account. Idempotent per dedupe key. Beyond the cap the occurrence is not kept
    /// as a reason (the caller audits it anyway); the account is still flagged.
    pub fn raise(&mut self, entry: ReasonEntry) -> Option<SyncChange> {
        let key = entry.dedupe_key();
        if self.active.contains_key(&key) {
            return None;
        }
        let was_in_sync = self.state == SyncState::InSync;
        let mut added = Vec::new();
        if self.active.len() < MAX_REASONS {
            added.push(entry.clone());
            self.active.insert(key, entry.clone());
        }
        if was_in_sync {
            self.state = SyncState::OutOfSync;
            self.flagged_at = Some(entry.detected_at);
        } else if added.is_empty() {
            return None;
        }
        Some(self.bump(added, Vec::new(), 0))
    }

    /// The Core removed the cause of every active reason with `code` in `scope`; the account
    /// stays out of sync.
    pub fn resolve(
        &mut self,
        code: SyncReason,
        scope: &ReasonScope,
        now: DateTime<Utc>,
    ) -> Option<SyncChange> {
        let mut entry = self.active.remove(&dedupe_key(code, scope))?;
        entry.resolved_at = Some(now);
        self.resolved.push(entry.clone());
        if self.resolved.len() > MAX_REASONS {
            self.resolved.remove(0);
        }
        Some(self.bump(Vec::new(), vec![entry], 0))
    }

    /// The user clears the flag (S11 AC-3) with the version they saw.
    pub fn clear(&mut self, expected_version: i64) -> Result<Option<SyncChange>, ClearError> {
        if self.state == SyncState::InSync {
            return if self.last_change_was_clear && expected_version + 1 == self.version {
                Ok(None)
            } else {
                Err(ClearError::NotOutOfSync)
            };
        }
        if expected_version != self.version {
            return Err(ClearError::StaleVersion {
                current: self.version,
            });
        }
        let cleared = self.active.len() + self.resolved.len();
        self.active.clear();
        self.resolved.clear();
        self.state = SyncState::InSync;
        self.flagged_at = None;
        let change = self.bump(Vec::new(), Vec::new(), cleared);
        self.last_change_was_clear = true;
        Ok(Some(change))
    }

    /// Protocol `sync_reasons` (welcome / account_state): unique codes, most severe first. When
    /// every reason is resolved the resolved codes are listed, since the protocol requires at
    /// least one reason while out of sync.
    pub fn wire_codes(&self) -> Vec<SyncReason> {
        let source: Vec<SyncReason> = if self.active.is_empty() {
            self.resolved.iter().map(|r| r.code).collect()
        } else {
            self.active.values().map(|r| r.code).collect()
        };
        let set: BTreeSet<_> = source.into_iter().collect();
        set.into_iter().take(20).collect()
    }

    fn bump(
        &mut self,
        added: Vec<ReasonEntry>,
        resolved: Vec<ReasonEntry>,
        cleared: usize,
    ) -> SyncChange {
        self.version += 1;
        self.last_change_was_clear = false;
        SyncChange {
            state: self.state,
            added,
            resolved,
            cleared,
            version: self.version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use uuid::Uuid;

    fn t(s: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(1_800_000_000 + s, 0).unwrap()
    }

    fn cmd(n: u128) -> CommandId {
        CommandId::from_uuid(Uuid::from_u128(n))
    }

    fn entry(code: SyncReason, scope: ReasonScope, at: i64) -> ReasonEntry {
        ReasonEntry::new(code, scope, t(at), ReasonDetails::default())
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
        assert!(
            SyncReason::ALL.windows(2).all(|w| w[0] < w[1]),
            "ALL follows severity order"
        );
    }

    #[test]
    fn one_active_reason_per_code_and_scope() {
        let mut s = AccountSync::new_in_sync();
        let c = s
            .raise(entry(
                SyncReason::CommandRejected,
                ReasonScope::Command(cmd(1)),
                0,
            ))
            .unwrap();
        assert_eq!((c.state, c.version), (SyncState::OutOfSync, 2));
        assert_eq!(s.flagged_at(), Some(t(0)));
        assert_eq!(
            s.raise(entry(
                SyncReason::CommandRejected,
                ReasonScope::Command(cmd(1)),
                1
            )),
            None
        );
        assert!(
            s.raise(entry(
                SyncReason::CommandRejected,
                ReasonScope::Command(cmd(2)),
                2
            ))
            .is_some()
        );
        assert_eq!(s.active().len(), 2);
        assert_eq!(s.version(), 3);
    }

    #[test]
    fn resolved_reasons_keep_the_account_out_of_sync() {
        let mut s = AccountSync::new_in_sync();
        s.raise(entry(
            SyncReason::BracketIntegrityUnknown,
            ReasonScope::Account,
            0,
        ));
        let c = s
            .resolve(
                SyncReason::BracketIntegrityUnknown,
                &ReasonScope::Account,
                t(5),
            )
            .unwrap();
        assert_eq!(c.state, SyncState::OutOfSync);
        assert!(s.active().is_empty());
        assert_eq!(s.resolved()[0].resolved_at, Some(t(5)));
        assert_eq!(s.wire_codes(), vec![SyncReason::BracketIntegrityUnknown]);
        // A new flag of the same key is a new occurrence; the resolved history survives.
        s.raise(entry(
            SyncReason::BracketIntegrityUnknown,
            ReasonScope::Account,
            9,
        ));
        assert_eq!((s.active().len(), s.resolved().len()), (1, 1));
    }

    #[test]
    fn clear_needs_the_current_version_and_is_idempotent_on_repeat() {
        let mut s = AccountSync::new_in_sync();
        assert_eq!(s.clear(1), Err(ClearError::NotOutOfSync));
        s.raise(entry(SyncReason::SnapshotMismatch, ReasonScope::Account, 0));
        s.raise(entry(
            SyncReason::CommandExpired,
            ReasonScope::Command(cmd(1)),
            1,
        ));
        assert_eq!(s.clear(2), Err(ClearError::StaleVersion { current: 3 }));
        let c = s.clear(3).unwrap().unwrap();
        assert_eq!((c.state, c.version, c.cleared), (SyncState::InSync, 4, 2));
        assert_eq!(s.flagged_at(), None);
        assert_eq!(s.clear(3), Ok(None));
        s.raise(entry(
            SyncReason::CommandExpired,
            ReasonScope::Command(cmd(1)),
            2,
        ));
        s.resolve(
            SyncReason::CommandExpired,
            &ReasonScope::Command(cmd(1)),
            t(3),
        );
        let v = s.version();
        s.clear(v).unwrap();
        assert_eq!(s.clear(v - 1), Err(ClearError::NotOutOfSync));
    }

    #[test]
    fn active_reasons_are_ordered_by_severity_and_capped() {
        let mut s = AccountSync::new_in_sync();
        s.raise(entry(SyncReason::BracketOrphanLeg, ReasonScope::Account, 0));
        s.raise(entry(
            SyncReason::BracketDoubleFill,
            ReasonScope::Account,
            1,
        ));
        assert_eq!(s.active()[0].code, SyncReason::BracketDoubleFill);
        for n in 0..40 {
            s.raise(entry(
                SyncReason::CommandRejected,
                ReasonScope::Command(cmd(n)),
                2,
            ));
        }
        assert_eq!(s.active().len(), MAX_REASONS);
        assert_eq!(s.state(), SyncState::OutOfSync);
    }

    #[test]
    fn dedupe_key_fits_the_column() {
        let i = Instrument::new("ABCDEFGHIJ", "2026-12").unwrap();
        for code in SyncReason::ALL {
            assert!(dedupe_key(code, &ReasonScope::Instrument(i.clone())).len() <= 200);
            assert!(dedupe_key(code, &ReasonScope::Command(cmd(u128::MAX))).len() <= 200);
        }
    }

    #[test]
    fn instrument_scope_fills_symbol_and_month() {
        let i = Instrument::new("MNQ", "2026-12").unwrap();
        let e = entry(SyncReason::SnapshotMismatch, ReasonScope::Instrument(i), 0);
        assert_eq!(e.details.instrument_symbol.as_deref(), Some("MNQ"));
        assert_eq!(e.details.contract_month.as_deref(), Some("2026-12"));
        assert_eq!(e.dedupe_key(), "sync.snapshot_mismatch:MNQ:2026-12");
    }

    #[test]
    fn dedupe_keys_parse_back() {
        let i = Instrument::new("MNQ", "2026-12").unwrap();
        for scope in [
            ReasonScope::Account,
            ReasonScope::Instrument(i),
            ReasonScope::Command(cmd(7)),
        ] {
            let key = dedupe_key(SyncReason::CommandRejected, &scope);
            assert_eq!(
                parse_dedupe_key(&key),
                Some((SyncReason::CommandRejected, scope))
            );
        }
        assert_eq!(parse_dedupe_key("sync.nope"), None);
        assert_eq!(parse_dedupe_key("sync.command_rejected:not-a-uuid"), None);
    }

    #[test]
    fn restore_in_sync_drops_stray_reasons() {
        let s = AccountSync::restore(
            SyncState::InSync,
            [entry(SyncReason::CommandRejected, ReasonScope::Account, 0)],
            7,
            Some(t(0)),
            None,
        );
        assert!(s.active().is_empty() && s.flagged_at().is_none());
    }
}
