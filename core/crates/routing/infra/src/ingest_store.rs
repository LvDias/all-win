//! Idempotent ingest (wave1-data-model §6.1): one READ COMMITTED transaction per
//! request, scoped by `app.user_id` so RLS applies.

use async_trait::async_trait;
use routing_application::enterprise::{IngressEvent, InstructionId};
use routing_application::ports::event_payload;
use routing_application::{IngestOutcome, IngestStore, NewInstruction, StoreError};
use shared_kernel::UserId;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgIngestStore {
    pool: PgPool,
}

impl PgIngestStore {
    /// `pool` connects as `allwin_app` (RLS enforced).
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn begin_for(
        &self,
        user_id: UserId,
    ) -> Result<Transaction<'static, Postgres>, StoreError> {
        let mut tx = self.pool.begin().await.map_err(store_error)?;
        sqlx::query!(
            "SELECT set_config('app.user_id', $1, true)",
            user_id.as_uuid().to_string()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(store_error)?;
        Ok(tx)
    }
}

#[async_trait]
impl IngestStore for PgIngestStore {
    async fn ingest(&self, i: &NewInstruction) -> Result<IngestOutcome, StoreError> {
        let mut tx = self.begin_for(i.user_id).await?;
        // A concurrent duplicate blocks on the primary-key entry until the first
        // transaction commits, then takes the DO NOTHING path.
        let inserted = sqlx::query_scalar!(
            r#"INSERT INTO routing.message_receipt (user_id, message_id, instruction_id, received_at)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (user_id, message_id) DO NOTHING
               RETURNING instruction_id"#,
            i.user_id.as_uuid(),
            i.message_id.as_str(),
            i.id.as_uuid(),
            i.times.received_at,
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(store_error)?;

        let outcome = match inserted {
            Some(id) => {
                sqlx::query!(
                    r#"INSERT INTO routing.instruction (
                         id, user_id, source_id, source_type, source_key_id, message_id,
                         schema_version, signal_type, action, instrument_symbol, contract_month,
                         quantity, position_ref, order_ref, sent_at, received_at, expiry_base, payload)
                       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
                    id,
                    i.user_id.as_uuid(),
                    i.source_id.as_uuid(),
                    i.source_type,
                    i.source_key_id,
                    i.message_id.as_str(),
                    i.schema_version,
                    i.signal_type,
                    i.action,
                    i.instrument_symbol,
                    i.contract_month,
                    i.quantity,
                    i.position_ref,
                    i.order_ref,
                    i.times.sent_at,
                    i.times.received_at,
                    i.times.expiry_base(),
                    i.payload,
                )
                .execute(&mut *tx)
                .await
                .map_err(store_error)?;
                IngestOutcome::Accepted(InstructionId::from_uuid(id))
            }
            None => {
                let original = sqlx::query_scalar!(
                    r#"SELECT instruction_id FROM routing.message_receipt
                       WHERE user_id = $1 AND message_id = $2"#,
                    i.user_id.as_uuid(),
                    i.message_id.as_str(),
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(store_error)?;
                IngestOutcome::Duplicate(InstructionId::from_uuid(original))
            }
        };

        append_outbox(&mut tx, &outcome.event_for(i)).await?;
        tx.commit().await.map_err(store_error)?;
        Ok(outcome)
    }

    async fn record_rejection(&self, event: &IngressEvent) -> Result<(), StoreError> {
        let mut tx = self.begin_for(event.user_id).await?;
        append_outbox(&mut tx, event).await?;
        tx.commit().await.map_err(store_error)
    }
}

async fn append_outbox(
    tx: &mut Transaction<'static, Postgres>,
    event: &IngressEvent,
) -> Result<(), StoreError> {
    sqlx::query!(
        r#"INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload)
           VALUES ($1, $2, $3, $4, $5)"#,
        Uuid::now_v7(),
        event.user_id.as_uuid(),
        event.event_type(),
        event.at,
        event_payload(event),
    )
    .execute(&mut **tx)
    .await
    .map_err(store_error)?;
    Ok(())
}

/// Database errors carry statement text and constraint names, never bound values;
/// they are logged by the caller as an internal error and never returned to clients.
fn store_error(e: sqlx::Error) -> StoreError {
    StoreError(e.to_string())
}
