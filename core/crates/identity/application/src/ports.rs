//! Ports implemented by `identity-infra` (or by other modules, wired in `app`).

use std::collections::HashMap;
use std::net::IpAddr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use identity_enterprise::codes::NormalizedCode;
use identity_enterprise::connection::{AdapterInfo, Connection, PairingCode};
use identity_enterprise::ids::{ConnectionId, InvitationId, SourceId};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::profile::Locale;
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::Session;
use identity_enterprise::source::Source;
use identity_enterprise::tokens::{CodeHmac, TokenHash};
use identity_enterprise::user::User;
use shared_kernel::UserId;
use thiserror::Error;

use crate::changes::ChangeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreConflict {
    EmailTaken,
    InvitationGone,
    NameTaken,
    KeyIdTaken,
    CodeHmacTaken,
    SourceRevoked,
    ConnectionUnpaired,
    PairingCodeGone,
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("store conflict: {0:?}")]
    Conflict(StoreConflict),
    /// Database or driver failure (detail for technical logs only).
    #[error("store unavailable: {0}")]
    Unavailable(String),
}

/// Admin listing row (identity fields only, no trading data).
#[derive(Clone, Debug, PartialEq)]
pub struct UserSummary {
    pub user: User,
    pub last_sign_in_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConnectionRecord {
    pub connection: Connection,
    /// Expiry of the active pairing code, if one is active.
    pub active_code_expires_at: Option<DateTime<Utc>>,
}

/// Persistence of the identity module. Reads of user-owned rows take the owner's `UserId`;
/// pre-authentication lookups (`*_by_token`, login, pairing) go through the narrow
/// `SECURITY DEFINER` functions of the data model §3.
#[async_trait]
pub trait IdentityStore: Send + Sync {
    async fn user_for_login(&self, email_normalized: &str) -> Result<Option<User>, StoreError>;
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
    /// Whether the user completed a sign-in from `ip` since `since` ("usual IPs", data model §4.1).
    async fn signed_in_from(
        &self,
        user_id: UserId,
        ip: IpAddr,
        since: DateTime<Utc>,
    ) -> Result<bool, StoreError>;

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

/// Hashes of secrets: SHA-256 for 256-bit tokens, HMAC-SHA-256 with a versioned pepper for
/// low-entropy codes (ALIGN-4).
pub trait SecretHasher: Send + Sync {
    fn sha256(&self, bytes: &[u8]) -> TokenHash;
    /// HMAC under the current pepper (for storing a new code).
    fn code_hmac(&self, code: &NormalizedCode) -> CodeHmac;
    /// HMACs under every pepper still accepted (for lookups across a pepper rotation).
    fn code_hmac_candidates(&self, code: &NormalizedCode) -> Vec<CodeHmac>;
}

/// Cryptographically secure randomness.
pub trait RandomSource: Send + Sync {
    fn fill(&self, buf: &mut [u8]);
}

#[derive(Debug, Error)]
#[error("e-mail delivery failed or disabled")]
pub struct MailError;

/// Transactional e-mail. Links are secrets: implementations never log them.
#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send_invitation(
        &self,
        to: &str,
        locale: Locale,
        invite_url: &SecretString,
    ) -> Result<(), MailError>;
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

/// Which plugin versions may pair (426 `pairing.adapter_version_unsupported`).
pub trait AdapterCompatibility: Send + Sync {
    fn is_supported(&self, adapter: &AdapterInfo) -> bool;
}
