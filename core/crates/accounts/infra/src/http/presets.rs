//! HTTP handlers of the per-account preset operations (contract `modules/presets.yaml`, HTTP
//! 6.0.0; S16, Torno): `getAccountPreset`, `getAccountPresetDiff`, `applyPresetVersion`,
//! `restorePresetValue`. Same precedence and error bodies as the accounts handlers (the parent
//! module's helpers).
//!
//! `presets/schemas.json` holds the contract components (refs resolved): requests are validated
//! against them, responses are tested against them, and `app::openapi` inserts the placeholders'
//! components ([`raw_components`]).

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::{Arc, LazyLock};

use access::SessionAuthenticator;
use access::http::{AccessState, Authenticated};
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Path, RawQuery, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use utoipa::openapi::RefOr;
use utoipa::openapi::schema::{Object, Schema};
use utoipa::{PartialSchema, ToSchema};
use uuid::Uuid;

use accounts_application::AccountsDeps;
use accounts_application::commands::{ApplyPresetVersion, OptInChoices, RestorePresetValue};
use accounts_application::enterprise::preset_values::account::{
    AccountValue, ApplyChoice, PRESET_FIELDS,
};
use accounts_application::enterprise::{FirmPlatform, MaxPositionQty, PresetVersionId, RuleField};
use accounts_application::presets::{
    AccountPresetView, ApplyPresetVersionUseCase, GetAccountPreset, GetAccountPresetDiff,
    PresetDiffView, ReleaseRef, RestorePresetValueUseCase,
};

use super::dto::{self, BadField, OptInsBody, StepUpBody, timestamp};
use super::{account_id, from_bad, from_error, from_invalid, invalid, request_origin, uuid_param};

const SCHEMAS_JSON: &str = include_str!("../presets/schemas.json");

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
    AccountPresetStateSchema => "AccountPresetState",
    PresetDiffSchema => "PresetDiff",
    ApplyPresetVersionRequestSchema => "ApplyPresetVersionRequest",
    RuleFieldSchema => "RuleField",
}

/// The inline request body of `restorePresetValue` (not a contract component).
pub struct RestorePresetValueBodySchema;

impl PartialSchema for RestorePresetValueBodySchema {
    fn schema() -> RefOr<Schema> {
        component("RestorePresetValueBody")
            .and_then(|v| serde_json::from_value::<Schema>(v.clone()).ok())
            .unwrap_or_else(|| Schema::Object(Object::new()))
            .into()
    }
}

impl ToSchema for RestorePresetValueBodySchema {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("RestorePresetValueBody")
    }
}

/// A contract component by name (`None` only if the embedded file is broken; tests catch it).
pub fn component(name: &str) -> Option<&'static Value> {
    SCHEMAS.get(name)
}

/// Component name → contract schema, for `app::openapi`.
pub fn raw_components() -> Vec<(&'static str, Value)> {
    PLACEHOLDERS
        .iter()
        .filter_map(|n| component(n).map(|v| (*n, v.clone())))
        .collect()
}

pub struct AccountPresetsHttpState {
    get: GetAccountPreset,
    diff: GetAccountPresetDiff,
    apply: ApplyPresetVersionUseCase,
    restore: RestorePresetValueUseCase,
    authenticator: Arc<dyn SessionAuthenticator>,
    public_origin: String,
    trust_forwarded_for: bool,
}

impl AccountPresetsHttpState {
    pub fn new(
        deps: AccountsDeps,
        authenticator: Arc<dyn SessionAuthenticator>,
        public_origin: String,
        trust_forwarded_for: bool,
    ) -> Self {
        Self {
            get: GetAccountPreset::new(deps.clone()),
            diff: GetAccountPresetDiff::new(deps.clone()),
            apply: ApplyPresetVersionUseCase::new(deps.clone()),
            restore: RestorePresetValueUseCase::new(deps),
            authenticator,
            public_origin,
            trust_forwarded_for,
        }
    }
}

impl AccessState for AccountPresetsHttpState {
    fn authenticator(&self) -> &Arc<dyn SessionAuthenticator> {
        &self.authenticator
    }

    fn public_origin(&self) -> &str {
        &self.public_origin
    }
}

type St = State<Arc<AccountPresetsHttpState>>;

pub fn router(state: Arc<AccountPresetsHttpState>) -> Router {
    Router::new()
        .route(
            "/api/v1/accounts/{account_id}/preset",
            get(get_account_preset),
        )
        .route(
            "/api/v1/accounts/{account_id}/preset-diff",
            get(get_account_preset_diff),
        )
        .route(
            "/api/v1/accounts/{account_id}/preset-version",
            post(apply_preset_version),
        )
        .route(
            "/api/v1/accounts/{account_id}/preset-values/{field}/restore",
            post(restore_preset_value),
        )
        .with_state(state)
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    get_account_preset,
    get_account_preset_diff,
    apply_preset_version,
    restore_preset_value,
))]
pub struct AccountPresetsApi;

// ---- requests -------------------------------------------------------------------------------

/// A body validated against its contract schema, then typed.
fn body<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
    name: &'static str,
    key: &'static str,
) -> Result<T, Box<Response>> {
    let schema = component(name).ok_or_else(|| Box::new(super::internal("schema missing")))?;
    let value =
        super::validate::body_with(bytes, key, schema).map_err(|i| Box::new(from_invalid(i)))?;
    serde_json::from_value(value)
        .map_err(|e| Box::new(super::internal(&format!("schema-valid body: {e}"))))
}

fn rule_field(raw: &str) -> Option<RuleField> {
    PRESET_FIELDS.into_iter().find(|f| f.as_str() == raw)
}

#[derive(Debug, Deserialize)]
struct ApplyBody {
    target_preset_version_id: Uuid,
    row_version: i32,
    firm_platform: Option<String>,
    #[serde(default)]
    choices: BTreeMap<String, String>,
    rule_values: Option<Value>,
    max_position_qty: Option<i64>,
    opt_ins: Option<OptInsBody>,
    step_up: Option<StepUpBody>,
}

#[derive(Debug, Deserialize)]
struct RestoreBody {
    row_version: i32,
}

fn opt_ins(body: Option<&OptInsBody>) -> Result<OptInChoices, BadField> {
    let Some(b) = body else {
        return Ok(OptInChoices::default());
    };
    Ok(OptInChoices {
        automation: b
            .automation
            .as_ref()
            .map(|c| dto::choice(c, "/opt_ins/automation/choice"))
            .transpose()?,
        copy: b
            .copy
            .as_ref()
            .map(|c| dto::choice(c, "/opt_ins/copy/choice"))
            .transpose()?,
    })
}

fn apply_command(
    account_id: accounts_application::enterprise::AccountId,
    b: ApplyBody,
) -> Result<ApplyPresetVersion, BadField> {
    let mut choices = Vec::new();
    for (key, value) in &b.choices {
        let field = rule_field(key).ok_or(BadField("/choices"))?;
        let choice = match value.as_str() {
            "keep_mine" => ApplyChoice::KeepMine,
            "use_preset" => ApplyChoice::UsePreset,
            _ => return Err(BadField("/choices")),
        };
        choices.push((field, choice));
    }
    Ok(ApplyPresetVersion {
        account_id,
        target_preset_version_id: PresetVersionId::from_uuid(b.target_preset_version_id),
        row_version: b.row_version,
        firm_platform: b
            .firm_platform
            .as_deref()
            .map(FirmPlatform::parse)
            .transpose()
            .map_err(|_| BadField("/firm_platform"))?,
        choices,
        rule_values: b.rule_values,
        max_position_qty: b
            .max_position_qty
            .map(MaxPositionQty::new)
            .transpose()
            .map_err(|_| BadField("/max_position_qty"))?,
        opt_ins: opt_ins(b.opt_ins.as_ref())?,
        step_up: b.step_up.map(|p| access::StepUpProof {
            challenge_id: p.challenge_id,
            code: p.code,
        }),
    })
}

// ---- responses ------------------------------------------------------------------------------

#[derive(Debug, Serialize)]
struct ReleaseRefDto {
    version_number: i32,
    published_at: String,
    verified_on: String,
}

impl From<&ReleaseRef> for ReleaseRefDto {
    fn from(r: &ReleaseRef) -> Self {
        Self {
            version_number: r.version_number,
            published_at: timestamp(r.published_at),
            verified_on: r.verified_on.to_string(),
        }
    }
}

fn account_value(v: &AccountValue) -> Value {
    match v {
        AccountValue::MaxPosition(n) => json!({ "contracts": n }),
        AccountValue::Rule(r) => serde_json::to_value(r.to_wire()).unwrap_or(Value::Null),
    }
}

#[derive(Debug, Serialize)]
struct AccountRuleFieldDto {
    field: &'static str,
    origin: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    chose: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chosen_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preset_value: Option<Value>,
}

#[derive(Debug, Serialize)]
struct NewerVersionDto {
    preset_version_id: Uuid,
    release: ReleaseRefDto,
    suspends: bool,
}

#[derive(Debug, Serialize)]
pub struct AccountPresetStateDto {
    account_id: Uuid,
    preset_version_id: Uuid,
    release: ReleaseRefDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    firm_platform: Option<String>,
    fields: Vec<AccountRuleFieldDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    newer_version: Option<NewerVersionDto>,
    row_version: i32,
}

impl From<&AccountPresetView> for AccountPresetStateDto {
    fn from(v: &AccountPresetView) -> Self {
        Self {
            account_id: v.account_id.as_uuid(),
            preset_version_id: v.preset_version_id.as_uuid(),
            release: ReleaseRefDto::from(&v.release),
            firm_platform: v.firm_platform.as_ref().map(|p| p.as_str().to_owned()),
            fields: v
                .fields
                .iter()
                .map(|f| AccountRuleFieldDto {
                    field: f.view.field.as_str(),
                    origin: f.view.origin.as_str(),
                    chose: f.view.chose.map(|(c, _)| c.as_str()),
                    chosen_at: f.view.chose.map(|(_, at)| timestamp(at)),
                    account_value: f.view.account_value.as_ref().map(account_value),
                    preset_value: f.preset_value.clone(),
                })
                .collect(),
            newer_version: v.newer_version.as_ref().map(|n| NewerVersionDto {
                preset_version_id: n.preset_version_id.as_uuid(),
                release: ReleaseRefDto::from(&n.release),
                suspends: n.suspends,
            }),
            row_version: v.row_version,
        }
    }
}

#[derive(Debug, Serialize)]
struct PresetDiffRowDto {
    field: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_preset_value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_preset_value: Option<Value>,
    edited: bool,
    requires_choice: bool,
    cap_conflict: bool,
}

#[derive(Debug, Serialize)]
struct OptInChangesDto {
    automation: Option<Uuid>,
    copy: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct PresetDiffDto {
    account_id: Uuid,
    from_preset_version_id: Uuid,
    to_preset_version_id: Uuid,
    rows: Vec<PresetDiffRowDto>,
    firm_platform_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    firm_platforms: Option<Vec<String>>,
    opt_in_changes: OptInChangesDto,
}

impl From<&PresetDiffView> for PresetDiffDto {
    fn from(v: &PresetDiffView) -> Self {
        Self {
            account_id: v.account_id.as_uuid(),
            from_preset_version_id: v.from_preset_version_id.as_uuid(),
            to_preset_version_id: v.to_preset_version_id.as_uuid(),
            rows: v
                .rows
                .iter()
                .map(|r| PresetDiffRowDto {
                    field: r.row.field.as_str(),
                    account_value: r.row.account_value.as_ref().map(account_value),
                    current_preset_value: r.current_preset_value.clone(),
                    new_preset_value: r.new_preset_value.clone(),
                    edited: r.row.edited,
                    requires_choice: r.row.requires_choice,
                    cap_conflict: r.row.cap_conflict,
                })
                .collect(),
            firm_platform_required: v.firm_platform_required,
            firm_platforms: v.firm_platform_required.then(|| {
                v.firm_platforms
                    .iter()
                    .map(|p| p.as_str().to_owned())
                    .collect()
            }),
            opt_in_changes: OptInChangesDto {
                automation: v.opt_in_changes[0].map(|id| id.as_uuid()),
                copy: v.opt_in_changes[1].map(|id| id.as_uuid()),
            },
        }
    }
}

fn state_json(v: &AccountPresetView) -> Response {
    Json(AccountPresetStateDto::from(v)).into_response()
}

// ---- handlers -------------------------------------------------------------------------------

#[utoipa::path(
    get,
    path = "/api/v1/accounts/{account_id}/preset",
    operation_id = "getAccountPreset",
    tag = "presets",
    params(("account_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 200, description = "The account's preset state.", body = AccountPresetStateSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Unknown or foreign account, or an account without a preset.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_account_preset(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw): Path<String>,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    match s.get.handle(&caller, id).await {
        Ok(v) => state_json(&v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/accounts/{account_id}/preset-diff",
    operation_id = "getAccountPresetDiff",
    tag = "presets",
    params(
        ("account_id" = contracts::http::Uuid, Path),
        ("target_preset_version_id" = contracts::http::Uuid, Query),
    ),
    responses(
        (status = 200, description = "The diff.", body = PresetDiffSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 422, description = "Not a newer version of the same preset.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_account_preset_diff(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw): Path<String>,
    RawQuery(query): RawQuery,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    let pointer = "/query/target_preset_version_id";
    let mut target = None;
    for pair in query.as_deref().unwrap_or("").split('&') {
        match pair.split_once('=') {
            Some(("target_preset_version_id", v)) if target.is_none() => {
                let Some(t) = uuid_param(v) else {
                    return invalid(Some(pointer.into()));
                };
                target = Some(t);
            }
            Some(("target_preset_version_id", _)) => return invalid(Some(pointer.into())),
            _ => {}
        }
    }
    let Some(target) = target else {
        return invalid(Some(pointer.into()));
    };
    match s
        .diff
        // `firm_platform` query: HTTP 6.2.0 (in review); until it is approved, the account's own.
        .handle(&caller, id, PresetVersionId::from_uuid(target), None)
        .await
    {
        Ok(v) => Json(PresetDiffDto::from(&v)).into_response(),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/accounts/{account_id}/preset-version",
    operation_id = "applyPresetVersion",
    tag = "presets",
    params(("account_id" = contracts::http::Uuid, Path)),
    request_body(content = ApplyPresetVersionRequestSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Applied and audited.", body = AccountPresetStateSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Step-up failed or forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Conflict; nothing saved.", body = contracts::http::Error),
        (status = 422, description = "Refused by a rule; nothing saved.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn apply_preset_version(
    State(s): St,
    Authenticated(caller): Authenticated,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    Path(raw): Path<String>,
    bytes: Bytes,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    let b = match body::<ApplyBody>(
        &bytes,
        "ApplyPresetVersionRequest",
        "presets.ApplyPresetVersionRequest",
    ) {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let cmd = match apply_command(id, b) {
        Ok(c) => c,
        Err(bad) => return from_bad(bad),
    };
    let origin = request_origin(&headers, peer.map(|c| (c.0).0), s.trust_forwarded_for);
    match s.apply.handle(&caller, &origin, cmd).await {
        Ok(v) => state_json(&v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/accounts/{account_id}/preset-values/{field}/restore",
    operation_id = "restorePresetValue",
    tag = "presets",
    params(
        ("account_id" = contracts::http::Uuid, Path),
        ("field" = RuleFieldSchema, Path),
    ),
    request_body(content = inline(RestorePresetValueBodySchema), content_type = "application/json"),
    responses(
        (status = 200, description = "Restored.", body = AccountPresetStateSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Stale row_version; nothing changed.", body = contracts::http::Error),
        (status = 422, description = "The preset value is not confirmed; nothing changed.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn restore_preset_value(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path((raw_id, raw_field)): Path<(String, String)>,
    bytes: Bytes,
) -> Response {
    let id = match account_id(&raw_id) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    let Some(field) = rule_field(&raw_field) else {
        return invalid(Some("/path/field".into()));
    };
    let b = match body::<RestoreBody>(
        &bytes,
        "RestorePresetValueBody",
        "presets.RestorePresetValueBody",
    ) {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let cmd = RestorePresetValue {
        account_id: id,
        field,
        row_version: b.row_version,
    };
    match s.restore.handle(&caller, cmd).await {
        Ok(v) => state_json(&v),
        Err(e) => from_error(e),
    }
}
