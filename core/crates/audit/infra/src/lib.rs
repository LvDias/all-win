//! `audit` module — infra layer: append-only `audit.audit_entry`.

use async_trait::async_trait;
use audit_application::enterprise::AuditEntry;
use audit_application::{AuditError, AuditStore};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgAuditStore {
    pool: PgPool,
}

impl PgAuditStore {
    /// `pool` connects as `allwin_app`; every append runs with the entry's user context.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditStore for PgAuditStore {
    async fn append(&self, e: &AuditEntry) -> Result<(), AuditError> {
        let mut tx = self.pool.begin().await.map_err(store_error)?;
        sqlx::query!(
            "SELECT set_config('app.user_id', $1, true)",
            e.user_id.as_uuid().to_string()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(store_error)?;
        sqlx::query!(
            r#"INSERT INTO audit.audit_entry (
                 id, user_id, occurred_at, source_event_id, event_seq, category, code, outcome,
                 actor, safe_to_display, instruction_id, source_id, source_name, source_key_id,
                 account_id, account_name, message_id, signal_type, is_exit, details)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
                       $17, $18, $19, $20)
               ON CONFLICT (occurred_at, source_event_id) DO NOTHING"#,
            e.id,
            e.user_id.as_uuid(),
            e.occurred_at,
            e.source_event_id,
            e.event_seq,
            e.category.as_str(),
            e.code,
            e.outcome.as_str(),
            e.actor.as_str(),
            e.safe_to_display,
            e.instruction_id,
            e.source_id,
            e.source_name,
            e.source_key_id,
            e.account_id,
            e.account_name,
            e.message_id,
            e.signal_type,
            e.is_exit,
            e.details,
        )
        .execute(&mut *tx)
        .await
        .map_err(store_error)?;
        tx.commit().await.map_err(store_error)
    }
}

fn store_error(e: sqlx::Error) -> AuditError {
    AuditError::Store(e.to_string())
}
