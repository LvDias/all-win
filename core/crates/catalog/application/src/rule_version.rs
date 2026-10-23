//! Opt-in rule versions (`catalog.opt_in_rule_version`; S5 contract §2.5, Q40, Q42). Published
//! texts are immutable: a correction is a new version with a `change` cause.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::CatalogError;

/// The bilingual reason of a material change (`LocalizedText`), as published.
#[derive(Debug, Clone, PartialEq)]
pub struct RuleChange {
    pub previous_rule_version_id: Uuid,
    pub previous_firm_stance: String,
    pub cause: String,
    pub reason: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleVersion {
    pub id: Uuid,
    /// `automation` / `copy`.
    pub kind: String,
    pub firm_stance: String,
    pub firm_platform: Option<String>,
    pub is_generic: bool,
    pub firm_id: Option<Uuid>,
    pub version_number: i32,
    pub published_at: DateTime<Utc>,
    /// `RuleQuote[]`, validated by the publisher's use case.
    pub quotes: Value,
    pub research: Option<Value>,
    pub change: Option<RuleChange>,
}

impl RuleVersion {
    /// `(url, accessed_at)` of each quote, for the audit of a choice (spec O6).
    pub fn sources(&self) -> Vec<(String, String)> {
        self.quotes
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|q| {
                Some((
                    q.get("url")?.as_str()?.to_owned(),
                    q.get("accessed_at")?.as_str()?.to_owned(),
                ))
            })
            .collect()
    }
}

pub const KINDS: [&str; 2] = ["automation", "copy"];

#[async_trait]
pub trait RuleVersionRepository: Send + Sync {
    async fn by_id(&self, id: Uuid) -> Result<Option<RuleVersion>, CatalogError>;

    /// The highest generic version of a kind.
    async fn current_generic(&self, kind: &str) -> Result<Option<RuleVersion>, CatalogError>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum OptInRulesError {
    /// 404: the preset version does not exist (presets come with S16).
    #[error("common.not_found")]
    PresetVersionUnknown,
    /// 400 `common.invalid_request` (`/query/firm_platform`): a platform without a preset is
    /// an invalid combination, as in `createAccount`.
    #[error("common.invalid_request")]
    FirmPlatformWithoutPreset,
    /// 422 `accounts.firm_platform_unknown` (`/query/firm_platform`): only with a preset (S16).
    #[error("accounts.firm_platform_unknown")]
    FirmPlatformUnknown,
    #[error("{0}")]
    Catalog(String),
}

/// `getOptInRules` / `getRuleVersion` and the accounts query port.
#[derive(Clone)]
pub struct RuleVersionQueries {
    repo: Arc<dyn RuleVersionRepository>,
}

impl RuleVersionQueries {
    pub fn new(repo: Arc<dyn RuleVersionRepository>) -> Self {
        Self { repo }
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<RuleVersion>, CatalogError> {
        self.repo.by_id(id).await
    }

    /// The current version of `kind` for a selection. Without a preset: the generic one.
    /// Preset versions have no catalog before S16, so a preset selection has none.
    pub async fn current(
        &self,
        preset_version_id: Option<Uuid>,
        kind: &str,
    ) -> Result<Option<RuleVersion>, CatalogError> {
        match preset_version_id {
            None => self.repo.current_generic(kind).await,
            Some(_) => Ok(None),
        }
    }

    /// `getOptInRules`: one version per kind.
    pub async fn for_selection(
        &self,
        preset_version_id: Option<Uuid>,
        firm_platform: Option<&str>,
    ) -> Result<[RuleVersion; 2], OptInRulesError> {
        // 400 before 404.
        if preset_version_id.is_none() && firm_platform.is_some() {
            return Err(OptInRulesError::FirmPlatformWithoutPreset);
        }
        if preset_version_id.is_some() {
            return Err(OptInRulesError::PresetVersionUnknown);
        }
        let get = |kind| async move {
            self.repo
                .current_generic(kind)
                .await
                .map_err(|e| OptInRulesError::Catalog(e.0))?
                .ok_or_else(|| OptInRulesError::Catalog(format!("no generic {kind} rule")))
        };
        Ok([get(KINDS[0]).await?, get(KINDS[1]).await?])
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use serde_json::json;

    struct Generic;

    fn generic(kind: &str, n: u128) -> RuleVersion {
        RuleVersion {
            id: Uuid::from_u128(n),
            kind: kind.into(),
            firm_stance: "unknown".into(),
            firm_platform: None,
            is_generic: true,
            firm_id: None,
            version_number: 1,
            published_at: DateTime::<Utc>::UNIX_EPOCH,
            quotes: json!([]),
            research: None,
            change: None,
        }
    }

    #[async_trait]
    impl RuleVersionRepository for Generic {
        async fn by_id(&self, id: Uuid) -> Result<Option<RuleVersion>, CatalogError> {
            Ok((id == Uuid::from_u128(1)).then(|| generic("automation", 1)))
        }
        async fn current_generic(&self, kind: &str) -> Result<Option<RuleVersion>, CatalogError> {
            Ok(Some(generic(
                kind,
                if kind == "automation" { 1 } else { 2 },
            )))
        }
    }

    fn queries() -> RuleVersionQueries {
        RuleVersionQueries::new(Arc::new(Generic))
    }

    #[tokio::test]
    async fn without_a_preset_the_generic_versions_apply() {
        let [a, c] = queries().for_selection(None, None).await.unwrap();
        assert_eq!((a.kind.as_str(), c.kind.as_str()), ("automation", "copy"));
        assert!(a.sources().is_empty());
        assert_eq!(
            queries()
                .for_selection(None, Some("rithmic"))
                .await
                .unwrap_err(),
            OptInRulesError::FirmPlatformWithoutPreset
        );
        // Preset versions have no catalog before S16.
        assert_eq!(
            queries()
                .for_selection(Some(Uuid::nil()), None)
                .await
                .unwrap_err(),
            OptInRulesError::PresetVersionUnknown
        );
        assert!(
            queries()
                .current(Some(Uuid::nil()), "automation")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn sources_come_from_the_quotes() {
        let mut r = generic("automation", 1);
        r.quotes = json!([{"text": "t", "language": "en", "page_title": "p",
                           "url": "https://firm.example/rules", "accessed_at": "2026-10-01"}]);
        assert_eq!(
            r.sources(),
            vec![(
                "https://firm.example/rules".to_owned(),
                "2026-10-01".to_owned()
            )]
        );
    }
}
