//! publish-release end to end on PostgreSQL as `allwin_publisher`: the contract's example files,
//! idempotency, all-or-nothing refusals, the role's limits and the serial publish lock.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::sync::Arc;
use std::time::{Duration, Instant};

use catalog_application::enterprise::release::import::{
    ImportOutcome, ImportPolicy, ImportProblem,
};
use catalog_application::release::{
    PublishError, PublishRelease, PublishResult, PublishedBy, ReleaseStore,
};
use catalog_infra::release_file::parse_release;
use catalog_infra::release_store::PgReleaseStore;
use serde_json::{Value, json};

const POLICY: ImportPolicy = ImportPolicy {
    accept_personal_device_not_confirmed: false,
    accept_not_material: false,
};

macro_rules! example {
    ($name:literal) => {
        include_str!(concat!(
            "../../../../../contracts/presets/v1/examples/",
            $name
        ))
    };
}

fn file(v: &Value) -> catalog_application::release::ValidatedRelease {
    parse_release(&serde_json::to_vec(v).unwrap()).unwrap()
}

/// Each test publishes its own firm: the examples' slug made unique.
fn with_slug(raw: &str, slug: &str) -> Value {
    let mut v: Value = serde_json::from_str(raw).unwrap();
    v["firm"]["slug"] = json!(slug);
    v
}

fn v2(slug: &str) -> Value {
    let mut v = with_slug(example!("valid/release-v2-stance-changed.json"), slug);
    // The 1.0.0 fixture repeats v1's drawdown while declaring it changed (fixed in presets 1.0.1).
    v["plans"][0]["phases"][0]["sizes"][0]["rule_sets"][0]["rules"]["drawdown"]["value"]["amount"] =
        json!("2500.00");
    v
}

fn slug() -> String {
    format!("t-{}", uuid::Uuid::now_v7().simple())
}

fn by() -> PublishedBy {
    PublishedBy::parse("torno").unwrap()
}

async fn count(db: &support::TestDb, sql: &str, slug: &str) -> i64 {
    sqlx::query_scalar(sqlx::AssertSqlSafe(sql.to_owned()))
        .bind(slug)
        .fetch_one(&db.admin)
        .await
        .unwrap()
}

const RELEASES: &str = "SELECT count(*) FROM catalog.firm_release r JOIN catalog.prop_firm f ON f.id = r.firm_id WHERE f.slug = $1";
const TEXTS: &str = "SELECT count(*) FROM catalog.opt_in_rule_version v JOIN catalog.prop_firm f ON f.id = v.firm_id WHERE f.slug = $1";
const PRESETS: &str = "SELECT count(*) FROM catalog.preset p JOIN catalog.prop_firm f ON f.id = p.firm_id WHERE f.slug = $1";
const VERSIONS: &str = "SELECT count(*) FROM catalog.preset_version p JOIN catalog.prop_firm f ON f.id = p.firm_id WHERE f.slug = $1";

#[tokio::test]
async fn publish_v1_then_v2_idempotently() {
    let Some(db) = support::db().await else {
        return;
    };
    let publish = PublishRelease::new(Arc::new(PgReleaseStore::new(db.publisher.clone())), POLICY);
    let s = slug();
    let v1 = file(&with_slug(example!("valid/release-v1-new-firm.json"), &s));

    let first = publish.handle(&v1, &by()).await.unwrap();
    let PublishResult::Published {
        version_number: 1,
        publish_seq: seq1,
    } = first
    else {
        panic!("{first:?}")
    };
    assert_eq!(count(&db, RELEASES, &s).await, 1);
    assert_eq!(count(&db, TEXTS, &s).await, 2);
    let presets = count(&db, PRESETS, &s).await;
    assert!(presets > 0);

    // The same file again: a successful no-op.
    assert_eq!(
        publish.handle(&v1, &by()).await,
        Ok(PublishResult::AlreadyPublished { version_number: 1 })
    );
    assert_eq!(count(&db, RELEASES, &s).await, 1);

    // v2: one new rule text (automation), the same preset rows, new preset versions.
    let second = publish.handle(&file(&v2(&s)), &by()).await.unwrap();
    let PublishResult::Published {
        version_number: 2,
        publish_seq: seq2,
    } = second
    else {
        panic!("{second:?}")
    };
    assert!(seq2 > seq1);
    assert_eq!(count(&db, TEXTS, &s).await, 3);
    assert_eq!(count(&db, PRESETS, &s).await, presets);
    assert_eq!(count(&db, VERSIONS, &s).await, 2 * presets);
    let (cause, previous): (Option<String>, bool) = sqlx::query_as(
        "SELECT change_cause, previous_id IS NOT NULL FROM catalog.opt_in_rule_version v
         JOIN catalog.prop_firm f ON f.id = v.firm_id WHERE f.slug = $1 AND v.version_number = 2",
    )
    .bind(&s)
    .fetch_one(&db.admin)
    .await
    .unwrap();
    assert_eq!((cause.as_deref(), previous), (Some("stance_changed"), true));

    // Other content for a published version: refused, nothing written.
    let mut other = v2(&s);
    other["firm"]["name"] = json!("Renamed");
    let err = publish.handle(&file(&other), &by()).await.unwrap_err();
    assert_eq!(
        err,
        PublishError::Rejected(vec![ImportProblem::SameVersionOtherContent(2)])
    );
    assert_eq!(count(&db, RELEASES, &s).await, 2);
}

#[tokio::test]
async fn a_refused_file_writes_nothing() {
    let Some(db) = support::db().await else {
        return;
    };
    let publish = PublishRelease::new(Arc::new(PgReleaseStore::new(db.publisher.clone())), POLICY);
    let s = slug();
    let mut bad = with_slug(example!("valid/release-v1-new-firm.json"), &s);
    bad["stances"]["automation"][0]["quotes"][0]["url"] =
        json!("https://elsewhere.example.org/rules");
    let err = publish.handle(&file(&bad), &by()).await.unwrap_err();
    assert!(
        matches!(&err, PublishError::Rejected(p) if p.iter().any(|p| matches!(p, ImportProblem::ForeignDomain(_)))),
        "{err:?}"
    );
    assert_eq!(
        count(
            &db,
            "SELECT count(*) FROM catalog.prop_firm WHERE slug = $1",
            &s
        )
        .await,
        0
    );
}

#[tokio::test]
async fn the_publisher_writes_only_the_catalog() {
    let Some(db) = support::db().await else {
        return;
    };
    let denied = |sql: &'static str| {
        let pool = db.publisher.clone();
        async move {
            let err = sqlx::query(sql)
                .execute(&pool)
                .await
                .unwrap_err()
                .to_string();
            assert!(err.contains("permission denied"), "{sql}: {err}");
        }
    };
    denied("INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload) VALUES (gen_random_uuid(), gen_random_uuid(), 'catalog.x', now(), '{}')").await;
    denied("UPDATE accounts.catalog_release_cursor SET last_publish_seq = 1").await;
    denied("UPDATE catalog.firm_release SET verified_on = now()").await;
    denied("DELETE FROM catalog.preset_rule_set").await;
    denied("UPDATE catalog.opt_in_rule_version SET firm_stance = 'allowed'").await;
    // A table lock above ROW EXCLUSIVE needs UPDATE/DELETE/TRUNCATE: the CLI uses the advisory lock.
    let mut tx = db.publisher.begin().await.unwrap();
    let err = sqlx::query("LOCK TABLE catalog.firm_release IN EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .unwrap_err()
        .to_string();
    assert!(err.contains("permission denied"), "{err}");
}

// Multi-thread: the first planner blocks its worker thread while holding the lock.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_publications_are_serial_and_commit_in_seq_order() {
    let Some(db) = support::db().await else {
        return;
    };
    // Two connections (the publisher pool has one): the lock does not depend on the role.
    let store = Arc::new(PgReleaseStore::new(db.admin.clone()));
    let (a, b) = (slug(), slug());
    let fa = file(&with_slug(example!("valid/release-v1-new-firm.json"), &a));
    let fb = file(&with_slug(example!("valid/release-v1-new-firm.json"), &b));
    let (locked_tx, locked_rx) = std::sync::mpsc::channel::<()>();
    let a_planned = Arc::new(std::sync::Mutex::new(None::<Instant>));
    let slow = {
        let store = store.clone();
        let a_planned = a_planned.clone();
        tokio::spawn(async move {
            let planner = |stored: Option<&_>| {
                // Inside the first transaction, after the publish lock: say so, then hold it.
                let _ = locked_tx.send(());
                std::thread::sleep(Duration::from_millis(600));
                *a_planned.lock().unwrap() = Some(Instant::now());
                catalog_application::enterprise::release::import::plan_import(
                    &fa.file, &fa.raw, fa.sha256, stored, POLICY,
                )
            };
            store.publish(&fa.file.firm.slug, &by(), &planner).await
        })
    };
    tokio::task::spawn_blocking(move || locked_rx.recv().unwrap())
        .await
        .unwrap();
    let b_planned = std::sync::Mutex::new(None::<Instant>);
    let planner = |stored: Option<&_>| -> Result<ImportOutcome, Vec<ImportProblem>> {
        *b_planned.lock().unwrap() = Some(Instant::now());
        catalog_application::enterprise::release::import::plan_import(
            &fb.file, &fb.raw, fb.sha256, stored, POLICY,
        )
    };
    let second = store
        .publish(&fb.file.firm.slug, &by(), &planner)
        .await
        .unwrap();
    let first = slow.await.unwrap();
    let (
        PublishResult::Published {
            publish_seq: s1, ..
        },
        PublishResult::Published {
            publish_seq: s2, ..
        },
    ) = (first.unwrap(), second)
    else {
        panic!("both publish")
    };
    // B's planner runs only after B holds the lock, i.e. after A committed, which is after A's
    // planner finished: no scheduling assumption.
    let a = a_planned.lock().unwrap().unwrap();
    let b = b_planned.lock().unwrap().unwrap();
    assert!(b > a, "the second publication waited for the first");
    assert!(s2 > s1, "commit order = publish_seq order");
}

#[tokio::test]
async fn the_feed_lists_releases_after_the_cursor_with_replaced_texts() {
    use catalog_application::release_feed::ReleasesPublishedAfter;
    let Some(db) = support::db().await else {
        return;
    };
    let publish = PublishRelease::new(Arc::new(PgReleaseStore::new(db.publisher.clone())), POLICY);
    let feed = ReleasesPublishedAfter::new(Arc::new(catalog_infra::PgInstrumentRepository::new(
        db.app.clone(),
    )));
    let s = slug();
    let PublishResult::Published {
        publish_seq: seq1, ..
    } = publish
        .handle(
            &file(&with_slug(example!("valid/release-v1-new-firm.json"), &s)),
            &by(),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let PublishResult::Published {
        publish_seq: seq2, ..
    } = publish.handle(&file(&v2(&s)), &by()).await.unwrap()
    else {
        panic!()
    };
    let after = feed.handle(seq1 - 1, 100).await.unwrap();
    let mine: Vec<_> = after
        .iter()
        .filter(|r| r.publish_seq == seq1 || r.publish_seq == seq2)
        .collect();
    assert_eq!(mine.len(), 2);
    assert!(
        mine[0].replaced.is_empty(),
        "a first release replaces nothing"
    );
    assert!(mine[0].superseded.is_empty() && !mine[1].superseded.is_empty());
    assert!(!mine[1].firm_name.is_empty());
    let r = &mine[1].replaced;
    assert_eq!(r.first().map(|t| t.new_version_number), Some(2));
    assert_eq!(r.len(), 1);
    assert_eq!(
        (r[0].kind.as_str(), r[0].cause.as_str(), r[0].suspends()),
        ("automation", "stance_changed", true)
    );
    assert!(
        feed.handle(seq2, 100)
            .await
            .unwrap()
            .iter()
            .all(|x| x.publish_seq > seq2)
    );
}
