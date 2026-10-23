//! HTTP handler of `listInstruments` (contract `modules/profiles.yaml`, S6). Global catalog data:
//! any signed-in user reads it; nothing is user-scoped. Values are exact decimal strings; the UI
//! does no arithmetic with them.
//!
//! `instrument_schemas.json` holds the contract component (refs resolved) for the OpenAPI
//! document, inserted by `app::openapi` like the other modules' ones.

use std::borrow::Cow;
use std::sync::{Arc, LazyLock};

use access::SessionAuthenticator;
use access::http::{AccessState, Authenticated};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use rust_decimal::Decimal;
use serde_json::{Map, Value, json};
use utoipa::openapi::RefOr;
use utoipa::openapi::schema::{Object, Schema};
use utoipa::{PartialSchema, ToSchema};

use catalog_application::enterprise::instrument::Instrument;
use catalog_application::instrument::ListInstruments;

const SCHEMAS_JSON: &str = include_str!("instrument_schemas.json");

static SCHEMAS: LazyLock<Map<String, Value>> = LazyLock::new(|| {
    serde_json::from_str::<Value>(SCHEMAS_JSON)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
});

/// Placeholder for the contract component `InstrumentList`.
pub struct InstrumentListSchema;

impl PartialSchema for InstrumentListSchema {
    fn schema() -> RefOr<Schema> {
        Schema::Object(Object::new()).into()
    }
}

impl ToSchema for InstrumentListSchema {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("InstrumentList")
    }
}

/// A contract component by name (for tests).
pub fn component(name: &str) -> Option<&'static Value> {
    SCHEMAS.get(name)
}

/// Component name → contract schema, for `app::openapi`.
pub fn raw_components() -> Vec<(&'static str, Value)> {
    SCHEMAS
        .get("InstrumentList")
        .map(|v| vec![("InstrumentList", v.clone())])
        .unwrap_or_default()
}

pub struct InstrumentsHttpState {
    list: ListInstruments,
    authenticator: Arc<dyn SessionAuthenticator>,
    public_origin: String,
}

impl InstrumentsHttpState {
    pub fn new(
        list: ListInstruments,
        authenticator: Arc<dyn SessionAuthenticator>,
        public_origin: String,
    ) -> Self {
        Self {
            list,
            authenticator,
            public_origin,
        }
    }
}

impl AccessState for InstrumentsHttpState {
    fn authenticator(&self) -> &Arc<dyn SessionAuthenticator> {
        &self.authenticator
    }

    fn public_origin(&self) -> &str {
        &self.public_origin
    }
}

pub fn router(state: Arc<InstrumentsHttpState>) -> Router {
    Router::new()
        .route("/api/v1/instruments", get(list_instruments))
        .with_state(state)
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(list_instruments))]
pub struct InstrumentsApi;

/// Money with at least 2 decimals, never cut (`MoneyString`).
fn money(value: Decimal) -> String {
    let mut v = value.normalize();
    v.rescale(v.scale().max(2));
    v.to_string()
}

fn instrument_json(i: &Instrument) -> Option<Value> {
    Some(json!({
        "id": i.id,
        "symbol": i.symbol.as_str(),
        "name": i.name,
        "exchange": i.exchange,
        "currency": i.currency,
        "tick_size": i.tick_size.normalize().to_string(),
        "tick_value": money(i.tick_value()?),
        "point_value": money(i.point_value),
        "price_decimals": i.price_decimals(),
    }))
}

fn internal(detail: &str) -> Response {
    tracing::error!(error = %detail, "instruments: internal error");
    let e = contracts::http::Error::new("common.internal");
    (StatusCode::INTERNAL_SERVER_ERROR, Json(e)).into_response()
}

#[utoipa::path(
    get,
    path = "/api/v1/instruments",
    operation_id = "listInstruments",
    tag = "profiles",
    responses(
        (status = 200, description = "The instruments, sorted by symbol.", body = InstrumentListSchema),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn list_instruments(
    State(s): State<Arc<InstrumentsHttpState>>,
    _caller: Authenticated,
) -> Response {
    let instruments = match s.list.handle().await {
        Ok(i) => i,
        Err(e) => return internal(&e.0),
    };
    match instruments
        .iter()
        .map(instrument_json)
        .collect::<Option<Vec<_>>>()
    {
        Some(list) => Json(json!({"instruments": list})).into_response(),
        None => internal("tick value overflow"),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use catalog_application::enterprise::Symbol;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn mnq_conforms_to_the_contract() {
        // As stored: NUMERIC(22,10) / NUMERIC(18,4).
        let mnq = Instrument {
            id: Uuid::parse_str("01a166e7-a8d0-700f-bade-6d8493132056").unwrap(),
            symbol: Symbol::parse("MNQ").unwrap(),
            name: "Micro E-mini Nasdaq-100".into(),
            exchange: "CME".into(),
            currency: "USD".into(),
            tick_size: Decimal::from_str("0.2500000000").unwrap(),
            point_value: Decimal::from_str("2.0000").unwrap(),
        };
        let list = json!({"instruments": [instrument_json(&mnq).unwrap()]});
        let validator = jsonschema::validator_for(component("InstrumentList").unwrap()).unwrap();
        assert!(validator.is_valid(&list), "{list}");
        let i = &list["instruments"][0];
        assert_eq!(
            (
                &i["tick_size"],
                &i["tick_value"],
                &i["point_value"],
                &i["price_decimals"]
            ),
            (&json!("0.25"), &json!("0.50"), &json!("2.00"), &json!(2))
        );
    }
}
