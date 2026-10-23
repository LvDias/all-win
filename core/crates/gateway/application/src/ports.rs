//! Ports. Consumer-side traits for other modules (implemented in `app`) and the gateway's own
//! store (implemented in `gateway-infra`).

use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use gateway_enterprise::command::{Command, CommandState};
use gateway_enterprise::ids::{AccountId, CommandId, ConnectionId, EventId, VenueAccountId};
use gateway_enterprise::instrument::Instrument;
use gateway_enterprise::protocol::ProtocolVersion;
use gateway_enterprise::report::ReportFact;
use gateway_enterprise::sync::AccountSync;
use serde_json::Value;
use shared_kernel::{Money, UserId};
use uuid::Uuid;

use crate::events::GatewayEvent;

#[derive(Debug, thiserror::Error)]
#[error("port: {0}")]
pub struct PortError(pub String);

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Optimistic concurrency lost (account_sync.version or command state): re-read and retry.
    #[error("concurrent change")]
    Conflict,
    #[error("store: {0}")]
    Other(String),
}

// ---- identity (Brasa): device tokens -------------------------------------------------------

/// Who a valid device token belongs to. The token itself never leaves identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterPrincipal {
    pub user_id: UserId,
    pub connection_id: ConnectionId,
    /// Safe to log (ADR-0001 §5).
    pub key_id: String,
    /// The user's UI locale, `pt-BR` or `en` (`welcome.locale`).
    pub locale: String,
}

/// Opaque presented credential; `Debug` never prints it.
pub struct PresentedToken(String);

impl PresentedToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for PresentedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PresentedToken(***)")
    }
}

#[async_trait]
pub trait DeviceTokens: Send + Sync {
    /// `None` when unknown, mismatched or revoked (close 4001/4002 is decided by the caller).
    async fn authenticate(
        &self,
        presented: &PresentedToken,
    ) -> Result<Option<AdapterPrincipal>, PortError>;
    /// 60-s revalidation of a live session (ADR-0004 §2).
    async fn still_valid(&self, key_id: &str) -> Result<bool, PortError>;
}

// ---- accounts (Malha): binding (ADR-0008) --------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundAccount {
    pub account_id: AccountId,
    /// 1.3 `welcome.accounts[].display_name`.
    pub display_name: String,
}

#[async_trait]
pub trait AccountBindings: Send + Sync {
    /// `account_for_venue`, batched for a `welcome`.
    async fn accounts_for_venues(
        &self,
        user_id: UserId,
        venues: &[VenueAccountId],
    ) -> Result<HashMap<VenueAccountId, BoundAccount>, PortError>;
    /// `binding_of(account_id)`.
    async fn binding_of(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<Option<VenueAccountId>, PortError>;
}

// ---- risk (Malha): limits and commission rate ----------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedLimits {
    /// `derived_from_version`; the protocol `limits.accounts[].version`.
    pub version: i64,
    /// Protocol `limits.accounts[]` item without `account_id` / `platform_account`.
    pub body: Value,
}

#[async_trait]
pub trait RiskLimits: Send + Sync {
    async fn current_limits(
        &self,
        user_id: UserId,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, DerivedLimits>, PortError>;
    /// `commission_per_contract_side` in force; `None` = no floor configured.
    async fn commission_rate(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<Option<Money>, PortError>;
}

// ---- gateway store (gateway-infra) ---------------------------------------------------------

/// `hello.accounts[]` facts upserted into `gateway.venue_account`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisiblePlatformAccount {
    pub platform_account: String,
    pub kind: String,
    pub oco_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VenueAccount {
    pub id: VenueAccountId,
    pub platform_account: String,
    pub kind: String,
    pub oco_location: String,
    /// The kind before this upsert, when it changed (audit for bound accounts, ADR-0008 §3.5).
    pub previous_kind: Option<String>,
}

impl VenueAccount {
    pub fn oco_local_or_unknown(&self) -> bool {
        self.oco_location != "server"
    }
}

/// A command row with its stored frame.
#[derive(Debug, Clone, PartialEq)]
pub struct StoredCommand {
    pub command: Command,
    pub instruction_id: Uuid,
    pub step: String,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub connection_id: ConnectionId,
    pub instrument: Instrument,
    pub msg_id: Uuid,
    /// The exact envelope; retransmitted unchanged.
    pub frame: String,
}

/// A command consumed from `routing.commands_requested`, resolved to a connection.
#[derive(Debug, Clone, PartialEq)]
pub struct NewCommand {
    pub stored: StoredCommand,
    pub payload: Value,
}

/// A reliable C→A message other than a command (`limits`, `instruction_result`).
#[derive(Debug, Clone, PartialEq)]
pub struct OutboundMessage {
    pub msg_id: Uuid,
    pub user_id: UserId,
    pub connection_id: ConnectionId,
    pub kind: String,
    pub payload: Value,
    pub protocol_version: ProtocolVersion,
    pub frame: String,
    pub created_at: DateTime<Utc>,
}

/// One accepted execution report, ready to insert.
#[derive(Debug, Clone, PartialEq)]
pub struct ReportRow {
    pub event_id: EventId,
    pub account_id: AccountId,
    pub connection_id: ConnectionId,
    pub instrument: Option<Instrument>,
    pub fact: ReportFact,
    pub fill_price: Option<String>,
    pub commission: Option<Money>,
    pub commission_rate: Option<Money>,
    pub commission_effective: Option<Money>,
    pub commission_source: Option<&'static str>,
    pub platform_order_id: Option<String>,
    pub avg_price_after: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub platform_time: Option<DateTime<Utc>>,
    pub payload: Value,
}

/// Sync changes to persist with compare-and-set on `account_sync.version`.
#[derive(Debug, Clone, PartialEq)]
pub struct SyncWrite {
    pub account_id: AccountId,
    pub expected_version: i64,
    pub next: AccountSync,
}

/// Command state change committed with the same transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandTransition {
    pub id: CommandId,
    pub from: CommandState,
    pub to: CommandState,
    pub at: DateTime<Utc>,
}

/// One atomic write: optional inbound receipt (msg_id dedupe), report, command transitions,
/// sync CAS writes and outbox events. All or nothing.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UnitOfWork {
    pub receipt: Option<(ConnectionId, Uuid)>,
    pub report: Option<ReportRow>,
    pub commands: Vec<CommandTransition>,
    pub sync: Vec<SyncWrite>,
    pub events: Vec<GatewayEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitOutcome {
    Committed,
    /// The receipt `msg_id`, the report `event_id` or its `(account_id, fill_id)` add already
    /// exists: nothing was written; the caller still acks.
    Duplicate,
}

#[async_trait]
pub trait GatewayStore: Send + Sync {
    async fn upsert_venue_accounts(
        &self,
        user_id: UserId,
        connection: ConnectionId,
        accounts: &[VisiblePlatformAccount],
        seen_at: DateTime<Utc>,
    ) -> Result<Vec<VenueAccount>, StoreError>;

    /// `ON CONFLICT (id) DO NOTHING`; returns how many rows were new.
    async fn insert_commands(
        &self,
        user_id: UserId,
        commands: &[NewCommand],
    ) -> Result<usize, StoreError>;

    /// `pending` and `sent` commands of a connection, in creation order.
    async fn open_commands(
        &self,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<StoredCommand>, StoreError>;

    async fn command(
        &self,
        user_id: UserId,
        id: CommandId,
    ) -> Result<Option<StoredCommand>, StoreError>;

    /// Commands of the connection that left the Core and are not final (`sent`, `acked`).
    async fn in_flight(
        &self,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<StoredCommand>, StoreError>;

    /// The venue account row (connection and platform account), when it exists.
    async fn venue_account(
        &self,
        user_id: UserId,
        id: VenueAccountId,
    ) -> Result<Option<(ConnectionId, VenueAccount)>, StoreError>;

    /// Compare-and-set `pending → sent` with `sent_at`, committed before the socket write.
    /// `false` when the row is no longer pending.
    /// The events (`execution.command_sent`) are written in the same transaction.
    async fn mark_sent(
        &self,
        user_id: UserId,
        id: CommandId,
        at: DateTime<Utc>,
        events: Vec<GatewayEvent>,
    ) -> Result<bool, StoreError>;

    /// Acks C→A messages (commands and outbound messages) of the connection.
    async fn ack(
        &self,
        user_id: UserId,
        connection: ConnectionId,
        msg_ids: &[Uuid],
        at: DateTime<Utc>,
    ) -> Result<(), StoreError>;

    async fn enqueue_outbound(&self, message: &OutboundMessage) -> Result<(), StoreError>;

    async fn unacked_outbound(
        &self,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<OutboundMessage>, StoreError>;

    /// Reports of (account, contract) since the last one that left the position flat
    /// (inclusive), in `seq` order: enough to rebuild the expected position and its identity.
    async fn position_history(
        &self,
        user_id: UserId,
        account: AccountId,
        instrument: &Instrument,
    ) -> Result<Vec<ReportFact>, StoreError>;

    /// Contracts of the account with a last known non-zero position, with that position.
    async fn open_positions(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<Vec<(Instrument, i64)>, StoreError>;

    /// Live All Win working orders of the account, from reports (see `ExpectedOrders`).
    async fn order_reports(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<Vec<(Instrument, ReportFact)>, StoreError>;

    /// Reports already received for a command (all children), in `seq` order.
    async fn command_reports(
        &self,
        user_id: UserId,
        command: CommandId,
    ) -> Result<Vec<ReportFact>, StoreError>;

    /// Whether an `add` exists for `(account_id, fill_id)`.
    async fn fill_known(
        &self,
        user_id: UserId,
        account: AccountId,
        fill_id: &str,
    ) -> Result<bool, StoreError>;

    async fn account_sync(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<AccountSync, StoreError>;

    async fn commit(&self, user_id: UserId, work: UnitOfWork) -> Result<CommitOutcome, StoreError>;
}
