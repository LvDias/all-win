//! Outbox dispatcher (ALIGN-2): finds unpublished events as `allwin_worker`, hands each
//! one to its registered consumers (on the `allwin_app` pool with the event's user
//! context), then marks it published. Delivery is at least once; consumers dedupe on
//! the event id.
//!
//! Architecture rule (Architect, decision (A), 2026-10-23): an event is never marked published unless
//! every consumer registered for its type handled it. Events without a consumer are not
//! even fetched, so they stay visible and never block the queue. Consumers are registered
//! before the dispatcher starts, and startup fails when an event type a module emits has
//! no consumer ([`Consumers::require`]).
//!
//! A failed delivery is retried with exponential backoff (`next_attempt_at`), so it never
//! blocks newer events; after [`MAX_ATTEMPTS`] it is dead-lettered and logged as an alert.
//! M1 note (Code Review 5 C3): with a second consumer type, delivery must be tracked per
//! consumer (cursor or receipt table), not by one `published_at`.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use audit_application::{PublishedEvent, RecordAuditEntry};
use chrono::{DateTime, Utc};
use serde_json::Value;
use shared_kernel::UserId;
use sqlx::PgPool;
use tokio::sync::Notify;
use uuid::Uuid;

const BATCH: i64 = 100;
const POLL: Duration = Duration::from_millis(500);
/// Attempts before an event is dead-lettered (Code Review 5 C2).
pub const MAX_ATTEMPTS: i32 = 20;
/// Retry delay: 2^attempts seconds, capped.
const MAX_BACKOFF_SECS: i64 = 600;
/// `app.outbox.last_error` length limit.
const MAX_ERROR_LEN: usize = 500;

/// An outbox row as delivered to consumers.
#[derive(Debug, Clone, PartialEq)]
pub struct OutboxEvent {
    pub id: Uuid,
    pub user_id: UserId,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub payload: Value,
}

#[async_trait]
pub trait EventConsumer: Send + Sync {
    /// For logs only.
    fn name(&self) -> &'static str;
    async fn handle(&self, event: &OutboxEvent) -> Result<(), String>;
}

#[derive(Debug, thiserror::Error)]
#[error("no consumer registered for outbox event types {0:?}")]
pub struct MissingConsumer(pub Vec<String>);

/// Consumers by event type, fixed before the dispatcher starts.
#[derive(Default, Clone)]
pub struct Consumers {
    by_type: BTreeMap<String, Vec<Arc<dyn EventConsumer>>>,
}

impl Consumers {
    pub fn register(&mut self, event_type: &str, consumer: Arc<dyn EventConsumer>) {
        self.by_type
            .entry(event_type.to_owned())
            .or_default()
            .push(consumer);
    }

    /// Fails unless every type in `emitted` has at least one consumer.
    pub fn require(&self, emitted: &[&str]) -> Result<(), MissingConsumer> {
        let missing: Vec<String> = emitted
            .iter()
            .filter(|t| !self.by_type.contains_key(**t))
            .map(|t| (*t).to_owned())
            .collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(MissingConsumer(missing))
        }
    }

    /// Event types with at least one consumer.
    pub fn types(&self) -> Vec<String> {
        self.by_type.keys().cloned().collect()
    }

    fn for_type(&self, event_type: &str) -> &[Arc<dyn EventConsumer>] {
        self.by_type.get(event_type).map_or(&[], Vec::as_slice)
    }
}

/// Audit consumer: records the event in the affected user's audit log.
pub struct AuditConsumer(pub RecordAuditEntry);

#[async_trait]
impl EventConsumer for AuditConsumer {
    fn name(&self) -> &'static str {
        "audit"
    }

    async fn handle(&self, e: &OutboxEvent) -> Result<(), String> {
        let event = PublishedEvent {
            id: e.id,
            user_id: e.user_id,
            event_type: e.event_type.clone(),
            occurred_at: e.occurred_at,
            payload: e.payload.clone(),
        };
        match self.0.handle(&event).await {
            Ok(true) => Ok(()),
            // Registered for a type audit cannot record: a wiring error, never "done".
            Ok(false) => Err(format!("event type {} is not auditable", e.event_type)),
            Err(err) => Err(err.to_string()),
        }
    }
}

pub struct OutboxDispatcher {
    worker: PgPool,
    consumers: Consumers,
    wake: Arc<Notify>,
}

impl OutboxDispatcher {
    pub fn new(worker: PgPool, consumers: Consumers, wake: Arc<Notify>) -> Self {
        Self {
            worker,
            consumers,
            wake,
        }
    }

    pub async fn run(self) {
        loop {
            let _ = tokio::time::timeout(POLL, self.wake.notified()).await;
            loop {
                match self.dispatch_batch().await {
                    // A full, fully delivered batch: more may be waiting.
                    Ok((seen, delivered)) if seen == BATCH as usize && delivered == seen => {
                        continue;
                    }
                    Ok(_) => break,
                    Err(e) => {
                        tracing::error!(error = %e, "outbox dispatch failed");
                        break;
                    }
                }
            }
        }
    }

    /// Returns `(events seen, events delivered)`.
    pub async fn dispatch_batch(&self) -> Result<(usize, usize), sqlx::Error> {
        let types = self.consumers.types();
        let mut tx = self.worker.begin().await?;
        let rows = sqlx::query!(
            r#"SELECT id, user_id, event_type, occurred_at, payload, attempts FROM app.outbox
               WHERE published_at IS NULL AND dead_lettered_at IS NULL
                 AND next_attempt_at <= now() AND event_type = ANY($1)
               ORDER BY next_attempt_at, occurred_at LIMIT $2 FOR UPDATE SKIP LOCKED"#,
            &types,
            BATCH
        )
        .fetch_all(&mut *tx)
        .await?;
        let seen = rows.len();
        let mut delivered: Vec<Uuid> = Vec::with_capacity(seen);
        for row in rows {
            let attempts = row.attempts;
            let event = OutboxEvent {
                id: row.id,
                user_id: UserId::from_uuid(row.user_id),
                event_type: row.event_type,
                occurred_at: row.occurred_at,
                payload: row.payload,
            };
            let consumers = self.consumers.for_type(&event.event_type);
            let mut errors = Vec::new();
            for consumer in consumers {
                if let Err(e) = consumer.handle(&event).await {
                    tracing::error!(event_id = %event.id, consumer = consumer.name(), error = %e, "outbox consumer failed");
                    errors.push(format!("{}: {e}", consumer.name()));
                }
            }
            if consumers.is_empty() {
                errors.push("no consumer".to_owned());
            }
            if errors.is_empty() {
                delivered.push(event.id);
                continue;
            }
            let attempts = attempts.saturating_add(1);
            let dead = attempts >= MAX_ATTEMPTS;
            if dead {
                tracing::error!(event_id = %event.id, event_type = %event.event_type, attempts, "ALERT outbox event dead-lettered");
            }
            sqlx::query!(
                r#"UPDATE app.outbox
                   SET attempts = $2, last_error = $3,
                       next_attempt_at = now() + $4::integer * interval '1 second',
                       dead_lettered_at = CASE WHEN $5 THEN now() END
                   WHERE id = $1"#,
                event.id,
                attempts,
                truncate(errors.join("; "), MAX_ERROR_LEN),
                i32::try_from(backoff_secs(attempts)).unwrap_or(i32::MAX),
                dead,
            )
            .execute(&mut *tx)
            .await?;
        }
        if !delivered.is_empty() {
            sqlx::query!(
                "UPDATE app.outbox SET published_at = now() WHERE id = ANY($1)",
                &delivered
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok((seen, delivered.len()))
    }
}

/// 2, 4, 8 … seconds, capped at [`MAX_BACKOFF_SECS`].
fn backoff_secs(attempts: i32) -> i64 {
    let exp = u32::try_from(attempts.clamp(0, 30)).unwrap_or(30);
    2_i64.saturating_pow(exp).min(MAX_BACKOFF_SECS)
}

fn truncate(mut s: String, max: usize) -> String {
    if s.len() > max {
        let mut end = max;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        s.truncate(end);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_grows_and_caps() {
        assert_eq!(backoff_secs(1), 2);
        assert_eq!(backoff_secs(5), 32);
        assert_eq!(backoff_secs(MAX_ATTEMPTS), MAX_BACKOFF_SECS);
        assert_eq!(truncate("é".repeat(400), 501).len(), 500);
    }

    struct Noop;

    #[async_trait]
    impl EventConsumer for Noop {
        fn name(&self) -> &'static str {
            "noop"
        }

        async fn handle(&self, _: &OutboxEvent) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn startup_fails_when_an_emitted_type_has_no_consumer() {
        let mut consumers = Consumers::default();
        consumers.register("ingress.received", Arc::new(Noop));
        assert!(consumers.require(&["ingress.received"]).is_ok());
        let err = consumers
            .require(&["ingress.received", "instruction.accepted"])
            .unwrap_err();
        assert_eq!(err.0, ["instruction.accepted"]);
    }
}
