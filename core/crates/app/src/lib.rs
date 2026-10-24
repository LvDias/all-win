//! Composition root: configuration, telemetry, HTTP server and wiring of module ports.

pub mod accounts;
pub mod config;
pub mod dev;
pub mod gateway;
pub mod http;
pub mod identity;
pub mod openapi;
pub mod outbox;
pub mod purge;
pub mod routing;
pub mod routing_ports;
pub mod telemetry;
pub mod unconsumed;
pub mod wiring;

use std::sync::Arc;

use adapter_tradingview::{
    IngressMetrics, IngressState, IpLimits, KeyedRateLimiter, RateLimit, UnknownKeyCache,
};
use anyhow::Context;
use audit_application::RecordAuditEntry;
use audit_infra::PgAuditStore;
use catalog_application::IsInstrumentEnabled;
use catalog_application::instrument::{ActiveContractQuery, ListInstruments};
use catalog_infra::PgInstrumentRepository;
use identity_application::usecases::connections::RecordConnectionLiveState;
use routing_application::enterprise::IngressEvent;
use routing_application::{IngestInstruction, IngestStore, RecordIngressRejection};
use routing_infra::PgIngestStore;
use shared_kernel::{Clock, SystemClock};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Notify;

/// Bounded size of the unknown-`key_id` cache (s1-webhook-ingest §2 step 2).
const UNKNOWN_KEY_CACHE_CAPACITY: usize = 10_000;

/// Migrations embedded in the binary; run as `allwin_owner` only.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

/// Shared state handed to HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    /// `allwin_worker` (BYPASSRLS): cross-user operational counts only (`/metrics`).
    pub worker: PgPool,
    /// Outbox event types with a registered consumer (for the unconsumed-events metric).
    pub consumed_event_types: Arc<[String]>,
    pub ingress: Arc<IngressState>,
    /// The routes behind the session authenticator: identity (`/api/v1/auth`, `/me`, `/admin`,
    /// `/sources`, `/connections`, `/adapter/v1/pairings`, `/events`), accounts and profiles,
    /// opt-in rules and instruments; `None` without `IDENTITY_PEPPERS`.
    pub sessions: Option<axum::Router>,
    /// The adapters' WebSocket (`GET /adapter/v1`, a device token, not a session); `None`
    /// without `IDENTITY_PEPPERS`.
    pub adapter: Option<axum::Router>,
    /// The gateway's counters for `/metrics`; `None` without `IDENTITY_PEPPERS`.
    pub gateway_metrics: Option<Arc<gateway::GatewayMetrics>>,
}

/// The wired application plus its background workers.
pub struct App {
    pub state: AppState,
    pub dispatcher: outbox::OutboxDispatcher,
    /// Identity startup work and background tasks; run [`identity::IdentityWiring::start`]
    /// before accepting connections.
    pub identity: Option<identity::IdentityWiring>,
}

/// Connects a pool without echoing connection details (they hold credentials).
pub async fn connect(url: &config::SecretString, max: u32) -> anyhow::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(max)
        .connect(url.expose())
        .await
        .map_err(|_| anyhow::anyhow!("database connection failed"))
}

impl App {
    pub async fn connect(config: &config::Config) -> anyhow::Result<Self> {
        let db = connect(&config.database_url, config.db_max_connections)
            .await
            .context("DATABASE_URL")?;
        let worker = connect(&config.worker_database_url, 2)
            .await
            .context("WORKER_DATABASE_URL")?;
        Self::wire(config, db, worker, Arc::new(SystemClock))
    }

    /// Wires every module. Fails when an emitted outbox event type has no consumer.
    pub fn wire(
        config: &config::Config,
        db: PgPool,
        worker: PgPool,
        clock: Arc<dyn Clock>,
    ) -> anyhow::Result<Self> {
        let wake = Arc::new(Notify::new());
        let store: Arc<dyn IngestStore> = Arc::new(wiring::NotifyingIngestStore {
            inner: Arc::new(PgIngestStore::new(db.clone())),
            wake: wake.clone(),
        });
        let ingress = Arc::new(IngressState {
            resolver: Arc::new(wiring::PgSourceKeyResolver::new(db.clone())),
            instruments: Arc::new(wiring::CatalogInstrumentSupport(IsInstrumentEnabled::new(
                Arc::new(PgInstrumentRepository::new(db.clone())),
            ))),
            ingest: IngestInstruction::new(store.clone()),
            reject: RecordIngressRejection::new(store),
            ip_limits: IpLimits {
                flood: KeyedRateLimiter::new(RateLimit {
                    per_minute: config.ip_flood_per_min,
                }),
                tradingview_flood: KeyedRateLimiter::new(RateLimit {
                    per_minute: config.tradingview_flood_per_min,
                }),
                tradingview_ranges: config.tradingview_source_ranges.clone(),
                token_failures: KeyedRateLimiter::new(RateLimit {
                    per_minute: config.ip_token_failures_per_min,
                }),
            },
            key_limiter: KeyedRateLimiter::new(RateLimit {
                per_minute: config.rate_per_key_per_min,
            }),
            unknown_keys: UnknownKeyCache::new(
                std::time::Duration::from_secs(config.unknown_key_cache_ttl_s),
                UNKNOWN_KEY_CACHE_CAPACITY,
            ),
            metrics: Arc::new(IngressMetrics::default()),
            clock: clock.clone(),
            trust_forwarded_for: config.trust_forwarded_for,
        });
        // Consumers are registered before the dispatcher exists (architecture rule).
        let audit: Arc<dyn outbox::EventConsumer> = Arc::new(outbox::AuditConsumer(
            RecordAuditEntry::new(Arc::new(PgAuditStore::new(db.clone()))),
        ));
        let mut consumers = outbox::Consumers::default();
        for event_type in IngressEvent::TYPES {
            consumers.register(event_type, audit.clone());
        }
        consumers.require(&IngressEvent::TYPES, unconsumed::UNCONSUMED)?;
        let account_store = accounts::store(&db);
        let mut adapter = None;
        let mut gateway_metrics = None;
        let (identity, sessions) = match &config.identity {
            Some(cfg) => {
                // accounts and the gateway read each other (ADR-0008): the gateway's reads of
                // accounts are set once accounts is wired. Identity's connections view reads
                // the gateway's platform accounts through the same late-set reads.
                let gateway_store = gateway::store(&db);
                let accounts_reads: gateway::AccountsReads = Arc::default();
                let wired = identity::wire(
                    config,
                    cfg,
                    &db,
                    &worker,
                    clock.clone(),
                    ingress.clone(),
                    Arc::new(accounts::AccountTargetsAdapter(account_store.clone())),
                    Arc::new(gateway::GatewayPresence {
                        store: gateway_store.clone(),
                        accounts: accounts_reads.clone(),
                    }),
                    audit.clone(),
                    &mut consumers,
                )?;
                let venues = Arc::new(gateway::GatewayVenues {
                    store: gateway_store.clone(),
                    identity: wired.deps.store.clone(),
                    accounts: accounts_reads.clone(),
                });
                let modules = accounts::wire(
                    &db,
                    account_store.clone(),
                    clock.clone(),
                    accounts::SessionPorts {
                        authenticator: wired.authenticator.clone(),
                        step_up: wired.step_up.clone(),
                        identity_store: wired.deps.store.clone(),
                    },
                    venues,
                    routing::account_routes(&db, wired.deps.store.clone()),
                    wired.public_origin.clone(),
                    config.trust_forwarded_for,
                    config.app_env.is_production(),
                    audit.clone(),
                    &mut consumers,
                );
                if accounts_reads.set(modules.reads).is_err() {
                    anyhow::bail!("accounts reads set twice");
                }
                let gateway = gateway::wire(
                    gateway::GatewayDeps {
                        store: gateway_store.clone(),
                        tokens: Arc::new(identity::IdentityDeviceTokens(wired.deps.clone())),
                        accounts: accounts_reads.clone(),
                        presence: RecordConnectionLiveState::new(wired.deps.clone()),
                        clock: clock.clone(),
                        app_env: Some(routing_ports::app_env_str(config.app_env).to_owned()),
                        token_failures_per_min: config.ip_token_failures_per_min,
                        trust_forwarded_for: config.trust_forwarded_for,
                    },
                    audit.clone(),
                    &mut consumers,
                )
                .map_err(|e| anyhow::anyhow!("gateway: {e}"))?;
                let gates =
                    routing_ports::risk_gates(config.app_env, Arc::new(gateway.reads.clone()));
                let instruments = Arc::new(PgInstrumentRepository::new(db.clone()));
                let routing = routing::wire(
                    &db,
                    account_store,
                    wired.deps.store.clone(),
                    modules.brackets,
                    ListInstruments::new(instruments.clone()),
                    ActiveContractQuery::new(instruments),
                    routing::RoutingPorts {
                        positions: Arc::new(gateway.reads.clone()),
                        out_of_sync: Arc::new(gateway.reads.clone()),
                        risk_blocked: gates.risk_blocked,
                        limits_pending: gates.limits_pending,
                        connections: Arc::new(routing_ports::GatewayConnections {
                            accounts: accounts_reads,
                            store: gateway_store,
                            identity: wired.deps.store.clone(),
                        }),
                    },
                    routing::RoutingSession {
                        authenticator: wired.authenticator.clone(),
                        step_up: wired.step_up.clone(),
                        public_origin: wired.public_origin.clone(),
                        trust_forwarded_for: config.trust_forwarded_for,
                    },
                    clock,
                );
                routing_ports::check_stand_ins(&routing.gates, config.app_env)
                    .map_err(|e| anyhow::anyhow!(e))?;
                let stand_ins = routing.gates.stand_ins();
                if !stand_ins.is_empty() {
                    tracing::warn!(
                        gates = ?stand_ins,
                        "routing gates 7–8: stand-in (no risk limits) — dev-stubs"
                    );
                }
                // The router and its decision consumer go together (one unit).
                consumers.register("ingress.received", routing.decisions);
                for code in routing_ports::ROUTING_AUDIT_CODES {
                    consumers.register(code, audit.clone());
                }
                let emitted: Vec<&str> = accounts_application::EMITTED_EVENT_TYPES
                    .iter()
                    .chain(accounts_application::profiles::PROFILE_EVENT_TYPES.iter())
                    .chain(gateway_application::events::EMITTED.iter())
                    .chain(gateway_application::events::AUDIT_CODES.iter())
                    .chain(gateway_application::events::NOTIFICATIONS.iter())
                    .copied()
                    .chain(routing_ports::emitted())
                    .chain([identity_application::changes::DEVICE_TOKEN_REVOKED_EVENT_TYPE])
                    .collect();
                consumers.require(&emitted, unconsumed::UNCONSUMED)?;
                let sessions = wired
                    .router
                    .clone()
                    .merge(modules.router)
                    .merge(routing.router);
                adapter = Some(gateway.router);
                gateway_metrics = Some(gateway.metrics);
                (Some(wired), Some(sessions))
            }
            None => {
                tracing::warn!(
                    "IDENTITY_PEPPERS is not set: the identity, accounts and profile routes are not mounted"
                );
                (None, None)
            }
        };
        unconsumed::check(&consumers, unconsumed::UNCONSUMED)?;
        Ok(Self {
            state: AppState {
                db,
                worker: worker.clone(),
                consumed_event_types: consumers.types().into(),
                ingress,
                sessions,
                adapter,
                gateway_metrics,
            },
            dispatcher: outbox::OutboxDispatcher::new(worker, consumers, wake),
            identity,
        })
    }
}
