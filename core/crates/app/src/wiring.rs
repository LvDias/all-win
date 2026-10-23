//! Implementations of consumer-owned ports that cross module boundaries.

use std::sync::Arc;

use adapter_tradingview::{InstrumentSupport, PortError, ResolvedSourceKey, SourceKeyResolver};
use async_trait::async_trait;
use catalog_application::IsInstrumentEnabled;
use routing_application::enterprise::{IngressEvent, SourceId};
use routing_application::{IngestOutcome, IngestStore, NewInstruction, StoreError};
use shared_kernel::UserId;
use sqlx::PgPool;
use tokio::sync::Notify;

/// Source-key lookup through `identity.resolve_source_key` (SECURITY DEFINER, wave1-data-model §3).
/// Moves behind an `identity-application` query once S3 exposes it.
pub struct PgSourceKeyResolver {
    pool: PgPool,
}

impl PgSourceKeyResolver {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SourceKeyResolver for PgSourceKeyResolver {
    async fn resolve(&self, key_id: &str) -> Result<Option<ResolvedSourceKey>, PortError> {
        let row = sqlx::query!(
            r#"SELECT user_id AS "user_id!", source_id AS "source_id!", source_type AS "source_type!",
                      secret_hash AS "secret_hash!", revoked AS "revoked!"
                 FROM identity.resolve_source_key($1)"#,
            key_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PortError(e.to_string()))?;
        Ok(row.map(|r| ResolvedSourceKey {
            user_id: UserId::from_uuid(r.user_id),
            source_id: SourceId::from_uuid(r.source_id),
            source_type: r.source_type,
            secret_hash: r.secret_hash,
            revoked: r.revoked,
        }))
    }
}

pub struct CatalogInstrumentSupport(pub IsInstrumentEnabled);

#[async_trait]
impl InstrumentSupport for CatalogInstrumentSupport {
    async fn is_enabled(&self, symbol: &str) -> Result<bool, PortError> {
        self.0
            .handle(symbol)
            .await
            .map_err(|e| PortError(e.to_string()))
    }
}

/// Wakes the outbox dispatcher after each committed write, so audit entries appear
/// immediately instead of on the next poll.
pub struct NotifyingIngestStore {
    pub inner: Arc<dyn IngestStore>,
    pub wake: Arc<Notify>,
}

#[async_trait]
impl IngestStore for NotifyingIngestStore {
    async fn ingest(&self, instruction: &NewInstruction) -> Result<IngestOutcome, StoreError> {
        let outcome = self.inner.ingest(instruction).await?;
        self.wake.notify_one();
        Ok(outcome)
    }

    async fn record_rejection(&self, event: &IngressEvent) -> Result<(), StoreError> {
        self.inner.record_rejection(event).await?;
        self.wake.notify_one();
        Ok(())
    }
}
