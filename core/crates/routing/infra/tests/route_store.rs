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
use support::{app_pool, events, user};

mod support;

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
        source_name: "TV main".into(),
        account_id: account,
        account_name: "Apex 50k".into(),
        action: ChangeAction::Created,
        cause: ChangeCause::User,
        previous: None,
        new: Some((r.signal_types.clone(), true)),
        at,
    };
    (r, c)
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
            .map(|(t, p)| (t.as_str(), p["details"]["action"].as_str().unwrap()))
            .collect::<Vec<_>>(),
        [
            ("routing.subscription_changed", "created"),
            ("routing.subscription_changed", "disabled"),
            ("routing.subscription_changed", "removed"),
        ]
    );
    assert_eq!(
        ev[0].1["details"]["signal_types"],
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
