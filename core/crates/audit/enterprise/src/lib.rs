//! `audit` module — enterprise layer: the user-visible audit entry (X-5, S10).
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

    /// Category of a stable event code by family, with the contract's exceptions
    /// (`KnownAuditCode`, HTTP 4.3.0; Architect, 2026-10-23). `None` for an unknown family.
    pub fn of_code(code: &str) -> Option<Self> {
        match code {
            "routing.subscription_changed" | "risk.rules_changed" => return Some(Self::Config),
            "trades.captured" | "trades.reconcile_mismatch" => return Some(Self::Execution),
            _ if code.starts_with("trades.manual_") => return Some(Self::Config),
            _ => {}
        }
        Some(match code.split('.').next()? {
            "ingress" => Self::Ingress,
            "routing" => Self::Routing,
            "execution" => Self::Execution,
            "risk" => Self::Risk,
            // `sync.*` (S11, HTTP 5.3.0): connection, for every sync code (Esquadro).
            "connection" | "sync" => Self::Connection,
            "security" => Self::Security,
            "config" | "sources" | "connections" | "accounts" | "profiles" | "presets" => {
                Self::Config
            }
            _ => return None,
        })
    }
}

/// S10 outcome taxonomy (Q-S10-2); set by the Core, never derived by clients.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Received,
    Routed,
    Executed,
    Partial,
    Skipped,
    Refused,
    Rejected,
    Expired,
    Unknown,
    Info,
}

impl Outcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Received => "received",
            Self::Routed => "routed",
            Self::Executed => "executed",
            Self::Partial => "partial",
            Self::Skipped => "skipped",
            Self::Refused => "refused",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
            Self::Info => "info",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "received" => Self::Received,
            "routed" => Self::Routed,
            "executed" => Self::Executed,
            "partial" => Self::Partial,
            "skipped" => Self::Skipped,
            "refused" => Self::Refused,
            "rejected" => Self::Rejected,
            "expired" => Self::Expired,
            "unknown" => Self::Unknown,
            "info" => Self::Info,
            _ => return None,
        })
    }
}

/// Who caused the entry (s10 §5): `user` when a person's request caused it, `system` for
/// what the Core or the plugin does on its own, `platform_admin` for an admin acting on
/// another user's data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    User,
    System,
    PlatformAdmin,
}

impl Actor {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::System => "system",
            Self::PlatformAdmin => "platform_admin",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "user" => Some(Self::User),
            "system" => Some(Self::System),
            "platform_admin" => Some(Self::PlatformAdmin),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuditEntry {
    pub id: Uuid,
    pub user_id: UserId,
    pub occurred_at: DateTime<Utc>,
    /// Outbox event id; redelivery of the same event is a no-op.
    pub source_event_id: Uuid,
    /// `app.outbox.seq` of the source event (timeline order).
    pub event_seq: i64,
    pub category: Category,
    /// Stable code; clients translate it.
    pub code: String,
    pub outcome: Outcome,
    pub actor: Actor,
    /// Whether the raw `details` may be shown as technical data.
    pub safe_to_display: bool,
    pub instruction_id: Option<Uuid>,
    pub source_id: Option<Uuid>,
    /// Snapshot; required with `source_id`.
    pub source_name: Option<String>,
    pub source_key_id: Option<String>,
    pub account_id: Option<Uuid>,
    /// Snapshot; required with `account_id`.
    pub account_name: Option<String>,
    pub message_id: Option<String>,
    pub signal_type: Option<String>,
    /// Exit action (routing and execution entries only).
    pub is_exit: Option<bool>,
    /// Typed by code (`AuditDetailsByCode`).
    pub details: Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories_follow_the_contract_table() {
        let cases = [
            ("ingress.received", Category::Ingress),
            ("routing.quantity_capped", Category::Routing),
            ("routing.subscription_changed", Category::Config),
            ("execution.filled", Category::Execution),
            ("trades.captured", Category::Execution),
            ("trades.reconcile_mismatch", Category::Execution),
            ("trades.manual_created", Category::Config),
            ("risk.limit_reached", Category::Risk),
            ("risk.rules_changed", Category::Config),
            ("connection.status_changed", Category::Connection),
            ("sync.flagged", Category::Connection),
            ("sync.cleared", Category::Connection),
            ("security.signed_in", Category::Security),
            ("sources.created", Category::Config),
            ("connections.paired", Category::Config),
            ("accounts.updated", Category::Config),
            ("profiles.default_changed", Category::Config),
            ("presets.version_applied", Category::Config),
        ];
        for (code, category) in cases {
            assert_eq!(Category::of_code(code), Some(category), "{code}");
        }
        assert_eq!(Category::of_code("trades.other"), None);
        assert_eq!(Category::of_code("nodot"), None);
    }
}
