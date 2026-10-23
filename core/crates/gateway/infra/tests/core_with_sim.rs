//! The Core gateway against `adapter-sim` over loopback WebSocket: the M1 path (entry → fill +
//! bracket → flatten → orphan cancel), a dropped connection with unacked reports, and dedupe of
//! their retransmission.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

mod common;

use std::time::Duration;

use adapter_sim::{Script, SimClient, SimConfig, SimEnd, Venue};
use chrono::Utc;
use gateway_application::consume::CommandsRequested;
use gateway_infra::ws::Control;
use serde_json::json;
use uuid::Uuid;

use common::server::{self, Server, TOKEN};

async fn request(srv: &Server, commands: serde_json::Value) {
    let ev: CommandsRequested = serde_json::from_value(json!({
        "instruction_id": Uuid::now_v7(), "account_id": srv.account,
        "created_at": gateway_application::wire::timestamp(Utc::now()), "commands": commands,
    }))
    .unwrap();
    srv.consume.handle(srv.user, &ev).await.unwrap();
    srv.runtime.signal(srv.connection, Control::Wake);
}

fn entry(id: Uuid) -> serde_json::Value {
    json!({ "command_id": id, "kind": "place", "step": "place", "position_effect": "open",
        "expires_at": gateway_application::wire::timestamp(Utc::now() + chrono::Duration::seconds(30)),
        "instrument": {"symbol": "MNQ", "contract_month": "2026-12"},
        "body": {"side": "buy", "quantity": 1, "order": {"type": "market"}, "position_effect": "open",
                 "protection": {"mode": "individual", "stop_ticks": 40, "target_ticks": 80}} })
}

fn flatten(id: Uuid) -> serde_json::Value {
    json!({ "command_id": id, "kind": "flatten", "step": "flatten",
        "instrument": {"symbol": "MNQ", "contract_month": "2026-12"}, "body": {"reason": "close"} })
}

async fn wait_for(db: &common::Db, sql: &'static str, want: i64) {
    for _ in 0..50 {
        let n: i64 = sqlx::query_scalar(sql)
            .fetch_one(&db.inspect)
            .await
            .unwrap();
        if n == want {
            return;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let n: i64 = sqlx::query_scalar(sql)
        .fetch_one(&db.inspect)
        .await
        .unwrap();
    assert_eq!(n, want, "{sql}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn m1_path_against_the_sim() {
    let Some(db) = common::setup().await else {
        return;
    };
    let srv = server::start(&db).await;
    let cfg = SimConfig::new(srv.url.clone(), TOKEN);
    let sim = tokio::spawn({
        let cfg = cfg.clone();
        async move {
            let mut c = SimClient::new(Venue::new(Uuid::nil(), "Sim101", Script::default()));
            c.run_session(&cfg).await
        }
    });
    tokio::time::sleep(Duration::from_millis(500)).await;

    let open = Uuid::now_v7();
    request(&srv, json!([entry(open)])).await;
    // received, filled (position 1), stop working, target working.
    wait_for(&db, "SELECT count(*) FROM gateway.execution_report", 4).await;
    let state: String = sqlx::query_scalar("SELECT state FROM gateway.command WHERE id = $1")
        .bind(open)
        .fetch_one(&db.inspect)
        .await
        .unwrap();
    assert_eq!(state, "completed");

    request(&srv, json!([flatten(Uuid::now_v7())])).await;
    // + flatten filled (position 0) + 2 orphan cancels.
    wait_for(&db, "SELECT count(*) FROM gateway.execution_report", 7).await;
    wait_for(
        &db,
        "SELECT count(*) FROM gateway.command WHERE state = 'completed'",
        2,
    )
    .await;
    let flagged: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM gateway.account_sync WHERE sync_state = 'out_of_sync'",
    )
    .fetch_one(&db.inspect)
    .await
    .unwrap();
    assert_eq!(flagged, 0, "a clean M1 path never flags the account");
    for code in [
        "gateway.execution_reported",
        "execution.command_sent",
        "execution.filled",
        "execution.cancelled",
    ] {
        let n: i64 = sqlx::query_scalar("SELECT count(*) FROM app.outbox WHERE event_type = $1")
            .bind(code)
            .fetch_one(&db.inspect)
            .await
            .unwrap();
        assert!(n > 0, "{code}");
    }
    sim.abort();
    db.teardown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn retransmitted_reports_after_a_drop_are_deduplicated() {
    let Some(db) = common::setup().await else {
        return;
    };
    let srv = server::start(&db).await;
    let mut cfg = SimConfig::new(srv.url.clone(), TOKEN);
    cfg.drop_after_commands = Some(1);
    let mut client = SimClient::new(Venue::new(Uuid::nil(), "Sim101", Script::default()));
    let session = tokio::spawn(async move {
        let end = client.run_session(&cfg).await.unwrap();
        (client, cfg, end)
    });
    tokio::time::sleep(Duration::from_millis(500)).await;
    request(&srv, json!([entry(Uuid::now_v7())])).await;
    let (mut client, mut cfg, end) = session.await.unwrap();
    assert_eq!(end, SimEnd::Dropped);
    // The sim dropped right after sending; its reports may be unacked. Reconnect: the outbox is
    // retransmitted in order and the Core dedupes it (msg_id / event_id / fill).
    cfg.drop_after_commands = None;
    let again = tokio::spawn(async move { client.run_session(&cfg).await });
    wait_for(&db, "SELECT count(*) FROM gateway.execution_report", 4).await;
    tokio::time::sleep(Duration::from_millis(500)).await;
    wait_for(&db, "SELECT count(*) FROM gateway.execution_report", 4).await;
    let fills: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM gateway.execution_report WHERE fill_id IS NOT NULL",
    )
    .fetch_one(&db.inspect)
    .await
    .unwrap();
    assert_eq!(fills, 1);
    again.abort();
    db.teardown().await;
}
