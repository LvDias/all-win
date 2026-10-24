//! Outbox consumers (Esquadro B5): `routing.commands_requested` (C1),
//! `identity.device_token_revoked` (C2), `account.binding_changed` (C3), `risk.limits_derived`
//! (C4). `app` maps each outbox row to these typed events and calls `handle`.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::command::{Command, CommandKind, PositionEffect};
use gateway_enterprise::ids::{AccountId, CommandId, ConnectionId};
use gateway_enterprise::instrument::Instrument;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use shared_kernel::UserId;
use uuid::Uuid;

use crate::hello::{limits_item, outbound};
use crate::ports::{
    AccountBindings, DerivedLimits, GatewayStore, NewCommand, PortError, StoreError, StoredCommand,
};
use crate::session::Sessions;
use crate::wire::{self, COMMAND_ENVELOPE_VERSION};

pub const COMMANDS_REQUESTED: &str = "routing.commands_requested";
pub const DEVICE_TOKEN_REVOKED: &str = "identity.device_token_revoked";
pub const BINDING_CHANGED: &str = "account.binding_changed";
pub const LIMITS_DERIVED: &str = "risk.limits_derived";

#[derive(Debug, thiserror::Error)]
pub enum ConsumeError {
    #[error("invalid event: {0}")]
    Invalid(String),
    #[error(transparent)]
    Port(#[from] PortError),
    #[error(transparent)]
    Store(#[from] StoreError),
}

// ---- C1 ------------------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct CommandsRequested {
    pub instruction_id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub commands: Vec<RequestedCommand>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestedCommand {
    pub command_id: Uuid,
    pub kind: CommandKind,
    pub step: String,
    #[serde(default)]
    pub depends_on: Option<Uuid>,
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub position_effect: Option<PositionEffect>,
    pub instrument: Instrument,
    #[serde(default)]
    pub refs: Option<Value>,
    /// The protocol `place` / `modify` / `cancel` / `flatten` object.
    pub body: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandsOutcome {
    /// New rows were stored for this connection; wake its delivery loop.
    Enqueued {
        connection: ConnectionId,
        new: usize,
    },
    /// The account has no binding (or its venue row is missing): nothing can be delivered.
    /// Routing already skips unbound accounts (`routing.platform_unbound`); this is the race.
    Unbound,
}

#[derive(Clone)]
pub struct ConsumeCommandsRequested {
    store: Arc<dyn GatewayStore>,
    bindings: Arc<dyn AccountBindings>,
}

impl ConsumeCommandsRequested {
    pub fn new(store: Arc<dyn GatewayStore>, bindings: Arc<dyn AccountBindings>) -> Self {
        Self { store, bindings }
    }

    pub async fn handle(
        &self,
        user: UserId,
        event: &CommandsRequested,
    ) -> Result<CommandsOutcome, ConsumeError> {
        let account = AccountId::from_uuid(event.account_id);
        let Some(venue_id) = self.bindings.binding_of(user, account).await? else {
            return Ok(CommandsOutcome::Unbound);
        };
        let Some((connection, venue)) = self.store.venue_account(user, venue_id).await? else {
            return Ok(CommandsOutcome::Unbound);
        };
        let mut rows = Vec::with_capacity(event.commands.len());
        for c in &event.commands {
            let command = Command::new_pending(
                CommandId::from_uuid(c.command_id),
                c.kind,
                c.position_effect,
                c.depends_on.map(CommandId::from_uuid),
                c.expires_at,
                event.created_at,
            )
            .map_err(|e| ConsumeError::Invalid(e.to_string()))?;
            let payload = command_payload(event, c, &venue.platform_account);
            let msg_id = Uuid::now_v7();
            // `created_at` is the envelope `sent_at`: the creation time, unchanged on every
            // retransmission (ADR-0004 §4).
            let frame = wire::envelope(
                COMMAND_ENVELOPE_VERSION,
                msg_id,
                "command",
                event.created_at,
                &payload,
            );
            rows.push(NewCommand {
                stored: StoredCommand {
                    command,
                    instruction_id: event.instruction_id,
                    step: c.step.clone(),
                    user_id: user,
                    account_id: account,
                    connection_id: connection,
                    instrument: c.instrument.clone(),
                    msg_id,
                    frame,
                },
                payload,
            });
        }
        let new = self.store.insert_commands(user, &rows).await?;
        Ok(CommandsOutcome::Enqueued { connection, new })
    }
}

fn command_payload(
    event: &CommandsRequested,
    c: &RequestedCommand,
    platform_account: &str,
) -> Value {
    let kind = match c.kind {
        CommandKind::Place => "place",
        CommandKind::Modify => "modify",
        CommandKind::Cancel => "cancel",
        CommandKind::Flatten => "flatten",
    };
    let mut m = Map::new();
    m.insert("command_id".into(), json!(c.command_id));
    m.insert("instruction_id".into(), json!(event.instruction_id));
    m.insert("account_id".into(), json!(event.account_id));
    m.insert("platform_account".into(), json!(platform_account));
    m.insert("instrument".into(), json!(c.instrument));
    m.insert("kind".into(), json!(kind));
    if let Some(e) = c.expires_at {
        m.insert("expires_at".into(), json!(wire::timestamp(e)));
    }
    if let Some(d) = c.depends_on {
        m.insert("depends_on".into(), json!(d));
    }
    if let Some(r) = &c.refs {
        m.insert("refs".into(), r.clone());
    }
    m.insert(kind.into(), c.body.clone());
    Value::Object(m)
}

// ---- C2 ------------------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceTokenRevoked {
    pub connection_id: Uuid,
    pub key_id: String,
}

/// Returns the live session to close with 4002, if any (its key matches).
pub fn on_token_revoked(
    sessions: &Sessions,
    event: &DeviceTokenRevoked,
) -> Option<Arc<crate::session::Session>> {
    sessions
        .get(ConnectionId::from_uuid(event.connection_id))
        .filter(|s| s.key_id == event.key_id)
}

// ---- C3 ------------------------------------------------------------------------------------

/// Esquadro X2 (c): a bind or rebind made during a session applies at the next handshake, and
/// commands only go to accounts in the current `welcome`. Nothing to do on the live session.
pub fn on_binding_changed(_event: &Value) {}

// ---- C4 ------------------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct LimitsDerived {
    pub account_id: Uuid,
    pub derived_from_version: i64,
    pub limits: Value,
}

#[derive(Clone)]
pub struct ConsumeLimitsDerived {
    store: Arc<dyn GatewayStore>,
    sessions: Sessions,
}

impl ConsumeLimitsDerived {
    pub fn new(store: Arc<dyn GatewayStore>, sessions: Sessions) -> Self {
        Self { store, sessions }
    }

    /// Enqueues a `limits` frame on the live session that maps the account, unless that session
    /// already sent this or a newer version (Esquadro L1). No session: nothing (the next
    /// `welcome` reads the current limits).
    pub async fn handle(
        &self,
        event: &LimitsDerived,
        now: DateTime<Utc>,
    ) -> Result<Option<ConnectionId>, ConsumeError> {
        let account = AccountId::from_uuid(event.account_id);
        let Some(session) = self.sessions.for_account(account) else {
            return Ok(None);
        };
        if !session.should_send_limits(account, event.derived_from_version) {
            return Ok(None);
        }
        let Some(mapped) = session.mapped(account) else {
            return Ok(None);
        };
        let derived = DerivedLimits {
            version: event.derived_from_version,
            body: event.limits.clone(),
        };
        let message = outbound(
            &session,
            "limits",
            json!({ "accounts": [limits_item(mapped, &derived)] }),
            now,
        );
        self.store.enqueue_outbound(&message).await?;
        Ok(Some(session.connection_id))
    }
}
