//! Adapter (device token) authentication for the gateway's WebSocket (S2, ADR-0004): the token
//! format and its hash stay in identity; the gateway only sees the principal.

use identity_enterprise::ids::ConnectionId;
use identity_enterprise::profile::Locale;
use identity_enterprise::secret::SecretString;
use identity_enterprise::tokens::{KeyId, PrefixedToken, PrefixedTokenKind};
use shared_kernel::UserId;
use subtle::ConstantTimeEq;

use crate::deps::IdentityDeps;
use crate::error::IdentityError;

/// Who an accepted device token speaks for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdapterPrincipal {
    pub user_id: UserId,
    pub connection_id: ConnectionId,
    pub key_id: KeyId,
    /// The user's locale (the gateway's `welcome.locale`).
    pub locale: Locale,
}

/// Checks a presented `awd_<key_id>_<secret>` token. `None` for a malformed, unknown, wrong or
/// revoked token, all alike.
pub struct AuthenticateDeviceToken {
    deps: IdentityDeps,
}

impl AuthenticateDeviceToken {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        presented: &SecretString,
    ) -> Result<Option<AdapterPrincipal>, IdentityError> {
        let deps = &self.deps;
        let Some(token) = PrefixedToken::parse(PrefixedTokenKind::Device, presented.expose())
        else {
            return Ok(None);
        };
        let Some(row) = deps.store.device_token(token.key_id()).await? else {
            return Ok(None);
        };
        let hash = deps.hasher.sha256(token.secret().as_bytes());
        let matches: bool = hash.as_bytes().ct_eq(row.token_hash.as_bytes()).into();
        if !matches || row.revoked {
            return Ok(None);
        }
        let Some(user) = deps.store.user(row.user_id).await? else {
            return Ok(None);
        };
        Ok(Some(AdapterPrincipal {
            user_id: row.user_id,
            connection_id: row.connection_id,
            key_id: token.key_id().clone(),
            locale: user.locale,
        }))
    }
}

/// The gateway's periodic revalidation of an open session: the token is still valid.
pub struct DeviceTokenStatus {
    deps: IdentityDeps,
}

impl DeviceTokenStatus {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, key_id: &KeyId) -> Result<bool, IdentityError> {
        Ok(self
            .deps
            .store
            .device_token(key_id)
            .await?
            .is_some_and(|row| !row.revoked))
    }
}
