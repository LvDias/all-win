//! Use-case results. One-time secrets are `SecretString` and appear only in the result of the
//! operation that creates them; HTTP adapters map these to the contract schemas.

use chrono::{DateTime, Utc};
use identity_enterprise::connection::{Connection, ConnectionStatus};
use identity_enterprise::ids::ConnectionId;
use identity_enterprise::invitation::{Invitation, InvitationStatus};
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

#[derive(Clone, Debug)]
pub enum LoginOutcome {
    Authenticated {
        user: User,
        cookie: SessionCookie,
        /// Only after a recovery-code sign-in.
        recovery_codes_remaining: Option<u8>,
    },
    Pending2fa {
        cookie: SessionCookie,
        pending_expires_at: DateTime<Utc>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionInfo {
    pub persistent: bool,
    pub idle_timeout_s: i64,
    pub absolute_timeout_s: i64,
    pub idle_expires_at: DateTime<Utc>,
    pub absolute_expires_at: DateTime<Utc>,
    pub step_up_valid_until: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecurityView {
    pub totp_enabled: bool,
    pub recovery_codes_remaining: u8,
    pub session: SessionInfo,
}

#[derive(Clone, Debug)]
pub struct TotpSetupView {
    pub otpauth_uri: SecretString,
    pub secret: SecretString,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct RecoveryCodesView {
    /// 10 codes in display form `XXXXX-XXXXX`, shown once.
    pub codes: Vec<SecretString>,
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

#[derive(Clone, Debug)]
pub struct ResetLink {
    pub reset_url: SecretString,
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
