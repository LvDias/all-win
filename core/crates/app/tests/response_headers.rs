//! Global response conventions (ADR-0001 §9): every Core response, including fallbacks
//! and error paths, carries `X-Content-Type-Options: nosniff` and `Cache-Control: no-store`.
//! These are not OpenAPI drift items (Architect, 2026-10-23). No database needed: the pool
//! points at a closed port, which also drives the 500 path.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Method, Request};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

const CANONICAL_SHAPED: &str = "awk_AbCdEf123456_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn app(ip_flood_per_min: u32) -> axum::Router {
    let config = app::config::Config::from_lookup(|k| match k {
        "DATABASE_URL" => Some("postgres://unused".into()),
        "INGRESS_IP_FLOOD_PER_MIN" => Some(ip_flood_per_min.to_string()),
        _ => None,
    })
    .unwrap();
    let pool = || {
        PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(300))
            .connect_lazy("postgres://nobody:none@127.0.0.1:1/none")
            .unwrap()
    };
    let wired = app::App::wire(
        &config,
        pool(),
        pool(),
        Arc::new(shared_kernel::SystemClock),
    );
    app::http::router(wired.unwrap().state)
}

async fn call(router: &axum::Router, method: Method, uri: &str) -> (u16, String, String) {
    let mut request = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from("{}"))
        .unwrap();
    request
        .extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([198, 51, 100, 9], 5000))));
    let response = router.clone().oneshot(request).await.unwrap();
    let header = |name: &str| {
        response
            .headers()
            .get(name)
            .map(|v| v.to_str().unwrap().to_owned())
            .unwrap_or_default()
    };
    (
        response.status().as_u16(),
        header("x-content-type-options"),
        header("cache-control"),
    )
}

#[tokio::test]
async fn every_route_and_fallback_carries_nosniff_and_no_store() {
    let router = app(1000);
    let ingress = format!("/ingress/v1/tradingview/{CANONICAL_SHAPED}");
    let cases = [
        (Method::GET, "/healthz".to_owned(), 204),
        (Method::POST, "/healthz".to_owned(), 405),
        (Method::GET, "/metrics".to_owned(), 200),
        (Method::GET, "/nope".to_owned(), 404),
        (Method::GET, "/Ingress/v1/x".to_owned(), 404),
        (Method::GET, ingress.clone(), 404),
        (
            Method::POST,
            "/ingress/v1/tradingview/malformed".to_owned(),
            404,
        ),
        // Canonical-shaped token, database unreachable: the 500 path.
        (Method::POST, ingress, 500),
    ];
    for (method, uri, expected) in cases {
        let (status, nosniff, cache) = call(&router, method.clone(), &uri).await;
        assert_eq!(status, expected, "{method} {uri}");
        assert_eq!(nosniff, "nosniff", "{method} {uri}");
        assert_eq!(cache, "no-store", "{method} {uri}");
    }
}

#[tokio::test]
async fn rate_limited_responses_carry_the_conventions_too() {
    let router = app(1);
    call(&router, Method::POST, "/ingress/v1/tradingview/x").await;
    let (status, nosniff, cache) = call(&router, Method::POST, "/ingress/v1/tradingview/x").await;
    assert_eq!(
        (status, nosniff.as_str(), cache.as_str()),
        (429, "nosniff", "no-store")
    );
}
