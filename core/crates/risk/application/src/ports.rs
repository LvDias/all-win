//! Ports of the risk module. Each trait belongs to its consumer (this module) and is implemented in
//! `crates/app` over the other module's query port (ADR-0006 §3.2/§3.7; no cross-module crate).

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use shared_kernel::UserId;
use thiserror::Error;
use uuid::Uuid;

use risk_enterprise::{ResetTime, RiskRules, RuleInputs, Scheduled};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{0}")]
pub struct PortError(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StoreError {
    /// The stored derivation is not older: a redelivered or out-of-order event, or a reset that
    /// already ran.
    #[error("stale derivation")]
    Stale,
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountKind {
    Prop,
    Real,
}

/// The account's inputs of its derived limits, read with their `limits_version` in ONE consistent
/// read from accounts (ADR-0006 §3.2).
#[derive(Debug, Clone, PartialEq)]
pub struct AccountInputs {
    pub kind: AccountKind,
    pub size_amount: Decimal,
    pub limits_version: i32,
    pub rules: RuleInputs,
    pub evaluation_start: Option<NaiveDate>,
    pub evaluation_earned_before: Option<Decimal>,
}

#[async_trait]
pub trait AccountRiskInputs: Send + Sync {
    /// `None` for an unknown, archived or another user's account.
    async fn read(
        &self,
        user_id: UserId,
        account_id: Uuid,
    ) -> Result<Option<AccountInputs>, PortError>;

    /// Every live account's `limits_version`, for the startup reconciliation and the 30 s monitor.
    async fn limits_versions(&self) -> Result<Vec<(UserId, Uuid, i32)>, PortError>;
}

/// The derived limits of one account (`risk.account_limits` + `risk.scheduled_limit`).
#[derive(Debug, Clone, PartialEq)]
pub struct StoredLimits {
    pub version: i32,
    pub derived_from_version: i32,
    pub effective_as_of: DateTime<Utc>,
    pub enforcement_enabled: bool,
    pub kind: AccountKind,
    pub size_amount: Decimal,
    pub in_force: RiskRules,
    pub scheduled: Vec<Scheduled>,
    /// The input value of each scheduled kind, kept to show `pending_changes` and to apply it at
    /// the reset.
    pub scheduled_values: RiskRules,
    pub evaluation_start: Option<NaiveDate>,
    pub evaluation_earned_before: Option<Decimal>,
}

impl StoredLimits {
    /// The reset in force: the firm's copy for prop accounts, the CME close otherwise.
    pub fn reset(&self) -> risk_enterprise::ResetSchedule {
        reset_of(self.kind, self.in_force.reset)
    }
}

pub fn reset_of(kind: AccountKind, reset: Option<ResetTime>) -> risk_enterprise::ResetSchedule {
    match (kind, reset) {
        (AccountKind::Prop, Some(r)) => risk_enterprise::ResetSchedule::Firm(r),
        _ => risk_enterprise::ResetSchedule::CmeSessionClose,
    }
}

#[async_trait]
pub trait RiskStore: Send + Sync {
    async fn limits(
        &self,
        user_id: UserId,
        account_id: Uuid,
    ) -> Result<Option<StoredLimits>, StoreError>;

    /// Writes the derivation only if (`derived_from_version`, `effective_as_of`) is newer than the
    /// stored pair (else `StoreError::Stale`), with `risk.limits_derived` in the outbox, in one
    /// transaction. The reset step keeps `derived_from_version` and moves `effective_as_of`.
    async fn save_derivation(
        &self,
        user_id: UserId,
        account_id: Uuid,
        limits: &StoredLimits,
    ) -> Result<(), StoreError>;
}
