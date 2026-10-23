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

/// Outbox payload of an ingress event (audit shape, s1-webhook-ingest §3).
pub fn event_payload(event: &IngressEvent) -> Value {
    let mut payload = Map::new();
    payload.insert("kind".into(), json!(event.event_type()));
    payload.insert("source_id".into(), json!(event.source_id));
    payload.insert("key_id".into(), json!(event.key_id));
    match &event.kind {
        IngressEventKind::Received {
            instruction_id,
            message_id,
            signal_type,
        }
        | IngressEventKind::DuplicateIgnored {
            instruction_id,
            message_id,
            signal_type,
        } => {
            payload.insert("instruction_id".into(), json!(instruction_id));
            payload.insert("message_id".into(), json!(message_id));
            payload.insert("signal_type".into(), json!(signal_type));
        }
        IngressEventKind::Rejected {
            code,
            pointer,
            message_id,
            size_bytes,
        } => {
            payload.insert("code".into(), json!(code.as_str()));
            if let Some(pointer) = pointer {
                payload.insert("pointer".into(), json!(pointer));
            }
            if let Some(message_id) = message_id {
                payload.insert("message_id".into(), json!(message_id));
            }
            if let Some(size_bytes) = size_bytes {
                payload.insert("size_bytes".into(), json!(size_bytes));
            }
        }
    }
    Value::Object(payload)
}
