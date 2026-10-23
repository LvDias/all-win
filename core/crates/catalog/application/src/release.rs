//! `publish-release` (S16 §2.11): imports one firm release file, all or nothing. Run by the admin
//! through the Core's CLI as `allwin_publisher`; never an HTTP route. The publisher writes only
//! `catalog`; the accounts job reads new releases by `publish_seq` (Esquadro (B)).

use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use catalog_enterprise::release::file::ReleaseFile;
use catalog_enterprise::release::import::{
    ImportOutcome, ImportPolicy, ImportProblem, StoredFirm, plan_import,
};

/// A release file the schema accepted (`contracts/presets/v1/release.schema.json`), with its
/// canonical hash. Built only by the infra parser.
#[derive(Debug, Clone)]
pub struct ValidatedRelease {
    pub file: ReleaseFile,
    pub raw: Value,
    pub sha256: [u8; 32],
}

/// Operator label recorded with the release (`^[a-z0-9][a-z0-9._-]{0,63}$`; never an e-mail).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedBy(String);

impl PublishedBy {
    pub fn parse(raw: &str) -> Option<Self> {
        let b = raw.as_bytes();
        let ok = (1..=64).contains(&b.len())
            && (b[0].is_ascii_lowercase() || b[0].is_ascii_digit())
            && b.iter().all(|c| {
                c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, b'.' | b'_' | b'-')
            });
        ok.then(|| Self(raw.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishResult {
    Published {
        version_number: i32,
        publish_seq: i64,
    },
    /// The same file was already published: nothing changed.
    AlreadyPublished { version_number: i32 },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PublishError {
    #[error("release refused: {0:?}")]
    Rejected(Vec<ImportProblem>),
    #[error("store: {0}")]
    Store(String),
}

/// The planning step run inside the store's transaction, on the snapshot it read.
pub type Planner<'a> =
    dyn Fn(Option<&StoredFirm>) -> Result<ImportOutcome, Vec<ImportProblem>> + Send + Sync + 'a;

#[async_trait]
pub trait ReleaseStore: Send + Sync {
    /// In one transaction, serialized with every other publication: reads the firm with `slug`,
    /// runs `plan`, and writes the plan's rows (or nothing).
    async fn publish(
        &self,
        slug: &str,
        published_by: &PublishedBy,
        plan: &Planner<'_>,
    ) -> Result<PublishResult, PublishError>;
}

pub struct PublishRelease {
    store: Arc<dyn ReleaseStore>,
    policy: ImportPolicy,
}

impl PublishRelease {
    pub fn new(store: Arc<dyn ReleaseStore>, policy: ImportPolicy) -> Self {
        Self { store, policy }
    }

    pub async fn handle(
        &self,
        release: &ValidatedRelease,
        published_by: &PublishedBy,
    ) -> Result<PublishResult, PublishError> {
        let policy = self.policy;
        let planner = |stored: Option<&StoredFirm>| {
            plan_import(&release.file, &release.raw, release.sha256, stored, policy)
        };
        self.store
            .publish(&release.file.firm.slug, published_by, &planner)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn published_by_is_a_label() {
        for ok in ["torno", "ops.admin-1", "a", "0x"] {
            assert!(PublishedBy::parse(ok).is_some(), "{ok}");
        }
        for bad in ["", "Torno", "a@b.com", "-x", ".x", "a b", &"a".repeat(65)] {
            assert!(PublishedBy::parse(bad).is_none(), "{bad}");
        }
    }
}
