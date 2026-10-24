//! Integration-test harness against a real PostgreSQL 16.
//!
//! `TEST_DATABASE_ADMIN_URL` (superuser; read from the environment, else from the
//! git-ignored `core/.env.test`) is used only to create test roles and one database per
//! test binary. Migrations run connected as `allwin_owner` (never a superuser, so default
//! privileges apply); handlers use `allwin_app` (RLS enforced) and the dispatcher
//! `allwin_worker`. Without a URL the tests are skipped, unless
//! `ALLWIN_REQUIRE_DB_TESTS=1` (CI), where a missing URL fails.

#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use std::net::SocketAddr;
use std::sync::Arc;

use app::config::Config;
use app::{App, AppState};
use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Request, Response};
use http_body_util::BodyExt;
use shared_kernel::SystemClock;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::OnceLock;
use tower::ServiceExt;

const ROLES: [(&str, &str, &str); 4] = [
    ("allwin_owner", "owner-test-only", "NOBYPASSRLS"),
    ("allwin_app", "app-test-only", "NOBYPASSRLS"),
    ("allwin_worker", "worker-test-only", "BYPASSRLS"),
    ("allwin_maint", "maint-test-only", "BYPASSRLS"),
];

pub struct TestDb {
    pub owner: PgPool,
    pub app: PgPool,
    pub worker: PgPool,
    pub maint: PgPool,
}

/// `(admin url, database name)`, created and migrated once per test binary on its own
/// runtime; pools are per test because each `#[tokio::test]` has its own runtime.
static DATABASE: OnceLock<Option<(String, String)>> = OnceLock::new();

fn admin_url() -> Option<String> {
    if let Ok(url) = std::env::var("TEST_DATABASE_ADMIN_URL") {
        return Some(url);
    }
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.env.test");
    let text = std::fs::read_to_string(file).ok()?;
    text.lines()
        .find_map(|l| l.strip_prefix("TEST_DATABASE_ADMIN_URL="))
        .map(|v| v.trim().trim_matches('"').to_owned())
}

fn with_credentials(admin: &str, user: &str, password: &str, db: &str) -> String {
    let mut url = url_parts(admin);
    url.user = format!("{user}:{password}");
    url.db = db.to_owned();
    format!("postgres://{}@{}/{}", url.user, url.host, url.db)
}

struct UrlParts {
    user: String,
    host: String,
    db: String,
}

fn url_parts(url: &str) -> UrlParts {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let (user, rest) = rest.rsplit_once('@').unwrap_or(("", rest));
    let (host, db) = rest.split_once('/').unwrap_or((rest, "postgres"));
    UrlParts {
        user: user.to_owned(),
        host: host.to_owned(),
        db: db.split('?').next().unwrap_or("postgres").to_owned(),
    }
}

/// Pools on the per-binary database, or `None` when DB tests are skipped.
pub async fn db() -> Option<TestDb> {
    let (admin, name) = DATABASE
        .get_or_init(|| {
            std::thread::spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(create_database())
            })
            .join()
            .unwrap()
        })
        .as_ref()?;
    Some(pools(admin, name).await)
}

/// Pools on a database of its own, for tests that register their own outbox consumers (the
/// registration refuses consumers another test left in a shared database).
pub async fn fresh_db() -> Option<TestDb> {
    let (admin, name) = create_database().await?;
    Some(pools(&admin, &name).await)
}

async fn pools(admin: &str, name: &str) -> TestDb {
    let pool = |user: &str, password: &str, max: u32| {
        let url = with_credentials(admin, user, password, name);
        async move {
            PgPoolOptions::new()
                .max_connections(max)
                // Generous: a loaded CI host must not turn queueing into PoolTimedOut.
                .acquire_timeout(std::time::Duration::from_secs(120))
                .connect(&url)
                .await
                .unwrap()
        }
    };
    TestDb {
        owner: pool("allwin_owner", "owner-test-only", 2).await,
        app: pool("allwin_app", "app-test-only", 8).await,
        worker: pool("allwin_worker", "worker-test-only", 2).await,
        maint: pool("allwin_maint", "maint-test-only", 1).await,
    }
}

async fn create_database() -> Option<(String, String)> {
    let Some(admin) = admin_url() else {
        assert!(
            std::env::var("ALLWIN_REQUIRE_DB_TESTS").as_deref() != Ok("1"),
            "TEST_DATABASE_ADMIN_URL is required"
        );
        eprintln!("SKIPPED: TEST_DATABASE_ADMIN_URL not set");
        return None;
    };
    let mut conn = PgConnection::connect(&admin)
        .await
        .expect("admin connection");
    // Roles are cluster-wide and shared by concurrent test binaries and databases: one writer at a
    // time (the identity harness takes the same lock), else "tuple concurrently updated".
    sqlx::query("SELECT pg_advisory_lock(7320301)")
        .execute(&mut conn)
        .await
        .unwrap();
    for (role, password, rls) in ROLES {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = $1)")
                .bind(role)
                .fetch_one(&mut conn)
                .await
                .unwrap();
        let verb = if exists { "ALTER" } else { "CREATE" };
        let sql = format!("{verb} ROLE {role} WITH LOGIN PASSWORD '{password}' {rls}");
        if let Err(e) = conn.execute(sqlx::AssertSqlSafe(sql)).await {
            // A concurrent binary created it first.
            assert!(e.to_string().contains("already exists"), "{e}");
        }
    }
    sqlx::query("SELECT pg_advisory_unlock(7320301)")
        .execute(&mut conn)
        .await
        .unwrap();
    let name = format!("allwin_test_{}", uuid::Uuid::now_v7().simple());
    conn.execute(sqlx::AssertSqlSafe(format!(
        "CREATE DATABASE {name} OWNER allwin_owner"
    )))
    .await
    .unwrap();
    let owner = PgPoolOptions::new()
        .max_connections(1)
        .connect(&with_credentials(
            &admin,
            "allwin_owner",
            "owner-test-only",
            &name,
        ))
        .await
        .unwrap();
    app::MIGRATOR
        .run(&owner)
        .await
        .expect("migrations as allwin_owner");
    owner.close().await;
    Some((admin, name))
}

pub fn config() -> Config {
    Config::from_lookup(|k| match k {
        "DATABASE_URL" => Some("postgres://unused".into()),
        "INGRESS_RATE_PER_KEY_PER_MIN" => Some("1000".into()),
        "INGRESS_IP_FLOOD_PER_MIN" => Some("100000".into()),
        "INGRESS_IP_TOKEN_FAILURES_PER_MIN" => Some("100000".into()),
        _ => None,
    })
    .unwrap()
}

pub fn wire(db: &TestDb, config: &Config) -> App {
    App::wire(
        config,
        db.app.clone(),
        db.worker.clone(),
        Arc::new(SystemClock),
    )
    .unwrap()
}

pub struct Seeded {
    pub token: String,
    pub key_id: String,
    pub secret: String,
    pub user_id: uuid::Uuid,
}

pub async fn seed(db: &TestDb) -> Seeded {
    let label = format!("t{}", uuid::Uuid::now_v7().simple());
    let token = app::dev::seed(&db.owner, &label[..32]).await.unwrap();
    let key_id = token[4..16].to_owned();
    let secret = token[17..].to_owned();
    let user_id = sqlx::query_scalar("SELECT user_id FROM identity.source_key WHERE key_id = $1")
        .bind(&key_id)
        .fetch_one(&db.owner)
        .await
        .unwrap();
    Seeded {
        token,
        key_id,
        secret,
        user_id,
    }
}

pub fn ingress_path(token: &str) -> String {
    format!("/ingress/v1/tradingview/{token}")
}

pub async fn send(
    state: &AppState,
    request: Request<Body>,
) -> (u16, axum::http::HeaderMap, Vec<u8>) {
    let mut request = request;
    request
        .extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([203, 0, 113, 7], 40000))));
    let response: Response<Body> = app::http::router(state.clone())
        .oneshot(request)
        .await
        .unwrap();
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let body = response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    (status, headers, body)
}

pub fn post_json(uri: &str, body: impl Into<String>) -> Request<Body> {
    Request::post(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.into()))
        .unwrap()
}

pub fn instruction(message_id: &str) -> String {
    format!(
        r#"{{"schema_version":"1.0","message_id":"{message_id}","sent_at":"2026-10-22T16:03:20Z",
            "source":{{"type":"tradingview"}},"signal_type":"trigger","action":"buy",
            "instrument":{{"symbol":"MNQ"}},"quantity":1}}"#
    )
}

pub fn unique_message_id() -> String {
    format!("tv-MNQ-test-{}", uuid::Uuid::now_v7().simple())
}

/// Runs `f` inside a transaction scoped to `user` as `allwin_app` (RLS on).
pub async fn as_user<T>(
    db: &TestDb,
    user: uuid::Uuid,
    f: impl for<'c> FnOnce(
        &'c mut PgConnection,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'c>>,
) -> T {
    let mut tx = db.app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let out = f(&mut tx).await;
    tx.rollback().await.unwrap();
    out
}

/// Delivers the outbox until `user` has no undelivered event. Other tests' dispatchers may hold
/// a consumer's lock, so "no row seen" alone is not "delivered".
pub async fn drain(app: &App, db: &TestDb, user: uuid::Uuid) {
    app.dispatcher.register().await.unwrap();
    for _ in 0..200 {
        app.dispatcher.dispatch_all().await.unwrap();
        let pending: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM app.outbox WHERE user_id = $1 AND published_at IS NULL AND dead_lettered_at IS NULL",
        )
        .bind(user)
        .fetch_one(&db.worker)
        .await
        .unwrap();
        if pending == 0 {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
    }
    panic!("outbox not drained for {user}");
}
