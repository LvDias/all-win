//! Request validation against the contract schemas: 400 `common.invalid_request` with the JSON
//! pointer of the first offending field. Pointers are built only from instance paths and
//! schema-defined property names, never from input values or unexpected keys.
//! Same rules as the accounts adapter (copied: modules never depend on each other).

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use serde_json::Value;

use super::schemas::component;

/// Maximum pointer length (`Error.error.pointer`).
const MAX_POINTER_LEN: usize = 256;

static VALIDATORS: LazyLock<Mutex<HashMap<&'static str, std::sync::Arc<jsonschema::Validator>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Invalid {
    /// Not JSON at all.
    Malformed,
    /// Schema violation at this pointer.
    At(String),
    /// The embedded schema is unusable (build defect; 500).
    Unavailable,
}

/// The cached validator of `schema` under `key` (keys are unique per schema across callers).
fn validator_with(
    key: &'static str,
    schema: &Value,
) -> Result<std::sync::Arc<jsonschema::Validator>, Invalid> {
    let mut cache = VALIDATORS.lock().map_err(|_| Invalid::Unavailable)?;
    if let Some(v) = cache.get(key) {
        return Ok(v.clone());
    }
    let v =
        std::sync::Arc::new(jsonschema::validator_for(schema).map_err(|_| Invalid::Unavailable)?);
    cache.insert(key, v.clone());
    Ok(v)
}

/// Parses `bytes` as JSON and validates it against the component `name`.
pub fn body(bytes: &[u8], name: &'static str) -> Result<Value, Invalid> {
    let schema = component(name).ok_or(Invalid::Unavailable)?;
    body_with(bytes, name, schema)
}

pub fn value_against(value: &Value, name: &'static str) -> Result<(), Invalid> {
    let schema = component(name).ok_or(Invalid::Unavailable)?;
    value_with(value, name, schema)
}

/// [`body`] against a schema the caller holds (another file's components), cached under `key`.
pub fn body_with(bytes: &[u8], key: &'static str, schema: &Value) -> Result<Value, Invalid> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| Invalid::Malformed)?;
    value_with(&value, key, schema)?;
    Ok(value)
}

/// [`value_against`] against a schema the caller holds, cached under `key`.
pub fn value_with(value: &Value, key: &'static str, schema: &Value) -> Result<(), Invalid> {
    let v = validator_with(key, schema)?;
    match v.iter_errors(value).next() {
        None => Ok(()),
        Some(e) => Err(Invalid::At(truncate(pointer_of(&e, schema, value)))),
    }
}

fn pointer_of(error: &jsonschema::ValidationError<'_>, schema: &Value, instance: &Value) -> String {
    let base = error.instance_path().to_string();
    match error.kind() {
        // Missing property: point at it (the name comes from the schema).
        jsonschema::error::ValidationErrorKind::Required {
            property: Value::String(name),
        } => format!("{base}/{}", escape(name)),
        // A forbidden combination (`not: {anyOf: [{required: [x]}…]}`): point at the first
        // forbidden property present, in schema order.
        jsonschema::error::ValidationErrorKind::Not { .. } => {
            let node = schema.pointer(&error.schema_path().to_string());
            let present = instance.pointer(&base).and_then(Value::as_object);
            node.and_then(|n| n.get("not").or(Some(n)))
                .and_then(|not| first_forbidden(not, present?))
                .map_or(base.clone(), |name| format!("{base}/{}", escape(&name)))
        }
        // Unexpected keys are input; point at the object that holds them.
        _ => base,
    }
}

fn first_forbidden(not: &Value, present: &serde_json::Map<String, Value>) -> Option<String> {
    let mut names: Vec<&str> = Vec::new();
    if let Some(required) = not.get("required").and_then(Value::as_array) {
        names.extend(required.iter().filter_map(Value::as_str));
    }
    if let Some(any) = not.get("anyOf").and_then(Value::as_array) {
        for alt in any {
            if let Some(required) = alt.get("required").and_then(Value::as_array) {
                names.extend(required.iter().filter_map(Value::as_str));
            }
        }
    }
    names
        .into_iter()
        .find(|n| present.contains_key(*n))
        .map(str::to_owned)
}

fn escape(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

fn truncate(mut s: String) -> String {
    if s.len() > MAX_POINTER_LEN {
        let mut end = MAX_POINTER_LEN;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        s.truncate(end);
    }
    s
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use serde_json::json;

    const SOURCE: &str = "3c4d5e6f-7a8b-4c9d-8e0f-1a2b3c4d5e6f";
    const ACCOUNT: &str = "0b9f4c1e-7d2a-4e7b-9a52-3c1d8e6f2a10";

    fn create(v: Value) -> Result<(), Invalid> {
        value_against(&v, "RouteCreateRequest")
    }

    fn update(v: Value) -> Result<(), Invalid> {
        value_against(&v, "RouteUpdateRequest")
    }

    #[test]
    fn valid_examples_pass() {
        create(json!({"source_id": SOURCE, "account_id": ACCOUNT,
                      "signal_types": ["trigger", "trade_plan"], "is_enabled": true}))
        .unwrap();
        create(
            json!({"source_id": SOURCE, "account_id": ACCOUNT, "signal_types": ["nq_trigger"],
                      "step_up": {"challenge_id": "abcdefghijklmnop", "code": "ABCD-1234"}}),
        )
        .unwrap();
        update(json!({"signal_types": ["trigger"], "is_enabled": false, "row_version": 3}))
            .unwrap();
    }

    #[test]
    fn pointers_name_the_offending_field() {
        assert_eq!(
            create(json!({"source_id": SOURCE, "account_id": ACCOUNT, "signal_types": []})),
            Err(Invalid::At("/signal_types".into()))
        );
        assert_eq!(
            update(json!({"signal_types": [], "is_enabled": true, "row_version": 1})),
            Err(Invalid::At("/signal_types".into()))
        );
        assert_eq!(
            create(json!({"source_id": SOURCE, "account_id": ACCOUNT,
                          "signal_types": ["trigger", "trigger"]})),
            Err(Invalid::At("/signal_types".into()))
        );
        assert_eq!(
            create(json!({"source_id": SOURCE, "account_id": ACCOUNT,
                          "signal_types": ["master_account"]})),
            Err(Invalid::At("/signal_types/0".into()))
        );
        assert_eq!(
            create(
                json!({"source_id": SOURCE.to_uppercase(), "account_id": ACCOUNT,
                          "signal_types": ["trigger"]})
            ),
            Err(Invalid::At("/source_id".into()))
        );
        assert_eq!(
            create(json!({"source_id": SOURCE, "signal_types": ["trigger"]})),
            Err(Invalid::At("/account_id".into()))
        );
        assert_eq!(
            update(json!({"signal_types": ["trigger"], "is_enabled": true})),
            Err(Invalid::At("/row_version".into()))
        );
        assert_eq!(
            update(json!({"signal_types": ["trigger"], "is_enabled": true, "row_version": 0})),
            Err(Invalid::At("/row_version".into()))
        );
        assert_eq!(
            create(
                json!({"source_id": SOURCE, "account_id": ACCOUNT, "signal_types": ["trigger"],
                          "step_up": {"challenge_id": "short", "code": "x"}})
            ),
            Err(Invalid::At("/step_up/challenge_id".into()))
        );
        // An unexpected key points at its object, never echoes the key.
        assert_eq!(
            create(
                json!({"source_id": SOURCE, "account_id": ACCOUNT, "signal_types": ["trigger"],
                          "secret_field": 1})
            ),
            Err(Invalid::At(String::new()))
        );
        assert_eq!(
            update(
                json!({"signal_types": ["trigger"], "is_enabled": true, "row_version": 1,
                          "step_up": {"challenge_id": "abcdefghijklmnop", "code": "x", "extra": 1}})
            ),
            Err(Invalid::At("/step_up".into()))
        );
        assert_eq!(body(b"{", "RouteCreateRequest"), Err(Invalid::Malformed));
    }
}
