//! Contract schemas of the route operations (`contracts/http/modules/routing.yaml`, S7).
//!
//! `schemas.json` holds the contract components with their `$ref`s resolved, extracted from
//! `contracts/http/openapi.bundled.yaml` (HTTP contract 5.3.0). They carry `if`/`then`/`not` rules
//! that `utoipa` cannot model, so:
//! - requests are validated against them before they are deserialized (400 with the pointer);
//! - the OpenAPI document gets them verbatim: the handler annotations name the placeholder types
//!   below, and `app::openapi` replaces each placeholder with [`raw_components`] (the same
//!   pattern as the accounts adapter). The drift check then compares them with the contract, so
//!   a stale copy fails the build's tests;
//! - the response DTOs are tested against them (`tests::responses_match_the_contract_schemas`).

use std::borrow::Cow;
use std::sync::LazyLock;

use serde_json::{Map, Value};
use utoipa::openapi::RefOr;
use utoipa::openapi::schema::{Object, Schema};
use utoipa::{PartialSchema, ToSchema};

const SCHEMAS_JSON: &str = include_str!("schemas.json");

static SCHEMAS: LazyLock<Map<String, Value>> = LazyLock::new(|| {
    serde_json::from_str::<Value>(SCHEMAS_JSON)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
});

/// A contract component by name (`None` only if the embedded file is broken; tests catch it).
pub fn component(name: &str) -> Option<&'static Value> {
    SCHEMAS.get(name)
}

/// Component name → contract schema, for `app::openapi` to insert into the generated document.
pub fn raw_components() -> Vec<(&'static str, Value)> {
    PLACEHOLDERS
        .iter()
        .filter_map(|name| component(name).map(|v| (*name, v.clone())))
        .collect()
}

macro_rules! placeholder {
    ($($ty:ident => $name:literal),* $(,)?) => {
        $(
            #[doc = concat!("Placeholder for the contract component `", $name, "`.")]
            pub struct $ty;

            impl PartialSchema for $ty {
                fn schema() -> RefOr<Schema> {
                    Schema::Object(Object::new()).into()
                }
            }

            impl ToSchema for $ty {
                fn name() -> Cow<'static, str> {
                    Cow::Borrowed($name)
                }
            }
        )*

        const PLACEHOLDERS: &[&str] = &[$($name),*];
    };
}

placeholder! {
    RouteSchema => "Route",
    RouteListSchema => "RouteList",
    RouteCreateRequestSchema => "RouteCreateRequest",
    RouteUpdateRequestSchema => "RouteUpdateRequest",
    RouteCandidatesSchema => "RouteCandidates",
}
