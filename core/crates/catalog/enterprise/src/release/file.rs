//! The preset release file (contracts/presets/v1/release.schema.json), as the Core reads it after
//! the schema has accepted it. Rule fields stay JSON documents: they are published texts with
//! provenance, shown and diffed, never computed (data model §5.7).

use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ReleaseFile {
    pub schema_version: String,
    pub firm: Firm,
    pub release: Release,
    pub platforms: Vec<Platform>,
    pub stances: Stances,
    #[serde(default)]
    pub plans: Vec<Plan>,
    #[serde(default)]
    pub excluded_plans: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Firm {
    pub slug: String,
    pub name: String,
    pub domains: Vec<String>,
    #[serde(default)]
    pub manual_only: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Release {
    pub version_number: i32,
    pub verified_on: String,
    pub research_file: String,
    pub changes: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Platform {
    pub code: String,
    pub label: String,
    #[serde(default)]
    pub manual_only: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Stances {
    pub automation: Vec<RuleText>,
    pub copy: Vec<RuleText>,
    pub vps: StanceInfo,
    pub personal_device: StanceInfo,
}

/// An opt-in rule text (automation or copy), for the firm or for one firm platform.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RuleText {
    pub kind: String,
    #[serde(default)]
    pub firm_platform: Option<String>,
    pub version: i32,
    pub firm_stance: String,
    pub quotes: Vec<Value>,
    #[serde(default)]
    pub research: Option<Value>,
    /// The transition into this version (version > 1).
    #[serde(default)]
    pub change: Option<RuleChange>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RuleChange {
    pub cause: String,
    #[serde(default)]
    pub reason: Option<Value>,
}

/// VPS / personal-device stance.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StanceInfo {
    pub value: String,
    pub quotes: Vec<Value>,
    #[serde(default)]
    pub research: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Plan {
    pub key: String,
    pub name: String,
    pub phases: Vec<Phase>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Phase {
    pub phase: String,
    pub sizes: Vec<Size>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Size {
    pub key: String,
    #[serde(default)]
    pub size_label: Option<String>,
    pub size_amount: String,
    pub rule_sets: Vec<RuleSet>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RuleSet {
    #[serde(default)]
    pub firm_platform: Option<String>,
    /// `PresetRules`: field name → `{status, value | published_values, sources}`.
    pub rules: Map<String, Value>,
    #[serde(default)]
    pub stored_only: Option<Value>,
}

/// A preset's stable identity in release files: plan key × phase × size key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresetKey {
    pub plan: String,
    pub phase: String,
    pub size: String,
}

impl PresetKey {
    pub fn to_json(&self) -> Value {
        serde_json::json!({"plan": self.plan, "phase": self.phase, "size": self.size})
    }
}

impl ReleaseFile {
    /// Every preset in the file with its key.
    pub fn presets(&self) -> impl Iterator<Item = (PresetKey, &Plan, &Size)> {
        self.plans.iter().flat_map(|plan| {
            plan.phases.iter().flat_map(move |phase| {
                phase.sizes.iter().map(move |size| {
                    (
                        PresetKey {
                            plan: plan.key.clone(),
                            phase: phase.phase.clone(),
                            size: size.key.clone(),
                        },
                        plan,
                        size,
                    )
                })
            })
        })
    }

    /// Both opt-in rule text lists.
    pub fn rule_texts(&self) -> impl Iterator<Item = &RuleText> {
        self.stances.automation.iter().chain(&self.stances.copy)
    }
}
