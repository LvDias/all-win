//! Router smoke tests that need no database (lazy pool).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

fn router() -> axum::Router {
    let db = PgPoolOptions::new()
        .connect_lazy("postgres://unused@127.0.0.1:1/unused")
        .unwrap();
    app::http::router(app::AppState { db })
}

#[tokio::test]
async fn healthz_answers_no_content() {
    let res = router()
        .oneshot(Request::get("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn unknown_paths_get_an_empty_404() {
    for uri in [
        "/nope",
        "/Ingress/v1/tradingview/awk_x",
        "/ingress/v1/x?t=1",
    ] {
        let res = router()
            .oneshot(Request::post(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND, "{uri}");
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty(), "{uri}");
    }
}
