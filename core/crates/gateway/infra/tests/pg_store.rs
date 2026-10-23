//! PostgreSQL integration tests of `PgGatewayStore` and migration #9.
//!
//! Needs `TEST_DATABASE_ADMIN_URL` (a disposable server, `infra/qa-db`); creates one database
//! per run from the repo migrations as `allwin_owner` and drops it at the end. Without the URL
//! the tests are skipped, unless `ALLWIN_REQUIRE_DB_TESTS=1`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use chrono::{DateTime, TimeZone, Utc};
use gateway_application::events::GatewayEvent;
use gateway_application::ports::{
    CommandTransition, CommitOutcome, GatewayStore, NewCommand, OutboundMessage, ReportRow,
    StoreError, StoredCommand, SyncWrite, UnitOfWork, VisiblePlatformAccount,
};
use gateway_enterprise::command::{Command, CommandKind, CommandState, PositionEffect};
use gateway_enterprise::ids::{AccountId, CommandId, ConnectionId, EventId};
use gateway_enterprise::instrument::Instrument;
use gateway_enterprise::protocol::ProtocolVersion;
use gateway_enterprise::report::{
    ChildRole, FillFact, FillOperation, Origin, ReportFact, ReportStatus,
};
use gateway_enterprise::sync::{ReasonDetails, ReasonEntry, ReasonScope, SyncReason, SyncState};
use gateway_infra::PgGatewayStore;
use serde_json::json;
use shared_kernel::{Money, UserId};
use sqlx::AssertSqlSafe;
use uuid::Uuid;

mod common;

use common::setup;

fn t(ms: i64) -> DateTime<Utc> {
    Utc.timestamp_millis_opt(1_800_000_000_000 + ms).unwrap()
}

fn mnq() -> Instrument {
    Instrument::new("MNQ", "2026-12").unwrap()
}

fn new_command(
    user: UserId,
    account: AccountId,
    conn: ConnectionId,
    n: u128,
    kind: CommandKind,
) -> NewCommand {
    let effect = (kind == CommandKind::Place).then_some(PositionEffect::Open);
    let expires = effect.map(|_| t(5_000));
    let id = Uuid::from_u128(n);
    let payload = json!({ "command_id": id, "kind": "x" });
    let msg_id = Uuid::now_v7();
    NewCommand {
        stored: StoredCommand {
            command: Command::new_pending(
                CommandId::from_uuid(id),
                kind,
                effect,
                None,
                expires,
                t(0),
            )
            .unwrap(),
            instruction_id: Uuid::from_u128(1000 + n),
            step: if kind == CommandKind::Place {
                "place".into()
            } else {
                "flatten".into()
            },
            user_id: user,
            account_id: account,
            connection_id: conn,
            instrument: mnq(),
            msg_id,
            frame: gateway_application::wire::envelope(
                ProtocolVersion::V1_2,
                msg_id,
                "command",
                t(0),
                &payload,
            ),
        },
        payload,
    }
}

fn report(account: AccountId, conn: ConnectionId, fact: ReportFact) -> ReportRow {
    ReportRow {
        event_id: EventId::from_uuid(Uuid::now_v7()),
        account_id: account,
        connection_id: conn,
        instrument: Some(mnq()),
        fill_price: fact.fill.as_ref().map(|_| "21450.25".to_owned()),
        commission: None,
        commission_rate: None,
        commission_effective: fact
            .fill
            .as_ref()
            .map(|_| Money::new(rust_decimal::Decimal::ZERO)),
        commission_source: fact.fill.as_ref().map(|_| "none"),
        platform_order_id: None,
        avg_price_after: None,
        occurred_at: t(1),
        platform_time: None,
        payload: json!({}),
        fact,
    }
}

fn fill_fact(cmd: Option<u128>, fill_id: &str, op: FillOperation, after: i64) -> ReportFact {
    ReportFact {
        seq: 0,
        command_id: cmd.map(|c| CommandId::from_uuid(Uuid::from_u128(c))),
        origin: if cmd.is_some() {
            Origin::AllWin
        } else {
            Origin::External
        },
        child_role: Some(ChildRole::Entry),
        status: ReportStatus::Filled,
        reason_code: None,
        fill: Some(FillFact {
            fill_id: fill_id.into(),
            operation: op,
            quantity: 1,
        }),
        position_after: Some(after),
    }
}

#[tokio::test]
async fn store_end_to_end() {
    let Some(db) = setup().await else { return };
    let store = PgGatewayStore::new(db.app.clone());
    let user = db.user().await;
    let other = db.user().await;
    let conn = ConnectionId::from_uuid(Uuid::now_v7());
    let account = AccountId::from_uuid(Uuid::now_v7());

    // venue accounts: insert, then kind change is reported once.
    let acc = |kind: &str| {
        vec![VisiblePlatformAccount {
            platform_account: "Sim101".into(),
            kind: kind.into(),
            oco_location: "local".into(),
        }]
    };
    let v1 = store
        .upsert_venue_accounts(user, conn, &acc("simulation"), t(0))
        .await
        .unwrap();
    assert_eq!(v1[0].previous_kind, None);
    let v2 = store
        .upsert_venue_accounts(user, conn, &acc("live"), t(1))
        .await
        .unwrap();
    assert_eq!(
        (v2[0].id, v2[0].previous_kind.as_deref()),
        (v1[0].id, Some("simulation"))
    );
    assert_eq!(
        store
            .venue_account(user, v1[0].id)
            .await
            .unwrap()
            .unwrap()
            .0,
        conn
    );
    assert!(
        store
            .venue_account(other, v1[0].id)
            .await
            .unwrap()
            .is_none(),
        "cross-user read"
    );

    // commands: idempotent insert, open order, CAS mark_sent with its event, ack.
    let place = new_command(user, account, conn, 1, CommandKind::Place);
    let flat = new_command(user, account, conn, 2, CommandKind::Flatten);
    assert_eq!(
        store
            .insert_commands(user, &[place.clone(), flat.clone()])
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        store
            .insert_commands(user, std::slice::from_ref(&place))
            .await
            .unwrap(),
        0
    );
    let open = store.open_commands(user, conn).await.unwrap();
    assert_eq!(open.len(), 2);
    assert_eq!(
        open[0].frame, place.stored.frame,
        "stored frame is returned byte for byte"
    );
    assert!(
        store.open_commands(other, conn).await.unwrap().is_empty(),
        "cross-user read"
    );
    let ev = GatewayEvent {
        id: Uuid::now_v7(),
        event_type: "execution.command_sent",
        occurred_at: t(2),
        payload: json!({"actor":"system","details":{},"refs":{}}),
    };
    assert!(
        store
            .mark_sent(user, place.stored.command.id, t(2), vec![ev])
            .await
            .unwrap()
    );
    assert!(
        !store
            .mark_sent(user, place.stored.command.id, t(3), vec![])
            .await
            .unwrap(),
        "CAS"
    );
    store
        .ack(user, conn, &[place.stored.msg_id], t(4))
        .await
        .unwrap();
    let c = store
        .command(user, place.stored.command.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(c.command.state, CommandState::Acked);
    assert_eq!(store.in_flight(user, conn).await.unwrap().len(), 1);

    // outbound
    let payload = json!({"accounts": []});
    let out_id = Uuid::now_v7();
    let m = OutboundMessage {
        msg_id: out_id,
        user_id: user,
        connection_id: conn,
        kind: "limits".into(),
        frame: gateway_application::wire::envelope(
            ProtocolVersion::V1_3,
            out_id,
            "limits",
            t(5),
            &payload,
        ),
        payload,
        protocol_version: ProtocolVersion::V1_3,
        created_at: t(5),
    };
    store.enqueue_outbound(&m).await.unwrap();
    assert_eq!(store.unacked_outbound(user, conn).await.unwrap().len(), 1);
    store.ack(user, conn, &[m.msg_id], t(6)).await.unwrap();
    assert!(store.unacked_outbound(user, conn).await.unwrap().is_empty());

    // reports: receipt and event/fill dedupe, command transition, history.
    let work = |fact: ReportFact, msg: Uuid| UnitOfWork {
        receipt: Some((conn, msg)),
        report: Some(report(account, conn, fact)),
        ..UnitOfWork::default()
    };
    let msg = Uuid::now_v7();
    assert_eq!(
        store
            .commit(
                user,
                work(fill_fact(Some(1), "F1", FillOperation::Add, 1), msg)
            )
            .await
            .unwrap(),
        CommitOutcome::Committed
    );
    assert_eq!(
        store
            .commit(
                user,
                work(fill_fact(Some(1), "F1", FillOperation::Add, 1), msg)
            )
            .await
            .unwrap(),
        CommitOutcome::Duplicate
    );
    assert_eq!(
        store
            .commit(
                user,
                work(
                    fill_fact(Some(1), "F1", FillOperation::Add, 1),
                    Uuid::now_v7()
                )
            )
            .await
            .unwrap(),
        CommitOutcome::Duplicate
    );
    assert_eq!(
        store
            .commit(
                user,
                work(
                    fill_fact(Some(1), "F1", FillOperation::Amend, 1),
                    Uuid::now_v7()
                )
            )
            .await
            .unwrap(),
        CommitOutcome::Committed
    );
    assert!(store.fill_known(user, account, "F1").await.unwrap());
    assert_eq!(
        store.open_positions(user, account).await.unwrap(),
        vec![(mnq(), 1)]
    );
    store
        .commit(
            user,
            work(fill_fact(None, "F2", FillOperation::Add, 0), Uuid::now_v7()),
        )
        .await
        .unwrap();
    store
        .commit(
            user,
            work(
                fill_fact(None, "F3", FillOperation::Add, -2),
                Uuid::now_v7(),
            ),
        )
        .await
        .unwrap();
    let h = store.position_history(user, account, &mnq()).await.unwrap();
    assert_eq!(
        h.iter().map(|f| f.position_after).collect::<Vec<_>>(),
        vec![Some(0), Some(-2)],
        "since the last flat"
    );
    assert!(h[0].seq < h[1].seq);
    assert_eq!(
        store
            .command_reports(user, place.stored.command.id)
            .await
            .unwrap()
            .len(),
        2
    );
    assert!(
        store
            .open_positions(other, account)
            .await
            .unwrap()
            .is_empty(),
        "cross-user read"
    );

    let trans = UnitOfWork {
        commands: vec![CommandTransition {
            id: place.stored.command.id,
            from: CommandState::Sent,
            to: CommandState::Completed,
            at: t(7),
        }],
        ..UnitOfWork::default()
    };
    assert!(
        matches!(store.commit(user, trans).await, Err(StoreError::Conflict)),
        "state CAS (it is acked)"
    );

    // sync: first write creates the row, stale version conflicts, reasons round-trip.
    let mut sync = store.account_sync(user, account).await.unwrap();
    assert_eq!(sync.version(), 1);
    let entry = ReasonEntry::new(
        SyncReason::SnapshotMismatch,
        ReasonScope::Instrument(mnq()),
        t(8),
        ReasonDetails {
            expected_position: Some(1),
            reported_position: Some(2),
            ..ReasonDetails::default()
        },
    );
    sync.raise(entry);
    let w = UnitOfWork {
        sync: vec![SyncWrite {
            account_id: account,
            expected_version: 1,
            next: sync.clone(),
        }],
        ..UnitOfWork::default()
    };
    store.commit(user, w.clone()).await.unwrap();
    assert!(matches!(
        store.commit(user, w).await,
        Err(StoreError::Conflict)
    ));
    let back = store.account_sync(user, account).await.unwrap();
    assert_eq!((back.state(), back.version()), (SyncState::OutOfSync, 2));
    assert_eq!(back.active()[0].details.reported_position, Some(2));
    assert_eq!(
        back.active()[0].details.instrument_symbol.as_deref(),
        Some("MNQ")
    );
    assert_eq!(
        store.account_sync(other, account).await.unwrap().version(),
        1,
        "cross-user read"
    );

    let events: i64 = sqlx::query_scalar("SELECT count(*) FROM app.outbox WHERE user_id = $1")
        .bind(user.as_uuid())
        .fetch_one(&db.inspect)
        .await
        .unwrap();
    assert_eq!(events, 1);
    db.teardown().await;
}

#[tokio::test]
async fn constraints_and_grants() {
    let Some(db) = setup().await else { return };
    let store = PgGatewayStore::new(db.app.clone());
    let user = db.user().await;
    let conn = ConnectionId::from_uuid(Uuid::now_v7());
    let account = AccountId::from_uuid(Uuid::now_v7());

    // N4: frame and payload cannot drift; a token-shaped value in a frame is refused.
    let mut drift = new_command(user, account, conn, 3, CommandKind::Flatten);
    drift.payload = json!({"other": true});
    assert!(store.insert_commands(user, &[drift]).await.is_err());
    let mut token = new_command(user, account, conn, 4, CommandKind::Flatten);
    token.payload = json!({"x": format!("awd_{}_{}", "A".repeat(12), "b".repeat(43))});
    token.stored.frame = gateway_application::wire::envelope(
        ProtocolVersion::V1_2,
        token.stored.msg_id,
        "command",
        t(0),
        &token.payload,
    );
    assert!(store.insert_commands(user, &[token]).await.is_err());

    // C2: the envelope's msg_id and version must match the row.
    let mut wrong_id = new_command(user, account, conn, 6, CommandKind::Flatten);
    wrong_id.stored.msg_id = Uuid::now_v7();
    assert!(store.insert_commands(user, &[wrong_id]).await.is_err());
    let mut wrong_version = new_command(user, account, conn, 7, CommandKind::Flatten);
    wrong_version.stored.frame = gateway_application::wire::envelope(
        ProtocolVersion::V1_3,
        wrong_version.stored.msg_id,
        "command",
        t(0),
        &wrong_version.payload,
    );
    assert!(store.insert_commands(user, &[wrong_version]).await.is_err());
    let payload = json!({"accounts": []});
    let msg_id = Uuid::now_v7();
    let outbound = |frame_type: &str, frame_id: Uuid| OutboundMessage {
        msg_id,
        user_id: user,
        connection_id: conn,
        kind: "limits".into(),
        frame: gateway_application::wire::envelope(
            ProtocolVersion::V1_3,
            frame_id,
            frame_type,
            t(5),
            &payload,
        ),
        payload: payload.clone(),
        protocol_version: ProtocolVersion::V1_3,
        created_at: t(5),
    };
    assert!(
        store
            .enqueue_outbound(&outbound("account_state", msg_id))
            .await
            .is_err(),
        "type drift"
    );
    assert!(
        store
            .enqueue_outbound(&outbound("limits", Uuid::now_v7()))
            .await
            .is_err(),
        "msg_id drift"
    );
    store
        .enqueue_outbound(&outbound("limits", msg_id))
        .await
        .unwrap();

    // C1: no runtime DELETE except on account_sync_reason (42501 insufficient_privilege).
    for table in [
        "venue_account",
        "command",
        "outbound_message",
        "account_sync",
    ] {
        let err = sqlx::raw_sql(AssertSqlSafe(format!("DELETE FROM gateway.{table}")))
            .execute(&db.app)
            .await
            .unwrap_err();
        assert_eq!(
            err.as_database_error().and_then(|e| e.code()).as_deref(),
            Some("42501"),
            "{table}"
        );
    }

    // The worker can move delivery columns but never edit a frame or payload.
    store
        .insert_commands(
            user,
            &[new_command(user, account, conn, 5, CommandKind::Flatten)],
        )
        .await
        .unwrap();
    assert!(
        sqlx::query("UPDATE gateway.command SET state = 'sent', sent_at = now()")
            .execute(&db.worker)
            .await
            .is_ok()
    );
    assert!(
        sqlx::query("UPDATE gateway.command SET frame = 'x'")
            .execute(&db.worker)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("UPDATE gateway.command SET payload = '{}'")
            .execute(&db.worker)
            .await
            .is_err()
    );
    // Reports are append-only for the app role.
    assert!(
        sqlx::query("UPDATE gateway.execution_report SET status = 'filled'")
            .execute(&db.app)
            .await
            .is_err()
    );
    // Without a user context, RLS hides every row from the app role.
    let visible: i64 = sqlx::query_scalar("SELECT count(*) FROM gateway.command")
        .fetch_one(&db.app)
        .await
        .unwrap();
    assert_eq!(visible, 0);

    // One row per protocol enum value (DBA request, rev. 16.7): every value fits its CHECK.
    let roles = [
        Some(ChildRole::Entry),
        Some(ChildRole::Stop),
        Some(ChildRole::Target),
        Some(ChildRole::Exit),
        Some(ChildRole::Flatten),
        None,
    ];
    let statuses = [
        ReportStatus::Received,
        ReportStatus::Working,
        ReportStatus::PartiallyFilled,
        ReportStatus::Filled,
        ReportStatus::Cancelled,
        ReportStatus::Rejected,
        ReportStatus::Expired,
        ReportStatus::Duplicate,
        ReportStatus::Unknown,
    ];
    let origins = [Origin::AllWin, Origin::LocalEnforcement, Origin::External];
    let reasons = [
        Some("command.already_flat"),
        Some("order.live_consent_cancelled"),
        Some("a.b.c"),
        None,
    ];
    let ops = [
        FillOperation::Add,
        FillOperation::Amend,
        FillOperation::Remove,
    ];
    let mut n = 0;
    for (i, status) in statuses.iter().enumerate() {
        let fact = ReportFact {
            seq: 0,
            command_id: Some(CommandId::from_uuid(Uuid::now_v7())),
            origin: origins[i % origins.len()],
            child_role: roles[i % roles.len()],
            status: *status,
            reason_code: reasons[i % reasons.len()].map(Into::into),
            fill: Some(FillFact {
                fill_id: format!("E{i}"),
                operation: ops[i % ops.len()],
                quantity: 1,
            }),
            position_after: Some(0),
        };
        let w = UnitOfWork {
            report: Some(report(account, conn, fact)),
            ..UnitOfWork::default()
        };
        assert_eq!(
            store.commit(user, w).await.unwrap(),
            CommitOutcome::Committed,
            "{status:?}"
        );
        n += 1;
    }
    for role in roles {
        let fact = ReportFact {
            seq: 0,
            command_id: None,
            origin: Origin::External,
            child_role: role,
            status: ReportStatus::Working,
            reason_code: None,
            fill: None,
            position_after: None,
        };
        let w = UnitOfWork {
            report: Some(report(account, conn, fact)),
            ..UnitOfWork::default()
        };
        assert_eq!(
            store.commit(user, w).await.unwrap(),
            CommitOutcome::Committed,
            "{role:?}"
        );
        n += 1;
    }
    let stored: i64 = sqlx::query_scalar("SELECT count(*) FROM gateway.execution_report")
        .fetch_one(&db.inspect)
        .await
        .unwrap();
    assert_eq!(stored, n);
    db.teardown().await;
}

#[tokio::test]
async fn deferred_inbound_is_stored_once_and_read_only() {
    let Some(db) = setup().await else { return };
    let store = PgGatewayStore::new(db.app.clone());
    let user = db.user().await;
    let other = db.user().await;
    let conn = ConnectionId::from_uuid(Uuid::now_v7());
    let msg = Uuid::now_v7();
    let work = |m: Uuid| UnitOfWork {
        receipt: Some((conn, m)),
        deferred: Some(gateway_application::ports::DeferredInbound {
            msg_id: m,
            connection_id: conn,
            account_id: Some(AccountId::from_uuid(Uuid::now_v7())),
            kind: "risk_event".into(),
            protocol_version: ProtocolVersion::V1_3,
            payload: json!({"kind": "warning"}),
            received_at: t(0),
        }),
        ..UnitOfWork::default()
    };
    assert_eq!(
        store.commit(user, work(msg)).await.unwrap(),
        CommitOutcome::Committed
    );
    assert_eq!(
        store.commit(user, work(msg)).await.unwrap(),
        CommitOutcome::Duplicate,
        "retransmission"
    );
    // After the receipt purge (48 h), the primary key still dedupes.
    sqlx::query("DELETE FROM gateway.inbound_receipt")
        .execute(&db.inspect)
        .await
        .unwrap();
    assert_eq!(
        store.commit(user, work(msg)).await.unwrap(),
        CommitOutcome::Duplicate,
        "after receipt purge"
    );
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM gateway.deferred_inbound")
        .fetch_one(&db.inspect)
        .await
        .unwrap();
    assert_eq!(rows, 1);

    // The app role marks processed_at only; never edits the payload or deletes.
    let mut tx = db.app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    assert_eq!(
        sqlx::query("UPDATE gateway.deferred_inbound SET processed_at = now()")
            .execute(&mut *tx)
            .await
            .unwrap()
            .rows_affected(),
        1
    );
    tx.commit().await.unwrap();
    for sql in [
        "UPDATE gateway.deferred_inbound SET payload = '{}'",
        "DELETE FROM gateway.deferred_inbound",
    ] {
        let err = sqlx::raw_sql(AssertSqlSafe(sql))
            .execute(&db.app)
            .await
            .unwrap_err();
        assert_eq!(
            err.as_database_error().and_then(|e| e.code()).as_deref(),
            Some("42501"),
            "{sql}"
        );
    }
    // The worker reads pending rows and cannot write; another user sees nothing.
    let seen: i64 = sqlx::query_scalar("SELECT count(*) FROM gateway.deferred_inbound")
        .fetch_one(&db.worker)
        .await
        .unwrap();
    assert_eq!(seen, 1);
    assert!(
        sqlx::query("UPDATE gateway.deferred_inbound SET processed_at = now()")
            .execute(&db.worker)
            .await
            .is_err()
    );
    let mut tx = db.app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(other.as_uuid().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let cross: i64 = sqlx::query_scalar("SELECT count(*) FROM gateway.deferred_inbound")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(cross, 0, "cross-user read");
    tx.commit().await.unwrap();
    db.teardown().await;
}
