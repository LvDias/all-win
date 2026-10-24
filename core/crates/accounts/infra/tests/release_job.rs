//! The release job end to end on PostgreSQL: the worker finds candidates with its column grant,
//! each user's change runs as that user (RLS), every write is idempotent, the cursor advances only
//! after a release is done, and a not_material release waits. The catalog is seeded by SQL and the
//! feed is a fake (no dependency on the catalog module).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::sync::{Arc, Mutex};

use accounts_application::enterprise::{FirmId, OptInKind, PresetVersionId, RuleVersionId};
use accounts_application::ports::PortError;
use accounts_application::presets::{
    FirmRelease, ProcessPublishedReleases, ReleaseFeed, ReleaseJobStore, ReplaceCause, ReplacedText,
};
use accounts_infra::presets::PgReleaseJobStore;
use async_trait::async_trait;
use serde_json::Value;
use shared_kernel::SystemClock;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

struct Feed(Mutex<Vec<FirmRelease>>);

#[async_trait]
impl ReleaseFeed for Feed {
    async fn published_after(&self, after: i64, limit: i64) -> Result<Vec<FirmRelease>, PortError> {
        let all = self.0.lock().unwrap();
        Ok(all
            .iter()
            .filter(|r| r.publish_seq > after)
            .take(usize::try_from(limit).unwrap())
            .cloned()
            .collect())
    }
}

/// The worker pool of the test database (the harness exposes app and admin only).
async fn worker(db: &support::TestDb) -> sqlx::PgPool {
    let opts = db.admin.connect_options();
    let url = format!(
        "postgres://allwin_worker:worker-test-only@{}:{}/{}",
        opts.get_host(),
        opts.get_port(),
        opts.get_database().unwrap()
    );
    PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap()
}

async fn exec(db: &support::TestDb, sql: &str) {
    sqlx::query(sqlx::AssertSqlSafe(sql.to_owned()))
        .execute(&db.admin)
        .await
        .unwrap();
}

async fn outbox(db: &support::TestDb, user: Uuid) -> Vec<(Uuid, String, Value)> {
    sqlx::query_as("SELECT id, event_type, payload FROM app.outbox WHERE user_id = $1 ORDER BY occurred_at, id")
        .bind(user)
        .fetch_all(&db.admin)
        .await
        .unwrap()
}

/// A prop account with a preset and a chosen automation opt-in.
async fn prop_account(
    db: &support::TestDb,
    user: Uuid,
    name: &str,
    rule: Uuid,
    pv: Uuid,
    firm: Uuid,
    archived: bool,
) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO accounts.account (id, user_id, display_name, kind, firm_id, preset_version_id, size_amount,
             max_position_qty, automation_state, automation_choice, automation_rule_version_id, automation_chosen_at,
             archived_at)
         VALUES ($1, $2, $3, 'prop', $4, $5, 50000, 5, 'chosen', 'accepted', $6, now() - interval '1 day',
             CASE WHEN $7 THEN now() END)",
    )
    .bind(id)
    .bind(user)
    .bind(name)
    .bind(firm)
    .bind(pv)
    .bind(rule)
    .bind(archived)
    .execute(&db.admin)
    .await
    .unwrap();
    id
}

static AUDIT_DETAILS: &str = include_str!("release_job_audit_details.json");

fn conforms(schema_name: &str, value: &Value) {
    let schemas: Value = serde_json::from_str(AUDIT_DETAILS).unwrap();
    let v = jsonschema::validator_for(&schemas[schema_name]).unwrap();
    let errors: Vec<String> = v
        .iter_errors(value)
        .map(|e| format!("{e} at {}", e.instance_path()))
        .collect();
    assert!(errors.is_empty(), "{schema_name}: {errors:?}\n{value:#}");
}

#[tokio::test]
async fn releases_suspend_announce_and_never_repeat() {
    let Some(db) = support::db().await else {
        return;
    };
    let (firm, r1, r2, rv1, rv2, rv3) = (
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
    );
    let (pv_old, pv_new) = (Uuid::now_v7(), Uuid::now_v7());
    exec(&db, &format!("INSERT INTO catalog.prop_firm (id, slug, name) VALUES ('{firm}', 'job-{}', 'Job Firm')", firm.simple())).await;
    for (rel, n) in [(r1, 1), (r2, 2)] {
        exec(&db, &format!(
            "INSERT INTO catalog.firm_release (id, firm_id, version_number, verified_on, research_file, source_sha256, published_by)
             VALUES ('{rel}', '{firm}', {n}, '2026-10-20', 'docs/product/presets/research/job.md', decode(md5('{rel}') || md5('{rel}x'), 'hex'), 'torno')"
        )).await;
    }
    exec(&db, &format!(
        "INSERT INTO catalog.opt_in_rule_version (id, kind, firm_stance, is_generic, firm_id, version_number, quotes, release_id, previous_id, change_cause)
         VALUES ('{rv1}', 'automation', 'allowed', false, '{firm}', 1, '[]', '{r1}', NULL, NULL),
                ('{rv2}', 'automation', 'forbidden', false, '{firm}', 2, '[]', '{r2}', '{rv1}', 'stance_changed')"
    )).await;

    let (me, them) = (support::user(&db).await, support::user(&db).await);
    let hit = prop_account(&db, me, "Hit", rv1, pv_old, firm, false).await;
    let other_rule = prop_account(&db, me, "Other", rv3, pv_new, firm, false).await;
    let archived = prop_account(&db, me, "Gone", rv1, pv_old, firm, true).await;
    let theirs = prop_account(&db, them, "Theirs", rv1, pv_new, firm, false).await;

    let cursor0: i64 =
        sqlx::query_scalar("SELECT last_publish_seq FROM accounts.catalog_release_cursor")
            .fetch_one(&db.admin)
            .await
            .unwrap();
    let release = FirmRelease {
        publish_seq: cursor0 + 1,
        release_id: r2,
        firm_id: FirmId::from_uuid(firm),
        firm_name: "Job Firm".into(),
        replaced: vec![ReplacedText {
            kind: OptInKind::Automation,
            previous: RuleVersionId::from_uuid(rv1),
            new: RuleVersionId::from_uuid(rv2),
            new_version_number: 2,
            cause: ReplaceCause::StanceChanged,
        }],
        superseded: vec![(
            PresetVersionId::from_uuid(pv_old),
            PresetVersionId::from_uuid(pv_new),
        )],
    };
    let feed = Arc::new(Feed(Mutex::new(vec![release.clone()])));
    let store = Arc::new(PgReleaseJobStore::new(worker(&db).await, db.app.clone()));
    let job = ProcessPublishedReleases::new(feed.clone(), store.clone(), Arc::new(SystemClock));

    let report = job.handle().await.unwrap();
    assert_eq!(
        (
            report.releases_done,
            report.users,
            report.suspended,
            report.notified
        ),
        (1, 2, 2, 1)
    );
    assert_eq!(store.cursor().await.unwrap(), cursor0 + 1);

    let state = |id: Uuid| {
        let pool = db.admin.clone();
        async move {
            sqlx::query_as::<_, (String, Option<String>, Option<Uuid>, Option<String>)>(
                "SELECT automation_state, automation_pending_reason, automation_rule_version_id, automation_choice FROM accounts.account WHERE id = $1",
            )
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap()
        }
    };
    // Suspended with the previous choice kept (shown, never preselected).
    assert_eq!(
        state(hit).await,
        (
            "pending_review".into(),
            Some("stance_changed".into()),
            Some(rv1),
            Some("accepted".into())
        )
    );
    assert_eq!(state(theirs).await.0, "pending_review");
    assert_eq!(state(other_rule).await.0, "chosen");
    assert_eq!(
        state(archived).await.0,
        "chosen",
        "archived accounts are left alone"
    );

    let mine = outbox(&db, me).await;
    // One transaction writes them with the same `occurred_at`; the ids are UUIDv5, so their order
    // is not the write order: compare as a set and pick each event by its type.
    let mut types: Vec<&str> = mine.iter().map(|(_, t, _)| t.as_str()).collect();
    types.sort_unstable();
    assert_eq!(
        types,
        [
            "account.changed",
            "accounts.opt_in_suspended",
            "presets.version_available"
        ]
    );
    let event = |ty: &str| mine.iter().find(|(_, t, _)| t == ty).unwrap();
    let (_, _, suspended) = event("accounts.opt_in_suspended");
    assert_eq!(suspended["actor"], "system");
    assert_eq!(suspended["refs"]["account_id"], hit.to_string());
    conforms("accounts.opt_in_suspended", &suspended["details"]);
    let (va_id, _, va) = event("presets.version_available");
    assert_eq!(va["account_ids"], serde_json::json!([hit.to_string()]));
    let theirs_events = outbox(&db, them).await;
    assert!(
        theirs_events
            .iter()
            .all(|(_, t, _)| t != "presets.version_available"),
        "theirs is on the new version"
    );

    // Again: nothing changes; even with the cursor moved back (a crash before the advance).
    assert_eq!(job.handle().await.unwrap().releases_done, 0);
    exec(
        &db,
        &format!("UPDATE accounts.catalog_release_cursor SET last_publish_seq = {cursor0}"),
    )
    .await;
    let again = job.handle().await.unwrap();
    assert_eq!(
        (again.releases_done, again.suspended, again.notified),
        (1, 0, 0)
    );
    let mine_again = outbox(&db, me).await;
    assert_eq!(mine_again.len(), mine.len(), "no duplicate event");
    assert!(
        mine_again
            .iter()
            .any(|(id, t, _)| t == "presets.version_available" && id == va_id),
        "the version_available id is deterministic"
    );

    // A not_material release waits: nothing written, cursor held (an operator alert upstream).
    let mut nm = release.clone();
    nm.publish_seq = cursor0 + 2;
    nm.replaced[0].cause = ReplaceCause::NotMaterial;
    feed.0.lock().unwrap().push(nm);
    let waiting = job.handle().await.unwrap();
    assert_eq!(waiting.waiting_at, Some(cursor0 + 2));
    assert_eq!(store.cursor().await.unwrap(), cursor0 + 1);
    assert_eq!(outbox(&db, me).await.len(), mine.len());

    // The worker finds work only: no other columns, no writes.
    let w = worker(&db).await;
    for sql in [
        "SELECT display_name FROM accounts.account LIMIT 1",
        "SELECT venue_account_id FROM accounts.account LIMIT 1",
        "UPDATE accounts.account SET automation_state = 'chosen'",
        "DELETE FROM accounts.catalog_release_cursor",
    ] {
        let e = sqlx::query(sql).execute(&w).await.unwrap_err().to_string();
        assert!(e.contains("permission denied"), "{sql}: {e}");
    }
    // The app role never touches the cursor.
    let e = sqlx::query("SELECT last_publish_seq FROM accounts.catalog_release_cursor")
        .execute(&db.app)
        .await
        .unwrap_err()
        .to_string();
    assert!(e.contains("permission denied"), "{e}");
}
