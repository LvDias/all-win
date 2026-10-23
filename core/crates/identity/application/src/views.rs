//! Use-case results. One-time secrets are `SecretString` and appear only in the result of the
//! operation that creates them; HTTP adapters map these to the contract schemas.

use chrono::{DateTime, Utc};
use identity_enterprise::channel::Channel;
use identity_enterprise::channel_change::ChangeStatus;
use identity_enterprise::connection::{Connection, ConnectionStatus};
use identity_enterprise::ids::{ChannelChangeId, ConnectionId, DeviceId};
use identity_enterprise::invitation::{Invitation, InvitationStatus};
use identity_enterprise::otp::CodeRole;
use identity_enterprise::secret::SecretString;
use identity_enterprise::source::Source;
use identity_enterprise::user::User;

use crate::ports::VisibleAccount;

/// New session cookie value (`__Host-aw_session`). `Max-Age` only when `persistent`.
#[derive(Clone, Debug)]
pub struct SessionCookie {
    pub value: SecretString,
    pub persistent: bool,
    pub expires_at: DateTime<Utc>,
}

/// New device cookie value (`__Host-aw_device`, Max-Age 400 days).
#[derive(Clone, Debug)]
pub struct DeviceCookie {
    pub value: identity_enterprise::secret::SecretString,
}

/// `ChallengeIssued`: the sealed challenge token goes only to the requester.
#[derive(Clone, Debug)]
pub struct ChallengeIssued {
    pub challenge_id: SecretString,
    pub expires_at: DateTime<Utc>,
    pub resend_after_s: i64,
}

/// A successful sign-in or sign-up.
#[derive(Clone, Debug)]
pub struct SignedIn {
    pub user: User,
    /// True when this sign-in registered a new device (never on sign-up).
    pub new_device: bool,
    pub session: SessionCookie,
    /// Set when the browser gets a (new or renewed) device cookie.
    pub device: DeviceCookie,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryView {
    Pending,
    Sent,
    Failed,
    NotReported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceView {
    pub id: DeviceId,
    pub label: String,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub current: bool,
}

#[derive(Clone, Debug)]
pub struct ChangeCodeView {
    pub role: CodeRole,
    pub challenge_id: SecretString,
    pub confirmed: bool,
}

#[derive(Clone, Debug)]
pub struct ChannelChangeView {
    pub id: ChannelChangeId,
    pub channel: Channel,
    pub status: ChangeStatus,
    pub masked_new_value: String,
    pub codes: Vec<ChangeCodeView>,
    pub expires_at: DateTime<Utc>,
    pub hold_until: Option<DateTime<Utc>>,
}

/// Result of a verified channel-change code; `session` is set when the current session rotated.
#[derive(Clone, Debug)]
pub struct ChannelChangeVerified {
    pub change: ChannelChangeView,
    pub session: Option<SessionCookie>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionInfo {
    pub persistent: bool,
    pub idle_timeout_s: i64,
    pub absolute_timeout_s: i64,
    pub idle_expires_at: DateTime<Utc>,
    pub absolute_expires_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct SecurityView {
    pub email: String,
    pub whatsapp: Option<String>,
    pub channel_change_allowed_at: DateTime<Utc>,
    pub pending_channel_change: Option<ChannelChangeView>,
    pub session: SessionInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvitationView {
    pub invitation: Invitation,
    pub status: InvitationStatus,
}

#[derive(Clone, Debug)]
pub struct CreatedInvitation {
    pub invitation: InvitationView,
    pub invite_url: SecretString,
    pub email_sent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvitationLookup {
    pub email: String,
    pub locale: identity_enterprise::profile::Locale,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceView {
    pub source: Source,
    pub is_automated: bool,
    /// `https://<host>/ingress/v1/<type>/awk_<key_id>_…`, never the secret.
    pub masked_url: String,
    pub last_received_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct SourceWithUrl {
    pub source: SourceView,
    pub webhook_url: SecretString,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionView {
    pub connection: Connection,
    pub status: ConnectionStatus,
    pub status_changed_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub pairing_code_expires_at: Option<DateTime<Utc>>,
    /// A "move to another machine" code is active while the current instance still works.
    pub repairing_pending: bool,
    pub visible_accounts: Vec<VisibleAccount>,
    pub open_position_accounts: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct PairingCodeView {
    /// Display form `XXXX-XXXX`, shown once.
    pub pairing_code: SecretString,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct CreatedConnection {
    pub connection: ConnectionView,
    pub pairing: PairingCodeView,
}

#[derive(Clone, Debug)]
pub struct PairingResult {
    pub connection_id: ConnectionId,
    /// `awd_<key_id>_<secret>`, returned exactly once.
    pub device_token: SecretString,
}
