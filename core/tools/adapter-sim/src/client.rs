//! The adapter side of a session (ADR-0004 §3–§4): hello, welcome, retransmission of the unacked
//! outbox in original order, the reconnect snapshot, then commands → ack → execution reports.
//! The outbox lives in memory: it survives reconnects, not a process restart.

use std::time::Duration;

use anyhow::{Context, anyhow};
use chrono::{SecondsFormat, Utc};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use uuid::Uuid;

use crate::venue::Venue;

#[derive(Debug, Clone)]
pub struct SimConfig {
    pub url: String,
    /// Device token; never logged.
    pub token: String,
    pub instance_id: Uuid,
    pub versions: Vec<String>,
    /// `simulation` or `playback` (the sim never reports a live account).
    pub kind: String,
    pub oco_location: String,
    /// Do not ack commands (the Core retransmits them).
    pub no_ack: bool,
    /// Drop the connection (no close frame) after this many commands, to test reconnects.
    pub drop_after_commands: Option<usize>,
    /// Close with 4100 ("Desconectar") after this many commands.
    pub user_disconnect_after_commands: Option<usize>,
}

impl SimConfig {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            instance_id: Uuid::now_v7(),
            versions: vec!["1.2".into(), "1.3".into()],
            kind: "simulation".into(),
            oco_location: "local".into(),
            no_ack: false,
            drop_after_commands: None,
            user_disconnect_after_commands: None,
        }
    }
}

/// How a session ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimEnd {
    /// The Core closed with this code.
    Closed(Option<u16>),
    /// The sim dropped the socket on purpose.
    Dropped,
    /// The sim closed with 4100.
    UserDisconnected,
}

pub struct SimClient {
    pub venue: Venue,
    /// Unacked reliable frames, in original order (byte-identical on retransmission).
    outbox: Vec<(Uuid, String)>,
    pub commands_seen: usize,
    pub negotiated: Option<String>,
}

fn ts() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn envelope(version: &str, kind: &str, payload: Value) -> (Uuid, String) {
    let id = Uuid::now_v7();
    let frame = json!({ "protocol_version": version, "msg_id": id, "type": kind, "sent_at": ts(), "payload": payload });
    (id, frame.to_string())
}

impl SimClient {
    pub fn new(venue: Venue) -> Self {
        Self {
            venue,
            outbox: Vec::new(),
            commands_seen: 0,
            negotiated: None,
        }
    }

    pub fn unacked(&self) -> usize {
        self.outbox.len()
    }

    /// One session, until the Core closes it, the socket drops, or a scripted disconnect.
    pub async fn run_session(&mut self, cfg: &SimConfig) -> anyhow::Result<SimEnd> {
        let mut req = cfg.url.as_str().into_client_request().context("url")?;
        req.headers_mut().insert(
            "authorization",
            format!("Bearer {}", cfg.token)
                .parse()
                .context("token header")?,
        );
        let (mut ws, _) = tokio_tungstenite::connect_async(req)
            .await
            .context("connect")?;

        let hello_version = cfg.versions.last().cloned().unwrap_or_else(|| "1.2".into());
        let (_, hello) = envelope(
            &hello_version,
            "hello",
            json!({
                "instance_id": cfg.instance_id,
                "adapter": {"type": "adapter_sim", "version": env!("CARGO_PKG_VERSION"), "build": "development"},
                "supported_protocol_versions": cfg.versions,
                "accounts": [{"platform_account": self.venue.platform_account, "kind": cfg.kind, "oco_location": cfg.oco_location}],
                "pending_outbound": self.outbox.len(),
            }),
        );
        ws.send(Message::Text(hello.into())).await?;

        let heartbeat = loop {
            let Some(msg) = ws.next().await else {
                return Ok(SimEnd::Closed(None));
            };
            match msg? {
                Message::Text(t) => {
                    let v: Value = serde_json::from_str(t.as_str())?;
                    match v["type"].as_str() {
                        Some("welcome") => {
                            let version = v["payload"]["protocol_version"]
                                .as_str()
                                .unwrap_or("1.2")
                                .to_owned();
                            let heartbeat = Duration::from_millis(
                                v["payload"]["heartbeat_interval_ms"]
                                    .as_u64()
                                    .unwrap_or(10_000),
                            );
                            if let Some(a) = v["payload"]["accounts"].as_array().and_then(|a| {
                                a.iter().find(|a| {
                                    a["platform_account"] == self.venue.platform_account.as_str()
                                })
                            }) && let Some(id) = a["account_id"]
                                .as_str()
                                .and_then(|s| Uuid::parse_str(s).ok())
                            {
                                self.venue.account_id = id;
                            }
                            tracing::info!(version = %version, "welcome");
                            self.negotiated = Some(version);
                            break heartbeat;
                        }
                        Some("error") => {
                            tracing::warn!(code = %v["payload"]["code"], "error before welcome")
                        }
                        _ => return Err(anyhow!("unexpected frame before welcome")),
                    }
                }
                Message::Close(f) => return Ok(SimEnd::Closed(f.map(|f| u16::from(f.code)))),
                _ => {}
            }
        };
        let version = self.negotiated.clone().unwrap_or_else(|| "1.2".into());

        // §3 step 3: retransmit the outbox in original order, then the reconnect snapshot.
        for (_, frame) in &self.outbox {
            ws.send(Message::Text(frame.clone().into())).await?;
        }
        let snapshot = envelope(
            &version,
            "position_snapshot",
            json!({
                "trigger": "reconnect", "taken_at": ts(), "accounts": [self.venue.snapshot_account()],
            }),
        );
        ws.send(Message::Text(snapshot.1.clone().into())).await?;
        self.outbox.push(snapshot);

        let mut tick = tokio::time::interval(heartbeat);
        tick.tick().await;
        loop {
            tokio::select! {
                msg = ws.next() => {
                    let Some(msg) = msg else { return Ok(SimEnd::Closed(None)) };
                    match msg? {
                        Message::Text(t) => {
                            let v: Value = serde_json::from_str(t.as_str())?;
                            let msg_id = v["msg_id"].clone();
                            match v["type"].as_str() {
                                Some("ack") => {
                                    let ids: Vec<String> = v["payload"]["msg_ids"].as_array().cloned().unwrap_or_default()
                                        .iter().filter_map(|x| x.as_str().map(str::to_owned)).collect();
                                    self.outbox.retain(|(id, _)| !ids.contains(&id.to_string()));
                                }
                                Some("command") => {
                                    self.commands_seen += 1;
                                    if !cfg.no_ack {
                                        let (_, ack) = envelope(&version, "ack", json!({ "msg_ids": [msg_id] }));
                                        ws.send(Message::Text(ack.into())).await?;
                                    }
                                    for report in self.venue.on_command(&v["payload"], Utc::now()) {
                                        let rep = envelope(&version, "execution_report", report);
                                        ws.send(Message::Text(rep.1.clone().into())).await?;
                                        self.outbox.push(rep);
                                    }
                                    if cfg.drop_after_commands.is_some_and(|n| self.commands_seen >= n) {
                                        return Ok(SimEnd::Dropped);
                                    }
                                    if cfg.user_disconnect_after_commands.is_some_and(|n| self.commands_seen >= n) {
                                        ws.close(Some(CloseFrame { code: CloseCode::from(4100), reason: "".into() })).await?;
                                        return Ok(SimEnd::UserDisconnected);
                                    }
                                }
                                // Reliable C→A messages: ack (limits are not enforced by the sim).
                                Some("limits" | "instruction_result" | "notification") => {
                                    let (_, ack) = envelope(&version, "ack", json!({ "msg_ids": [msg_id] }));
                                    ws.send(Message::Text(ack.into())).await?;
                                }
                                Some("error") => tracing::warn!(code = %v["payload"]["code"], "error from Core"),
                                _ => {}
                            }
                        }
                        Message::Close(f) => return Ok(SimEnd::Closed(f.map(|f| u16::from(f.code)))),
                        _ => {}
                    }
                }
                _ = tick.tick() => {
                    let (_, hb) = envelope(&version, "heartbeat", json!({}));
                    ws.send(Message::Text(hb.into())).await?;
                }
            }
        }
    }
}
