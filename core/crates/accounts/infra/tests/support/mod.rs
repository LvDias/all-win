//! PostgreSQL 16 harness (same rules as `crates/app/tests/support`): `TEST_DATABASE_ADMIN_URL`
//! (env, else the git-ignored `core/.env.test`) creates the test roles and one database per test
//! binary; migrations run as `allwin_owner`; the code under test uses `allwin_app` (RLS on).
//! Without a URL the tests are skipped, unless `ALLWIN_REQUIRE_DB_TESTS=1`.

#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use std::sync::OnceLock;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../../migrations");

const ROLES: [(&str, &str, &str); 5] = [
    ("allwin_owner", "owner-test-only", "NOBYPASSRLS"),
    ("allwin_app", "app-test-only", "NOBYPASSRLS"),
    ("allwin_worker", "worker-test-only", "BYPASSRLS"),
    ("allwin_maint", "maint-test-only", "BYPASSRLS"),
    ("allwin_publisher", "publisher-test-only", "NOBYPASSRLS"),
];

pub struct TestDb {
    pub app: PgPool,
    /// Superuser on the test database: seeds rows of other modules (identity users) and
    /// inspects every row regardless of RLS.
    pub admin: PgPool,
}

static DATABASE: OnceLock<Option<(String, String)>> = OnceLock::new();

fn admin_url() -> Option<String> {
    if let Ok(url) = std::env::var("TEST_DATABASE_ADMIN_URL") {
        return Some(url);
    }
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../.env.test");
    let text = std::fs::read_to_string(file).ok()?;
    text.lines()
        .find_map(|l| l.strip_prefix("TEST_DATABASE_ADMIN_URL="))
        .map(|v| v.trim().trim_matches('"').to_owned())
}

/// `postgres://user:pw@host/db` with other credentials and database.
fn with(admin: &str, credentials: Option<(&str, &str)>, db: &str) -> String {
    let rest = admin.split_once("://").map_or(admin, |(_, r)| r);
    let (user, rest) = rest.rsplit_once('@').unwrap_or(("", rest));
    let host = rest.split_once('/').map_or(rest, |(h, _)| h);
    let user = credentials.map_or(user.to_owned(), |(u, p)| format!("{u}:{p}"));
    format!("postgres://{user}@{host}/{db}")
}

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
    let pool = |url: String, max: u32| async move {
        PgPoolOptions::new()
            .max_connections(max)
            .acquire_timeout(std::time::Duration::from_secs(120))
            .connect(&url)
            .await
            .unwrap()
    };
    Some(TestDb {
        app: pool(with(admin, Some(("allwin_app", "app-test-only")), name), 8).await,
        admin: pool(with(admin, None, name), 2).await,
    })
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
            assert!(e.to_string().contains("already exists"), "{e}");
        }
    }
    let name = format!("allwin_test_{}", Uuid::now_v7().simple());
    conn.execute(sqlx::AssertSqlSafe(format!(
        "CREATE DATABASE {name} OWNER allwin_owner"
    )))
    .await
    .unwrap();
    let owner = PgPoolOptions::new()
        .max_connections(1)
        .connect(&with(
            &admin,
            Some(("allwin_owner", "owner-test-only")),
            &name,
        ))
        .await
        .unwrap();
    MIGRATOR
        .run(&owner)
        .await
        .expect("migrations as allwin_owner");
    owner.close().await;
    Some((admin, name))
}

/// A user row (identity owns the table; the superuser seeds it for these tests). Works before and
/// after the passwordless migration (`…060800`: `password_hash` dropped, `email_verified_at NOT
/// NULL` added), whichever this branch carries.
pub async fn user(db: &TestDb) -> Uuid {
    let id = Uuid::now_v7();
    let email = format!("{}@example.invalid", id.simple());
    let passwordless: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM information_schema.columns
           WHERE table_schema = 'identity' AND table_name = 'app_user'
             AND column_name = 'email_verified_at')",
    )
    .fetch_one(&db.admin)
    .await
    .unwrap();
    let sql = if passwordless {
        "INSERT INTO identity.app_user (id, email, email_normalized, email_verified_at, time_zone)
         VALUES ($1, $2, $2, now(), 'America/Sao_Paulo')"
    } else {
        "INSERT INTO identity.app_user (id, email, email_normalized, password_hash, time_zone)
         VALUES ($1, $2, $2, 'unusable', 'America/Sao_Paulo')"
    };
    sqlx::query(sql)
        .bind(id)
        .bind(email)
        .execute(&db.admin)
        .await
        .unwrap();
    id
}
