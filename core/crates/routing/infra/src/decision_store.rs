//! Routing decisions (`routing.routing_decision`, wave1-data-model §6.2): append-only, one row
//! per (instruction, account), written with its audit event in one user-scoped transaction.

use async_trait::async_trait;
use routing_application::decisions::{DecisionRecord, DecisionStore, InstructionToRoute};
use routing_application::enterprise::{InstructionId, SourceId};
use routing_application::routes::PortError;
use shared_kernel::UserId;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgDecisionStore {
    pool: PgPool,
}

impl PgDecisionStore {
    /// `pool` connects as `allwin_app` (RLS enforced).
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn begin_for(&self, user: UserId) -> Result<Transaction<'static, Postgres>, PortError> {
        let mut tx = self.pool.begin().await.map_err(port_error)?;
        sqlx::query!(
            "SELECT set_config('app.user_id', $1, true)",
            user.as_uuid().to_string()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(port_error)?;
        Ok(tx)
    }
}

#[async_trait]
impl DecisionStore for PgDecisionStore {
    async fn instruction(
        &self,
        user: UserId,
        id: InstructionId,
    ) -> Result<Option<InstructionToRoute>, PortError> {
        let mut tx = self.begin_for(user).await?;
        // RLS scopes the read; the explicit user_id is defence in depth.
        let row = sqlx::query!(
            r#"SELECT id, source_id, message_id, signal_type, action, instrument_symbol, quantity,
                      expiry_base,
                      payload->>'position_effect' AS position_effect,
                      payload->>'side' AS side
                 FROM routing.instruction
                WHERE user_id = $1 AND id = $2"#,
            user.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(port_error)?;
        tx.commit().await.map_err(port_error)?;
        Ok(row.map(|r| InstructionToRoute {
            id: InstructionId::from_uuid(r.id),
            source_id: SourceId::from_uuid(r.source_id),
            message_id: r.message_id,
            signal_type: r.signal_type,
            action: r.action,
            position_effect: r.position_effect,
            side: r.side,
            instrument_symbol: r.instrument_symbol,
            quantity: r.quantity,
            expiry_base: r.expiry_base,
        }))
    }

    async fn record(&self, user: UserId, decisions: &[DecisionRecord]) -> Result<usize, PortError> {
        let mut tx = self.begin_for(user).await?;
        let mut new = 0;
        for d in decisions {
            // A concurrent redelivery blocks on the unique entry and then takes DO NOTHING,
            // so exactly one transaction audits each decision.
            let inserted = sqlx::query_scalar!(
                r#"INSERT INTO routing.routing_decision (
                     id, user_id, instruction_id, account_id, account_name, outcome, reason_code,
                     multiplier, max_position_qty, position_before, requested_qty, executed_qty,
                     decided_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                   ON CONFLICT (instruction_id, account_id) DO NOTHING
                   RETURNING id"#,
                d.id,
                user.as_uuid(),
                d.instruction_id.as_uuid(),
                d.account_id.as_uuid(),
                d.account_name,
                d.outcome.as_str(),
                d.reason_code,
                int(d.multiplier)?,
                int(d.max_position_qty)?,
                i32::try_from(d.position_before)
                    .map_err(|_| PortError("position_before out of range".into()))?,
                d.requested_qty.map(int).transpose()?,
                d.executed_qty.map(int).transpose()?,
                d.decided_at,
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(port_error)?;
            if inserted.is_none() {
                continue;
            }
            sqlx::query!(
                r#"INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload)
                   VALUES ($1, $2, $3, $4, $5)"#,
                Uuid::now_v7(),
                user.as_uuid(),
                d.event_type,
                d.decided_at,
                d.payload,
            )
            .execute(&mut *tx)
            .await
            .map_err(port_error)?;
            new += 1;
        }
        tx.commit().await.map_err(port_error)?;
        Ok(new)
    }
}

fn int(v: u32) -> Result<i32, PortError> {
    i32::try_from(v).map_err(|_| PortError("quantity out of range".into()))
}

/// Database errors carry statement text and constraint names, never bound values.
fn port_error(e: sqlx::Error) -> PortError {
    PortError(e.to_string())
}
