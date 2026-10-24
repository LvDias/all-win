//! HTTP handlers of the route operations (contract `modules/routing.yaml`, HTTP 5.3.0; S7).
//! Precedence: 401 → 403 origin (extractor) → 400 (path, query, body schema) → 404 → step-up →
//! 409 → 422. Error bodies follow the contract `Error` schema.

pub mod dto;
pub mod schemas;
pub mod validate;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use access::http::{AccessState, Authenticated};
use access::{
    AccessError, Caller, RequestOrigin, SessionAuthenticator, StepUpAction, StepUpGate, StepUpProof,
};
use async_trait::async_trait;
use axum::Router;
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, put};
use axum::{Extension, Json};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use routing_application::enterprise::{AccountId, RouteId, SourceId};
use routing_application::routes::{
    CreateRoute, DeleteRoute, EnableAuthorizer, ListRouteCandidates, ListRoutes, RouteError,
    RouteFilter, RoutingDeps, StepUpDenied, UpdateRoute,
};

use dto::{BadField, RouteCandidatesDto, RouteDto, RouteListDto};
use schemas::*;
use validate::Invalid;

pub struct RoutesHttpState {
    list: ListRoutes,
    candidates: ListRouteCandidates,
    create: CreateRoute,
    update: UpdateRoute,
    delete: DeleteRoute,
    authenticator: Arc<dyn SessionAuthenticator>,
    step_up: Arc<dyn StepUpGate>,
    public_origin: String,
    trust_forwarded_for: bool,
}

impl RoutesHttpState {
    pub fn new(
        deps: RoutingDeps,
        authenticator: Arc<dyn SessionAuthenticator>,
        step_up: Arc<dyn StepUpGate>,
        public_origin: String,
        trust_forwarded_for: bool,
    ) -> Self {
        Self {
            list: ListRoutes::new(deps.clone()),
            candidates: ListRouteCandidates::new(deps.clone()),
            create: CreateRoute::new(deps.clone()),
            update: UpdateRoute::new(deps.clone()),
            delete: DeleteRoute::new(deps),
            authenticator,
            step_up,
            public_origin,
            trust_forwarded_for,
        }
    }
}

impl AccessState for RoutesHttpState {
    fn authenticator(&self) -> &Arc<dyn SessionAuthenticator> {
        &self.authenticator
    }

    fn public_origin(&self) -> &str {
        &self.public_origin
    }
}

type St = State<Arc<RoutesHttpState>>;

pub fn router(state: Arc<RoutesHttpState>) -> Router {
    Router::new()
        .route("/api/v1/routes", get(list_routes).post(create_route))
        .route(
            "/api/v1/routes/{route_id}",
            put(update_route).delete(delete_route),
        )
        .route("/api/v1/route-candidates", get(list_route_candidates))
        .with_state(state)
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    list_routes,
    create_route,
    update_route,
    delete_route,
    list_route_candidates,
))]
pub struct RoutesApi;

// ---- step-up ------------------------------------------------------------------------------

/// The request's `route_enable` step-up for the use cases: the proof of the body, checked and
/// consumed by the access port for the account the use case names.
struct RouteEnableStepUp<'a> {
    gate: &'a dyn StepUpGate,
    caller: &'a Caller,
    proof: Option<StepUpProof>,
    origin: &'a RequestOrigin,
}

#[async_trait]
impl EnableAuthorizer for RouteEnableStepUp<'_> {
    async fn authorize(&self, account: AccountId) -> Result<(), StepUpDenied> {
        let target = Some(account.as_uuid());
        let grant = self
            .gate
            .authorize(
                self.caller,
                StepUpAction::RouteEnable,
                target,
                self.proof.clone(),
                self.origin,
            )
            .await
            .map_err(denied)?;
        // Fail closed on a grant for anything else.
        if grant.covers(self.caller, StepUpAction::RouteEnable, target) {
            Ok(())
        } else {
            tracing::error!("routing: step-up grant does not cover route_enable");
            Err(StepUpDenied::Unavailable)
        }
    }
}

fn denied(e: AccessError) -> StepUpDenied {
    match e {
        AccessError::StepUpRequired => StepUpDenied::Required,
        AccessError::CodeInvalid => StepUpDenied::CodeInvalid,
        AccessError::RateLimited { retry_after_s } => StepUpDenied::RateLimited { retry_after_s },
        // The session was authenticated by the extractor; the gate cannot refuse it here.
        AccessError::Unauthenticated | AccessError::Unavailable => StepUpDenied::Unavailable,
    }
}

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
    tracing::error!(error = %detail, "routing: internal error");
    error_body(StatusCode::INTERNAL_SERVER_ERROR, "common.internal", None)
}

fn from_invalid(i: Invalid) -> Response {
    match i {
        Invalid::Malformed => invalid(None),
        Invalid::At(p) => invalid(Some(p)),
        Invalid::Unavailable => internal("routing schema unavailable"),
    }
}

fn from_bad(b: BadField) -> Response {
    invalid(Some(b.0))
}

/// Status and pointer of a refusal (step-up and port errors have their own responses).
fn refusal(e: &RouteError) -> (StatusCode, Option<&'static str>) {
    match e {
        RouteError::NotFound => (StatusCode::NOT_FOUND, None),
        RouteError::RouteExists
        | RouteError::VersionConflict
        | RouteError::SourceRevoked { on_create: false } => (StatusCode::CONFLICT, None),
        RouteError::SourceRevoked { on_create: true } | RouteError::SourceUnknown => {
            (StatusCode::UNPROCESSABLE_ENTITY, Some("/source_id"))
        }
        RouteError::AccountUnknown | RouteError::AccountManualOnly => {
            (StatusCode::UNPROCESSABLE_ENTITY, Some("/account_id"))
        }
        RouteError::NotEligible { pointer, .. } => {
            (StatusCode::UNPROCESSABLE_ENTITY, Some(pointer.as_str()))
        }
        RouteError::StepUp(_) | RouteError::Port(_) => (StatusCode::INTERNAL_SERVER_ERROR, None),
    }
}

fn from_step_up(d: StepUpDenied) -> Response {
    match d {
        StepUpDenied::Required => access::http::error_response(AccessError::StepUpRequired),
        StepUpDenied::CodeInvalid => access::http::error_response(AccessError::CodeInvalid),
        StepUpDenied::RateLimited { retry_after_s } => {
            access::http::error_response(AccessError::RateLimited { retry_after_s })
        }
        StepUpDenied::Unavailable => internal("step-up gate unavailable"),
    }
}

fn from_error(e: RouteError) -> Response {
    match e {
        RouteError::StepUp(d) => from_step_up(d),
        RouteError::Port(p) => internal(&p.to_string()),
        e => {
            let (status, pointer) = refusal(&e);
            error_body(status, &e.to_string(), pointer.map(str::to_owned))
        }
    }
}

// ---- request parts ------------------------------------------------------------------------

/// Rejections are boxed: a `Response` is large (clippy `result_large_err`).
type Rejection = Box<Response>;

/// Lowercase canonical UUID (`components.schemas.Uuid`).
fn uuid_param(raw: &str) -> Option<Uuid> {
    let ok = raw.len() == 36
        && raw
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b) || b == b'-');
    ok.then(|| Uuid::parse_str(raw).ok()).flatten()
}

fn route_id(raw: &str) -> Result<RouteId, Rejection> {
    uuid_param(raw)
        .map(RouteId::from_uuid)
        .ok_or_else(|| Box::new(invalid(Some("/path/route_id".into()))))
}

fn query(uri: &Uri) -> Result<HashMap<String, String>, Rejection> {
    Query::<HashMap<String, String>>::try_from_uri(uri)
        .map(|q| q.0)
        .map_err(|_| Box::new(invalid(None)))
}

/// An optional UUID query parameter; a present but non-canonical value is 400 at
/// `/query/<name>`.
fn query_uuid(q: &HashMap<String, String>, name: &'static str) -> Result<Option<Uuid>, Rejection> {
    q.get(name)
        .map(|raw| uuid_param(raw).ok_or_else(|| Box::new(invalid(Some(format!("/query/{name}"))))))
        .transpose()
}

fn parse_body<T: DeserializeOwned>(bytes: &[u8], schema: &'static str) -> Result<T, Rejection> {
    let value = validate::body(bytes, schema).map_err(|i| Box::new(from_invalid(i)))?;
    serde_json::from_value(value).map_err(|e| {
        Box::new(internal(&format!(
            "schema-valid body not representable: {e}"
        )))
    })
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
    s: &RoutesHttpState,
    headers: &HeaderMap,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
) -> RequestOrigin {
    request_origin(headers, peer.map(|c| (c.0).0), s.trust_forwarded_for)
}

fn route_json(status: StatusCode, v: &routing_application::routes::RouteView) -> Response {
    (status, Json(RouteDto::from(v))).into_response()
}

// ---- handlers -----------------------------------------------------------------------------

#[utoipa::path(
    get,
    path = "/api/v1/routes",
    operation_id = "listRoutes",
    tag = "routing",
    params(
        ("source_id" = Option<contracts::http::Uuid>, Query),
        ("account_id" = Option<contracts::http::Uuid>, Query),
    ),
    responses(
        (status = 200, description = "The routes, sorted by account `display_name`.", body = RouteListSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn list_routes(State(s): St, Authenticated(caller): Authenticated, uri: Uri) -> Response {
    let run = async {
        let q = query(&uri)?;
        let filter = RouteFilter {
            source_id: query_uuid(&q, "source_id")?.map(SourceId::from_uuid),
            account_id: query_uuid(&q, "account_id")?.map(AccountId::from_uuid),
        };
        let views = s
            .list
            .handle(caller.user_id, filter)
            .await
            .map_err(|e| Box::new(from_error(e)))?;
        Ok::<_, Rejection>(Json(RouteListDto::from(views.as_slice())).into_response())
    };
    run.await.unwrap_or_else(|r| *r)
}

#[utoipa::path(
    post,
    path = "/api/v1/routes",
    operation_id = "createRoute",
    tag = "routing",
    request_body(content = RouteCreateRequestSchema, content_type = "application/json"),
    responses(
        (status = 201, description = "Created and audited.", body = RouteSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Step-up failed or forbidden origin.", body = contracts::http::Error),
        (status = 409, description = "`routing.route_exists`; nothing created.", body = contracts::http::Error),
        (status = 422, description = "Refused by a rule; nothing created.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn create_route(
    State(s): St,
    Authenticated(caller): Authenticated,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let typed = match parse_body::<dto::CreateBody>(&body, "RouteCreateRequest") {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let (cmd, proof) = match dto::create_command(typed) {
        Ok(c) => c,
        Err(b) => return from_bad(b),
    };
    let origin = origin_of(&s, &headers, peer);
    let step_up = RouteEnableStepUp {
        gate: s.step_up.as_ref(),
        caller: &caller,
        proof,
        origin: &origin,
    };
    match s.create.handle(caller.user_id, cmd, &step_up).await {
        Ok(v) => route_json(StatusCode::CREATED, &v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/routes/{route_id}",
    operation_id = "updateRoute",
    tag = "routing",
    params(("route_id" = contracts::http::Uuid, Path)),
    request_body(content = RouteUpdateRequestSchema, content_type = "application/json"),
    responses(
        (status = 200, description = "Saved (a request that changes nothing is not audited).", body = RouteSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Step-up failed or forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "`routing.source_revoked` or `common.version_conflict`; nothing changed.", body = contracts::http::Error),
        (status = 422, description = "Switching on a route whose account is not eligible (pointer `/is_enabled`).", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn update_route(
    State(s): St,
    Authenticated(caller): Authenticated,
    peer: Option<Extension<ConnectInfo<SocketAddr>>>,
    headers: HeaderMap,
    Path(raw): Path<String>,
    body: Bytes,
) -> Response {
    let id = match route_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    let typed = match parse_body::<dto::UpdateBody>(&body, "RouteUpdateRequest") {
        Ok(b) => b,
        Err(r) => return *r,
    };
    let (cmd, proof) = match dto::update_command(typed) {
        Ok(c) => c,
        Err(b) => return from_bad(b),
    };
    let origin = origin_of(&s, &headers, peer);
    let step_up = RouteEnableStepUp {
        gate: s.step_up.as_ref(),
        caller: &caller,
        proof,
        origin: &origin,
    };
    match s.update.handle(caller.user_id, id, cmd, &step_up).await {
        Ok(v) => route_json(StatusCode::OK, &v),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/routes/{route_id}",
    operation_id = "deleteRoute",
    tag = "routing",
    params(("route_id" = contracts::http::Uuid, Path)),
    responses(
        (status = 204, description = "Removed and audited."),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 403, description = "Forbidden origin.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 409, description = "`routing.source_revoked`; nothing removed.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn delete_route(
    State(s): St,
    Authenticated(caller): Authenticated,
    Path(raw): Path<String>,
) -> Response {
    let id = match route_id(&raw) {
        Ok(id) => id,
        Err(r) => return *r,
    };
    match s.delete.handle(caller.user_id, id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => from_error(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/route-candidates",
    operation_id = "listRouteCandidates",
    tag = "routing",
    params(("source_id" = contracts::http::Uuid, Query)),
    responses(
        (status = 200, description = "The candidates.", body = RouteCandidatesSchema),
        (status = 400, description = "Invalid request.", body = contracts::http::Error),
        (status = 401, description = "Unauthenticated.", body = contracts::http::Error),
        (status = 404, description = "Not found.", body = contracts::http::Error),
        (status = 500, description = "Internal error.", body = contracts::http::Error),
    ),
)]
async fn list_route_candidates(
    State(s): St,
    Authenticated(caller): Authenticated,
    uri: Uri,
) -> Response {
    let run = async {
        let q = query(&uri)?;
        let source_id = query_uuid(&q, "source_id")?
            .ok_or_else(|| Box::new(invalid(Some("/query/source_id".into()))))?;
        let candidates = s
            .candidates
            .handle(caller.user_id, SourceId::from_uuid(source_id))
            .await
            .map_err(|e| Box::new(from_error(e)))?;
        Ok::<_, Rejection>(Json(RouteCandidatesDto::from(&candidates)).into_response())
    };
    run.await.unwrap_or_else(|r| *r)
}
