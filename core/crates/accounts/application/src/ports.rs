//! Ports of the accounts use cases. The store is this module's own; the others are query ports
//! of other modules, defined here by the consumer (ADR-0006 §3.7) and wired in `crates/app`.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use thiserror::Error;
use uuid::Uuid;

use accounts_enterprise::preset_values::AccountRuleValue;
use accounts_enterprise::profile::AccountBracketProfile;
use accounts_enterprise::{
    Account, AccountId, AccountKind, ConnectionId, FirmId, FirmPlatform, MaxPositionQty,
    MoneyAmount, OptInKind, PresetVersionId, RuleField, RuleOrigin, RuleVersionRef, SizeAmount,
    VenueAccountId,
};
use shared_kernel::UserId;

use crate::events::AccountsEvent;

/// A dependency failed; the use case answers 500 and changes nothing.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("port unavailable: {0}")]
pub struct PortError(pub String);

// ---- own store ----------------------------------------------------------------------------

/// Unique-constraint refusals the store maps from the database (the race after a pre-check).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StoreError {
    #[error("accounts.name_taken")]
    NameTaken,
    #[error("accounts.platform_account_taken")]
    BindingTaken,
    /// `account_create_key`: a concurrent create with the same key committed first.
    #[error("create key taken")]
    CreateKeyTaken,
    /// `row_version` no longer matches.
    #[error("common.version_conflict")]
    VersionConflict,
    #[error("store: {0}")]
    Other(String),
}

/// A create replayed by its `Idempotency-Key` (S5 contract §2.2).
#[derive(Debug, Clone)]
pub struct CreateKeyRecord {
    /// As it is now, archived or not.
    pub account: Account,
    pub request_hash: [u8; 32],
}

/// The commission floor change of a write (rule value `commission_per_contract_side`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommissionWrite {
    Unchanged,
    /// Upsert the value with this origin.
    Set(RuleOrigin),
    /// Keep a tombstone (`is_unset`), so risk can schedule the removal (ADR-0006 §3.7).
    Unset(RuleOrigin),
}

/// The desired `account_rule_value` row of one field: `None` removes it (the value comes from the
/// preset again). The use case passes only the rows that change something (Malha, S16 (2)).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleValueWrite {
    pub field: RuleField,
    pub row: Option<AccountRuleValue>,
}

#[derive(Debug, Clone)]
pub struct NewAccount {
    pub account: Account,
    pub idempotency_key: Uuid,
    pub request_hash: [u8; 32],
    /// Initial stop/target profiles copied in the same transaction (S6, Torno).
    pub initial_profiles: Vec<AccountBracketProfile>,
    /// The rows a preset account needs (S16 AC-2, AC-6), written after the account.
    pub rule_values: Vec<RuleValueWrite>,
    pub events: Vec<AccountsEvent>,
}

/// An update of a live account: the new state, the `row_version` it was read at, and the
/// events written in the same transaction.
#[derive(Debug, Clone)]
pub struct AccountWrite {
    pub account: Account,
    pub expected_row_version: i32,
    /// The stored `limits_version`. With `rule_values`, the save also requires it: S13's
    /// `putAccountRiskRules` writes rule values without `row_version` (5.4.0), so `row_version`
    /// alone does not protect the diff the rows were computed from.
    pub expected_limits_version: i32,
    pub commission: CommissionWrite,
    /// Changed rows only; written after the account, in the same transaction.
    pub rule_values: Vec<RuleValueWrite>,
    pub events: Vec<AccountsEvent>,
}

#[async_trait]
pub trait AccountStore: Send + Sync {
    /// The caller's live accounts, sorted by `display_name` (case-insensitive).
    async fn list(&self, user_id: UserId) -> Result<Vec<Account>, StoreError>;

    /// A live account of the caller (`None` for unknown, archived or another user's).
    async fn get(&self, user_id: UserId, id: AccountId) -> Result<Option<Account>, StoreError>;

    async fn find_by_create_key(
        &self,
        user_id: UserId,
        key: Uuid,
    ) -> Result<Option<CreateKeyRecord>, StoreError>;

    /// Another live account of the caller has this name (case-insensitive).
    async fn name_taken(
        &self,
        user_id: UserId,
        name: &str,
        except: Option<AccountId>,
    ) -> Result<bool, StoreError>;

    /// Another live account of the caller is bound to this platform account.
    async fn binding_taken(
        &self,
        user_id: UserId,
        venue_account_id: VenueAccountId,
        except: Option<AccountId>,
    ) -> Result<bool, StoreError>;

    /// The account's `account_rule_value` rows (S16; empty for an unknown account).
    async fn rule_values(
        &self,
        user_id: UserId,
        id: AccountId,
    ) -> Result<Vec<AccountRuleValue>, StoreError>;

    /// The account's evaluation target, if it has one (rule value `evaluation_profit_target`,
    /// or its preset's; S13b).
    async fn evaluation_target(
        &self,
        user_id: UserId,
        id: AccountId,
    ) -> Result<Option<MoneyAmount>, StoreError>;

    /// Inserts the account, its rule values, its initial profiles and the events in one
    /// transaction.
    async fn create(&self, new: NewAccount) -> Result<(), StoreError>;

    /// Writes the new state if `row_version` still matches (bumping it), with its rule values and
    /// the events.
    async fn save(&self, write: AccountWrite) -> Result<(), StoreError>;
}

// ---- other modules' query ports -----------------------------------------------------------

/// Which rule versions apply: the generic ones (prop without a preset, Q40), or a preset's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleSelection {
    Generic,
    Preset {
        preset_version_id: PresetVersionId,
        firm_platform: Option<FirmPlatform>,
    },
}

/// Source of the rule text shown to the user (audited with each choice, spec O6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSource {
    pub url: String,
    pub accessed_at: chrono::NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentRule {
    pub rule: RuleVersionRef,
    pub sources: Vec<RuleSource>,
}

/// catalog: the opt-in rule versions in force (`catalog.opt_in_rule_version`).
#[async_trait]
pub trait RuleVersions: Send + Sync {
    /// The current version for the selection and kind (`None`: none published).
    async fn current(
        &self,
        selection: &RuleSelection,
        kind: OptInKind,
    ) -> Result<Option<CurrentRule>, PortError>;
}

/// catalog (S16): what an account needs from a preset version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetFacts {
    pub preset_version_id: PresetVersionId,
    pub firm_id: FirmId,
    pub firm_name: String,
    pub plan_name: String,
    /// Phase code (`evaluation`, `funded`, `live`).
    pub phase: String,
    pub size_amount: SizeAmount,
    pub max_position_qty: Option<MaxPositionQty>,
    /// Platforms with their own rules; empty when the rules do not vary by platform.
    pub firm_platforms: Vec<FirmPlatform>,
    /// Not superseded by a newer version of the same preset.
    pub is_current: bool,
    /// The preset that this version belongs to (for "same preset, newer version").
    pub preset_id: Uuid,
    /// Its resolved rules carry an evaluation target (S13b): only then do `evaluation_start` /
    /// `evaluation_earned_before` apply.
    pub has_evaluation_target: bool,
    /// The cap when the maximum position is not confirmed: the highest published value (S16
    /// §2.10); `None` when `max_position_qty` is verified.
    pub max_position_cap: Option<MaxPositionQty>,
    /// The same preset's version in the newest release; `is_current` ⇔ `None`.
    pub superseded_by: Option<PresetVersionId>,
    /// S16 AC-7: manual tracking only (no routing).
    pub manual_only: bool,
    pub release_version_number: i32,
    pub release_published_at: DateTime<Utc>,
    pub release_verified_on: chrono::NaiveDate,
    /// One set without `firm_platform`, or one per firm platform. The adapter converts the
    /// published documents; one that does not convert is a `PortError` (fail closed).
    pub rule_sets: Vec<PresetRuleSet>,
}

/// A preset version's rules for one firm platform (or for all).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetRuleSet {
    pub firm_platform: Option<FirmPlatform>,
    pub rules: accounts_enterprise::preset_values::PresetRules,
    /// The same rules as the contract's `PresetRules` (without the release files' quotes), shown
    /// as `preset_value` / the diff's preset values. Display only: no decision reads it; every
    /// rule decision uses the typed `rules`.
    pub wire: serde_json::Value,
}

#[async_trait]
pub trait PresetCatalog: Send + Sync {
    /// A published, not withdrawn preset version.
    async fn version(&self, id: PresetVersionId) -> Result<Option<PresetFacts>, PortError>;
}

/// gateway's kind of a platform account (ADR-0004 §8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VenueKind {
    Simulation,
    Playback,
    Live,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VenueAccountFacts {
    pub venue_account_id: VenueAccountId,
    pub kind: VenueKind,
}

/// The bound platform account as gateway last saw it (`AccountBinding`); strings pass through.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingView {
    pub venue_account_id: VenueAccountId,
    pub connection_id: ConnectionId,
    pub connection_name: String,
    pub connection_status: String,
    pub platform_account: String,
    pub kind: String,
    pub oco_location: String,
}

/// gateway (ADR-0008): query only; the binding itself is accounts' column.
#[async_trait]
pub trait VenueAccounts: Send + Sync {
    /// A platform account reported by one of the caller's paired connections. `None` for an
    /// unknown account, another user's connection, or an unpaired connection (all identical).
    async fn resolve(
        &self,
        user_id: UserId,
        connection_id: ConnectionId,
        platform_account: &str,
    ) -> Result<Option<VenueAccountFacts>, PortError>;

    async fn describe(
        &self,
        user_id: UserId,
        ids: &[VenueAccountId],
    ) -> Result<Vec<BindingView>, PortError>;

    /// Last known from the platform (Q38: a UX guard; the read is not atomic with the save).
    async fn has_open_position(
        &self,
        user_id: UserId,
        venue_account_id: VenueAccountId,
    ) -> Result<bool, PortError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteRef {
    pub source_id: Uuid,
    pub source_name: String,
}

/// routing (S7): subscriptions of an account.
#[async_trait]
pub trait AccountRoutes: Send + Sync {
    async fn routes_of(&self, user_id: UserId, id: AccountId) -> Result<Vec<RouteRef>, PortError>;

    /// An enabled route from an automated source feeds this account.
    async fn has_enabled_automated_route(
        &self,
        user_id: UserId,
        id: AccountId,
    ) -> Result<bool, PortError>;
}

/// identity: the user's time zone, for date defaults (evaluation start = registration date).
#[async_trait]
pub trait UserTimeZone: Send + Sync {
    async fn time_zone(&self, user_id: UserId) -> Result<Tz, PortError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InitialProfileError {
    #[error("accounts.profile_size_required")]
    ProfileSizeRequired,
    #[error("accounts.profile_size_invalid")]
    ProfileSizeInvalid,
    #[error("profiles: {0}")]
    Unavailable(String),
}

/// S6 (Torno): the initial stop/target profiles of a new account, copied from the user's
/// defaults of the nearest standard size (Q60).
#[async_trait]
pub trait InitialProfilePlanner: Send + Sync {
    async fn plan(
        &self,
        user_id: UserId,
        kind: AccountKind,
        size: &SizeAmount,
        profile_size: Option<&SizeAmount>,
        now: DateTime<Utc>,
    ) -> Result<Vec<AccountBracketProfile>, InitialProfileError>;
}
