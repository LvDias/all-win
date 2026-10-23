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
                 id, user_id, occurred_at, source_event_id, category, code, instruction_id,
                 source_id, source_key_id, signal_type, details)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               ON CONFLICT (occurred_at, source_event_id) DO NOTHING"#,
            e.id,
            e.user_id.as_uuid(),
            e.occurred_at,
            e.source_event_id,
            e.category.as_str(),
            e.code,
            e.instruction_id,
            e.source_id,
            e.source_key_id,
            e.signal_type,
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
