//! `catalog` module — infra layer: `catalog.instrument` (global, no RLS).

pub mod instrument;
pub mod instrument_http;
pub mod release_file;
pub mod release_store;
pub mod rule_version;
pub mod rule_version_http;

use async_trait::async_trait;
use catalog_application::enterprise::Symbol;
use catalog_application::{CatalogError, InstrumentRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgInstrumentRepository {
    pool: PgPool,
}

impl PgInstrumentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InstrumentRepository for PgInstrumentRepository {
    async fn is_enabled(&self, symbol: &Symbol) -> Result<Option<bool>, CatalogError> {
        sqlx::query_scalar!(
            "SELECT is_enabled FROM catalog.instrument WHERE symbol = $1",
            symbol.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CatalogError(e.to_string()))
    }
}
