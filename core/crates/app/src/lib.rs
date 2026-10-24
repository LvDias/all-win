//! Composition root: configuration, telemetry, HTTP server and wiring of module ports.

pub mod config;
pub mod dev;
pub mod http;
pub mod identity;
pub mod openapi;
pub mod outbox;
pub mod purge;
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
use catalog_infra::PgInstrumentRepository;
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
    /// The identity routes (`/api/v1/auth`, `/me`, `/admin`, `/sources`, `/connections`,
    /// `/adapter/v1/pairings`, `/events`); `None` without `IDENTITY_PEPPERS`.
    pub identity: Option<axum::Router>,
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
        let identity = match &config.identity {
            Some(cfg) => Some(identity::wire(
                config,
                cfg,
                &db,
                &worker,
                clock,
                ingress.clone(),
                audit,
                &mut consumers,
            )?),
            None => {
                tracing::warn!("IDENTITY_PEPPERS is not set: the identity routes are not mounted");
                None
            }
        };
        unconsumed::check(&consumers, unconsumed::UNCONSUMED)?;
        Ok(Self {
            state: AppState {
                db,
                worker: worker.clone(),
                consumed_event_types: consumers.types().into(),
                ingress,
                identity: identity.as_ref().map(|w| w.router.clone()),
            },
            dispatcher: outbox::OutboxDispatcher::new(worker, consumers, wake),
            identity,
        })
    }
}
