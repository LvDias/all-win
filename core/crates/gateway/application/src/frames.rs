//! One adapter frame after the handshake (ADR-0004 §3–§4, §9): validate, dispatch, and decide
//! what goes back (ack, error) and whether the session closes. The infra socket loop only
//! pumps frames through this.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::command::CommandState;
use gateway_enterprise::protocol::ProtocolVersion;
use gateway_enterprise::sync::{ReasonDetails, SyncReason};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::intake::{HandleExecutionReport, IntakeError};
use gateway_enterprise::ids::AccountId;

use crate::deliver::{Finish, finish};
use crate::ports::{DeferredInbound, GatewayStore, StoreError, UnitOfWork};
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
    /// An adapter `error` to log and alert on (code and ref only, never `detail`).
    pub alert: Option<AdapterAlert>,
}

/// Operational alert: the adapter refused a Core frame (a Core bug, ADR-0004 §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterAlert {
    pub code: String,
    pub ref_msg_id: Option<Uuid>,
    /// The ref is a Core command (now `failed`, account flagged).
    pub command: bool,
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
        // §9 rollback rule (Grosa M5): a frame from another minor is accepted when its type exists
        // in the negotiated version; the full schema has already validated its payload.
        let known_major =
            ProtocolVersion::parse(&env.protocol_version).is_ok_and(|e| e.same_major(v));
        if !known_major || !type_exists(&env.kind, v) {
            return FrameOutcome::reply(error_frame(
                v,
                "protocol.unsupported_value",
                Some(env.msg_id),
                now,
            ));
        }
        let invalid =
            |id| FrameOutcome::reply(error_frame(v, "protocol.invalid_message", Some(id), now));
        match env.kind.as_str() {
            "heartbeat" | "risk_state" => FrameOutcome::default(),
            "error" => self.adapter_error(session, &env, now).await,
            "ack" => {
                let Ok(ack) = serde_json::from_value::<Ack>(env.payload) else {
                    return invalid(env.msg_id);
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
            "execution_report" | "position_snapshot" | "risk_event" | "instruction"
            | "trade_completed" | "notification_read" => self.reliable(session, env, now).await,
            // `hello` after the handshake, or a C→A type sent by the adapter.
            _ => FrameOutcome::close(CloseCode::ProtocolViolation, "unexpected_type"),
        }
    }

    /// Reliable A→C frames. A known `(connection_id, msg_id)` is acked at once, before any
    /// mapping check, so a retransmission after a lost ack never loops (Grosa M4).
    async fn reliable(&self, session: &Session, env: Envelope, now: DateTime<Utc>) -> FrameOutcome {
        let v = session.version;
        let ack = || FrameOutcome::reply(ack_frame(v, &[env.msg_id], now));
        let invalid = || {
            FrameOutcome::reply(error_frame(
                v,
                "protocol.invalid_message",
                Some(env.msg_id),
                now,
            ))
        };
        match self
            .store
            .receipt_known(session.user_id, session.connection_id, env.msg_id)
            .await
        {
            Ok(true) => return ack(),
            Ok(false) => {}
            Err(_) => return FrameOutcome::close(CloseCode::Normal, "intake_retry"),
        }
        match env.kind.as_str() {
            "execution_report" => {
                let Ok(report) = serde_json::from_value::<ExecutionReport>(env.payload.clone())
                else {
                    return invalid();
                };
                match self
                    .intake
                    .handle(session, env.msg_id, &report, &env.payload, now)
                    .await
                {
                    Ok(_) => ack(),
                    Err(IntakeError::Invalid(_) | IntakeError::Store(StoreError::Rejected(_))) => {
                        invalid()
                    }
                    Err(_) => FrameOutcome::close(CloseCode::Normal, "intake_retry"),
                }
            }
            "position_snapshot" => {
                let Ok(snapshot) = serde_json::from_value::<PositionSnapshot>(env.payload) else {
                    return invalid();
                };
                match self
                    .snapshot
                    .handle(session, env.msg_id, env.sent_at, &snapshot, now)
                    .await
                {
                    Ok(_) => FrameOutcome {
                        replies: vec![ack_frame(v, &[env.msg_id], now)],
                        close: None,
                        deliver: true,
                        alert: None,
                    },
                    Err(
                        SnapshotError::Invalid(_) | SnapshotError::Store(StoreError::Rejected(_)),
                    ) => invalid(),
                    Err(_) => FrameOutcome::close(CloseCode::Normal, "intake_retry"),
                }
            }
            // Store-and-ack for types whose consumer is a later slice (Esquadro). The account id is
            // kept only when it is in this session's mapping (Grosa M10); consumers re-verify.
            _ => {
                let account_id = env
                    .payload
                    .get("account_id")
                    .and_then(Value::as_str)
                    .and_then(|s| Uuid::parse_str(s).ok())
                    .map(AccountId::from_uuid)
                    .filter(|a| session.mapped(*a).is_some());
                let Ok(version) = ProtocolVersion::parse(&env.protocol_version) else {
                    return invalid();
                };
                let work = UnitOfWork {
                    receipt: Some((session.connection_id, env.msg_id)),
                    deferred: Some(DeferredInbound {
                        msg_id: env.msg_id,
                        connection_id: session.connection_id,
                        account_id,
                        kind: env.kind.clone(),
                        protocol_version: version,
                        payload: env.payload,
                        received_at: now,
                    }),
                    ..UnitOfWork::default()
                };
                match self.store.commit(session.user_id, work).await {
                    Ok(_) => ack(),
                    // A deterministic refusal (e.g. a token-shaped value): never a retry loop (L3).
                    Err(StoreError::Rejected(_)) => invalid(),
                    Err(_) => FrameOutcome::close(CloseCode::Normal, "intake_retry"),
                }
            }
        }
    }

    /// An `error` from the adapter (Esquadro H2): logged by code and ref only, never acked, the
    /// session stays open. When it refers to a Core command, that frame got a definitive answer:
    /// the command is `failed` at once (no more retransmission), audited as `execution.rejected`,
    /// and the account is flagged `sync.command_rejected`. Any other ref: alert only.
    async fn adapter_error(
        &self,
        session: &Session,
        env: &Envelope,
        now: DateTime<Utc>,
    ) -> FrameOutcome {
        let code = env
            .payload
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("protocol.unknown")
            .to_owned();
        let Some(ref_id) = env
            .payload
            .get("ref_msg_id")
            .and_then(Value::as_str)
            .and_then(|s| Uuid::parse_str(s).ok())
        else {
            return FrameOutcome {
                alert: Some(AdapterAlert {
                    code,
                    ref_msg_id: None,
                    command: false,
                }),
                ..FrameOutcome::default()
            };
        };
        let stored = match self
            .store
            .command_by_msg(session.user_id, session.connection_id, ref_id)
            .await
        {
            Ok(s) => s,
            Err(_) => return FrameOutcome::close(CloseCode::Normal, "intake_retry"),
        };
        let Some(stored) = stored else {
            return FrameOutcome {
                alert: Some(AdapterAlert {
                    code,
                    ref_msg_id: Some(ref_id),
                    command: false,
                }),
                ..FrameOutcome::default()
            };
        };
        let Ok(refs) = crate::audit::command_refs(
            self.intake.bindings(),
            session,
            stored.account_id,
            stored.instruction_id,
        )
        .await
        else {
            return FrameOutcome::close(CloseCode::Normal, "intake_retry");
        };
        let outcome = Finish {
            to: CommandState::Failed,
            reason: Some((
                SyncReason::CommandRejected,
                ReasonDetails {
                    command_id: Some(stored.command.id),
                    adapter_reason_code: Some(code.clone()),
                    ..ReasonDetails::default()
                },
            )),
            events: vec![crate::events::audit(
                "execution.rejected",
                now,
                json!({ "origin": "all_win", "platform_reason": code }),
                refs.clone(),
            )],
        };
        let alert = Some(AdapterAlert {
            code,
            ref_msg_id: Some(ref_id),
            command: true,
        });
        match finish(&*self.store, &stored, outcome, &refs, now).await {
            Ok(()) => FrameOutcome {
                alert,
                ..FrameOutcome::default()
            },
            Err(_) => FrameOutcome::close(CloseCode::Normal, "intake_retry"),
        }
    }
}

/// Whether an A→C (or shared) type exists in a negotiated version (§9).
fn type_exists(kind: &str, v: ProtocolVersion) -> bool {
    match kind {
        "trade_completed" | "notification_read" => v.supports_notifications(),
        _ => true,
    }
}
