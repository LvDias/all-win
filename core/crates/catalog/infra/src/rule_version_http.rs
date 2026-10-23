//! HTTP handlers of `getOptInRules` and `getRuleVersion` (contract `modules/presets.yaml`, S5
//! part). Global catalog data: any signed-in user reads it; nothing is user-scoped.
//!
//! `rule_version_schemas.json` holds the contract components (refs resolved) for the OpenAPI
//! document, inserted by `app::openapi` like the accounts ones (see `accounts_infra::http::schemas`).

use std::borrow::Cow;
use std::sync::{Arc, LazyLock};

use access::SessionAuthenticator;
use access::http::{AccessState, Authenticated};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use chrono::SecondsFormat;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use utoipa::openapi::RefOr;
use utoipa::openapi::schema::{Object, Schema};
use utoipa::{PartialSchema, ToSchema};
use uuid::Uuid;

use catalog_application::rule_version::{OptInRulesError, RuleVersion, RuleVersionQueries};

const SCHEMAS_JSON: &str = include_str!("rule_version_schemas.json");

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
    OptInRulesSchema => "OptInRules",
    RuleVersionSchema => "RuleVersion",
    FirmPlatformSchema => "FirmPlatform",
}

/// Component name → contract schema, for `app::openapi`.
pub fn raw_components() -> Vec<(&'static str, Value)> {
    PLACEHOLDERS
        .iter()
        .filter_map(|n| SCHEMAS.get(*n).map(|v| (*n, v.clone())))
        .collect()
}

pub struct RuleVersionHttpState {
    queries: RuleVersionQueries,
    authenticator: Arc<dyn SessionAuthenticator>,
    public_origin: String,
}

impl RuleVersionHttpState {
    pub fn new(
        queries: RuleVersionQueries,
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

impl AccessState for RuleVersionHttpState {
    fn authenticator(&self) -> &Arc<dyn SessionAuthenticator> {
        &self.authenticator
    }

    fn public_origin(&self) -> &str {
        &self.public_origin
    }
}

pub fn router(state: Arc<RuleVersionHttpState>) -> Router {
    Router::new()
        .route("/api/v1/opt-in-rules", get(get_opt_in_rules))
        .route(
            "/api/v1/rule-versions/{rule_version_id}",
            get(get_rule_version),
        )
        .with_state(state)
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(get_opt_in_rules, get_rule_version))]
pub struct RuleVersionApi;

#[derive(Debug, Serialize)]
struct ChangeDto {
    previous_rule_version_id: Uuid,
    previous_firm_stance: String,
    cause: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct RuleVersionDto {
    id: Uuid,
    kind: String,
    firm_stance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    firm_platform: Option<String>,
    is_generic: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    firm_id: Option<Uuid>,
    version_number: i32,
    published_at: String,
    quotes: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    research: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    change: Option<ChangeDto>,
}

impl From<RuleVersion> for RuleVersionDto {
    fn from(r: RuleVersion) -> Self {
        Self {
            id: r.id,
            kind: r.kind,
            firm_stance: r.firm_stance,
            firm_platform: r.firm_platform,
            is_generic: r.is_generic,
            firm_id: r.firm_id,
            version_number: r.version_number,
            published_at: r.published_at.to_rfc3339_opts(SecondsFormat::AutoSi, true),
            quotes: r.quotes,
            research: r.research,
            change: r.change.map(|c| ChangeDto {
                previous_rule_version_id: c.previous_rule_version_id,
                previous_firm_stance: c.previous_firm_stance,
                cause: c.cause,
                reason: c.reason,
            }),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OptInRulesDto {
    automation: RuleVersionDto,
    copy: RuleVersionDto,
}

fn error(status: StatusCode, code: &str, pointer: Option<&str>) -> Response {
    let mut e = contracts::http::Error::new(code);
    e.error.pointer = pointer.map(str::to_owned);
    (status, Json(e)).into_response()
}

fn internal(detail: &str) -> Response {
    tracing::error!(error = %detail, "catalog: internal error");
    error(StatusCode::INTERNAL_SERVER_ERROR, "common.internal", None)
}

/// Lowercase canonical UUID (`components.schemas.Uuid`).
fn uuid_param(raw: &str) -> Option<Uuid> {
    let ok = raw.len() == 36
        && raw
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b) || b == b'-');
    ok.then(|| Uuid::parse_str(raw).ok()).flatten()
}

fn firm_platform_ok(raw: &str) -> bool {
    let mut chars = raw.chars();
    raw.len() <= 32
        && chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[derive(Debug, Deserialize)]
struct RulesQuery {
    preset_version_id: Option<String>,
    firm_platform: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/v1/opt-in-rules",
    operation_id = "getOptInRules",
    tag = "presets",
    params(
        ("preset_version_id" = Option<contracts::http::Uuid>, Query),
        ("firm_platform" = Option<FirmPlatformSchema>, Query),
    ),
    responses(
        (status = 200, description = "One rule version per kind.", body = OptInRulesSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Unknown preset version.", body = contracts::http::Error),
        (status = 422, description = "Firm platform required or unknown.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_opt_in_rules(
    State(s): State<Arc<RuleVersionHttpState>>,
    _caller: Authenticated,
    query: Result<Query<RulesQuery>, axum::extract::rejection::QueryRejection>,
) -> Response {
    let Ok(Query(q)) = query else {
        return error(StatusCode::BAD_REQUEST, "common.invalid_request", None);
    };
    let preset = match q.preset_version_id.as_deref().map(uuid_param) {
        None => None,
        Some(Some(id)) => Some(id),
        Some(None) => {
            return error(
                StatusCode::BAD_REQUEST,
                "common.invalid_request",
                Some("/query/preset_version_id"),
            );
        }
    };
    if q.firm_platform
        .as_deref()
        .is_some_and(|p| !firm_platform_ok(p))
    {
        return error(
            StatusCode::BAD_REQUEST,
            "common.invalid_request",
            Some("/query/firm_platform"),
        );
    }
    match s
        .queries
        .for_selection(preset, q.firm_platform.as_deref())
        .await
    {
        Ok([automation, copy]) => Json(OptInRulesDto {
            automation: automation.into(),
            copy: copy.into(),
        })
        .into_response(),
        Err(OptInRulesError::PresetVersionUnknown) => {
            error(StatusCode::NOT_FOUND, "common.not_found", None)
        }
        Err(OptInRulesError::FirmPlatformUnknown) => error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "accounts.firm_platform_unknown",
            Some("/query/firm_platform"),
        ),
        Err(OptInRulesError::Catalog(m)) => internal(&m),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/rule-versions/{rule_version_id}",
    operation_id = "getRuleVersion",
    tag = "presets",
    params(("rule_version_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 200, description = "The rule version.", body = RuleVersionSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_rule_version(
    State(s): State<Arc<RuleVersionHttpState>>,
    _caller: Authenticated,
    Path(raw): Path<String>,
) -> Response {
    let Some(id) = uuid_param(&raw) else {
        return error(
            StatusCode::BAD_REQUEST,
            "common.invalid_request",
            Some("/path/rule_version_id"),
        );
    };
    match s.queries.get(id).await {
        Ok(Some(r)) => Json(RuleVersionDto::from(r)).into_response(),
        Ok(None) => error(StatusCode::NOT_FOUND, "common.not_found", None),
        Err(e) => internal(&e.0),
    }
}
