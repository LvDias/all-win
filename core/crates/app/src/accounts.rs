//! Wiring of the accounts module (S5) with its stop/target profiles (S6) and the catalog reads
//! they serve (opt-in rules, instruments): dependencies, the catalog adapters, routers and outbox
//! consumers. Ports of modules that are not built yet fail closed.

use std::sync::Arc;

use access::{SessionAuthenticator, StepUpGate};
use accounts_application::enterprise::opt_in::{FirmStance, OptInKind, RuleVersionRef};
use accounts_application::enterprise::profile::TickSpec;
use accounts_application::enterprise::{
    AccountId, ConnectionId, PresetVersionId, SizeAmount, VenueAccountId,
};
use accounts_application::ports::{
    AccountRoutes, AccountStore, BindingView, CurrentRule, PortError, PresetCatalog, PresetFacts,
    RouteRef, RuleSelection, RuleSource, RuleVersions, UserTimeZone, VenueAccountFacts,
    VenueAccounts,
};
use accounts_application::profiles::ports::{EnabledInstrument, ProfileCatalog};
use accounts_application::profiles::{
    BracketQuery, PROFILE_EVENT_TYPES, ProfileDeps, ProfilePlanner, ProfileUseCases,
};
use accounts_application::{AUDIT_EVENT_TYPES, AccountsDeps};
use accounts_infra::profiles::PgProfileStore;
use accounts_infra::store::PgAccountStore;
use async_trait::async_trait;
use axum::Router;
use catalog_application::instrument::{ListInstruments, ListStandardSizes};
use catalog_application::rule_version::RuleVersionQueries;
use catalog_infra::PgInstrumentRepository;
use catalog_infra::rule_version::PgRuleVersionRepository;
use identity_application::ports::{AccountTargets, IdentityStore};
use shared_kernel::{Clock, UserId};
use sqlx::PgPool;

use crate::outbox::{Consumers, EventConsumer};

/// Identity's step-up targets (`AccountTargets`) over the accounts store, built before identity
/// (which needs it) and before accounts (which needs identity): a live account of this user, as
/// `ReadAccounts::exists`. Unknown, archived and other users' accounts are all `false`.
pub struct AccountTargetsAdapter(pub Arc<dyn AccountStore>);

#[async_trait]
impl AccountTargets for AccountTargetsAdapter {
    async fn owned_by(&self, user: UserId, account: uuid::Uuid) -> Result<bool, String> {
        self.0
            .get(user, AccountId::from_uuid(account))
            .await
            .map(|a| a.is_some())
            .map_err(|e| e.to_string())
    }
}

/// The accounts store, shared by the identity adapter above and the accounts wiring.
pub fn store(db: &PgPool) -> Arc<dyn AccountStore> {
    Arc::new(PgAccountStore::new(db.clone()))
}

/// catalog: the opt-in rule versions in force. Presets come with S16, so a preset selection has
/// no current version yet.
struct CatalogRules(RuleVersionQueries);

#[async_trait]
impl RuleVersions for CatalogRules {
    async fn current(
        &self,
        selection: &RuleSelection,
        kind: OptInKind,
    ) -> Result<Option<CurrentRule>, PortError> {
        let preset = match selection {
            RuleSelection::Generic => None,
            RuleSelection::Preset {
                preset_version_id, ..
            } => Some(preset_version_id.as_uuid()),
        };
        let Some(v) = self
            .0
            .current(preset, kind.as_str())
            .await
            .map_err(|e| PortError(e.0))?
        else {
            return Ok(None);
        };
        let stance = FirmStance::parse(&v.firm_stance)
            .ok_or_else(|| PortError(format!("rule version {}: unknown firm stance", v.id)))?;
        let sources = v
            .sources()
            .into_iter()
            .map(|(url, accessed_at)| {
                chrono::NaiveDate::parse_from_str(&accessed_at, "%Y-%m-%d")
                    .map(|accessed_at| RuleSource { url, accessed_at })
                    .map_err(|_| PortError(format!("rule version {}: bad accessed_at", v.id)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Some(CurrentRule {
            rule: RuleVersionRef {
                id: accounts_application::enterprise::RuleVersionId::from_uuid(v.id),
                kind,
                stance,
            },
            sources,
        }))
    }
}

/// catalog (S16) is not built: no preset version exists (`accounts.preset_version_unknown`).
struct NoPresets;

#[async_trait]
impl PresetCatalog for NoPresets {
    async fn version(&self, _: PresetVersionId) -> Result<Option<PresetFacts>, PortError> {
        Ok(None)
    }
}

/// gateway (S2) is not built: no platform account resolves, so nothing can be bound. An open
/// position is never assumed absent.
struct NoVenues;

#[async_trait]
impl VenueAccounts for NoVenues {
    async fn resolve(
        &self,
        _: UserId,
        _: ConnectionId,
        _: &str,
    ) -> Result<Option<VenueAccountFacts>, PortError> {
        Ok(None)
    }

    async fn describe(
        &self,
        _: UserId,
        _: &[VenueAccountId],
    ) -> Result<Vec<BindingView>, PortError> {
        Ok(Vec::new())
    }

    async fn has_open_position(&self, _: UserId, _: VenueAccountId) -> Result<bool, PortError> {
        Err(PortError("gateway unavailable".into()))
    }
}

/// routing (S7) subscriptions are not wired yet: no account has routes.
struct NoRoutes;

#[async_trait]
impl AccountRoutes for NoRoutes {
    async fn routes_of(&self, _: UserId, _: AccountId) -> Result<Vec<RouteRef>, PortError> {
        Ok(Vec::new())
    }

    async fn has_enabled_automated_route(
        &self,
        _: UserId,
        _: AccountId,
    ) -> Result<bool, PortError> {
        Ok(false)
    }
}

/// identity: the user's time zone (IANA name from the profile).
struct IdentityTimeZones(Arc<dyn IdentityStore>);

#[async_trait]
impl UserTimeZone for IdentityTimeZones {
    async fn time_zone(&self, user_id: UserId) -> Result<chrono_tz::Tz, PortError> {
        let user = self
            .0
            .user(user_id)
            .await
            .map_err(|e| PortError(e.to_string()))?
            .ok_or_else(|| PortError("unknown user".into()))?;
        user.time_zone
            .as_str()
            .parse()
            .map_err(|_| PortError("stored time zone is not IANA".into()))
    }
}

/// catalog for the profiles: enabled instruments with exact tick data, and the standard sizes.
/// An instrument whose tick data the profiles refuse (e.g. a tick value with more than 4 decimal
/// places) is left out with an error log: profiles and brackets never see it, so routing answers
/// `routing.no_bracket_profile` for it, while every other instrument keeps working.
struct CatalogProfiles {
    instruments: ListInstruments,
    sizes: ListStandardSizes,
}

#[async_trait]
impl ProfileCatalog for CatalogProfiles {
    async fn enabled_instruments(&self) -> Result<Vec<EnabledInstrument>, PortError> {
        let all = self
            .instruments
            .handle()
            .await
            .map_err(|e| PortError(e.0))?;
        Ok(all
            .into_iter()
            .filter_map(|i| match TickSpec::new(i.tick_size, i.point_value) {
                Ok(ticks) => Some(EnabledInstrument { id: i.id, ticks }),
                Err(e) => {
                    tracing::error!(instrument_id = %i.id, error = %e, "instrument left out of profiles: tick data refused");
                    None
                }
            })
            .collect())
    }

    async fn standard_sizes(&self) -> Result<Vec<SizeAmount>, PortError> {
        self.sizes
            .handle()
            .await
            .map_err(|e| PortError(e.0))?
            .into_iter()
            .map(|d| SizeAmount::new(d).map_err(|e| PortError(format!("standard size {d}: {e}"))))
            .collect()
    }
}

/// What `app` keeps from accounts after wiring.
pub struct AccountsWiring {
    /// accounts, profiles, opt-in rules and instruments.
    pub router: Router,
    /// For routing's bracket coverage (S7), handed over when routing is wired.
    pub brackets: BracketQuery,
}

/// Session-side ports from identity that every module router needs.
pub struct SessionPorts {
    pub authenticator: Arc<dyn SessionAuthenticator>,
    pub step_up: Arc<dyn StepUpGate>,
    pub identity_store: Arc<dyn IdentityStore>,
}

/// Builds accounts, profiles and the catalog reads, and registers their audited event types.
#[allow(clippy::too_many_arguments)]
pub fn wire(
    db: &PgPool,
    store: Arc<dyn AccountStore>,
    clock: Arc<dyn Clock>,
    session: SessionPorts,
    public_origin: String,
    trust_forwarded_for: bool,
    production: bool,
    audit: Arc<dyn EventConsumer>,
    consumers: &mut Consumers,
) -> AccountsWiring {
    let reader = Arc::new(PgInstrumentRepository::new(db.clone()));
    let rules = RuleVersionQueries::new(Arc::new(PgRuleVersionRepository::new(db.clone())));
    let catalog: Arc<dyn ProfileCatalog> = Arc::new(CatalogProfiles {
        instruments: ListInstruments::new(reader.clone()),
        sizes: ListStandardSizes::new(reader.clone()),
    });
    let profile_store = Arc::new(PgProfileStore::new(db.clone()));
    let deps = AccountsDeps {
        store,
        rules: Arc::new(CatalogRules(rules.clone())),
        presets: Arc::new(NoPresets),
        venues: Arc::new(NoVenues),
        routes: Arc::new(NoRoutes),
        time_zones: Arc::new(IdentityTimeZones(session.identity_store)),
        profiles: Arc::new(ProfilePlanner::new(profile_store.clone(), catalog.clone())),
        step_up: session.step_up,
        clock: clock.clone(),
        production,
    };
    let profiles = ProfileUseCases::new(ProfileDeps {
        store: profile_store.clone(),
        catalog: catalog.clone(),
        clock,
    });
    for t in AUDIT_EVENT_TYPES.iter().chain(PROFILE_EVENT_TYPES.iter()) {
        consumers.register(t, audit.clone());
    }
    let auth = session.authenticator;
    let router = Router::new()
        .merge(accounts_infra::http::router(Arc::new(
            accounts_infra::http::AccountsHttpState::new(
                deps,
                auth.clone(),
                public_origin.clone(),
                trust_forwarded_for,
            ),
        )))
        .merge(accounts_infra::profiles::http::router(Arc::new(
            accounts_infra::profiles::http::ProfilesHttpState::new(
                profiles,
                auth.clone(),
                public_origin.clone(),
            ),
        )))
        .merge(catalog_infra::rule_version_http::router(Arc::new(
            catalog_infra::rule_version_http::RuleVersionHttpState::new(
                rules,
                auth.clone(),
                public_origin.clone(),
            ),
        )))
        .merge(catalog_infra::instrument_http::router(Arc::new(
            catalog_infra::instrument_http::InstrumentsHttpState::new(
                ListInstruments::new(reader),
                auth,
                public_origin,
            ),
        )));
    AccountsWiring {
        router,
        brackets: BracketQuery::new(profile_store, catalog),
    }
}
