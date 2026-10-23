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
        loop {
            let msg = tokio::time::timeout(Duration::from_secs(5), c.next())
                .await
                .expect("timed out");
            match msg {
                Some(Ok(Message::Text(t))) => {
                    let v = self.check(t.as_str());
                    if v["type"] != "heartbeat" {
                        return Ok(v);
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
    let command = h.next(&mut c).await.unwrap();
    assert_eq!(
        (
            command["type"].as_str(),
            command["protocol_version"].as_str()
        ),
        (Some("command"), Some("1.2"))
    );
    let first_bytes = command.to_string();

    // Not acked; the connection drops. A new session retransmits the same frame.
    drop(c);
    tokio::time::sleep(Duration::from_millis(300)).await;
    let mut c = h.connect(Some(TOKEN)).await;
    h.hello(&mut c, &["1.3"]).await;
    h.next(&mut c).await.unwrap();
    h.snapshot(&mut c, "1.3").await;
    h.next(&mut c).await.unwrap();
    let again = h.next(&mut c).await.unwrap();
    assert_eq!(
        again.to_string(),
        first_bytes,
        "byte-identical retransmission"
    );
    let msg_id = again["msg_id"].clone();
    let ack = json!({"protocol_version": "1.3", "msg_id": Uuid::now_v7(), "type": "ack",
        "sent_at": gateway_application::wire::timestamp(Utc::now()), "payload": {"msg_ids": [msg_id]}});
    c.send(Message::Text(ack.to_string().into())).await.unwrap();

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
    let reasons: Vec<Option<String>> = sqlx::query_scalar(
        "SELECT payload->>'disconnect_reason' FROM app.outbox WHERE event_type = 'gateway.connection_session_changed' ORDER BY occurred_at",
    )
    .fetch_all(&db.inspect)
    .await
    .unwrap();
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
