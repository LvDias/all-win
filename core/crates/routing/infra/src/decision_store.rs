//! Routing decisions (`routing.routing_decision`, wave1-data-model §6.2): append-only, one row
//! per (instruction, account), or a `close` then an `open` row for a two-step `reverse`
//! (ADR-0003 §4.1 r4), written with its audit event in one user-scoped transaction.

use async_trait::async_trait;
use routing_application::decisions::{
    DecidedLeg, DecisionRecord, DecisionStore, InstructionToRoute, Leg,
};
use routing_application::enterprise::{AccountId, InstructionId, SourceId};
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
                      expiry_base, to_char(contract_month, 'YYYY-MM') AS contract_month,
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
            contract_month: r.contract_month,
            quantity: r.quantity,
            expiry_base: r.expiry_base,
        }))
    }

    async fn decided(
        &self,
        user: UserId,
        instruction: InstructionId,
    ) -> Result<Vec<DecidedLeg>, PortError> {
        let mut tx = self.begin_for(user).await?;
        let rows = sqlx::query!(
            r#"SELECT account_id, leg, position_before FROM routing.routing_decision
                WHERE user_id = $1 AND instruction_id = $2"#,
            user.as_uuid(),
            instruction.as_uuid(),
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(port_error)?;
        tx.commit().await.map_err(port_error)?;
        rows.into_iter()
            .map(|r| {
                Ok(DecidedLeg {
                    account_id: AccountId::from_uuid(r.account_id),
                    leg: match r.leg.as_str() {
                        "whole" => Leg::Whole,
                        "close" => Leg::Close,
                        "open" => Leg::Open,
                        _ => return Err(PortError("unknown decision leg".into())),
                    },
                    position_before: r.position_before.map_or(0, i64::from),
                })
            })
            .collect()
    }

    async fn record(&self, user: UserId, decisions: &[DecisionRecord]) -> Result<usize, PortError> {
        let mut tx = self.begin_for(user).await?;
        let mut new = 0;
        for d in decisions {
            // Core rule (a CHECK cannot see the other row): an opening only follows its close
            // leg, read in this transaction.
            if d.leg == Leg::Open {
                let close = sqlx::query_scalar!(
                    r#"SELECT EXISTS (SELECT 1 FROM routing.routing_decision
                        WHERE user_id = $1 AND instruction_id = $2 AND account_id = $3
                          AND leg = 'close') AS "close!""#,
                    user.as_uuid(),
                    d.instruction_id.as_uuid(),
                    d.account_id.as_uuid(),
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(port_error)?;
                if !close {
                    return Err(PortError("an open leg needs its close leg".into()));
                }
            }
            // A concurrent redelivery blocks on the unique index (`routing_decision_leg_key`)
            // and then takes DO NOTHING, so exactly one transaction audits each decision.
            let inserted = sqlx::query_scalar!(
                r#"INSERT INTO routing.routing_decision (
                     id, user_id, instruction_id, account_id, account_name, outcome, reason_code,
                     multiplier, max_position_qty, position_before, requested_qty, executed_qty,
                     decided_at, leg)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                   ON CONFLICT (instruction_id, account_id,
                                (CASE leg WHEN 'open' THEN 'open' ELSE 'first' END)) DO NOTHING
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
                d.position_before
                    .map(i32::try_from)
                    .transpose()
                    .map_err(|_| PortError("position_before out of range".into()))?,
                d.requested_qty.map(int).transpose()?,
                d.executed_qty.map(int).transpose()?,
                d.decided_at,
                d.leg.as_str(),
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(port_error)?;
            if inserted.is_none() {
                continue;
            }
            new += 1;
            let Some((event_type, payload)) = &d.audit else {
                continue;
            };
            sqlx::query!(
                r#"INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload)
                   VALUES ($1, $2, $3, $4, $5)"#,
                Uuid::now_v7(),
                user.as_uuid(),
                event_type,
                d.decided_at,
                payload,
            )
            .execute(&mut *tx)
            .await
            .map_err(port_error)?;
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
