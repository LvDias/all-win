//! Consumer-owned ports, implemented in `app` by the owning modules
//! (`identity` for source keys, `catalog` for instruments).

use async_trait::async_trait;
use routing_application::enterprise::SourceId;
use shared_kernel::UserId;

/// Failure of a port; the message is for the technical log only and never holds a secret.
#[derive(Debug)]
pub struct PortError(pub String);

impl std::fmt::Display for PortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PortError {}

/// What `identity.resolve_source_key(key_id)` returns (wave1-data-model §3).
#[derive(Clone)]
pub struct ResolvedSourceKey {
    pub user_id: UserId,
    pub source_id: SourceId,
    pub source_type: String,
    /// SHA-256 of the 32 decoded secret bytes.
    pub secret_hash: Vec<u8>,
    /// Key revoked, or its source archived.
    pub revoked: bool,
}

impl std::fmt::Debug for ResolvedSourceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolvedSourceKey")
            .field("user_id", &self.user_id)
            .field("source_id", &self.source_id)
            .field("source_type", &self.source_type)
            .field("revoked", &self.revoked)
            .finish_non_exhaustive()
    }
}

#[async_trait]
pub trait SourceKeyResolver: Send + Sync {
    /// Pre-authentication lookup by the public `key_id`.
    async fn resolve(&self, key_id: &str) -> Result<Option<ResolvedSourceKey>, PortError>;
}

#[async_trait]
pub trait InstrumentSupport: Send + Sync {
    /// Is the neutral root symbol enabled in the instrument catalog?
    async fn is_enabled(&self, symbol: &str) -> Result<bool, PortError>;
}
