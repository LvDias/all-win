//! `ReleaseStore` on the presets catalog (§5.7), connected as `allwin_publisher`: it writes only
//! `catalog`, inserts published rows and renames identity rows, never deletes. One publication is
//! one transaction whose first statement is the publish lock, so publications are serial and
//! commit in `publish_seq` order (the accounts job's cursor never skips one).

use std::collections::{BTreeMap, HashMap};

use async_trait::async_trait;
use chrono::NaiveDate;
use serde_json::{Map, Value};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use catalog_application::enterprise::release::file::PresetKey;
use catalog_application::enterprise::release::import::{
    ImportOutcome, ReleasePlan, StoredFirm, StoredPreset, StoredRelease, StoredRuleText,
    StoredStance,
};
use catalog_application::release::{
    Planner, PublishError, PublishResult, PublishedBy, ReleaseStore,
};

#[derive(Clone)]
pub struct PgReleaseStore {
    pool: PgPool,
}

impl PgReleaseStore {
    /// `pool` connects as `allwin_publisher`.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn store_err(e: sqlx::Error) -> PublishError {
    PublishError::Store(e.to_string())
}

fn corrupt(what: &str) -> PublishError {
    PublishError::Store(format!("stored release: invalid {what}"))
}

#[async_trait]
impl ReleaseStore for PgReleaseStore {
    async fn publish(
        &self,
        slug: &str,
        published_by: &PublishedBy,
        plan: &Planner<'_>,
    ) -> Result<PublishResult, PublishError> {
        let mut tx = self.pool.begin().await.map_err(store_err)?;
        // Two-key space, apart from the one-key outbox consumer locks (Esmeril N4).
        sqlx::query!("SELECT pg_advisory_xact_lock(hashtext('catalog'), hashtext('publish'))")
            .fetch_one(&mut *tx)
            .await
            .map_err(store_err)?;
        let stored = load_firm(&mut tx, slug).await?;
        let outcome = plan(stored.as_ref()).map_err(PublishError::Rejected)?;
        let plan = match outcome {
            ImportOutcome::AlreadyPublished { version_number } => {
                tx.rollback().await.map_err(store_err)?;
                return Ok(PublishResult::AlreadyPublished { version_number });
            }
            ImportOutcome::Publish(plan) => plan,
        };
        let publish_seq = write_plan(&mut tx, &plan, stored.as_ref(), published_by).await?;
        tx.commit().await.map_err(store_err)?;
        Ok(PublishResult::Published {
            version_number: plan.version_number,
            publish_seq,
        })
    }
}

/// Everything stored for the firm `slug` that the import checks need.
async fn load_firm(
    conn: &mut PgConnection,
    slug: &str,
) -> Result<Option<StoredFirm>, PublishError> {
    let Some(firm) = sqlx::query!(
        "SELECT id, name FROM catalog.prop_firm WHERE slug = $1",
        slug
    )
    .fetch_optional(&mut *conn)
    .await
    .map_err(store_err)?
    else {
        return Ok(None);
    };
    let releases = sqlx::query!(
        "SELECT id, version_number, source_sha256 FROM catalog.firm_release WHERE firm_id = $1 ORDER BY version_number",
        firm.id
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(store_err)?;
    let Some(latest) = releases.last() else {
        return Err(corrupt("firm without a release"));
    };
    let latest_id = latest.id;
    let mut release_hashes = HashMap::new();
    for r in &releases {
        let hash: [u8; 32] = r
            .source_sha256
            .as_slice()
            .try_into()
            .map_err(|_| corrupt("source_sha256"))?;
        release_hashes.insert(r.version_number, hash);
    }

    // The text in force per slot is the highest version (unchanged texts keep their row).
    let texts = sqlx::query!(
        r#"SELECT DISTINCT ON (kind, firm_platform)
                  id, kind, firm_platform, version_number, firm_stance, quotes, research
           FROM catalog.opt_in_rule_version
           WHERE firm_id = $1 AND NOT is_generic
           ORDER BY kind, firm_platform, version_number DESC"#,
        firm.id
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(store_err)?;
    let rule_texts = texts
        .into_iter()
        .map(|t| {
            (
                (t.kind, t.firm_platform),
                StoredRuleText {
                    id: t.id,
                    version: t.version_number,
                    firm_stance: t.firm_stance,
                    quotes: t.quotes,
                    research: t.research,
                },
            )
        })
        .collect();

    let mut stances = HashMap::new();
    for s in sqlx::query!(
        "SELECT kind, value, quotes, research FROM catalog.firm_stance WHERE release_id = $1",
        latest_id
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(store_err)?
    {
        stances.insert(
            s.kind,
            StoredStance {
                value: s.value,
                quotes: s.quotes,
                research: s.research,
            },
        );
    }
    let mut stance = |kind: &str| stances.remove(kind).ok_or_else(|| corrupt(kind));
    let vps = stance("vps")?;
    let personal_device = stance("personal_device")?;

    let plans = sqlx::query!(
        "SELECT id, key, name FROM catalog.firm_plan WHERE firm_id = $1",
        firm.id
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(store_err)?
    .into_iter()
    .map(|p| (p.key, (p.id, p.name)))
    .collect();

    let presets_rows = sqlx::query!(
        r#"SELECT p.id, fp.key AS plan_key, p.phase, p.size_key, p.size_amount, p.size_label
           FROM catalog.preset p JOIN catalog.firm_plan fp ON fp.id = p.plan_id
           WHERE p.firm_id = $1"#,
        firm.id
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(store_err)?;
    let mut presets = HashMap::new();
    let mut key_of = HashMap::new();
    for p in presets_rows {
        let key = PresetKey {
            plan: p.plan_key,
            phase: p.phase,
            size: p.size_key,
        };
        key_of.insert(p.id, key.clone());
        presets.insert(
            key,
            StoredPreset {
                id: p.id,
                size_amount: p.size_amount,
                size_label: p.size_label,
            },
        );
    }

    let mut latest_presets: BTreeMap<PresetKey, BTreeMap<Option<String>, Map<String, Value>>> =
        BTreeMap::new();
    for rs in sqlx::query!(
        r#"SELECT pv.preset_id, rs.firm_platform, rs.rules
           FROM catalog.preset_version pv
           JOIN catalog.preset_rule_set rs ON rs.preset_version_id = pv.id
           WHERE pv.release_id = $1"#,
        latest_id
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(store_err)?
    {
        let key = key_of
            .get(&rs.preset_id)
            .ok_or_else(|| corrupt("preset version"))?;
        let Value::Object(rules) = rs.rules else {
            return Err(corrupt("rules"));
        };
        latest_presets
            .entry(key.clone())
            .or_default()
            .insert(rs.firm_platform, rules);
    }

    Ok(Some(StoredFirm {
        id: firm.id,
        name: firm.name,
        release_hashes,
        latest: StoredRelease {
            version_number: latest.version_number,
            rule_texts,
            vps,
            personal_device,
            presets: latest_presets,
        },
        plans,
        presets,
    }))
}

/// Inserts the plan's rows; returns the release's `publish_seq`.
async fn write_plan(
    conn: &mut PgConnection,
    plan: &ReleasePlan,
    stored: Option<&StoredFirm>,
    published_by: &PublishedBy,
) -> Result<i64, PublishError> {
    let firm_id = match plan.firm_id {
        Some(id) => {
            if stored.is_some_and(|f| f.name != plan.firm_name) {
                sqlx::query!(
                    "UPDATE catalog.prop_firm SET name = $2 WHERE id = $1",
                    id,
                    plan.firm_name
                )
                .execute(&mut *conn)
                .await
                .map_err(store_err)?;
            }
            id
        }
        None => {
            let id = Uuid::now_v7();
            sqlx::query!(
                "INSERT INTO catalog.prop_firm (id, slug, name) VALUES ($1, $2, $3)",
                id,
                plan.slug,
                plan.firm_name
            )
            .execute(&mut *conn)
            .await
            .map_err(store_err)?;
            id
        }
    };

    let release_id = Uuid::now_v7();
    let verified_on = NaiveDate::parse_from_str(&plan.verified_on, "%Y-%m-%d")
        .map_err(|_| PublishError::Store("verified_on".into()))?;
    let publish_seq = sqlx::query_scalar!(
        r#"INSERT INTO catalog.firm_release
             (id, firm_id, version_number, verified_on, research_file, source_sha256, published_by,
              changes, manual_only, excluded_plans)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
           RETURNING publish_seq"#,
        release_id,
        firm_id,
        plan.version_number,
        verified_on,
        plan.research_file,
        &plan.source_sha256[..],
        published_by.as_str(),
        plan.changes,
        plan.manual_only,
        plan.excluded_plans,
    )
    .fetch_one(&mut *conn)
    .await
    .map_err(store_err)?;

    for (code, label, manual_only) in &plan.platforms {
        sqlx::query!(
            "INSERT INTO catalog.firm_release_platform (release_id, code, label, manual_only) VALUES ($1, $2, $3, $4)",
            release_id,
            code,
            label,
            manual_only.as_ref(),
        )
        .execute(&mut *conn)
        .await
        .map_err(store_err)?;
    }
    for (kind, s) in [
        ("vps", &plan.vps),
        ("personal_device", &plan.personal_device),
    ] {
        sqlx::query!(
            "INSERT INTO catalog.firm_stance (release_id, kind, value, quotes, research) VALUES ($1, $2, $3, $4, $5)",
            release_id,
            kind,
            s.value,
            s.quotes,
            s.research.as_ref(),
        )
        .execute(&mut *conn)
        .await
        .map_err(store_err)?;
    }
    for t in &plan.new_rule_texts {
        sqlx::query!(
            r#"INSERT INTO catalog.opt_in_rule_version
                 (id, kind, firm_stance, is_generic, firm_id, firm_platform, version_number, quotes,
                  research, change_cause, change_reason, previous_id, release_id)
               VALUES ($1, $2, $3, false, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#,
            Uuid::now_v7(),
            t.kind,
            t.firm_stance,
            firm_id,
            t.firm_platform.as_deref(),
            t.version,
            t.quotes,
            t.research.as_ref(),
            t.change_cause.as_deref(),
            t.change_reason.as_ref(),
            t.previous_id,
            release_id,
        )
        .execute(&mut *conn)
        .await
        .map_err(store_err)?;
    }

    let mut plan_ids = HashMap::new();
    for p in &plan.plans {
        let id = match p.existing_id {
            Some(id) => {
                if stored
                    .and_then(|f| f.plans.get(&p.key))
                    .is_some_and(|(_, name)| *name != p.name)
                {
                    sqlx::query!(
                        "UPDATE catalog.firm_plan SET name = $2 WHERE id = $1",
                        id,
                        p.name
                    )
                    .execute(&mut *conn)
                    .await
                    .map_err(store_err)?;
                }
                id
            }
            None => {
                let id = Uuid::now_v7();
                sqlx::query!(
                    "INSERT INTO catalog.firm_plan (id, firm_id, key, name) VALUES ($1, $2, $3, $4)",
                    id,
                    firm_id,
                    p.key,
                    p.name
                )
                .execute(&mut *conn)
                .await
                .map_err(store_err)?;
                id
            }
        };
        plan_ids.insert(p.key.as_str(), id);
    }

    for w in &plan.presets {
        let plan_id = *plan_ids
            .get(w.plan_key.as_str())
            .ok_or_else(|| corrupt("plan key"))?;
        let preset_id = match w.existing_id {
            Some(id) => {
                let label_changed = stored
                    .and_then(|f| f.presets.values().find(|p| p.id == id))
                    .is_some_and(|p| p.size_label != w.size_label);
                if label_changed {
                    sqlx::query!(
                        "UPDATE catalog.preset SET size_label = $2 WHERE id = $1",
                        id,
                        w.size_label.as_deref()
                    )
                    .execute(&mut *conn)
                    .await
                    .map_err(store_err)?;
                }
                id
            }
            None => {
                let id = Uuid::now_v7();
                sqlx::query!(
                    r#"INSERT INTO catalog.preset (id, firm_id, plan_id, phase, size_amount, size_key, size_label)
                       VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
                    id,
                    firm_id,
                    plan_id,
                    w.phase,
                    w.size_amount,
                    w.size_key,
                    w.size_label.as_deref(),
                )
                .execute(&mut *conn)
                .await
                .map_err(store_err)?;
                id
            }
        };
        let version_id = Uuid::now_v7();
        sqlx::query!(
            r#"INSERT INTO catalog.preset_version (id, preset_id, firm_id, release_id, manual_only)
               VALUES ($1, $2, $3, $4, $5)"#,
            version_id,
            preset_id,
            firm_id,
            release_id,
            plan.manual_only.as_ref(),
        )
        .execute(&mut *conn)
        .await
        .map_err(store_err)?;
        for (platform, rules, stored_only) in &w.rule_sets {
            sqlx::query!(
                r#"INSERT INTO catalog.preset_rule_set (preset_version_id, firm_platform, rules, stored_only)
                   VALUES ($1, $2, $3, $4)"#,
                version_id,
                platform.as_deref(),
                rules,
                stored_only.as_ref(),
            )
            .execute(&mut *conn)
            .await
            .map_err(store_err)?;
        }
    }
    Ok(publish_seq)
}
