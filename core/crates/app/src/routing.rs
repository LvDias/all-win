//! Wiring of the routing module (S7): the adapters of its ports over accounts (S5), profiles
//! (S6), identity sources and the catalog rollover; the decision consumer of `ingress.received`;
//! the routes router; and accounts' `AccountRoutes` port. The gateway's ports (expected position,
//! gate 6, account kind, connection state) come from `gateway.rs`; gates 7–8 come from the
//! caller (real, or the test-build stand-ins behind `dev-stubs`).

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use access::{SessionAuthenticator, StepUpGate};
use accounts_application::enterprise::opt_in::{Choice, OptInState};
use accounts_application::enterprise::{
    Account, AccountId as AccountsAccountId, LifecycleState as Lifecycle,
};
use accounts_application::ports::{
    AccountRoutes, AccountStore, PortError as AccountsPortError, RouteRef,
};
use accounts_application::profiles::BracketQuery;
use async_trait::async_trait;
use axum::Router;
use catalog_application::instrument::{ActiveContractQuery, ListInstruments};
use chrono::{DateTime, Utc};
use identity_application::ports::IdentityStore;
use routing_application::commands::Protection;
use routing_application::decisions::{
    ActiveContract, BracketFor, Contract, DecideError, DecideInstruction, DecisionDeps,
    EXIT_RETRY_CAP, EntryGate, EntryGatePorts, ExitStallAlert, PositionFor, exit_stall_alert,
};
use routing_application::enterprise::{
    AccountId, InstructionId, LifecycleState, NotAccepted, OptIn, SourceId,
};
use routing_application::routes::{
    AccountDirectory, AccountFacts, AccountSizing, BracketCoverage, PortError, RouteFilter,
    RouteStore, RoutingDeps, SourceDirectory, SourceFacts,
};
use routing_infra::{PgDecisionStore, PgRouteStore};
use shared_kernel::{Clock, UserId};
use sqlx::PgPool;
use uuid::Uuid;

use crate::outbox::{EventConsumer, OutboxEvent, RetryPolicy, StartPolicy};

/// TODO(S5): read `accounts.account.command_ttl_ms` once the account exposes it. No operation
/// changes it yet, so every account has the column default (Q16/T2).
const COMMAND_TTL_MS: u32 = 5000;

/// Gateway: the accounts whose bound platform account is on a live session now (route status).
#[async_trait]
pub trait AccountConnections: Send + Sync {
    async fn connected(&self, user: UserId, accounts: &[Uuid]) -> Result<HashSet<Uuid>, String>;
}

/// What the gateway and the caller hand to routing.
pub struct RoutingPorts {
    /// The gateway's expected position (`PositionFor`).
    pub positions: Arc<dyn PositionFor>,
    /// Gate 6, the gateway's sync state; always real.
    pub out_of_sync: Arc<dyn EntryGate>,
    /// Gates 7–8: real, or the test-build stand-ins (ADR-0006 rule 8).
    pub risk_blocked: Arc<dyn EntryGate>,
    pub limits_pending: Arc<dyn EntryGate>,
    pub connections: Arc<dyn AccountConnections>,
}

/// Session-side ports the routes router needs.
pub struct RoutingSession {
    pub authenticator: Arc<dyn SessionAuthenticator>,
    pub step_up: Arc<dyn StepUpGate>,
    pub public_origin: String,
    pub trust_forwarded_for: bool,
}

pub struct RoutingWiring {
    pub router: Router,
    /// Consumer of `ingress.received`; the app registers it (Architect (C)(a): not while a
    /// gate is a stand-in, except in a test build).
    pub decisions: Arc<dyn EventConsumer>,
    /// The entry gates as wired, for the startup warning and release checks (`stand_ins()`).
    pub gates: EntryGatePorts,
}

/// Accounts' `AccountRoutes` port over the route store, built before accounts (which needs it).
pub fn account_routes(db: &PgPool, identity: Arc<dyn IdentityStore>) -> Arc<dyn AccountRoutes> {
    Arc::new(RoutesOfAccount {
        routes: Arc::new(PgRouteStore::new(db.clone())),
        sources: Arc::new(IdentitySources(identity)),
    })
}

/// Builds routing.
#[allow(clippy::too_many_arguments)]
pub fn wire(
    db: &PgPool,
    accounts: Arc<dyn AccountStore>,
    identity: Arc<dyn IdentityStore>,
    brackets: BracketQuery,
    instruments: ListInstruments,
    contracts: ActiveContractQuery,
    ports: RoutingPorts,
    session: RoutingSession,
    clock: Arc<dyn Clock>,
) -> RoutingWiring {
    let routes: Arc<dyn RouteStore> = Arc::new(PgRouteStore::new(db.clone()));
    // The decision never reads the connection state (Brasa M2): a gateway status hiccup must
    // not fail exits. Only the route-status view asks for it.
    let decision_accounts = Arc::new(Accounts {
        store: accounts.clone(),
        connections: None,
    });
    let directory = Arc::new(Accounts {
        store: accounts,
        connections: Some(ports.connections),
    });
    let sources = Arc::new(IdentitySources(identity));
    let profiles = Arc::new(Profiles {
        brackets,
        instruments,
    });
    let gates = EntryGatePorts {
        out_of_sync: ports.out_of_sync,
        risk_blocked: ports.risk_blocked,
        limits_pending: ports.limits_pending,
        no_bracket_profile: profiles.clone(),
    };
    let decide = DecideInstruction::new(DecisionDeps {
        store: Arc::new(PgDecisionStore::new(db.clone())),
        routes: routes.clone(),
        accounts: decision_accounts,
        sources: sources.clone(),
        positions: ports.positions,
        contracts: Arc::new(CatalogContracts(contracts)),
        gates: gates.clone(),
        clock: clock.clone(),
    });
    let consumer_clock = clock.clone();
    let deps = RoutingDeps {
        routes,
        accounts: directory,
        sources,
        // Route status shows gates 6–8 once their owners expose a per-account read.
        gates: None,
        brackets: Some(profiles),
        clock,
    };
    let router = routing_infra::http::router(Arc::new(routing_infra::http::RoutesHttpState::new(
        deps,
        session.authenticator,
        session.step_up,
        session.public_origin,
        session.trust_forwarded_for,
    )));
    RoutingWiring {
        router,
        decisions: Arc::new(DecisionConsumer {
            decide,
            clock: consumer_clock,
            stalls: StallClock::default(),
        }),
        gates,
    }
}

// ---- consumer ---------------------------------------------------------------------------------

/// `ingress.received` → the instruction's routing decision (S7 AC-2…AC-8). Idempotent: a
/// redelivery stores and requests nothing new.
///
/// Retries (Architect, L4): never dead-lettered by count, backoff capped at 30 s. An exit waits
/// for gate 6 as long as it takes (the 72 h terminal comes with HTTP 6.3.0) and alerts the
/// operator at 60 s and at 5 min + every 30 min. An entry ends by its TTL inside the decision;
/// past [`ENTRY_GIVE_UP`] (a port down, a defect) the proven entries are given up as
/// `routing.expired` with `gates_unknown` in the user's audit, never executed late; what is not
/// a proven entry, or cannot be classified, keeps waiting (Vigia M1/M2). Until the give-up is
/// written the operator gets the stalled alert and the delivery is retried. A permanent error
/// (malformed event) is alerted and dropped at once.
struct DecisionConsumer {
    decide: DecideInstruction,
    clock: Arc<dyn Clock>,
    stalls: StallClock,
}

/// Well past any entry TTL (1–60 s): a failing proven entry is given up after this
/// (PROVISIONAL threshold).
const ENTRY_GIVE_UP: chrono::Duration = chrono::Duration::minutes(10);

#[async_trait]
impl EventConsumer for DecisionConsumer {
    fn name(&self) -> &'static str {
        "routing.decide"
    }

    /// Never route history: an instruction received before the consumer existed is not
    /// decided late (it would only expire).
    fn start(&self) -> StartPolicy {
        StartPolicy::FromNow
    }

    /// Each decision reads ports and commits on its own; one event per dispatch transaction.
    fn batch(&self) -> i64 {
        1
    }

    fn retry(&self) -> RetryPolicy {
        RetryPolicy {
            max_attempts: None,
            max_backoff_secs: EXIT_RETRY_CAP.num_seconds(),
        }
    }

    async fn handle(&self, e: &OutboxEvent) -> Result<(), String> {
        let Some(id) = e.payload["refs"]["instruction_id"]
            .as_str()
            .and_then(|s| Uuid::parse_str(s).ok())
        else {
            tracing::error!(event_id = %e.id, "ALERT routing.decide: ingress.received without refs.instruction_id; dropped");
            return Ok(());
        };
        let key = (e.user_id, id);
        let err = match self
            .decide
            .handle(e.user_id, InstructionId::from_uuid(id))
            .await
        {
            // Decided, or nothing to route ever (purged; not a V1 routable action).
            Ok(_) | Err(DecideError::NotFound | DecideError::NotRoutable) => {
                self.stalls.forget(key);
                return Ok(());
            }
            Err(err) => err,
        };
        // `occurred_at` of `ingress.received` is the instruction's receipt.
        let stalled = self.clock.now() - e.occurred_at;
        let mut exits = err.waiting_exits();
        let action = e.payload["details"]["action"].as_str().unwrap_or_default();
        // A close (or a reverse's close leg) whose decision cannot run is an exit waiting too.
        let mut waits = !exits.is_empty() || matches!(action, "close" | "reverse");
        if stalled >= ENTRY_GIVE_UP {
            // Classified from the stored instruction, failing closed toward waiting.
            match self
                .decide
                .give_up_entries(e.user_id, InstructionId::from_uuid(id))
                .await
            {
                Ok(given_up) => {
                    if !given_up.expired.is_empty() {
                        let account_ids: Vec<Uuid> = given_up
                            .expired
                            .iter()
                            .map(|d| d.account_id.as_uuid())
                            .collect();
                        tracing::warn!(
                            instruction_id = %id, account_ids = ?account_ids,
                            stalled_s = stalled.num_seconds(), error = %err,
                            "routing.decide gave up: entry expired with gates unknown, audited, never routed late"
                        );
                    }
                    if !given_up.waiting {
                        self.stalls.forget(key);
                        return Ok(());
                    }
                    exits = given_up.waiting_accounts;
                    waits = true;
                }
                Err(not_given_up) => {
                    let previous = self.stalls.observe(key, stalled);
                    if exit_stall_alert(previous, stalled).is_some() {
                        tracing::error!(
                            instruction_id = %id, stalled_s = stalled.num_seconds(),
                            error = %not_given_up,
                            "ALERT routing.decide stalled: not decided and not given up, retrying, never routed late"
                        );
                    }
                    return Err(not_given_up.to_string());
                }
            }
        }
        if waits {
            let instrument = match &err {
                DecideError::GatesUnavailable { contract, .. } => {
                    format!("{} {}", contract.symbol, contract.month)
                }
                _ => e.payload["details"]["instrument_symbol"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            };
            let account_ids: Vec<Uuid> = exits.iter().map(AccountId::as_uuid).collect();
            let previous = self.stalls.observe(key, stalled);
            let stalled_s = stalled.num_seconds();
            match exit_stall_alert(previous, stalled) {
                Some(ExitStallAlert::Waiting) => tracing::warn!(
                    instruction_id = %id, instrument = %instrument, account_ids = ?account_ids,
                    stalled_s, error = %err, "ALERT exit decision waiting for account sync check"
                ),
                Some(ExitStallAlert::Stalled) => tracing::error!(
                    instruction_id = %id, instrument = %instrument, account_ids = ?account_ids,
                    stalled_s, error = %err, "ALERT exit decision stalled"
                ),
                None => {}
            }
            return Err(err.to_string());
        }
        Err(err.to_string())
    }
}

/// The time since receipt at the previous attempt of each waiting exit, for the edge-triggered
/// alert (Esmeril, WIP #13). Bounded: an entry goes when the instruction is decided or dropped,
/// and past [`StallClock::CAPACITY`] the map is cleared (an alert may then repeat, never miss).
#[derive(Default)]
struct StallClock(std::sync::Mutex<HashMap<(UserId, Uuid), chrono::Duration>>);

impl StallClock {
    const CAPACITY: usize = 10_000;

    /// Records `stalled` and returns the previous value, if this process saw one.
    fn observe(&self, key: (UserId, Uuid), stalled: chrono::Duration) -> Option<chrono::Duration> {
        let Ok(mut map) = self.0.lock() else {
            return None;
        };
        if map.len() >= Self::CAPACITY && !map.contains_key(&key) {
            map.clear();
        }
        map.insert(key, stalled)
    }

    fn forget(&self, key: (UserId, Uuid)) {
        if let Ok(mut map) = self.0.lock() {
            map.remove(&key);
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.0.lock().map_or(0, |m| m.len())
    }
}

// ---- adapters ---------------------------------------------------------------------------------

/// accounts (S5): the caller's live accounts as routing reads them. With `connections`, the
/// route-status view (an error fails the view); without, the decision, which never reads
/// `connection_connected` (left `false`).
struct Accounts {
    store: Arc<dyn AccountStore>,
    connections: Option<Arc<dyn AccountConnections>>,
}

#[async_trait]
impl AccountDirectory for Accounts {
    async fn accounts(&self, user: UserId) -> Result<Vec<AccountFacts>, PortError> {
        let accounts = self
            .store
            .list(user)
            .await
            .map_err(|e| PortError(e.to_string()))?;
        let connected = match &self.connections {
            Some(c) => {
                let ids: Vec<Uuid> = accounts.iter().map(|a| a.id.as_uuid()).collect();
                c.connected(user, &ids).await.map_err(PortError)?
            }
            None => HashSet::new(),
        };
        Ok(accounts
            .iter()
            .map(|a| facts(a, connected.contains(&a.id.as_uuid())))
            .collect())
    }
}

fn facts(a: &Account, connected: bool) -> AccountFacts {
    AccountFacts {
        id: AccountId::from_uuid(a.id.as_uuid()),
        display_name: a.display_name.as_str().to_owned(),
        // Manual-only firms come with S16.
        manual_only: None,
        lifecycle: match a.lifecycle {
            Lifecycle::Active => None,
            Lifecycle::Paused => Some(LifecycleState::Paused),
            Lifecycle::Disabled => Some(LifecycleState::Disabled),
        },
        platform_bound: a.binding.is_some(),
        automation: match a.automation {
            OptInState::NotRequired => OptIn::NotRequired,
            // S5 stores an accepted automation choice only behind a step-up (create, update and
            // setAccountOptIn: AccountCreateOptInAccept / AccountOptInAccept),
            // so every `Accepted` here was confirmed with one.
            OptInState::Chosen(c) if c.choice == Choice::Accepted => {
                OptIn::Accepted { with_step_up: true }
            }
            OptInState::Chosen(_) => OptIn::NotAccepted(NotAccepted::Declined),
            OptInState::PendingReview { .. } => OptIn::NotAccepted(NotAccepted::PendingReview),
        },
        connection_connected: connected && a.binding.is_some(),
        sizing: AccountSizing {
            multiplier: a.multiplier.value(),
            max_position_qty: a.max_position_qty.value(),
            command_ttl_ms: COMMAND_TTL_MS,
        },
    }
}

/// identity: the user's sources. Every source type of V1 (TradingView) is automated.
struct IdentitySources(Arc<dyn IdentityStore>);

#[async_trait]
impl SourceDirectory for IdentitySources {
    async fn source(&self, user: UserId, id: SourceId) -> Result<Option<SourceFacts>, PortError> {
        let source = self
            .0
            .source(
                user,
                identity_enterprise::ids::SourceId::from_uuid(id.as_uuid()),
            )
            .await
            .map_err(|e| PortError(e.to_string()))?;
        Ok(source.map(|s| SourceFacts {
            revoked: s.revoked_at.is_some(),
            automated: match s.source_type {
                identity_enterprise::source::SourceType::TradingView => true,
            },
            name: s.name.as_str().to_owned(),
        }))
    }
}

/// catalog: the rollover calendar (P5).
struct CatalogContracts(ActiveContractQuery);

#[async_trait]
impl ActiveContract for CatalogContracts {
    async fn active(&self, symbol: &str, at: DateTime<Utc>) -> Result<Option<String>, PortError> {
        self.0
            .handle(symbol, at)
            .await
            .map_err(|e| PortError(e.to_string()))
    }
}

/// profiles (S6): gate 9 and the commands' protection, and the route status' coverage.
struct Profiles {
    brackets: BracketQuery,
    instruments: ListInstruments,
}

impl Profiles {
    async fn enabled(&self) -> Result<Vec<(String, Uuid)>, PortError> {
        Ok(self
            .instruments
            .handle()
            .await
            .map_err(|e| PortError(e.to_string()))?
            .into_iter()
            .map(|i| (i.symbol.as_str().to_owned(), i.id))
            .collect())
    }
}

fn accounts_ids(accounts: &[AccountId]) -> Vec<AccountsAccountId> {
    accounts
        .iter()
        .map(|a| AccountsAccountId::from_uuid(a.as_uuid()))
        .collect()
}

#[async_trait]
impl BracketFor for Profiles {
    async fn brackets(
        &self,
        user: UserId,
        contract: &Contract,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, Protection>, PortError> {
        let Some((_, instrument)) = self
            .enabled()
            .await?
            .into_iter()
            .find(|(symbol, _)| *symbol == contract.symbol)
        else {
            // Not an enabled instrument: no account has a usable profile.
            return Ok(HashMap::new());
        };
        let found = self
            .brackets
            .brackets_for(user, instrument, &accounts_ids(accounts))
            .await
            .map_err(|e| PortError(e.to_string()))?;
        Ok(found
            .into_iter()
            .map(|(a, b)| {
                let p = Protection {
                    stop_ticks: b.equivalents.stop.ticks,
                    target_ticks: b.equivalents.target.ticks,
                };
                (AccountId::from_uuid(a.as_uuid()), p)
            })
            .collect())
    }
}

#[async_trait]
impl BracketCoverage for Profiles {
    async fn missing(
        &self,
        user: UserId,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, Vec<Uuid>>, PortError> {
        let instruments: Vec<Uuid> = self
            .enabled()
            .await?
            .into_iter()
            .map(|(_, id)| id)
            .collect();
        let missing = self
            .brackets
            .missing_brackets(user, &accounts_ids(accounts), &instruments)
            .await
            .map_err(|e| PortError(e.to_string()))?;
        Ok(missing
            .into_iter()
            .map(|(a, ids)| (AccountId::from_uuid(a.as_uuid()), ids))
            .collect())
    }
}

/// accounts' view of an account's subscriptions (Esmeril F1; replaces `NoRoutes`).
struct RoutesOfAccount {
    routes: Arc<dyn RouteStore>,
    sources: Arc<IdentitySources>,
}

impl RoutesOfAccount {
    async fn of(
        &self,
        user: UserId,
        id: AccountsAccountId,
    ) -> Result<Vec<routing_application::routes::RouteRecord>, AccountsPortError> {
        self.routes
            .list(
                user,
                RouteFilter {
                    source_id: None,
                    account_id: Some(AccountId::from_uuid(id.as_uuid())),
                },
            )
            .await
            .map_err(|e| AccountsPortError(e.0))
    }
}

#[async_trait]
impl AccountRoutes for RoutesOfAccount {
    async fn routes_of(
        &self,
        user_id: UserId,
        id: AccountsAccountId,
    ) -> Result<Vec<RouteRef>, AccountsPortError> {
        let mut refs = Vec::new();
        for route in self.of(user_id, id).await? {
            let source = self
                .sources
                .source(user_id, route.source_id)
                .await
                .map_err(|e| AccountsPortError(e.0))?;
            if let Some(s) = source {
                refs.push(RouteRef {
                    source_id: route.source_id.as_uuid(),
                    source_name: s.name,
                });
            }
        }
        Ok(refs)
    }

    async fn has_enabled_automated_route(
        &self,
        user_id: UserId,
        id: AccountsAccountId,
    ) -> Result<bool, AccountsPortError> {
        for route in self
            .of(user_id, id)
            .await?
            .into_iter()
            .filter(|r| r.is_enabled)
        {
            let source = self
                .sources
                .source(user_id, route.source_id)
                .await
                .map_err(|e| AccountsPortError(e.0))?;
            if source.is_some_and(|s| s.automated) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_stall_clock_returns_the_previous_value_and_stays_bounded() {
        let clock = StallClock::default();
        let key = (UserId::new_v7(), Uuid::now_v7());
        let s = chrono::Duration::seconds;
        assert_eq!(clock.observe(key, s(2)), None);
        assert_eq!(clock.observe(key, s(32)), Some(s(2)));
        clock.forget(key);
        assert_eq!(clock.len(), 0, "a decided instruction is dropped");
        for _ in 0..StallClock::CAPACITY {
            clock.observe((UserId::new_v7(), Uuid::now_v7()), s(1));
        }
        assert_eq!(clock.len(), StallClock::CAPACITY);
        clock.observe(key, s(1));
        assert_eq!(clock.len(), 1, "past the capacity the map starts over");
    }
}
