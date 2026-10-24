//! `InstrumentReader` on `catalog.instrument` (global, no RLS).

use async_trait::async_trait;
use catalog_application::CatalogError;
use catalog_application::enterprise::Symbol;
use catalog_application::enterprise::instrument::Instrument;
use catalog_application::instrument::{ContractReader, InstrumentReader};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::PgInstrumentRepository;

struct InstrumentRow {
    id: Uuid,
    symbol: String,
    name: String,
    exchange: String,
    currency: String,
    tick_size: Decimal,
    point_value: Decimal,
}

impl TryFrom<InstrumentRow> for Instrument {
    type Error = CatalogError;

    fn try_from(row: InstrumentRow) -> Result<Self, CatalogError> {
        Ok(Self {
            id: row.id,
            symbol: Symbol::parse(&row.symbol).map_err(|e| CatalogError(e.to_string()))?,
            name: row.name,
            exchange: row.exchange,
            currency: row.currency,
            tick_size: row.tick_size,
            point_value: row.point_value,
        })
    }
}

#[async_trait]
impl InstrumentReader for PgInstrumentRepository {
    async fn list_enabled(&self) -> Result<Vec<Instrument>, CatalogError> {
        sqlx::query_as!(
            InstrumentRow,
            r#"SELECT id, symbol, name, exchange, currency AS "currency!", tick_size, point_value
               FROM catalog.instrument WHERE is_enabled ORDER BY symbol"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| CatalogError(e.to_string()))?
        .into_iter()
        .map(Instrument::try_from)
        .collect()
    }

    async fn find_enabled(&self, id: Uuid) -> Result<Option<Instrument>, CatalogError> {
        sqlx::query_as!(
            InstrumentRow,
            r#"SELECT id, symbol, name, exchange, currency AS "currency!", tick_size, point_value
               FROM catalog.instrument WHERE id = $1 AND is_enabled"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CatalogError(e.to_string()))?
        .map(Instrument::try_from)
        .transpose()
    }

    async fn standard_account_sizes(&self) -> Result<Vec<Decimal>, CatalogError> {
        sqlx::query_scalar!(
            "SELECT size_amount FROM catalog.standard_account_size ORDER BY size_amount"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| CatalogError(e.to_string()))
    }
}

#[async_trait]
impl ContractReader for PgInstrumentRepository {
    async fn active_contract(
        &self,
        symbol: &str,
        at: DateTime<Utc>,
    ) -> Result<Option<String>, CatalogError> {
        sqlx::query_scalar!(
            r#"SELECT to_char(c.contract_month, 'YYYY-MM') AS "month!"
                 FROM catalog.instrument_contract c
                 JOIN catalog.instrument i ON i.id = c.instrument_id
                WHERE i.symbol = $1 AND i.is_enabled AND c.active_from <= $2
                ORDER BY c.active_from DESC
                LIMIT 1"#,
            symbol,
            at
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CatalogError(e.to_string()))
    }
}
