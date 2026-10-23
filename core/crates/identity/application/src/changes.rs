//! Atomic write sets. A use case describes every row change, audit record and event of one
//! business operation in a [`ChangeSet`]; the store applies it in a single transaction (all or
//! nothing) under the RLS scope of `scope`. Conditional writes fail with a
//! [`crate::ports::StoreConflict`] and roll the whole set back.

use std::net::IpAddr;

use chrono::{DateTime, Utc};
use identity_enterprise::connection::{Connection, ConnectionStatus, PairedInstance, PairingCode};
use identity_enterprise::ids::{ConnectionId, InvitationId, PairingCodeId, SessionId, SourceId};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::session::{EndReason, Session};
use identity_enterprise::source::{DisplayName, Source};
use identity_enterprise::tokens::{KeyId, TokenHash};
use shared_kernel::UserId;
use uuid::Uuid;

/// Public part plus hash of a prefixed token; the secret itself is never persisted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredKey {
    pub key_id: KeyId,
    pub secret_hash: TokenHash,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Write {
    UpdateProfile {
        user_id: UserId,
        locale: Option<Locale>,
        time_zone: Option<TimeZoneName>,
    },
    InsertSession(Session),
    /// Sliding idle refresh and step-up time.
    UpdateSessionActivity {
        session_id: SessionId,
        last_activity_at: DateTime<Utc>,
        idle_expires_at: DateTime<Utc>,
        authenticated_at: DateTime<Utc>,
    },
    /// No-op when the session already ended.
    EndSession {
        session_id: SessionId,
        reason: EndReason,
        at: DateTime<Utc>,
    },
    EndUserSessions {
        user_id: UserId,
        except: Option<SessionId>,
        reason: EndReason,
        at: DateTime<Utc>,
    },
    /// Conflict `EmailTaken` when the e-mail has an open invitation (unique index) or a user.
    /// An expired open invitation for the same e-mail must be superseded first in the same set.
    InsertInvitation {
        invitation: Invitation,
        token_hash: TokenHash,
    },
    /// Sets `superseded_at` on an open invitation that already expired.
    SupersedeExpiredInvitation {
        invitation_id: InvitationId,
        at: DateTime<Utc>,
    },
    /// Conflict `InvitationGone` unless the invitation is still pending at `at`.
    AcceptInvitation {
        invitation_id: InvitationId,
        user_id: UserId,
        at: DateTime<Utc>,
    },
    /// Idempotent; no effect on an accepted invitation.
    RevokeInvitation {
        invitation_id: InvitationId,
        at: DateTime<Utc>,
    },
    /// Conflicts `NameTaken`, `KeyIdTaken`.
    InsertSource {
        source: Source,
        key: StoredKey,
    },
    /// Conflicts `NameTaken`, `SourceRevoked`.
    RenameSource {
        user_id: UserId,
        source_id: SourceId,
        name: DisplayName,
    },
    /// Revoke the active key and insert the new one (immediate rotation). Conflicts
    /// `KeyIdTaken`, `SourceRevoked`.
    RotateSourceKey {
        user_id: UserId,
        source_id: SourceId,
        key: StoredKey,
        at: DateTime<Utc>,
    },
    /// Idempotent: archives the source and revokes its key.
    RevokeSource {
        user_id: UserId,
        source_id: SourceId,
        at: DateTime<Utc>,
    },
    /// Conflict `NameTaken`.
    InsertConnection(Connection),
    /// Conflicts `NameTaken`, `ConnectionUnpaired`.
    RenameConnection {
        user_id: UserId,
        connection_id: ConnectionId,
        name: DisplayName,
    },
    /// Invalidates the connection's active code (superseded) and inserts the new one.
    /// Conflicts `ConnectionUnpaired`, `CodeHmacTaken`.
    IssuePairingCode(PairingCode),
    /// Conflict `PairingCodeGone` unless the code is still redeemable at `at`.
    UsePairingCode {
        code_id: PairingCodeId,
        at: DateTime<Utc>,
    },
    /// Records the paired instance; revokes the previous device token and inserts the new one
    /// (re-pairing, ADR-0004 §8). Conflicts `ConnectionUnpaired`, `KeyIdTaken`.
    PairConnection {
        user_id: UserId,
        connection_id: ConnectionId,
        instance: PairedInstance,
        device_key: StoredKey,
    },
    /// Live status written from gateway adapter-session events (`Connection::record_live_state`).
    UpdateConnectionStatus {
        user_id: UserId,
        connection_id: ConnectionId,
        status: ConnectionStatus,
        status_changed_at: DateTime<Utc>,
        last_seen_at: Option<DateTime<Utc>>,
    },
    /// Startup reset: `connected`/`outdated` → `disconnected`; no-op for any other status.
    DisconnectLiveConnection {
        user_id: UserId,
        connection_id: ConnectionId,
        at: DateTime<Utc>,
    },
    /// Idempotent: archives the connection (status `unpaired`), revokes its device token,
    /// invalidates its code.
    UnpairConnection {
        user_id: UserId,
        connection_id: ConnectionId,
        at: DateTime<Utc>,
    },
}

/// Who performed an audited action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Actor {
    User(UserId),
    Admin(UserId),
    /// The Core itself (automatic action).
    System,
    /// The paired plugin of a connection.
    Adapter(ConnectionId),
    /// Anonymous request carrying a valid one-time token (invitation).
    TokenHolder,
}

/// Which visible trail the record belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trail {
    /// The affected user's visible audit log.
    User(UserId),
    /// The platform administrators' trail, kept in the acting admin's log.
    Admin(UserId),
}

/// Audited security/business actions. Variants carry ids and public `key_id`s only, never
/// secrets, links or codes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditAction {
    AccountCreated {
        invitation_id: InvitationId,
    },
    InvitationIssued {
        invitation_id: InvitationId,
    },
    InvitationRevoked {
        invitation_id: InvitationId,
    },
    SourceCreated {
        source_id: SourceId,
        key_id: KeyId,
    },
    SourceRenamed {
        source_id: SourceId,
    },
    SourceRotated {
        source_id: SourceId,
        old_key_id: KeyId,
        new_key_id: KeyId,
    },
    SourceRevoked {
        source_id: SourceId,
        key_id: KeyId,
    },
    ConnectionCreated {
        connection_id: ConnectionId,
    },
    ConnectionRenamed {
        connection_id: ConnectionId,
    },
    PairingCodeIssued {
        connection_id: ConnectionId,
        repairing: bool,
    },
    ConnectionPaired {
        connection_id: ConnectionId,
        instance_id: Uuid,
        device_key_id: KeyId,
        replaced_previous_instance: bool,
    },
    ConnectionUnpaired {
        connection_id: ConnectionId,
        open_positions_acknowledged: bool,
    },
    /// Core restart: no adapter session survives it, so the connection is marked disconnected.
    ConnectionDisconnectedByRestart {
        connection_id: ConnectionId,
    },
}

/// Outbox `event_type` (= audit code) of every audited action. The audit module records these
/// in the affected user's log; `app` registers the audit consumer for each type, one by one.
pub const AUDIT_EVENT_TYPES: &[&str] = &[
    "security.account_created",
    "security.invitation_issued",
    "security.invitation_revoked",
    "config.source_created",
    "config.source_renamed",
    "config.source_rotated",
    "config.source_revoked",
    "config.connection_created",
    "config.connection_renamed",
    "security.pairing_code_issued",
    "connection.paired",
    "connection.unpaired",
    "connection.disconnected_by_restart",
];

/// Outbox `event_type`s of the integration events streamed to the web (SSE).
pub const SSE_EVENT_TYPES: &[&str] = &[
    "identity.sessions_ended",
    "identity.connection_status_changed",
    "identity.connection_paired",
    "identity.pairing_code_invalidated",
];

/// Every `event_type` identity writes to the outbox; `app` requires a consumer for each.
pub fn emitted_event_types() -> impl Iterator<Item = &'static str> {
    AUDIT_EVENT_TYPES.iter().chain(SSE_EVENT_TYPES).copied()
}

impl AuditAction {
    /// Outbox `event_type` and audit code; clients translate it.
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::AccountCreated { .. } => "security.account_created",
            Self::InvitationIssued { .. } => "security.invitation_issued",
            Self::InvitationRevoked { .. } => "security.invitation_revoked",
            Self::SourceCreated { .. } => "config.source_created",
            Self::SourceRenamed { .. } => "config.source_renamed",
            Self::SourceRotated { .. } => "config.source_rotated",
            Self::SourceRevoked { .. } => "config.source_revoked",
            Self::ConnectionCreated { .. } => "config.connection_created",
            Self::ConnectionRenamed { .. } => "config.connection_renamed",
            Self::PairingCodeIssued { .. } => "security.pairing_code_issued",
            Self::ConnectionPaired { .. } => "connection.paired",
            Self::ConnectionUnpaired { .. } => "connection.unpaired",
            Self::ConnectionDisconnectedByRestart { .. } => "connection.disconnected_by_restart",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditRecord {
    pub trail: Trail,
    pub actor: Actor,
    pub action: AuditAction,
    pub at: DateTime<Utc>,
    /// Client address of the request, when there is one.
    pub ip: Option<IpAddr>,
}

/// Which sessions a `SessionsEnded` event covers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EndedSessions {
    One(SessionId),
    /// Every session of the user except these (the tab that rotated keeps working: it holds
    /// the new cookie, and its old session id is excluded so it is not told to sign in again).
    AllExcept(Vec<SessionId>),
}

/// Integration events published through the outbox (SSE projection, gateway, audit).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentityEvent {
    /// SSE `session.ended`, sent before the stream of those sessions is closed.
    SessionsEnded {
        user_id: UserId,
        sessions: EndedSessions,
        reason: EndReason,
        at: DateTime<Utc>,
    },
    /// SSE `connection.paired`; the gateway adds the visible accounts after `hello`.
    ConnectionPaired {
        user_id: UserId,
        connection_id: ConnectionId,
        instance_name: String,
        at: DateTime<Utc>,
    },
    /// SSE `connection.status_changed`.
    ConnectionStatusChanged {
        user_id: UserId,
        connection_id: ConnectionId,
        status: ConnectionStatus,
        at: DateTime<Utc>,
    },
    /// SSE `connection.pairing_code_invalidated` (reason `superseded`).
    PairingCodeSuperseded {
        user_id: UserId,
        connection_id: ConnectionId,
        at: DateTime<Utc>,
    },
    // `identity.device_token_revoked` (gateway closes the WebSocket with 4002) is added by S2
    // together with its gateway consumer (Esquadro decision 1, rule A). Until then the revoked
    // `device_token` row is the source of truth: the gateway refuses it at connect.
}

impl IdentityEvent {
    /// Outbox `event_type`.
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::SessionsEnded { .. } => "identity.sessions_ended",
            Self::ConnectionStatusChanged { .. } => "identity.connection_status_changed",
            Self::ConnectionPaired { .. } => "identity.connection_paired",
            Self::PairingCodeSuperseded { .. } => "identity.pairing_code_invalidated",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChangeSet {
    /// RLS scope (`app.user_id`) of the transaction: the user whose rows change.
    pub scope: UserId,
    pub writes: Vec<Write>,
    pub audit: Vec<AuditRecord>,
    pub events: Vec<IdentityEvent>,
}

impl ChangeSet {
    pub fn new(scope: UserId) -> Self {
        Self {
            scope,
            writes: Vec::new(),
            audit: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn write(mut self, write: Write) -> Self {
        self.writes.push(write);
        self
    }

    pub fn audit(mut self, record: AuditRecord) -> Self {
        self.audit.push(record);
        self
    }

    pub fn event(mut self, event: IdentityEvent) -> Self {
        self.events.push(event);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use identity_enterprise::session::EndReason;

    #[test]
    fn every_emitted_type_is_listed_once_with_the_right_consumer_family() {
        let k = KeyId::parse("AAAAAAAAAAAA").unwrap();
        let (u, s, c, i) = (
            UserId::new_v7(),
            SourceId::new_v7(),
            ConnectionId::new_v7(),
            InvitationId::new_v7(),
        );
        let actions = [
            AuditAction::AccountCreated { invitation_id: i },
            AuditAction::InvitationIssued { invitation_id: i },
            AuditAction::InvitationRevoked { invitation_id: i },
            AuditAction::SourceCreated {
                source_id: s,
                key_id: k.clone(),
            },
            AuditAction::SourceRenamed { source_id: s },
            AuditAction::SourceRotated {
                source_id: s,
                old_key_id: k.clone(),
                new_key_id: k.clone(),
            },
            AuditAction::SourceRevoked {
                source_id: s,
                key_id: k.clone(),
            },
            AuditAction::ConnectionCreated { connection_id: c },
            AuditAction::ConnectionRenamed { connection_id: c },
            AuditAction::PairingCodeIssued {
                connection_id: c,
                repairing: false,
            },
            AuditAction::ConnectionPaired {
                connection_id: c,
                instance_id: Uuid::nil(),
                device_key_id: k,
                replaced_previous_instance: false,
            },
            AuditAction::ConnectionUnpaired {
                connection_id: c,
                open_positions_acknowledged: true,
            },
            AuditAction::ConnectionDisconnectedByRestart { connection_id: c },
        ];
        let at = chrono::Utc::now();
        let events = [
            IdentityEvent::SessionsEnded {
                user_id: u,
                sessions: EndedSessions::AllExcept(vec![]),
                reason: EndReason::Logout,
                at,
            },
            IdentityEvent::ConnectionStatusChanged {
                user_id: u,
                connection_id: c,
                status: ConnectionStatus::Connected,
                at,
            },
            IdentityEvent::ConnectionPaired {
                user_id: u,
                connection_id: c,
                instance_name: String::new(),
                at,
            },
            IdentityEvent::PairingCodeSuperseded {
                user_id: u,
                connection_id: c,
                at,
            },
        ];
        let audit: Vec<&str> = actions.iter().map(AuditAction::event_type).collect();
        let sse: Vec<&str> = events.iter().map(IdentityEvent::event_type).collect();
        assert_eq!(audit, AUDIT_EVENT_TYPES);
        assert_eq!(sse, SSE_EVENT_TYPES);
        let all: std::collections::BTreeSet<&str> = emitted_event_types().collect();
        assert_eq!(all.len(), AUDIT_EVENT_TYPES.len() + SSE_EVENT_TYPES.len());
        for t in &all {
            let audited = matches!(
                t.split('.').next(),
                Some("security" | "config" | "connection")
            );
            assert_eq!(AUDIT_EVENT_TYPES.contains(t), audited, "{t}");
        }
        assert!(
            !all.contains("identity.device_token_revoked"),
            "added by S2 with its consumer"
        );
    }
}
