//! Route configuration use cases (S7: `listRoutes`, `createRoute`, `updateRoute`, `deleteRoute`,
//! `listRouteCandidates`) and the Q67 b switch-off of an account that became ineligible.
//!
//! Facts owned by other modules come through consumer-owned ports wired in `crates/app`
//! (ADR-0001 §4). A port that is not wired yet (`None`) contributes no reason.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use routing_enterprise::{
    AccountId, EffectiveStatus, LifecycleState, OptIn, RouteFacts, RouteId, RouteReason,
    RouteSignalType, SignalTypes, SourceId, Uuid,
};
use serde_json::{Value, json};
use shared_kernel::{Clock, UserId};

// ---------------------------------------------------------------------------------------------
// Ports
// ---------------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
#[error("routing port: {0}")]
pub struct PortError(pub String);

/// A stored route (`routing.subscription`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteRecord {
    pub id: RouteId,
    pub user_id: UserId,
    pub source_id: SourceId,
    pub account_id: AccountId,
    pub signal_types: SignalTypes,
    pub is_enabled: bool,
    pub row_version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RouteFilter {
    pub source_id: Option<SourceId>,
    pub account_id: Option<AccountId>,
}

/// `routing.subscription_changed` `action`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeAction {
    Created,
    Updated,
    Enabled,
    Disabled,
    Removed,
}

impl ChangeAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Removed => "removed",
        }
    }
}

/// Who caused a change: the user, or the Core because the account became ineligible (Q67 b).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeCause {
    User,
    AccountIneligible,
}

/// The audit event of one route change, appended to the outbox in the change's transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteChange {
    pub route_id: RouteId,
    pub source_id: SourceId,
    /// Snapshots for the audit entry (required with the ids).
    pub source_name: String,
    pub account_id: AccountId,
    pub account_name: String,
    pub action: ChangeAction,
    pub cause: ChangeCause,
    pub previous: Option<(SignalTypes, bool)>,
    pub new: Option<(SignalTypes, bool)>,
    pub at: DateTime<Utc>,
}

impl RouteChange {
    pub const EVENT_TYPE: &'static str = "routing.subscription_changed";

    /// Outbox payload: the audit envelope `{actor, details, refs}` with `details` exactly
    /// `DetailsSubscriptionChanged` (s10 §5: actor `user` for the user's change, `system` for
    /// the Q67 b switch-off).
    pub fn payload(&self) -> Value {
        let (actor, cause) = match self.cause {
            ChangeCause::User => ("user", "user"),
            ChangeCause::AccountIneligible => ("system", "account_ineligible"),
        };
        let types = |t: &SignalTypes| t.iter().map(RouteSignalType::as_str).collect::<Vec<_>>();
        let mut details = serde_json::Map::new();
        details.insert("action".into(), json!(self.action.as_str()));
        details.insert("route_id".into(), json!(self.route_id));
        details.insert("source_id".into(), json!(self.source_id));
        details.insert("source_name".into(), json!(self.source_name));
        if let Some((t, enabled)) = &self.previous {
            details.insert("previous_signal_types".into(), json!(types(t)));
            details.insert("previous_is_enabled".into(), json!(enabled));
        }
        if let Some((t, enabled)) = &self.new {
            details.insert("signal_types".into(), json!(types(t)));
            details.insert("is_enabled".into(), json!(enabled));
        }
        details.insert("cause".into(), json!(cause));
        json!({
            "actor": actor,
            "details": details,
            "refs": { "account_id": self.account_id, "account_name": self.account_name },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateOutcome {
    Created,
    /// A route for (source, account) already exists; nothing was written.
    Exists,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateOutcome {
    Updated(RouteRecord),
    /// `row_version` did not match; nothing was written.
    VersionConflict,
    NotFound,
}

/// A full replacement of a route's mutable fields (optimistic concurrency).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteUpdate {
    pub user_id: UserId,
    pub id: RouteId,
    pub expected_version: i32,
    pub signal_types: SignalTypes,
    pub is_enabled: bool,
    pub at: DateTime<Utc>,
}

/// Persistence of routes; every write runs in one user-scoped transaction together with its
/// outbox event.
#[async_trait]
pub trait RouteStore: Send + Sync {
    async fn list(&self, user: UserId, filter: RouteFilter) -> Result<Vec<RouteRecord>, PortError>;
    async fn get(&self, user: UserId, id: RouteId) -> Result<Option<RouteRecord>, PortError>;
    async fn create(
        &self,
        route: &RouteRecord,
        change: &RouteChange,
    ) -> Result<CreateOutcome, PortError>;
    /// Replaces signal types and the switch when `expected_version` matches, bumping
    /// `row_version`; `change` is the audit event written in the same transaction.
    async fn update(
        &self,
        update: &RouteUpdate,
        change: Option<&RouteChange>,
    ) -> Result<UpdateOutcome, PortError>;
    async fn delete(
        &self,
        user: UserId,
        id: RouteId,
        change: &RouteChange,
    ) -> Result<bool, PortError>;
}

/// The caller's non-archived account, as routing needs it (S5; the app adapter may compose
/// the connection status from `identity`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountFacts {
    pub id: AccountId,
    pub display_name: String,
    /// The preset's verbatim reason when the firm is manual-only (S16 AC-7); such accounts are
    /// never candidates. Always `None` before S16.
    pub manual_only: Option<Value>,
    pub lifecycle: Option<LifecycleState>,
    pub platform_bound: bool,
    pub automation: OptIn,
    pub connection_connected: bool,
}

#[async_trait]
pub trait AccountDirectory: Send + Sync {
    /// Every non-archived account of `user`; another user's accounts never appear.
    async fn accounts(&self, user: UserId) -> Result<Vec<AccountFacts>, PortError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFacts {
    pub revoked: bool,
    pub automated: bool,
    /// The user's name for the source (audit snapshot).
    pub name: String,
}

#[async_trait]
pub trait SourceDirectory: Send + Sync {
    /// `None` for an unknown source or another user's.
    async fn source(&self, user: UserId, id: SourceId) -> Result<Option<SourceFacts>, PortError>;
}

/// Entry-only gates owned by later slices (sync S11, risk S13, limits ADR-0006).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EntryGateFacts {
    pub out_of_sync: Option<bool>,
    pub risk_blocked: Option<bool>,
    pub limits_pending: Option<bool>,
}

#[async_trait]
pub trait EntryGates: Send + Sync {
    async fn gates(
        &self,
        user: UserId,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, EntryGateFacts>, PortError>;
}

#[async_trait]
pub trait BracketCoverage: Send + Sync {
    /// Per account, the enabled instruments without a stop/target profile (S6).
    async fn missing(
        &self,
        user: UserId,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, Vec<Uuid>>, PortError>;
}

/// Why a `route_enable` step-up refused the request (mapped from the access port by the
/// HTTP adapter).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepUpDenied {
    Required,
    CodeInvalid,
    RateLimited { retry_after_s: u64 },
    Unavailable,
}

/// Checks and consumes the request's `route_enable` proof for one account, before anything
/// that can answer 409/422. Built per request by the HTTP adapter.
#[async_trait]
pub trait EnableAuthorizer: Send + Sync {
    async fn authorize(&self, account: AccountId) -> Result<(), StepUpDenied>;
}

// ---------------------------------------------------------------------------------------------
// Errors and views
// ---------------------------------------------------------------------------------------------

/// JSON pointer of a create/update refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pointer {
    SourceId,
    AccountId,
    IsEnabled,
}

impl Pointer {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceId => "/source_id",
            Self::AccountId => "/account_id",
            Self::IsEnabled => "/is_enabled",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RouteError {
    #[error("common.not_found")]
    NotFound,
    #[error("routing.route_exists")]
    RouteExists,
    #[error("common.version_conflict")]
    VersionConflict,
    /// 409 on update/delete, 422 `/source_id` on create.
    #[error("routing.source_revoked")]
    SourceRevoked { on_create: bool },
    #[error("routing.source_unknown")]
    SourceUnknown,
    #[error("routing.account_unknown")]
    AccountUnknown,
    #[error("routing.account_manual_only")]
    AccountManualOnly,
    /// `routing.automation_not_accepted` / `routing.platform_unbound`.
    #[error("{}", .reason.code())]
    NotEligible {
        reason: RouteReason,
        pointer: Pointer,
    },
    #[error("step-up denied")]
    StepUp(StepUpDenied),
    #[error(transparent)]
    Port(#[from] PortError),
}

/// A route with its effective status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteView {
    pub route: RouteRecord,
    pub effective: EffectiveStatus,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub account_id: AccountId,
    pub blocking_reasons: Vec<RouteReason>,
    pub status_if_routed: EffectiveStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates {
    pub source_id: SourceId,
    pub candidates: Vec<Candidate>,
    /// `(account, the preset's reason)`.
    pub manual_only: Vec<(AccountId, Value)>,
    pub computed_at: DateTime<Utc>,
}

pub struct CreateRouteCommand {
    pub source_id: SourceId,
    pub account_id: AccountId,
    pub signal_types: SignalTypes,
    pub is_enabled: bool,
}

pub struct UpdateRouteCommand {
    pub signal_types: SignalTypes,
    pub is_enabled: bool,
    pub row_version: i32,
}

// ---------------------------------------------------------------------------------------------
// Use cases
// ---------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct RoutingDeps {
    pub routes: Arc<dyn RouteStore>,
    pub accounts: Arc<dyn AccountDirectory>,
    pub sources: Arc<dyn SourceDirectory>,
    pub gates: Option<Arc<dyn EntryGates>>,
    pub brackets: Option<Arc<dyn BracketCoverage>>,
    pub clock: Arc<dyn Clock>,
}

/// Account facts plus the per-account entry gates, for one user.
struct FactBook {
    accounts: HashMap<AccountId, AccountFacts>,
    gates: HashMap<AccountId, EntryGateFacts>,
    brackets: Option<HashMap<AccountId, Vec<Uuid>>>,
}

impl RoutingDeps {
    async fn fact_book(&self, user: UserId) -> Result<FactBook, PortError> {
        let accounts: HashMap<_, _> = self
            .accounts
            .accounts(user)
            .await?
            .into_iter()
            .map(|a| (a.id, a))
            .collect();
        let ids: Vec<AccountId> = accounts.keys().copied().collect();
        let gates = match &self.gates {
            Some(g) => g.gates(user, &ids).await?,
            None => HashMap::new(),
        };
        let brackets = match &self.brackets {
            Some(b) => Some(b.missing(user, &ids).await?),
            None => None,
        };
        Ok(FactBook {
            accounts,
            gates,
            brackets,
        })
    }
}

impl FactBook {
    fn facts(&self, account: &AccountFacts, source: &SourceFacts) -> RouteFacts {
        let gates = self.gates.get(&account.id).copied().unwrap_or_default();
        RouteFacts {
            source_revoked: source.revoked,
            source_automated: source.automated,
            account_lifecycle: account.lifecycle,
            platform_bound: account.platform_bound,
            automation: account.automation,
            connection_connected: account.connection_connected,
            out_of_sync: gates.out_of_sync,
            risk_blocked: gates.risk_blocked,
            instruments_without_bracket: self
                .brackets
                .as_ref()
                .map(|m| m.get(&account.id).cloned().unwrap_or_default()),
            limits_pending: gates.limits_pending,
        }
    }

    fn display_name(&self, id: AccountId) -> &str {
        self.accounts
            .get(&id)
            .map_or("", |a| a.display_name.as_str())
    }
}

/// `listRoutes`: sorted by account display name. Routes of revoked sources are included.
pub struct ListRoutes {
    deps: RoutingDeps,
}

impl ListRoutes {
    pub fn new(deps: RoutingDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        user: UserId,
        filter: RouteFilter,
    ) -> Result<Vec<RouteView>, RouteError> {
        let routes = self.deps.routes.list(user, filter).await?;
        let book = self.deps.fact_book(user).await?;
        let now = self.deps.clock.now();
        let mut sources: HashMap<SourceId, Option<SourceFacts>> = HashMap::new();
        let mut views = Vec::with_capacity(routes.len());
        for route in routes {
            let source = match sources.get(&route.source_id) {
                Some(s) => s.clone(),
                None => {
                    let s = self.deps.sources.source(user, route.source_id).await?;
                    sources.insert(route.source_id, s.clone());
                    s
                }
            };
            // An archived account's routes no longer exist (S5 archiveAccount); a missing
            // account or source here is a transient race and is skipped.
            let (Some(account), Some(source)) = (book.accounts.get(&route.account_id), source)
            else {
                continue;
            };
            let effective = book.facts(account, &source).status_of(route.is_enabled);
            views.push(RouteView {
                route,
                effective,
                computed_at: now,
            });
        }
        views.sort_by(|a, b| {
            book.display_name(a.route.account_id)
                .cmp(book.display_name(b.route.account_id))
                .then(a.route.id.cmp(&b.route.id))
        });
        Ok(views)
    }
}

/// `listRouteCandidates`.
pub struct ListRouteCandidates {
    deps: RoutingDeps,
}

impl ListRouteCandidates {
    pub fn new(deps: RoutingDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        user: UserId,
        source_id: SourceId,
    ) -> Result<Candidates, RouteError> {
        let source = self
            .deps
            .sources
            .source(user, source_id)
            .await?
            .ok_or(RouteError::NotFound)?;
        let routed: Vec<AccountId> = self
            .deps
            .routes
            .list(
                user,
                RouteFilter {
                    source_id: Some(source_id),
                    account_id: None,
                },
            )
            .await?
            .into_iter()
            .map(|r| r.account_id)
            .collect();
        let book = self.deps.fact_book(user).await?;
        let mut accounts: Vec<&AccountFacts> = book
            .accounts
            .values()
            .filter(|a| !routed.contains(&a.id))
            .collect();
        accounts.sort_by(|a, b| a.display_name.cmp(&b.display_name).then(a.id.cmp(&b.id)));
        let mut candidates = Vec::new();
        let mut manual_only = Vec::new();
        for account in accounts {
            if let Some(reason) = &account.manual_only {
                manual_only.push((account.id, reason.clone()));
                continue;
            }
            let facts = book.facts(account, &source);
            candidates.push(Candidate {
                account_id: account.id,
                blocking_reasons: facts.blocking_reasons(),
                status_if_routed: facts.status_if_routed(),
            });
        }
        Ok(Candidates {
            source_id,
            candidates,
            manual_only,
            computed_at: self.deps.clock.now(),
        })
    }
}

/// `createRoute`.
pub struct CreateRoute {
    deps: RoutingDeps,
}

impl CreateRoute {
    pub fn new(deps: RoutingDeps) -> Self {
        Self { deps }
    }

    /// Order: 422 source/account unknown → step-up → 422 revoked/manual-only/ineligible →
    /// 409 exists (the natural key is checked by the store, last).
    pub async fn handle(
        &self,
        user: UserId,
        cmd: CreateRouteCommand,
        step_up: &dyn EnableAuthorizer,
    ) -> Result<RouteView, RouteError> {
        let source = self
            .deps
            .sources
            .source(user, cmd.source_id)
            .await?
            .ok_or(RouteError::SourceUnknown)?;
        let book = self.deps.fact_book(user).await?;
        let account = book
            .accounts
            .get(&cmd.account_id)
            .ok_or(RouteError::AccountUnknown)?;
        let facts = book.facts(account, &source);
        if cmd.is_enabled && facts.enable_needs_step_up() {
            step_up
                .authorize(cmd.account_id)
                .await
                .map_err(RouteError::StepUp)?;
        }
        if source.revoked {
            return Err(RouteError::SourceRevoked { on_create: true });
        }
        if account.manual_only.is_some() {
            return Err(RouteError::AccountManualOnly);
        }
        if let Some(reason) = facts.enable_refusal() {
            return Err(RouteError::NotEligible {
                reason,
                pointer: Pointer::AccountId,
            });
        }
        let now = self.deps.clock.now();
        let route = RouteRecord {
            id: RouteId::new_v7(),
            user_id: user,
            source_id: cmd.source_id,
            account_id: cmd.account_id,
            signal_types: cmd.signal_types,
            is_enabled: cmd.is_enabled,
            row_version: 1,
            created_at: now,
            updated_at: now,
        };
        let change = RouteChange {
            route_id: route.id,
            source_id: route.source_id,
            source_name: source.name.clone(),
            account_id: route.account_id,
            account_name: account.display_name.clone(),
            action: ChangeAction::Created,
            cause: ChangeCause::User,
            previous: None,
            new: Some((route.signal_types.clone(), route.is_enabled)),
            at: now,
        };
        match self.deps.routes.create(&route, &change).await? {
            CreateOutcome::Exists => Err(RouteError::RouteExists),
            CreateOutcome::Created => Ok(RouteView {
                effective: facts.status_of(route.is_enabled),
                route,
                computed_at: now,
            }),
        }
    }
}

/// `updateRoute`.
pub struct UpdateRoute {
    deps: RoutingDeps,
}

impl UpdateRoute {
    pub fn new(deps: RoutingDeps) -> Self {
        Self { deps }
    }

    /// Order: 404 → step-up (switching on) → 409 revoked → 409 version → 422 not eligible.
    pub async fn handle(
        &self,
        user: UserId,
        id: RouteId,
        cmd: UpdateRouteCommand,
        step_up: &dyn EnableAuthorizer,
    ) -> Result<RouteView, RouteError> {
        let current = self
            .deps
            .routes
            .get(user, id)
            .await?
            .ok_or(RouteError::NotFound)?;
        let source = self
            .deps
            .sources
            .source(user, current.source_id)
            .await?
            .ok_or(RouteError::NotFound)?;
        let book = self.deps.fact_book(user).await?;
        let account = book
            .accounts
            .get(&current.account_id)
            .ok_or(RouteError::NotFound)?;
        let facts = book.facts(account, &source);
        let switching_on = cmd.is_enabled && !current.is_enabled;
        if switching_on && facts.enable_needs_step_up() {
            step_up
                .authorize(current.account_id)
                .await
                .map_err(RouteError::StepUp)?;
        }
        if source.revoked {
            return Err(RouteError::SourceRevoked { on_create: false });
        }
        if cmd.row_version != current.row_version {
            return Err(RouteError::VersionConflict);
        }
        if switching_on && let Some(reason) = facts.enable_refusal() {
            return Err(RouteError::NotEligible {
                reason,
                pointer: Pointer::IsEnabled,
            });
        }
        let now = self.deps.clock.now();
        let action = match (current.is_enabled, cmd.is_enabled) {
            (false, true) => Some(ChangeAction::Enabled),
            (true, false) => Some(ChangeAction::Disabled),
            _ if current.signal_types != cmd.signal_types => Some(ChangeAction::Updated),
            _ => None,
        };
        let change = action.map(|action| RouteChange {
            route_id: id,
            source_id: current.source_id,
            source_name: source.name.clone(),
            account_id: current.account_id,
            account_name: account.display_name.clone(),
            action,
            cause: ChangeCause::User,
            previous: Some((current.signal_types.clone(), current.is_enabled)),
            new: Some((cmd.signal_types.clone(), cmd.is_enabled)),
            at: now,
        });
        if change.is_none() {
            // A request that changes nothing is not audited and does not bump the version.
            return Ok(RouteView {
                effective: facts.status_of(current.is_enabled),
                route: current,
                computed_at: now,
            });
        }
        match self
            .deps
            .routes
            .update(
                &RouteUpdate {
                    user_id: user,
                    id,
                    expected_version: cmd.row_version,
                    signal_types: cmd.signal_types,
                    is_enabled: cmd.is_enabled,
                    at: now,
                },
                change.as_ref(),
            )
            .await?
        {
            UpdateOutcome::Updated(route) => Ok(RouteView {
                effective: facts.status_of(route.is_enabled),
                route,
                computed_at: now,
            }),
            UpdateOutcome::VersionConflict => Err(RouteError::VersionConflict),
            UpdateOutcome::NotFound => Err(RouteError::NotFound),
        }
    }
}

/// `deleteRoute`: open positions and orders are not changed.
pub struct DeleteRoute {
    deps: RoutingDeps,
}

impl DeleteRoute {
    pub fn new(deps: RoutingDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, user: UserId, id: RouteId) -> Result<(), RouteError> {
        let current = self
            .deps
            .routes
            .get(user, id)
            .await?
            .ok_or(RouteError::NotFound)?;
        let source = self.deps.sources.source(user, current.source_id).await?;
        let Some(source) = source.filter(|s| !s.revoked) else {
            return Err(RouteError::SourceRevoked { on_create: false });
        };
        let account_name = self
            .deps
            .accounts
            .accounts(user)
            .await?
            .into_iter()
            .find(|a| a.id == current.account_id)
            .map(|a| a.display_name)
            .ok_or(RouteError::NotFound)?;
        let change = RouteChange {
            route_id: id,
            source_id: current.source_id,
            source_name: source.name,
            account_id: current.account_id,
            account_name,
            action: ChangeAction::Removed,
            cause: ChangeCause::User,
            previous: Some((current.signal_types, current.is_enabled)),
            new: None,
            at: self.deps.clock.now(),
        };
        if self.deps.routes.delete(user, id, &change).await? {
            Ok(())
        } else {
            Err(RouteError::NotFound)
        }
    }
}

/// Q67 b: switches off the account's enabled routes from sources it is no longer eligible
/// for, audited with cause `account_ineligible`. They stay off when it becomes eligible again.
pub struct SwitchOffIneligibleRoutes {
    deps: RoutingDeps,
}

impl SwitchOffIneligibleRoutes {
    pub fn new(deps: RoutingDeps) -> Self {
        Self { deps }
    }

    /// Returns the routes switched off. Safe to repeat (an already-off route is left alone).
    pub async fn handle(
        &self,
        user: UserId,
        account_id: AccountId,
    ) -> Result<Vec<RouteId>, RouteError> {
        let routes = self
            .deps
            .routes
            .list(
                user,
                RouteFilter {
                    source_id: None,
                    account_id: Some(account_id),
                },
            )
            .await?;
        let book = self.deps.fact_book(user).await?;
        let Some(account) = book.accounts.get(&account_id) else {
            return Ok(Vec::new());
        };
        let mut off = Vec::new();
        for route in routes.into_iter().filter(|r| r.is_enabled) {
            let Some(source) = self.deps.sources.source(user, route.source_id).await? else {
                continue;
            };
            if !book.facts(account, &source).must_switch_off() {
                continue;
            }
            let now = self.deps.clock.now();
            let change = RouteChange {
                route_id: route.id,
                source_id: route.source_id,
                source_name: source.name.clone(),
                account_id,
                account_name: account.display_name.clone(),
                action: ChangeAction::Disabled,
                cause: ChangeCause::AccountIneligible,
                previous: Some((route.signal_types.clone(), true)),
                new: Some((route.signal_types.clone(), false)),
                at: now,
            };
            if let UpdateOutcome::Updated(_) = self
                .deps
                .routes
                .update(
                    &RouteUpdate {
                        user_id: user,
                        id: route.id,
                        expected_version: route.row_version,
                        signal_types: route.signal_types.clone(),
                        is_enabled: false,
                        at: now,
                    },
                    Some(&change),
                )
                .await?
            {
                off.push(route.id);
            }
        }
        Ok(off)
    }
}

#[cfg(test)]
#[path = "routes_tests.rs"]
mod tests;
