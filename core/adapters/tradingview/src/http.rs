//! `POST /ingress/v1/tradingview/{token}` (OpenAPI `ingressTradingView`).
//!
//! Order (s1-webhook-ingest §2, HTTP 1.1.2): per-IP flood cap → token from the raw
//! path (before the body is read; every failure charged to the IP's token-failure
//! budget) → per-key limit → content type and size → JSON → credential guard →
//! schema → source policy → idempotent insert. Nothing after `/ingress` is ever logged;
//! the span gets `key_id` only for a canonical token.

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Instant;

use axum::Router;
use axum::body::Body;
use axum::extract::{ConnectInfo, Request, State};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use chrono::NaiveDate;
use contracts::http::{IngressAccepted, IngressAcceptedStatus, IngressRejected};
use contracts::instruction::v1::{self, Instruction, ParseError};
use routing_application::enterprise::{
    IngressEvent, IngressEventKind, InstructionId, IntakeTimes, MessageId, RejectionCode,
};
use routing_application::{
    IngestInstruction, IngestOutcome, NewInstruction, RecordIngressRejection,
};
use serde_json::Value;
use shared_kernel::Clock;
use tracing::Span;

use crate::ip::{Cidr, IpKey};
use crate::metrics::{IngressMetrics, Outcome};
use crate::negative_cache::UnknownKeyCache;
use crate::ports::{InstrumentSupport, ResolvedSourceKey, SourceKeyResolver};
use crate::rate_limit::KeyedRateLimiter;
use crate::token::IngressToken;
use crate::{SOURCE_TYPE, policy};

/// Route template; the only route under `/ingress`.
pub const INGRESS_ROUTE: &str = "/ingress/v1/tradingview/{token}";
const PATH_PREFIX: &str = "/ingress/v1/tradingview/";
/// Body limit (contract: 16 KiB).
pub const MAX_BODY_BYTES: usize = 16 * 1024;
/// Highest instruction minor this build accepts (`ingress.unsupported_version` above it).
pub const SUPPORTED_MINOR: u64 = 0;

/// Per-IP limits (s1-webhook-ingest §2 steps 1–2).
pub struct IpLimits {
    /// Step 1, every request: `ingress.ip_flood_per_min`.
    pub flood: KeyedRateLimiter<IpKey>,
    /// Step 1 for `tradingview_source_ranges`: `ingress.tradingview_flood_per_min`.
    pub tradingview_flood: KeyedRateLimiter<IpKey>,
    /// TradingView's published webhook addresses (configuration, never request data).
    pub tradingview_ranges: Vec<Cidr>,
    /// Step 2, failures only: `ingress.ip_token_failures_per_min`.
    pub token_failures: KeyedRateLimiter<IpKey>,
}

pub struct IngressState {
    pub resolver: Arc<dyn SourceKeyResolver>,
    pub instruments: Arc<dyn InstrumentSupport>,
    pub ingest: IngestInstruction,
    pub reject: RecordIngressRejection,
    pub ip_limits: IpLimits,
    pub key_limiter: KeyedRateLimiter<String>,
    /// Canonical `key_id`s not found in the database (never revoked or inactive ones).
    pub unknown_keys: UnknownKeyCache,
    pub metrics: Arc<IngressMetrics>,
    pub clock: Arc<dyn Clock>,
    /// Take the client IP from the last `X-Forwarded-For` entry (set by Caddy).
    pub trust_forwarded_for: bool,
}

/// OpenAPI of this adapter; `app` merges it into the Core document.
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(ingress),
    components(schemas(contracts::http::RetryAfterSeconds))
)]
pub struct IngressApi;

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Path)]
#[allow(dead_code)]
struct IngressPathParams {
    /// `awk_<key_id>_<secret>`; any invalid value gets the empty 404.
    #[param(max_length = 128)]
    token: String,
}

pub fn router(state: Arc<IngressState>) -> Router {
    Router::new()
        .route(INGRESS_ROUTE, any(ingress))
        .with_state(state)
}

/// Any other ingress-like request (other case, double slash, extra segments): the same
/// flood cap and failure budget as a bad token, then the empty 404 (or the identical 429).
pub fn unmatched(state: &IngressState, request: &Request) -> Response {
    let ip = client_ip(request, state.trust_forwarded_for);
    let (outcome, response) = flood_check(state, ip).unwrap_or_else(|| token_failure(state, ip));
    state.metrics.record(outcome);
    response
}

/// Step 1: `Some(429)` when the IP is over its flood cap.
fn flood_check(s: &IngressState, ip: Option<IpAddr>) -> Option<(Outcome, Response)> {
    let ip = ip?;
    Span::current().record("client.ip", tracing::field::display(ip));
    let limits = &s.ip_limits;
    let limiter = if limits.tradingview_ranges.iter().any(|r| r.contains(ip)) {
        &limits.tradingview_flood
    } else {
        &limits.flood
    };
    limiter
        .check(&IpKey::from(ip), Instant::now())
        .err()
        .map(|retry| (Outcome::RateLimitedIp, rate_limited(retry)))
}

/// Charges a token failure to the IP: the empty 404 while the budget lasts, then a 429
/// identical for every failure kind (it never reveals whether a key exists).
fn token_failure(s: &IngressState, ip: Option<IpAddr>) -> (Outcome, Response) {
    let charged = ip.map(|ip| {
        s.ip_limits
            .token_failures
            .check(&IpKey::from(ip), Instant::now())
    });
    match charged {
        Some(Err(retry)) => (Outcome::RateLimitedFailures, rate_limited(retry)),
        _ => (Outcome::NotFound, not_found()),
    }
}

/// Authenticated request context.
struct Caller {
    user_id: shared_kernel::UserId,
    source_id: routing_application::enterprise::SourceId,
    key_id: String,
}

/// Receive a TradingView alert (V1-S1).
#[utoipa::path(
    post,
    path = "/ingress/v1/tradingview/{token}",
    operation_id = "ingressTradingView",
    tag = "ingress",
    security(()),
    params(IngressPathParams),
    request_body(
        content(
            (contracts::http::InstructionV1 = "application/json"),
            (contracts::http::PlainInstruction = "text/plain"),
        ),
    ),
    responses(
        (status = 200, description = "Accepted, or duplicate.", body = contracts::http::IngressAccepted),
        (status = 400, description = "Invalid JSON, schema violation or unsupported version.", body = contracts::http::IngressRejected),
        (status = 404, description = "Unknown, revoked or malformed token; any other path or method. Empty body."),
        (status = 413, description = "Payload too large.", body = contracts::http::IngressRejected),
        (status = 415, description = "Unsupported media type.", body = contracts::http::IngressRejected),
        (status = 422, description = "Source policy or catalog.", body = contracts::http::IngressRejected),
        (status = 429, description = "Rate limited.", body = contracts::http::IngressRejected,
            headers(("Retry-After" = contracts::http::RetryAfterSeconds, description = "Seconds to wait."))),
        (status = 500, description = "Internal error.", body = contracts::http::IngressRejected),
    ),
)]
async fn ingress(State(s): State<Arc<IngressState>>, request: Request) -> Response {
    let received_at = s.clock.now();
    let outcome = handle(&s, request, received_at).await;
    s.metrics.record(outcome.0);
    outcome.1
}

async fn handle(
    s: &IngressState,
    request: Request,
    received_at: chrono::DateTime<chrono::Utc>,
) -> (Outcome, Response) {
    let ip = client_ip(&request, s.trust_forwarded_for);
    if let Some(limited) = flood_check(s, ip) {
        return limited;
    }
    if request.method() != Method::POST {
        return token_failure(s, ip);
    }
    // The raw (still percent-encoded) path: an encoded or altered token never matches.
    let Some(token) = request
        .uri()
        .path()
        .strip_prefix(PATH_PREFIX)
        .and_then(IngressToken::parse)
    else {
        return token_failure(s, ip);
    };
    Span::current().record("key_id", token.key_id());

    if s.unknown_keys.is_unknown(token.key_id(), Instant::now()) {
        return token_failure(s, ip);
    }
    let key = match s.resolver.resolve(token.key_id()).await {
        Ok(Some(key)) if authenticates(&token, &key) => key,
        // Revoked, inactive or mismatched: never cached, always looked up again.
        Ok(Some(_)) => return token_failure(s, ip),
        Ok(None) => {
            s.unknown_keys
                .remember_unknown(token.key_id(), Instant::now());
            return token_failure(s, ip);
        }
        Err(e) => return internal(&e),
    };
    let caller = Caller {
        user_id: key.user_id,
        source_id: key.source_id,
        key_id: token.key_id().to_owned(),
    };
    drop(token);

    if let Err(retry) = s.key_limiter.check(&caller.key_id, Instant::now()) {
        let rejection = Rejection {
            code: RejectionCode::RateLimited,
            pointer: None,
            message_id: None,
        };
        audit_rejection(s, &caller, received_at, rejection, None).await;
        return (Outcome::RateLimitedKey, rate_limited(retry));
    }

    // Body size for the audit entry: declared first, then the bytes actually read.
    let mut size_bytes = declared_length(request.headers());
    match accept(s, &caller, request, received_at, &mut size_bytes).await {
        Ok(outcome) => {
            let (status, label) = match outcome {
                IngestOutcome::Accepted(_) => (IngressAcceptedStatus::Accepted, Outcome::Accepted),
                IngestOutcome::Duplicate(_) => {
                    (IngressAcceptedStatus::Duplicate, Outcome::Duplicate)
                }
            };
            let body = IngressAccepted {
                status,
                instruction_id: outcome.instruction_id().to_string(),
            };
            (label, (StatusCode::OK, axum::Json(body)).into_response())
        }
        Err(Failure::Rejected(rejection)) => {
            let response = rejected(
                status_of(rejection.code),
                rejection.code,
                rejection.pointer.as_deref(),
            );
            audit_rejection(s, &caller, received_at, rejection, size_bytes).await;
            (Outcome::Rejected, response)
        }
        Err(Failure::Internal(e)) => internal(&e),
    }
}

fn authenticates(token: &IngressToken, key: &ResolvedSourceKey) -> bool {
    // Evaluate the hash comparison first so a revoked key costs the same as a live one.
    let matches = token.matches(&key.secret_hash);
    matches && !key.revoked && key.source_type == SOURCE_TYPE
}

struct Rejection {
    code: RejectionCode,
    pointer: Option<String>,
    message_id: Option<MessageId>,
}

enum Failure {
    Rejected(Rejection),
    Internal(String),
}

fn reject(code: RejectionCode, pointer: Option<&str>) -> Failure {
    Failure::Rejected(Rejection {
        code,
        pointer: pointer.map(str::to_owned),
        message_id: None,
    })
}

async fn accept(
    s: &IngressState,
    caller: &Caller,
    request: Request,
    received_at: chrono::DateTime<chrono::Utc>,
    size_bytes: &mut Option<u64>,
) -> Result<IngestOutcome, Failure> {
    if !allowed_content_type(request.headers()) {
        return Err(reject(RejectionCode::UnsupportedMediaType, None));
    }
    if declared_length(request.headers()).is_some_and(|n| n > MAX_BODY_BYTES as u64) {
        return Err(reject(RejectionCode::PayloadTooLarge, None));
    }
    let bytes = axum::body::to_bytes(request.into_body(), MAX_BODY_BYTES)
        .await
        .map_err(|_| reject(RejectionCode::PayloadTooLarge, None))?;
    *size_bytes = Some(bytes.len() as u64);
    let text = std::str::from_utf8(&bytes).map_err(|_| reject(RejectionCode::InvalidJson, None))?;
    let value: Value =
        serde_json::from_str(text).map_err(|_| reject(RejectionCode::InvalidJson, None))?;
    // Defense in depth: a token pasted into the body is never stored (ALIGN-3).
    if contains_token(text) {
        return Err(reject(RejectionCode::SchemaViolation, Some("")));
    }
    if let Some((major, minor)) = v1::peek_schema_version(&value)
        && (major != 1 || minor > SUPPORTED_MINOR)
    {
        return Err(reject(
            RejectionCode::UnsupportedVersion,
            Some("/schema_version"),
        ));
    }
    let instruction = match v1::parse_value(value.clone()) {
        Ok(i) => i,
        Err(ParseError::Schema { pointers }) => {
            let pointer = pointers.into_iter().next().unwrap_or_default();
            return Err(reject(RejectionCode::SchemaViolation, Some(&pointer)));
        }
        Err(ParseError::Malformed) => return Err(reject(RejectionCode::InvalidJson, None)),
        // Schema-valid but not a real value, e.g. `sent_at` on 30 February.
        Err(ParseError::Unrepresentable(_)) => {
            return Err(reject(RejectionCode::SchemaViolation, Some("")));
        }
        Err(e @ ParseError::SchemaUnavailable(_)) => return Err(Failure::Internal(e.to_string())),
    };
    let message_id = MessageId::parse(instruction.message_id.clone())
        .map_err(|_| reject(RejectionCode::SchemaViolation, Some("/message_id")))?;
    let with_message = |code, pointer: &str| {
        Failure::Rejected(Rejection {
            code,
            pointer: Some(pointer.to_owned()),
            message_id: Some(message_id.clone()),
        })
    };
    policy::check(&instruction).map_err(|(code, pointer)| with_message(code, pointer))?;
    let enabled = s
        .instruments
        .is_enabled(&instruction.instrument.symbol)
        .await
        .map_err(|e| Failure::Internal(e.to_string()))?;
    if !enabled {
        return Err(with_message(
            RejectionCode::UnsupportedInstrument,
            "/instrument/symbol",
        ));
    }

    let new = new_instruction(caller, &instruction, message_id, value, received_at);
    s.ingest
        .handle(new)
        .await
        .map_err(|e| Failure::Internal(e.to_string()))
}

fn new_instruction(
    caller: &Caller,
    i: &Instruction,
    message_id: MessageId,
    payload: Value,
    received_at: chrono::DateTime<chrono::Utc>,
) -> NewInstruction {
    NewInstruction {
        id: InstructionId::new_v7(),
        user_id: caller.user_id,
        source_id: caller.source_id,
        source_type: i.source.kind.clone(),
        source_key_id: Some(caller.key_id.clone()),
        message_id,
        schema_version: i.schema_version.to_string(),
        signal_type: wire_name(&i.signal_type),
        action: wire_name(&i.action),
        instrument_symbol: i.instrument.symbol.clone(),
        contract_month: i
            .instrument
            .contract_month
            .as_deref()
            .and_then(first_day_of),
        quantity: i.quantity.and_then(|q| i32::try_from(q).ok()),
        position_ref: i.refs.as_ref().and_then(|r| r.position_ref.clone()),
        order_ref: i.refs.as_ref().and_then(|r| r.order_ref.clone()),
        times: IntakeTimes {
            sent_at: i.sent_at,
            received_at,
        },
        payload,
    }
}

/// snake_case wire name of a contract enum.
fn wire_name<T: serde::Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(Value::String(s)) => s,
        _ => String::new(),
    }
}

/// `YYYY-MM` → first day of that month.
fn first_day_of(month: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(&format!("{month}-01"), "%Y-%m-%d").ok()
}

async fn audit_rejection(
    s: &IngressState,
    caller: &Caller,
    at: chrono::DateTime<chrono::Utc>,
    rejection: Rejection,
    size_bytes: Option<u64>,
) {
    let event = IngressEvent {
        user_id: caller.user_id,
        at,
        source_id: caller.source_id,
        key_id: caller.key_id.clone(),
        kind: IngressEventKind::Rejected {
            code: rejection.code,
            pointer: rejection.pointer,
            message_id: rejection.message_id,
            size_bytes,
        },
    };
    if let Err(e) = s.reject.handle(event).await {
        tracing::error!(error = %e, "ingress rejection not audited");
    }
}

fn status_of(code: RejectionCode) -> StatusCode {
    match code {
        RejectionCode::RateLimited => StatusCode::TOO_MANY_REQUESTS,
        RejectionCode::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        RejectionCode::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
        RejectionCode::InvalidJson
        | RejectionCode::SchemaViolation
        | RejectionCode::UnsupportedVersion => StatusCode::BAD_REQUEST,
        RejectionCode::SourceMismatch
        | RejectionCode::ActionNotAllowedForSource
        | RejectionCode::OrderTypeNotAllowedForSource
        | RejectionCode::SignalTypeNotAllowedForSource
        | RejectionCode::UnsupportedInstrument => StatusCode::UNPROCESSABLE_ENTITY,
    }
}

fn rejected(status: StatusCode, code: RejectionCode, pointer: Option<&str>) -> Response {
    let body = IngressRejected::new(code.as_str(), pointer);
    (status, axum::Json(body)).into_response()
}

fn rate_limited(retry_after_secs: u64) -> Response {
    let mut response = rejected(
        StatusCode::TOO_MANY_REQUESTS,
        RejectionCode::RateLimited,
        None,
    );
    response.headers_mut().insert(
        header::RETRY_AFTER,
        HeaderValue::from(retry_after_secs.max(1)),
    );
    response
}

/// Unknown, revoked or malformed token, any other method: an empty 404 (S1 AC-2).
pub fn not_found() -> Response {
    (StatusCode::NOT_FOUND, Body::empty()).into_response()
}

fn internal(error: &dyn std::fmt::Display) -> (Outcome, Response) {
    tracing::error!(error = %error, "ingress internal error");
    let body = IngressRejected::new("common.internal", None);
    (
        Outcome::Internal,
        (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(body)).into_response(),
    )
}

fn allowed_content_type(headers: &HeaderMap) -> bool {
    let Some(value) = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
    else {
        return false;
    };
    let essence = value.split(';').next().unwrap_or_default().trim();
    essence.eq_ignore_ascii_case("application/json") || essence.eq_ignore_ascii_case("text/plain")
}

fn declared_length(headers: &HeaderMap) -> Option<u64> {
    headers
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
}

/// Client address: the last `X-Forwarded-For` hop when the proxy is trusted,
/// otherwise the TCP peer.
fn client_ip(request: &Request, trust_forwarded_for: bool) -> Option<IpAddr> {
    if trust_forwarded_for
        && let Some(ip) = request
            .headers()
            .get_all("x-forwarded-for")
            .iter()
            .filter_map(|v| v.to_str().ok())
            .flat_map(|v| v.split(','))
            .next_back()
            .and_then(|v| v.trim().parse().ok())
    {
        return Some(ip);
    }
    request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|c| c.0.ip())
}

/// `aw[kd]_<12>_<43>` anywhere in the text (the database CHECKs use the same pattern).
fn contains_token(text: &str) -> bool {
    let b = text.as_bytes();
    let is_secret = |c: u8| c.is_ascii_alphanumeric() || c == b'_' || c == b'-';
    (0..b.len()).any(|i| {
        b[i..].len() >= 60
            && b[i] == b'a'
            && b[i + 1] == b'w'
            && matches!(b[i + 2], b'k' | b'd')
            && b[i + 3] == b'_'
            && b[i + 4..i + 16].iter().all(u8::is_ascii_alphanumeric)
            && b[i + 16] == b'_'
            && b[i + 17..i + 60].iter().all(|c| is_secret(*c))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_pattern_detection_matches_the_database_check() {
        let secret = "A".repeat(43);
        assert!(contains_token(&format!(
            r#"{{"x":"awk_AbCdEf123456_{secret}"}}"#
        )));
        assert!(contains_token(&format!("awd_AbCdEf123456_{secret}")));
        assert!(!contains_token(r#"{"secret":"awk_k1a2b3_c2VjcmV0"}"#));
        assert!(!contains_token(""));
    }

    #[test]
    fn content_types() {
        let mut h = HeaderMap::new();
        assert!(!allowed_content_type(&h));
        for (v, ok) in [
            ("application/json", true),
            ("application/json; charset=utf-8", true),
            ("Text/Plain", true),
            ("application/xml", false),
        ] {
            h.insert(header::CONTENT_TYPE, HeaderValue::from_static(v));
            assert_eq!(allowed_content_type(&h), ok, "{v}");
        }
    }

    #[test]
    fn contract_month_maps_to_first_day() {
        assert_eq!(
            first_day_of("2026-12"),
            NaiveDate::from_ymd_opt(2026, 12, 1)
        );
    }
}
