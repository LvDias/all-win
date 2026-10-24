//! Per-consumer outbox tracking (#14, wave1-data-model §6.4 rule 2, rev. 16.9): start cursors,
//! per-consumer retries and dead letters, the backfill reset, retirement, the delivery/registration
//! consistency rule and the NOTIFY wake-up. Each test registers its own consumers, so each runs on
//! a database of its own (`fresh_db`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::sync::{Arc, Mutex};

use app::outbox::{
    Consumers, EventConsumer, MAX_ATTEMPTS, OutboxDispatcher, OutboxEvent, RegistrationError,
    StartPolicy,
};
use support::*;
use uuid::Uuid;

const TYPE: &str = "test.event";

/// Records what it handled; fails every delivery when `fail`.
struct Probe {
    name: &'static str,
    start: StartPolicy,
    fail: bool,
    seen: Mutex<Vec<Uuid>>,
}

impl Probe {
    fn new(name: &'static str, start: StartPolicy, fail: bool) -> Arc<Self> {
        Arc::new(Self {
            name,
            start,
            fail,
            seen: Mutex::new(Vec::new()),
        })
    }

    fn seen(&self) -> Vec<Uuid> {
        self.seen.lock().unwrap().clone()
    }
}

#[async_trait::async_trait]
impl EventConsumer for Probe {
    fn name(&self) -> &'static str {
        self.name
    }

    fn start(&self) -> StartPolicy {
        self.start
    }

    async fn handle(&self, e: &OutboxEvent) -> Result<(), String> {
        self.seen.lock().unwrap().push(e.id);
        if self.fail {
            Err("consumer unavailable".into())
        } else {
            Ok(())
        }
    }
}

fn dispatcher(db: &TestDb, probes: &[Arc<Probe>]) -> OutboxDispatcher {
    let mut consumers = Consumers::default();
    for p in probes {
        consumers.register(TYPE, p.clone());
    }
    OutboxDispatcher::new(
        db.worker.clone(),
        consumers,
        Arc::new(tokio::sync::Notify::new()),
    )
}

async fn append(db: &TestDb) -> Uuid {
    let id = Uuid::now_v7();
    let user = Uuid::now_v7();
    let mut tx = db.app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload) VALUES ($1, $2, $3, now(), '{}')",
    )
    .bind(id)
    .bind(user)
    .bind(TYPE)
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    id
}

async fn published(db: &TestDb, id: Uuid) -> bool {
    sqlx::query_scalar("SELECT published_at IS NOT NULL FROM app.outbox WHERE id = $1")
        .bind(id)
        .fetch_one(&db.worker)
        .await
        .unwrap()
}

/// `(attempts, last_error, next attempt in the future, delivered, dead-lettered)`.
async fn delivery(
    db: &TestDb,
    consumer: &str,
    id: Uuid,
) -> Option<(i32, Option<String>, bool, bool, bool)> {
    sqlx::query_as(
        "SELECT attempts, last_error, next_attempt_at > now(), delivered_at IS NOT NULL,
                dead_lettered_at IS NOT NULL
           FROM app.outbox_delivery WHERE consumer = $1 AND event_id = $2",
    )
    .bind(consumer)
    .bind(id)
    .fetch_optional(&db.worker)
    .await
    .unwrap()
}

#[tokio::test]
async fn a_failing_consumer_retries_and_dead_letters_without_blocking_the_other() {
    let Some(db) = fresh_db().await else { return };
    let ok = Probe::new("t.ok", StartPolicy::Backfill, false);
    let bad = Probe::new("t.bad", StartPolicy::Backfill, true);
    let d = dispatcher(&db, &[ok.clone(), bad.clone()]);
    d.register().await.unwrap();
    let id = append(&db).await;
    d.dispatch_all().await.unwrap();
    assert_eq!(ok.seen(), [id]);
    assert_eq!(
        delivery(&db, "t.ok", id).await.map(|r| (r.0, r.3)),
        Some((1, true))
    );
    let (attempts, error, later, delivered, dead) = delivery(&db, "t.bad", id).await.unwrap();
    assert_eq!((attempts, delivered, dead), (1, false, false));
    assert!(later, "backoff: the next attempt is in the future");
    assert_eq!(error.as_deref(), Some("consumer unavailable"));
    assert!(!published(&db, id).await, "not done for t.bad yet");
    // Not due: skipped; the other consumer is never asked twice.
    d.dispatch_all().await.unwrap();
    assert_eq!((ok.seen().len(), bad.seen().len()), (1, 1));
    // Last attempt: dead-lettered for t.bad only, never fetched again, row stays unpublished.
    sqlx::query(
        "UPDATE app.outbox_delivery SET attempts = $2, next_attempt_at = now() WHERE consumer = 't.bad' AND event_id = $1",
    )
    .bind(id)
    .bind(MAX_ATTEMPTS - 1)
    .execute(&db.worker)
    .await
    .unwrap();
    d.dispatch_all().await.unwrap();
    let (attempts, _, _, _, dead) = delivery(&db, "t.bad", id).await.unwrap();
    assert_eq!((attempts, dead), (MAX_ATTEMPTS, true));
    d.dispatch_all().await.unwrap();
    assert_eq!(bad.seen().len(), 2, "a dead letter is never fetched again");
    assert!(!published(&db, id).await, "a dead letter is never done");
    // N5: every delivery row belongs to a registered (consumer, event type) pair.
    let orphans: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM app.outbox_delivery d JOIN app.outbox o ON o.id = d.event_id
          WHERE NOT EXISTS (SELECT 1 FROM app.outbox_consumer c
                             WHERE c.consumer = d.consumer AND c.event_type = o.event_type)",
    )
    .fetch_one(&db.worker)
    .await
    .unwrap();
    assert_eq!(orphans, 0);
}

#[tokio::test]
async fn a_from_now_consumer_starts_after_the_rows_already_written() {
    let Some(db) = fresh_db().await else { return };
    let before = append(&db).await;
    let live = Probe::new("t.live", StartPolicy::FromNow, false);
    let d = dispatcher(&db, std::slice::from_ref(&live));
    d.register().await.unwrap();
    assert!(
        published(&db, before).await,
        "before the start cursor of the only consumer: done at registration"
    );
    let after = append(&db).await;
    d.dispatch_all().await.unwrap();
    assert_eq!(live.seen(), [after]);
    assert!(published(&db, after).await);
    // Registration is idempotent and the cursor never moves.
    let start: i64 = sqlx::query_scalar("SELECT start_seq FROM app.outbox_consumer")
        .fetch_one(&db.worker)
        .await
        .unwrap();
    d.register().await.unwrap();
    let again: i64 = sqlx::query_scalar("SELECT start_seq FROM app.outbox_consumer")
        .fetch_one(&db.worker)
        .await
        .unwrap();
    assert_eq!(start, again);
}

#[tokio::test]
async fn a_backfill_consumer_added_later_receives_rows_others_completed() {
    let Some(db) = fresh_db().await else { return };
    let live = Probe::new("t.live", StartPolicy::FromNow, false);
    let first = dispatcher(&db, std::slice::from_ref(&live));
    first.register().await.unwrap();
    let id = append(&db).await;
    first.dispatch_all().await.unwrap();
    assert!(published(&db, id).await);
    // A new backfill consumer: the row goes back to the fetch (Esmeril M1).
    let audit = Probe::new("t.audit", StartPolicy::Backfill, false);
    let second = dispatcher(&db, &[live.clone(), audit.clone()]);
    second.register().await.unwrap();
    assert!(!published(&db, id).await, "not done for the new consumer");
    second.dispatch_all().await.unwrap();
    assert_eq!(audit.seen(), [id]);
    assert_eq!(live.seen(), [id], "the first consumer is not asked again");
    assert!(published(&db, id).await, "done for both again");
}

#[tokio::test]
async fn a_dropped_consumer_fails_startup_until_retired_and_retirement_publishes() {
    let Some(db) = fresh_db().await else { return };
    let keep = Probe::new("t.keep", StartPolicy::Backfill, false);
    let gone = Probe::new("t.gone", StartPolicy::Backfill, true);
    let both = dispatcher(&db, &[keep.clone(), gone]);
    both.register().await.unwrap();
    let id = append(&db).await;
    both.dispatch_all().await.unwrap();
    assert!(!published(&db, id).await);
    let only = dispatcher(&db, &[keep]);
    match only.register().await {
        Err(RegistrationError::Unregistered(pairs)) => {
            assert_eq!(pairs, [format!("t.gone/{TYPE}")]);
        }
        other => panic!("expected Unregistered, got {other:?}"),
    }
    // The operator step (allwin_maint), then the next startup publishes what is now done.
    sqlx::query("UPDATE app.outbox_consumer SET retired_at = now() WHERE consumer = 't.gone'")
        .execute(&db.maint)
        .await
        .unwrap();
    only.register().await.unwrap();
    assert!(
        published(&db, id).await,
        "done for every remaining consumer"
    );
}

#[tokio::test]
async fn a_changed_start_policy_fails_startup() {
    let Some(db) = fresh_db().await else { return };
    dispatcher(&db, &[Probe::new("t.c", StartPolicy::FromNow, false)])
        .register()
        .await
        .unwrap();
    let r = dispatcher(&db, &[Probe::new("t.c", StartPolicy::Backfill, false)])
        .register()
        .await;
    assert!(
        matches!(r, Err(RegistrationError::PolicyChanged { .. })),
        "{r:?}"
    );
}

#[tokio::test]
async fn the_worker_cannot_move_a_start_cursor_or_rewrite_a_delivery_key() {
    let Some(db) = fresh_db().await else { return };
    let p = Probe::new("t.g", StartPolicy::Backfill, false);
    let d = dispatcher(&db, &[p]);
    d.register().await.unwrap();
    let id = append(&db).await;
    d.dispatch_all().await.unwrap();
    for sql in [
        "UPDATE app.outbox_consumer SET start_seq = 2",
        "UPDATE app.outbox_consumer SET retired_at = now()",
        "UPDATE app.outbox_delivery SET consumer = 't.other'",
    ] {
        let e = sqlx::query(sql).execute(&db.worker).await.unwrap_err();
        assert!(e.to_string().contains("permission denied"), "{sql}: {e}");
    }
    assert!(published(&db, id).await);
}

#[tokio::test]
async fn an_insert_notifies_the_dispatcher_at_commit_without_payload() {
    let Some(db) = fresh_db().await else { return };
    let mut listener = sqlx::postgres::PgListener::connect_with(&db.worker)
        .await
        .unwrap();
    listener.listen(app::outbox::NOTIFY_CHANNEL).await.unwrap();
    append(&db).await;
    let n = tokio::time::timeout(std::time::Duration::from_secs(10), listener.recv())
        .await
        .expect("a notification after the commit")
        .unwrap();
    assert_eq!(n.channel(), app::outbox::NOTIFY_CHANNEL);
    assert_eq!(n.payload(), "", "never an id or a type");
}

/// Inserts `n` rows of `event_type` that occurred `days` ago, for one user.
async fn append_many(db: &TestDb, event_type: &str, n: i32, days: i32) {
    let user = Uuid::now_v7();
    let mut tx = db.app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload)
         SELECT gen_random_uuid(), $1, $2, now() - make_interval(days => $4), '{}'
           FROM generate_series(1, $3)",
    )
    .bind(user)
    .bind(event_type)
    .bind(n)
    .bind(days)
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
}

async fn count(db: &TestDb, sql: &'static str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(&db.worker).await.unwrap()
}

/// §6.5 retention (as allwin_maint): old rows done for every consumer or of a listed type go,
/// with their delivery rows (FK cascade through outbox_delivery_event_idx); young, unfinished and
/// dead-lettered rows stay.
#[tokio::test]
async fn the_purge_keeps_only_what_is_young_unfinished_or_dead() {
    let Some(db) = fresh_db().await else { return };
    let p = Probe::new("t.p", StartPolicy::Backfill, false);
    let d = dispatcher(&db, &[p]);
    d.register().await.unwrap();
    append_many(&db, TYPE, 1100, 8).await;
    append_many(&db, TYPE, 3, 1).await;
    while d.dispatch_all().await.unwrap().0 > 0 {}
    append_many(&db, TYPE, 2, 8).await; // old, never delivered: not done
    append_many(&db, "test.listed", 4, 8).await; // old, listed: purged
    append_many(&db, "test.listed", 1, 1).await; // young, listed: kept
    sqlx::query(
        "UPDATE app.outbox SET dead_lettered_at = now()
          WHERE id = (SELECT id FROM app.outbox WHERE event_type = 'test.listed' ORDER BY seq LIMIT 1)",
    )
    .execute(&db.maint)
    .await
    .unwrap(); // a legacy dead letter among the old listed rows: kept
    sqlx::query("ANALYZE app.outbox_delivery")
        .execute(&db.owner)
        .await
        .unwrap();
    let plan: Vec<String> = sqlx::query_scalar(
        "EXPLAIN SELECT 1 FROM app.outbox_delivery WHERE event_id = '0190d3c4-0000-7000-8000-000000000001'",
    )
    .fetch_all(&db.owner)
    .await
    .unwrap();
    assert!(
        plan.iter().any(|l| l.contains("outbox_delivery_event_idx")),
        "the cascade lookup uses the index: {plan:?}"
    );
    let deleted = app::purge::purge_outbox(&db.maint, &["test.listed".to_owned()])
        .await
        .unwrap();
    assert_eq!(deleted, 1100 + 3);
    assert_eq!(
        count(&db, "SELECT count(*) FROM app.outbox").await,
        3 + 2 + 1 + 1
    );
    assert_eq!(
        count(&db, "SELECT count(*) FROM app.outbox_delivery").await,
        3,
        "delivery rows went with their events"
    );
}

/// A consumer with an external side effect takes one event per transaction (Crivo L1/L2).
struct OneAtATime(Arc<Probe>);

#[async_trait::async_trait]
impl EventConsumer for OneAtATime {
    fn name(&self) -> &'static str {
        self.0.name
    }

    fn batch(&self) -> i64 {
        1
    }

    async fn handle(&self, e: &OutboxEvent) -> Result<(), String> {
        self.0.handle(e).await
    }
}

#[tokio::test]
async fn a_batch_of_one_consumer_commits_each_event_on_its_own() {
    let Some(db) = fresh_db().await else { return };
    let probe = Probe::new("t.mail", StartPolicy::Backfill, false);
    let consumer = OneAtATime(probe.clone());
    let mut consumers = Consumers::default();
    consumers.register(TYPE, Arc::new(OneAtATime(probe.clone())));
    let d = OutboxDispatcher::new(
        db.worker.clone(),
        consumers,
        Arc::new(tokio::sync::Notify::new()),
    );
    d.register().await.unwrap();
    for _ in 0..3 {
        append(&db).await;
    }
    assert_eq!(d.dispatch_batch(&consumer).await.unwrap(), (1, 1));
    assert_eq!(probe.seen().len(), 1);
    assert_eq!(
        count(
            &db,
            "SELECT count(*) FROM app.outbox_delivery WHERE delivered_at IS NOT NULL"
        )
        .await,
        1
    );
}
