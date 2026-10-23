//! Cross-cutting `/api/v1` request guards (HTTP contract 1.1.0 conventions). Self-contained:
//! depends on axum, serde_json, utoipa and `contracts::http` only, so it can move to a shared
//! crate when another module needs it.
//!
//! - [`check_origin`]: strict `Origin` check on state-changing methods → 403 `common.forbidden_origin`.
//! - [`ValidJson`]: body validated against the request type's **own utoipa schema** (the same
//!   schema the generated OpenAPI publishes and the drift check compares with the contract),
//!   then deserialized; the first violation → 400 `common.invalid_request` with `pointer`.
//! - [`path_uuid`], [`query_bool`]: parameters → `/path/<name>`, `/query/<name>` pointers.

use std::collections::HashMap;
use std::sync::OnceLock;

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use axum::http::{HeaderMap, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use utoipa::PartialSchema;

/// Largest accepted JSON body for `/api/v1` operations.
pub const MAX_BODY_BYTES: usize = 64 * 1024;

/// A rejected request, rendered as the contract `Error` body.
#[derive(Debug)]
pub struct Rejection {
    pub status: StatusCode,
    pub body: contracts::http::Error,
}

impl Rejection {
    pub fn invalid(pointer: &str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: contracts::http::Error::new("common.invalid_request").with_pointer(pointer),
        }
    }

    pub fn forbidden_origin() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            body: contracts::http::Error::new("common.forbidden_origin"),
        }
    }

    pub fn internal() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: contracts::http::Error::new("common.internal"),
        }
    }
}

impl IntoResponse for Rejection {
    fn into_response(self) -> Response {
        (self.status, axum::Json(self.body)).into_response()
    }
}

pub fn is_state_changing(method: &Method) -> bool {
    matches!(
        *method,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    )
}

/// `Origin` must equal the app origin exactly on state-changing requests.
pub fn check_origin(method: &Method, headers: &HeaderMap, allowed: &str) -> Result<(), Rejection> {
    if !is_state_changing(method) {
        return Ok(());
    }
    match headers.get(header::ORIGIN).and_then(|v| v.to_str().ok()) {
        Some(origin) if origin == allowed => Ok(()),
        _ => Err(Rejection::forbidden_origin()),
    }
}

/// Component schemas used to resolve `$ref`s while validating (set once at router build).
static COMPONENTS: OnceLock<Map<String, Value>> = OnceLock::new();

/// Registers the component schemas of the module's OpenAPI document.
pub fn register_components(doc: &utoipa::openapi::OpenApi) {
    let components = serde_json::to_value(doc)
        .ok()
        .and_then(|v| v.pointer("/components/schemas").cloned())
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default();
    let _ = COMPONENTS.set(components);
}

/// JSON body validated against `T`'s schema, then deserialized.
pub struct ValidJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidJson<T>
where
    T: DeserializeOwned + PartialSchema,
    S: Send + Sync,
{
    type Rejection = Rejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let json_type = req
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|ct| {
                ct.split(';')
                    .next()
                    .is_some_and(|m| m.trim().eq_ignore_ascii_case("application/json"))
            });
        if !json_type {
            return Err(Rejection::invalid(""));
        }
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| Rejection::invalid(""))?;
        if bytes.len() > MAX_BODY_BYTES {
            return Err(Rejection::invalid(""));
        }
        let value: Value = serde_json::from_slice(&bytes).map_err(|_| Rejection::invalid(""))?;
        validate_against::<T>(&value)?;
        serde_json::from_value(value)
            .map(ValidJson)
            .map_err(|_| Rejection::invalid(""))
    }
}

pub fn validate_against<T: PartialSchema>(value: &Value) -> Result<(), Rejection> {
    let schema = serde_json::to_value(T::schema()).map_err(|_| Rejection::internal())?;
    let empty = Map::new();
    let components = COMPONENTS.get().unwrap_or(&empty);
    validate(&schema, value, "", components).map_err(|e| match e {
        Violation::At(pointer) => Rejection::invalid(&pointer),
        Violation::UnsupportedSchema(_) => Rejection::internal(),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum Violation {
    At(String),
    /// The schema uses a keyword or pattern this validator does not implement (a bug, not a
    /// client error); covered by a test over every request schema of the module.
    UnsupportedSchema(String),
}

fn escape(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

/// Validates the JSON Schema subset utoipa emits for request bodies: `$ref`, `type`,
/// `properties`, `required`, `additionalProperties: false`, `minLength`, `maxLength`,
/// `pattern` (known patterns), `enum`, `minimum`, `maximum`, `minProperties`, `items`.
pub fn validate(
    schema: &Value,
    value: &Value,
    at: &str,
    components: &Map<String, Value>,
) -> Result<(), Violation> {
    if let Some(r) = schema.get("$ref").and_then(Value::as_str) {
        let name = r
            .strip_prefix("#/components/schemas/")
            .ok_or_else(|| Violation::UnsupportedSchema(r.into()))?;
        let target = components
            .get(name)
            .ok_or_else(|| Violation::UnsupportedSchema(r.into()))?;
        return validate(target, value, at, components);
    }
    for key in ["oneOf", "anyOf", "allOf"] {
        if let Some(options) = schema.get(key).and_then(Value::as_array) {
            // Option<T> fields: utoipa may wrap the type; accept the first non-null option.
            let non_null: Vec<&Value> = options
                .iter()
                .filter(|o| o.get("type") != Some(&Value::from("null")))
                .collect();
            if key == "allOf" || non_null.len() == 1 {
                for option in non_null {
                    validate(option, value, at, components)?;
                }
                return Ok(());
            }
            return Err(Violation::UnsupportedSchema(key.into()));
        }
    }
    let violation = || Violation::At(at.to_owned());
    let types: Vec<&str> = match schema.get("type") {
        Some(Value::String(t)) => vec![t.as_str()],
        Some(Value::Array(ts)) => ts.iter().filter_map(Value::as_str).collect(),
        _ => vec![],
    };
    if !types.is_empty() {
        let actual = match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(n) if n.is_i64() || n.is_u64() => "integer",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };
        let ok = types
            .iter()
            .any(|t| *t == actual || (*t == "number" && actual == "integer"));
        if !ok {
            return Err(violation());
        }
    }
    if let Some(allowed) = schema.get("enum").and_then(Value::as_array)
        && !allowed.contains(value)
    {
        return Err(violation());
    }
    match value {
        Value::String(s) => {
            let chars = s.chars().count();
            if schema
                .get("minLength")
                .and_then(Value::as_u64)
                .is_some_and(|m| (chars as u64) < m)
            {
                return Err(violation());
            }
            if schema
                .get("maxLength")
                .and_then(Value::as_u64)
                .is_some_and(|m| (chars as u64) > m)
            {
                return Err(violation());
            }
            if let Some(pattern) = schema.get("pattern").and_then(Value::as_str) {
                match pattern_matches(pattern, s) {
                    Some(true) => {}
                    Some(false) => return Err(violation()),
                    None => return Err(Violation::UnsupportedSchema(pattern.into())),
                }
            }
        }
        Value::Number(n) => {
            if let (Some(min), Some(v)) =
                (schema.get("minimum").and_then(Value::as_f64), n.as_f64())
                && v < min
            {
                return Err(violation());
            }
            if let (Some(max), Some(v)) =
                (schema.get("maximum").and_then(Value::as_f64), n.as_f64())
                && v > max
            {
                return Err(violation());
            }
        }
        Value::Array(items) => {
            if let Some(item_schema) = schema.get("items") {
                for (i, item) in items.iter().enumerate() {
                    validate(item_schema, item, &format!("{at}/{i}"), components)?;
                }
            }
        }
        Value::Object(obj) => {
            let props = schema.get("properties").and_then(Value::as_object);
            if let Some(required) = schema.get("required").and_then(Value::as_array) {
                for name in required.iter().filter_map(Value::as_str) {
                    if !obj.contains_key(name) {
                        return Err(Violation::At(format!("{at}/{}", escape(name))));
                    }
                }
            }
            if schema
                .get("minProperties")
                .and_then(Value::as_u64)
                .is_some_and(|m| (obj.len() as u64) < m)
            {
                return Err(violation());
            }
            for (name, v) in obj {
                let child = format!("{at}/{}", escape(name));
                match props.and_then(|p| p.get(name)) {
                    Some(prop_schema) => validate(prop_schema, v, &child, components)?,
                    None if schema.get("additionalProperties") == Some(&Value::Bool(false)) => {
                        return Err(Violation::At(child));
                    }
                    None => {}
                }
            }
        }
        _ => {}
    }
    Ok(())
}

/// The patterns the contract uses in `/api/v1` and `/adapter/v1` requests. A pattern missing
/// here is reported as an unsupported schema (500) and caught by tests.
pub fn pattern_matches(pattern: &str, s: &str) -> Option<bool> {
    let b = s.as_bytes();
    Some(match pattern {
        "^[0-9]{6}$" => b.len() == 6 && b.iter().all(u8::is_ascii_digit),
        "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$" => is_lower_uuid(s),
        "^[A-Za-z_]+(/[A-Za-z0-9_+-]+)*$" => {
            let mut parts = s.split('/');
            let first = parts.next().unwrap_or_default();
            !first.is_empty()
                && first.bytes().all(|c| c.is_ascii_alphabetic() || c == b'_')
                && parts.all(|p| {
                    !p.is_empty()
                        && p.bytes()
                            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'+' | b'-'))
                })
        }
        "^[a-z][a-z0-9_]{0,31}$" => {
            (1..=32).contains(&b.len())
                && b[0].is_ascii_lowercase()
                && b.iter()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'_')
        }
        _ => return None,
    })
}

pub fn is_lower_uuid(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 36
        && b.iter().enumerate().all(|(i, c)| match i {
            8 | 13 | 18 | 23 => *c == b'-',
            _ => c.is_ascii_digit() || (b'a'..=b'f').contains(c),
        })
}

/// A path parameter that must be a lowercase UUID (`components.schemas.Uuid`).
pub fn path_uuid(params: &HashMap<String, String>, name: &str) -> Result<uuid::Uuid, Rejection> {
    params
        .get(name)
        .filter(|v| is_lower_uuid(v))
        .and_then(|v| uuid::Uuid::parse_str(v).ok())
        .ok_or_else(|| Rejection::invalid(&format!("/path/{name}")))
}

/// An optional boolean query parameter (`true` / `false`; default when absent).
pub fn query_bool(query: Option<&str>, name: &str, default: bool) -> Result<bool, Rejection> {
    let mut value = default;
    for pair in query
        .unwrap_or_default()
        .split('&')
        .filter(|p| !p.is_empty())
    {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k == name {
            value = match v {
                "true" => true,
                "false" => false,
                _ => return Err(Rejection::invalid(&format!("/query/{name}"))),
            };
        }
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn check(schema: Value, value: Value) -> Result<(), Violation> {
        validate(&schema, &value, "", &Map::new())
    }

    #[test]
    fn object_rules_and_pointers() {
        let schema = json!({
            "type": "object", "additionalProperties": false, "required": ["name"],
            "properties": { "name": { "type": "string", "minLength": 1, "maxLength": 3 }, "a/b": { "type": "boolean" } }
        });
        assert_eq!(check(schema.clone(), json!({"name": "abc"})), Ok(()));
        assert_eq!(
            check(schema.clone(), json!({})),
            Err(Violation::At("/name".into()))
        );
        assert_eq!(
            check(schema.clone(), json!({"name": "abcd"})),
            Err(Violation::At("/name".into()))
        );
        assert_eq!(
            check(schema.clone(), json!({"name": ""})),
            Err(Violation::At("/name".into()))
        );
        assert_eq!(
            check(schema.clone(), json!({"name": "é€x"})),
            Ok(()),
            "length in characters"
        );
        assert_eq!(
            check(schema.clone(), json!({"name": "a", "x": 1})),
            Err(Violation::At("/x".into()))
        );
        assert_eq!(
            check(schema.clone(), json!({"name": "a", "a/b": 1})),
            Err(Violation::At("/a~1b".into()))
        );
        assert_eq!(check(schema, json!([])), Err(Violation::At(String::new())));
    }

    #[test]
    fn refs_enums_and_patterns() {
        let components: Map<String, Value> = serde_json::from_value(json!({
            "Code": { "type": "string", "pattern": "^[0-9]{6}$" }
        }))
        .unwrap();
        let schema = json!({ "type": "object", "properties": {
            "code": { "$ref": "#/components/schemas/Code" },
            "build": { "type": "string", "enum": ["release", "development"] } } });
        assert_eq!(
            validate(
                &schema,
                &json!({"code": "123456", "build": "release"}),
                "",
                &components
            ),
            Ok(())
        );
        assert_eq!(
            validate(&schema, &json!({"code": "12345a"}), "", &components),
            Err(Violation::At("/code".into()))
        );
        assert_eq!(
            validate(&schema, &json!({"build": "debug"}), "", &components),
            Err(Violation::At("/build".into()))
        );
        let unknown = json!({ "type": "string", "pattern": "^x+$" });
        assert!(matches!(
            check(unknown, json!("x")),
            Err(Violation::UnsupportedSchema(_))
        ));
    }

    #[test]
    fn known_patterns() {
        assert_eq!(
            pattern_matches("^[A-Za-z_]+(/[A-Za-z0-9_+-]+)*$", "America/Sao_Paulo"),
            Some(true)
        );
        assert_eq!(
            pattern_matches("^[A-Za-z_]+(/[A-Za-z0-9_+-]+)*$", "Etc/GMT+3"),
            Some(true)
        );
        assert_eq!(
            pattern_matches("^[A-Za-z_]+(/[A-Za-z0-9_+-]+)*$", "America//X"),
            Some(false)
        );
        assert_eq!(
            pattern_matches("^[a-z][a-z0-9_]{0,31}$", "ninjatrader8"),
            Some(true)
        );
        assert_eq!(
            pattern_matches("^[a-z][a-z0-9_]{0,31}$", "8nt"),
            Some(false)
        );
        assert!(is_lower_uuid("01900000-0000-7000-8000-00000000000a"));
        assert!(!is_lower_uuid("01900000-0000-7000-8000-00000000000A"));
    }

    #[test]
    fn origin_only_on_state_changing_methods() {
        let mut h = HeaderMap::new();
        assert!(check_origin(&Method::GET, &h, "https://app.test").is_ok());
        assert!(check_origin(&Method::POST, &h, "https://app.test").is_err());
        h.insert(header::ORIGIN, "https://evil.test".parse().unwrap());
        assert!(check_origin(&Method::PATCH, &h, "https://app.test").is_err());
        h.insert(header::ORIGIN, "https://app.test".parse().unwrap());
        assert!(check_origin(&Method::DELETE, &h, "https://app.test").is_ok());
    }

    #[test]
    fn parameters() {
        let mut p = HashMap::new();
        p.insert("source_id".to_string(), "not-a-uuid".to_string());
        assert_eq!(
            path_uuid(&p, "source_id")
                .unwrap_err()
                .body
                .error
                .pointer
                .as_deref(),
            Some("/path/source_id")
        );
        assert!(query_bool(Some("include_revoked=true"), "include_revoked", false).unwrap());
        assert!(!query_bool(None, "include_revoked", false).unwrap());
        assert_eq!(
            query_bool(Some("include_revoked=1"), "include_revoked", false)
                .unwrap_err()
                .body
                .error
                .pointer
                .as_deref(),
            Some("/query/include_revoked")
        );
    }
}
