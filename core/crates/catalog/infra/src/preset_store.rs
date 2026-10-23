//! `PresetReader` on the presets catalog (§5.7), as `allwin_app` (SELECT only; global, no RLS).

use async_trait::async_trait;
use uuid::Uuid;

use catalog_application::CatalogError;
use catalog_application::preset::{
    FirmData, FirmRow, PlanRow, PlatformRow, PresetReader, PresetRow, PresetVersionRow, ReleaseRow,
    RuleSetRow, RuleTextRow, StanceRow,
};

use crate::PgInstrumentRepository;

fn err(e: sqlx::Error) -> CatalogError {
    CatalogError(e.to_string())
}

#[async_trait]
impl PresetReader for PgInstrumentRepository {
    async fn firm_ids(&self) -> Result<Vec<Uuid>, CatalogError> {
        sqlx::query_scalar!(
            r#"SELECT f.id FROM catalog.prop_firm f
               WHERE EXISTS (SELECT 1 FROM catalog.firm_release r WHERE r.firm_id = f.id)
               ORDER BY lower(f.name), f.id"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(err)
    }

    async fn firm_data(&self, firm_id: Uuid) -> Result<FirmData, CatalogError> {
        let mut tx = self.pool.begin().await.map_err(err)?;
        let firm = sqlx::query_as!(
            FirmRow,
            "SELECT id, slug, name FROM catalog.prop_firm WHERE id = $1",
            firm_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(err)?;
        if firm.is_none() {
            return Ok(FirmData::default());
        }
        let releases = sqlx::query_as!(
            ReleaseRow,
            r#"SELECT id, version_number, published_at, verified_on, changes, manual_only, excluded_plans
               FROM catalog.firm_release WHERE firm_id = $1 ORDER BY version_number"#,
            firm_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        let platforms = sqlx::query_as!(
            PlatformRow,
            r#"SELECT p.release_id, p.code, p.label FROM catalog.firm_release_platform p
               JOIN catalog.firm_release r ON r.id = p.release_id WHERE r.firm_id = $1 ORDER BY p.code"#,
            firm_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        let stances = sqlx::query_as!(
            StanceRow,
            r#"SELECT s.release_id, s.kind, s.value, s.quotes, s.research FROM catalog.firm_stance s
               JOIN catalog.firm_release r ON r.id = s.release_id WHERE r.firm_id = $1"#,
            firm_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        let rule_texts = sqlx::query_as!(
            RuleTextRow,
            r#"SELECT id, release_id AS "release_id!", kind, firm_platform, version_number, firm_stance,
                      quotes, research, published_at
               FROM catalog.opt_in_rule_version WHERE firm_id = $1 AND NOT is_generic
               ORDER BY kind, firm_platform NULLS FIRST, version_number"#,
            firm_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        let plans = sqlx::query_as!(
            PlanRow,
            "SELECT id, key, name FROM catalog.firm_plan WHERE firm_id = $1 ORDER BY lower(name), id",
            firm_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        let presets = sqlx::query_as!(
            PresetRow,
            r#"SELECT id, plan_id, phase, size_key, size_amount, size_label FROM catalog.preset
               WHERE firm_id = $1"#,
            firm_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        let versions = sqlx::query_as!(
            PresetVersionRow,
            "SELECT id, preset_id, release_id, manual_only FROM catalog.preset_version WHERE firm_id = $1",
            firm_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        let rule_sets = sqlx::query_as!(
            RuleSetRow,
            r#"SELECT rs.preset_version_id, rs.firm_platform, rs.rules FROM catalog.preset_rule_set rs
               JOIN catalog.preset_version v ON v.id = rs.preset_version_id
               WHERE v.firm_id = $1 ORDER BY rs.firm_platform NULLS FIRST"#,
            firm_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        tx.commit().await.map_err(err)?;
        Ok(FirmData {
            firm,
            releases,
            platforms,
            stances,
            rule_texts,
            plans,
            presets,
            versions,
            rule_sets,
        })
    }

    async fn firm_of_preset_version(
        &self,
        preset_version_id: Uuid,
    ) -> Result<Option<Uuid>, CatalogError> {
        sqlx::query_scalar!(
            "SELECT firm_id FROM catalog.preset_version WHERE id = $1",
            preset_version_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(err)
    }
}

#[async_trait]
impl catalog_application::release_feed::ReleaseFeedReader for PgInstrumentRepository {
    async fn published_after(
        &self,
        after: i64,
        limit: i64,
    ) -> Result<Vec<catalog_application::release_feed::PublishedRelease>, CatalogError> {
        use catalog_application::release_feed::{PublishedRelease, ReplacedRuleText};
        let mut tx = self.pool.begin().await.map_err(err)?;
        let releases = sqlx::query!(
            r#"SELECT publish_seq, id, firm_id, version_number FROM catalog.firm_release
               WHERE publish_seq > $1 ORDER BY publish_seq LIMIT $2"#,
            after,
            limit
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        let ids: Vec<Uuid> = releases.iter().map(|r| r.id).collect();
        // A replaced text has a predecessor; a firm's first texts replace nothing.
        let replaced = sqlx::query!(
            r#"SELECT release_id AS "release_id!", kind, firm_platform, previous_id AS "previous_id!",
                      id, change_cause AS "change_cause!"
               FROM catalog.opt_in_rule_version
               WHERE release_id = ANY($1) AND previous_id IS NOT NULL"#,
            &ids
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
        tx.commit().await.map_err(err)?;
        Ok(releases
            .into_iter()
            .map(|r| PublishedRelease {
                publish_seq: r.publish_seq,
                release_id: r.id,
                firm_id: r.firm_id,
                version_number: r.version_number,
                replaced: replaced
                    .iter()
                    .filter(|t| t.release_id == r.id)
                    .map(|t| ReplacedRuleText {
                        kind: t.kind.clone(),
                        firm_platform: t.firm_platform.clone(),
                        previous_rule_version_id: t.previous_id,
                        new_rule_version_id: t.id,
                        cause: t.change_cause.clone(),
                    })
                    .collect(),
            })
            .collect())
    }
}
