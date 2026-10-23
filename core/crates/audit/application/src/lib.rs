//! `audit` module — application layer: turns published domain events into audit
//! entries. Input is the outbox envelope, so `audit` depends on no other module.

use std::sync::Arc;

use async_trait::async_trait;
use audit_enterprise::{AuditEntry, Category};
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use shared_kernel::UserId;
use uuid::Uuid;

pub use audit_enterprise as enterprise;

/// A domain event as published through `app.outbox`.
#[derive(Debug, Clone, PartialEq)]
pub struct PublishedEvent {
    pub id: Uuid,
    pub user_id: UserId,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub payload: Value,
}

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("audit store: {0}")]
    Store(String),
}

#[async_trait]
pub trait AuditStore: Send + Sync {
    /// Appends once per `(occurred_at, source_event_id)`; a redelivery is a no-op.
    async fn append(&self, entry: &AuditEntry) -> Result<(), AuditError>;
}

/// Records a published event in the affected user's audit log.
#[derive(Clone)]
pub struct RecordAuditEntry {
    store: Arc<dyn AuditStore>,
}

impl RecordAuditEntry {
    pub fn new(store: Arc<dyn AuditStore>) -> Self {
        Self { store }
    }

    /// Events whose type has no audit category are not audited (`Ok(false)`).
    pub async fn handle(&self, event: &PublishedEvent) -> Result<bool, AuditError> {
        match entry_for(event) {
            Some(entry) => self.store.append(&entry).await.map(|()| true),
            None => Ok(false),
        }
    }
}

/// Well-known payload fields become columns; the remaining fields stay in `details`.
pub fn entry_for(event: &PublishedEvent) -> Option<AuditEntry> {
    let category = Category::of_code(&event.event_type)?;
    let mut details: Map<String, Value> = event.payload.as_object().cloned().unwrap_or_default();
    details.remove("kind");
    let uuid_field = |d: &mut Map<String, Value>, k: &str| {
        d.remove(k)
            .and_then(|v| v.as_str().and_then(|s| Uuid::parse_str(s).ok()))
    };
    let text_field = |d: &mut Map<String, Value>, k: &str| {
        d.remove(k).and_then(|v| v.as_str().map(str::to_owned))
    };
    let instruction_id = uuid_field(&mut details, "instruction_id");
    let source_id = uuid_field(&mut details, "source_id");
    let source_key_id = text_field(&mut details, "key_id").filter(|k| !k.is_empty());
    let signal_type = text_field(&mut details, "signal_type");
    // The rejection code is the reason; keep it in details next to the pointer.
    if let Some(code) = details.remove("code") {
        details.insert("error_code".into(), code);
    }
    Some(AuditEntry {
        id: Uuid::now_v7(),
        user_id: event.user_id,
        occurred_at: event.occurred_at,
        source_event_id: event.id,
        category,
        code: event.event_type.clone(),
        instruction_id,
        source_id,
        source_key_id,
        signal_type,
        details: Value::Object(details),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn event(event_type: &str, payload: Value) -> PublishedEvent {
        PublishedEvent {
            id: Uuid::now_v7(),
            user_id: UserId::new_v7(),
            event_type: event_type.into(),
            occurred_at: Utc::now(),
            payload,
        }
    }

    #[test]
    fn received_event_maps_to_columns() {
        let iid = Uuid::now_v7();
        let sid = Uuid::now_v7();
        let e = event(
            "ingress.received",
            json!({"kind":"ingress.received","instruction_id":iid,"source_id":sid,
                   "key_id":"AbCdEf123456","message_id":"m-1","signal_type":"trigger"}),
        );
        let a = entry_for(&e).unwrap();
        assert_eq!(a.category, Category::Ingress);
        assert_eq!(a.code, "ingress.received");
        assert_eq!(a.instruction_id, Some(iid));
        assert_eq!(a.source_id, Some(sid));
        assert_eq!(a.source_key_id.as_deref(), Some("AbCdEf123456"));
        assert_eq!(a.signal_type.as_deref(), Some("trigger"));
        assert_eq!(a.details, json!({"message_id":"m-1"}));
        assert_eq!(a.source_event_id, e.id);
    }

    #[test]
    fn rejection_keeps_reason_and_pointer() {
        let e = event(
            "ingress.rejected",
            json!({"kind":"ingress.rejected","code":"ingress.schema_violation","pointer":"/quantity",
                   "source_id":Uuid::now_v7(),"key_id":"AbCdEf123456"}),
        );
        let a = entry_for(&e).unwrap();
        assert_eq!(
            a.details,
            json!({"error_code":"ingress.schema_violation","pointer":"/quantity"})
        );
    }

    #[test]
    fn unknown_category_is_not_audited() {
        assert!(entry_for(&event("instruction.accepted", json!({}))).is_none());
    }
}
