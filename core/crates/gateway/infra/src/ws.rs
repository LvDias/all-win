//! `GET /adapter/v1` (ADR-0004 §2–§4): one task per adapter session. The task authenticates the
//! device token, runs the handshake, then pumps frames through `HandleFrame` one at a time (so
//! intake is serialized per connection), runs delivery passes, heartbeats and the 60-s token
//! revalidation, and records the disconnect. The token is never logged; only `key_id` is.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::Response;
use axum::routing::get;
use gateway_application::deliver::DeliverCommands;
use gateway_application::disconnect::HandleDisconnect;
use gateway_application::frames::{
    CLOSE_BY_USER, CloseCode, FrameValidator, HandleFrame, error_frame, heartbeat_frame,
};
use gateway_application::hello::{HandleHello, HelloOutcome};
use gateway_application::ports::{DeviceTokens, PresentedToken};
use gateway_application::session::{Session, SessionSettings, Sessions};
use gateway_application::wire::{Envelope, Hello};
use gateway_enterprise::ids::{ConnectionId, SessionId};
use gateway_enterprise::protocol::ProtocolVersion;
use gateway_enterprise::triggers::SessionEnd;
use shared_kernel::Clock;
use tokio::sync::mpsc;

/// Frames are at most 256 KiB (ADR-0004 §2).
const MAX_FRAME: usize = 256 * 1024;
const HELLO_TIMEOUT: Duration = Duration::from_secs(10);
const TICK: Duration = Duration::from_millis(250);
const TOKEN_RECHECK: Duration = Duration::from_secs(60);

/// Signals to a live session task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    /// Run a delivery pass now (new commands or limits for this connection).
    Wake,
    /// Close with this code (4002 revoked, 4004 superseded, 4000 shutdown).
    Close(CloseCode),
}

pub struct GatewayRuntime {
    pub sessions: Sessions,
    pub tokens: Arc<dyn DeviceTokens>,
    pub validator: Arc<dyn FrameValidator>,
    pub hello: HandleHello,
    pub frames: HandleFrame,
    pub deliver: DeliverCommands,
    pub disconnect: HandleDisconnect,
    pub settings: SessionSettings,
    pub clock: Arc<dyn Clock>,
    controls: Mutex<HashMap<SessionId, mpsc::Sender<Control>>>,
}

impl GatewayRuntime {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sessions: Sessions,
        tokens: Arc<dyn DeviceTokens>,
        validator: Arc<dyn FrameValidator>,
        hello: HandleHello,
        frames: HandleFrame,
        deliver: DeliverCommands,
        disconnect: HandleDisconnect,
        settings: SessionSettings,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            sessions,
            tokens,
            validator,
            hello,
            frames,
            deliver,
            disconnect,
            settings,
            clock,
            controls: Mutex::new(HashMap::new()),
        }
    }

    fn controls(&self) -> std::sync::MutexGuard<'_, HashMap<SessionId, mpsc::Sender<Control>>> {
        self.controls
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Sends a signal to the live session of a connection, if any.
    pub fn signal(&self, connection: ConnectionId, control: Control) {
        if let Some(session) = self.sessions.get(connection)
            && let Some(tx) = self.controls().get(&session.id)
        {
            // Full queue: a pending signal already wakes the task.
            let _ = tx.try_send(control);
        }
    }

    fn signal_session(&self, session: SessionId, control: Control) {
        if let Some(tx) = self.controls().get(&session) {
            let _ = tx.try_send(control);
        }
    }
}

pub fn router(runtime: Arc<GatewayRuntime>) -> Router {
    Router::new()
        .route("/adapter/v1", get(upgrade))
        .with_state(runtime)
}

fn bearer(headers: &HeaderMap) -> Option<PresentedToken> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    let token = value.strip_prefix("Bearer ")?;
    Some(PresentedToken::new(token.to_owned()))
}

/// The device is authenticated BEFORE the upgrade (Guarita I2). A refused peer still gets the
/// upgrade, only to receive close 4001 (the code the plugin acts on, ADR-0004 §2); nothing is
/// read from it, and frames are capped at the protocol's 256 KiB either way.
async fn upgrade(
    State(rt): State<Arc<GatewayRuntime>>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Response {
    let auth = match bearer(&headers) {
        Some(t) => rt.tokens.authenticate(&t).await,
        None => Ok(None),
    };
    ws.max_frame_size(MAX_FRAME)
        .max_message_size(MAX_FRAME)
        .on_upgrade(move |socket| run(rt, socket, auth))
}

async fn close(socket: &mut WebSocket, code: CloseCode, reason: &'static str) {
    let _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: code.code(),
            reason: reason.into(),
        })))
        .await;
}

async fn send(socket: &mut WebSocket, frame: String) -> bool {
    socket.send(Message::Text(frame.into())).await.is_ok()
}

async fn run(
    rt: Arc<GatewayRuntime>,
    mut socket: WebSocket,
    auth: Result<
        Option<gateway_application::ports::AdapterPrincipal>,
        gateway_application::ports::PortError,
    >,
) {
    let principal = match auth {
        Ok(Some(p)) => p,
        Ok(None) => return close(&mut socket, CloseCode::InvalidToken, "invalid_token").await,
        Err(_) => return close(&mut socket, CloseCode::Normal, "unavailable").await,
    };

    // The adapter speaks first: `hello`.
    let first = tokio::time::timeout(HELLO_TIMEOUT, socket.recv()).await;
    let Ok(Some(Ok(Message::Text(text)))) = first else {
        return close(&mut socket, CloseCode::ProtocolViolation, "hello_expected").await;
    };
    let hello = serde_json::from_str::<serde_json::Value>(text.as_str())
        .ok()
        .filter(|v| rt.validator.validate(v).is_ok())
        .and_then(|v| serde_json::from_value::<Envelope>(v).ok())
        .filter(|e| e.kind == "hello")
        .and_then(|e| serde_json::from_value::<Hello>(e.payload).ok());
    let Some(hello) = hello else {
        return close(&mut socket, CloseCode::ProtocolViolation, "hello_expected").await;
    };

    let now = rt.clock.now();
    let (session, welcome, limits) = match rt.hello.handle(&principal, &hello, now).await {
        Ok(HelloOutcome::Welcome {
            session,
            welcome_frame,
            limits,
        }) => (session, welcome_frame, limits),
        Ok(HelloOutcome::Outdated) => {
            let err = error_frame(
                ProtocolVersion::MIN_SUPPORTED,
                "protocol.unsupported_version",
                None,
                now,
            );
            let _ = send(&mut socket, err).await;
            return close(&mut socket, CloseCode::Outdated, "unsupported_version").await;
        }
        Err(_) => return close(&mut socket, CloseCode::Normal, "unavailable").await,
    };
    tracing::info!(key_id = %principal.key_id, session = %session.id, version = %session.version, "adapter session started");

    let (tx, mut rx) = mpsc::channel(16);
    rt.controls().insert(session.id, tx);
    if let Some(old) = rt.sessions.register(session.clone()) {
        rt.signal_session(old.id, Control::Close(CloseCode::Superseded));
    }

    let mut end = SessionEnd::Network;
    let mut superseded = false;
    let mut ok = send(&mut socket, welcome).await;
    if let (true, Some(m)) = (ok, limits) {
        session.write_due(m.msg_id, now, rt.settings.ack_timeout_ms);
        ok = send(&mut socket, m.frame).await;
    }
    if ok {
        let outcome = pump(&rt, &session, &mut socket, &mut rx).await;
        end = outcome.0;
        superseded = outcome.1;
    }
    rt.controls().remove(&session.id);
    if let Err(e) = rt
        .disconnect
        .handle(&session, end, superseded, rt.clock.now())
        .await
    {
        tracing::error!(session = %session.id, error = %e, "recording the disconnect failed");
    }
    tracing::info!(key_id = %principal.key_id, session = %session.id, "adapter session ended");
}

/// Returns how the session ended and whether it was superseded.
async fn pump(
    rt: &GatewayRuntime,
    session: &Session,
    socket: &mut WebSocket,
    rx: &mut mpsc::Receiver<Control>,
) -> (SessionEnd, bool) {
    let heartbeat = Duration::from_millis(u64::from(rt.settings.heartbeat_interval_ms));
    let mut tick = tokio::time::interval(TICK);
    let mut last_rx = tokio::time::Instant::now();
    let mut last_tx = tokio::time::Instant::now();
    let mut last_token_check = tokio::time::Instant::now();
    loop {
        tokio::select! {
            incoming = socket.recv() => {
                last_rx = tokio::time::Instant::now();
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        let outcome = rt.frames.handle(session, text.as_str(), rt.clock.now()).await;
                        for frame in outcome.replies {
                            if !send(socket, frame).await {
                                return (SessionEnd::Network, false);
                            }
                            last_tx = tokio::time::Instant::now();
                        }
                        if let Some((code, reason)) = outcome.close {
                            if code == CloseCode::Normal {
                                tracing::warn!(session = %session.id, reason, "closing for retry");
                            }
                            close(socket, code, reason).await;
                            return (SessionEnd::Network, false);
                        }
                        if outcome.deliver && !deliver(rt, session, socket).await {
                            return (SessionEnd::Network, false);
                        }
                    }
                    Some(Ok(Message::Close(frame))) => {
                        let by_user = frame.is_some_and(|f| f.code == CLOSE_BY_USER);
                        return (if by_user { SessionEnd::ByUser } else { SessionEnd::Network }, false);
                    }
                    Some(Ok(Message::Binary(_))) => {
                        close(socket, CloseCode::ProtocolViolation, "binary_frame").await;
                        return (SessionEnd::Network, false);
                    }
                    Some(Ok(Message::Ping(_) | Message::Pong(_))) => {}
                    Some(Err(_)) | None => return (SessionEnd::Network, false),
                }
            }
            control = rx.recv() => match control {
                Some(Control::Wake) => {
                    if !deliver(rt, session, socket).await {
                        return (SessionEnd::Network, false);
                    }
                }
                Some(Control::Close(code)) => {
                    close(socket, code, "").await;
                    return (SessionEnd::Network, code == CloseCode::Superseded);
                }
                None => return (SessionEnd::Network, false),
            },
            _ = tick.tick() => {
                let now = tokio::time::Instant::now();
                if now.duration_since(last_rx) >= heartbeat * 3 {
                    close(socket, CloseCode::HeartbeatTimeout, "heartbeat_timeout").await;
                    return (SessionEnd::Network, false);
                }
                if now.duration_since(last_token_check) >= TOKEN_RECHECK {
                    last_token_check = now;
                    if let Ok(false) = rt.tokens.still_valid(&session.key_id).await {
                        close(socket, CloseCode::Revoked, "revoked").await;
                        return (SessionEnd::Network, false);
                    }
                }
                if now.duration_since(last_tx) >= heartbeat {
                    if !send(socket, heartbeat_frame(session.version, rt.clock.now())).await {
                        return (SessionEnd::Network, false);
                    }
                    last_tx = now;
                }
                if !deliver(rt, session, socket).await {
                    return (SessionEnd::Network, false);
                }
            }
        }
    }
}

/// One delivery pass; `false` when the socket failed. A frame whose `sent` commit happened but
/// whose write failed is retransmitted at the next session (ADR-0004 §5 acceptance criterion).
async fn deliver(rt: &GatewayRuntime, session: &Session, socket: &mut WebSocket) -> bool {
    match rt.deliver.handle(session, rt.clock.now()).await {
        Ok(out) => {
            for o in out {
                if !send(socket, o.frame).await {
                    return false;
                }
            }
            true
        }
        Err(e) => {
            tracing::error!(session = %session.id, error = %e, "delivery pass failed");
            true
        }
    }
}
