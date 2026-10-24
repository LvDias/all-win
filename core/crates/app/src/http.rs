//! HTTP router. Spans and request logs use the matched route template
//! (`http.route`), or `<unmatched>` / `<unmatched-ingress>`; never the raw URI.

use axum::Router;
use axum::extract::{MatchedPath, Request, State};
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

use crate::AppState;

pub const UNMATCHED: &str = "<unmatched>";
pub const UNMATCHED_INGRESS: &str = "<unmatched-ingress>";

pub fn router(state: AppState) -> Router {
    let ingress = adapter_tradingview::router(state.ingress.clone());
    let identity = state.identity.clone().unwrap_or_default();
    Router::new()
        .route("/healthz", get(healthz))
        .route("/metrics", get(metrics))
        .fallback(fallback)
        .with_state(state)
        .merge(ingress)
        .merge(identity)
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
    // Dead-lettered: an audit entry missing from a user's log (X-5). Unconsumed: events of
    // a type with no registered consumer, never fetched (Architect decision (A)). Alert on > 0.
    // Allowlisted types (crate::unconsumed) are not unconsumed: no consumer yet, by decision.
    // Dead letters count both the legacy #1b rows and the per-consumer ones (#14). When a new
    // backfill consumer is registered, its type's retained rows become unpublished again until it
    // catches up: an old-unpublished alert during that catch-up is expected (Esmeril note).
    let mut consumed: Vec<String> = state.consumed_event_types.to_vec();
    consumed.extend(crate::unconsumed::types(crate::unconsumed::UNCONSUMED));
    match sqlx::query!(
        r#"SELECT (SELECT count(*) FROM app.outbox WHERE dead_lettered_at IS NOT NULL)
                + (SELECT count(*) FROM app.outbox_delivery WHERE dead_lettered_at IS NOT NULL)
                  AS "dead_lettered!",
                  (SELECT count(*) FROM app.outbox
                    WHERE published_at IS NULL AND dead_lettered_at IS NULL
                      AND NOT (event_type = ANY($1))) AS "unconsumed!",
                  pg_notification_queue_usage() AS "notify_queue_usage!""#,
        &consumed
    )
    .fetch_one(&state.worker)
    .await
    {
        Ok(row) => {
            out.push_str("# TYPE allwin_outbox_dead_lettered gauge\n");
            out.push_str(&format!(
                "allwin_outbox_dead_lettered {}\n",
                row.dead_lettered
            ));
            out.push_str("# TYPE allwin_outbox_unconsumed gauge\n");
            out.push_str(&format!("allwin_outbox_unconsumed {}\n", row.unconsumed));
            // Alert above 0.5: a listener stuck in a transaction lets the queue fill, and then
            // the producers' commits fail (#14).
            out.push_str("# TYPE allwin_outbox_notify_queue_usage gauge\n");
            out.push_str(&format!(
                "allwin_outbox_notify_queue_usage {}\n",
                row.notify_queue_usage
            ));
        }
        Err(e) => tracing::error!(error = %e, "metrics: outbox counts failed"),
    }
    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4")], out)
}

/// Every unmatched request gets an empty 404. Ingress-like ones go through the ingress
/// flood cap and token-failure budget, so a near-miss URL is indistinguishable from an
/// unknown token (S1 AC-2, HTTP 1.1.2 step 2).
async fn fallback(State(state): State<AppState>, request: Request) -> Response {
    if is_ingress_like(request.uri().path()) {
        return adapter_tradingview::unmatched(&state.ingress, &request);
    }
    StatusCode::NOT_FOUND.into_response()
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
