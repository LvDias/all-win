//! End to end over a loopback WebSocket: the real `/adapter/v1` server, the PostgreSQL store and
//! the protocol schema; the other modules' ports are fakes. Every frame the Core sends is
//! validated against the contract schema.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

mod common;

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use gateway_application::consume::{CommandsRequested, ConsumeCommandsRequested};
use gateway_application::frames::FrameValidator;
use gateway_enterprise::ids::{AccountId, ConnectionId};
use gateway_infra::ws::Control;
use gateway_infra::{GatewayRuntime, ProtocolSchema};
use serde_json::{Value, json};
use shared_kernel::UserId;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use uuid::Uuid;

use common::server::{self, TOKEN};

type Client = WebSocketStream<MaybeTlsStream<TcpStream>>;

struct Harness {
    url: String,
    runtime: Arc<GatewayRuntime>,
    schema: Arc<ProtocolSchema>,
    consume: ConsumeCommandsRequested,
    user: UserId,
    account: AccountId,
    connection: ConnectionId,
}

impl Harness {
    async fn connect(&self, token: Option<&str>) -> Client {
        let mut req = self.url.as_str().into_client_request().unwrap();
        if let Some(t) = token {
            req.headers_mut()
                .insert("authorization", format!("Bearer {t}").parse().unwrap());
        }
        tokio_tungstenite::connect_async(req).await.unwrap().0
    }

    fn check(&self, frame: &str) -> Value {
        let v: Value = serde_json::from_str(frame).unwrap();
        assert!(
            self.schema.validate(&v).is_ok(),
            "Core sent a schema-invalid frame: {frame}"
        );
        v
    }

    /// Next text frame from the Core, skipping heartbeats; `None` on close (with its code).
    async fn next(&self, c: &mut Client) -> Result<Value, Option<u16>> {
        self.next_raw(c).await.map(|(_, v)| v)
    }

    /// As `next`, with the exact text received (for byte-identical comparisons, Grosa L8).
    async fn next_raw(&self, c: &mut Client) -> Result<(String, Value), Option<u16>> {
        loop {
            let msg = tokio::time::timeout(Duration::from_secs(5), c.next())
                .await
                .expect("timed out");
            match msg {
                Some(Ok(Message::Text(t))) => {
                    let v = self.check(t.as_str());
                    if v["type"] != "heartbeat" {
                        return Ok((t.as_str().to_owned(), v));
                    }
                }
                Some(Ok(Message::Close(f))) => return Err(f.map(|f| u16::from(f.code))),
                Some(Ok(_)) => {}
                _ => return Err(None),
            }
        }
    }

    async fn hello(&self, c: &mut Client, versions: &[&str]) {
        let frame = json!({
            "protocol_version": "1.2", "msg_id": Uuid::now_v7(), "type": "hello",
            "sent_at": gateway_application::wire::timestamp(Utc::now()),
            "payload": {
                "instance_id": Uuid::from_u128(3),
                "adapter": {"type": "ninjatrader8", "version": "1.3.0", "build": "development"},
                "supported_protocol_versions": versions,
                "accounts": [{"platform_account": "Sim101", "kind": "simulation", "oco_location": "local"}],
            },
        });
        c.send(Message::Text(frame.to_string().into()))
            .await
            .unwrap();
    }

    async fn snapshot(&self, c: &mut Client, version: &str) -> Uuid {
        let msg_id = Uuid::now_v7();
        let frame = json!({
            "protocol_version": version, "msg_id": msg_id, "type": "position_snapshot",
            "sent_at": gateway_application::wire::timestamp(Utc::now()),
            "payload": {"trigger": "reconnect", "taken_at": gateway_application::wire::timestamp(Utc::now()),
                "accounts": [{"account_id": self.account, "platform_account": "Sim101", "connected": true,
                              "positions": [], "working_orders": []}]},
        });
        c.send(Message::Text(frame.to_string().into()))
            .await
            .unwrap();
        msg_id
    }

    async fn request_entry(&self) {
        let ev: CommandsRequested = serde_json::from_value(json!({
            "instruction_id": Uuid::now_v7(), "account_id": self.account,
            "created_at": gateway_application::wire::timestamp(Utc::now()),
            "commands": [{
                "command_id": Uuid::now_v7(), "kind": "place", "step": "place", "position_effect": "open",
                "expires_at": gateway_application::wire::timestamp(Utc::now() + chrono::Duration::seconds(60)),
                "instrument": {"symbol": "MNQ", "contract_month": "2026-12"},
                "body": {"side": "buy", "quantity": 1, "order": {"type": "market"}, "position_effect": "open",
                         "protection": {"mode": "individual", "stop_ticks": 40, "target_ticks": 80}},
            }],
        }))
        .unwrap();
        self.consume.handle(self.user, &ev).await.unwrap();
        self.runtime.signal(self.connection, Control::Wake);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn adapter_session_end_to_end() {
    let Some(db) = common::setup().await else {
        return;
    };
    let srv = server::start(&db).await;
    let (user, account, connection) = (srv.user, srv.account, srv.connection);
    let h = Harness {
        url: srv.url.clone(),
        runtime: srv.runtime.clone(),
        schema: srv.schema.clone(),
        consume: srv.consume.clone(),
        user,
        account,
        connection,
    };

    // 4001 without a valid token.
    let mut c = h.connect(None).await;
    assert_eq!(h.next(&mut c).await, Err(Some(4001)));
    let mut c = h.connect(Some("awd_wrong")).await;
    assert_eq!(h.next(&mut c).await, Err(Some(4001)));

    // 4003 + protocol.unsupported_version below 1.2.
    let mut c = h.connect(Some(TOKEN)).await;
    h.hello(&mut c, &["1.0", "1.1"]).await;
    assert_eq!(
        h.next(&mut c).await.unwrap()["payload"]["code"],
        "protocol.unsupported_version"
    );
    assert_eq!(h.next(&mut c).await, Err(Some(4003)));

    // Handshake at 1.3, then no command before the reconnect snapshot.
    let mut c = h.connect(Some(TOKEN)).await;
    h.hello(&mut c, &["1.2", "1.3"]).await;
    let welcome = h.next(&mut c).await.unwrap();
    assert_eq!(
        (
            welcome["type"].as_str(),
            welcome["payload"]["protocol_version"].as_str()
        ),
        (Some("welcome"), Some("1.3"))
    );
    assert_eq!(
        welcome["payload"]["accounts"][0]["account_id"],
        json!(account)
    );
    h.request_entry().await;
    assert!(
        tokio::time::timeout(Duration::from_millis(800), h.next(&mut c))
            .await
            .is_err(),
        "nothing before the snapshot"
    );
    let snap_id = h.snapshot(&mut c, "1.3").await;
    let ack = h.next(&mut c).await.unwrap();
    assert_eq!(ack["payload"]["msg_ids"], json!([snap_id]));
    let (first_bytes, command) = h.next_raw(&mut c).await.unwrap();
    assert_eq!(
        (
            command["type"].as_str(),
            command["protocol_version"].as_str()
        ),
        (Some("command"), Some("1.2"))
    );

    // Not acked; the connection drops. A new session retransmits the same frame. Wait until the
    // Core has seen the drop: reconnecting earlier would supersede the old session instead of
    // ending it as `network` (a race under load, found with O2's parallel test).
    drop(c);
    wait_until(|| h.runtime.sessions.get(connection).is_none()).await;
    let mut c = h.connect(Some(TOKEN)).await;
    h.hello(&mut c, &["1.3"]).await;
    h.next(&mut c).await.unwrap();
    h.snapshot(&mut c, "1.3").await;
    h.next(&mut c).await.unwrap();
    let (again_bytes, again) = h.next_raw(&mut c).await.unwrap();
    assert_eq!(
        again_bytes, first_bytes,
        "byte-identical retransmission (raw text)"
    );
    let msg_id = again["msg_id"].clone();
    let ack = json!({"protocol_version": "1.3", "msg_id": Uuid::now_v7(), "type": "ack",
        "sent_at": gateway_application::wire::timestamp(Utc::now()), "payload": {"msg_ids": [msg_id]}});
    c.send(Message::Text(ack.to_string().into())).await.unwrap();
    // The ack is processed before the next session supersedes this one (same race under load).
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    loop {
        let state: String = sqlx::query_scalar("SELECT state FROM gateway.command")
            .fetch_one(&db.inspect)
            .await
            .unwrap();
        if state == "acked" || tokio::time::Instant::now() >= deadline {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // A second session for the same connection supersedes the first (4004).
    let mut c2 = h.connect(Some(TOKEN)).await;
    h.hello(&mut c2, &["1.3"]).await;
    assert_eq!(h.next(&mut c2).await.unwrap()["type"], "welcome");
    assert_eq!(h.next(&mut c).await, Err(Some(4004)));

    // "Desconectar" in the plugin (4100) ends the session as by-user.
    c2.close(Some(CloseFrame {
        code: CloseCode::from(4100),
        reason: "".into(),
    }))
    .await
    .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let state: String = sqlx::query_scalar("SELECT state FROM gateway.command")
        .fetch_one(&db.inspect)
        .await
        .unwrap();
    assert_eq!(state, "acked");
    // The by-user disconnect is recorded after the socket closes: wait for it (Sonda M-S2-1:
    // reading after a fixed sleep raced it under load). Causal order is the outbox `seq`.
    let reasons = session_reasons_until_user(&db, 1).await;
    // outdated; then connected, network drop, connected, (superseded: nothing), connected, user.
    assert_eq!(
        reasons.first().cloned().flatten().as_deref(),
        Some("outdated")
    );
    assert_eq!(reasons.last().cloned().flatten().as_deref(), Some("user"));
    assert_eq!(
        reasons
            .iter()
            .filter(|r| r.as_deref() == Some("network"))
            .count(),
        1
    );
    db.teardown().await;
}

fn harness(srv: &server::Server) -> Harness {
    Harness {
        url: srv.url.clone(),
        runtime: srv.runtime.clone(),
        schema: srv.schema.clone(),
        consume: srv.consume.clone(),
        user: srv.user,
        account: srv.account,
        connection: srv.connection,
    }
}

/// Crivo g1/g2: failed authentications are charged to the client IP; over budget even the
/// valid token gets the same 4001, with no detail.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failed_authentications_are_throttled_per_ip_with_the_same_refusal() {
    let Some(db) = common::setup().await else {
        return;
    };
    let srv = server::start(&db).await;
    let h = harness(&srv);
    for _ in 0..gateway_infra::ws::DEFAULT_FAILURES_PER_MIN {
        let mut c = h.connect(Some("awd_wrong")).await;
        assert_eq!(h.next(&mut c).await, Err(Some(4001)));
    }
    let mut c = h.connect(Some(TOKEN)).await;
    assert_eq!(h.next(&mut c).await, Err(Some(4001)), "throttled");
    db.teardown().await;
}

/// G1 (Guarita): a frame over the protocol's 256 KiB closes the socket; nothing is stored.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_oversized_frame_closes_the_session() {
    let Some(db) = common::setup().await else {
        return;
    };
    let srv = server::start(&db).await;
    let h = harness(&srv);
    let mut c = h.connect(Some(TOKEN)).await;
    h.hello(&mut c, &["1.2"]).await;
    let welcome = h.next(&mut c).await.unwrap();
    assert_eq!(welcome["type"], "welcome");
    let big = "x".repeat(256 * 1024 + 1);
    let _ = c.send(Message::Text(big.into())).await;
    let end = loop {
        match h.next(&mut c).await {
            Ok(_) => continue,
            Err(code) => break code,
        }
    };
    assert!(
        matches!(end, None | Some(1009)),
        "closed for size, got {end:?}"
    );
    let receipts: i64 =
        sqlx::query_scalar("SELECT count(*) FROM gateway.inbound_receipt WHERE connection_id = $1")
            .bind(srv.connection.as_uuid())
            .fetch_one(&db.inspect)
            .await
            .unwrap();
    assert_eq!(receipts, 0);
    db.teardown().await;
}

/// Polls `done` every 20 ms for up to 10 s.
async fn wait_until(done: impl Fn() -> bool) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while !done() && tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

/// `disconnect_reason` of every `gateway.connection_session_changed`, in outbox order, once at
/// least `users` by-user disconnects are recorded (or after 10 s).
async fn session_reasons_until_user(db: &common::Db, users: usize) -> Vec<Option<String>> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    loop {
        let reasons: Vec<Option<String>> = sqlx::query_scalar(
            "SELECT payload->>'disconnect_reason' FROM app.outbox WHERE event_type = 'gateway.connection_session_changed' ORDER BY seq",
        )
        .fetch_all(&db.inspect)
        .await
        .unwrap();
        let by_user = reasons
            .iter()
            .filter(|r| r.as_deref() == Some("user"))
            .count();
        if by_user >= users || tokio::time::Instant::now() >= deadline {
            return reasons;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// Sonda M-S2-1: the plugin's "Desconectar" (close 4100) is always a by-user end, never a
/// network drop, however quickly sessions follow each other.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn close_4100_is_always_classified_as_by_user() {
    let Some(db) = common::setup().await else {
        return;
    };
    let srv = server::start(&db).await;
    let h = harness(&srv);
    const ROUNDS: usize = 15;
    for round in 1..=ROUNDS {
        let mut c = h.connect(Some(TOKEN)).await;
        h.hello(&mut c, &["1.3"]).await;
        assert_eq!(h.next(&mut c).await.unwrap()["type"], "welcome");
        c.close(Some(CloseFrame {
            code: CloseCode::from(4100),
            reason: "".into(),
        }))
        .await
        .unwrap();
        // Recorded before the next session starts (a quicker reconnect would supersede it).
        session_reasons_until_user(&db, round).await;
    }
    let reasons = session_reasons_until_user(&db, ROUNDS).await;
    let ends: Vec<&str> = reasons.iter().filter_map(|r| r.as_deref()).collect();
    assert_eq!(ends.len(), ROUNDS, "{ends:?}");
    assert!(ends.iter().all(|r| *r == "user"), "{ends:?}");
    db.teardown().await;
}

/// O2 / ADR-0004 §5: a reconnect window that never closes (e.g. the snapshot was refused as
/// invalid) holds every command; the Core closes with 4000 `snapshot_timeout` so the adapter
/// resends, counts consecutive closes per connection, and resets once a window closes.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_unevaluated_reconnect_window_closes_with_4000_and_is_counted() {
    let Some(db) = common::setup().await else {
        return;
    };
    let settings = gateway_application::session::SessionSettings {
        snapshot_warn_ms: 200,
        snapshot_timeout_ms: 1_000,
        ..Default::default()
    };
    let srv = server::start_with(&db, settings).await;
    let h = harness(&srv);
    for n in 1..=3 {
        let mut c = h.connect(Some(TOKEN)).await;
        h.hello(&mut c, &["1.2"]).await;
        assert_eq!(h.next(&mut c).await.unwrap()["type"], "welcome");
        let end = loop {
            match h.next(&mut c).await {
                Ok(_) => continue,
                Err(code) => break code,
            }
        };
        assert_eq!(end, Some(4000), "closed for a resend");
        assert_eq!(
            gateway_infra::ws::snapshot_timeouts(&srv.runtime, srv.connection),
            n
        );
    }
    // A snapshot evaluated in time closes the window and resets the count.
    let mut c = h.connect(Some(TOKEN)).await;
    h.hello(&mut c, &["1.2"]).await;
    assert_eq!(h.next(&mut c).await.unwrap()["type"], "welcome");
    h.snapshot(&mut c, "1.2").await;
    assert_eq!(h.next(&mut c).await.unwrap()["type"], "ack");
    tokio::time::sleep(Duration::from_millis(1_500)).await;
    assert_eq!(
        gateway_infra::ws::snapshot_timeouts(&srv.runtime, srv.connection),
        0
    );
    db.teardown().await;
}
