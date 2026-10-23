//! `PgRouteStore` against PostgreSQL 16: natural-key races, optimistic concurrency, outbox
//! events in the same transaction, and per-user isolation (RLS as `allwin_app`).
//!
//! Needs `TEST_DATABASE_ADMIN_URL` (superuser; environment or the git-ignored `core/.env.test`)
//! to create one database per run; migrations run as `allwin_owner`. Skipped without it,
//! unless `ALLWIN_REQUIRE_DB_TESTS=1`.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use chrono::{TimeZone, Utc};
use routing_application::enterprise::{AccountId, RouteId, RouteSignalType, SignalTypes, SourceId};
use routing_application::routes::{
    ChangeAction, ChangeCause, CreateOutcome, RouteChange, RouteFilter, RouteRecord, RouteStore,
    RouteUpdate, UpdateOutcome,
};
use routing_infra::PgRouteStore;
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
async fn app_pool() -> Option<(PgPool, PgPool)> {
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
async fn user(owner: &PgPool) -> UserId {
    let id = UserId::new_v7();
    let mut tx = owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(id.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    // Any NOT NULL credential column of the identity schema in force gets a placeholder.
    let has_password: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM information_schema.columns
           WHERE table_schema = 'identity' AND table_name = 'app_user' AND column_name = 'password_hash')",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    let sql = if has_password {
        "INSERT INTO identity.app_user (id, email, email_normalized, password_hash, time_zone)
         VALUES ($1, $2, $2, 'x', 'UTC')"
    } else {
        "INSERT INTO identity.app_user (id, email, email_normalized, email_verified_at, time_zone)
         VALUES ($1, $2, $2, now(), 'UTC')"
    };
    sqlx::query(sql)
        .bind(id.as_uuid())
        .bind(format!("{}@test.invalid", id.as_uuid()))
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    id
}

fn types(t: &[RouteSignalType]) -> SignalTypes {
    SignalTypes::new(t.iter().copied()).unwrap()
}

fn route(user: UserId, source: SourceId, account: AccountId) -> (RouteRecord, RouteChange) {
    let at = Utc.with_ymd_and_hms(2026, 10, 23, 12, 0, 0).unwrap();
    let r = RouteRecord {
        id: RouteId::new_v7(),
        user_id: user,
        source_id: source,
        account_id: account,
        signal_types: types(&[RouteSignalType::Trigger, RouteSignalType::NqTrigger]),
        is_enabled: true,
        row_version: 1,
        created_at: at,
        updated_at: at,
    };
    let c = RouteChange {
        route_id: r.id,
        source_id: source,
        account_id: account,
        action: ChangeAction::Created,
        cause: ChangeCause::User,
        previous: None,
        new: Some((r.signal_types.clone(), true)),
        at,
    };
    (r, c)
}

/// Outbox event types of `user`, oldest first (read as the owner within the user context).
async fn events(owner: &PgPool, user: UserId) -> Vec<(String, serde_json::Value)> {
    let mut tx = owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query_as("SELECT event_type, payload FROM app.outbox WHERE user_id = $1 ORDER BY id")
        .bind(user.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .unwrap()
}

#[tokio::test]
async fn route_lifecycle_with_outbox_events_and_isolation() {
    let Some((owner, app)) = app_pool().await else {
        return;
    };
    let store = PgRouteStore::new(app.clone());
    let (a, b) = (user(&owner).await, user(&owner).await);
    let (source, account) = (SourceId::new_v7(), AccountId::new_v7());
    let (r, c) = route(a, source, account);

    assert_eq!(store.create(&r, &c).await.unwrap(), CreateOutcome::Created);
    assert_eq!(store.get(a, r.id).await.unwrap(), Some(r.clone()));
    let listed = store
        .list(
            a,
            RouteFilter {
                source_id: Some(source),
                account_id: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(listed, std::slice::from_ref(&r));

    // The same pair again: nothing written, no event.
    let (dup, dup_change) = route(a, source, account);
    assert_eq!(
        store.create(&dup, &dup_change).await.unwrap(),
        CreateOutcome::Exists
    );

    // Optimistic concurrency.
    let mut u = RouteUpdate {
        user_id: a,
        id: r.id,
        expected_version: 1,
        signal_types: types(&[RouteSignalType::TradePlan]),
        is_enabled: false,
        at: r.created_at,
    };
    let mut change = c.clone();
    change.action = ChangeAction::Disabled;
    let UpdateOutcome::Updated(updated) = store.update(&u, Some(&change)).await.unwrap() else {
        panic!("not updated");
    };
    assert_eq!(updated.row_version, 2);
    assert!(!updated.is_enabled);
    assert_eq!(updated.signal_types, types(&[RouteSignalType::TradePlan]));
    assert_eq!(
        store.update(&u, Some(&change)).await.unwrap(),
        UpdateOutcome::VersionConflict
    );
    u.id = RouteId::new_v7();
    assert_eq!(
        store.update(&u, Some(&change)).await.unwrap(),
        UpdateOutcome::NotFound
    );

    // Another user sees and changes nothing.
    assert!(
        store
            .list(b, RouteFilter::default())
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(store.get(b, r.id).await.unwrap(), None);
    let foreign = RouteUpdate {
        user_id: b,
        id: r.id,
        expected_version: 2,
        ..u.clone()
    };
    assert_eq!(
        store.update(&foreign, Some(&change)).await.unwrap(),
        UpdateOutcome::NotFound
    );
    assert!(!store.delete(b, r.id, &change).await.unwrap());
    // Writing a route for another user is refused by RLS.
    let (mut theirs, theirs_change) = route(b, source, account);
    theirs.user_id = b;
    assert_eq!(
        store.create(&theirs, &theirs_change).await.unwrap(),
        CreateOutcome::Created
    );
    assert_eq!(
        store.list(a, RouteFilter::default()).await.unwrap().len(),
        1
    );

    let mut removed = c.clone();
    removed.action = ChangeAction::Removed;
    assert!(store.delete(a, r.id, &removed).await.unwrap());
    assert!(!store.delete(a, r.id, &removed).await.unwrap());

    let ev = events(&owner, a).await;
    assert_eq!(
        ev.iter()
            .map(|(t, p)| (t.as_str(), p["action"].as_str().unwrap()))
            .collect::<Vec<_>>(),
        [
            ("routing.subscription_changed", "created"),
            ("routing.subscription_changed", "disabled"),
            ("routing.subscription_changed", "removed"),
        ]
    );
    assert_eq!(
        ev[0].1["new"]["signal_types"],
        serde_json::json!(["trigger", "nq_trigger"])
    );
    assert_eq!(events(&owner, b).await.len(), 1);
}

#[tokio::test]
async fn concurrent_creates_of_one_pair_store_one_route() {
    let Some((owner, app)) = app_pool().await else {
        return;
    };
    let store = PgRouteStore::new(app);
    let a = user(&owner).await;
    let (source, account) = (SourceId::new_v7(), AccountId::new_v7());
    let tasks: Vec<_> = (0..8)
        .map(|_| {
            let store = store.clone();
            tokio::spawn(async move {
                let (r, c) = route(a, source, account);
                store.create(&r, &c).await.unwrap()
            })
        })
        .collect();
    let mut created = 0;
    for t in tasks {
        if t.await.unwrap() == CreateOutcome::Created {
            created += 1;
        }
    }
    assert_eq!(created, 1);
    assert_eq!(events(&owner, a).await.len(), 1);
}

#[tokio::test]
async fn follower_subscriptions_are_not_s7_routes() {
    let Some((owner, app)) = app_pool().await else {
        return;
    };
    let store = PgRouteStore::new(app.clone());
    let a = user(&owner).await;
    let id = uuid::Uuid::now_v7();
    let mut tx = app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(a.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO routing.subscription (id, user_id, source_id, account_id, signal_types)
         VALUES ($1, $2, gen_random_uuid(), gen_random_uuid(), ARRAY['master_account'])",
    )
    .bind(id)
    .bind(a.as_uuid())
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    assert!(
        store
            .list(a, RouteFilter::default())
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(store.get(a, RouteId::from_uuid(id)).await.unwrap(), None);
}
