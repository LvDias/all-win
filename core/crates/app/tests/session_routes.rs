//! The module routes wired behind the session authenticator (identity, accounts S5, profiles S6,
//! opt-in rules, instruments) are mounted with identity and answer 401 without a session; without
//! `IDENTITY_PEPPERS` none of them is mounted. Every handler of the wired module routers, taken
//! from their own OpenAPI documents (so a new handler is covered as soon as it is annotated),
//! checks the session first and then, on writes, `Origin` (Esmeril F2, Guarita I1). No database
//! needed: requests stop at the guards.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::extract::ConnectInfo;
use axum::http::{Method, Request, header};
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

const STUB_COOKIE: &str = "stub-session";

/// Accepts exactly [`STUB_COOKIE`]; anything else is unauthenticated.
struct StubSessions;

#[async_trait::async_trait]
impl access::SessionAuthenticator for StubSessions {
    async fn authenticate(
        &self,
        cookie: Option<&str>,
        _: access::Activity,
    ) -> Result<access::Caller, access::AccessError> {
        match cookie {
            Some(STUB_COOKIE) => Ok(access::Caller {
                user_id: shared_kernel::UserId::new_v7(),
                session_id: uuid::Uuid::now_v7(),
                device_id: uuid::Uuid::now_v7(),
                is_platform_admin: false,
            }),
            _ => Err(access::AccessError::Unauthenticated),
        }
    }
}

/// Never reached: requests stop at the session and Origin guards.
struct NoStepUp;

#[async_trait::async_trait]
impl access::StepUpGate for NoStepUp {
    async fn authorize(
        &self,
        _: &access::Caller,
        _: access::StepUpAction,
        _: Option<uuid::Uuid>,
        _: Option<access::StepUpProof>,
        _: &access::RequestOrigin,
    ) -> Result<access::StepUpGrant, access::AccessError> {
        Err(access::AccessError::Unauthenticated)
    }
}

/// Every (method, path) of the module routers `app::accounts::wire` mounts, from their OpenAPI
/// documents, with the path parameters filled in.
fn wired_operations() -> Vec<(Method, String)> {
    use utoipa::OpenApi;
    let mut doc = accounts_infra::http::AccountsApi::openapi();
    doc.merge(accounts_infra::profiles::http::ProfilesApi::openapi());
    doc.merge(catalog_infra::rule_version_http::RuleVersionApi::openapi());
    doc.merge(catalog_infra::instrument_http::InstrumentsApi::openapi());
    let fill = |path: &str| {
        path.replace("{kind}", "automation")
            .replace("{size_amount}", "50000")
            .split('/')
            .map(|seg| {
                if seg.starts_with('{') {
                    "0190d3c4-0000-7000-8000-000000000001"
                } else {
                    seg
                }
            })
            .collect::<Vec<_>>()
            .join("/")
    };
    let mut ops = Vec::new();
    for (path, item) in &doc.paths.paths {
        for (method, op) in [
            (Method::GET, &item.get),
            (Method::POST, &item.post),
            (Method::PUT, &item.put),
            (Method::PATCH, &item.patch),
            (Method::DELETE, &item.delete),
        ] {
            if op.is_some() {
                ops.push((method, fill(path)));
            }
        }
    }
    ops
}

#[tokio::test]
async fn every_wired_handler_checks_the_session_then_origin() {
    let lazy = || {
        PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(300))
            .connect_lazy("postgres://nobody:none@127.0.0.1:1/none")
            .unwrap()
    };
    let db = lazy();
    let mut consumers = app::outbox::Consumers::default();
    let audit: Arc<dyn app::outbox::EventConsumer> = Arc::new(app::outbox::AuditConsumer(
        audit_application::RecordAuditEntry::new(Arc::new(audit_infra::PgAuditStore::new(
            db.clone(),
        ))),
    ));
    let wired = app::accounts::wire(
        &db,
        app::accounts::store(&db),
        Arc::new(shared_kernel::SystemClock),
        app::accounts::SessionPorts {
            authenticator: Arc::new(StubSessions),
            step_up: Arc::new(NoStepUp),
            identity_store: Arc::new(identity_infra::store::PgIdentityStore::new(
                db.clone(),
                lazy(),
            )),
        },
        "https://app.test".into(),
        false,
        false,
        audit,
        &mut consumers,
    );
    let ops = wired_operations();
    assert_eq!(ops.len(), 19, "{ops:?}");
    let send = |method: Method, uri: String, cookie: Option<&str>, origin: &str| {
        let mut b = Request::builder()
            .method(method)
            .uri(uri)
            .header(header::ORIGIN, origin)
            .header(header::CONTENT_TYPE, "application/json");
        if let Some(c) = cookie {
            b = b.header(header::COOKIE, format!("__Host-aw_session={c}"));
        }
        let mut request = b.body(Body::from("{}")).unwrap();
        request
            .extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([198, 51, 100, 9], 5000))));
        let router = wired.router.clone();
        async move {
            let response = router.oneshot(request).await.unwrap();
            let status = response.status().as_u16();
            let body = to_bytes(response.into_body(), 1 << 16).await.unwrap();
            (status, String::from_utf8_lossy(&body).into_owned())
        }
    };
    for (method, uri) in ops {
        let (status, body) = send(method.clone(), uri.clone(), None, "https://app.test").await;
        assert_eq!(
            (status, body.contains("auth.unauthenticated")),
            (401, true),
            "{method} {uri} without a session: {body}"
        );
        if method != Method::GET {
            let (status, body) = send(
                method.clone(),
                uri.clone(),
                Some(STUB_COOKIE),
                "https://evil.test",
            )
            .await;
            assert_eq!(
                (status, body.contains("common.forbidden_origin")),
                (403, true),
                "{method} {uri} with a foreign Origin: {body}"
            );
        }
    }
}
