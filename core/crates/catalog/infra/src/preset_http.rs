//! HTTP handlers of `listFirms`, `getFirm`, `listFirmReleases` and `getPresetVersion` (contract
//! `modules/presets.yaml`, S16). Global catalog data: any signed-in user reads it.
//!
//! `preset_schemas.json` holds the contract components (refs resolved) for the OpenAPI document,
//! inserted by `app::openapi` like the other modules' ones.

use std::borrow::Cow;
use std::sync::{Arc, LazyLock};

use access::SessionAuthenticator;
use access::http::{AccessState, Authenticated};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Map, Value};
use utoipa::openapi::RefOr;
use utoipa::openapi::schema::{Object, Schema};
use utoipa::{PartialSchema, ToSchema};
use uuid::Uuid;

use catalog_application::preset::PresetQueries;

const SCHEMAS_JSON: &str = include_str!("preset_schemas.json");

static SCHEMAS: LazyLock<Map<String, Value>> = LazyLock::new(|| {
    serde_json::from_str::<Value>(SCHEMAS_JSON)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
});

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
    FirmListSchema => "FirmList",
    FirmDetailSchema => "FirmDetail",
    FirmReleaseListSchema => "FirmReleaseList",
    PresetVersionSchema => "PresetVersion",
}

/// A contract component by name (for tests).
pub fn component(name: &str) -> Option<&'static Value> {
    SCHEMAS.get(name)
}

/// Component name → contract schema, for `app::openapi`.
pub fn raw_components() -> Vec<(&'static str, Value)> {
    PLACEHOLDERS
        .iter()
        .filter_map(|n| SCHEMAS.get(*n).map(|v| (*n, v.clone())))
        .collect()
}

pub struct PresetsHttpState {
    queries: PresetQueries,
    authenticator: Arc<dyn SessionAuthenticator>,
    public_origin: String,
}

impl PresetsHttpState {
    pub fn new(
        queries: PresetQueries,
        authenticator: Arc<dyn SessionAuthenticator>,
        public_origin: String,
    ) -> Self {
        Self {
            queries,
            authenticator,
            public_origin,
        }
    }
}

impl AccessState for PresetsHttpState {
    fn authenticator(&self) -> &Arc<dyn SessionAuthenticator> {
        &self.authenticator
    }

    fn public_origin(&self) -> &str {
        &self.public_origin
    }
}

type St = State<Arc<PresetsHttpState>>;

pub fn router(state: Arc<PresetsHttpState>) -> Router {
    Router::new()
        .route("/api/v1/firms", get(list_firms))
        .route("/api/v1/firms/{firm_id}", get(get_firm))
        .route("/api/v1/firms/{firm_id}/releases", get(list_firm_releases))
        .route(
            "/api/v1/preset-versions/{preset_version_id}",
            get(get_preset_version),
        )
        .with_state(state)
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(list_firms, get_firm, list_firm_releases, get_preset_version))]
pub struct PresetsApi;

fn error(status: StatusCode, code: &str, pointer: Option<&str>) -> Response {
    let mut e = contracts::http::Error::new(code);
    e.error.pointer = pointer.map(str::to_owned);
    (status, Json(e)).into_response()
}

fn internal(detail: &str) -> Response {
    tracing::error!(error = %detail, "presets: internal error");
    error(StatusCode::INTERNAL_SERVER_ERROR, "common.internal", None)
}

/// Lowercase canonical UUID (`components.schemas.Uuid`).
fn uuid_param(raw: &str, pointer: &str) -> Result<Uuid, Box<Response>> {
    let ok = raw.len() == 36
        && raw
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b) || b == b'-');
    ok.then(|| Uuid::parse_str(raw).ok())
        .flatten()
        .ok_or_else(|| {
            Box::new(error(
                StatusCode::BAD_REQUEST,
                "common.invalid_request",
                Some(pointer),
            ))
        })
}

fn found(result: Result<Option<Value>, catalog_application::CatalogError>) -> Response {
    match result {
        Ok(Some(v)) => Json(v).into_response(),
        Ok(None) => error(StatusCode::NOT_FOUND, "common.not_found", None),
        Err(e) => internal(&e.0),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/firms",
    operation_id = "listFirms",
    tag = "presets",
    responses(
        (status = 200, description = "The firms with presets.", body = FirmListSchema),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn list_firms(State(s): St, _caller: Authenticated) -> Response {
    match s.queries.list_firms().await {
        Ok(v) => Json(v).into_response(),
        Err(e) => internal(&e.0),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/firms/{firm_id}",
    operation_id = "getFirm",
    tag = "presets",
    params(("firm_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 200, description = "The firm.", body = FirmDetailSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_firm(State(s): St, _caller: Authenticated, Path(raw): Path<String>) -> Response {
    match uuid_param(&raw, "/path/firm_id") {
        Ok(id) => found(s.queries.get_firm(id).await),
        Err(r) => *r,
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/firms/{firm_id}/releases",
    operation_id = "listFirmReleases",
    tag = "presets",
    params(("firm_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 200, description = "The firm's releases, newest first.", body = FirmReleaseListSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn list_firm_releases(
    State(s): St,
    _caller: Authenticated,
    Path(raw): Path<String>,
) -> Response {
    match uuid_param(&raw, "/path/firm_id") {
        Ok(id) => found(s.queries.list_releases(id).await),
        Err(r) => *r,
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/preset-versions/{preset_version_id}",
    operation_id = "getPresetVersion",
    tag = "presets",
    params(("preset_version_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 200, description = "The preset version with its rules.", body = PresetVersionSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_preset_version(
    State(s): St,
    _caller: Authenticated,
    Path(raw): Path<String>,
) -> Response {
    match uuid_param(&raw, "/path/preset_version_id") {
        Ok(id) => found(s.queries.get_preset_version(id).await),
        Err(r) => *r,
    }
}
