//! Outbox dispatcher (ALIGN-2): finds unpublished events as `allwin_worker`, hands each
//! one to its consumers (audit) on the `allwin_app` pool with the event's user context,
//! then marks it published. Delivery is at least once; consumers dedupe on the event id.

use std::sync::Arc;
use std::time::Duration;

use audit_application::{PublishedEvent, RecordAuditEntry};
use shared_kernel::UserId;
use sqlx::PgPool;
use tokio::sync::Notify;
use uuid::Uuid;

const BATCH: i64 = 100;
const POLL: Duration = Duration::from_millis(500);

pub struct OutboxDispatcher {
    worker: PgPool,
    audit: RecordAuditEntry,
    wake: Arc<Notify>,
}

impl OutboxDispatcher {
    pub fn new(worker: PgPool, audit: RecordAuditEntry, wake: Arc<Notify>) -> Self {
        Self {
            worker,
            audit,
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
        let mut tx = self.worker.begin().await?;
        let rows = sqlx::query!(
            r#"SELECT id, user_id, event_type, occurred_at, payload FROM app.outbox
               WHERE published_at IS NULL ORDER BY occurred_at LIMIT $1 FOR UPDATE SKIP LOCKED"#,
            BATCH
        )
        .fetch_all(&mut *tx)
        .await?;
        let seen = rows.len();
        let mut delivered: Vec<Uuid> = Vec::with_capacity(seen);
        for row in rows {
            let event = PublishedEvent {
                id: row.id,
                user_id: UserId::from_uuid(row.user_id),
                event_type: row.event_type,
                occurred_at: row.occurred_at,
                payload: row.payload,
            };
            match self.audit.handle(&event).await {
                Ok(_) => delivered.push(event.id),
                Err(e) => {
                    tracing::error!(event_id = %event.id, error = %e, "audit consumer failed")
                }
            }
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
