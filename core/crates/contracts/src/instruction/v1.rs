//! Instruction contract v1 (`contracts/instruction/v1`).
//!
//! [`parse`] validates the raw JSON against the canonical schema first and only
//! then maps it to [`Instruction`], so structural rules live in one place.

use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Canonical JSON Schema (draft 2020-12), embedded at build time.
pub const SCHEMA_JSON: &str =
    include_str!("../../../../../contracts/instruction/v1/instruction.schema.json");

/// Highest minor of major 1 this build implements. `parse` accepts any 1.x the
/// schema accepts; whether a newer minor is acceptable is Core policy.
pub const SUPPORTED_MINOR: u32 = 0;

static VALIDATOR: LazyLock<Result<jsonschema::Validator, String>> = LazyLock::new(|| {
    let schema: Value = serde_json::from_str(SCHEMA_JSON).map_err(|e| e.to_string())?;
    jsonschema::validator_for(&schema).map_err(|e| e.to_string())
});

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// Body is not JSON.
    #[error("malformed json")]
    Malformed,
    /// JSON does not satisfy the schema. Holds instance paths only, never values,
    /// so it is safe to log and to show the user.
    #[error("schema violation at {paths:?}")]
    Schema { paths: Vec<String> },
    /// Schema-valid JSON the DTO cannot represent (contract/DTO drift).
    #[error("schema-valid instruction not representable: {0}")]
    Unrepresentable(String),
    /// The embedded schema failed to compile (build defect).
    #[error("instruction schema unavailable: {0}")]
    SchemaUnavailable(String),
}

/// Validates raw bytes against the v1 schema and maps them to [`Instruction`].
pub fn parse(bytes: &[u8]) -> Result<Instruction, ParseError> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| ParseError::Malformed)?;
    parse_value(value)
}

pub fn parse_value(value: Value) -> Result<Instruction, ParseError> {
    let validator = VALIDATOR
        .as_ref()
        .map_err(|e| ParseError::SchemaUnavailable(e.clone()))?;
    let paths: Vec<String> = validator
        .iter_errors(&value)
        .map(|e| e.instance_path().to_string())
        .collect();
    if !paths.is_empty() {
        return Err(ParseError::Schema { paths });
    }
    serde_json::from_value(value).map_err(|e| ParseError::Unrepresentable(e.to_string()))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Instruction {
    pub schema_version: SchemaVersion,
    pub message_id: String,
    pub sent_at: DateTime<Utc>,
    pub source: Source,
    pub signal_type: SignalType,
    pub action: Action,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_effect: Option<PositionEffect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    pub instrument: Instrument,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<Changes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refs: Option<Refs>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::str_option"
    )]
    pub reference_price: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Map<String, Value>>,
}

/// `"1.<minor>"`; the schema guarantees the format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaVersion {
    pub major: u32,
    pub minor: u32,
}

impl std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl std::str::FromStr for SchemaVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, minor) = s.split_once('.').ok_or("missing '.'")?;
        Ok(Self {
            major: major.parse().map_err(|_| "bad major")?,
            minor: minor.parse().map_err(|_| "bad minor")?,
        })
    }
}

impl Serialize for SchemaVersion {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for SchemaVersion {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    /// Source kind; must match the kind bound to the authenticated credential.
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalType {
    Trigger,
    TradePlan,
    NqTrigger,
    MasterAccount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Buy,
    Sell,
    Close,
    Reverse,
    Modify,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionEffect {
    Open,
    Reduce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Long,
    Short,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Instrument {
    pub symbol: String,
    /// `YYYY-MM`; omitted means the catalog's active contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_month: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Order {
    #[serde(rename = "type")]
    pub kind: OrderType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<OrderRole>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::str_option"
    )]
    pub limit_price: Option<Decimal>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::str_option"
    )]
    pub stop_price: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderRole {
    Entry,
    Stop,
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    Day,
    Gtc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Changes {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "rust_decimal::serde::str_option"
    )]
    pub price_delta: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Refs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_order_ref: Option<String>,
}
