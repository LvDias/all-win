//! Shared PostgreSQL harness of the routing-infra tests: one database per test binary,
//! migrated as `allwin_owner`; pools as `allwin_owner` and `allwin_app` (RLS on).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr,
    dead_code
)]

use shared_kernel::UserId;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};

const PASSWORDS: [(&str, &str); 2] = [
    ("allwin_owner", "owner-test-only"),
    ("allwin_app", "app-test-only"),
];

fn admin_url() -> Option<String> {
    if let Ok(url) = std::env::var("TEST_DATABASE_ADMIN_URL") {
        return Some(url);
    }
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../.env.test");
    std::fs::read_to_string(file)
        .ok()?
        .lines()
        .find_map(|l| l.strip_prefix("TEST_DATABASE_ADMIN_URL="))
        .map(|v| v.trim().trim_matches('"').to_owned())
}

fn url_as(admin: &str, role: &str, db: &str) -> String {
    let password = PASSWORDS.iter().find(|(r, _)| *r == role).unwrap().1;
    let rest = admin.split_once("://").map_or(admin, |(_, r)| r);
    let host = rest.rsplit_once('@').map_or(rest, |(_, h)| h);
    let host = host.split_once('/').map_or(host, |(h, _)| h);
    format!("postgres://{role}:{password}@{host}/{db}")
}

/// `(admin url, database name)`, created and migrated once per test binary on its own
/// runtime (role DDL must not run concurrently); pools are per test.
static DATABASE: std::sync::OnceLock<Option<(String, String)>> = std::sync::OnceLock::new();

async fn create_database() -> Option<(String, String)> {
    let Some(admin) = admin_url() else {
        assert!(
            std::env::var("ALLWIN_REQUIRE_DB_TESTS").as_deref() != Ok("1"),
            "TEST_DATABASE_ADMIN_URL is required"
        );
        eprintln!("SKIPPED: TEST_DATABASE_ADMIN_URL not set");
        return None;
    };
    let mut conn = PgConnection::connect(&admin).await.unwrap();
    for (role, password) in PASSWORDS {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = $1)")
                .bind(role)
                .fetch_one(&mut conn)
                .await
                .unwrap();
        // Roles are cluster-wide and shared with other test binaries: create, never alter.
        if !exists {
            let sql = format!("CREATE ROLE {role} WITH LOGIN PASSWORD '{password}' NOBYPASSRLS");
            if let Err(e) = conn.execute(sqlx::AssertSqlSafe(sql)).await {
                assert!(e.to_string().contains("already exists"), "{e}");
            }
        }
    }
    // Other roles named by migrations (grants) must exist too.
    for role in ["allwin_worker", "allwin_maint"] {
        let sql = format!(
            "DO $$ BEGIN CREATE ROLE {role}; EXCEPTION WHEN duplicate_object OR unique_violation THEN NULL; END $$"
        );
        conn.execute(sqlx::AssertSqlSafe(sql)).await.unwrap();
    }
    let name = format!("allwin_test_{}", uuid::Uuid::now_v7().simple());
    conn.execute(sqlx::AssertSqlSafe(format!(
        "CREATE DATABASE {name} OWNER allwin_owner"
    )))
    .await
    .unwrap();
    let owner = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url_as(&admin, "allwin_owner", &name))
        .await
        .unwrap();
    sqlx::migrate!("../../../migrations")
        .run(&owner)
        .await
        .expect("migrations as allwin_owner");
    owner.close().await;
    Some((admin, name))
}

/// `(owner pool, allwin_app pool)` on the per-binary database, or `None` when skipped.
pub async fn app_pool() -> Option<(PgPool, PgPool)> {
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
    let pool = |role: &str, max: u32| {
        let url = url_as(admin, role, name);
        async move {
            PgPoolOptions::new()
                .max_connections(max)
                .acquire_timeout(std::time::Duration::from_secs(120))
                .connect(&url)
                .await
                .unwrap()
        }
    };
    Some((pool("allwin_owner", 2).await, pool("allwin_app", 8).await))
}

/// Inserts a user row (the owner is subject to forced RLS, so the user context is set).
pub async fn user(owner: &PgPool) -> UserId {
    let id = UserId::new_v7();
    let mut tx = owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(id.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    // Fill whichever of these columns the identity schema in force has (S3's passwordless
    // 060800 drops password_hash and makes email_verified_at NOT NULL; F-W-1).
    let columns: Vec<String> = sqlx::query_scalar(
        "SELECT column_name::text FROM information_schema.columns
          WHERE table_schema = 'identity' AND table_name = 'app_user'
            AND column_name IN ('password_hash', 'email_verified_at')",
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();
    let has = |c: &str| columns.iter().any(|x| x == c);
    let mut names = String::from("id, email, email_normalized, time_zone");
    let mut values = String::from("$1, $2, $2, 'UTC'");
    if has("password_hash") {
        names.push_str(", password_hash");
        values.push_str(", 'x'");
    }
    if has("email_verified_at") {
        names.push_str(", email_verified_at");
        values.push_str(", now()");
    }
    let sql = format!("INSERT INTO identity.app_user ({names}) VALUES ({values})");
    sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(id.as_uuid())
        .bind(format!("{}@test.invalid", id.as_uuid()))
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    id
}

/// Outbox event types of `user`, oldest first (read as the owner within the user context).
pub async fn events(owner: &PgPool, user: UserId) -> Vec<(String, serde_json::Value)> {
    let mut tx = owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query_as("SELECT event_type, payload FROM app.outbox WHERE user_id = $1 ORDER BY seq")
        .bind(user.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .unwrap()
}
