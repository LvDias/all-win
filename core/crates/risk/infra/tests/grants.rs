//! Migrations #10 (`…061500`) and #10b (`…061510`) grants and RLS: the worker finds due work through
//! its column grants only, allwin_app cannot delete rows that a runtime path never deletes, and
//! only allwin_maint purges applied fills.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use sqlx::Acquire;
use uuid::Uuid;

fn denied(e: sqlx::Error) -> bool {
    e.as_database_error()
        .and_then(|d| d.code())
        .is_some_and(|c| c == "42501")
}

/// An account with limits, one schedule, one ended block and one trading day (seeded as superuser).
async fn seed(db: &support::TestDb) -> (Uuid, Uuid) {
    let user = support::user(db).await;
    let account = Uuid::now_v7();
    for sql in [
        "INSERT INTO risk.account_limits (user_id, account_id, version, derived_from_version,
           effective_as_of, daily_loss_kind, daily_loss_limit, reset_source)
         VALUES ($1, $2, 1, 1, now(), 'hard', 500, 'cme_session_close')",
        "INSERT INTO risk.scheduled_limit (user_id, account_id, kind, is_unset, effective_from,
           derived_from_version) VALUES ($1, $2, 'daily_loss', true, now() + interval '1 day', 1)",
        "INSERT INTO risk.account_block (id, user_id, account_id, cause, blocked_at, until, ended_at)
         VALUES (gen_random_uuid(), $1, $2, 'daily_loss', now() - interval '2 hours',
                 now() - interval '1 hour', now())",
        "INSERT INTO risk.trading_day (user_id, account_id, trading_date, starts_at, ends_at, source)
         VALUES ($1, $2, current_date, now() - interval '1 hour', now() + interval '1 hour',
                 'cme_session_close')",
        "INSERT INTO risk.applied_fill (user_id, account_id, fill_id, trading_date, qty, gross,
           commission_effective, commission_gap, last_event_id)
         VALUES ($1, $2, 'F-1', current_date, 2, -45.50, 1.24, 0.00, gen_random_uuid())",
    ] {
        sqlx::query(sql)
            .bind(user)
            .bind(account)
            .execute(&db.admin)
            .await
            .unwrap();
    }
    (user, account)
}

#[tokio::test]
async fn worker_reads_only_its_due_work_columns() {
    let Some(db) = support::db().await else {
        return;
    };
    let (user, account) = seed(&db).await;

    for sql in [
        "SELECT user_id, account_id, effective_from FROM risk.scheduled_limit WHERE account_id = $1",
        "SELECT user_id, account_id, until, ended_at FROM risk.account_block WHERE account_id = $1",
        "SELECT user_id, account_id, effective_as_of, reset_source, reset_local_time, reset_time_zone
           FROM risk.account_limits WHERE account_id = $1",
    ] {
        let found: Uuid = sqlx::query_scalar(sql)
            .bind(account)
            .fetch_one(&db.worker)
            .await
            .unwrap_or_else(|e| panic!("{sql}: {e}"));
        assert_eq!(found, user, "BYPASSRLS finds other users' due work");
    }

    for sql in [
        "SELECT daily_loss_limit FROM risk.account_limits WHERE account_id = $1",
        "SELECT new_value FROM risk.scheduled_limit WHERE account_id = $1",
        "SELECT cause FROM risk.account_block WHERE account_id = $1",
        "SELECT account_id FROM risk.trading_day WHERE account_id = $1",
        "SELECT account_id FROM risk.evaluation_progress WHERE account_id = $1",
        "SELECT account_id FROM risk.drawdown_tracker WHERE account_id = $1",
        "SELECT account_id FROM risk.applied_fill WHERE account_id = $1",
        "UPDATE risk.account_block SET ended_at = now() WHERE account_id = $1",
        "UPDATE risk.account_limits SET effective_as_of = now() WHERE account_id = $1",
        "DELETE FROM risk.scheduled_limit WHERE account_id = $1",
    ] {
        let e = sqlx::query(sql)
            .bind(account)
            .execute(&db.worker)
            .await
            .expect_err(sql);
        assert!(denied(e), "{sql}");
    }
}

#[tokio::test]
async fn app_deletes_only_due_schedules() {
    let Some(db) = support::db().await else {
        return;
    };
    let (user, account) = seed(&db).await;
    let mut tx = db.app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let removed = sqlx::query("DELETE FROM risk.scheduled_limit WHERE account_id = $1")
        .bind(account)
        .execute(&mut *tx)
        .await
        .unwrap();
    assert_eq!(removed.rows_affected(), 1);
    for table in [
        "applied_fill",
        "account_block",
        "trading_day",
        "evaluation_progress",
        "drawdown_tracker",
        "account_limits",
    ] {
        let sql = format!("DELETE FROM risk.{table} WHERE account_id = $1");
        let mut sp = tx.begin().await.unwrap();
        let e = sqlx::query(sqlx::AssertSqlSafe(sql))
            .bind(account)
            .execute(&mut *sp)
            .await
            .expect_err(table);
        assert!(denied(e), "{table}");
        sp.rollback().await.unwrap();
    }
}

async fn as_user(db: &support::TestDb, user: Uuid) -> sqlx::Transaction<'static, sqlx::Postgres> {
    let mut tx = db.app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    tx
}

#[tokio::test]
async fn applied_fills_are_per_user_and_purged_only_by_maint() {
    let Some(db) = support::db().await else {
        return;
    };
    let (owner, account) = seed(&db).await;
    let other = support::user(&db).await;

    let mut tx = as_user(&db, other).await;
    let seen: i64 = sqlx::query_scalar("SELECT count(*) FROM risk.applied_fill")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(seen, 0, "another user's fills are invisible");
    let e = sqlx::query(
        "INSERT INTO risk.applied_fill (user_id, account_id, fill_id, trading_date, qty, gross,
           commission_effective, commission_gap, last_event_id)
         VALUES ($1, $2, 'F-2', current_date, 1, 0, 0, 0, gen_random_uuid())",
    )
    .bind(owner)
    .bind(account)
    .execute(&mut *tx)
    .await
    .expect_err("insert with another user's id");
    assert!(e.to_string().contains("row-level security"), "{e}");
    tx.rollback().await.unwrap();

    let mut tx = as_user(&db, owner).await;
    let dup = sqlx::query(
        "INSERT INTO risk.applied_fill (user_id, account_id, fill_id, trading_date, qty, gross,
           commission_effective, commission_gap, last_event_id)
         VALUES ($1, $2, 'F-1', current_date, 2, -45.50, 1.24, 0, gen_random_uuid())
         ON CONFLICT (account_id, fill_id) DO NOTHING",
    )
    .bind(owner)
    .bind(account)
    .execute(&mut *tx)
    .await
    .unwrap();
    assert_eq!(dup.rows_affected(), 0, "a redelivered fill is a no-op");
    let e = sqlx::query("UPDATE risk.applied_fill SET removed_at = now() WHERE account_id = $1")
        .bind(account)
        .execute(&mut *tx)
        .await
        .expect_err("a removed fill keeps amounts");
    assert!(e.to_string().contains("applied_fill_removed_check"), "{e}");
    tx.rollback().await.unwrap();

    let purged = sqlx::query(
        "DELETE FROM risk.applied_fill WHERE account_id = $1 AND trading_date <= current_date",
    )
    .bind(account)
    .execute(&db.maint)
    .await
    .unwrap();
    assert_eq!(purged.rows_affected(), 1);
}
