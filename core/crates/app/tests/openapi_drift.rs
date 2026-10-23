//! Drift check: the Core's generated OpenAPI against `contracts/http/openapi.yaml`.
//!
//! Rules (Architect, 2026-10-23):
//! - The generated spec is an exact subset of the contract. Every generated operation
//!   exists in the contract and matches it. Contract operations without a handler are
//!   listed in `not_implemented`; a listed operation that gains a handler, or an
//!   unlisted one without a handler, fails.
//! - Per operation, after `$ref` resolution, compare: path and method, `operationId`,
//!   effective `security`, parameters (name, in, required, schema), request body and
//!   responses by status (content schemas and declared headers).
//! - Ignored everywhere: `description`, `title`, `summary`, `example(s)`, `x-*`
//!   (plus `$schema`/`$id`/`$defs`/`$comment` once references are resolved).
//!   Normalized: order of `required`/`enum`/`type`, numbers (`1 == 1.0`), and
//!   `const: X`, rewritten to the equivalent `enum: [X]` on both sides (when the same
//!   schema has no `enum`), so spelling differences need no allowlist entry.
//! - Differences `utoipa` cannot express are listed in `contracts/http/drift-allowlist.yaml`
//!   as `{operationId, pointer, reason}`; an entry matching no difference fails.
//! - The embedded instruction schema equals `contracts/instruction/v1/instruction.schema.json`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

const METHODS: [&str; 8] = [
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];
/// Keywords whose value is a map of user-chosen names (never stripped as keywords).
const NAME_MAPS: [&str; 7] = [
    "properties",
    "patternProperties",
    "dependentSchemas",
    "dependentRequired",
    "content",
    "headers",
    "responses",
];
const UNORDERED: [&str; 3] = ["required", "enum", "type"];
const MAX_DEPTH: usize = 64;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn contract_dir() -> PathBuf {
    repo_root().join("contracts/http")
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Allowlist {
    #[serde(default)]
    not_implemented: Vec<String>,
    #[serde(default)]
    entries: Vec<AllowEntry>,
}

#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct AllowEntry {
    operation_id: String,
    pointer: String,
    reason: String,
}

/// Reference resolution context: the document `#/...` points into and the directory
/// relative file references are resolved from.
#[derive(Clone)]
struct Ctx {
    doc: Value,
    dir: PathBuf,
}

fn load_file(path: &Path) -> Value {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    if path.extension().is_some_and(|e| e == "json") {
        serde_json::from_str(&text).unwrap()
    } else {
        yaml_serde::from_str(&text).unwrap()
    }
}

fn resolve(value: &Value, ctx: &Ctx, depth: usize) -> Value {
    assert!(depth < MAX_DEPTH, "reference cycle or nesting too deep");
    match value {
        Value::Object(map) => {
            // A schema resource with `$defs` is the base for its own `#/$defs/...` references.
            let ctx = if map.contains_key("$defs") {
                Ctx {
                    doc: value.clone(),
                    dir: ctx.dir.clone(),
                }
            } else {
                ctx.clone()
            };
            if let Some(Value::String(reference)) = map.get("$ref") {
                let (target, target_ctx) = follow(reference, &ctx);
                let mut out = match resolve(&target, &target_ctx, depth + 1) {
                    Value::Object(m) => m,
                    other => return other,
                };
                for (k, v) in map {
                    if k != "$ref" {
                        out.insert(k.clone(), resolve(v, &ctx, depth + 1));
                    }
                }
                return Value::Object(out);
            }
            Value::Object(
                map.iter()
                    .map(|(k, v)| (k.clone(), resolve(v, &ctx, depth + 1)))
                    .collect(),
            )
        }
        Value::Array(items) => {
            Value::Array(items.iter().map(|v| resolve(v, ctx, depth + 1)).collect())
        }
        other => other.clone(),
    }
}

fn follow(reference: &str, ctx: &Ctx) -> (Value, Ctx) {
    let (file, fragment) = reference.split_once('#').unwrap_or((reference, ""));
    let target_ctx = if file.is_empty() {
        ctx.clone()
    } else {
        let path = ctx.dir.join(file);
        Ctx {
            doc: load_file(&path),
            dir: path.parent().unwrap().to_path_buf(),
        }
    };
    let target = target_ctx
        .doc
        .pointer(fragment)
        .unwrap_or_else(|| panic!("unresolved $ref {reference}"))
        .clone();
    (target, target_ctx)
}

fn ignored(key: &str) -> bool {
    matches!(
        key,
        "description"
            | "title"
            | "summary"
            | "example"
            | "examples"
            | "$schema"
            | "$id"
            | "$defs"
            | "$comment"
    ) || key.starts_with("x-")
}

/// Strips ignored keywords and normalizes ordering and numbers.
fn canonical(value: &Value, in_name_map: bool) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = Map::new();
            // `const: X` ≡ `enum: [X]` (JSON Schema 2020-12 §6.1.2–6.1.3).
            let rewritten;
            let map = match (in_name_map, map.get("const"), map.contains_key("enum")) {
                (false, Some(value), false) => {
                    let mut m = map.clone();
                    m.remove("const");
                    m.insert("enum".into(), Value::Array(vec![value.clone()]));
                    rewritten = m;
                    &rewritten
                }
                _ => map,
            };
            for (k, v) in map {
                if !in_name_map && ignored(k) {
                    continue;
                }
                let child = canonical(v, !in_name_map && NAME_MAPS.contains(&k.as_str()));
                let child = match (UNORDERED.contains(&k.as_str()) && !in_name_map, child) {
                    (true, Value::Array(mut items)) => {
                        items.sort_by_key(|v| v.to_string());
                        Value::Array(items)
                    }
                    (_, c) => c,
                };
                out.insert(k.clone(), child);
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(|v| canonical(v, false)).collect()),
        Value::Number(n) => match n.as_f64() {
            Some(f) if f.fract() == 0.0 && f.abs() < 9.0e15 => Value::from(f as i64),
            _ => Value::Number(n.clone()),
        },
        other => other.clone(),
    }
}

/// `(path, method, operationId) → comparable operation`.
fn operations(doc: &Value, ctx: &Ctx) -> BTreeMap<String, (String, Value)> {
    let root_security = doc.get("security").cloned().unwrap_or(Value::Array(vec![]));
    let mut out = BTreeMap::new();
    for (path, item) in doc["paths"].as_object().unwrap() {
        let item = resolve(item, ctx, 0);
        let shared_params = item
            .get("parameters")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for method in METHODS {
            let Some(op) = item.get(method) else { continue };
            let id = op["operationId"]
                .as_str()
                .unwrap_or_else(|| panic!("{method} {path}: no operationId"))
                .to_owned();
            let mut params: BTreeMap<(String, String), Value> = BTreeMap::new();
            for p in shared_params.iter().chain(
                op.get("parameters")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten(),
            ) {
                let mut p = p.as_object().unwrap().clone();
                let key = (
                    p["in"].as_str().unwrap().to_owned(),
                    p["name"].as_str().unwrap().to_owned(),
                );
                p.entry("required").or_insert(Value::Bool(false));
                params.insert(key, Value::Object(p));
            }
            let mut cmp = Map::new();
            cmp.insert("operationId".into(), Value::String(id.clone()));
            cmp.insert(
                "security".into(),
                op.get("security")
                    .cloned()
                    .unwrap_or_else(|| root_security.clone()),
            );
            cmp.insert(
                "parameters".into(),
                Value::Array(params.into_values().collect()),
            );
            if let Some(body) = op.get("requestBody") {
                let mut body = body.as_object().unwrap().clone();
                body.entry("required").or_insert(Value::Bool(false));
                cmp.insert("requestBody".into(), Value::Object(body));
            }
            cmp.insert(
                "responses".into(),
                op.get("responses").cloned().unwrap_or_default(),
            );
            out.insert(
                id,
                (
                    format!("{} {path}", method.to_uppercase()),
                    canonical(&Value::Object(cmp), false),
                ),
            );
        }
    }
    out
}

fn escape(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

fn diff(generated: &Value, contract: &Value, pointer: String, out: &mut BTreeSet<String>) {
    match (generated, contract) {
        (Value::Object(g), Value::Object(c)) => {
            for key in g.keys().chain(c.keys()).collect::<BTreeSet<_>>() {
                let child = format!("{pointer}/{}", escape(key));
                match (g.get(key), c.get(key)) {
                    (Some(gv), Some(cv)) => diff(gv, cv, child, out),
                    _ => {
                        out.insert(child);
                    }
                }
            }
        }
        (Value::Array(g), Value::Array(c)) if g.len() == c.len() => {
            for (i, (gv, cv)) in g.iter().zip(c).enumerate() {
                diff(gv, cv, format!("{pointer}/{i}"), out);
            }
        }
        (g, c) if g == c => {}
        _ => {
            out.insert(pointer);
        }
    }
}

#[test]
fn generated_spec_is_an_exact_subset_of_the_contract() {
    let dir = contract_dir();
    let contract_doc = load_file(&dir.join("openapi.yaml"));
    let contract_ops = operations(
        &contract_doc,
        &Ctx {
            doc: contract_doc.clone(),
            dir: dir.clone(),
        },
    );
    let generated_doc = app::openapi::document().unwrap();
    let generated_ops = operations(
        &generated_doc,
        &Ctx {
            doc: generated_doc.clone(),
            dir: dir.clone(),
        },
    );
    let allow: Allowlist =
        yaml_serde::from_str(&std::fs::read_to_string(dir.join("drift-allowlist.yaml")).unwrap())
            .unwrap();

    let mut failures = Vec::new();
    let not_implemented: BTreeSet<&String> = allow.not_implemented.iter().collect();
    for id in contract_ops.keys() {
        match (generated_ops.contains_key(id), not_implemented.contains(id)) {
            (true, true) => failures.push(format!(
                "{id}: has a handler but is listed in not_implemented"
            )),
            (false, false) => failures.push(format!(
                "{id}: no handler and not listed in not_implemented"
            )),
            _ => {}
        }
    }
    for id in &allow.not_implemented {
        if !contract_ops.contains_key(id) {
            failures.push(format!(
                "{id}: listed in not_implemented but not in the contract"
            ));
        }
    }

    let mut used: BTreeSet<AllowEntry> = BTreeSet::new();
    for (id, (route, generated)) in &generated_ops {
        let Some((contract_route, contract)) = contract_ops.get(id) else {
            failures.push(format!("{id} ({route}): not in the contract"));
            continue;
        };
        if route != contract_route {
            failures.push(format!(
                "{id}: generated at {route}, contract has {contract_route}"
            ));
        }
        let mut differences = BTreeSet::new();
        diff(generated, contract, String::new(), &mut differences);
        for pointer in differences {
            match allow
                .entries
                .iter()
                .find(|e| &e.operation_id == id && e.pointer == pointer)
            {
                Some(entry) => {
                    used.insert(entry.clone());
                }
                None => failures.push(format!(
                    "{id} {pointer}: generated {} vs contract {}",
                    generated
                        .pointer(&pointer)
                        .map_or("<absent>".into(), Value::to_string),
                    contract
                        .pointer(&pointer)
                        .map_or("<absent>".into(), Value::to_string),
                )),
            }
        }
    }
    for entry in &allow.entries {
        if !used.contains(entry) {
            failures.push(format!(
                "stale allowlist entry: {} {}",
                entry.operation_id, entry.pointer
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "OpenAPI drift:\n{}",
        failures.join("\n")
    );
}

#[test]
fn embedded_instruction_schema_is_the_contract_file() {
    let file = std::fs::read(repo_root().join("contracts/instruction/v1/instruction.schema.json"))
        .unwrap();
    assert_eq!(
        Sha256::digest(contracts::instruction::v1::SCHEMA_JSON.as_bytes()),
        Sha256::digest(&file),
        "core/crates/contracts embeds a different instruction schema"
    );
}

#[test]
fn diff_and_normalization_rules() {
    let a = serde_json::json!({"type": ["null", "string"], "required": ["b", "a"], "maximum": 1.0, "description": "x"});
    let b = serde_json::json!({"type": ["string", "null"], "required": ["a", "b"], "maximum": 1, "x-k": true});
    let mut out = BTreeSet::new();
    diff(
        &canonical(&a, false),
        &canonical(&b, false),
        String::new(),
        &mut out,
    );
    assert!(out.is_empty(), "{out:?}");
    // const is the single-value enum; a property named `const` is not rewritten.
    let a = serde_json::json!({"properties": {"status": {"const": "rejected"}, "const": {"type": "string"}}});
    let b = serde_json::json!({"properties": {"status": {"enum": ["rejected"]}, "const": {"type": "string"}}});
    let mut out = BTreeSet::new();
    diff(
        &canonical(&a, false),
        &canonical(&b, false),
        String::new(),
        &mut out,
    );
    assert!(out.is_empty(), "{out:?}");
    // A property named like an ignored keyword is still compared.
    let a = serde_json::json!({"properties": {"description": {"type": "string"}}});
    let b = serde_json::json!({"properties": {}});
    let mut out = BTreeSet::new();
    diff(
        &canonical(&a, false),
        &canonical(&b, false),
        String::new(),
        &mut out,
    );
    assert_eq!(
        out.into_iter().collect::<Vec<_>>(),
        ["/properties/description"]
    );
}

/// Shared schemas in `contracts::http`, checked against the contract components of the
/// same name before any operation uses them (same resolution and normalization rules).
#[derive(utoipa::OpenApi)]
#[openapi(components(schemas(
    contracts::http::Uuid,
    contracts::http::ErrorCode,
    contracts::http::Timestamp,
    contracts::http::TimeZone,
    contracts::http::DisplayName,
    contracts::http::Locale,
    contracts::http::Error,
    contracts::http::IngressAccepted,
    contracts::http::IngressRejected,
)))]
struct SharedSchemas;

#[test]
fn shared_schemas_match_the_contract_components() {
    use utoipa::OpenApi;
    let dir = contract_dir();
    let contract = load_file(&dir.join("openapi.yaml"));
    let contract_ctx = Ctx {
        doc: contract.clone(),
        dir: dir.clone(),
    };
    let generated = serde_json::to_value(SharedSchemas::openapi()).unwrap();
    let generated_ctx = Ctx {
        doc: generated.clone(),
        dir,
    };
    let mut failures = Vec::new();
    let names = [
        "Uuid",
        "ErrorCode",
        "Timestamp",
        "TimeZone",
        "DisplayName",
        "Locale",
        "Error",
        "IngressAccepted",
        "IngressRejected",
    ];
    for name in names {
        let g = generated["components"]["schemas"]
            .get(name)
            .unwrap_or_else(|| panic!("{name} not generated"));
        let c = contract["components"]["schemas"]
            .get(name)
            .unwrap_or_else(|| panic!("{name} not in contract"));
        let mut out = BTreeSet::new();
        diff(
            &canonical(&resolve(g, &generated_ctx, 0), false),
            &canonical(&resolve(c, &contract_ctx, 0), false),
            String::new(),
            &mut out,
        );
        failures.extend(out.into_iter().map(|p| format!("{name} {p}")));
    }
    assert!(
        failures.is_empty(),
        "shared schema drift:\n{}",
        failures.join("\n")
    );
}
