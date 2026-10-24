//! User-visible ingress outcomes (s1-webhook-ingest §3). Each one becomes an
//! audit entry through the outbox; none carries a token, secret or URI.

use chrono::{DateTime, Utc};
use shared_kernel::UserId;

use crate::{InstructionId, MessageId, SourceId};

/// Stable `ingress.*` rejection codes (OpenAPI `ErrorCode`); clients translate them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RejectionCode {
    RateLimited,
    UnsupportedMediaType,
    PayloadTooLarge,
    InvalidJson,
    SchemaViolation,
    UnsupportedVersion,
    SourceMismatch,
    ActionNotAllowedForSource,
    OrderTypeNotAllowedForSource,
    SignalTypeNotAllowedForSource,
    UnsupportedInstrument,
}

impl RejectionCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RateLimited => "ingress.rate_limited",
            Self::UnsupportedMediaType => "ingress.unsupported_media_type",
            Self::PayloadTooLarge => "ingress.payload_too_large",
            Self::InvalidJson => "ingress.invalid_json",
            Self::SchemaViolation => "ingress.schema_violation",
            Self::UnsupportedVersion => "ingress.unsupported_version",
            Self::SourceMismatch => "ingress.source_mismatch",
            Self::ActionNotAllowedForSource => "ingress.action_not_allowed_for_source",
            Self::OrderTypeNotAllowedForSource => "ingress.order_type_not_allowed_for_source",
            Self::SignalTypeNotAllowedForSource => "ingress.signal_type_not_allowed_for_source",
            Self::UnsupportedInstrument => "ingress.unsupported_instrument",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngressEventKind {
    Received {
        instruction_id: InstructionId,
        message_id: MessageId,
        signal_type: String,
        action: String,
        instrument_symbol: String,
        quantity: Option<i32>,
        sent_at: DateTime<Utc>,
    },
    DuplicateIgnored {
        instruction_id: InstructionId,
        message_id: MessageId,
        signal_type: String,
        /// The first message's content was purged (Q13 b): it could not be compared.
        content_not_retained: bool,
    },
    /// The same `message_id` again with a different compared field (HTTP 5.4.0): still 200 and
    /// nothing executed, but made visible. Field names only, never values.
    DuplicateConflict {
        instruction_id: InstructionId,
        message_id: MessageId,
        signal_type: String,
        first_received_at: DateTime<Utc>,
        differing_fields: Vec<&'static str>,
    },
    Rejected {
        code: RejectionCode,
        pointer: Option<String>,
        message_id: Option<MessageId>,
        /// Body size in bytes when known (Q13 §8); never the body itself.
        size_bytes: Option<u64>,
    },
    /// The source key's own rate limit (`ingress.rate_limited`); nothing was read.
    RateLimited { retry_after_s: u64 },
}

/// One ingress outcome for an authenticated source key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngressEvent {
    pub user_id: UserId,
    pub at: DateTime<Utc>,
    pub source_id: SourceId,
    /// Snapshot of the source's name (audit `source_name`, required with `source_id`).
    pub source_name: String,
    /// Public part of the token; the only identifying value allowed in audit (ALIGN-3).
    pub key_id: String,
    pub kind: IngressEventKind,
}

impl IngressEvent {
    /// Every outbox `event_type` this module emits; each must have a consumer at startup.
    pub const TYPES: [&'static str; 5] = [
        "ingress.received",
        "ingress.duplicate_ignored",
        "ingress.duplicate_conflict",
        "ingress.rejected",
        "ingress.rate_limited",
    ];

    /// Outbox `event_type` and audit `code`.
    pub fn event_type(&self) -> &'static str {
        match self.kind {
            IngressEventKind::Received { .. } => "ingress.received",
            IngressEventKind::DuplicateIgnored { .. } => "ingress.duplicate_ignored",
            IngressEventKind::DuplicateConflict { .. } => "ingress.duplicate_conflict",
            IngressEventKind::Rejected { .. } => "ingress.rejected",
            IngressEventKind::RateLimited { .. } => "ingress.rate_limited",
        }
    }
}
