//! Live adapter sessions, in memory (one Core process; Esquadro B4: supersede needs no table).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use gateway_enterprise::ids::{AccountId, ConnectionId, SessionId};
use gateway_enterprise::protocol::ProtocolVersion;
use shared_kernel::UserId;
use uuid::Uuid;

use crate::ports::VenueAccount;

/// Server settings sent in `welcome`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSettings {
    pub heartbeat_interval_ms: u32,
    pub ack_timeout_ms: u32,
    /// Always true outside production (ADR-0004 §8 environment guard).
    pub simulation_only: bool,
}

impl Default for SessionSettings {
    fn default() -> Self {
        Self {
            heartbeat_interval_ms: 10_000,
            ack_timeout_ms: 5_000,
            simulation_only: true,
        }
    }
}

/// An account in this session's `welcome` mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappedAccount {
    pub account_id: AccountId,
    /// Account name snapshot for audit `refs.account_name`.
    pub display_name: String,
    pub venue: VenueAccount,
}

#[derive(Debug)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub connection_id: ConnectionId,
    pub key_id: String,
    pub version: ProtocolVersion,
    pub instance_id: Uuid,
    pub adapter_version: String,
    accounts: BTreeMap<AccountId, MappedAccount>,
    /// Bound accounts left out of the mapping by the environment guard (live/unknown).
    refused: BTreeSet<AccountId>,
    state: Mutex<SessionState>,
}

#[derive(Debug, Default)]
struct SessionState {
    snapshot_evaluated: BTreeSet<AccountId>,
    last_limits_sent: HashMap<AccountId, i64>,
    /// Last write of each reliable frame, for retransmission after `ack_timeout_ms`.
    written: HashMap<Uuid, DateTime<Utc>>,
}

impl Session {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: SessionId,
        user_id: UserId,
        connection_id: ConnectionId,
        key_id: String,
        version: ProtocolVersion,
        instance_id: Uuid,
        adapter_version: String,
        accounts: impl IntoIterator<Item = MappedAccount>,
    ) -> Self {
        Self {
            id,
            user_id,
            connection_id,
            key_id,
            version,
            instance_id,
            adapter_version,
            accounts: accounts.into_iter().map(|a| (a.account_id, a)).collect(),
            refused: BTreeSet::new(),
            state: Mutex::new(SessionState::default()),
        }
    }

    pub fn with_refused(mut self, refused: impl IntoIterator<Item = AccountId>) -> Self {
        self.refused = refused.into_iter().collect();
        self
    }

    pub fn refused_by_environment(&self, account: AccountId) -> bool {
        self.refused.contains(&account)
    }

    fn state(&self) -> std::sync::MutexGuard<'_, SessionState> {
        // A poisoned lock only means another task panicked mid-update; the data is plain maps.
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn mapped(&self, account: AccountId) -> Option<&MappedAccount> {
        self.accounts.get(&account)
    }

    pub fn accounts(&self) -> impl Iterator<Item = &MappedAccount> {
        self.accounts.values()
    }

    pub fn snapshot_evaluated(&self, account: AccountId) -> bool {
        self.state().snapshot_evaluated.contains(&account)
    }

    pub fn mark_snapshot_evaluated(&self, account: AccountId) {
        self.state().snapshot_evaluated.insert(account);
    }

    /// Esquadro L1: a limits version ≤ the one already sent in this session is dropped.
    pub fn should_send_limits(&self, account: AccountId, version: i64) -> bool {
        let mut s = self.state();
        match s.last_limits_sent.get(&account) {
            Some(&sent) if version <= sent => false,
            _ => {
                s.last_limits_sent.insert(account, version);
                true
            }
        }
    }

    /// First write, or the ack timeout passed since the last write.
    pub fn write_due(&self, msg_id: Uuid, now: DateTime<Utc>, ack_timeout_ms: u32) -> bool {
        let mut s = self.state();
        let due = s
            .written
            .get(&msg_id)
            .is_none_or(|&at| now - at >= Duration::milliseconds(i64::from(ack_timeout_ms)));
        if due {
            s.written.insert(msg_id, now);
        }
        due
    }

    pub fn acked(&self, msg_ids: &[Uuid]) {
        let mut s = self.state();
        for id in msg_ids {
            s.written.remove(id);
        }
    }
}

/// Live sessions by connection.
#[derive(Debug, Default, Clone)]
pub struct Sessions {
    inner: Arc<Mutex<HashMap<ConnectionId, Arc<Session>>>>,
}

impl Sessions {
    fn map(&self) -> std::sync::MutexGuard<'_, HashMap<ConnectionId, Arc<Session>>> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Registers a new session; returns the one it supersedes (close 4004, no E3 for it).
    pub fn register(&self, session: Arc<Session>) -> Option<Arc<Session>> {
        self.map().insert(session.connection_id, session)
    }

    /// Removes the session only if it is still the current one of its connection.
    pub fn remove(&self, session: &Session) -> bool {
        let mut map = self.map();
        match map.get(&session.connection_id) {
            Some(current) if current.id == session.id => {
                map.remove(&session.connection_id);
                true
            }
            _ => false,
        }
    }

    pub fn get(&self, connection: ConnectionId) -> Option<Arc<Session>> {
        self.map().get(&connection).cloned()
    }

    pub fn is_current(&self, session: &Session) -> bool {
        self.map()
            .get(&session.connection_id)
            .is_some_and(|s| s.id == session.id)
    }

    /// The live session whose `welcome` maps `account`.
    pub fn for_account(&self, account: AccountId) -> Option<Arc<Session>> {
        self.map()
            .values()
            .find(|s| s.mapped(account).is_some())
            .cloned()
    }

    pub fn all(&self) -> Vec<Arc<Session>> {
        self.map().values().cloned().collect()
    }
}
