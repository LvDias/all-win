//! `audit` module — enterprise layer: the user-visible audit entry (X-5).
//! Entries are immutable and never hold a token, secret or raw URI.

use chrono::{DateTime, Utc};
use serde_json::Value;
use shared_kernel::UserId;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Ingress,
    Routing,
    Execution,
    Risk,
    Connection,
    Security,
    Config,
}

impl Category {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ingress => "ingress",
            Self::Routing => "routing",
            Self::Execution => "execution",
            Self::Risk => "risk",
            Self::Connection => "connection",
            Self::Security => "security",
            Self::Config => "config",
        }
    }

    /// Category from the first segment of a stable event code (`ingress.received`).
    pub fn of_code(code: &str) -> Option<Self> {
        Some(match code.split('.').next()? {
            "ingress" => Self::Ingress,
            "routing" => Self::Routing,
            "execution" => Self::Execution,
            "risk" => Self::Risk,
            "connection" => Self::Connection,
            "security" => Self::Security,
            "config" => Self::Config,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuditEntry {
    pub id: Uuid,
    pub user_id: UserId,
    pub occurred_at: DateTime<Utc>,
    /// Outbox event id; redelivery of the same event is a no-op.
    pub source_event_id: Uuid,
    pub category: Category,
    /// Stable code; clients translate it.
    pub code: String,
    pub instruction_id: Option<Uuid>,
    pub source_id: Option<Uuid>,
    pub source_key_id: Option<String>,
    pub signal_type: Option<String>,
    pub details: Value,
}
