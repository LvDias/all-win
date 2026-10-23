//! Request validation against the contract schemas: 400 `common.invalid_request` with the JSON
//! pointer of the first offending field. Pointers are built only from instance paths and
//! schema-defined property names, never from input values or unexpected keys.

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

fn validator(name: &'static str) -> Result<std::sync::Arc<jsonschema::Validator>, Invalid> {
    let mut cache = VALIDATORS.lock().map_err(|_| Invalid::Unavailable)?;
    if let Some(v) = cache.get(name) {
        return Ok(v.clone());
    }
    let schema = component(name).ok_or(Invalid::Unavailable)?;
    let v =
        std::sync::Arc::new(jsonschema::validator_for(schema).map_err(|_| Invalid::Unavailable)?);
    cache.insert(name, v.clone());
    Ok(v)
}

/// Parses `bytes` as JSON and validates it against the component `name`.
pub fn body(bytes: &[u8], name: &'static str) -> Result<Value, Invalid> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| Invalid::Malformed)?;
    value_against(&value, name)?;
    Ok(value)
}

pub fn value_against(value: &Value, name: &'static str) -> Result<(), Invalid> {
    let v = validator(name)?;
    let schema = component(name).ok_or(Invalid::Unavailable)?;
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

    fn check(v: Value) -> Result<(), Invalid> {
        value_against(&v, "AccountCreateRequest")
    }

    #[test]
    fn valid_examples_pass() {
        check(json!({"kind": "real", "display_name": "Conta real", "size_amount": "25000.00",
                     "max_position_qty": 3, "multiplier": 1,
                     "binding": {"connection_id": "5a2b8c3d-1e4f-4a6b-8c7d-9e0f1a2b3c4d", "platform_account": "Sim101"}}))
        .unwrap();
        check(
            json!({"kind": "prop", "display_name": "Apex 50K 1", "firm_name": "Apex",
                     "plan_name": "50K", "phase": "Evaluation", "size_amount": "50000.00",
                     "max_position_qty": 2,
                     "opt_ins": {"automation": {"choice": "declined",
                                 "rule_version_id": "1f2e3d4c-5b6a-4978-8a9b-0c1d2e3f4a5b"}}}),
        )
        .unwrap();
    }

    #[test]
    fn pointers_name_the_offending_field() {
        assert_eq!(
            check(json!({"kind": "real", "display_name": "x", "max_position_qty": 1})),
            Err(Invalid::At("/size_amount".into()))
        );
        assert_eq!(
            check(
                json!({"kind": "real", "display_name": "x", "size_amount": "1.00",
                         "max_position_qty": 1, "firm_name": "Apex"})
            ),
            Err(Invalid::At("/firm_name".into()))
        );
        assert_eq!(
            check(
                json!({"kind": "real", "display_name": "x", "size_amount": "0",
                         "max_position_qty": 1})
            ),
            Err(Invalid::At("/size_amount".into()))
        );
        assert_eq!(
            check(
                json!({"kind": "real", "display_name": "x", "size_amount": "1",
                         "max_position_qty": 1, "multiplier": 1.5})
            ),
            Err(Invalid::At("/multiplier".into()))
        );
        // An unexpected key points at its object, never echoes the key.
        assert_eq!(
            check(
                json!({"kind": "real", "display_name": "x", "size_amount": "1",
                         "max_position_qty": 1, "secret_field": 1})
            ),
            Err(Invalid::At(String::new()))
        );
        assert_eq!(body(b"{", "AccountCreateRequest"), Err(Invalid::Malformed));
    }
}
