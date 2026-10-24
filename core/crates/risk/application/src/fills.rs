//! `gateway.execution_reported` consumer (E1, Bigorna): each platform fill is applied once to the
//! account's position and trading day (data model rev. 16.23 §5.6). The dedupe is the database's:
//! `risk.applied_fill` (account, fill_id). Manual trades never reach here (S13 AC-2).
//!
//! - add: position before (the platform first), average-cost gross, the trading day fixed by the
//!   fill's own time, the position set to the platform's `position_after` / `avg_price_after`.
//! - amend: the fill's gross is recomputed from its own stored inputs; the difference goes to the
//!   day it was first applied to.
//! - remove: the row is zeroed and kept as the dedupe record.
//!
//! An unknown cost basis realizes 0 and makes the account's P&L stale (routing skips entries).

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use shared_kernel::UserId;
use uuid::Uuid;

use risk_enterprise::{Position, Side, apply_fill, position_before, reconcile, round_money};

use crate::limits::RiskError;
use crate::ports::{PortError, RiskStore, StoreError};

/// The fill part of one `gateway.execution_reported` event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportedFill {
    pub event_id: Uuid,
    pub account_id: Uuid,
    pub fill_id: String,
    pub operation: FillOperation,
    pub instrument_symbol: String,
    pub contract_month: NaiveDate,
    pub side: Side,
    pub qty: i64,
    pub price: Decimal,
    pub reported_commission: Option<Decimal>,
    pub commission_effective: Decimal,
    pub position_after: Option<i64>,
    pub avg_price_after: Option<Decimal>,
    /// `platform_time` when the adapter sent it (Playback follows it), else `occurred_at`.
    pub at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillOperation {
    Add,
    Amend,
    Remove,
}

/// The contract's price facts (catalog, through the app).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstrumentFacts {
    pub point_value: Decimal,
    pub tick_size: Decimal,
}

#[async_trait]
pub trait InstrumentPoints: Send + Sync {
    /// `None` for an unknown symbol (the fill is not applied; it is retried).
    async fn facts(&self, symbol: &str) -> Result<Option<InstrumentFacts>, PortError>;
}

/// One stored `risk.applied_fill` row (what was applied, and the inputs of its gross).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedFill {
    pub fill_id: String,
    pub trading_date: NaiveDate,
    pub qty: i64,
    pub gross: Decimal,
    pub commission_effective: Decimal,
    pub commission_gap: Decimal,
    pub instrument_symbol: String,
    pub contract_month: NaiveDate,
    pub side: Side,
    pub price: Decimal,
    pub position_before: Option<i64>,
    pub avg_price_before: Option<Decimal>,
    pub point_value: Decimal,
    pub last_event_id: Uuid,
    pub removed: bool,
}

/// The write of one applied report, in one risk transaction: the fill row (insert for an add,
/// update for an amend/remove), the trading day's sums by `delta_*`, and the position.
#[derive(Debug, Clone, PartialEq)]
pub struct FillWrite {
    pub row: AppliedFill,
    /// A new row (an add): the trading day `[start, end)` it falls in, created if missing. An
    /// amend/remove keeps the row's own day.
    pub new_day: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub delta_gross: Decimal,
    pub delta_commission: Decimal,
    pub delta_gap: Decimal,
    /// `None` when nothing is known about the position after the report.
    pub position: Option<Position>,
    /// The cost basis became unknown or a reducing fill could not be realized: P&L is stale.
    pub stale: bool,
}

#[async_trait]
pub trait FillStore: Send + Sync {
    async fn applied_fill(
        &self,
        user_id: UserId,
        account_id: Uuid,
        fill_id: &str,
    ) -> Result<Option<AppliedFill>, StoreError>;

    async fn position(
        &self,
        user_id: UserId,
        account_id: Uuid,
        symbol: &str,
        contract_month: NaiveDate,
    ) -> Result<Option<Position>, StoreError>;

    /// `StoreError::Stale` when the insert of a new row conflicts (a concurrent delivery won).
    async fn write_fill(
        &self,
        user_id: UserId,
        account_id: Uuid,
        write: &FillWrite,
    ) -> Result<(), StoreError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillOutcome {
    Applied {
        stale: bool,
    },
    /// The same add again (redelivery): nothing written.
    Duplicate,
    /// An amend/remove of a fill risk never applied (the gateway refuses those; fail closed).
    UnknownFill,
    /// The account has no derived limits yet (no trading day to cut by): retried later.
    NoLimits,
    /// The catalog does not know the symbol: retried later.
    UnknownInstrument,
}

#[derive(Clone)]
pub struct FillsDeps {
    pub limits: Arc<dyn RiskStore>,
    pub fills: Arc<dyn FillStore>,
    pub instruments: Arc<dyn InstrumentPoints>,
}

pub struct ApplyFill {
    deps: FillsDeps,
}

impl ApplyFill {
    pub fn new(deps: FillsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        user_id: UserId,
        f: &ReportedFill,
    ) -> Result<FillOutcome, RiskError> {
        let d = &self.deps;
        let store = |e: StoreError| RiskError::Store(e.to_string());
        let existing = d
            .fills
            .applied_fill(user_id, f.account_id, &f.fill_id)
            .await
            .map_err(store)?;
        let Some(limits) = d
            .limits
            .limits(user_id, f.account_id)
            .await
            .map_err(store)?
        else {
            return Ok(FillOutcome::NoLimits);
        };
        let reset = limits.reset();
        let write = match (f.operation, existing) {
            (FillOperation::Add, Some(_)) => return Ok(FillOutcome::Duplicate),
            (FillOperation::Amend | FillOperation::Remove, None) => {
                return Ok(FillOutcome::UnknownFill);
            }
            (FillOperation::Add, None) => {
                let Some(facts) = d.instruments.facts(&f.instrument_symbol).await? else {
                    return Ok(FillOutcome::UnknownInstrument);
                };
                let stored = d
                    .fills
                    .position(
                        user_id,
                        f.account_id,
                        &f.instrument_symbol,
                        f.contract_month,
                    )
                    .await
                    .map_err(store)?;
                let day = reset.day_at(f.at);
                add(f, stored, facts, day.date, day.starts_at, day.ends_at)
            }
            (op, Some(old)) => {
                let tick = d
                    .instruments
                    .facts(&old.instrument_symbol)
                    .await?
                    .map_or(Decimal::ZERO, |i| i.tick_size);
                correct(f, op, old, tick)
            }
        };
        match d.fills.write_fill(user_id, f.account_id, &write).await {
            Ok(()) => Ok(FillOutcome::Applied { stale: write.stale }),
            Err(StoreError::Stale) => Ok(FillOutcome::Duplicate),
            Err(e) => Err(store(e)),
        }
    }
}

/// `effective − reported`: what the platform balance lacks (0 when the platform's covers it).
fn gap(f: &ReportedFill) -> Decimal {
    (f.commission_effective - f.reported_commission.unwrap_or(Decimal::ZERO)).max(Decimal::ZERO)
}

fn add(
    f: &ReportedFill,
    stored: Option<Position>,
    facts: InstrumentFacts,
    date: NaiveDate,
    day_start: DateTime<Utc>,
    day_end: DateTime<Utc>,
) -> FillWrite {
    let before = position_before(stored, f.side, f.qty, f.position_after, f.avg_price_after);
    let effect = before.map(|b| apply_fill(b, f.side, f.qty, f.price, facts.point_value));
    let realized = effect.and_then(|e| e.realized_gross);
    let computed = effect.map(|e| e.after);
    let position = match (computed, f.position_after) {
        (Some(c), _) => {
            Some(reconcile(c, f.position_after, f.avg_price_after, facts.tick_size).position)
        }
        (None, Some(q)) => Some(platform_only(q, f.avg_price_after)),
        (None, None) => None,
    };
    let gross = realized.unwrap_or(Decimal::ZERO);
    let known_before = before.filter(|b| b.basis_known);
    // Unknown: the position before, a reducing fill's basis, or the basis after.
    let stale = before.is_none()
        || (effect.is_some() && realized.is_none())
        || position.is_some_and(|p| !p.basis_known);
    FillWrite {
        row: AppliedFill {
            fill_id: f.fill_id.clone(),
            trading_date: date,
            qty: f.qty,
            gross,
            commission_effective: f.commission_effective,
            commission_gap: gap(f),
            instrument_symbol: f.instrument_symbol.clone(),
            contract_month: f.contract_month,
            side: f.side,
            price: f.price,
            position_before: before.map(|b| b.qty),
            avg_price_before: known_before.filter(|b| b.qty != 0).map(|b| b.avg_price),
            point_value: facts.point_value,
            last_event_id: f.event_id,
            removed: false,
        },
        new_day: Some((day_start, day_end)),
        delta_gross: gross,
        delta_commission: f.commission_effective,
        delta_gap: gap(f),
        position,
        stale,
    }
}

/// An amend recomputes the fill's own gross from its stored inputs; a remove zeroes it. Either way
/// the position follows the report (the platform's figures are authoritative for a correction).
fn correct(f: &ReportedFill, op: FillOperation, old: AppliedFill, tick: Decimal) -> FillWrite {
    let (qty, price, commission, gap_now, gross, unknown) = if op == FillOperation::Remove {
        (
            0,
            old.price,
            Decimal::ZERO,
            Decimal::ZERO,
            Decimal::ZERO,
            false,
        )
    } else {
        let closes = old
            .position_before
            .is_some_and(|b| b != 0 && (b > 0) != (old.side == Side::Buy));
        let recomputed = match (old.position_before, old.avg_price_before) {
            (Some(b), Some(avg)) if closes => {
                let closed = b.abs().min(f.qty);
                let direction = if b > 0 { Decimal::ONE } else { -Decimal::ONE };
                Some(round_money(
                    Decimal::from(closed) * (f.price - avg) * direction * old.point_value,
                ))
            }
            _ if closes => None,
            _ => Some(Decimal::ZERO),
        };
        (
            f.qty,
            f.price,
            f.commission_effective,
            gap(f),
            recomputed.unwrap_or(Decimal::ZERO),
            recomputed.is_none(),
        )
    };
    let position = f
        .position_after
        .map(|q| reconcile(platform_only(q, None), Some(q), f.avg_price_after, tick).position);
    let stale = unknown || position.is_some_and(|p| !p.basis_known);
    FillWrite {
        delta_gross: gross - old.gross,
        delta_commission: commission - old.commission_effective,
        delta_gap: gap_now - old.commission_gap,
        row: AppliedFill {
            qty,
            gross,
            commission_effective: commission,
            commission_gap: gap_now,
            price,
            last_event_id: f.event_id,
            removed: op == FillOperation::Remove,
            ..old
        },
        new_day: None,
        position,
        stale,
    }
}

/// The platform's position alone: an open one without its average price has an unknown basis.
fn platform_only(qty: i64, avg: Option<Decimal>) -> Position {
    match (qty, avg) {
        (0, _) => Position::FLAT,
        (q, Some(a)) => Position {
            qty: q,
            avg_price: a,
            basis_known: true,
        },
        (q, None) => Position {
            qty: q,
            avg_price: Decimal::ZERO,
            basis_known: false,
        },
    }
}
