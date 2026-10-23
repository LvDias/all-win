//! One adapter frame after the handshake (ADR-0004 §3–§4, §9): validate, dispatch, and decide
//! what goes back (ack, error) and whether the session closes. The infra socket loop only
//! pumps frames through this.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::protocol::ProtocolVersion;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::intake::{HandleExecutionReport, IntakeError};
use crate::ports::GatewayStore;
use crate::session::Session;
use crate::snapshot::{HandlePositionSnapshot, SnapshotError};
use crate::wire::{self, Ack, Envelope, ExecutionReport, PositionSnapshot};

/// Validates a whole frame against `contracts/adapter-protocol/v1/messages.schema.json`.
pub trait FrameValidator: Send + Sync {
    fn validate(&self, frame: &Value) -> Result<(), String>;
}

/// WebSocket close codes (ADR-0004 §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseCode {
    /// Server restart, or an intake failure (reason `intake_retry`, Esquadro P1/X1).
    Normal = 4000,
    InvalidToken = 4001,
    Revoked = 4002,
    Outdated = 4003,
    Superseded = 4004,
    HeartbeatTimeout = 4005,
    ProtocolViolation = 4006,
}

impl CloseCode {
    pub const fn code(self) -> u16 {
        self as u16
    }
}

/// Sent by the adapter when the user pressed "Desconectar" (protocol 1.3).
pub const CLOSE_BY_USER: u16 = 4100;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrameOutcome {
    /// Frames to write now, in order.
    pub replies: Vec<String>,
    /// Close the session with this code and reason after writing the replies.
    pub close: Option<(CloseCode, &'static str)>,
    /// Run a delivery pass (a reconnect snapshot was evaluated).
    pub deliver: bool,
}

impl FrameOutcome {
    fn reply(frame: String) -> Self {
        Self {
            replies: vec![frame],
            ..Self::default()
        }
    }

    fn close(code: CloseCode, reason: &'static str) -> Self {
        Self {
            close: Some((code, reason)),
            ..Self::default()
        }
    }
}

pub fn ack_frame(version: ProtocolVersion, msg_ids: &[Uuid], now: DateTime<Utc>) -> String {
    wire::envelope(
        version,
        Uuid::now_v7(),
        "ack",
        now,
        &json!({ "msg_ids": msg_ids }),
    )
}

pub fn error_frame(
    version: ProtocolVersion,
    code: &str,
    ref_msg_id: Option<Uuid>,
    now: DateTime<Utc>,
) -> String {
    let mut payload = json!({ "code": code });
    if let Some(r) = ref_msg_id {
        payload["ref_msg_id"] = json!(r);
    }
    wire::envelope(version, Uuid::now_v7(), "error", now, &payload)
}

pub fn heartbeat_frame(version: ProtocolVersion, now: DateTime<Utc>) -> String {
    wire::envelope(version, Uuid::now_v7(), "heartbeat", now, &json!({}))
}

#[derive(Clone)]
pub struct HandleFrame {
    validator: Arc<dyn FrameValidator>,
    store: Arc<dyn GatewayStore>,
    intake: HandleExecutionReport,
    snapshot: HandlePositionSnapshot,
}

impl HandleFrame {
    pub fn new(
        validator: Arc<dyn FrameValidator>,
        store: Arc<dyn GatewayStore>,
        intake: HandleExecutionReport,
        snapshot: HandlePositionSnapshot,
    ) -> Self {
        Self {
            validator,
            store,
            intake,
            snapshot,
        }
    }

    pub async fn handle(&self, session: &Session, text: &str, now: DateTime<Utc>) -> FrameOutcome {
        let v = session.version;
        let Ok(value) = serde_json::from_str::<Value>(text) else {
            return FrameOutcome::close(CloseCode::ProtocolViolation, "not_json");
        };
        let ref_id = value
            .get("msg_id")
            .and_then(Value::as_str)
            .and_then(|s| Uuid::parse_str(s).ok());
        if self.validator.validate(&value).is_err() {
            // Not acked: a reliable frame stays in the adapter's outbox (§9), never executed or lost.
            return FrameOutcome::reply(error_frame(v, "protocol.invalid_message", ref_id, now));
        }
        let Ok(env) = serde_json::from_value::<Envelope>(value) else {
            return FrameOutcome::reply(error_frame(v, "protocol.invalid_message", ref_id, now));
        };
        // A frame from a newer minor than the session's is refused without ack (rollback, §9).
        let accepted =
            ProtocolVersion::parse(&env.protocol_version).is_ok_and(|e| v.accepts_envelope(e));
        if !accepted {
            return FrameOutcome::reply(error_frame(
                v,
                "protocol.unsupported_value",
                Some(env.msg_id),
                now,
            ));
        }
        match env.kind.as_str() {
            "heartbeat" => FrameOutcome::default(),
            "ack" => {
                let Ok(ack) = serde_json::from_value::<Ack>(env.payload) else {
                    return FrameOutcome::reply(error_frame(
                        v,
                        "protocol.invalid_message",
                        Some(env.msg_id),
                        now,
                    ));
                };
                session.acked(&ack.msg_ids);
                match self
                    .store
                    .ack(session.user_id, session.connection_id, &ack.msg_ids, now)
                    .await
                {
                    Ok(()) => FrameOutcome::default(),
                    // The ack is lost; the frames are retransmitted and acked again (idempotent).
                    Err(_) => FrameOutcome::close(CloseCode::Normal, "intake_retry"),
                }
            }
            "execution_report" => {
                let Ok(report) = serde_json::from_value::<ExecutionReport>(env.payload.clone())
                else {
                    return FrameOutcome::reply(error_frame(
                        v,
                        "protocol.invalid_message",
                        Some(env.msg_id),
                        now,
                    ));
                };
                match self
                    .intake
                    .handle(session, env.msg_id, &report, &env.payload, now)
                    .await
                {
                    Ok(_) => FrameOutcome::reply(ack_frame(v, &[env.msg_id], now)),
                    Err(IntakeError::Invalid(_)) => FrameOutcome::reply(error_frame(
                        v,
                        "protocol.invalid_message",
                        Some(env.msg_id),
                        now,
                    )),
                    Err(_) => FrameOutcome::close(CloseCode::Normal, "intake_retry"),
                }
            }
            "position_snapshot" => {
                let Ok(snapshot) = serde_json::from_value::<PositionSnapshot>(env.payload) else {
                    return FrameOutcome::reply(error_frame(
                        v,
                        "protocol.invalid_message",
                        Some(env.msg_id),
                        now,
                    ));
                };
                match self
                    .snapshot
                    .handle(session, env.msg_id, &snapshot, now)
                    .await
                {
                    Ok(_) => FrameOutcome {
                        replies: vec![ack_frame(v, &[env.msg_id], now)],
                        close: None,
                        deliver: true,
                    },
                    Err(SnapshotError::Invalid(_)) => FrameOutcome::reply(error_frame(
                        v,
                        "protocol.invalid_message",
                        Some(env.msg_id),
                        now,
                    )),
                    Err(_) => FrameOutcome::close(CloseCode::Normal, "intake_retry"),
                }
            }
            // Reliable A→C messages whose Core consumer belongs to a later slice (S13/S15 risk,
            // S18 notifications, S19 master input, S21 trades): not acked, so they stay in the
            // adapter's durable outbox until a Core version that stores them is deployed.
            "risk_event" | "instruction" | "trade_completed" | "notification_read" => {
                FrameOutcome::default()
            }
            // Latest-value-wins display data: nothing to store in S2.
            "risk_state" => FrameOutcome::default(),
            // `hello` after the handshake, or a C→A type sent by the adapter.
            _ => FrameOutcome::close(CloseCode::ProtocolViolation, "unexpected_type"),
        }
    }
}
