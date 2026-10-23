//! Import checks against the contract's release examples (contracts/presets/v1/examples), plus
//! targeted edits of them for each Core check.

use std::collections::{BTreeMap, HashMap};

use serde_json::{Value, json};
use uuid::Uuid;

use super::canonical_json;
use super::file::ReleaseFile;
use super::import::*;

macro_rules! example {
    ($name:literal) => {
        include_str!(concat!(
            "../../../../../../contracts/presets/v1/examples/",
            $name
        ))
    };
}

const V1: &str = example!("valid/release-v1-new-firm.json");
const V2: &str = example!("valid/release-v2-stance-changed.json");
const VALID: [&str; 5] = [
    V1,
    V2,
    example!("valid/release-manual-only.json"),
    example!("valid/release-not-confirmed-per-platform.json"),
    example!("valid/release-stored-only-platform-manual-device-not-confirmed.json"),
];

const POLICY: ImportPolicy = ImportPolicy {
    accept_personal_device_not_confirmed: false,
    accept_not_material: false,
};

fn sha(v: &Value) -> [u8; 32] {
    // Tests only need distinct, stable keys: the canonical bytes folded into 32 bytes.
    let mut out = [0u8; 32];
    for (i, b) in canonical_json(v).iter().enumerate() {
        out[i % 32] = out[i % 32].wrapping_mul(31).wrapping_add(*b);
    }
    out
}

fn run(
    raw: &Value,
    stored: Option<&StoredFirm>,
    policy: ImportPolicy,
) -> Result<ImportOutcome, Vec<ImportProblem>> {
    let file: ReleaseFile = serde_json::from_value(raw.clone()).unwrap();
    plan_import(&file, raw, sha(raw), stored, policy)
}

fn plan(raw: &Value, stored: Option<&StoredFirm>) -> ReleasePlan {
    match run(raw, stored, POLICY) {
        Ok(ImportOutcome::Publish(p)) => *p,
        other => panic!("expected a plan: {other:?}"),
    }
}

fn problems(raw: &Value, stored: Option<&StoredFirm>) -> Vec<ImportProblem> {
    run(raw, stored, POLICY).expect_err("expected problems")
}

/// What the store would hold after publishing `plan` on top of `before`.
fn stored_after(plan: &ReleasePlan, raw: &Value, before: Option<StoredFirm>) -> StoredFirm {
    let file: ReleaseFile = serde_json::from_value(raw.clone()).unwrap();
    let mut firm = before.unwrap_or_else(|| StoredFirm {
        id: Uuid::now_v7(),
        name: plan.firm_name.clone(),
        release_hashes: HashMap::new(),
        latest: StoredRelease {
            version_number: 0,
            rule_texts: BTreeMap::new(),
            vps: plan.vps.clone(),
            personal_device: plan.personal_device.clone(),
            presets: BTreeMap::new(),
        },
        plans: HashMap::new(),
        presets: HashMap::new(),
    });
    firm.release_hashes
        .insert(plan.version_number, plan.source_sha256);
    firm.latest.version_number = plan.version_number;
    firm.latest.vps = plan.vps.clone();
    firm.latest.personal_device = plan.personal_device.clone();
    for t in &plan.new_rule_texts {
        firm.latest.rule_texts.insert(
            (t.kind.clone(), t.firm_platform.clone()),
            StoredRuleText {
                id: Uuid::now_v7(),
                version: t.version,
                firm_stance: t.firm_stance.clone(),
                quotes: t.quotes.clone(),
                research: t.research.clone(),
            },
        );
    }
    for p in &plan.plans {
        firm.plans
            .entry(p.key.clone())
            .or_insert((Uuid::now_v7(), p.name.clone()));
    }
    firm.latest.presets.clear();
    for (key, _, size) in file.presets() {
        let w = plan
            .presets
            .iter()
            .find(|w| w.size_key == key.size && w.plan_key == key.plan && w.phase == key.phase)
            .unwrap();
        firm.presets.entry(key.clone()).or_insert(StoredPreset {
            id: Uuid::now_v7(),
            size_amount: w.size_amount,
            size_label: w.size_label.clone(),
        });
        firm.latest.presets.insert(
            key,
            size.rule_sets
                .iter()
                .map(|r| (r.firm_platform.clone(), r.rules.clone()))
                .collect(),
        );
    }
    firm
}

fn parse(s: &str) -> Value {
    serde_json::from_str(s).unwrap()
}

/// The contract's v2 example declares a drawdown change but repeats v1's value (its fixture only
/// passes the schema); make the declared change real so the Core's diff (check 9) agrees.
fn v2() -> Value {
    let mut v2 = parse(V2);
    v2["plans"][0]["phases"][0]["sizes"][0]["rule_sets"][0]["rules"]["drawdown"]["value"]["amount"] =
        json!("2500.00");
    v2
}

#[test]
fn every_valid_example_imports_as_a_new_firm_or_after_v1() {
    let policy = ImportPolicy {
        accept_personal_device_not_confirmed: true,
        accept_not_material: false,
    };
    for raw in VALID.iter().map(|s| parse(s)) {
        if raw["release"]["version_number"] == 1 {
            assert!(
                matches!(run(&raw, None, policy), Ok(ImportOutcome::Publish(_))),
                "{}",
                raw["firm"]["slug"]
            );
        }
    }
}

#[test]
fn v1_plan_writes_every_text_and_preset() {
    let raw = parse(V1);
    let p = plan(&raw, None);
    assert_eq!((p.firm_id, p.version_number), (None, 1));
    assert!(
        p.new_rule_texts
            .iter()
            .all(|t| t.version == 1 && t.previous_id.is_none() && t.change_cause.is_none())
    );
    assert_eq!(p.new_rule_texts.len(), 2);
    assert!(!p.presets.is_empty() && p.presets.iter().all(|w| w.existing_id.is_none()));
}

#[test]
fn v2_after_v1_writes_only_the_changed_text_and_keeps_ids() {
    let v1 = parse(V1);
    let stored = stored_after(&plan(&v1, None), &v1, None);
    let v2 = v2();
    let p = plan(&v2, Some(&stored));
    assert_eq!(p.firm_id, Some(stored.id));
    // Only automation changed: one new text with its predecessor and cause.
    assert_eq!(p.new_rule_texts.len(), 1);
    let t = &p.new_rule_texts[0];
    assert_eq!(
        (t.kind.as_str(), t.version, t.change_cause.as_deref()),
        ("automation", 2, Some("stance_changed"))
    );
    assert_eq!(
        t.previous_id,
        Some(stored.latest.rule_texts[&("automation".into(), None)].id)
    );
    // Existing presets keep their ids.
    assert!(p.presets.iter().all(|w| w.existing_id.is_some()));
}

#[test]
fn the_same_file_again_is_a_no_op_and_other_content_is_refused() {
    let v1 = parse(V1);
    let stored = stored_after(&plan(&v1, None), &v1, None);
    assert_eq!(
        run(&v1, Some(&stored), POLICY),
        Ok(ImportOutcome::AlreadyPublished { version_number: 1 })
    );
    let mut other = v1.clone();
    other["firm"]["name"] = json!("Renamed");
    assert_eq!(
        problems(&other, Some(&stored)),
        [ImportProblem::SameVersionOtherContent(1)]
    );
}

#[test]
fn version_must_be_the_next_one() {
    let mut v2 = v2();
    assert!(
        problems(&v2, None).contains(&ImportProblem::VersionNotNext {
            expected: 1,
            got: 2
        })
    );
    let v1 = parse(V1);
    let stored = stored_after(&plan(&v1, None), &v1, None);
    v2["release"]["version_number"] = json!(3);
    assert!(
        problems(&v2, Some(&stored)).contains(&ImportProblem::VersionNotNext {
            expected: 2,
            got: 3
        })
    );
}

#[test]
fn changes_must_equal_the_cores_diff() {
    let v1 = parse(V1);
    let stored = stored_after(&plan(&v1, None), &v1, None);
    let mut v2 = v2();
    // Drop the declared drawdown change: the Core still finds it.
    v2["release"]["changes"]
        .as_array_mut()
        .unwrap()
        .retain(|c| c["kind"] != "rule_value_changed");
    let p = problems(&v2, Some(&stored));
    assert!(
        matches!(&p[..], [ImportProblem::ChangesMismatch { missing, unexpected }] if missing.len() == 1 && unexpected.is_empty()),
        "{p:?}"
    );
    // A v1 cannot declare changes.
    let mut bad = v1.clone();
    bad["release"]["changes"] = json!([{"kind": "preset_added", "preset": {"plan": "standard", "phase": "evaluation", "size": "50k"}}]);
    assert!(matches!(
        problems(&bad, None)[..],
        [ImportProblem::ChangesMismatch { .. }]
    ));
}

#[test]
fn rule_text_versions_follow_the_stored_ones() {
    let v1 = parse(V1);
    let stored = stored_after(&plan(&v1, None), &v1, None);
    // Changed text with the same version.
    let mut v2 = v2();
    v2["stances"]["automation"][0]["version"] = json!(1);
    v2["stances"]["automation"][0]
        .as_object_mut()
        .unwrap()
        .remove("change");
    assert!(
        problems(&v2, Some(&stored))
            .iter()
            .any(|p| matches!(p, ImportProblem::RuleTextVersion { got: 1, .. }))
    );
    // A new firm starts at 1.
    let mut first = v1.clone();
    first["stances"]["copy"][0]["version"] = json!(2);
    assert!(
        problems(&first, None)
            .iter()
            .any(|p| matches!(p, ImportProblem::RuleTextVersion { got: 2, .. }))
    );
}

#[test]
fn uniqueness_platforms_domains_and_texts() {
    let v1 = parse(V1);
    let mut f = v1.clone();
    let size = f["plans"][0]["phases"][0]["sizes"][0].clone();
    let mut twin = size.clone();
    twin["key"] = json!("other");
    f["plans"][0]["phases"][0]["sizes"]
        .as_array_mut()
        .unwrap()
        .push(twin);
    assert!(
        problems(&f, None)
            .iter()
            .any(|p| matches!(p, ImportProblem::Duplicate(d) if d.starts_with("size_amount")))
    );

    let mut f = v1.clone();
    f["plans"][0]["phases"][0]["sizes"][0]["rule_sets"][0]["firm_platform"] = json!("nope");
    let p = problems(&f, None);
    assert!(
        p.contains(&ImportProblem::UnknownPlatform("nope".into())),
        "{p:?}"
    );

    let mut f = v1.clone();
    f["stances"]["automation"][0]["quotes"][0]["url"] = json!("https://evil.example.org/x");
    assert!(problems(&f, None).contains(&ImportProblem::ForeignDomain("evil.example.org".into())));
    let mut f = v1.clone();
    f["stances"]["automation"][0]["quotes"][0]["url"] = json!("https://notexample-firm.com/x");
    assert!(
        problems(&f, None).contains(&ImportProblem::ForeignDomain("notexample-firm.com".into()))
    );

    let token = format!("awk_{}_{}", "A".repeat(12), "b".repeat(43));
    let mut f = v1.clone();
    f["plans"][0]["name"] = json!(format!("Plan {token}"));
    assert!(
        problems(&f, None)
            .iter()
            .any(|p| matches!(p, ImportProblem::UnsafeText(path) if path == "/plans/0/name"))
    );
    let mut f = v1.clone();
    f["firm"]["name"] = json!("Firm\u{0007}");
    assert!(
        problems(&f, None)
            .iter()
            .any(|p| matches!(p, ImportProblem::UnsafeText(_)))
    );
}

#[test]
fn verified_data_and_evaluation_target_rules() {
    let v1 = parse(V1);
    let mut f = v1.clone();
    let rules = f["plans"][0]["phases"][0]["sizes"][0]["rule_sets"][0]["rules"]
        .as_object_mut()
        .unwrap();
    for field in rules.values_mut() {
        if field["status"] == "verified" {
            let value = field["value"].clone();
            let sources = field["sources"].clone();
            *field = json!({"status": "not_confirmed", "published_values": [
                {"value": value, "sources": sources}, {"value": value, "sources": sources}]});
        }
    }
    assert!(
        problems(&f, None)
            .iter()
            .any(|p| matches!(p, ImportProblem::NoVerifiedField(_)))
    );

    let mut f = v1.clone();
    f["plans"] = json!([]);
    assert!(problems(&f, None).contains(&ImportProblem::NoPreset));
}

#[test]
fn personal_device_not_confirmed_waits_for_http_5_4() {
    let raw = parse(example!(
        "valid/release-stored-only-platform-manual-device-not-confirmed.json"
    ));
    assert!(problems(&raw, None).contains(&ImportProblem::PersonalDeviceNotConfirmed));
    let open = ImportPolicy {
        accept_personal_device_not_confirmed: true,
        accept_not_material: false,
    };
    assert!(run(&raw, None, open).is_ok());
}

#[test]
fn stance_split_changes_are_refused() {
    let v1 = parse(V1);
    let stored = stored_after(&plan(&v1, None), &v1, None);
    let mut v2 = v2();
    v2["stances"]["copy"][0]["firm_platform"] = json!(v2["platforms"][0]["code"]);
    let p = problems(&v2, Some(&stored));
    assert!(
        p.iter()
            .any(|p| matches!(p, ImportProblem::StanceSplitChanged(_))
                || matches!(p, ImportProblem::PlatformSplit(_))),
        "{p:?}"
    );
}

#[test]
fn canonical_json_sorts_keys_at_every_level() {
    let a = json!({"b": 1, "a": {"d": [1, {"z": 0, "y": "é"}], "c": null}});
    assert_eq!(
        String::from_utf8(canonical_json(&a)).unwrap(),
        r#"{"a":{"c":null,"d":[1,{"y":"é","z":0}]},"b":1}"#
    );
}

#[test]
fn not_material_waits_for_the_accounts_path() {
    let v1 = parse(V1);
    let stored = stored_after(&plan(&v1, None), &v1, None);
    // A typo fix in the copy text, declared not material.
    let mut v2 = parse(V1);
    v2["release"]["version_number"] = json!(2);
    let quote = &mut v2["stances"]["copy"][0];
    if quote["quotes"].as_array().is_some_and(|q| !q.is_empty()) {
        quote["quotes"][0]["text"] = json!("Copy trading is permitted (typo fixed).");
    } else {
        quote["research"]["searched_on"] = json!("2026-10-21");
    }
    quote["version"] = json!(2);
    quote["change"] = json!({"cause": "not_material"});
    v2["release"]["changes"] =
        json!([{"kind": "stance_text_changed", "stance": "copy", "material": false}]);
    let p = problems(&v2, Some(&stored));
    assert!(
        p.iter()
            .any(|x| matches!(x, ImportProblem::NotMaterialUnsupported(_))),
        "{p:?}"
    );
    let open = ImportPolicy {
        accept_personal_device_not_confirmed: false,
        accept_not_material: true,
    };
    assert!(
        matches!(run(&v2, Some(&stored), open), Ok(ImportOutcome::Publish(_))),
        "{:?}",
        run(&v2, Some(&stored), open)
    );
}
