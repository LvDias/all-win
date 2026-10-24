//! Reading the presets catalog (S16; contract `modules/presets.yaml`): `listFirms`, `getFirm`,
//! `listFirmReleases`, `getPresetVersion`, and the preset facts the accounts module reads through
//! its own port. Global data: any signed-in user reads it, nothing is user-scoped.
//!
//! The rules and quotes are published documents (JSON) and pass through as stored. Release
//! `changes` are stored with the release file's keys and shown with ids.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, SecondsFormat, Utc};
use rust_decimal::Decimal;
use serde_json::{Map, Value, json};
use uuid::Uuid;

use crate::CatalogError;

// ---- rows the reader returns ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct FirmRow {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReleaseRow {
    pub id: Uuid,
    pub version_number: i32,
    pub published_at: DateTime<Utc>,
    pub verified_on: NaiveDate,
    pub changes: Value,
    pub manual_only: Option<Value>,
    pub excluded_plans: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlatformRow {
    pub release_id: Uuid,
    pub code: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StanceRow {
    pub release_id: Uuid,
    pub kind: String,
    pub value: String,
    pub quotes: Value,
    pub research: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleTextRow {
    pub id: Uuid,
    pub release_id: Uuid,
    pub kind: String,
    pub firm_platform: Option<String>,
    pub version_number: i32,
    pub firm_stance: String,
    pub quotes: Value,
    pub research: Option<Value>,
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanRow {
    pub id: Uuid,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PresetRow {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub phase: String,
    pub size_key: String,
    pub size_amount: Decimal,
    pub size_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PresetVersionRow {
    pub id: Uuid,
    pub preset_id: Uuid,
    pub release_id: Uuid,
    pub manual_only: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleSetRow {
    pub preset_version_id: Uuid,
    pub firm_platform: Option<String>,
    pub rules: Value,
}

/// Everything published for one firm.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FirmData {
    pub firm: Option<FirmRow>,
    /// Ascending by version.
    pub releases: Vec<ReleaseRow>,
    pub platforms: Vec<PlatformRow>,
    pub stances: Vec<StanceRow>,
    pub rule_texts: Vec<RuleTextRow>,
    pub plans: Vec<PlanRow>,
    pub presets: Vec<PresetRow>,
    pub versions: Vec<PresetVersionRow>,
    pub rule_sets: Vec<RuleSetRow>,
}

#[async_trait]
pub trait PresetReader: Send + Sync {
    /// Firms with at least one release, by name.
    async fn firm_ids(&self) -> Result<Vec<Uuid>, CatalogError>;
    async fn firm_data(&self, firm_id: Uuid) -> Result<FirmData, CatalogError>;
    async fn firm_of_preset_version(
        &self,
        preset_version_id: Uuid,
    ) -> Result<Option<Uuid>, CatalogError>;
}

/// What another module needs from a preset version (accounts reads it through its own
/// `PresetCatalog` port; the adapter in `crates/app` maps it).
#[derive(Debug, Clone, PartialEq)]
pub struct PresetVersionFacts {
    pub preset_version_id: Uuid,
    pub preset_id: Uuid,
    pub firm_id: Uuid,
    pub firm_name: String,
    pub plan_name: String,
    pub phase: String,
    pub size_amount: Decimal,
    /// The published maximum position when verified; `None` when not confirmed.
    pub max_position_qty: Option<i64>,
    /// The cap when not confirmed: the highest published value (S16 §2.10).
    pub max_position_cap: Option<i64>,
    /// Firm platforms with their own rule set; empty when one set covers every platform.
    pub firm_platforms: Vec<String>,
    /// The same preset's version in the latest release, when newer than this one.
    pub superseded_by: Option<Uuid>,
    pub has_evaluation_target: bool,
    /// S16 AC-7 reason (RuleQuote) when the preset is for manual tracking only.
    pub manual_only: Option<Value>,
    /// `(firm_platform, PresetRules)` as the contract shows them (without the release files'
    /// quotes).
    pub rule_sets: Vec<(Option<String>, Value)>,
    pub release_version_number: i32,
    pub release_published_at: DateTime<Utc>,
    pub release_verified_on: NaiveDate,
}

fn contracts_of(v: &Value) -> Option<i64> {
    v.get("contracts").and_then(Value::as_i64)
}

// ---- use cases ------------------------------------------------------------------------------

#[derive(Clone)]
pub struct PresetQueries {
    reader: Arc<dyn PresetReader>,
}

impl PresetQueries {
    pub fn new(reader: Arc<dyn PresetReader>) -> Self {
        Self { reader }
    }

    /// `listFirms` (`FirmList`).
    pub async fn list_firms(&self) -> Result<Value, CatalogError> {
        let mut firms = Vec::new();
        for id in self.reader.firm_ids().await? {
            let data = self.reader.firm_data(id).await?;
            if let Some(view) = FirmView::of(&data) {
                firms.push(view.summary());
            }
        }
        Ok(json!({"firms": firms}))
    }

    /// `getFirm` (`FirmDetail`); `None` → 404.
    pub async fn get_firm(&self, firm_id: Uuid) -> Result<Option<Value>, CatalogError> {
        let data = self.reader.firm_data(firm_id).await?;
        Ok(FirmView::of(&data).map(|v| v.detail()))
    }

    /// `listFirmReleases` (`FirmReleaseList`), newest first; `None` → 404.
    pub async fn list_releases(&self, firm_id: Uuid) -> Result<Option<Value>, CatalogError> {
        let data = self.reader.firm_data(firm_id).await?;
        Ok(FirmView::of(&data).map(|v| v.releases()))
    }

    /// Facts of a preset version for the accounts port; `None` when it does not exist.
    pub async fn version_facts(
        &self,
        preset_version_id: Uuid,
    ) -> Result<Option<PresetVersionFacts>, CatalogError> {
        let Some(firm_id) = self
            .reader
            .firm_of_preset_version(preset_version_id)
            .await?
        else {
            return Ok(None);
        };
        let data = self.reader.firm_data(firm_id).await?;
        Ok(FirmView::of(&data).and_then(|v| v.facts(preset_version_id)))
    }

    /// `getPresetVersion` (`PresetVersion`); `None` → 404.
    pub async fn get_preset_version(
        &self,
        preset_version_id: Uuid,
    ) -> Result<Option<Value>, CatalogError> {
        let Some(firm_id) = self
            .reader
            .firm_of_preset_version(preset_version_id)
            .await?
        else {
            return Ok(None);
        };
        let data = self.reader.firm_data(firm_id).await?;
        Ok(FirmView::of(&data).and_then(|v| v.preset_version(preset_version_id)))
    }
}

// ---- assembly -------------------------------------------------------------------------------

fn ts(t: DateTime<Utc>) -> String {
    t.to_rfc3339_opts(SecondsFormat::AutoSi, true)
}

/// `SizeAmount` on the wire: 2 decimals.
fn size_wire(d: Decimal) -> String {
    let mut v = d;
    v.rescale(2);
    v.to_string()
}

/// Number of distinct rule fields that are `not_confirmed` in any of the rule sets.
fn not_confirmed_count<'a>(sets: impl Iterator<Item = &'a Value>) -> usize {
    let mut fields = BTreeSet::new();
    for rules in sets {
        for (field, v) in rules.as_object().into_iter().flatten() {
            if v.get("status").and_then(Value::as_str) == Some("not_confirmed") {
                fields.insert(field.clone());
            }
        }
    }
    fields.len()
}

/// One phase of a plan with its size options (and their amounts, for ordering).
type PhaseSizes = (String, Vec<(Decimal, Value)>);

/// Every `sources` entry as HTTP `SourceRef`: the file's per-source `quote` is import evidence.
fn without_source_quotes(v: &Value) -> Value {
    match v {
        Value::Object(m) => Value::Object(
            m.iter()
                .map(|(k, child)| {
                    let child = match (k.as_str(), child) {
                        ("sources", Value::Array(items)) => Value::Array(
                            items
                                .iter()
                                .map(|i| match i {
                                    Value::Object(src) => Value::Object(
                                        src.iter()
                                            .filter(|(k, _)| k.as_str() != "quote")
                                            .map(|(k, v)| (k.clone(), v.clone()))
                                            .collect(),
                                    ),
                                    other => other.clone(),
                                })
                                .collect(),
                        ),
                        _ => without_source_quotes(child),
                    };
                    (k.clone(), child)
                })
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.iter().map(without_source_quotes).collect()),
        other => other.clone(),
    }
}

/// `research` as HTTP `SourceRef`s: the file's per-source `quote` is import evidence, not shown.
fn research_json(r: &Value) -> Value {
    let mut out = r.clone();
    if let Some(sources) = out.get_mut("sources").and_then(Value::as_array_mut) {
        for s in sources.iter_mut().filter_map(Value::as_object_mut) {
            s.remove("quote");
        }
    }
    out
}

struct FirmView<'a> {
    d: &'a FirmData,
    firm: &'a FirmRow,
    latest: &'a ReleaseRow,
}

impl<'a> FirmView<'a> {
    fn of(d: &'a FirmData) -> Option<Self> {
        Some(Self {
            d,
            firm: d.firm.as_ref()?,
            latest: d.releases.last()?,
        })
    }

    fn release_ref(r: &ReleaseRow) -> Value {
        json!({"version_number": r.version_number, "published_at": ts(r.published_at), "verified_on": r.verified_on.to_string()})
    }

    /// Rule texts in force at `release` (the highest version per slot published up to it).
    fn texts_at(&self, release: &ReleaseRow) -> Vec<&'a RuleTextRow> {
        let upto: BTreeSet<Uuid> = self
            .d
            .releases
            .iter()
            .filter(|r| r.version_number <= release.version_number)
            .map(|r| r.id)
            .collect();
        let mut best: BTreeMap<(String, Option<String>), &RuleTextRow> = BTreeMap::new();
        for t in self
            .d
            .rule_texts
            .iter()
            .filter(|t| upto.contains(&t.release_id))
        {
            let slot = (t.kind.clone(), t.firm_platform.clone());
            if best
                .get(&slot)
                .is_none_or(|b| b.version_number < t.version_number)
            {
                best.insert(slot, t);
            }
        }
        best.into_values().collect()
    }

    fn rule_version(&self, t: &RuleTextRow) -> Value {
        let mut v = json!({
            "id": t.id, "kind": t.kind, "firm_stance": t.firm_stance, "is_generic": false,
            "firm_id": self.firm.id, "firm_name": self.firm.name, "version_number": t.version_number,
            "published_at": ts(t.published_at), "quotes": t.quotes,
        });
        if let Some(p) = &t.firm_platform {
            v["firm_platform"] = json!(p);
        }
        if let Some(r) = &t.research {
            v["research"] = research_json(r);
        }
        v
    }

    fn stance(&self, release: &ReleaseRow, kind: &str) -> Option<&'a StanceRow> {
        self.d
            .stances
            .iter()
            .find(|s| s.release_id == release.id && s.kind == kind)
    }

    fn stance_info(s: Option<&StanceRow>) -> Value {
        match s {
            Some(s) => {
                let mut v = json!({"value": s.value, "quotes": s.quotes});
                if let Some(r) = &s.research {
                    v["research"] = research_json(r);
                }
                v
            }
            None => json!({"value": "unknown", "quotes": []}),
        }
    }

    fn stance_summary(texts: &[&RuleTextRow], kind: &str) -> String {
        let stances: BTreeSet<&str> = texts
            .iter()
            .filter(|t| t.kind == kind)
            .map(|t| t.firm_stance.as_str())
            .collect();
        match stances.len() {
            1 => stances.into_iter().next().unwrap_or("unknown").to_owned(),
            0 => "unknown".to_owned(),
            _ => "varies_by_platform".to_owned(),
        }
    }

    fn versions_of(&self, release: &ReleaseRow) -> Vec<&'a PresetVersionRow> {
        self.d
            .versions
            .iter()
            .filter(|v| v.release_id == release.id)
            .collect()
    }

    fn sets_of(&self, version_id: Uuid) -> Vec<&'a RuleSetRow> {
        self.d
            .rule_sets
            .iter()
            .filter(|s| s.preset_version_id == version_id)
            .collect()
    }

    fn preset(&self, id: Uuid) -> Option<&'a PresetRow> {
        self.d.presets.iter().find(|p| p.id == id)
    }

    fn plan(&self, id: Uuid) -> Option<&'a PlanRow> {
        self.d.plans.iter().find(|p| p.id == id)
    }

    fn summary(&self) -> Value {
        let texts = self.texts_at(self.latest);
        let versions = self.versions_of(self.latest);
        let mut plan_ids: Vec<Uuid> = Vec::new();
        let mut count = 0;
        for v in &versions {
            if let Some(p) = self.preset(v.preset_id)
                && !plan_ids.contains(&p.plan_id)
            {
                plan_ids.push(p.plan_id);
            }
            count += not_confirmed_count(self.sets_of(v.id).iter().map(|s| &s.rules));
        }
        let plan_names: Vec<&str> = plan_ids
            .iter()
            .filter_map(|id| self.plan(*id))
            .map(|p| p.name.as_str())
            .collect();
        let device = self
            .stance(self.latest, "personal_device")
            .map_or("unknown", |s| s.value.as_str());
        let mut v = json!({
            "id": self.firm.id, "slug": self.firm.slug, "name": self.firm.name,
            "plan_names": plan_names,
            "stances": {
                "automation": Self::stance_summary(&texts, "automation"),
                "copy": Self::stance_summary(&texts, "copy"),
                "vps": self.stance(self.latest, "vps").map_or("unknown", |s| s.value.as_str()),
                // HTTP 5.1.1 has no not_confirmed here (5.4.0 adds it; the import refuses it until then).
                "personal_device": device,
            },
            "not_confirmed_count": count,
            "release": Self::release_ref(self.latest),
        });
        if let Some(m) = &self.latest.manual_only {
            v["manual_only"] = m.clone();
        }
        v
    }

    fn detail(&self) -> Value {
        let texts = self.texts_at(self.latest);
        let platforms: Vec<Value> = self
            .d
            .platforms
            .iter()
            .filter(|p| p.release_id == self.latest.id)
            .map(|p| json!({"code": p.code, "label": p.label}))
            .collect();
        // plan → phase → sizes, in plan then phase order of first appearance, sizes by amount.
        let mut plans: Vec<(Uuid, Vec<PhaseSizes>)> = Vec::new();
        // Plans and phases in the file's order: versions are inserted in file order with
        // time-ordered (v7) ids. Sizes ascending by amount (Compasso, presets 5.4.0 text).
        let mut versions = self.versions_of(self.latest);
        versions.sort_by_key(|v| v.id);
        for v in versions {
            let Some(p) = self.preset(v.preset_id) else {
                continue;
            };
            let sets = self.sets_of(v.id);
            let option = {
                let mut o = json!({
                    "preset_id": p.id, "preset_version_id": v.id, "size_amount": size_wire(p.size_amount),
                    "platform_specific": sets.iter().any(|s| s.firm_platform.is_some()),
                    "not_confirmed_count": not_confirmed_count(sets.iter().map(|s| &s.rules)),
                });
                if let Some(l) = &p.size_label {
                    o["size_label"] = json!(l);
                }
                o
            };
            let plan = match plans.iter_mut().position(|(id, _)| *id == p.plan_id) {
                Some(i) => &mut plans[i].1,
                None => {
                    plans.push((p.plan_id, Vec::new()));
                    let last = plans.len() - 1;
                    &mut plans[last].1
                }
            };
            match plan.iter_mut().find(|(phase, _)| *phase == p.phase) {
                Some((_, sizes)) => sizes.push((p.size_amount, option)),
                None => plan.push((p.phase.clone(), vec![(p.size_amount, option)])),
            }
        }
        for (_, phases) in &mut plans {
            for (_, sizes) in phases.iter_mut() {
                sizes.sort_by_key(|(amount, _)| *amount);
            }
        }
        let plans: Vec<Value> = plans
            .into_iter()
            .map(|(id, phases)| {
                json!({
                    "plan_id": id,
                    "name": self.plan(id).map_or("", |p| p.name.as_str()),
                    "phases": phases.into_iter().map(|(phase, sizes)| {
                        json!({"phase": phase, "sizes": sizes.into_iter().map(|(_, o)| o).collect::<Vec<_>>()})
                    }).collect::<Vec<_>>(),
                })
            })
            .collect();
        let excluded: Vec<Value> = self
            .latest
            .excluded_plans
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|e| e.get("name").cloned())
            .collect();
        let rule_versions = |kind: &str| -> Vec<Value> {
            texts
                .iter()
                .filter(|t| t.kind == kind)
                .map(|t| self.rule_version(t))
                .collect()
        };
        json!({
            "summary": self.summary(),
            "stances": {
                "automation": rule_versions("automation"),
                "copy": rule_versions("copy"),
                "vps": Self::stance_info(self.stance(self.latest, "vps")),
                "personal_device": Self::stance_info(self.stance(self.latest, "personal_device")),
            },
            "platforms": platforms,
            "plans": plans,
            "excluded_plans": excluded,
        })
    }

    /// Release `changes` with ids instead of the file's preset keys. `firm_platform` is left out
    /// until HTTP 5.4.0 adds it to `ReleaseChange`.
    fn changes(&self, r: &ReleaseRow) -> Vec<Value> {
        let plan_by_key: HashMap<&str, Uuid> = self
            .d
            .plans
            .iter()
            .map(|p| (p.key.as_str(), p.id))
            .collect();
        let preset_id = |key: &Value| -> Option<Uuid> {
            let plan = plan_by_key.get(key.get("plan")?.as_str()?)?;
            let phase = key.get("phase")?.as_str()?;
            let size = key.get("size")?.as_str()?;
            self.d
                .presets
                .iter()
                .find(|p| p.plan_id == *plan && p.phase == phase && p.size_key == size)
                .map(|p| p.id)
        };
        r.changes
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|c| {
                let mut out: Map<String, Value> = c.as_object()?.clone();
                out.remove("firm_platform");
                if let Some(key) = out.remove("preset") {
                    out.insert("preset_id".into(), json!(preset_id(&key)?));
                }
                Some(Value::Object(out))
            })
            .collect()
    }

    fn releases(&self) -> Value {
        let releases: Vec<Value> = self
            .d
            .releases
            .iter()
            .rev()
            .map(|r| {
                let rule_versions: Vec<Value> = self
                    .texts_at(r)
                    .into_iter()
                    .map(|t| {
                        let mut v = json!({"id": t.id, "kind": t.kind});
                        if let Some(p) = &t.firm_platform {
                            v["firm_platform"] = json!(p);
                        }
                        v
                    })
                    .collect();
                json!({
                    "version_number": r.version_number,
                    "published_at": ts(r.published_at),
                    "verified_on": r.verified_on.to_string(),
                    "changes": self.changes(r),
                    "rule_versions": rule_versions,
                    "vps": Self::stance_info(self.stance(r, "vps")),
                    "personal_device": Self::stance_info(self.stance(r, "personal_device")),
                })
            })
            .collect();
        json!({"firm_id": self.firm.id, "releases": releases})
    }

    fn superseded_by(&self, v: &PresetVersionRow) -> Option<Uuid> {
        let release = self.d.releases.iter().find(|r| r.id == v.release_id)?;
        self.d
            .versions
            .iter()
            .filter(|o| o.preset_id == v.preset_id && o.id != v.id)
            .filter_map(|o| {
                Some((
                    self.d
                        .releases
                        .iter()
                        .find(|r| r.id == o.release_id)?
                        .version_number,
                    o.id,
                ))
            })
            .filter(|(n, _)| *n > release.version_number)
            .max_by_key(|(n, _)| *n)
            .map(|(_, id)| id)
    }

    fn facts(&self, id: Uuid) -> Option<PresetVersionFacts> {
        let v = self.d.versions.iter().find(|v| v.id == id)?;
        let p = self.preset(v.preset_id)?;
        let plan = self.plan(p.plan_id)?;
        let release = self.d.releases.iter().find(|r| r.id == v.release_id)?;
        let sets = self.sets_of(v.id);
        let max_fields: Vec<&Value> = sets
            .iter()
            .filter_map(|s| s.rules.get("max_position_qty"))
            .collect();
        let verified: Vec<i64> = max_fields
            .iter()
            .filter(|f| f.get("status").and_then(Value::as_str) == Some("verified"))
            .filter_map(|f| f.get("value").and_then(contracts_of))
            .collect();
        let published: Vec<i64> = max_fields
            .iter()
            .flat_map(|f| {
                f.get("published_values")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
            })
            .filter_map(|pv| pv.get("value").and_then(contracts_of))
            .collect();
        // One value when every rule set verifies the same maximum.
        let max_position_qty = (verified.len() == max_fields.len() && !verified.is_empty())
            .then(|| verified.iter().copied().min())
            .flatten()
            .filter(|m| verified.iter().all(|x| x == m));
        Some(PresetVersionFacts {
            preset_version_id: v.id,
            preset_id: p.id,
            firm_id: self.firm.id,
            firm_name: self.firm.name.clone(),
            plan_name: plan.name.clone(),
            phase: p.phase.clone(),
            size_amount: p.size_amount,
            max_position_qty,
            max_position_cap: published
                .iter()
                .chain(&verified)
                .copied()
                .max()
                .filter(|_| max_position_qty.is_none()),
            firm_platforms: sets
                .iter()
                .filter_map(|s| s.firm_platform.clone())
                .collect(),
            superseded_by: self.superseded_by(v),
            has_evaluation_target: sets
                .iter()
                .any(|s| s.rules.get("evaluation_profit_target").is_some()),
            manual_only: v.manual_only.clone(),
            rule_sets: sets
                .iter()
                .map(|s| (s.firm_platform.clone(), without_source_quotes(&s.rules)))
                .collect(),
            release_version_number: release.version_number,
            release_published_at: release.published_at,
            release_verified_on: release.verified_on,
        })
    }

    fn preset_version(&self, id: Uuid) -> Option<Value> {
        let v = self.d.versions.iter().find(|v| v.id == id)?;
        let p = self.preset(v.preset_id)?;
        let plan = self.plan(p.plan_id)?;
        let release = self.d.releases.iter().find(|r| r.id == v.release_id)?;
        let sets = self.sets_of(v.id);
        let rule_sets: Vec<Value> = sets
            .iter()
            .map(|s| match &s.firm_platform {
                Some(fp) => json!({"firm_platform": fp, "rules": without_source_quotes(&s.rules)}),
                None => json!({"rules": without_source_quotes(&s.rules)}),
            })
            .collect();
        let superseded_by = self.superseded_by(v);
        let mut out = json!({
            "id": v.id, "preset_id": p.id, "firm_id": self.firm.id, "firm_name": self.firm.name,
            "plan_id": plan.id, "plan_name": plan.name, "phase": p.phase,
            "size_amount": size_wire(p.size_amount),
            "release": Self::release_ref(release),
            "rule_sets": rule_sets,
            "not_confirmed_count": not_confirmed_count(sets.iter().map(|s| &s.rules)),
        });
        if let Some(l) = &p.size_label {
            out["size_label"] = json!(l);
        }
        if let Some(m) = &v.manual_only {
            out["manual_only"] = m.clone();
        }
        if let Some(s) = superseded_by {
            out["superseded_by"] = json!(s);
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn source_quotes_are_import_evidence_only() {
        let rules = json!({"drawdown": {"status": "not_confirmed", "published_values": [
            {"value": {"amount": "1"}, "sources": [{"url": "https://a.example/x", "accessed_at": "2026-10-20", "quote": "q"}]},
            {"value": {"amount": "2"}, "sources": [{"url": "https://a.example/y", "accessed_at": "2026-10-20"}]}]}});
        let out = without_source_quotes(&rules);
        assert_eq!(
            out["drawdown"]["published_values"][0]["sources"][0],
            json!({"url": "https://a.example/x", "accessed_at": "2026-10-20"})
        );
        assert_eq!(
            out["drawdown"]["published_values"][0]["value"],
            json!({"amount": "1"})
        );
        let research = json!({"searched_on": "2026-10-20", "sources": [{"url": "https://a.example", "accessed_at": "2026-10-20", "quote": "q"}]});
        assert!(
            research_json(&research)["sources"][0]
                .get("quote")
                .is_none()
        );
    }
}
