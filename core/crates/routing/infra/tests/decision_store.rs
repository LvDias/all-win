//! `PgDecisionStore` against PostgreSQL 16: the instruction read, one decision per
//! (instruction, account) with its audit event in the same transaction, redelivery and
//! concurrency without a second audit, append-only rows, and per-user isolation.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use chrono::{TimeZone, Utc};
use routing_application::decisions::{DecisionRecord, DecisionStore};
use routing_application::enterprise::decision::Outcome;
use routing_application::enterprise::{AccountId, InstructionId, SourceId};
use routing_infra::PgDecisionStore;
use serde_json::json;
use shared_kernel::UserId;
use sqlx::PgPool;
use support::{app_pool, events, user};

mod support;

/// Inserts an accepted `reverse` instruction of `user` (as the owner, in the user context).
async fn instruction(owner: &PgPool, user: UserId, source: SourceId) -> InstructionId {
    let id = InstructionId::new_v7();
    let at = Utc.with_ymd_and_hms(2026, 10, 23, 12, 0, 0).unwrap();
    let mut tx = owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO routing.instruction (id, user_id, source_id, source_type, message_id,
           schema_version, signal_type, action, instrument_symbol, quantity, sent_at, received_at,
           expiry_base, payload)
         VALUES ($1, $2, $3, 'tradingview', $4, '1.0', 'trigger', 'reverse', 'MNQ', 2, $5, $5, $5,
           $6)",
    )
    .bind(id.as_uuid())
    .bind(user.as_uuid())
    .bind(source.as_uuid())
    .bind(format!("m-{}", id.as_uuid().simple()))
    .bind(at)
    .bind(json!({"action":"reverse","side":"short","quantity":2}))
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    id
}

fn decision(user: UserId, instruction: InstructionId, account: AccountId) -> DecisionRecord {
    DecisionRecord {
        id: uuid::Uuid::now_v7(),
        user_id: user,
        instruction_id: instruction,
        account_id: account,
        account_name: "Apex 50k".into(),
        outcome: Outcome::Capped,
        reason_code: Some("routing.out_of_sync"),
        multiplier: 1,
        max_position_qty: 5,
        position_before: 3,
        requested_qty: Some(2),
        executed_qty: Some(0),
        decided_at: Utc.with_ymd_and_hms(2026, 10, 23, 12, 0, 1).unwrap(),
        event_type: "routing.out_of_sync",
        payload: json!({"actor":"system","details":{"reverse_closed_qty":3},
                        "refs":{"outcome":"partial","account_id":account,"account_name":"Apex 50k"}}),
    }
}

#[tokio::test]
async fn decisions_are_stored_once_with_their_audit_event_and_isolated() {
    let Some((owner, app)) = app_pool().await else {
        return;
    };
    let store = PgDecisionStore::new(app.clone());
    let alice = user(&owner).await;
    let bob = user(&owner).await;
    let source = SourceId::new_v7();
    let iid = instruction(&owner, alice, source).await;

    let i = store.instruction(alice, iid).await.unwrap().unwrap();
    assert_eq!((i.id, i.source_id), (iid, source));
    assert_eq!(
        (i.action.as_str(), i.side.as_deref()),
        ("reverse", Some("short"))
    );
    assert_eq!((i.position_effect, i.quantity), (None, Some(2)));
    assert_eq!(
        i.expiry_base,
        Utc.with_ymd_and_hms(2026, 10, 23, 12, 0, 0).unwrap()
    );
    // Another user's instruction does not exist for bob.
    assert!(store.instruction(bob, iid).await.unwrap().is_none());

    let (a1, a2) = (AccountId::new_v7(), AccountId::new_v7());
    let first = [decision(alice, iid, a1), decision(alice, iid, a2)];
    assert_eq!(store.record(alice, &first).await.unwrap(), 2);
    // A redelivery decides again: nothing new is stored or audited.
    let again = [decision(alice, iid, a1)];
    assert_eq!(store.record(alice, &again).await.unwrap(), 0);
    let ev = events(&owner, alice).await;
    assert_eq!(ev.len(), 2);
    assert!(
        ev.iter()
            .all(|(t, p)| t == "routing.out_of_sync" && p["refs"]["outcome"] == "partial")
    );

    let row: (
        String,
        Option<String>,
        i32,
        Option<i32>,
        Option<i32>,
        String,
    ) = {
        let mut tx = app.begin().await.unwrap();
        sqlx::query("SELECT set_config('app.user_id', $1, true)")
            .bind(alice.as_uuid().to_string())
            .execute(&mut *tx)
            .await
            .unwrap();
        sqlx::query_as(
            "SELECT outcome, reason_code, position_before, requested_qty, executed_qty, account_name
               FROM routing.routing_decision WHERE account_id = $1",
        )
        .bind(a1.as_uuid())
        .fetch_one(&mut *tx)
        .await
        .unwrap()
    };
    assert_eq!(
        row,
        (
            "capped".into(),
            Some("routing.out_of_sync".into()),
            3,
            Some(2),
            Some(0),
            "Apex 50k".into()
        )
    );

    // Bob sees no decision of alice; a decision for alice's instruction cannot be written in
    // bob's context (RLS and the (user_id, instruction_id) foreign key).
    let mut tx = app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(bob.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let seen: i64 = sqlx::query_scalar("SELECT count(*) FROM routing.routing_decision")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(seen, 0);
    tx.rollback().await.unwrap();
    let cross = [decision(bob, iid, AccountId::new_v7())];
    assert!(store.record(bob, &cross).await.is_err());
    assert!(events(&owner, bob).await.is_empty());

    // Append-only for the application role.
    let mut tx = app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(alice.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let err = sqlx::query("UPDATE routing.routing_decision SET outcome = 'routed'")
        .execute(&mut *tx)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("permission denied"), "{err}");
}

#[tokio::test]
async fn concurrent_redeliveries_audit_each_decision_once() {
    let Some((owner, app)) = app_pool().await else {
        return;
    };
    let store = PgDecisionStore::new(app);
    let u = user(&owner).await;
    let iid = instruction(&owner, u, SourceId::new_v7()).await;
    let account = AccountId::new_v7();
    let runs = (0..6).map(|_| {
        let store = store.clone();
        async move { store.record(u, &[decision(u, iid, account)]).await.unwrap() }
    });
    let new: usize = futures_sum(runs).await;
    assert_eq!(new, 1);
    assert_eq!(events(&owner, u).await.len(), 1);
}

async fn futures_sum(
    runs: impl Iterator<Item = impl std::future::Future<Output = usize> + Send + 'static>,
) -> usize {
    let handles: Vec<_> = runs.map(tokio::spawn).collect();
    let mut total = 0;
    for h in handles {
        total += h.await.unwrap();
    }
    total
}
