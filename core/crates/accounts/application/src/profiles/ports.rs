//! Ports of the profile use cases: the profile store (this module's tables) and the catalog query
//! port (instrument tick data and standard sizes), defined here by the consumer and wired in
//! `crates/app` (ADR-0006 §3.7).

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use accounts_enterprise::profile::{
    AccountBracketProfile, DefaultProfile, DistanceProfile, TickSpec,
};
use accounts_enterprise::{AccountId, AccountKind, AccountName, SizeAmount};
use shared_kernel::UserId;

use super::events::ProfileEvent;
use crate::ports::PortError;

/// An enabled instrument and its exact tick data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnabledInstrument {
    pub id: Uuid,
    pub ticks: TickSpec,
}

/// catalog: `catalog.instrument` (enabled) and `catalog.standard_account_size`.
#[async_trait]
pub trait ProfileCatalog: Send + Sync {
    /// Enabled instruments in `listInstruments` order (by symbol).
    async fn enabled_instruments(&self) -> Result<Vec<EnabledInstrument>, PortError>;
    async fn standard_sizes(&self) -> Result<Vec<SizeAmount>, PortError>;
}

/// What a profile needs from its live account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountFacts {
    pub kind: AccountKind,
    pub size_amount: SizeAmount,
    pub display_name: AccountName,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ProfileStoreError {
    /// Unknown, archived, or another user's account.
    #[error("account not found")]
    AccountNotFound,
    /// The row changed since it was read (`row_version`, or the default's previous values).
    #[error("raced")]
    Raced,
    #[error("store: {0}")]
    Other(String),
}

/// Upsert of one default, conditional on the values read (`None`: it did not exist).
#[derive(Debug, Clone)]
pub struct DefaultWrite {
    pub user_id: UserId,
    pub expected: Option<DistanceProfile>,
    pub default: DefaultProfile,
    pub event: ProfileEvent,
}

/// Write of an account profile, conditional on `row_version` (`None`: it did not exist).
#[derive(Debug, Clone)]
pub struct AccountProfileWrite {
    pub user_id: UserId,
    pub account_id: AccountId,
    pub expected_row_version: Option<i32>,
    pub profile: AccountBracketProfile,
    pub event: ProfileEvent,
}

/// The profiles of one live account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveAccountProfiles {
    pub account_id: AccountId,
    pub profiles: Vec<AccountBracketProfile>,
}

/// `accounts.default_bracket_profile` / `accounts.account_bracket_profile`, user-scoped (RLS).
#[async_trait]
pub trait ProfileStore: Send + Sync {
    /// The user's defaults, of one instrument or all.
    async fn defaults(
        &self,
        user_id: UserId,
        instrument_id: Option<Uuid>,
    ) -> Result<Vec<DefaultProfile>, ProfileStoreError>;

    /// `Raced` when the stored default no longer equals `expected`.
    async fn put_default(&self, write: DefaultWrite) -> Result<(), ProfileStoreError>;

    /// The live account's facts and profiles (`None`: unknown, archived or another user's).
    async fn account_profiles(
        &self,
        user_id: UserId,
        account_id: AccountId,
    ) -> Result<Option<(AccountFacts, Vec<AccountBracketProfile>)>, ProfileStoreError>;

    /// `Raced` on a stale `row_version`; `AccountNotFound` when the account is gone.
    async fn write_account_profile(
        &self,
        write: AccountProfileWrite,
    ) -> Result<(), ProfileStoreError>;

    /// The live accounts among `account_ids` with their profiles for `instrument_ids` (execution).
    async fn live_profiles(
        &self,
        user_id: UserId,
        account_ids: &[AccountId],
        instrument_ids: &[Uuid],
    ) -> Result<Vec<LiveAccountProfiles>, ProfileStoreError>;
}
