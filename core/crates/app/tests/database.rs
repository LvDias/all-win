//! Migration-level guarantees (DBA checklist, wave1-data-model §3, §9).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use sqlx::Acquire;
use support::*;

const PLANTED: &str = "awk_AbCdEf123456_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

#[tokio::test]
async fn every_user_table_has_forced_rls_and_a_policy() {
    let Some(db) = db().await else { return };
    let db = &db;
    let missing: Vec<String> = sqlx::query_scalar(
        r#"SELECT n.nspname || '.' || c.relname
             FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
            WHERE c.relkind IN ('r', 'p') AND NOT c.relispartition
              AND n.nspname IN ('app','identity','catalog','accounts','routing','gateway','audit','risk')
              AND EXISTS (SELECT 1 FROM pg_attribute a WHERE a.attrelid = c.oid AND a.attname = 'user_id' AND NOT a.attisdropped)
              AND (NOT c.relrowsecurity OR NOT c.relforcerowsecurity
                   OR NOT EXISTS (SELECT 1 FROM pg_policy p WHERE p.polrelid = c.oid))"#,
    )
    .fetch_all(&db.owner)
    .await
    .unwrap();
    assert!(missing.is_empty(), "{missing:?}");
}

#[tokio::test]
async fn outbox_is_scoped_append_only_and_token_free() {
    let Some(db) = db().await else { return };
    let db = &db;
    let user = seed(db).await.user_id;
    let other = seed(db).await.user_id;
    // No context: nothing visible, nothing insertable.
    let visible: i64 = sqlx::query_scalar("SELECT count(*) FROM app.outbox")
        .fetch_one(&db.app)
        .await
        .unwrap();
    assert_eq!(visible, 0);
    let insert = |u: uuid::Uuid, payload: serde_json::Value| {
        sqlx::query("INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload) VALUES ($1, $2, 'ingress.received', now(), $3)")
            .bind(uuid::Uuid::now_v7())
            .bind(u)
            .bind(payload)
    };
    assert!(
        insert(user, serde_json::json!({}))
            .execute(&db.app)
            .await
            .is_err()
    );
    as_user(db, user, move |c| {
        Box::pin(async move {
            assert!(
                insert(other, serde_json::json!({}))
                    .execute(&mut *c)
                    .await
                    .is_err(),
                "WITH CHECK"
            );
        })
    })
    .await;
    as_user(db, user, move |c| {
        Box::pin(async move {
            let e = insert(user, serde_json::json!({ "x": PLANTED }))
                .execute(&mut *c)
                .await
                .unwrap_err();
            assert!(e.to_string().contains("outbox_no_token"), "{e}");
        })
    })
    .await;
    for sql in [
        "UPDATE app.outbox SET published_at = now()",
        "DELETE FROM app.outbox",
    ] {
        let e = as_user(db, user, move |c| {
            Box::pin(async move { sqlx::query(sql).execute(c).await.unwrap_err() })
        })
        .await;
        assert!(e.to_string().contains("permission denied"), "{sql}: {e}");
    }
}

#[tokio::test]
async fn audit_entries_are_immutable_and_receipts_append_only() {
    let Some(db) = db().await else { return };
    let db = &db;
    let user = seed(db).await.user_id;
    for sql in [
        "UPDATE audit.audit_entry SET code = 'ingress.x'",
        "DELETE FROM audit.audit_entry",
        "UPDATE routing.message_receipt SET received_at = now()",
        "DELETE FROM routing.message_receipt",
    ] {
        let e = as_user(db, user, move |c| {
            Box::pin(async move { sqlx::query(sql).execute(c).await.unwrap_err() })
        })
        .await;
        assert!(e.to_string().contains("permission denied"), "{sql}: {e}");
    }
    // The trigger holds even for the owner; only allwin_maint may delete (closed-account erasure).
    let insert_entry = || {
        sqlx::query(
            "INSERT INTO audit.audit_entry (id, user_id, occurred_at, source_event_id, category, code) VALUES ($1, $2, now(), $3, 'ingress', 'ingress.received')",
        )
        .bind(uuid::Uuid::now_v7())
        .bind(user)
        .bind(uuid::Uuid::now_v7())
    };
    let mut tx = db.owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    insert_entry().execute(&mut *tx).await.unwrap();
    for sql in [
        "UPDATE audit.audit_entry SET code = 'ingress.x'",
        "DELETE FROM audit.audit_entry",
    ] {
        let mut sp = tx.begin().await.unwrap();
        let e = sqlx::query(sql).execute(&mut *sp).await.unwrap_err();
        assert!(e.to_string().contains("immutable"), "{sql}: {e}");
        sp.rollback().await.unwrap();
    }
    tx.commit().await.unwrap();

    let e = sqlx::query("UPDATE audit.audit_entry SET code = 'ingress.x' WHERE user_id = $1")
        .bind(user)
        .execute(&db.maint)
        .await
        .unwrap_err();
    assert!(
        e.to_string().contains("permission denied") || e.to_string().contains("immutable"),
        "{e}"
    );
    let erased = sqlx::query("DELETE FROM audit.audit_entry WHERE user_id = $1")
        .bind(user)
        .execute(&db.maint)
        .await
        .unwrap()
        .rows_affected();
    assert_eq!(erased, 1);
}

#[tokio::test]
async fn audit_partitions_are_maintained_through_definer_functions_only() {
    let Some(db) = db().await else { return };
    let db = &db;
    // Idempotent; callable by maint only.
    sqlx::query("SELECT audit.ensure_partitions(3)")
        .execute(&db.maint)
        .await
        .unwrap();
    assert!(
        sqlx::query("SELECT audit.ensure_partitions(3)")
            .execute(&db.app)
            .await
            .is_err()
    );
    let months: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_inherits WHERE inhparent = 'audit.audit_entry'::regclass",
    )
    .fetch_one(&db.owner)
    .await
    .unwrap();
    assert!(months >= 5, "current + 3 ahead + default, got {months}");
    // An old, empty partition is detached and dropped; current months are kept.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS audit.audit_entry_200001 PARTITION OF audit.audit_entry
         FOR VALUES FROM ('2000-01-01 00:00+00') TO ('2000-02-01 00:00+00')",
    )
    .execute(&db.owner)
    .await
    .unwrap();
    let dropped: i32 = sqlx::query_scalar("SELECT audit.drop_partitions_before('2000-02-01')")
        .fetch_one(&db.maint)
        .await
        .unwrap();
    assert_eq!(dropped, 1);
    assert!(
        sqlx::query("SELECT audit.drop_partitions_before('2000-02-15')")
            .execute(&db.maint)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("ALTER TABLE audit.audit_entry DETACH PARTITION audit.audit_entry_default")
            .execute(&db.maint)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn token_check_rejects_a_planted_token_in_instruction_payload() {
    let Some(db) = db().await else { return };
    let db = &db;
    let s = seed(db).await;
    let user = s.user_id;
    let e = as_user(db, user, move |c| {
        Box::pin(async move {
            sqlx::query(
                "INSERT INTO routing.instruction (id, user_id, source_id, source_type, message_id, schema_version, signal_type, action,
                   instrument_symbol, sent_at, received_at, expiry_base, payload)
                 VALUES ($1, $2, $3, 'tradingview', 'm', '1.0', 'trigger', 'buy', 'MNQ', now(), now(), now(), $4)",
            )
            .bind(uuid::Uuid::now_v7())
            .bind(user)
            .bind(uuid::Uuid::now_v7())
            .bind(serde_json::json!({ "details": { "t": PLANTED } }))
            .execute(c)
            .await
            .unwrap_err()
        })
    })
    .await;
    assert!(e.to_string().contains("instruction_no_token"), "{e}");
}

#[tokio::test]
async fn decimals_round_trip_at_maximum_scale() {
    let Some(db) = db().await else { return };
    let db = &db;
    let (price, money): (rust_decimal::Decimal, rust_decimal::Decimal) = sqlx::query_as(
        "SELECT '999999999999.0000000001'::numeric(22,10), '99999999999999.9999'::numeric(18,4)",
    )
    .fetch_one(&db.app)
    .await
    .unwrap();
    assert_eq!(price.to_string(), "999999999999.0000000001");
    assert_eq!(money.to_string(), "99999999999999.9999");
    let tick: rust_decimal::Decimal =
        sqlx::query_scalar("SELECT tick_size FROM catalog.instrument WHERE symbol = 'MNQ'")
            .fetch_one(&db.app)
            .await
            .unwrap();
    assert_eq!(tick.to_string(), "0.2500000000");
}
