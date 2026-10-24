//! Shared PostgreSQL setup for the gateway infra tests (see `pg_store.rs`).
#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

pub mod server;

use std::path::PathBuf;

use shared_kernel::UserId;
use sqlx::postgres::PgPoolOptions;
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;

const ROLES: [(&str, &str, &str); 4] = [
    ("allwin_owner", "owner-test-only", "NOBYPASSRLS"),
    ("allwin_app", "app-test-only", "NOBYPASSRLS"),
    ("allwin_worker", "worker-test-only", "BYPASSRLS"),
    ("allwin_maint", "maint-test-only", "BYPASSRLS"),
];

fn with_db(url: &str, db: &str) -> String {
    let (base, query) = url
        .split_once('?')
        .map_or((url, None), |(b, q)| (b, Some(q)));
    let cut = base.rfind('/').unwrap();
    let mut out = format!("{}/{db}", &base[..cut]);
    if let Some(q) = query {
        out.push('?');
        out.push_str(q);
    }
    out
}

fn as_role(admin: &str, role: &str, db: &str) -> String {
    let password = ROLES
        .iter()
        .find(|(r, _, _)| *r == role)
        .map(|(_, p, _)| *p)
        .unwrap();
    let url = with_db(admin, db);
    let (scheme, rest) = url.split_once("://").unwrap();
    let host = rest.rsplit_once('@').map_or(rest, |(_, h)| h);
    format!("{scheme}://{role}:{password}@{host}")
}

pub struct Db {
    pub admin_url: String,
    pub name: String,
    pub app: PgPool,
    pub worker: PgPool,
    pub inspect: PgPool,
}

pub async fn setup() -> Option<Db> {
    let Ok(admin_url) = std::env::var("TEST_DATABASE_ADMIN_URL") else {
        assert!(
            std::env::var("ALLWIN_REQUIRE_DB_TESTS").as_deref() != Ok("1"),
            "TEST_DATABASE_ADMIN_URL is required"
        );
        eprintln!("skipped: TEST_DATABASE_ADMIN_URL not set");
        return None;
    };
    let name = format!("allwin_gw_t_{}", Uuid::now_v7().simple());
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_url)
        .await
        .unwrap();
    sqlx::query("SELECT pg_advisory_lock(7320301)")
        .execute(&admin)
        .await
        .unwrap();
    for (role, password, rls) in ROLES {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = $1)")
                .bind(role)
                .fetch_one(&admin)
                .await
                .unwrap();
        let verb = if exists { "ALTER" } else { "CREATE" };
        // Constants only, never input.
        let sql = format!("{verb} ROLE {role} WITH LOGIN PASSWORD '{password}' {rls}");
        if let Err(e) = sqlx::raw_sql(AssertSqlSafe(sql)).execute(&admin).await {
            assert!(e.to_string().contains("already exists"), "{e}");
        }
    }
    sqlx::query("SELECT pg_advisory_unlock(7320301)")
        .execute(&admin)
        .await
        .unwrap();
    // `name` comes from a UUID, never from input.
    sqlx::raw_sql(AssertSqlSafe(format!(
        "CREATE DATABASE {name} OWNER allwin_owner"
    )))
    .execute(&admin)
    .await
    .unwrap();
    admin.close().await;
    let owner = PgPoolOptions::new()
        .max_connections(1)
        .connect(&as_role(&admin_url, "allwin_owner", &name))
        .await
        .unwrap();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../migrations");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "sql"))
        .collect();
    files.sort();
    for f in files {
        let sql = std::fs::read_to_string(&f).unwrap();
        let mut tx = owner.begin().await.unwrap();
        sqlx::raw_sql(AssertSqlSafe(sql))
            .execute(&mut *tx)
            .await
            .unwrap_or_else(|e| panic!("{}: {e}", f.display()));
        tx.commit().await.unwrap();
    }
    owner.close().await;
    let app = PgPoolOptions::new()
        .max_connections(4)
        .connect(&as_role(&admin_url, "allwin_app", &name))
        .await
        .unwrap();
    let worker = PgPoolOptions::new()
        .max_connections(1)
        .connect(&as_role(&admin_url, "allwin_worker", &name))
        .await
        .unwrap();
    let inspect = PgPoolOptions::new()
        .max_connections(1)
        .connect(&with_db(&admin_url, &name))
        .await
        .unwrap();
    Some(Db {
        admin_url,
        name,
        app,
        worker,
        inspect,
    })
}

impl Db {
    pub async fn teardown(self) {
        self.app.close().await;
        self.worker.close().await;
        self.inspect.close().await;
        let admin = PgPoolOptions::new()
            .max_connections(1)
            .connect(&self.admin_url)
            .await
            .unwrap();
        sqlx::raw_sql(AssertSqlSafe(format!(
            "DROP DATABASE {} WITH (FORCE)",
            self.name
        )))
        .execute(&admin)
        .await
        .unwrap();
    }

    pub async fn user(&self) -> UserId {
        let id = Uuid::now_v7();
        sqlx::query(
            "INSERT INTO identity.app_user (id, email, email_normalized, time_zone, email_verified_at)
             VALUES ($1, $2, $2, 'America/Sao_Paulo', now())",
        )
        .bind(id)
        .bind(format!("{}@example.test", id.simple()))
        .execute(&self.inspect)
        .await
        .unwrap();
        UserId::from_uuid(id)
    }
}
