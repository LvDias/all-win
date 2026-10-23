//! The releases published after a cursor (Esquadro (B)): the accounts worker job reads them in
//! `publish_seq` order through its own port (adapter in `crates/app`) and suspends the opt-ins whose
//! rule text changed materially. Only `catalog` data; no user data.

use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::CatalogError;

/// One opt-in rule text that a release replaced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplacedRuleText {
    pub kind: String,
    pub firm_platform: Option<String>,
    pub previous_rule_version_id: Uuid,
    pub new_rule_version_id: Uuid,
    pub new_version_number: i32,
    /// `stance_changed` / `text_changed` suspend `chosen` opt-ins; `not_material` never does.
    pub cause: String,
}

impl ReplacedRuleText {
    pub fn suspends(&self) -> bool {
        matches!(self.cause.as_str(), "stance_changed" | "text_changed")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedRelease {
    pub publish_seq: i64,
    pub release_id: Uuid,
    pub firm_id: Uuid,
    pub firm_name: String,
    pub version_number: i32,
    pub replaced: Vec<ReplacedRuleText>,
    /// `(previous preset version, its version in this release)`: accounts on the previous one
    /// are told a newer version is available (S16 AC-3; never applied silently).
    pub superseded: Vec<(Uuid, Uuid)>,
}

#[async_trait]
pub trait ReleaseFeedReader: Send + Sync {
    /// Releases with `publish_seq > after`, ascending, at most `limit`.
    async fn published_after(
        &self,
        after: i64,
        limit: i64,
    ) -> Result<Vec<PublishedRelease>, CatalogError>;
}

/// Query port: `releases_published_after(publish_seq)`.
#[derive(Clone)]
pub struct ReleasesPublishedAfter {
    reader: Arc<dyn ReleaseFeedReader>,
}

impl ReleasesPublishedAfter {
    pub fn new(reader: Arc<dyn ReleaseFeedReader>) -> Self {
        Self { reader }
    }

    pub async fn handle(
        &self,
        after: i64,
        limit: i64,
    ) -> Result<Vec<PublishedRelease>, CatalogError> {
        self.reader
            .published_after(after.max(0), limit.clamp(1, 100))
            .await
    }
}
