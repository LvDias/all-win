//! The module routes wired behind the session authenticator (identity, accounts S5, profiles S6,
//! opt-in rules, instruments) are mounted with identity and answer 401 without a session; without
//! `IDENTITY_PEPPERS` none of them is mounted. No database needed: no session cookie means no
//! lookup.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::extract::ConnectInfo;
use axum::http::Request;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

const PEPPER: &str = "1:AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA"; // test-only, 32 bytes

const ROUTES: [&str; 7] = [
    "/api/v1/me",
    "/api/v1/accounts",
    "/api/v1/default-profiles",
    "/api/v1/profile-sizes/nearest?size_amount=50000",
    "/api/v1/instruments",
    "/api/v1/opt-in-rules",
    "/api/v1/accounts/0190d3c4-0000-7000-8000-000000000001/profiles",
];

fn app(with_identity: bool) -> axum::Router {
    let config = app::config::Config::from_lookup(move |k| match k {
        "DATABASE_URL" => Some("postgres://unused".into()),
        "APP_ENV" => Some("test".into()),
        "IDENTITY_PEPPERS" if with_identity => Some(PEPPER.into()),
        "APP_PUBLIC_ORIGIN" if with_identity => Some("https://app.test".into()),
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
    )
    .unwrap();
    app::http::router(wired.state)
}

async fn get(router: &axum::Router, uri: &str) -> (u16, String) {
    let mut request = Request::get(uri).body(Body::empty()).unwrap();
    request
        .extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([198, 51, 100, 9], 5000))));
    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status().as_u16();
    let body = to_bytes(response.into_body(), 1 << 16).await.unwrap();
    (status, String::from_utf8_lossy(&body).into_owned())
}

#[tokio::test]
async fn module_routes_need_a_session() {
    let router = app(true);
    for uri in ROUTES {
        let (status, body) = get(&router, uri).await;
        assert_eq!(status, 401, "{uri}: {body}");
        assert!(body.contains("auth.unauthenticated"), "{uri}: {body}");
    }
}

#[tokio::test]
async fn without_identity_no_module_route_is_mounted() {
    let router = app(false);
    for uri in ROUTES {
        let (status, _) = get(&router, uri).await;
        assert_eq!(status, 404, "{uri}");
    }
}
