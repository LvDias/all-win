//! Contract schemas of the profile operations (`contracts/http/modules/profiles.yaml`).
//!
//! `schemas.json` holds the contract components with their `$ref`s resolved, extracted from
//! `contracts/http/openapi.bundled.yaml` (plus the inline restore body as `ProfileRestoreRequest`).
//! Same pattern as `http::schemas`: requests are validated against them before they are
//! deserialized, `app::openapi` inserts them verbatim for the placeholders below, and the
//! response bodies are tested against them.

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
    DistanceUnitSchema => "DistanceUnit",
    DistanceValueSchema => "DistanceValue",
    DistanceProfileSchema => "DistanceProfile",
    AccountProfileRequestSchema => "AccountProfileRequest",
    ProfileRestoreRequestSchema => "ProfileRestoreRequest",
    ProfileEquivalentsSchema => "ProfileEquivalents",
    NearestProfileSizeSchema => "NearestProfileSize",
    DefaultProfileEntrySchema => "DefaultProfileEntry",
    DefaultProfileListSchema => "DefaultProfileList",
    AccountProfileSchema => "AccountProfile",
    AccountProfileListSchema => "AccountProfileList",
    SizeAmountSchema => "SizeAmount",
}
