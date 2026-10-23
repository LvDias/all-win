//! Adapter-protocol frames (contracts/adapter-protocol/v1). Schema validation happens in infra
//! before a frame reaches these types; here they are typed views and envelope builders.

use chrono::{DateTime, SecondsFormat, Utc};
use gateway_enterprise::instrument::Instrument;
use gateway_enterprise::protocol::ProtocolVersion;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

/// `sent_at` / timestamps: UTC with `Z`, millisecond precision.
pub fn timestamp(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// The exact envelope text (keys in `serde_json`'s sorted order, so it is reproducible from its
/// parts). The stored text is what is retransmitted, never re-serialized.
pub fn envelope(
    version: ProtocolVersion,
    msg_id: Uuid,
    kind: &str,
    sent_at: DateTime<Utc>,
    payload: &Value,
) -> String {
    let mut env = Map::new();
    env.insert(
        "protocol_version".into(),
        Value::String(version.to_string()),
    );
    env.insert("msg_id".into(), Value::String(msg_id.to_string()));
    env.insert("type".into(), Value::String(kind.to_owned()));
    env.insert("sent_at".into(), Value::String(timestamp(sent_at)));
    env.insert("payload".into(), payload.clone());
    Value::Object(env).to_string()
}

/// Commands are created before any session exists. Their envelope uses 1.2: the lowest version
/// the Core accepts, and every accepted session (≥ 1.2) accepts a 1.2 envelope (§9).
pub const COMMAND_ENVELOPE_VERSION: ProtocolVersion = ProtocolVersion::V1_2;

#[derive(Debug, Clone, Deserialize)]
pub struct Envelope {
    pub protocol_version: String,
    pub msg_id: Uuid,
    #[serde(rename = "type")]
    pub kind: String,
    pub sent_at: DateTime<Utc>,
    pub payload: Value,
}

// ---- A→C payloads --------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct Hello {
    pub instance_id: Uuid,
    pub adapter: AdapterInfo,
    pub supported_protocol_versions: Vec<String>,
    pub accounts: Vec<HelloAccount>,
    #[serde(default)]
    pub pending_outbound: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdapterInfo {
    #[serde(rename = "type")]
    pub kind: String,
    pub version: String,
    pub build: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HelloAccount {
    pub platform_account: String,
    #[serde(default)]
    pub connection_name: Option<String>,
    pub kind: String,
    pub oco_location: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ack {
    pub msg_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionReport {
    pub event_id: Uuid,
    #[serde(default)]
    pub command_id: Option<Uuid>,
    pub account_id: Uuid,
    pub platform_account: String,
    pub instrument: Instrument,
    pub origin: String,
    #[serde(default)]
    pub child_role: Option<String>,
    #[serde(default)]
    pub platform_order_id: Option<String>,
    pub status: String,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub quantity: Option<u32>,
    #[serde(default)]
    pub fill: Option<Fill>,
    #[serde(default)]
    pub position_after: Option<PositionAfter>,
    #[serde(default)]
    pub reason_code: Option<String>,
    pub occurred_at: DateTime<Utc>,
    #[serde(default)]
    pub platform_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fill {
    pub fill_id: String,
    pub quantity: u32,
    /// Exact decimal string.
    pub price: String,
    #[serde(default)]
    pub commission: Option<String>,
    #[serde(default)]
    pub operation: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionAfter {
    pub quantity: i64,
    #[serde(default)]
    pub average_price: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionSnapshot {
    pub trigger: String,
    pub taken_at: DateTime<Utc>,
    pub accounts: Vec<SnapshotAccount>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SnapshotAccount {
    pub account_id: Uuid,
    pub platform_account: String,
    pub connected: bool,
    pub positions: Vec<SnapshotPosition>,
    pub working_orders: Vec<SnapshotOrder>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SnapshotPosition {
    pub instrument: Instrument,
    pub position: PositionAfter,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SnapshotOrder {
    pub platform_order_id: String,
    #[serde(default)]
    pub command_id: Option<Uuid>,
    #[serde(default)]
    pub child_role: Option<String>,
    pub instrument: Instrument,
    pub origin: String,
    #[serde(default)]
    pub state: Option<String>,
}

// ---- C→A payloads --------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct Welcome {
    pub session_id: Uuid,
    pub connection_id: Uuid,
    pub protocol_version: String,
    pub heartbeat_interval_ms: u32,
    pub ack_timeout_ms: u32,
    pub simulation_only: bool,
    pub locale: String,
    pub accounts: Vec<WelcomeAccount>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WelcomeAccount {
    pub account_id: Uuid,
    pub platform_account: String,
    pub roles: Vec<&'static str>,
    pub sync_state: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sync_reasons: Vec<String>,
    /// 1.3 only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountState {
    pub account_id: Uuid,
    pub sync_state: &'static str,
    pub sync_reasons: Vec<String>,
    pub changed_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorPayload {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_msg_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json::json;

    #[test]
    fn envelope_is_reproducible_and_utc_z() {
        let at = Utc.timestamp_millis_opt(1_800_000_000_123).unwrap();
        let id = Uuid::from_u128(1);
        let a = envelope(
            ProtocolVersion::V1_2,
            id,
            "command",
            at,
            &json!({"b": 1, "a": 2}),
        );
        let b = envelope(
            ProtocolVersion::V1_2,
            id,
            "command",
            at,
            &json!({"a": 2, "b": 1}),
        );
        assert_eq!(a, b);
        let env: Value = serde_json::from_str(&a).unwrap();
        assert_eq!(env["protocol_version"], "1.2");
        assert_eq!(env["msg_id"], "00000000-0000-0000-0000-000000000001");
        assert_eq!(env["type"], "command");
        assert_eq!(env["sent_at"], "2027-01-15T08:00:00.123Z");
    }
}
