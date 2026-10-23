//! Ports implemented by `routing-infra`.

use async_trait::async_trait;
use chrono::NaiveDate;
use routing_enterprise::{
    IngressEvent, IngressEventKind, InstructionId, IntakeTimes, MessageId, SourceId,
};
use serde_json::{Map, Value, json};
use shared_kernel::UserId;

/// A validated instruction ready to be stored (columns of `routing.instruction`).
#[derive(Debug, Clone, PartialEq)]
pub struct NewInstruction {
    pub id: InstructionId,
    pub user_id: UserId,
    pub source_id: SourceId,
    /// Snapshot of the source's name for the audit entry; not a `routing.instruction` column.
    pub source_name: String,
    pub source_type: String,
    pub source_key_id: Option<String>,
    pub message_id: MessageId,
    pub schema_version: String,
    pub signal_type: String,
    pub action: String,
    pub instrument_symbol: String,
    pub contract_month: Option<NaiveDate>,
    pub quantity: Option<i32>,
    pub position_ref: Option<String>,
    pub order_ref: Option<String>,
    pub times: IntakeTimes,
    /// The validated instruction document; never contains credentials.
    pub payload: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IngestOutcome {
    Accepted(InstructionId),
    /// `message_id` already seen for this user; nothing is stored or routed again.
    Duplicate(InstructionId),
}

impl IngestOutcome {
    pub fn instruction_id(self) -> InstructionId {
        match self {
            Self::Accepted(id) | Self::Duplicate(id) => id,
        }
    }

    /// The audit event the store appends in the same transaction as the outcome.
    pub fn event_for(self, instruction: &NewInstruction) -> IngressEvent {
        let message_id = instruction.message_id.clone();
        let signal_type = instruction.signal_type.clone();
        let kind = match self {
            Self::Accepted(instruction_id) => IngressEventKind::Received {
                instruction_id,
                message_id,
                signal_type,
                action: instruction.action.clone(),
                instrument_symbol: instruction.instrument_symbol.clone(),
                quantity: instruction.quantity,
                sent_at: instruction.times.sent_at,
            },
            Self::Duplicate(instruction_id) => IngressEventKind::DuplicateIgnored {
                instruction_id,
                message_id,
                signal_type,
            },
        };
        IngressEvent {
            user_id: instruction.user_id,
            at: instruction.times.received_at,
            source_id: instruction.source_id,
            source_name: instruction.source_name.clone(),
            key_id: instruction.source_key_id.clone().unwrap_or_default(),
            kind,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("ingest store: {0}")]
pub struct StoreError(pub String);

#[async_trait]
pub trait IngestStore: Send + Sync {
    /// In one user-scoped transaction: insert the receipt `(user_id, message_id)`; if it
    /// is new, insert the instruction, otherwise read the original `instruction_id`; then
    /// append `outcome.event_for(instruction)` to the outbox. Exactly one concurrent
    /// caller gets `Accepted`.
    async fn ingest(&self, instruction: &NewInstruction) -> Result<IngestOutcome, StoreError>;

    /// Appends a rejection event to the outbox (no receipt, no instruction).
    async fn record_rejection(&self, event: &IngressEvent) -> Result<(), StoreError>;
}

/// Outbox payload of an ingress event: the audit envelope `{actor, details, refs}`, with
/// `details` exactly `AuditDetailsByCode[code]` (`DetailsIngress*`, HTTP 4.3.0) and the column
/// values in `refs`. The actor is always `system` (s10 §5: an automated source, identified by
/// `source_id`). Never a token, body value or URL: only the `key_id`.
pub fn event_payload(event: &IngressEvent) -> Value {
    let mut refs = Map::new();
    refs.insert("source_id".into(), json!(event.source_id));
    refs.insert("source_name".into(), json!(event.source_name));
    if !event.key_id.is_empty() {
        refs.insert("source_key_id".into(), json!(event.key_id));
    }
    let details = match &event.kind {
        IngressEventKind::Received {
            instruction_id,
            message_id,
            signal_type,
            action,
            instrument_symbol,
            quantity,
            sent_at,
        } => {
            refs.insert("instruction_id".into(), json!(instruction_id));
            refs.insert("message_id".into(), json!(message_id));
            refs.insert("signal_type".into(), json!(signal_type));
            let mut d = Map::new();
            d.insert("action".into(), json!(action));
            d.insert("instrument_symbol".into(), json!(instrument_symbol));
            if let Some(q) = quantity {
                d.insert("quantity".into(), json!(q));
            }
            d.insert(
                "sent_at".into(),
                json!(sent_at.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)),
            );
            Value::Object(d)
        }
        IngressEventKind::DuplicateIgnored {
            instruction_id,
            message_id,
            signal_type,
        } => {
            refs.insert("instruction_id".into(), json!(instruction_id));
            refs.insert("message_id".into(), json!(message_id));
            refs.insert("signal_type".into(), json!(signal_type));
            json!({ "original_instruction_id": instruction_id })
        }
        IngressEventKind::Rejected {
            code,
            pointer,
            message_id,
            size_bytes,
        } => {
            if let Some(message_id) = message_id {
                refs.insert("message_id".into(), json!(message_id));
            }
            json!({
                "error_code": code.as_str(),
                // RFC 6901: "" is the whole document (no specific field).
                "pointer": pointer.as_deref().unwrap_or(""),
                "body_size": size_bytes.unwrap_or(0),
            })
        }
        IngressEventKind::RateLimited { retry_after_s } => {
            json!({ "retry_after_s": (*retry_after_s).max(1) })
        }
    };
    json!({ "actor": "system", "details": details, "refs": refs })
}
