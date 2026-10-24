//! ADR-0006 consumer side: derive an account's limits from its accounts inputs, and the
//! trading-day reset step. Both run the same derivation and write only through
//! `RiskStore::save_derivation` (only a newer derivation wins).

use std::sync::Arc;

use chrono::{DateTime, Utc};
use shared_kernel::{Clock, UserId};
use uuid::Uuid;

use risk_enterprise::derive;

use crate::ports::{
    AccountInputs, AccountRiskInputs, PortError, RiskStore, StoreError, StoredLimits, reset_of,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RiskError {
    #[error("risk port: {0}")]
    Port(String),
    #[error("risk store: {0}")]
    Store(String),
}

impl From<PortError> for RiskError {
    fn from(e: PortError) -> Self {
        Self::Port(e.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// A new derivation was written (`risk.limits_derived` emitted).
    Derived { version: i32 },
    /// Nothing newer to write (redelivered or out-of-order event, or the reset already ran).
    UpToDate,
    /// The account is unknown, archived or another user's.
    NoAccount,
}

#[derive(Clone)]
pub struct LimitsDeps {
    pub inputs: Arc<dyn AccountRiskInputs>,
    pub store: Arc<dyn RiskStore>,
    pub clock: Arc<dyn Clock>,
}

/// `account.limits_changed` consumer and the startup reconciliation (same path, ADR-0006 §3.2).
pub struct DeriveLimits {
    deps: LimitsDeps,
}

impl DeriveLimits {
    pub fn new(deps: LimitsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, user_id: UserId, account_id: Uuid) -> Result<Outcome, RiskError> {
        let d = &self.deps;
        let Some(inputs) = d.inputs.read(user_id, account_id).await? else {
            return Ok(Outcome::NoAccount);
        };
        let stored = d.store.limits(user_id, account_id).await.map_err(store)?;
        if stored
            .as_ref()
            .is_some_and(|s| s.derived_from_version >= inputs.limits_version)
        {
            return Ok(Outcome::UpToDate);
        }
        let now = d.clock.now();
        let next = derived(stored.as_ref(), &inputs, now, None);
        save(d, user_id, account_id, next).await
    }
}

/// One step per trading-day start (ADR-0006 §3.7): due schedules fall into the limits in force,
/// `effective_as_of` moves to the day's start, no `limits_version` bump. Ending the previous
/// day's blocks is the block store's part of the same step (S15).
pub struct ResetTradingDay {
    deps: LimitsDeps,
}

impl ResetTradingDay {
    pub fn new(deps: LimitsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, user_id: UserId, account_id: Uuid) -> Result<Outcome, RiskError> {
        let d = &self.deps;
        let Some(stored) = d.store.limits(user_id, account_id).await.map_err(store)? else {
            return Ok(Outcome::UpToDate);
        };
        let day_start = stored.reset().day_at(d.clock.now()).starts_at;
        if stored.effective_as_of >= day_start {
            return Ok(Outcome::UpToDate);
        }
        let Some(inputs) = d.inputs.read(user_id, account_id).await? else {
            return Ok(Outcome::NoAccount);
        };
        let next = derived(Some(&stored), &inputs, day_start, Some(day_start));
        save(d, user_id, account_id, next).await
    }
}

/// The next stored limits. `day_start` = `Some` for the reset step (the new `effective_as_of`).
fn derived(
    stored: Option<&StoredLimits>,
    inputs: &AccountInputs,
    now: DateTime<Utc>,
    day_start: Option<DateTime<Utc>>,
) -> StoredLimits {
    let reset = stored.map_or_else(
        || reset_of(inputs.kind, inputs.rules.rules.reset),
        StoredLimits::reset,
    );
    let derivation = derive(stored.map(|s| &s.in_force), &inputs.rules, reset, now);
    StoredLimits {
        version: stored.map_or(1, |s| s.version + 1),
        derived_from_version: inputs.limits_version,
        effective_as_of: day_start
            .or(stored.map(|s| s.effective_as_of))
            .unwrap_or_else(|| reset.day_at(now).starts_at),
        enforcement_enabled: stored.is_some_and(|s| s.enforcement_enabled),
        kind: inputs.kind,
        size_amount: inputs.size_amount,
        max_position_qty: inputs.max_position_qty,
        in_force: derivation.in_force,
        scheduled: derivation.scheduled,
        scheduled_values: inputs.rules.rules,
        evaluation_start: inputs.evaluation_start,
        evaluation_earned_before: inputs.evaluation_earned_before,
    }
}

/// One account's limits for the plugin: `version` is the plugin's token (monotonic, it moves at
/// the reset too); `derived_from_version` is what routing's gate compares (ADR-0006 §3.3).
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedLimits {
    pub version: i32,
    pub derived_from_version: i32,
    pub body: serde_json::Value,
}

/// Query for the gateway's `limits` frame after a welcome (one call for every mapped account). An
/// account without derived limits is absent: the plugin keeps its cache and routing's gate holds
/// entries (`routing.limits_pending`).
pub struct CurrentLimits {
    deps: LimitsDeps,
}

impl CurrentLimits {
    pub fn new(deps: LimitsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        user_id: UserId,
        account_ids: &[Uuid],
    ) -> Result<std::collections::HashMap<Uuid, DerivedLimits>, RiskError> {
        let d = &self.deps;
        let now = d.clock.now();
        let mut out = std::collections::HashMap::new();
        for &account_id in account_ids {
            let Some(limits) = d.store.limits(user_id, account_id).await.map_err(store)? else {
                continue;
            };
            let day_start = limits.reset().day_at(now).starts_at;
            let (e_start, tracker) = d
                .store
                .anchor(user_id, account_id, day_start)
                .await
                .map_err(store)?;
            let anchor = crate::wire::DayAnchor {
                current_start_at: day_start,
                e_start: e_start.unwrap_or(limits.size_amount),
                tracker,
            };
            out.insert(
                account_id,
                DerivedLimits {
                    version: limits.version,
                    derived_from_version: limits.derived_from_version,
                    body: crate::wire::limits_body(&limits, &anchor),
                },
            );
        }
        Ok(out)
    }
}

async fn save(
    d: &LimitsDeps,
    user_id: UserId,
    account_id: Uuid,
    next: StoredLimits,
) -> Result<Outcome, RiskError> {
    match d.store.save_derivation(user_id, account_id, &next).await {
        Ok(()) => Ok(Outcome::Derived {
            version: next.version,
        }),
        Err(StoreError::Stale) => Ok(Outcome::UpToDate),
        Err(e) => Err(store(e)),
    }
}

fn store(e: StoreError) -> RiskError {
    RiskError::Store(e.to_string())
}
