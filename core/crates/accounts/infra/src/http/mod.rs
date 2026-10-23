//! HTTP handlers of the accounts operations (contract `modules/accounts.yaml`, 4.3.0).
//! Precedence: 401 → 403 origin (extractor) → 400 (path, header, body schema) → 404 → step-up →
//! 409 → 422. Error bodies follow the contract `Error` schema.

pub mod dto;
pub mod schemas;
pub mod validate;

#[cfg(test)]
mod tests;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use access::http::{AccessState, Authenticated};
use access::{RequestOrigin, SessionAuthenticator};
use axum::Router;
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use axum::{Extension, Json};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use accounts_application::commands::{RestartEvaluation, SetLifecycle, SetOptIn};
use accounts_application::enterprise::{AccountId, LifecycleState, OptInKind};
use accounts_application::{
    AccountsDeps, AccountsError, ArchiveAccount, CreateAccountUseCase, CreateOutcome, ReadAccounts,
    RestartAccountEvaluation, SetAccountLifecycle, SetAccountOptIn, UpdateAccountUseCase,
};

use dto::{AccountDto, AccountListDto, ArchiveImpactDto, BadField, OptInStateDto};
use schemas::*;
use validate::Invalid;

pub struct AccountsHttpState {
    read: ReadAccounts,
    create: CreateAccountUseCase,
    update: UpdateAccountUseCase,
    lifecycle: SetAccountLifecycle,
    opt_in: SetAccountOptIn,
    archive: ArchiveAccount,
    restart: RestartAccountEvaluation,
    authenticator: Arc<dyn SessionAuthenticator>,
    public_origin: String,
    trust_forwarded_for: bool,
}

impl AccountsHttpState {
    pub fn new(
        deps: AccountsDeps,
        authenticator: Arc<dyn SessionAuthenticator>,
        public_origin: String,
        trust_forwarded_for: bool,
    ) -> Self {
        Self {
            read: ReadAccounts::new(deps.clone()),
            create: CreateAccountUseCase::new(deps.clone()),
            update: UpdateAccountUseCase::new(deps.clone()),
            lifecycle: SetAccountLifecycle::new(deps.clone()),
            opt_in: SetAccountOptIn::new(deps.clone()),
            archive: ArchiveAccount::new(deps.clone()),
            restart: RestartAccountEvaluation::new(deps),
            authenticator,
            public_origin,
            trust_forwarded_for,
        }
    }
}

impl AccessState for AccountsHttpState {
    fn authenticator(&self) -> &Arc<dyn SessionAuthenticator> {
        &self.authenticator
    }

    fn public_origin(&self) -> &str {
        &self.public_origin
    }
}

type St = State<Arc<AccountsHttpState>>;

pub fn router(state: Arc<AccountsHttpState>) -> Router {
    Router::new()
        .route("/api/v1/accounts", get(list_accounts).post(create_account))
        .route(
            "/api/v1/accounts/{account_id}",
            get(get_account).put(update_account).delete(archive_account),
        )
        .route(
            "/api/v1/accounts/{account_id}/archive-impact",
            get(get_account_archive_impact),
        )
        .route(
            "/api/v1/accounts/{account_id}/lifecycle",
            put(set_account_lifecycle),
        )
        .route(
            "/api/v1/accounts/{account_id}/evaluation/restart",
            post(restart_evaluation),
        )
        .route(
            "/api/v1/accounts/{account_id}/opt-ins/{kind}",
            put(set_account_opt_in),
        )
        .with_state(state)
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    list_accounts,
    create_account,
    get_account,
    update_account,
    archive_account,
    get_account_archive_impact,
    set_account_lifecycle,
    restart_evaluation,
    set_account_opt_in,
))]
pub struct AccountsApi;

// ---- errors -------------------------------------------------------------------------------

fn error_body(status: StatusCode, code: &str, pointer: Option<String>) -> Response {
    let mut e = contracts::http::Error::new(code);
    e.error.pointer = pointer;
    (status, Json(e)).into_response()
}

fn invalid(pointer: Option<String>) -> Response {
    error_body(StatusCode::BAD_REQUEST, "common.invalid_request", pointer)
}

fn internal(detail: &str) -> Response {
    tracing::error!(error = %detail, "accounts: internal error");
    error_body(StatusCode::INTERNAL_SERVER_ERROR, "common.internal", None)
}

fn from_invalid(i: Invalid) -> Response {
    match i {
        Invalid::Malformed => invalid(None),
        Invalid::At(p) => invalid(Some(p)),
        Invalid::Unavailable => internal("accounts schema unavailable"),
    }
}

fn from_bad(b: BadField) -> Response {
    invalid(Some(b.0.to_owned()))
}

fn from_error(e: AccountsError) -> Response {
    match e {
        AccountsError::Access(a) => access::http::error_response(a),
        AccountsError::Internal(m) => internal(&m),
        e => {
            let status =
                StatusCode::from_u16(e.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let mut body = contracts::http::Error::new(e.code());
            body.error.pointer = e.pointer();
            if let AccountsError::RuleVersionChanged { current } = &e {
                body.error.current_rule_version_id = Some(current.as_uuid().to_string());
            }
            (status, Json(body)).into_response()
        }
    }
}

// ---- request parts ------------------------------------------------------------------------

/// Lowercase canonical UUID (`components.schemas.Uuid`).
fn uuid_param(raw: &str) -> Option<Uuid> {
    let ok = raw.len() == 36
        && raw
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b) || b == b'-');
    ok.then(|| Uuid::parse_str(raw).ok()).flatten()
}

/// Rejections are boxed: a `Response` is large (clippy `result_large_err`).
fn account_id(raw: &str) -> Result<AccountId, Box<Response>> {
    uuid_param(raw)
        .map(AccountId::from_uuid)
        .ok_or_else(|| Box::new(invalid(Some("/path/account_id".into()))))
}

fn parse_body<T: DeserializeOwned>(
    bytes: &[u8],
    schema: &'static str,
) -> Result<(T, serde_json::Value), Box<Response>> {
    let value = validate::body(bytes, schema).map_err(|i| Box::new(from_invalid(i)))?;
    let typed = serde_json::from_value(value.clone()).map_err(|e| {
        Box::new(internal(&format!(
            "schema-valid body not representable: {e}"
        )))
    })?;
    Ok((typed, value))
}

/// Client address for the step-up audit: the last `X-Forwarded-For` hop when Caddy sets it.
fn request_origin(
    headers: &HeaderMap,
    peer: Option<SocketAddr>,
    trust_forwarded_for: bool,
) -> RequestOrigin {
    let forwarded = trust_forwarded_for
        .then(|| {
            headers
                .get_all("x-forwarded-for")
                .iter()
                .filter_map(|v| v.to_str().ok())
                .flat_map(|v| v.split(','))
                .next_back()
                .and_then(|v| v.trim().parse::<IpAddr>().ok())
        })
        .flatten();
    RequestOrigin {
        ip: forwarded
            .or(peer.map(|p| p.ip()))
            .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
    }
}

fn origin_of(
    s: &AccountsHttpState,
    headers: &HeaderMap,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
) -> RequestOrigin {
    request_origin(headers, peer.map(|c| (c.0).0), s.trust_forwarded_for)
}

fn account_json(status: StatusCode, v: &accounts_application::views::AccountView) -> Response {
    (status, Json(AccountDto::from(v))).into_response()
}

// ---- handlers -----------------------------------------------------------------------------

#[utoipa::path(
    get,
    path = "/api/v1/accounts",
    operation_id = "listAccounts",
    tag = "accounts",
    responses(
        (status = 200, description = "The accounts.", body = AccountListSchema),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn list_accounts(State(s): St, Authenticated(caller): Authenticated) -> Response {
    match s.read.list(&caller).await {
        Ok(views) => Json(AccountListDto {
            accounts: views.iter().map(AccountDto::from).collect(),
        })
        .into_response(),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/accounts",
    operation_id = "createAccount",
    tag = "accounts",
    params(("Idempotency-Key" = contracts::http::Uuid, Header, description = "Once per create submission.")),
    request_body(content = AccountCreateRequestSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Replay of a create already done with this key.", body = AccountSchema),
        (status = 201, description = "Created and audited.", body = AccountSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Step-up failed or forbidden origin.", body = contracts::http::Error),
        (status = 409, description = "Conflict; nothing saved.", body = contracts::http::Error),
        (status = 422, description = "Refused by a rule; nothing saved.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn create_account(
    State(s): St,
    Authenticated(caller): Authenticated,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let Some(key) = headers
        .get("idempotency-key")
        .and_then(|v| v.to_str().ok())
        .and_then(uuid_param)
    else {
        return invalid(Some("/header/Idempotency-Key".into()));
    };
    let (typed, value) = match parse_body::<dto::CreateBody>(&body, "AccountCreateRequest") {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let cmd = match dto::create_command(typed, key, dto::request_hash(&value)) {
        Ok(c) => c,
        Err(b) => return from_bad(b),
    };
    let origin = origin_of(&s, &headers, peer);
    match s.create.handle(&caller, &origin, cmd).await {
        Ok(CreateOutcome::Created(v)) => account_json(StatusCode::CREATED, &v),
        Ok(CreateOutcome::Replayed(v)) => account_json(StatusCode::OK, &v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/accounts/{account_id}",
    operation_id = "getAccount",
    tag = "accounts",
    params(("account_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 200, description = "The account.", body = AccountSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_account(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw): Path<String>,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    match s.read.get(&caller, id).await {
        Ok(v) => account_json(StatusCode::OK, &v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/accounts/{account_id}",
    operation_id = "updateAccount",
    tag = "accounts",
    params(("account_id" = contracts::http::Uuid, Path)),
    request_body(content = AccountUpdateRequestSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Saved and audited.", body = AccountSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Step-up failed or forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Conflict; nothing saved.", body = contracts::http::Error),
        (status = 422, description = "Refused by a rule; nothing saved.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn update_account(
    State(s): St,
    Authenticated(caller): Authenticated,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    Path(raw): Path<String>,
    body: Bytes,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    let (typed, _) = match parse_body::<dto::UpdateBody>(&body, "AccountUpdateRequest") {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let cmd = match dto::update_command(id, typed) {
        Ok(c) => c,
        Err(b) => return from_bad(b),
    };
    let origin = origin_of(&s, &headers, peer);
    match s.update.handle(&caller, &origin, cmd).await {
        Ok(v) => account_json(StatusCode::OK, &v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/accounts/{account_id}",
    operation_id = "archiveAccount",
    tag = "accounts",
    params(("account_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 204, description = "Archived and audited."),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Open position; nothing changed.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn archive_account(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw): Path<String>,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    match s.archive.handle(&caller, id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/accounts/{account_id}/archive-impact",
    operation_id = "getAccountArchiveImpact",
    tag = "accounts",
    params(("account_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 200, description = "The impact.", body = ArchiveImpactSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn get_account_archive_impact(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw): Path<String>,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    match s.read.archive_impact(&caller, id).await {
        Ok(i) => Json(ArchiveImpactDto::from(&i)).into_response(),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/accounts/{account_id}/lifecycle",
    operation_id = "setAccountLifecycle",
    tag = "accounts",
    params(("account_id" = contracts::http::Uuid, Path)),
    request_body(content = LifecycleRequestSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "The account in its new state.", body = AccountSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Stale row_version or invalid transition.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn set_account_lifecycle(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw): Path<String>,
    body: Bytes,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    let (b, _) = match parse_body::<dto::LifecycleBody>(&body, "LifecycleRequest") {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let Some(state) = LifecycleState::parse(&b.state) else {
        return invalid(Some("/state".into()));
    };
    let cmd = SetLifecycle {
        account_id: id,
        state,
        row_version: b.row_version,
    };
    match s.lifecycle.handle(&caller, cmd).await {
        Ok(v) => account_json(StatusCode::OK, &v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/accounts/{account_id}/evaluation/restart",
    operation_id = "restartEvaluation",
    tag = "accounts",
    params(("account_id" = contracts::http::Uuid, Path)),
    request_body(content = EvaluationRestartRequestSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Restarted and audited.", body = AccountSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Stale row_version.", body = contracts::http::Error),
        (status = 422, description = "No evaluation target, or a start in the future.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn restart_evaluation(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw): Path<String>,
    body: Bytes,
) -> Response {
    let id = match account_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    let (b, _) = match parse_body::<dto::RestartBody>(&body, "EvaluationRestartRequest") {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let parsed = dto::date(&b.evaluation_start, "/evaluation_start").and_then(|start| {
        let earned = b
            .evaluation_earned_before
            .as_deref()
            .map(|m| dto::money(m, "/evaluation_earned_before"))
            .transpose()?;
        Ok((start, earned))
    });
    let (evaluation_start, evaluation_earned_before) = match parsed {
        Ok(p) => p,
        Err(bad) => return from_bad(bad),
    };
    let cmd = RestartEvaluation {
        account_id: id,
        evaluation_start,
        evaluation_earned_before,
        row_version: b.row_version,
    };
    match s.restart.handle(&caller, cmd).await {
        Ok(v) => account_json(StatusCode::OK, &v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/accounts/{account_id}/opt-ins/{kind}",
    operation_id = "setAccountOptIn",
    tag = "accounts",
    params(
        ("account_id" = contracts::http::Uuid, Path),
        ("kind" = OptInKindSchema, Path),
    ),
    request_body(content = OptInChoiceBodySchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Saved and audited.", body = OptInStateSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Step-up failed or forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "Stale rule version; nothing saved.", body = contracts::http::Error),
        (status = 422, description = "Not applicable.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn set_account_opt_in(
    State(s): St,
    Authenticated(caller): Authenticated,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    Path((raw_id, raw_kind)): Path<(String, String)>,
    body: Bytes,
) -> Response {
    let id = match account_id(&raw_id) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    let Some(kind) = OptInKind::parse(&raw_kind) else {
        return invalid(Some("/path/kind".into()));
    };
    let (b, _) = match parse_body::<dto::OptInBody>(&body, "OptInChoiceBody") {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let request = match dto::choice(
        &dto::ChoiceBody {
            choice: b.choice.clone(),
            rule_version_id: b.rule_version_id,
        },
        "/choice",
    ) {
        Ok(r) => r,
        Err(bad) => return from_bad(bad),
    };
    let cmd = SetOptIn {
        account_id: id,
        kind,
        request,
        step_up: b.step_up.map(|p| access::StepUpProof {
            challenge_id: p.challenge_id,
            code: p.code,
        }),
    };
    let origin = origin_of(&s, &headers, peer);
    match s.opt_in.handle(&caller, &origin, cmd).await {
        Ok(v) => Json(OptInStateDto::from(&v)).into_response(),
        Err(e) => from_error(e),
    }
}
