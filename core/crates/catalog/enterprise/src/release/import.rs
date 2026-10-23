//! Importing one firm release (contracts/presets/v1/README.md, "Core import checks"): all or
//! nothing. The checks run against what is stored for the firm; the result is either "already
//! published" (the same file again) or a plan of rows to insert. Ids of new rows are assigned by
//! the store; existing identity rows (firm, plan, preset, rule text) keep theirs.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::str::FromStr;

use rust_decimal::Decimal;
use serde_json::{Map, Value, json};
use uuid::Uuid;

use super::file::{PresetKey, ReleaseFile, RuleSet, RuleText, StanceInfo};

/// Rule text kinds and their slot (`kind`, `firm_platform`).
pub type TextSlot = (String, Option<String>);

/// An opt-in rule text in force for the firm (the highest version of its slot).
#[derive(Debug, Clone, PartialEq)]
pub struct StoredRuleText {
    pub id: Uuid,
    pub version: i32,
    pub firm_stance: String,
    pub quotes: Value,
    pub research: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoredStance {
    pub value: String,
    pub quotes: Value,
    pub research: Option<Value>,
}

/// A preset identity row (stable across releases).
#[derive(Debug, Clone, PartialEq)]
pub struct StoredPreset {
    pub id: Uuid,
    pub size_amount: Decimal,
    pub size_label: Option<String>,
}

/// The latest release's content, for the diff.
#[derive(Debug, Clone, PartialEq)]
pub struct StoredRelease {
    pub version_number: i32,
    pub rule_texts: BTreeMap<TextSlot, StoredRuleText>,
    pub vps: StoredStance,
    pub personal_device: StoredStance,
    /// Presets offered by the latest release with their rule sets (`firm_platform` → rules).
    pub presets: BTreeMap<PresetKey, BTreeMap<Option<String>, Map<String, Value>>>,
}

/// What is stored for the firm with this slug.
#[derive(Debug, Clone, PartialEq)]
pub struct StoredFirm {
    pub id: Uuid,
    pub name: String,
    /// Every published release's file hash, by version number.
    pub release_hashes: HashMap<i32, [u8; 32]>,
    pub latest: StoredRelease,
    /// Plan key → (id, name), every plan ever published.
    pub plans: HashMap<String, (Uuid, String)>,
    /// Preset key → identity row, every preset ever published.
    pub presets: HashMap<PresetKey, StoredPreset>,
}

/// Switches tied to other deliveries (Esquadro's merge conditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportPolicy {
    /// `personal_device: not_confirmed` only once HTTP 5.4.0 can return it.
    pub accept_personal_device_not_confirmed: bool,
}

/// Why a file is refused. Every problem found is reported (the import never stops at the first).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ImportProblem {
    #[error("schema_version {0} is newer than this Core reads (1.0)")]
    UnsupportedSchemaVersion(String),
    #[error("version_number {got}: expected {expected}")]
    VersionNotNext { expected: i32, got: i32 },
    #[error("version_number {0} is already published with other content")]
    SameVersionOtherContent(i32),
    #[error("duplicate {0}")]
    Duplicate(String),
    #[error("firm_platform {0} is not in platforms")]
    UnknownPlatform(String),
    #[error("{0}: one entry without firm_platform, or exactly one per platform")]
    PlatformSplit(String),
    #[error("{0}: the stance platform split differs from the stored release (not supported)")]
    StanceSplitChanged(String),
    #[error("url host {0} is not on the firm's domains")]
    ForeignDomain(String),
    #[error("{0}: evaluation_profit_target outside an evaluation phase")]
    EvaluationTargetOutsideEvaluation(String),
    #[error("{slot}: rule text version {got}, expected {expected}")]
    RuleTextVersion {
        slot: String,
        expected: String,
        got: i32,
    },
    #[error("{0}: size key reused with another size_amount")]
    SizeKeyAmountChanged(String),
    #[error("{0}: no verified rule field")]
    NoVerifiedField(String),
    #[error("no preset is offered")]
    NoPreset,
    #[error("text with a token-shaped part or a control character at {0}")]
    UnsafeText(String),
    #[error("personal_device not_confirmed needs HTTP 5.4.0")]
    PersonalDeviceNotConfirmed,
    #[error("invalid {0}")]
    Invalid(String),
    #[error("changes differ from the Core's diff: missing {missing:?}, unexpected {unexpected:?}")]
    ChangesMismatch {
        missing: Vec<String>,
        unexpected: Vec<String>,
    },
}

/// A rule text to insert (its slot's next version, or the first).
#[derive(Debug, Clone, PartialEq)]
pub struct NewRuleText {
    pub kind: String,
    pub firm_platform: Option<String>,
    pub version: i32,
    pub firm_stance: String,
    pub quotes: Value,
    pub research: Option<Value>,
    pub previous_id: Option<Uuid>,
    pub change_cause: Option<String>,
    pub change_reason: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanWrite {
    pub existing_id: Option<Uuid>,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PresetWrite {
    pub existing_id: Option<Uuid>,
    pub plan_key: String,
    pub phase: String,
    pub size_key: String,
    pub size_amount: Decimal,
    pub size_label: Option<String>,
    /// `(firm_platform, rules, stored_only)`.
    pub rule_sets: Vec<(Option<String>, Value, Option<Value>)>,
}

/// Everything one publication inserts (and renames), in one transaction.
#[derive(Debug, Clone, PartialEq)]
pub struct ReleasePlan {
    pub firm_id: Option<Uuid>,
    pub slug: String,
    pub firm_name: String,
    pub version_number: i32,
    pub verified_on: String,
    pub research_file: String,
    pub source_sha256: [u8; 32],
    pub changes: Value,
    pub manual_only: Option<Value>,
    pub excluded_plans: Value,
    /// `(code, label, manual_only)`.
    pub platforms: Vec<(String, String, Option<Value>)>,
    pub vps: StoredStance,
    pub personal_device: StoredStance,
    pub new_rule_texts: Vec<NewRuleText>,
    pub plans: Vec<PlanWrite>,
    pub presets: Vec<PresetWrite>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportOutcome {
    /// The same file was already published for this version: nothing to do.
    AlreadyPublished {
        version_number: i32,
    },
    Publish(Box<ReleasePlan>),
}

/// Runs the checks and builds the plan. `sha256` is the hash of the file's canonical JSON.
pub fn plan_import(
    file: &ReleaseFile,
    raw: &Value,
    sha256: [u8; 32],
    stored: Option<&StoredFirm>,
    policy: ImportPolicy,
) -> Result<ImportOutcome, Vec<ImportProblem>> {
    let version = file.release.version_number;
    if let Some(firm) = stored
        && let Some(hash) = firm.release_hashes.get(&version)
    {
        // Idempotent publishing: the same file is a no-op, other content is an error.
        return if *hash == sha256 {
            Ok(ImportOutcome::AlreadyPublished {
                version_number: version,
            })
        } else {
            Err(vec![ImportProblem::SameVersionOtherContent(version)])
        };
    }
    let mut p = Vec::new();
    // A Core that reads 1.0 refuses a newer minor (README, "Versioning").
    if file.schema_version != "1.0" {
        p.push(ImportProblem::UnsupportedSchemaVersion(
            file.schema_version.clone(),
        ));
    }
    // 1–2. A new slug is a new firm at version 1; otherwise exactly the stored one + 1.
    let expected = stored.map_or(1, |f| f.latest.version_number.saturating_add(1));
    if version != expected {
        p.push(ImportProblem::VersionNotNext {
            expected,
            got: version,
        });
    }
    check_unique(file, &mut p);
    check_platforms(file, &mut p);
    check_domains(file, raw, &mut p);
    check_texts(raw, "", &mut p);
    if file.stances.personal_device.value == "not_confirmed"
        && !policy.accept_personal_device_not_confirmed
    {
        p.push(ImportProblem::PersonalDeviceNotConfirmed);
    }
    let presets = presets(file, stored, &mut p);
    let new_rule_texts = rule_texts(file, stored, &mut p);
    if let Some(firm) = stored {
        let computed = diff(file, &firm.latest);
        compare_changes(&file.release.changes, &computed, &mut p);
    } else if !file.release.changes.is_empty() {
        compare_changes(&file.release.changes, &[], &mut p);
    }
    if !p.is_empty() {
        return Err(p);
    }
    let plans = file
        .plans
        .iter()
        .map(|plan| PlanWrite {
            existing_id: stored
                .and_then(|f| f.plans.get(&plan.key))
                .map(|(id, _)| *id),
            key: plan.key.clone(),
            name: plan.name.clone(),
        })
        .collect();
    Ok(ImportOutcome::Publish(Box::new(ReleasePlan {
        firm_id: stored.map(|f| f.id),
        slug: file.firm.slug.clone(),
        firm_name: file.firm.name.clone(),
        version_number: version,
        verified_on: file.release.verified_on.clone(),
        research_file: file.release.research_file.clone(),
        source_sha256: sha256,
        changes: Value::Array(file.release.changes.clone()),
        manual_only: file.firm.manual_only.clone(),
        excluded_plans: Value::Array(file.excluded_plans.clone()),
        platforms: file
            .platforms
            .iter()
            .map(|pl| (pl.code.clone(), pl.label.clone(), pl.manual_only.clone()))
            .collect(),
        vps: stance(&file.stances.vps),
        personal_device: stance(&file.stances.personal_device),
        new_rule_texts,
        plans,
        presets,
    })))
}

fn stance(s: &StanceInfo) -> StoredStance {
    StoredStance {
        value: s.value.clone(),
        quotes: Value::Array(s.quotes.clone()),
        research: s.research.clone(),
    }
}

fn key_text(k: &PresetKey) -> String {
    format!("{}/{}/{}", k.plan, k.phase, k.size)
}

fn slot_text((kind, platform): &TextSlot) -> String {
    match platform {
        Some(p) => format!("{kind}@{p}"),
        None => kind.clone(),
    }
}

/// Check 3: Plan keys; phases within a plan; size keys and size amounts within a phase.
fn check_unique(file: &ReleaseFile, p: &mut Vec<ImportProblem>) {
    let mut plans = BTreeSet::new();
    for plan in &file.plans {
        if !plans.insert(&plan.key) {
            p.push(ImportProblem::Duplicate(format!("plan key {}", plan.key)));
        }
        let mut phases = BTreeSet::new();
        for phase in &plan.phases {
            if !phases.insert(&phase.phase) {
                p.push(ImportProblem::Duplicate(format!(
                    "phase {}/{}",
                    plan.key, phase.phase
                )));
            }
            let mut keys = BTreeSet::new();
            let mut amounts = BTreeSet::new();
            for size in &phase.sizes {
                if !keys.insert(&size.key) {
                    p.push(ImportProblem::Duplicate(format!(
                        "size key {}/{}/{}",
                        plan.key, phase.phase, size.key
                    )));
                }
                match Decimal::from_str(&size.size_amount) {
                    Ok(a) if !amounts.insert(a.normalize()) => {
                        p.push(ImportProblem::Duplicate(format!(
                            "size_amount {}/{}/{}",
                            plan.key, phase.phase, size.size_amount
                        )))
                    }
                    Ok(_) => {}
                    Err(_) => p.push(ImportProblem::Invalid(format!(
                        "size_amount {}",
                        size.size_amount
                    ))),
                }
            }
        }
    }
    let mut codes = BTreeSet::new();
    for pl in &file.platforms {
        if !codes.insert(&pl.code) {
            p.push(ImportProblem::Duplicate(format!("platform {}", pl.code)));
        }
    }
}

/// Check 4–5: Platforms used exist; one entry without a platform, or exactly one per platform.
fn check_platforms(file: &ReleaseFile, p: &mut Vec<ImportProblem>) {
    let codes: BTreeSet<&str> = file.platforms.iter().map(|pl| pl.code.as_str()).collect();
    let split = |what: String, used: Vec<Option<&str>>, p: &mut Vec<ImportProblem>| {
        for code in used.iter().flatten() {
            if !codes.contains(code) {
                p.push(ImportProblem::UnknownPlatform((*code).to_owned()));
            }
        }
        let whole = used.len() == 1 && used[0].is_none();
        let set: BTreeSet<&str> = used.iter().flatten().copied().collect();
        let per_platform =
            used.iter().all(Option::is_some) && set.len() == used.len() && set == codes;
        if !(whole || per_platform) {
            p.push(ImportProblem::PlatformSplit(what));
        }
    };
    for (name, list) in [
        ("stances.automation", &file.stances.automation),
        ("stances.copy", &file.stances.copy),
    ] {
        split(
            name.to_owned(),
            list.iter().map(|t| t.firm_platform.as_deref()).collect(),
            p,
        );
    }
    for (key, _, size) in file.presets() {
        split(
            format!("rule_sets of {}", key_text(&key)),
            size.rule_sets
                .iter()
                .map(|r| r.firm_platform.as_deref())
                .collect(),
            p,
        );
    }
}

/// Check 6: Every `url` in the file is on one of the firm's domains or their subdomains.
fn check_domains(file: &ReleaseFile, raw: &Value, p: &mut Vec<ImportProblem>) {
    let domains: Vec<String> = file
        .firm
        .domains
        .iter()
        .map(|d| d.to_ascii_lowercase())
        .collect();
    let mut urls = Vec::new();
    collect_urls(raw, &mut urls);
    for url in urls {
        let host = url
            .strip_prefix("https://")
            .unwrap_or(&url)
            .split(['/', '?', '#', ':'])
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        let ok = domains.iter().any(|d| {
            host == *d
                || host
                    .strip_suffix(d.as_str())
                    .is_some_and(|rest| rest.ends_with('.'))
        });
        if !ok {
            p.push(ImportProblem::ForeignDomain(host));
        }
    }
}

fn collect_urls(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::Object(m) => {
            for (k, child) in m {
                match (k.as_str(), child) {
                    ("url", Value::String(s)) => out.push(s.clone()),
                    _ => collect_urls(child, out),
                }
            }
        }
        Value::Array(items) => items.iter().for_each(|i| collect_urls(i, out)),
        _ => {}
    }
}

/// Check 10: No string anywhere (names, labels, quotes, published texts, descriptions) holds a
/// token-shaped part or a control character other than newline and tab.
fn check_texts(v: &Value, path: &str, p: &mut Vec<ImportProblem>) {
    match v {
        Value::String(s) => {
            if has_token_shape(s) || s.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
                p.push(ImportProblem::UnsafeText(path.to_owned()));
            }
        }
        Value::Object(m) => {
            for (k, child) in m {
                check_texts(child, &format!("{path}/{k}"), p);
            }
        }
        Value::Array(items) => {
            for (i, child) in items.iter().enumerate() {
                check_texts(child, &format!("{path}/{i}"), p);
            }
        }
        _ => {}
    }
}

/// `aw[kd]_` followed by 12 alphanumerics, `_` and 43 base64url characters (S1 token shape).
fn has_token_shape(s: &str) -> bool {
    let b = s.as_bytes();
    (0..b.len()).any(|i| {
        b.get(i..i + 4)
            .is_some_and(|w| w == b"awk_" || w == b"awd_")
            && b.get(i + 4..i + 16)
                .is_some_and(|w| w.iter().all(u8::is_ascii_alphanumeric))
            && b.get(i + 16) == Some(&b'_')
            && b.get(i + 17..i + 60).is_some_and(|w| {
                w.iter()
                    .all(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c == b'-')
            })
    })
}

/// Presets to write, with checks 7, 11 and the size-key identity.
fn presets(
    file: &ReleaseFile,
    stored: Option<&StoredFirm>,
    p: &mut Vec<ImportProblem>,
) -> Vec<PresetWrite> {
    let mut out = Vec::new();
    for (key, _, size) in file.presets() {
        let amount = Decimal::from_str(&size.size_amount)
            .unwrap_or_default()
            .normalize();
        for rs in &size.rule_sets {
            if key.phase != "evaluation" && rs.rules.contains_key("evaluation_profit_target") {
                p.push(ImportProblem::EvaluationTargetOutsideEvaluation(key_text(
                    &key,
                )));
            }
            if !rs
                .rules
                .values()
                .any(|f| f.get("status").and_then(Value::as_str) == Some("verified"))
            {
                p.push(ImportProblem::NoVerifiedField(key_text(&key)));
            }
        }
        let existing = stored.and_then(|f| f.presets.get(&key));
        if let Some(e) = existing
            && e.size_amount.normalize() != amount
        {
            p.push(ImportProblem::SizeKeyAmountChanged(key_text(&key)));
        }
        out.push(PresetWrite {
            existing_id: existing.map(|e| e.id),
            plan_key: key.plan.clone(),
            phase: key.phase.clone(),
            size_key: key.size.clone(),
            size_amount: amount,
            size_label: size.size_label.clone(),
            rule_sets: size
                .rule_sets
                .iter()
                .map(|r| {
                    (
                        r.firm_platform.clone(),
                        Value::Object(r.rules.clone()),
                        r.stored_only.clone(),
                    )
                })
                .collect(),
        });
    }
    if out.is_empty() {
        p.push(ImportProblem::NoPreset);
    }
    out
}

fn same_text(t: &RuleText, s: &StoredRuleText) -> bool {
    t.firm_stance == s.firm_stance
        && Value::Array(t.quotes.clone()) == s.quotes
        && t.research == s.research
}

/// Check 8: Each slot's version is the stored one when unchanged, the stored one + 1 when the stance or
/// text changed, 1 for a new firm. Only new versions are written.
fn rule_texts(
    file: &ReleaseFile,
    stored: Option<&StoredFirm>,
    p: &mut Vec<ImportProblem>,
) -> Vec<NewRuleText> {
    let mut out = Vec::new();
    if let Some(firm) = stored {
        let file_slots: BTreeSet<TextSlot> = file
            .rule_texts()
            .map(|t| (t.kind.clone(), t.firm_platform.clone()))
            .collect();
        let stored_slots: BTreeSet<TextSlot> = firm.latest.rule_texts.keys().cloned().collect();
        if file_slots != stored_slots {
            p.push(ImportProblem::StanceSplitChanged(
                "stances.automation/copy".into(),
            ));
            return out;
        }
    }
    for t in file.rule_texts() {
        let slot = (t.kind.clone(), t.firm_platform.clone());
        let current = stored.and_then(|f| f.latest.rule_texts.get(&slot));
        let (expected, is_new) = match current {
            None => (1, true),
            Some(s) if same_text(t, s) => (s.version, false),
            Some(s) => (s.version.saturating_add(1), true),
        };
        if t.version != expected {
            p.push(ImportProblem::RuleTextVersion {
                slot: slot_text(&slot),
                expected: expected.to_string(),
                got: t.version,
            });
            continue;
        }
        if is_new {
            out.push(NewRuleText {
                kind: t.kind.clone(),
                firm_platform: t.firm_platform.clone(),
                version: t.version,
                firm_stance: t.firm_stance.clone(),
                quotes: Value::Array(t.quotes.clone()),
                research: t.research.clone(),
                previous_id: current.map(|s| s.id),
                change_cause: current.and(t.change.as_ref()).map(|c| c.cause.clone()),
                change_reason: current
                    .and(t.change.as_ref())
                    .and_then(|c| c.reason.clone()),
            });
        }
    }
    out
}

/// A rule field compared without its sources: a new access date or page is not a value change.
fn without_sources(v: &Value) -> Value {
    match v {
        Value::Object(m) => Value::Object(
            m.iter()
                .filter(|(k, _)| k.as_str() != "sources")
                .map(|(k, child)| (k.clone(), without_sources(child)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.iter().map(without_sources).collect()),
        other => other.clone(),
    }
}

fn platform_field(m: &mut Map<String, Value>, platform: &Option<String>) {
    if let Some(p) = platform {
        m.insert("firm_platform".into(), json!(p));
    }
}

/// Check 9: The Core's own `changes` against the latest stored release.
pub fn diff(file: &ReleaseFile, last: &StoredRelease) -> Vec<Value> {
    let mut out = Vec::new();
    for t in file.rule_texts() {
        let Some(s) = last
            .rule_texts
            .get(&(t.kind.clone(), t.firm_platform.clone()))
        else {
            continue;
        };
        let mut c = Map::new();
        c.insert("stance".into(), json!(t.kind));
        platform_field(&mut c, &t.firm_platform);
        if t.firm_stance != s.firm_stance {
            c.insert("kind".into(), json!("stance_changed"));
            c.insert("from".into(), json!(s.firm_stance));
            c.insert("to".into(), json!(t.firm_stance));
        } else if !same_text(t, s) {
            let material = t
                .change
                .as_ref()
                .is_some_and(|ch| ch.cause == "text_changed");
            c.insert("kind".into(), json!("stance_text_changed"));
            c.insert("material".into(), json!(material));
            if let Some(reason) = t
                .change
                .as_ref()
                .and_then(|ch| ch.reason.clone())
                .filter(|_| material)
            {
                c.insert("reason".into(), reason);
            }
        } else {
            continue;
        }
        out.push(Value::Object(c));
    }
    for (name, now, before) in [
        ("vps", &file.stances.vps, &last.vps),
        (
            "personal_device",
            &file.stances.personal_device,
            &last.personal_device,
        ),
    ] {
        if now.value != before.value {
            out.push(json!({"kind": "stance_changed", "stance": name, "from": before.value, "to": now.value}));
        } else if Value::Array(now.quotes.clone()) != before.quotes
            || now.research != before.research
        {
            out.push(json!({"kind": "stance_text_changed", "stance": name, "material": false}));
        }
    }
    let now: BTreeMap<PresetKey, &Vec<RuleSet>> =
        file.presets().map(|(k, _, s)| (k, &s.rule_sets)).collect();
    for key in last.presets.keys().filter(|k| !now.contains_key(*k)) {
        out.push(json!({"kind": "preset_removed", "preset": key.to_json()}));
    }
    for (key, sets) in &now {
        let Some(before) = last.presets.get(key) else {
            out.push(json!({"kind": "preset_added", "preset": key.to_json()}));
            continue;
        };
        for rs in sets.iter() {
            let old = before.get(&rs.firm_platform);
            let fields: BTreeSet<&String> = rs
                .rules
                .keys()
                .chain(old.into_iter().flat_map(|o| o.keys()))
                .collect();
            for field in fields {
                let a = rs.rules.get(field).map(without_sources);
                let b = old.and_then(|o| o.get(field)).map(without_sources);
                if a != b {
                    let mut c = Map::new();
                    c.insert("kind".into(), json!("rule_value_changed"));
                    c.insert("field".into(), json!(field));
                    c.insert("preset".into(), key.to_json());
                    platform_field(&mut c, &rs.firm_platform);
                    out.push(Value::Object(c));
                }
            }
        }
    }
    out
}

/// Order-insensitive comparison of the file's `changes` with the computed ones.
fn compare_changes(declared: &[Value], computed: &[Value], p: &mut Vec<ImportProblem>) {
    let canon = |v: &Value| String::from_utf8(super::canonical_json(v)).unwrap_or_default();
    let mut left: Vec<String> = declared.iter().map(canon).collect();
    let mut unexpected = Vec::new();
    for c in computed.iter().map(canon) {
        match left.iter().position(|d| *d == c) {
            Some(i) => {
                left.swap_remove(i);
            }
            None => unexpected.push(c),
        }
    }
    // `left`: declared but not computed; `unexpected`: computed but not declared (missing in the file).
    if !left.is_empty() || !unexpected.is_empty() {
        p.push(ImportProblem::ChangesMismatch {
            missing: unexpected,
            unexpected: left,
        });
    }
}
