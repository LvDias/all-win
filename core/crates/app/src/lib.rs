//! Composition root: configuration, telemetry, HTTP server and wiring of module ports.

pub mod config;
pub mod dev;
pub mod http;
pub mod outbox;
pub mod telemetry;
pub mod wiring;

use std::sync::Arc;

use adapter_tradingview::{IngressMetrics, IngressState, KeyedRateLimiter, RateLimit};
use anyhow::Context;
use audit_application::RecordAuditEntry;
use audit_infra::PgAuditStore;
use catalog_application::IsInstrumentEnabled;
use catalog_infra::PgInstrumentRepository;
use routing_application::{IngestInstruction, IngestStore, RecordIngressRejection};
use routing_infra::PgIngestStore;
use shared_kernel::{Clock, SystemClock};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Notify;

/// Migrations embedded in the binary; run as `allwin_owner` only.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

/// Shared state handed to HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub ingress: Arc<IngressState>,
}

/// The wired application plus its background workers.
pub struct App {
    pub state: AppState,
    pub dispatcher: outbox::OutboxDispatcher,
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
        Ok(Self::wire(config, db, worker, Arc::new(SystemClock)))
    }

    pub fn wire(
        config: &config::Config,
        db: PgPool,
        worker: PgPool,
        clock: Arc<dyn Clock>,
    ) -> Self {
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
            ip_limiter: KeyedRateLimiter::new(RateLimit {
                per_minute: config.rate_per_ip_per_min,
            }),
            key_limiter: KeyedRateLimiter::new(RateLimit {
                per_minute: config.rate_per_key_per_min,
            }),
            metrics: Arc::new(IngressMetrics::default()),
            clock,
            trust_forwarded_for: config.trust_forwarded_for,
        });
        let audit = RecordAuditEntry::new(Arc::new(PgAuditStore::new(db.clone())));
        Self {
            state: AppState { db, ingress },
            dispatcher: outbox::OutboxDispatcher::new(worker, audit, wake),
        }
    }
}
