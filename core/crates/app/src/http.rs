//! HTTP router. Spans and request logs use the matched route template
//! (`http.route`), or `<unmatched>` / `<unmatched-ingress>`; never the raw URI.

use axum::Router;
use axum::extract::{MatchedPath, Request, State};
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::get;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

use crate::AppState;

pub const UNMATCHED: &str = "<unmatched>";
pub const UNMATCHED_INGRESS: &str = "<unmatched-ingress>";

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/metrics", get(metrics))
        .with_state(state.clone())
        .merge(adapter_tradingview::router(state.ingress.clone()))
        .fallback(fallback)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(make_span)
                .on_request(())
                .on_response(on_response),
        )
        // ADR-0001 §9: every Core response.
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
}

async fn healthz() -> StatusCode {
    StatusCode::NO_CONTENT
}

/// Operational counters (no keys, users or tokens). Served on the Core bind only;
/// Caddy does not proxy it.
async fn metrics(State(state): State<AppState>) -> impl IntoResponse {
    let mut out = String::new();
    state.ingress.metrics.render(&mut out);
    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4")], out)
}

/// Every unmatched request gets an empty 404, so a near-miss ingress URL is
/// indistinguishable from an unknown token (S1 AC-2).
async fn fallback() -> StatusCode {
    StatusCode::NOT_FOUND
}

/// Ingress-like: path starts with `/ingress` compared case-insensitively after
/// percent-decoding and collapsing repeated `/` (s1-webhook-ingest §4.1).
pub fn is_ingress_like(path: &str) -> bool {
    let decoded = percent_decode_lossy(path);
    let trimmed = decoded.trim_start_matches('/');
    trimmed.len() >= 7 && trimmed.as_bytes()[..7].eq_ignore_ascii_case(b"ingress")
}

fn percent_decode_lossy(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let (Some(h), Some(l)) = (hex(bytes[i + 1]), hex(bytes[i + 2]))
        {
            out.push((h << 4) | l);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Route template for logs and spans.
pub fn route_label(request: &Request) -> &str {
    match request.extensions().get::<MatchedPath>() {
        // The ingress route answers any other method with the empty 404 (§4.2).
        Some(m)
            if m.as_str() == adapter_tradingview::INGRESS_ROUTE
                && request.method() != Method::POST =>
        {
            UNMATCHED_INGRESS
        }
        Some(matched) => matched.as_str(),
        None if is_ingress_like(request.uri().path()) => UNMATCHED_INGRESS,
        None => UNMATCHED,
    }
}

fn make_span(request: &Request) -> Span {
    tracing::info_span!(
        "http.request",
        http.method = %request.method(),
        http.route = route_label(request),
        http.status = tracing::field::Empty,
        client.ip = tracing::field::Empty,
        key_id = tracing::field::Empty,
    )
}

fn on_response<B>(response: &axum::http::Response<B>, latency: std::time::Duration, span: &Span) {
    span.record("http.status", response.status().as_u16());
    tracing::info!(latency_ms = latency.as_millis() as u64, "request");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingress_like_detection_follows_the_masking_rule() {
        for p in [
            "/ingress",
            "/ingress/v1/tradingview/x",
            "//ingress/v1",
            "/Ingress/v1",
            "/INGRESS/V1",
            "/%69ngress/v1",
            "/%49NGRESS",
        ] {
            assert!(is_ingress_like(p), "{p}");
        }
        for p in ["/api/v1/awk_x", "/", "/ingres", "/x/ingress", "/pt-BR/abc"] {
            assert!(!is_ingress_like(p), "{p}");
        }
    }
}
