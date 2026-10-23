//! Dependencies shared by every use case (wired once in `app`).

use std::net::IpAddr;
use std::sync::Arc;

use identity_enterprise::secret::SecretString;
use identity_enterprise::tokens::{KeyId, SecretBytes, TokenHash};
use shared_kernel::Clock;

use crate::ports::{
    AdapterCompatibility, ConnectionPresence, IdentityStore, Mailer, RandomSource, SecretHasher,
    SourceActivity, SourceKeyListener,
};
use crate::throttle::{PairingThrottle, PublicEndpointThrottle};

#[derive(Clone, Debug)]
pub struct IdentitySettings {
    /// Public origin of the app, e.g. `https://app.example.com` (no trailing slash). Used to build
    /// invitation and webhook URLs.
    pub public_origin: String,
}

#[derive(Clone)]
pub struct IdentityDeps {
    pub store: Arc<dyn IdentityStore>,
    pub clock: Arc<dyn Clock>,
    pub hasher: Arc<dyn SecretHasher>,
    pub random: Arc<dyn RandomSource>,
    pub mailer: Arc<dyn Mailer>,
    pub presence: Arc<dyn ConnectionPresence>,
    pub source_activity: Arc<dyn SourceActivity>,
    pub source_keys: Arc<dyn SourceKeyListener>,
    pub adapters: Arc<dyn AdapterCompatibility>,
    pub pairing_throttle: Arc<dyn PairingThrottle>,
    pub public_throttle: Arc<dyn PublicEndpointThrottle>,
    pub settings: IdentitySettings,
}

/// Request facts the use cases need (never the raw URL or headers).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestMeta {
    pub ip: IpAddr,
    pub user_agent: Option<String>,
}

/// A fresh 256-bit token: the value handed out once, and the hash that is stored.
pub(crate) struct IssuedToken {
    pub secret: SecretBytes,
    pub hash: TokenHash,
}

impl IssuedToken {
    pub fn value(&self) -> SecretString {
        self.secret.encode()
    }
}

impl IdentityDeps {
    pub(crate) fn random_bytes<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        self.random.fill(&mut buf);
        buf
    }

    pub(crate) fn issue_token(&self) -> IssuedToken {
        let secret = SecretBytes::from_random(self.random_bytes());
        let hash = self.hasher.sha256(secret.as_bytes());
        IssuedToken { secret, hash }
    }

    /// Hash of a token presented by a client; `None` when it is not a well-formed token.
    pub(crate) fn hash_presented_token(&self, presented: &str) -> Option<TokenHash> {
        SecretBytes::decode(presented).map(|s| self.hasher.sha256(s.as_bytes()))
    }

    pub(crate) fn new_key_id(&self) -> KeyId {
        KeyId::generate(|buf| self.random.fill(buf))
    }
}
