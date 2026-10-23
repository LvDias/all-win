//! Ports implemented by `identity-infra` (or by other modules, wired in `app`).

use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use identity_enterprise::channel::{Channel, Destination};
use identity_enterprise::channel_change::ChannelChange;
use identity_enterprise::codes::NormalizedCode;
use identity_enterprise::connection::{AdapterInfo, Connection, PairingCode};
use identity_enterprise::device::KnownDevice;
use identity_enterprise::ids::{
    ChannelChangeId, CodeId, ConnectionId, DeviceId, InvitationId, SourceId,
};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::otp::{ChallengeClaims, DeviceClass, OneTimeCode, Purpose, SignupCode};
use identity_enterprise::profile::Locale;
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::Session;
use identity_enterprise::source::Source;
use identity_enterprise::tokens::{CodeHmac, TokenHash};
use identity_enterprise::user::{User, UserStatus};
use shared_kernel::UserId;
use thiserror::Error;

use crate::changes::ChangeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreConflict {
    /// The e-mail belongs to a user or has an open invitation.
    EmailTaken,
    /// The WhatsApp number belongs to another user.
    WhatsappTaken,
    InvitationGone,
    NameTaken,
    KeyIdTaken,
    CodeHmacTaken,
    SourceRevoked,
    ConnectionUnpaired,
    PairingCodeGone,
    /// A one-time or sign-up code is no longer open (consumed, invalidated, expired or resent).
    CodeGone,
    /// Another channel change of the user is open (`channel_change_one_open`).
    ChannelChangeOpen,
    /// The channel change is no longer in the expected state.
    ChannelChangeGone,
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("store conflict: {0:?}")]
    Conflict(StoreConflict),
    /// Database or driver failure (detail for technical logs only).
    #[error("store unavailable: {0}")]
    Unavailable(String),
}

/// What `find_user_for_login` returns (data model §3): enough to send a code, nothing more.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoginTarget {
    pub user_id: UserId,
    pub status: UserStatus,
    pub locale: Locale,
    pub destinations: Vec<Destination>,
}

impl LoginTarget {
    pub fn destination(&self, channel: Channel) -> Option<&Destination> {
        self.destinations.iter().find(|d| d.channel() == channel)
    }
}

/// Admin listing row (identity fields only, no trading data).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserSummary {
    pub id: UserId,
    pub email: String,
    pub status: UserStatus,
    pub is_platform_admin: bool,
    pub whatsapp_verified: bool,
    pub created_at: DateTime<Utc>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConnectionRecord {
    pub connection: Connection,
    /// Expiry of the active pairing code, if one is active.
    pub active_code_expires_at: Option<DateTime<Utc>>,
}

/// Sends counted in one window of the durable per-identifier limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowCount {
    pub window_seconds: i32,
    pub window_start: DateTime<Utc>,
    pub sends: i32,
}

/// Persistence of the identity module. Reads of user-owned rows take the owner's `UserId`;
/// pre-authentication lookups (token, login, code, pairing) go through the narrow
/// `SECURITY DEFINER` functions of the data model §3.
#[async_trait]
pub trait IdentityStore: Send + Sync {
    /// `find_user_for_login`; also runs, for every identifier, the statements a known one runs.
    async fn login_target(&self, email_normalized: &str)
    -> Result<Option<LoginTarget>, StoreError>;
    async fn user(&self, user_id: UserId) -> Result<Option<User>, StoreError>;
    /// Every user; answered only when `admin` is an active platform admin (RLS scope).
    async fn users(&self, admin: UserId) -> Result<Vec<UserSummary>, StoreError>;
    /// Whether a user has this e-mail; answered only for an active platform admin.
    async fn email_registered(
        &self,
        admin: UserId,
        email_normalized: &str,
    ) -> Result<bool, StoreError>;

    async fn session_by_token(&self, token_hash: &TokenHash)
    -> Result<Option<Session>, StoreError>;
    async fn session(
        &self,
        user_id: UserId,
        session_id: identity_enterprise::ids::SessionId,
    ) -> Result<Option<Session>, StoreError>;

    /// The user's device row for a device-cookie hash, revoked or not.
    async fn device_by_key(
        &self,
        user_id: UserId,
        key_hash: &TokenHash,
    ) -> Result<Option<KnownDevice>, StoreError>;
    /// A non-revoked device of ANY user holds this cookie hash (shared browser, §4.1).
    async fn device_key_in_use(&self, key_hash: &TokenHash) -> Result<bool, StoreError>;
    async fn device(
        &self,
        user_id: UserId,
        device_id: DeviceId,
    ) -> Result<Option<KnownDevice>, StoreError>;
    /// Non-revoked devices of the user.
    async fn devices(&self, user_id: UserId) -> Result<Vec<KnownDevice>, StoreError>;

    async fn invitation(
        &self,
        invitation_id: InvitationId,
    ) -> Result<Option<Invitation>, StoreError>;
    async fn invitation_by_token(
        &self,
        token_hash: &TokenHash,
    ) -> Result<Option<Invitation>, StoreError>;
    /// Open (neither accepted nor revoked) invitation for an e-mail, even if expired.
    async fn open_invitation_for(
        &self,
        email_normalized: &str,
    ) -> Result<Option<Invitation>, StoreError>;
    async fn invitations(&self) -> Result<Vec<Invitation>, StoreError>;

    async fn signup_code(&self, code_id: CodeId) -> Result<Option<SignupCode>, StoreError>;
    /// `resolve_one_time_code`: owner and purpose only.
    async fn one_time_code_owner(
        &self,
        code_id: CodeId,
    ) -> Result<Option<(UserId, Purpose)>, StoreError>;
    async fn one_time_code(
        &self,
        user_id: UserId,
        code_id: CodeId,
    ) -> Result<Option<OneTimeCode>, StoreError>;
    async fn change_codes(
        &self,
        user_id: UserId,
        change_id: ChannelChangeId,
    ) -> Result<Vec<OneTimeCode>, StoreError>;
    /// Step-up and channel-change codes the user requested since `since` (per-user send limit).
    async fn signed_in_sends_since(
        &self,
        user_id: UserId,
        since: DateTime<Utc>,
    ) -> Result<i64, StoreError>;

    async fn channel_change(
        &self,
        user_id: UserId,
        change_id: ChannelChangeId,
    ) -> Result<Option<ChannelChange>, StoreError>;
    /// The user's change in `awaiting_codes` or `on_hold`, if any.
    async fn open_channel_change(
        &self,
        user_id: UserId,
    ) -> Result<Option<ChannelChange>, StoreError>;
    /// The user's most recent change, whatever its status.
    async fn latest_channel_change(
        &self,
        user_id: UserId,
    ) -> Result<Option<ChannelChange>, StoreError>;
    /// Holds whose time has come, across users (worker role).
    async fn due_channel_holds(
        &self,
        now: DateTime<Utc>,
    ) -> Result<Vec<(UserId, ChannelChangeId)>, StoreError>;

    /// Counts one send in every window of the durable per-identifier limits (upsert per window)
    /// and returns the counts after it. Runs for registered and unknown identifiers alike.
    async fn count_send(
        &self,
        identifier: &CodeHmac,
        device_class: DeviceClass,
        windows: &[i32],
        now: DateTime<Utc>,
    ) -> Result<Vec<WindowCount>, StoreError>;
    /// Current counts without counting (cooldown check before a send).
    async fn send_counts(
        &self,
        identifier: &CodeHmac,
        device_class: DeviceClass,
        windows: &[i32],
        now: DateTime<Utc>,
    ) -> Result<Vec<WindowCount>, StoreError>;
    /// Runs the statements a login code insert runs, writing nothing (unknown identifiers, so
    /// statements and timing match up to the commit; policy §3).
    async fn login_code_placeholder(&self) -> Result<(), StoreError>;

    async fn sources(
        &self,
        user_id: UserId,
        include_revoked: bool,
    ) -> Result<Vec<Source>, StoreError>;
    async fn source(
        &self,
        user_id: UserId,
        source_id: SourceId,
    ) -> Result<Option<Source>, StoreError>;

    async fn connections(
        &self,
        user_id: UserId,
        include_unpaired: bool,
    ) -> Result<Vec<ConnectionRecord>, StoreError>;
    async fn connection(
        &self,
        user_id: UserId,
        connection_id: ConnectionId,
    ) -> Result<Option<ConnectionRecord>, StoreError>;
    /// Pairing code whose HMAC equals one of `candidates` and that is redeemable at `now`.
    async fn redeemable_pairing_code(
        &self,
        candidates: &[CodeHmac],
        now: DateTime<Utc>,
    ) -> Result<Option<PairingCode>, StoreError>;

    /// Connections of every user still marked `connected`/`outdated` (startup, worker role).
    async fn live_connections(&self) -> Result<Vec<(UserId, ConnectionId)>, StoreError>;

    /// Applies the change set atomically, with its audit records and outbox events.
    async fn commit(&self, changes: ChangeSet) -> Result<(), StoreError>;
}

/// Hashes and MACs of secrets (ALIGN-4, policy rev. 2 §2.3). Every keyed use has its own key,
/// derived from the versioned pepper with HKDF-SHA-256.
pub trait SecretHasher: Send + Sync {
    /// SHA-256 for 256-bit tokens (session, device, invitation, webhook, device token).
    fn sha256(&self, bytes: &[u8]) -> TokenHash;
    /// Pairing code HMAC under the current pepper.
    fn code_hmac(&self, code: &NormalizedCode) -> CodeHmac;
    /// Pairing code HMACs under every pepper still accepted.
    fn code_hmac_candidates(&self, code: &NormalizedCode) -> Vec<CodeHmac>;
    /// `HMAC(k_code, challenge_id ‖ code)` under the current pepper.
    fn otp_hmac(&self, code_id: CodeId, code: &NormalizedCode) -> CodeHmac;
    /// The same under a given pepper version (verification uses the row's version).
    fn otp_hmac_with(
        &self,
        code_id: CodeId,
        code: &NormalizedCode,
        version: i16,
    ) -> Option<CodeHmac>;
    /// `HMAC(k_ident, normalized identifier)` under the current pepper (send limits, §3).
    fn identifier_hmac(&self, normalized_identifier: &str) -> CodeHmac;
    /// Seals challenge claims with `k_challenge`; the result is URL-safe (`ChallengeId` pattern).
    fn seal_challenge(&self, claims: &ChallengeClaims) -> SecretString;
    /// Opens a sealed challenge; `None` for a forged, altered or malformed token.
    fn open_challenge(&self, token: &str) -> Option<ChallengeClaims>;
    /// Constant-time equality of two MACs.
    fn mac_eq(&self, a: &CodeHmac, b: &CodeHmac) -> bool;
}

/// Cryptographically secure randomness.
pub trait RandomSource: Send + Sync {
    fn fill(&self, buf: &mut [u8]);
}

#[derive(Debug, Error)]
#[error("e-mail delivery failed or disabled")]
pub struct MailError;

/// Transactional e-mail for links (invitations). Links are secrets: implementations never log them.
#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send_invitation(
        &self,
        to: &str,
        locale: Locale,
        invite_url: &SecretString,
    ) -> Result<(), MailError>;
}

/// Why a code is sent, for the message text (policy §2.3 "what it is for").
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodeMessage {
    Signup,
    Login {
        /// Coarse browser/OS of the requester.
        device_label: Option<String>,
        requested_at: DateTime<Utc>,
    },
    StepUp(identity_enterprise::otp::StepUpAction),
    ChannelChange {
        channel: Channel,
    },
}

/// A code on its way to a channel. `code` prints `***` in `Debug`.
#[derive(Clone, Debug)]
pub struct CodeDelivery {
    pub destination: Destination,
    pub locale: Locale,
    pub code: SecretString,
    pub message: CodeMessage,
}

#[derive(Debug, Error)]
pub enum DeliveryError {
    /// No provider configured for the channel (policy: DisabledSender until Q8/U17).
    #[error("delivery disabled")]
    Disabled,
    /// The provider refused or failed; the class is safe to audit (never the code or address).
    #[error("delivery failed: {0}")]
    Failed(String),
}

impl DeliveryError {
    pub fn class(&self) -> &str {
        match self {
            Self::Disabled => "disabled",
            Self::Failed(class) => class,
        }
    }
}

/// Sends one-time codes (e-mail now, WhatsApp with S3b). Never logs the code or the address.
#[async_trait]
pub trait CodeSender: Send + Sync {
    async fn send(&self, delivery: &CodeDelivery) -> Result<(), DeliveryError>;
}

/// Runs a delivery after the commit and after the HTTP response, detached from the request
/// (policy §2.3), then reports the outcome to [`crate::usecases::delivery::RecordDelivery`].
pub trait DeliveryQueue: Send + Sync {
    fn dispatch(&self, job: crate::usecases::delivery::DeliveryJob);
}

/// Gateway's view of the accounts a paired instance reports (query port owned by identity, wired in `app`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConnectionAccounts {
    pub visible_accounts: Vec<VisibleAccount>,
    /// Platform accounts with an open position (unpair confirmation).
    pub open_position_accounts: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisibleAccount {
    pub platform_account: String,
    /// `simulation`, `playback`, `live` or `unknown`.
    pub kind: String,
    /// `local`, `server` or `unknown`.
    pub oco_location: String,
    /// All Win account bound to it (S5).
    pub account_id: Option<uuid::Uuid>,
}

#[async_trait]
pub trait ConnectionPresence: Send + Sync {
    async fn accounts(
        &self,
        user_id: UserId,
        connections: &[ConnectionId],
    ) -> Result<HashMap<ConnectionId, ConnectionAccounts>, String>;
}

/// Ingest's view of when a source last delivered an instruction (query port).
#[async_trait]
pub trait SourceActivity: Send + Sync {
    async fn last_received(
        &self,
        user_id: UserId,
        sources: &[SourceId],
    ) -> Result<HashMap<SourceId, DateTime<Utc>>, String>;
}

/// Told about every source `key_id` that becomes valid in this process, after its commit
/// (create and rotate). `app` wires it to the ingress negative cache
/// (`adapter_tradingview::UnknownKeyCache::invalidate`), so a fresh key is never answered
/// as unknown.
pub trait SourceKeyListener: Send + Sync {
    fn key_activated(&self, key_id: &identity_enterprise::tokens::KeyId);
}

/// Accounts' answer to "is this account the user's?" (query port, wired in `app`); step-up targets
/// of the account actions must exist and belong to the caller (404 otherwise, nothing sent).
#[async_trait]
pub trait AccountTargets: Send + Sync {
    async fn owned_by(&self, user_id: UserId, account_id: uuid::Uuid) -> Result<bool, String>;
}

/// Which plugin versions may pair (426 `pairing.adapter_version_unsupported`).
pub trait AdapterCompatibility: Send + Sync {
    fn is_supported(&self, adapter: &AdapterInfo) -> bool;
}

/// Operational alerts (no user data): repeated delivery failures, global send threshold, a
/// channel change requested again after a cancellation.
pub trait SecurityAlerts: Send + Sync {
    fn raise(&self, alert: SecurityAlert);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecurityAlert {
    GlobalSendsExceeded { sends: usize },
    PrefixSendsExceeded,
    ChannelChangeAfterCancel { user_id: UserId },
}
