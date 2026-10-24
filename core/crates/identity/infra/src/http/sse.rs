//! `GET /api/v1/events` (Server-Sent Events, contract schema `Event`).
//!
//! [`SseHub`] fans identity's outbox events (`identity.*`) out to the open streams of each
//! user; `app`'s outbox dispatcher calls [`SseHub::publish`] after delivery. Events carry no
//! SSE `id` and are not replayed. The stream never refreshes the idle timeout; it re-checks
//! its session every [`HEARTBEAT`] and, when the session ended, sends `session.ended` before
//! closing. A slow client that falls behind is disconnected (it refetches on reconnect).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::body::{Body, Bytes};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::Response;
use chrono::{DateTime, Utc};
use http_body_util::Channel;
use identity_application::auth::{AuthenticateSession, StreamCheck};
use identity_application::deps::IdentityDeps;
use identity_enterprise::connection::ConnectionStatus;
use identity_enterprise::ids::{DeviceId, SessionId};
use serde_json::{Value, json};
use shared_kernel::UserId;
use tokio::sync::broadcast;
use uuid::Uuid;

pub const HEARTBEAT: Duration = Duration::from_secs(25);
const PER_USER_BUFFER: usize = 64;

/// Which sessions a `session.ended` message closes.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Ends {
    One(Uuid),
    AllExcept(Vec<Uuid>),
    /// Every session bound to this device ("forget this device").
    Device(Uuid),
}

impl Ends {
    fn covers(&self, session: SessionId, device: DeviceId) -> bool {
        match self {
            Self::One(id) => *id == session.as_uuid(),
            Self::AllExcept(except) => !except.contains(&session.as_uuid()),
            Self::Device(id) => *id == device.as_uuid(),
        }
    }
}

#[derive(Clone, Debug)]
struct Message {
    event: &'static str,
    data: String,
    ends: Option<Ends>,
}

#[derive(Default)]
pub struct SseHub {
    channels: Mutex<HashMap<UserId, broadcast::Sender<Message>>>,
}

fn uuid_at(payload: &Value, key: &str) -> Option<Uuid> {
    payload
        .get(key)
        .and_then(Value::as_str)
        .and_then(|s| Uuid::parse_str(s).ok())
}

fn at(occurred_at: DateTime<Utc>) -> String {
    super::schemas::ts(occurred_at)
}

impl SseHub {
    pub fn new() -> Self {
        Self::default()
    }

    /// Maps an `identity.*` outbox event to an SSE event for the user's open streams.
    /// Returns false for event types that are not streamed.
    pub fn publish(
        &self,
        user_id: UserId,
        event_type: &str,
        occurred_at: DateTime<Utc>,
        payload: &Value,
    ) -> bool {
        let message = match event_type {
            "identity.sessions_ended" => {
                let ends = match (uuid_at(payload, "session_id"), uuid_at(payload, "device_id")) {
                    (Some(id), _) => Ends::One(id),
                    (None, Some(device)) => Ends::Device(device),
                    (None, None) => Ends::AllExcept(
                        payload
                            .get("all_except")
                            .and_then(Value::as_array)
                            .map(|a| a.iter().filter_map(|v| v.as_str().and_then(|s| Uuid::parse_str(s).ok())).collect())
                            .unwrap_or_default(),
                    ),
                };
                let reason = payload.get("reason").and_then(Value::as_str).unwrap_or("expired");
                if reason == "rotated" {
                    return false;
                }
                Message {
                    event: "session.ended",
                    data: json!({ "type": "session.ended", "at": at(occurred_at), "reason": reason }).to_string(),
                    ends: Some(ends),
                }
            }
            "identity.connection_status_changed" => {
                let mut data = json!({
                    "type": "connection.status_changed", "at": at(occurred_at),
                    "connection_id": payload.get("connection_id"), "status": payload.get("status"),
                });
                if let Some(reason) = payload
                    .get("status")
                    .and_then(Value::as_str)
                    .and_then(ConnectionStatus::parse)
                    .and_then(ConnectionStatus::implied_disconnect_reason)
                {
                    data["disconnect_reason"] = json!(reason.as_str());
                }
                Message {
                    event: "connection.status_changed",
                    data: data.to_string(),
                    ends: None,
                }
            }
            "identity.connection_paired" => Message {
                event: "connection.paired",
                data: json!({
                    "type": "connection.paired", "at": at(occurred_at),
                    "connection_id": payload.get("connection_id"), "instance_name": payload.get("instance_name"),
                    // The instance reports its accounts after `hello`; clients refetch the connection.
                    "visible_accounts": [],
                })
                .to_string(),
                ends: None,
            },
            "identity.pairing_code_invalidated" => Message {
                event: "connection.pairing_code_invalidated",
                data: json!({
                    "type": "connection.pairing_code_invalidated", "at": at(occurred_at),
                    "connection_id": payload.get("connection_id"), "reason": payload.get("reason"),
                })
                .to_string(),
                ends: None,
            },
            _ => return false,
        };
        let channels = self
            .channels
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(tx) = channels.get(&user_id) {
            let _ = tx.send(message);
        }
        true
    }

    fn subscribe(&self, user_id: UserId) -> broadcast::Receiver<Message> {
        let mut channels = self
            .channels
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        channels.retain(|_, tx| tx.receiver_count() > 0);
        channels
            .entry(user_id)
            .or_insert_with(|| broadcast::channel(PER_USER_BUFFER).0)
            .subscribe()
    }
}

fn frame(event: &str, data: &str) -> Bytes {
    Bytes::from(format!("event: {event}\ndata: {data}\n\n"))
}

/// Opens the stream for an authenticated session (`cookie` is re-checked on each heartbeat).
pub fn open(
    hub: &Arc<SseHub>,
    deps: IdentityDeps,
    user_id: UserId,
    session_id: SessionId,
    device_id: DeviceId,
    cookie: String,
) -> Response {
    let mut rx = hub.subscribe(user_id);
    let (mut tx, body) = Channel::<Bytes>::new(16);
    tokio::spawn(async move {
        if tx
            .send_data(Bytes::from_static(b": connected\n\n"))
            .await
            .is_err()
        {
            return;
        }
        let auth = AuthenticateSession::new(deps.clone());
        let mut beat = tokio::time::interval(HEARTBEAT);
        beat.tick().await;
        loop {
            tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(m) => {
                        let closing = m.ends.as_ref().is_some_and(|e| e.covers(session_id, device_id));
                        if m.ends.is_some() && !closing {
                            continue;
                        }
                        if tx.send_data(frame(m.event, &m.data)).await.is_err() || closing {
                            return;
                        }
                    }
                    Err(_) => return,
                },
                _ = beat.tick() => {
                    match auth.stream_check(&cookie).await {
                        StreamCheck::Live => {}
                        StreamCheck::Ended(reason) => {
                            let data = json!({ "type": "session.ended", "at": at(deps.clock.now()), "reason": reason.as_str() }).to_string();
                            let _ = tx.send_data(frame("session.ended", &data)).await;
                            return;
                        }
                        StreamCheck::Close => return,
                    }
                    if tx.send_data(Bytes::from_static(b": ping\n\n")).await.is_err() {
                        return;
                    }
                }
            }
        }
    });
    let mut response = Response::new(Body::new(body));
    *response.status_mut() = StatusCode::OK;
    let h = response.headers_mut();
    h.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/event-stream"),
    );
    h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    h.insert("x-accel-buffering", HeaderValue::from_static("no"));
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_identity_events_and_skips_others() {
        let hub = SseHub::new();
        let user = UserId::new_v7();
        let mut rx = hub.subscribe(user);
        let id = Uuid::now_v7();
        assert!(hub.publish(
            user,
            "identity.connection_status_changed",
            Utc::now(),
            &json!({"connection_id": id.to_string(), "status": "connected"})
        ));
        let m = rx.try_recv().unwrap();
        assert_eq!(m.event, "connection.status_changed");
        let v: Value = serde_json::from_str(&m.data).unwrap();
        assert_eq!(v["status"], "connected");
        assert!(v.get("disconnect_reason").is_none());
        assert!(v["at"].as_str().unwrap().ends_with('Z'));
        assert!(hub.publish(
            user,
            "identity.connection_status_changed",
            Utc::now(),
            &json!({"connection_id": id.to_string(), "status": "outdated"})
        ));
        let v: Value = serde_json::from_str(&rx.try_recv().unwrap().data).unwrap();
        assert_eq!(v["disconnect_reason"], "outdated");
        assert!(!hub.publish(user, "security.signed_in", Utc::now(), &json!({})));
        assert!(!hub.publish(
            user,
            "identity.sessions_ended",
            Utc::now(),
            &json!({"session_id": id.to_string(), "reason": "rotated"})
        ));
    }

    #[test]
    fn session_end_scope() {
        let s = SessionId::new_v7();
        let d = DeviceId::new_v7();
        assert!(Ends::One(s.as_uuid()).covers(s, d));
        assert!(!Ends::AllExcept(vec![s.as_uuid()]).covers(s, d));
        assert!(Ends::AllExcept(vec![]).covers(s, d));
        assert!(Ends::Device(d.as_uuid()).covers(s, d));
        assert!(!Ends::Device(Uuid::now_v7()).covers(s, d));
    }
}
