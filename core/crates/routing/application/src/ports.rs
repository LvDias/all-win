//! Ports implemented by `routing-infra`.

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use routing_enterprise::{
    IngressEvent, IngressEventKind, InstructionId, IntakeTimes, MessageId, SourceId,
};
use serde_json::{Map, Value, json};
use shared_kernel::UserId;

use crate::duplicate;

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
                content_not_retained: false,
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

/// What a repeated `message_id` is, against the first message (HTTP 5.4.0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DuplicateVerdict {
    /// Same compared fields: `ingress.duplicate_ignored`.
    Same,
    /// The first message's content was purged (Q13 b): `ingress.duplicate_ignored` with
    /// `content_not_retained`. Never a conflict, since there is nothing to compare.
    NotRetained,
    /// A compared field differs: `ingress.duplicate_conflict`.
    Conflict {
        first_received_at: DateTime<Utc>,
        differing_fields: Vec<&'static str>,
    },
}

/// What the store knows about the first message of a `message_id`.
#[derive(Debug, Clone, PartialEq)]
pub struct FirstReceipt {
    pub instruction_id: InstructionId,
    pub received_at: DateTime<Utc>,
    /// `NULL` once purged, and on rows older than the hash (DEV only).
    pub body_sha256: Option<[u8; 32]>,
    pub content_purged: bool,
    /// The first instruction's document while it is retained.
    pub payload: Option<Value>,
}

impl FirstReceipt {
    /// Compares a repeated message with the first one. The hash decides "same" without loading
    /// anything else; field names come only from the retained first document.
    pub fn verdict(&self, again: &NewInstruction) -> DuplicateVerdict {
        if self.body_sha256 == Some(duplicate::body_sha256(&again.payload)) {
            return DuplicateVerdict::Same;
        }
        match (&self.payload, self.content_purged) {
            (Some(first), false) => {
                let differing_fields = duplicate::differing(first, &again.payload);
                if differing_fields.is_empty() {
                    DuplicateVerdict::Same
                } else {
                    DuplicateVerdict::Conflict {
                        first_received_at: self.received_at,
                        differing_fields,
                    }
                }
            }
            _ => DuplicateVerdict::NotRetained,
        }
    }

    /// The audit event of the repeated message.
    pub fn event_for(&self, again: &NewInstruction, verdict: DuplicateVerdict) -> IngressEvent {
        let instruction_id = self.instruction_id;
        let message_id = again.message_id.clone();
        let signal_type = again.signal_type.clone();
        let kind = match verdict {
            DuplicateVerdict::Same | DuplicateVerdict::NotRetained => {
                IngressEventKind::DuplicateIgnored {
                    instruction_id,
                    message_id,
                    signal_type,
                    content_not_retained: verdict == DuplicateVerdict::NotRetained,
                }
            }
            DuplicateVerdict::Conflict {
                first_received_at,
                differing_fields,
            } => IngressEventKind::DuplicateConflict {
                instruction_id,
                message_id,
                signal_type,
                first_received_at,
                differing_fields,
            },
        };
        IngressEvent {
            user_id: again.user_id,
            at: again.times.received_at,
            source_id: again.source_id,
            source_name: again.source_name.clone(),
            key_id: again.source_key_id.clone().unwrap_or_default(),
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
            content_not_retained,
        } => {
            refs.insert("instruction_id".into(), json!(instruction_id));
            refs.insert("message_id".into(), json!(message_id));
            refs.insert("signal_type".into(), json!(signal_type));
            let mut d = Map::new();
            d.insert("original_instruction_id".into(), json!(instruction_id));
            if *content_not_retained {
                d.insert("content_not_retained".into(), json!(true));
            }
            Value::Object(d)
        }
        IngressEventKind::DuplicateConflict {
            instruction_id,
            message_id,
            signal_type,
            first_received_at,
            differing_fields,
        } => {
            refs.insert("instruction_id".into(), json!(instruction_id));
            refs.insert("message_id".into(), json!(message_id));
            refs.insert("signal_type".into(), json!(signal_type));
            let ts = |t: &DateTime<Utc>| t.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true);
            json!({
                "source_name": event.source_name,
                "message_id": message_id,
                "original_instruction_id": instruction_id,
                "first_received_at": ts(first_received_at),
                "second_received_at": ts(&event.at),
                "differing_fields": differing_fields,
            })
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

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use routing_enterprise::{IntakeTimes, MessageId, SourceId};
    use serde_json::json;

    use super::*;

    fn at(s: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 24, 12, 0, s).unwrap()
    }

    fn doc(quantity: i64, sent_at: &str) -> Value {
        json!({"schema_version": "1.0", "message_id": "tv-1", "sent_at": sent_at,
               "source": {"type": "tradingview"}, "signal_type": "trigger", "action": "buy",
               "instrument": {"symbol": "MNQ"}, "quantity": quantity})
    }

    fn again(payload: Value) -> NewInstruction {
        NewInstruction {
            id: InstructionId::new_v7(),
            user_id: UserId::new_v7(),
            source_id: SourceId::new_v7(),
            source_name: "TV main".into(),
            source_type: "tradingview".into(),
            source_key_id: Some("AbCdEf123456".into()),
            message_id: MessageId::parse("tv-1").unwrap(),
            schema_version: "1.0".into(),
            signal_type: "trigger".into(),
            action: "buy".into(),
            instrument_symbol: "MNQ".into(),
            contract_month: None,
            quantity: Some(2),
            position_ref: None,
            order_ref: None,
            times: IntakeTimes {
                sent_at: at(9),
                received_at: at(9),
            },
            payload,
        }
    }

    fn first(payload: Option<Value>, purged: bool) -> FirstReceipt {
        FirstReceipt {
            instruction_id: InstructionId::new_v7(),
            received_at: at(0),
            body_sha256: payload.as_ref().map(duplicate::body_sha256),
            content_purged: purged,
            payload,
        }
    }

    #[test]
    fn a_re_send_with_only_sent_at_changed_is_a_plain_duplicate() {
        let f = first(Some(doc(1, "2026-10-24T12:00:00Z")), false);
        let a = again(doc(1, "2026-10-24T12:00:09Z"));
        assert_eq!(f.verdict(&a), DuplicateVerdict::Same);
        let e = f.event_for(&a, DuplicateVerdict::Same);
        assert_eq!(e.event_type(), "ingress.duplicate_ignored");
        assert_eq!(
            event_payload(&e)["details"],
            json!({"original_instruction_id": f.instruction_id})
        );
    }

    #[test]
    fn a_changed_compared_field_is_a_conflict_with_names_only() {
        let f = first(Some(doc(1, "2026-10-24T12:00:00Z")), false);
        let a = again(doc(2, "2026-10-24T12:00:09Z"));
        let v = f.verdict(&a);
        assert_eq!(
            v,
            DuplicateVerdict::Conflict {
                first_received_at: at(0),
                differing_fields: vec!["quantity"],
            }
        );
        let e = f.event_for(&a, v);
        assert_eq!(e.event_type(), "ingress.duplicate_conflict");
        let p = event_payload(&e);
        assert_eq!(
            p["details"],
            json!({
                "source_name": "TV main", "message_id": "tv-1",
                "original_instruction_id": f.instruction_id,
                "first_received_at": "2026-10-24T12:00:00Z",
                "second_received_at": "2026-10-24T12:00:09Z",
                "differing_fields": ["quantity"],
            })
        );
        // Never a value of the body.
        assert!(!p.to_string().contains("\"quantity\":2"));
        assert_eq!(p["refs"]["instruction_id"], json!(f.instruction_id));
    }

    #[test]
    fn after_the_purge_a_re_send_is_a_duplicate_with_content_not_retained() {
        let f = first(None, true);
        for quantity in [1, 2] {
            let a = again(doc(quantity, "2026-10-24T12:00:09Z"));
            let v = f.verdict(&a);
            assert_eq!(v, DuplicateVerdict::NotRetained);
            let e = f.event_for(&a, v);
            assert_eq!(e.event_type(), "ingress.duplicate_ignored");
            assert_eq!(event_payload(&e)["details"]["content_not_retained"], true);
        }
    }

    #[test]
    fn a_row_older_than_the_hash_compares_its_retained_document() {
        let mut f = first(Some(doc(1, "2026-10-24T12:00:00Z")), false);
        f.body_sha256 = None;
        assert_eq!(
            f.verdict(&again(doc(1, "2026-10-24T12:00:09Z"))),
            DuplicateVerdict::Same
        );
        assert!(matches!(
            f.verdict(&again(doc(3, "2026-10-24T12:00:09Z"))),
            DuplicateVerdict::Conflict { .. }
        ));
    }
}
