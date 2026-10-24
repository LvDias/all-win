//! HTTP handlers of the profile operations (contract `modules/profiles.yaml`).
//! Precedence: 401 → 403 origin (extractor) → 400 (path, query, body schema) → 404 → 409 → 422.
//! No step-up (Q35 list). Error pointers: `/path/…`, `/query/…` (preview, nearest, list), or the
//! body field.

use std::collections::HashMap;
use std::sync::Arc;

use access::SessionAuthenticator;
use access::http::{AccessState, Authenticated};
use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use accounts_application::enterprise::profile::{
    DistanceProfile, DistanceUnit, Equivalent, ProfileEquivalents,
};
use accounts_application::enterprise::{AccountId, SizeAmount};
use accounts_application::profiles::events::distance_json;
use accounts_application::profiles::{
    AccountProfileEntry, AccountProfileView, DefaultEntry, NearestCandidate, ProfileUseCases,
    ProfilesError,
};

use super::schemas::{self, *};
use crate::http::validate::{self, Invalid};

pub struct ProfilesHttpState {
    uc: ProfileUseCases,
    authenticator: Arc<dyn SessionAuthenticator>,
    public_origin: String,
}

impl ProfilesHttpState {
    pub fn new(
        uc: ProfileUseCases,
        authenticator: Arc<dyn SessionAuthenticator>,
        public_origin: String,
    ) -> Self {
        Self {
            uc,
            authenticator,
            public_origin,
        }
    }
}

impl AccessState for ProfilesHttpState {
    fn authenticator(&self) -> &Arc<dyn SessionAuthenticator> {
        &self.authenticator
    }

    fn public_origin(&self) -> &str {
        &self.public_origin
    }
}

type St = State<Arc<ProfilesHttpState>>;

pub fn router(state: Arc<ProfilesHttpState>) -> Router {
    Router::new()
        .route("/api/v1/default-profiles", get(list_default_profiles))
        .route(
            "/api/v1/default-profiles/{instrument_id}/{size_amount}",
            put(put_default_profile),
        )
        .route(
            "/api/v1/accounts/{account_id}/profiles",
            get(list_account_profiles),
        )
        .route(
            "/api/v1/accounts/{account_id}/profiles/{instrument_id}",
            put(put_account_profile),
        )
        .route(
            "/api/v1/accounts/{account_id}/profiles/{instrument_id}/restore",
            post(restore_account_profile),
        )
        .route(
            "/api/v1/profile-sizes/nearest",
            get(get_nearest_profile_size),
        )
        .route("/api/v1/profile-preview", get(preview_profile))
        .with_state(state)
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        list_default_profiles,
        put_default_profile,
        list_account_profiles,
        put_account_profile,
        restore_account_profile,
        get_nearest_profile_size,
        preview_profile,
    ),
    modifiers(&OptionalRestoreBody)
)]
pub struct ProfilesApi;

/// The restore body is optional (`requestBody.required: false`) with the plain object schema:
/// `Option<T>` would render `oneOf [T, null]`, which the contract does not have.
struct OptionalRestoreBody;

impl utoipa::Modify for OptionalRestoreBody {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let body = openapi
            .paths
            .paths
            .get_mut("/api/v1/accounts/{account_id}/profiles/{instrument_id}/restore")
            .and_then(|item| item.post.as_mut())
            .and_then(|op| op.request_body.as_mut());
        if let Some(utoipa::openapi::RefOr::T(body)) = body {
            body.required = Some(utoipa::openapi::Required::False);
        }
    }
}

pub use schemas::raw_components;

// ---- errors -------------------------------------------------------------------------------

fn error_body(status: StatusCode, code: &str, pointer: Option<String>) -> Response {
    let mut e = contracts::http::Error::new(code);
    e.error.pointer = pointer;
    (status, Json(e)).into_response()
}

fn invalid(pointer: impl Into<String>) -> Response {
    error_body(
        StatusCode::BAD_REQUEST,
        "common.invalid_request",
        Some(pointer.into()),
    )
}

fn internal(detail: &str) -> Response {
    tracing::error!(error = %detail, "profiles: internal error");
    error_body(StatusCode::INTERNAL_SERVER_ERROR, "common.internal", None)
}

/// `prefix` is `""` for body fields and `"/query"` for query parameters.
fn from_error(e: ProfilesError, prefix: &str) -> Response {
    let status = match &e {
        ProfilesError::NotFound => StatusCode::NOT_FOUND,
        ProfilesError::VersionConflict => StatusCode::CONFLICT,
        ProfilesError::Distance(_)
        | ProfilesError::DefaultMissing
        | ProfilesError::ProfileSizeRequired
        | ProfilesError::ProfileSizeInvalid => StatusCode::UNPROCESSABLE_ENTITY,
        ProfilesError::Internal(m) => return internal(m),
    };
    error_body(status, e.code(), e.field().map(|f| format!("{prefix}/{f}")))
}

fn from_invalid(i: Invalid, prefix: &str) -> Response {
    match i {
        Invalid::Malformed => error_body(StatusCode::BAD_REQUEST, "common.invalid_request", None),
        Invalid::At(p) => invalid(format!("{prefix}{p}")),
        Invalid::Unavailable => internal("profiles schema unavailable"),
    }
}

// ---- request parts ------------------------------------------------------------------------

/// Rejections are boxed: a `Response` is large (clippy `result_large_err`).
type Rejection = Box<Response>;

/// Lowercase canonical UUID (`components.schemas.Uuid`).
fn uuid_at(raw: Option<&str>, pointer: &str) -> Result<Uuid, Rejection> {
    raw.filter(|r| {
        r.len() == 36
            && r.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b) || b == b'-')
    })
    .and_then(|r| Uuid::parse_str(r).ok())
    .ok_or_else(|| Box::new(invalid(pointer)))
}

/// `SizeAmount` (contract pattern, then the value type).
fn size_at(raw: Option<&str>, pointer: &str) -> Result<SizeAmount, Rejection> {
    let schema = schemas::component("SizeAmount")
        .ok_or_else(|| Box::new(internal("profiles schema unavailable")))?;
    let raw = raw.ok_or_else(|| Box::new(invalid(pointer)))?;
    validate::value_with(&json!(raw), "profiles.SizeAmount", schema)
        .map_err(|_| Box::new(invalid(pointer)))?;
    raw.parse()
        .ok()
        .and_then(|d| SizeAmount::new(d).ok())
        .ok_or_else(|| Box::new(invalid(pointer)))
}

/// A distance pair in `unit`: the unit's shape (`DistanceProfileShape`); the side that does not
/// fit is the pointer (`{prefix}/stop_distance` or `…/target_distance`).
fn distances(
    unit: DistanceUnit,
    stop: &str,
    target: &str,
    prefix: &str,
) -> Result<DistanceProfile, Rejection> {
    DistanceProfile::parse(unit, stop, target)
        .map_err(|e| Box::new(invalid(format!("{prefix}/{}", e.side.field()))))
}

fn query(uri: &Uri) -> Result<HashMap<String, String>, Rejection> {
    Query::<HashMap<String, String>>::try_from_uri(uri)
        .map(|q| q.0)
        .map_err(|_| {
            Box::new(error_body(
                StatusCode::BAD_REQUEST,
                "common.invalid_request",
                None,
            ))
        })
}

fn parse_body<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    name: &'static str,
    key: &'static str,
) -> Result<T, Rejection> {
    let schema = schemas::component(name)
        .ok_or_else(|| Box::new(internal("profiles schema unavailable")))?;
    let value =
        validate::body_with(bytes, key, schema).map_err(|i| Box::new(from_invalid(i, "")))?;
    serde_json::from_value(value).map_err(|e| {
        Box::new(internal(&format!(
            "schema-valid body not representable: {e}"
        )))
    })
}

#[derive(Deserialize)]
struct DistanceBody {
    distance_unit: String,
    stop_distance: String,
    target_distance: String,
    #[serde(default)]
    row_version: Option<i64>,
}

impl DistanceBody {
    fn profile(&self) -> Result<DistanceProfile, Rejection> {
        let unit = DistanceUnit::parse(&self.distance_unit)
            .ok_or_else(|| Box::new(invalid("/distance_unit")))?;
        distances(unit, &self.stop_distance, &self.target_distance, "")
    }
}

#[derive(Deserialize)]
struct RestoreBody {
    #[serde(default)]
    profile_size: Option<String>,
    #[serde(default)]
    row_version: Option<i64>,
}

/// A `row_version` beyond `i32` can never match a stored one: it conflicts like any stale value.
fn row_version(v: Option<i64>) -> Option<i32> {
    v.map(|v| i32::try_from(v).unwrap_or(-1))
}

// ---- responses ----------------------------------------------------------------------------

fn ts(t: DateTime<Utc>) -> String {
    t.to_rfc3339_opts(SecondsFormat::AutoSi, true)
}

fn equivalent_json(e: &Equivalent) -> Value {
    json!({
        "ticks": e.ticks,
        "points": e.points.to_string(),
        "usd": e.usd.to_string(),
        "rounded": e.rounded,
    })
}

fn equivalents_json(instrument_id: Uuid, e: &ProfileEquivalents) -> Value {
    json!({
        "instrument_id": instrument_id,
        "stop": equivalent_json(&e.stop),
        "target": equivalent_json(&e.target),
    })
}

fn account_profile_json(v: &AccountProfileView) -> Value {
    let p = &v.profile;
    let mut out = json!({
        "account_id": v.account_id.as_uuid(),
        "instrument_id": p.instrument_id(),
        "profile": distance_json(p.profile()),
        "equivalents": equivalents_json(p.instrument_id(), &v.equivalents),
        "origin": p.origin().as_str(),
        "customized": p.customized(),
        "row_version": p.row_version(),
        "updated_at": ts(p.updated_at()),
    });
    if let Some(c) = p.copied_from() {
        out["copied_from_size"] = json!(c.size_amount.to_wire());
        out["copied_at"] = json!(ts(c.at));
        if let Some(d) = &v.default_now {
            out["default_now"] = distance_json(d);
        }
    }
    out
}

fn account_profiles_json(account_id: AccountId, entries: &[AccountProfileEntry]) -> Value {
    let profiles: Vec<Value> = entries
        .iter()
        .map(|e| match &e.profile {
            Some(v) => {
                json!({"instrument_id": e.instrument_id, "profile": account_profile_json(v)})
            }
            None => json!({"instrument_id": e.instrument_id}),
        })
        .collect();
    json!({"account_id": account_id.as_uuid(), "profiles": profiles})
}

fn default_entry_json(e: &DefaultEntry) -> Value {
    let mut out = json!({"instrument_id": e.instrument_id, "size_amount": e.size_amount.to_wire()});
    if let Some((profile, updated_at)) = &e.profile {
        out["profile"] = distance_json(profile);
        out["updated_at"] = json!(ts(*updated_at));
    }
    out
}

fn nearest_json(size: &SizeAmount, candidates: &[NearestCandidate]) -> Value {
    let c: Vec<Value> = candidates
        .iter()
        .map(|c| json!({"size_amount": c.size_amount.to_wire(), "has_default": c.has_default}))
        .collect();
    json!({"size_amount": size.to_wire(), "candidates": c})
}

// ---- handlers -----------------------------------------------------------------------------

#[utoipa::path(
    get,
    path = "/api/v1/default-profiles",
    operation_id = "listDefaultProfiles",
    tag = "profiles",
    params(("instrument_id" = contracts::http::Uuid, Query)),
    responses(
        (status = 200, description = "The defaults.", body = DefaultProfileListSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn list_default_profiles(
    State(s): St,
    Authenticated(caller): Authenticated,
    uri: Uri,
) -> Response {
    let run = async {
        let q = query(&uri)?;
        let instrument_id = uuid_at(
            q.get("instrument_id").map(String::as_str),
            "/query/instrument_id",
        )?;
        let defaults =
            s.uc.list_defaults(&caller, instrument_id)
                .await
                .map_err(|e| Box::new(from_error(e, "/query")))?;
        let list: Vec<Value> = defaults.iter().map(default_entry_json).collect();
        Ok::<_, Rejection>(
            Json(json!({"instrument_id": instrument_id, "defaults": list})).into_response(),
        )
    };
    run.await.unwrap_or_else(|r| *r)
}

#[utoipa::path(
    put,
    path = "/api/v1/default-profiles/{instrument_id}/{size_amount}",
    operation_id = "putDefaultProfile",
    tag = "profiles",
    params(
        ("instrument_id" = contracts::http::Uuid, Path),
        ("size_amount" = SizeAmountSchema, Path),
    ),
    request_body(content = DistanceProfileSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Saved.", body = DefaultProfileEntrySchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 422, description = "Refused by a rule; nothing saved.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn put_default_profile(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path((raw_instrument, raw_size)): Path<(String, String)>,
    body: Bytes,
) -> Response {
    let run = async {
        let instrument_id = uuid_at(Some(&raw_instrument), "/path/instrument_id")?;
        let size = size_at(Some(&raw_size), "/path/size_amount")?;
        let b: DistanceBody = parse_body(&body, "DistanceProfile", "profiles.DistanceProfile")?;
        let profile = b.profile()?;
        let entry =
            s.uc.put_default(&caller, instrument_id, size, profile)
                .await
                .map_err(|e| Box::new(from_error(e, "")))?;
        Ok::<_, Rejection>(Json(default_entry_json(&entry)).into_response())
    };
    run.await.unwrap_or_else(|r| *r)
}

#[utoipa::path(
    get,
    path = "/api/v1/accounts/{account_id}/profiles",
    operation_id = "listAccountProfiles",
    tag = "profiles",
    params(("account_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 200, description = "The account's profiles.", body = AccountProfileListSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn list_account_profiles(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw_account): Path<String>,
) -> Response {
    let run = async {
        let account_id = AccountId::from_uuid(uuid_at(Some(&raw_account), "/path/account_id")?);
        let entries =
            s.uc.list_account_profiles(&caller, account_id)
                .await
                .map_err(|e| Box::new(from_error(e, "")))?;
        Ok::<_, Rejection>(Json(account_profiles_json(account_id, &entries)).into_response())
    };
    run.await.unwrap_or_else(|r| *r)
}

#[utoipa::path(
    put,
    path = "/api/v1/accounts/{account_id}/profiles/{instrument_id}",
    operation_id = "putAccountProfile",
    tag = "profiles",
    params(
        ("account_id" = contracts::http::Uuid, Path),
        ("instrument_id" = contracts::http::Uuid, Path),
    ),
    request_body(content = AccountProfileRequestSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Saved and audited.", body = AccountProfileSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Version conflict; nothing saved.", body = contracts::http::Error),
        (status = 422, description = "Refused by a rule; nothing saved.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn put_account_profile(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path((raw_account, raw_instrument)): Path<(String, String)>,
    body: Bytes,
) -> Response {
    let run = async {
        let account_id = AccountId::from_uuid(uuid_at(Some(&raw_account), "/path/account_id")?);
        let instrument_id = uuid_at(Some(&raw_instrument), "/path/instrument_id")?;
        let b: DistanceBody = parse_body(
            &body,
            "AccountProfileRequest",
            "profiles.AccountProfileRequest",
        )?;
        let profile = b.profile()?;
        let view =
            s.uc.put_account_profile(
                &caller,
                account_id,
                instrument_id,
                profile,
                row_version(b.row_version),
            )
            .await
            .map_err(|e| Box::new(from_error(e, "")))?;
        Ok::<_, Rejection>(Json(account_profile_json(&view)).into_response())
    };
    run.await.unwrap_or_else(|r| *r)
}

#[utoipa::path(
    post,
    path = "/api/v1/accounts/{account_id}/profiles/{instrument_id}/restore",
    operation_id = "restoreAccountProfile",
    tag = "profiles",
    params(
        ("account_id" = contracts::http::Uuid, Path),
        ("instrument_id" = contracts::http::Uuid, Path),
    ),
    request_body(content = ProfileRestoreRequestSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Restored.", body = AccountProfileSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Version conflict; nothing changed.", body = contracts::http::Error),
        (status = 422, description = "Refused by a rule; nothing changed.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn restore_account_profile(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path((raw_account, raw_instrument)): Path<(String, String)>,
    body: Bytes,
) -> Response {
    let run = async {
        let account_id = AccountId::from_uuid(uuid_at(Some(&raw_account), "/path/account_id")?);
        let instrument_id = uuid_at(Some(&raw_instrument), "/path/instrument_id")?;
        // The body is optional.
        let b = if body.is_empty() {
            RestoreBody {
                profile_size: None,
                row_version: None,
            }
        } else {
            parse_body(
                &body,
                "ProfileRestoreRequest",
                "profiles.ProfileRestoreRequest",
            )?
        };
        let profile_size = match b.profile_size.as_deref() {
            Some(raw) => Some(size_at(Some(raw), "/profile_size")?),
            None => None,
        };
        let view =
            s.uc.restore_account_profile(
                &caller,
                account_id,
                instrument_id,
                profile_size,
                row_version(b.row_version),
            )
            .await
            .map_err(|e| Box::new(from_error(e, "")))?;
        Ok::<_, Rejection>(Json(account_profile_json(&view)).into_response())
    };
    run.await.unwrap_or_else(|r| *r)
}

#[utoipa::path(
    get,
    path = "/api/v1/profile-sizes/nearest",
    operation_id = "getNearestProfileSize",
    tag = "profiles",
    params(
        ("size_amount" = SizeAmountSchema, Query),
        ("instrument_id" = contracts::http::Uuid, Query),
    ),
    responses(
        (status = 200, description = "One or two candidates.", body = NearestProfileSizeSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_nearest_profile_size(
    State(s): St,
    Authenticated(caller): Authenticated,
    uri: Uri,
) -> Response {
    let run = async {
        let q = query(&uri)?;
        let size = size_at(
            q.get("size_amount").map(String::as_str),
            "/query/size_amount",
        )?;
        let instrument_id = uuid_at(
            q.get("instrument_id").map(String::as_str),
            "/query/instrument_id",
        )?;
        let candidates =
            s.uc.nearest(&caller, &size, instrument_id)
                .await
                .map_err(|e| Box::new(from_error(e, "/query")))?;
        Ok::<_, Rejection>(Json(nearest_json(&size, &candidates)).into_response())
    };
    run.await.unwrap_or_else(|r| *r)
}

#[utoipa::path(
    get,
    path = "/api/v1/profile-preview",
    operation_id = "previewProfile",
    tag = "profiles",
    params(
        ("instrument_id" = contracts::http::Uuid, Query),
        ("distance_unit" = DistanceUnitSchema, Query),
        ("stop_distance" = DistanceValueSchema, Query),
        ("target_distance" = DistanceValueSchema, Query),
    ),
    responses(
        (status = 200, description = "The equivalents.", body = ProfileEquivalentsSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 422, description = "Refused by a rule.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn preview_profile(State(s): St, _caller: Authenticated, uri: Uri) -> Response {
    let run = async {
        let q = query(&uri)?;
        let param = |name: &str| q.get(name).map(String::as_str);
        let instrument_id = uuid_at(param("instrument_id"), "/query/instrument_id")?;
        let unit = param("distance_unit")
            .and_then(DistanceUnit::parse)
            .ok_or_else(|| Box::new(invalid("/query/distance_unit")))?;
        let stop =
            param("stop_distance").ok_or_else(|| Box::new(invalid("/query/stop_distance")))?;
        let target =
            param("target_distance").ok_or_else(|| Box::new(invalid("/query/target_distance")))?;
        let requested = distances(unit, stop, target, "/query")?;
        let eq =
            s.uc.preview(instrument_id, &requested)
                .await
                .map_err(|e| Box::new(from_error(e, "/query")))?;
        Ok::<_, Rejection>(Json(equivalents_json(instrument_id, &eq)).into_response())
    };
    run.await.unwrap_or_else(|r| *r)
}
