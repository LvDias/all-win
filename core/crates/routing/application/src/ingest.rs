//! Use cases.

use std::sync::Arc;

use routing_enterprise::IngressEvent;

use crate::ports::{IngestOutcome, IngestStore, NewInstruction, StoreError};

/// Stores an accepted instruction once per `(user_id, message_id)` (ADR-0003 §5).
/// Routing starts asynchronously after commit (M1), never here.
#[derive(Clone)]
pub struct IngestInstruction {
    store: Arc<dyn IngestStore>,
}

impl IngestInstruction {
    pub fn new(store: Arc<dyn IngestStore>) -> Self {
        Self { store }
    }

    pub async fn handle(&self, instruction: NewInstruction) -> Result<IngestOutcome, StoreError> {
        self.store.ingest(&instruction).await
    }
}

/// Makes a rejection visible to the user (audit through the outbox).
#[derive(Clone)]
pub struct RecordIngressRejection {
    store: Arc<dyn IngestStore>,
}

impl RecordIngressRejection {
    pub fn new(store: Arc<dyn IngestStore>) -> Self {
        Self { store }
    }

    pub async fn handle(&self, event: IngressEvent) -> Result<(), StoreError> {
        self.store.record_rejection(&event).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use async_trait::async_trait;
    use chrono::{TimeZone, Utc};
    use routing_enterprise::{
        IngressEventKind, InstructionId, IntakeTimes, MessageId, RejectionCode, SourceId,
    };
    use shared_kernel::UserId;

    use super::*;
    use crate::ports::event_payload;

    #[derive(Default)]
    struct FakeStore {
        receipts: Mutex<HashMap<(UserId, String), InstructionId>>,
        events: Mutex<Vec<IngressEvent>>,
    }

    #[async_trait]
    impl IngestStore for FakeStore {
        async fn ingest(&self, i: &NewInstruction) -> Result<IngestOutcome, StoreError> {
            let mut receipts = self.receipts.lock().unwrap();
            let key = (i.user_id, i.message_id.as_str().to_owned());
            let outcome = match receipts.get(&key) {
                Some(id) => IngestOutcome::Duplicate(*id),
                None => {
                    receipts.insert(key, i.id);
                    IngestOutcome::Accepted(i.id)
                }
            };
            self.events.lock().unwrap().push(outcome.event_for(i));
            Ok(outcome)
        }

        async fn record_rejection(&self, event: &IngressEvent) -> Result<(), StoreError> {
            self.events.lock().unwrap().push(event.clone());
            Ok(())
        }
    }

    fn instruction(user: UserId, msg: &str) -> NewInstruction {
        let t = Utc.with_ymd_and_hms(2026, 10, 23, 12, 0, 0).unwrap();
        NewInstruction {
            id: InstructionId::new_v7(),
            user_id: user,
            source_id: SourceId::new_v7(),
            source_name: "TV main".into(),
            source_type: "tradingview".into(),
            source_key_id: Some("AbCdEf123456".into()),
            message_id: MessageId::parse(msg).unwrap(),
            schema_version: "1.0".into(),
            signal_type: "trigger".into(),
            action: "buy".into(),
            instrument_symbol: "MNQ".into(),
            contract_month: None,
            quantity: Some(1),
            position_ref: None,
            order_ref: None,
            times: IntakeTimes {
                sent_at: t,
                received_at: t,
            },
            payload: serde_json::json!({}),
        }
    }

    #[tokio::test]
    async fn duplicate_returns_the_original_id_and_is_audited() {
        let store = Arc::new(FakeStore::default());
        let uc = IngestInstruction::new(store.clone());
        let user = UserId::new_v7();
        let first = uc.handle(instruction(user, "m-1")).await.unwrap();
        let second = uc.handle(instruction(user, "m-1")).await.unwrap();
        assert!(matches!(first, IngestOutcome::Accepted(_)));
        assert_eq!(second, IngestOutcome::Duplicate(first.instruction_id()));
        let kinds: Vec<_> = store
            .events
            .lock()
            .unwrap()
            .iter()
            .map(|e| e.event_type())
            .collect();
        assert_eq!(kinds, ["ingress.received", "ingress.duplicate_ignored"]);
    }

    #[tokio::test]
    async fn same_message_id_for_two_users_is_independent() {
        let uc = IngestInstruction::new(Arc::new(FakeStore::default()));
        let a = uc
            .handle(instruction(UserId::new_v7(), "m-1"))
            .await
            .unwrap();
        let b = uc
            .handle(instruction(UserId::new_v7(), "m-1"))
            .await
            .unwrap();
        assert!(matches!(a, IngestOutcome::Accepted(_)));
        assert!(matches!(b, IngestOutcome::Accepted(_)));
    }

    #[test]
    fn rejection_payload_has_the_audit_shape_and_key_id_only() {
        let event = IngressEvent {
            user_id: UserId::new_v7(),
            at: Utc::now(),
            source_id: SourceId::new_v7(),
            source_name: "TV main".into(),
            key_id: "AbCdEf123456".into(),
            kind: IngressEventKind::Rejected {
                code: RejectionCode::SchemaViolation,
                pointer: Some("/quantity".into()),
                message_id: None,
                size_bytes: Some(212),
            },
        };
        let p = event_payload(&event);
        assert_eq!(p["actor"], "system");
        assert_eq!(
            p["details"],
            serde_json::json!({"error_code": "ingress.schema_violation", "pointer": "/quantity", "body_size": 212})
        );
        assert_eq!(p["refs"]["source_key_id"], "AbCdEf123456");
        assert_eq!(p["refs"]["source_name"], "TV main");
        assert!(p["refs"].get("message_id").is_none());
        assert!(!p.to_string().contains("awk_"));
    }
}
