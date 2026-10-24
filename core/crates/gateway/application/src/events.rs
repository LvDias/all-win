//! Outbox events the gateway writes in the same transaction as the change they describe
//! (Esquadro B5/B6). Names and payloads are internal to the Core; money and prices are strings.

use chrono::{DateTime, Utc};
use gateway_enterprise::ids::{AccountId, ConnectionId, SessionId};
use gateway_enterprise::sync::{ReasonEntry, SyncChange, SyncState};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::wire::timestamp;

pub const EXECUTION_REPORTED: &str = "gateway.execution_reported";
pub const ACCOUNT_SYNC_CHANGED: &str = "gateway.account_sync_changed";
pub const CONNECTION_SESSION_CHANGED: &str = "gateway.connection_session_changed";

/// Every event type the gateway emits (for `Consumers::require`).
pub const EMITTED: &[&str] = &[
    EXECUTION_REPORTED,
    ACCOUNT_SYNC_CHANGED,
    CONNECTION_SESSION_CHANGED,
];

/// Audit codes the gateway writes as `{actor, details, refs}` envelopes (HTTP 5.3.0
/// `KnownAuditCode`). `sync.flagged` and `sync.restored` are also the S18 notification source.
pub const AUDIT_CODES: &[&str] = &[
    "connection.account_kind_changed",
    "execution.cancelled",
    "execution.command_sent",
    "execution.expired",
    "execution.fill_corrected",
    "execution.filled",
    "execution.order_received",
    "execution.order_working",
    "execution.partially_filled",
    "execution.rejected",
    "execution.state_unknown",
    "execution.unbound_activity",
    "execution.undelivered",
    "sync.flagged",
    "sync.reason_resolved",
    "sync.restored",
];

/// Notification events with their own type (registry code), consumed by S18 (from_now).
pub const NOTIFICATIONS: &[&str] = &[
    "routing.reverse_close_only",
    "sync.bracket_double_fill",
    "sync.bracket_incomplete",
    "sync.bracket_orphan_leg",
    "sync.flatten_failed_unprotected",
];

#[derive(Debug, Clone, PartialEq)]
pub struct GatewayEvent {
    pub id: Uuid,
    pub event_type: &'static str,
    pub occurred_at: DateTime<Utc>,
    pub payload: Value,
}

impl GatewayEvent {
    fn new(event_type: &'static str, occurred_at: DateTime<Utc>, payload: Value) -> Self {
        Self {
            id: Uuid::now_v7(),
            event_type,
            occurred_at,
            payload,
        }
    }
}

fn reason_json(r: &ReasonEntry) -> Value {
    let mut v = json!({
        "code": r.code.as_str(),
        "dedupe_key": r.dedupe_key(),
        "detected_at": timestamp(r.detected_at),
        "details": r.details,
    });
    if let Some(at) = r.resolved_at {
        v["resolved_at"] = Value::String(timestamp(at));
    }
    v
}

/// E2 `gateway.account_sync_changed`.
pub fn account_sync_changed(
    account: AccountId,
    change: &SyncChange,
    active_codes: Vec<String>,
    cause: Value,
    occurred_at: DateTime<Utc>,
) -> GatewayEvent {
    GatewayEvent::new(
        ACCOUNT_SYNC_CHANGED,
        occurred_at,
        json!({
            "account_id": account,
            "sync_state": match change.state { SyncState::InSync => "in_sync", SyncState::OutOfSync => "out_of_sync" },
            "reasons": active_codes,
            "added": change.added.iter().map(reason_json).collect::<Vec<_>>(),
            "resolved": change.resolved.iter().map(reason_json).collect::<Vec<_>>(),
            "cleared": change.cleared,
            "cause": cause,
            "version": change.version,
        }),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    Connected,
    Disconnected,
    Outdated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisconnectReason {
    User,
    Network,
    Outdated,
}

/// E3 `gateway.connection_session_changed` (identity consumes it; transitions only).
#[allow(clippy::too_many_arguments)]
pub fn connection_session_changed(
    connection: ConnectionId,
    session: SessionId,
    status: SessionStatus,
    reason: Option<DisconnectReason>,
    protocol_version: Option<String>,
    adapter_version: &str,
    instance_id: Uuid,
    occurred_at: DateTime<Utc>,
) -> GatewayEvent {
    GatewayEvent::new(
        CONNECTION_SESSION_CHANGED,
        occurred_at,
        json!({
            "connection_id": connection,
            "session_id": session,
            "status": match status {
                SessionStatus::Connected => "connected",
                SessionStatus::Disconnected => "disconnected",
                SessionStatus::Outdated => "outdated",
            },
            "disconnect_reason": reason.map(|r| match r {
                DisconnectReason::User => "user",
                DisconnectReason::Network => "network",
                DisconnectReason::Outdated => "outdated",
            }),
            "protocol_version": protocol_version,
            "adapter_version": adapter_version,
            "instance_id": instance_id,
            "occurred_at": timestamp(occurred_at),
        }),
    )
}

/// E1 `gateway.execution_reported`; `report` is the payload built by intake.
pub fn execution_reported(occurred_at: DateTime<Utc>, report: Value) -> GatewayEvent {
    GatewayEvent::new(EXECUTION_REPORTED, occurred_at, report)
}

/// An audit-coded event (Esquadro (a)): the audit consumer reads the code from `event_type`
/// and the envelope `{actor, details, refs}`; `details` follow `AuditDetailsByCode` (HTTP 5.3.0).
pub fn audit(
    code: &'static str,
    occurred_at: DateTime<Utc>,
    details: Value,
    refs: Value,
) -> GatewayEvent {
    GatewayEvent::new(
        code,
        occurred_at,
        json!({ "actor": "system", "details": details, "refs": refs }),
    )
}

/// A notification fact for S18 (Compasso/Esquadro, s18-notifications §3): type = the registry
/// code, written with its audit entry in one transaction; allowlisted `from_now` until S18.
pub fn notification(
    code: &'static str,
    occurred_at: DateTime<Utc>,
    account: AccountId,
    target: Value,
    audit_event_id: Uuid,
    params: Value,
) -> GatewayEvent {
    GatewayEvent::new(
        code,
        occurred_at,
        json!({
            "account_id": account,
            "target": target,
            "occurred_at": timestamp(occurred_at),
            "audit_event_id": audit_event_id,
            "params": params,
        }),
    )
}
