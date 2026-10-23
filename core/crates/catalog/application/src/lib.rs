//! `catalog` module — application layer: instrument support queries.
//! Supporting an instrument is data (`is_enabled`), not code (S1 AC-7).

use std::sync::Arc;

use async_trait::async_trait;
pub use catalog_enterprise as enterprise;

pub mod instrument;
pub mod release;
pub mod rule_version;
use catalog_enterprise::Symbol;

#[derive(Debug, thiserror::Error)]
#[error("instrument catalog: {0}")]
pub struct CatalogError(pub String);

#[async_trait]
pub trait InstrumentRepository: Send + Sync {
    /// `None` when the symbol is not in the catalog.
    async fn is_enabled(&self, symbol: &Symbol) -> Result<Option<bool>, CatalogError>;
}

/// Query port: is this symbol enabled for trading?
#[derive(Clone)]
pub struct IsInstrumentEnabled {
    repo: Arc<dyn InstrumentRepository>,
}

impl IsInstrumentEnabled {
    pub fn new(repo: Arc<dyn InstrumentRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle(&self, symbol: &str) -> Result<bool, CatalogError> {
        match Symbol::parse(symbol) {
            Ok(symbol) => Ok(self.repo.is_enabled(&symbol).await?.unwrap_or(false)),
            Err(_) => Ok(false),
        }
    }
}
