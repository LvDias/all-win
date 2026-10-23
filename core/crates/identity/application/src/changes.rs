//! Atomic write sets. A use case describes every row change, audit record and event of one
//! business operation in a [`ChangeSet`]; the store applies it in a single transaction (all or
//! nothing) under the RLS scope of `scope`. Conditional writes fail with a
//! [`crate::ports::StoreConflict`] and roll the whole set back.

use std::net::IpAddr;

use chrono::{DateTime, Utc};
use identity_enterprise::channel::{Channel, Destination};
use identity_enterprise::channel_change::ChannelChange;
use identity_enterprise::connection::{Connection, ConnectionStatus, PairedInstance, PairingCode};
use identity_enterprise::device::KnownDevice;
use identity_enterprise::ids::{
    ChannelChangeId, CodeId, ConnectionId, DeviceId, InvitationId, PairingCodeId, SessionId,
    SourceId,
};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::otp::{
    DeliveryState, InvalidReason, OneTimeCode, Purpose, SignupCode, StepUpAction,
};
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::session::{EndReason, Session};
use identity_enterprise::source::{DisplayName, Source};
use identity_enterprise::tokens::{CodeHmac, KeyId, TokenHash};
use identity_enterprise::user::User;
use shared_kernel::UserId;
use uuid::Uuid;

/// Public part plus hash of a prefixed token; the secret itself is never persisted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredKey {
    pub key_id: KeyId,
    pub secret_hash: TokenHash,
}

/// Which code table a code write targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeTable {
    /// `identity.one_time_code` (RLS: the change set's scope must be the owner).
    OneTime,
    /// `identity.signup_code` (admin-side data, no RLS).
    Signup,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Write {
    /// Sign-up completion. Conflict `EmailTaken` when the normalized e-mail already belongs to a user.
    InsertUser(User),
    UpdateProfile {
        user_id: UserId,
        locale: Option<Locale>,
        time_zone: Option<TimeZoneName>,
    },
    /// Replaces the value of a channel and stamps `*_verified_at` and `channel_changed_at`.
    /// Conflicts `EmailTaken` / `WhatsappTaken` when another user holds the value.
    SetChannelValue {
        user_id: UserId,
        value: Destination,
        at: DateTime<Utc>,
    },

    /// A browser seen for the first time by this user (the Core inserts it only when the user
    /// has no row for that cookie hash).
    InsertDevice(KnownDevice),
    /// Known device seen again at sign-in.
    TouchDevice {
        user_id: UserId,
        device_id: DeviceId,
        ip: Option<IpAddr>,
        at: DateTime<Utc>,
    },
    /// Idempotent ("forget this device").
    RevokeDevice {
        user_id: UserId,
        device_id: DeviceId,
        at: DateTime<Utc>,
    },
    /// Every device of the user ("sign out everywhere", hold applied).
    RevokeAllDevices {
        user_id: UserId,
        at: DateTime<Utc>,
    },

    InsertSession(Session),
    /// Sliding idle refresh.
    UpdateSessionActivity {
        session_id: SessionId,
        last_activity_at: DateTime<Utc>,
        idle_expires_at: DateTime<Utc>,
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
    EndDeviceSessions {
        user_id: UserId,
        device_id: DeviceId,
        reason: EndReason,
        at: DateTime<Utc>,
    },

    /// Conflict `EmailTaken` when the e-mail has an open invitation (unique index) or a user.
    /// An expired open invitation for the same e-mail must be superseded first in the same set.
    InsertInvitation {
        invitation: Invitation,
        token_hash: TokenHash,
    },
    /// Sets `superseded_at` on an open invitation that already expired, and invalidates its open
    /// sign-up codes.
    SupersedeExpiredInvitation {
        invitation_id: InvitationId,
        at: DateTime<Utc>,
    },
    /// Conflict `InvitationGone` unless the invitation is still pending (DB clock).
    AcceptInvitation {
        invitation_id: InvitationId,
        user_id: UserId,
        at: DateTime<Utc>,
    },
    /// Idempotent; no effect on an accepted invitation. Invalidates its open sign-up codes.
    RevokeInvitation {
        invitation_id: InvitationId,
        at: DateTime<Utc>,
    },

    InsertOneTimeCode(OneTimeCode),
    InsertSignupCode(SignupCode),
    /// Resend: new code for an open challenge (attempts kept). Conflict `CodeGone` when the code
    /// is no longer open (DB clock).
    ResendCode {
        table: CodeTable,
        code_id: CodeId,
        code_hmac: CodeHmac,
        sent_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    },
    /// One wrong attempt: `failed_attempts + 1`, invalidated at 5. No effect on a closed code.
    /// Always committed on its own, never with a success.
    FailCode {
        table: CodeTable,
        code_id: CodeId,
    },
    /// Consumes the code if it is still open with exactly this HMAC (DB clock). Conflict `CodeGone`.
    ConsumeCode {
        table: CodeTable,
        code_id: CodeId,
        code_hmac: CodeHmac,
        at: DateTime<Utc>,
    },
    /// Detached delivery outcome.
    SetDeliveryState {
        table: CodeTable,
        code_id: CodeId,
        state: DeliveryState,
    },
    /// Invalidates every open code of the user (channel changed).
    InvalidateUserCodes {
        user_id: UserId,
        reason: InvalidReason,
        at: DateTime<Utc>,
    },

    /// Conflict `ChannelChangeOpen` when another change of the user is open.
    InsertChannelChange(ChannelChange),
    /// Lazy expiry of the user's `awaiting_codes` change past `expires_at`; invalidates its codes.
    ExpireStaleChannelChange {
        user_id: UserId,
        at: DateTime<Utc>,
    },
    /// `awaiting_codes` → `completed`, if still awaiting and unexpired (DB clock). Conflict
    /// `ChannelChangeGone`.
    CompleteChannelChange {
        change_id: ChannelChangeId,
        at: DateTime<Utc>,
    },
    /// The change is still `awaiting_codes` and unexpired (DB clock), or conflict
    /// `ChannelChangeGone`. Confirms one code of a change that needs more codes.
    AssertChannelChangeAwaiting {
        change_id: ChannelChangeId,
        at: DateTime<Utc>,
    },
    /// `awaiting_codes` → `on_hold` until `hold_until`, same condition as completion.
    HoldChannelChange {
        change_id: ChannelChangeId,
        hold_until: DateTime<Utc>,
        at: DateTime<Utc>,
    },
    /// `on_hold` → `completed` (the worker applies a due hold). Conflict `ChannelChangeGone`.
    FinishHeldChannelChange {
        change_id: ChannelChangeId,
        at: DateTime<Utc>,
    },
    /// `awaiting_codes` / `on_hold` → `cancelled`, invalidating its open codes. Conflict
    /// `ChannelChangeGone` unless it is open.
    CancelChannelChange {
        change_id: ChannelChangeId,
        by_session: Option<SessionId>,
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
    /// Anonymous request carrying a valid one-time token or code (invitation, sign-up).
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepUpFailure {
    Missing,
    CodeInvalid,
}

/// Audited actions; `event_type()` is the contract `AuditCode` (modules/audit.yaml) and the fields
/// are its `details`. Ids, public `key_id`s, labels and masked values only: never a secret, a
/// link, a code or an unmasked channel value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditAction {
    SignedIn {
        channel: Channel,
        new_device: bool,
        device_label: String,
    },
    SignInFailed {
        channel: Channel,
    },
    CodeRateLimited {
        purpose: Purpose,
    },
    CodeDeliveryFailed {
        channel: Channel,
        purpose: Purpose,
    },
    NewDeviceNoticeSent {
        device_label: String,
    },
    StepUpSucceeded {
        action: StepUpAction,
        target_id: Option<Uuid>,
    },
    StepUpFailed {
        action: StepUpAction,
        target_id: Option<Uuid>,
        reason: StepUpFailure,
    },
    ChannelChangeRequested {
        change_id: ChannelChangeId,
        channel: Channel,
        masked_new_value: String,
    },
    ChannelChangeOnHold {
        change_id: ChannelChangeId,
        channel: Channel,
        hold_until: DateTime<Utc>,
    },
    ChannelChangeCompleted {
        change_id: ChannelChangeId,
        channel: Channel,
        masked_old_value: Option<String>,
        masked_new_value: String,
    },
    ChannelChangeCancelled {
        change_id: ChannelChangeId,
        channel: Channel,
        by_user: bool,
    },
    ChannelChangeNotCompleted {
        change_id: ChannelChangeId,
        channel: Channel,
    },
    DeviceForgotten {
        device_id: DeviceId,
        device_label: String,
    },
    SignedOutEverywhere,
    /// Admin trail.
    InvitationIssued {
        invitation_id: InvitationId,
        masked_email: String,
        email_sent: bool,
    },
    /// Admin trail.
    InvitationRevoked {
        invitation_id: InvitationId,
    },
    /// The new user's log, at sign-up completion.
    AccountCreated {
        invitation_id: InvitationId,
    },
    SourceCreated {
        source_id: SourceId,
        source_name: String,
        key_id: KeyId,
    },
    SourceRenamed {
        source_id: SourceId,
        source_name: String,
        previous_name: String,
        key_id: KeyId,
    },
    SourceRotated {
        source_id: SourceId,
        source_name: String,
        key_id: KeyId,
        previous_key_id: KeyId,
    },
    SourceRevoked {
        source_id: SourceId,
        source_name: String,
        key_id: KeyId,
    },
    ConnectionCreated {
        connection_id: ConnectionId,
        connection_name: String,
    },
    ConnectionRenamed {
        connection_id: ConnectionId,
        connection_name: String,
        previous_name: String,
    },
    PairingCodeIssued {
        connection_id: ConnectionId,
        connection_name: String,
        code_expires_at: DateTime<Utc>,
    },
    /// `connections.paired`, or `connections.repaired` when an instance was already paired.
    ConnectionPaired {
        connection_id: ConnectionId,
        connection_name: String,
        instance_name: String,
        adapter_version: String,
        repaired: bool,
    },
    ConnectionUnpaired {
        connection_id: ConnectionId,
        connection_name: String,
        open_positions_acknowledged: bool,
    },
    /// Live status change, including the startup reset to `disconnected`.
    ConnectionStatusChanged {
        connection_id: ConnectionId,
        connection_name: String,
        previous_status: ConnectionStatus,
        status: ConnectionStatus,
    },
}

/// Outbox `event_type` (= audit code) of every audited action. The audit module records these
/// in the affected user's log; `app` registers the audit consumer for each type, one by one.
pub const AUDIT_EVENT_TYPES: &[&str] = &[
    "security.signed_in",
    "security.sign_in_failed",
    "security.code_rate_limited",
    "security.code_delivery_failed",
    "security.new_device_notice_sent",
    "security.step_up_succeeded",
    "security.step_up_failed",
    "security.channel_change_requested",
    "security.channel_change_on_hold",
    "security.channel_change_completed",
    "security.channel_change_cancelled",
    "security.channel_change_not_completed",
    "security.device_forgotten",
    "security.signed_out_everywhere",
    "security.invitation_issued",
    "security.invitation_revoked",
    "security.account_created",
    "sources.created",
    "sources.renamed",
    "sources.rotated",
    "sources.revoked",
    "connections.created",
    "connections.renamed",
    "connections.pairing_code_issued",
    "connections.paired",
    "connections.repaired",
    "connections.unpaired",
    "connection.status_changed",
];

/// Outbox `event_type`s of the integration events streamed to the web (SSE).
pub const SSE_EVENT_TYPES: &[&str] = &[
    "identity.sessions_ended",
    "identity.connection_status_changed",
    "identity.connection_paired",
    "identity.pairing_code_invalidated",
];

/// Outbox `event_type` of the e-mail notices (no secret inside; policy §5.3, §7).
pub const NOTICE_EVENT_TYPE: &str = "identity.notice_requested";

/// Every `event_type` identity writes to the outbox; `app` requires a consumer for each.
pub fn emitted_event_types() -> impl Iterator<Item = &'static str> {
    AUDIT_EVENT_TYPES
        .iter()
        .chain(SSE_EVENT_TYPES)
        .copied()
        .chain(std::iter::once(NOTICE_EVENT_TYPE))
}

impl AuditAction {
    /// Outbox `event_type` and audit code; clients translate it.
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::SignedIn { .. } => "security.signed_in",
            Self::SignInFailed { .. } => "security.sign_in_failed",
            Self::CodeRateLimited { .. } => "security.code_rate_limited",
            Self::CodeDeliveryFailed { .. } => "security.code_delivery_failed",
            Self::NewDeviceNoticeSent { .. } => "security.new_device_notice_sent",
            Self::StepUpSucceeded { .. } => "security.step_up_succeeded",
            Self::StepUpFailed { .. } => "security.step_up_failed",
            Self::ChannelChangeRequested { .. } => "security.channel_change_requested",
            Self::ChannelChangeOnHold { .. } => "security.channel_change_on_hold",
            Self::ChannelChangeCompleted { .. } => "security.channel_change_completed",
            Self::ChannelChangeCancelled { .. } => "security.channel_change_cancelled",
            Self::ChannelChangeNotCompleted { .. } => "security.channel_change_not_completed",
            Self::DeviceForgotten { .. } => "security.device_forgotten",
            Self::SignedOutEverywhere => "security.signed_out_everywhere",
            Self::InvitationIssued { .. } => "security.invitation_issued",
            Self::InvitationRevoked { .. } => "security.invitation_revoked",
            Self::AccountCreated { .. } => "security.account_created",
            Self::SourceCreated { .. } => "sources.created",
            Self::SourceRenamed { .. } => "sources.renamed",
            Self::SourceRotated { .. } => "sources.rotated",
            Self::SourceRevoked { .. } => "sources.revoked",
            Self::ConnectionCreated { .. } => "connections.created",
            Self::ConnectionRenamed { .. } => "connections.renamed",
            Self::PairingCodeIssued { .. } => "connections.pairing_code_issued",
            Self::ConnectionPaired {
                repaired: false, ..
            } => "connections.paired",
            Self::ConnectionPaired { repaired: true, .. } => "connections.repaired",
            Self::ConnectionUnpaired { .. } => "connections.unpaired",
            Self::ConnectionStatusChanged { .. } => "connection.status_changed",
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
    /// The sessions bound to one device.
    Device(DeviceId),
}

/// E-mail notices sent through the outbox (they hold no secret; policy §5.3, §7, §9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Notice {
    /// To the account e-mail after a sign-in from a new device.
    NewDevice {
        device_label: String,
        ip: Option<IpAddr>,
        channel: Channel,
    },
    /// To the old value (and the other channel) after a change.
    ChannelChanged {
        to: Destination,
        channel: Channel,
        masked_new_value: String,
    },
    /// To every channel: a change is on hold until `hold_until` and can be cancelled.
    ChannelChangeOnHold {
        to: Destination,
        channel: Channel,
        masked_new_value: String,
        hold_until: DateTime<Utc>,
    },
    /// To every channel after a cancellation.
    ChannelChangeCancelled { to: Destination, channel: Channel },
}

/// Integration events published through the outbox (SSE projection, gateway, audit, notices).
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
    /// An e-mail notice for the user, in their locale.
    NoticeRequested {
        user_id: UserId,
        locale: Locale,
        notice: Notice,
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
            Self::NoticeRequested { .. } => NOTICE_EVENT_TYPE,
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

    /// Appends another set of the same scope (e.g. a step-up consume plus the action).
    pub fn extend(mut self, other: ChangeSet) -> Self {
        debug_assert_eq!(self.scope, other.scope);
        self.writes.extend(other.writes);
        self.audit.extend(other.audit);
        self.events.extend(other.events);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use identity_enterprise::session::EndReason;

    #[test]
    fn every_audit_type_is_listed_once_and_categorized() {
        let all: std::collections::BTreeSet<&str> = emitted_event_types().collect();
        assert_eq!(
            all.len(),
            AUDIT_EVENT_TYPES.len() + SSE_EVENT_TYPES.len() + 1
        );
        for t in AUDIT_EVENT_TYPES {
            assert!(
                matches!(
                    t.split('.').next(),
                    Some("security" | "sources" | "connections" | "connection")
                ),
                "{t}"
            );
        }
        let c = ConnectionId::new_v7();
        let k = KeyId::parse("AAAAAAAAAAAA").unwrap();
        let samples = [
            AuditAction::ConnectionPaired {
                connection_id: c,
                connection_name: "Desk".into(),
                instance_name: "PC".into(),
                adapter_version: "1.0.0".into(),
                repaired: true,
            },
            AuditAction::SourceRotated {
                source_id: SourceId::new_v7(),
                source_name: "Main".into(),
                key_id: k.clone(),
                previous_key_id: k,
            },
            AuditAction::SignedOutEverywhere,
        ];
        for a in samples {
            assert!(AUDIT_EVENT_TYPES.contains(&a.event_type()));
        }
        let e = IdentityEvent::SessionsEnded {
            user_id: UserId::new_v7(),
            sessions: EndedSessions::AllExcept(vec![]),
            reason: EndReason::SignedOutEverywhere,
            at: chrono::Utc::now(),
        };
        assert!(SSE_EVENT_TYPES.contains(&e.event_type()));
        assert!(
            !all.contains("identity.device_token_revoked"),
            "added by S2 with its consumer"
        );
    }
}
