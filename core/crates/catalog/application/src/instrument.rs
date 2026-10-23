//! `listInstruments` and the enabled-instrument lookup that other modules reach through their own
//! ports, wired in `app` (ADR-0006 §3.7).

use std::sync::Arc;

use async_trait::async_trait;
use catalog_enterprise::instrument::Instrument;
use uuid::Uuid;

use crate::CatalogError;

#[async_trait]
pub trait InstrumentReader: Send + Sync {
    /// Enabled instruments, sorted by symbol.
    async fn list_enabled(&self) -> Result<Vec<Instrument>, CatalogError>;
    /// `None` when the id is unknown or the instrument is not enabled.
    async fn find_enabled(&self, id: Uuid) -> Result<Option<Instrument>, CatalogError>;
}

/// `listInstruments`: the global catalog (no user data).
#[derive(Clone)]
pub struct ListInstruments {
    reader: Arc<dyn InstrumentReader>,
}

impl ListInstruments {
    pub fn new(reader: Arc<dyn InstrumentReader>) -> Self {
        Self { reader }
    }

    pub async fn handle(&self) -> Result<Vec<Instrument>, CatalogError> {
        self.reader.list_enabled().await
    }
}

/// Query port: an enabled instrument's tick data by id.
#[derive(Clone)]
pub struct FindEnabledInstrument {
    reader: Arc<dyn InstrumentReader>,
}

impl FindEnabledInstrument {
    pub fn new(reader: Arc<dyn InstrumentReader>) -> Self {
        Self { reader }
    }

    pub async fn handle(&self, id: Uuid) -> Result<Option<Instrument>, CatalogError> {
        self.reader.find_enabled(id).await
    }
}
