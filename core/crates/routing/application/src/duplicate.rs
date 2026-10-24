//! Duplicate comparison (HTTP 5.4.0 `ingress.duplicate_conflict`; Architect, 2026-10-24): a
//! `message_id` seen again is compared on a CLOSED list of the fields that define the
//! instruction for execution. Anything outside the list (`message_id`, `sent_at`,
//! `reference_price`, `details`, `schema_version`) never makes a conflict. A new instruction
//! field joins the list only through a contract change (`contracts/instruction/v1/README.md`).
//!
//! The receipt keeps `body_sha256` only while the instruction's content is retained: the Q13 b
//! purge clears it with the instruction, so no fingerprint of the content outlives it.

use serde_json::Value;
use sha2::{Digest, Sha256};

/// The compared fields, in the contract's order (`DetailsIngressDuplicateConflict
/// .differing_fields` enum).
pub const COMPARED_FIELDS: [&str; 10] = [
    "source",
    "signal_type",
    "action",
    "position_effect",
    "side",
    "instrument",
    "quantity",
    "order",
    "changes",
    "refs",
];

/// sha256 of the canonical JSON object holding only the compared fields that are present
/// (absent ≠ null, as in the schema): `routing.message_receipt.body_sha256`.
pub fn body_sha256(document: &Value) -> [u8; 32] {
    let mut body = String::from("{");
    let mut first = true;
    for name in COMPARED_FIELDS {
        if let Some(value) = document.get(name) {
            if !first {
                body.push(',');
            }
            first = false;
            canonical(&Value::String(name.to_owned()), &mut body);
            body.push(':');
            canonical(value, &mut body);
        }
    }
    body.push('}');
    Sha256::digest(body.as_bytes()).into()
}

/// The compared fields that differ between two retained documents, in contract order (names
/// only, never values). A field present in one and absent in the other differs.
pub fn differing(first: &Value, second: &Value) -> Vec<&'static str> {
    COMPARED_FIELDS
        .into_iter()
        .filter(|name| {
            let text = |doc: &Value| {
                doc.get(name).map(|v| {
                    let mut out = String::new();
                    canonical(v, &mut out);
                    out
                })
            };
            text(first) != text(second)
        })
        .collect()
}

/// Canonical JSON: object keys sorted, no whitespace, numbers in one form (an integral float
/// is written as an integer, so `1` and `1.0` compare equal), strings as serde_json escapes
/// them. Independent of how `serde_json::Map` orders keys.
fn canonical(v: &Value, out: &mut String) {
    match v {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (n, k) in keys.into_iter().enumerate() {
                if n > 0 {
                    out.push(',');
                }
                canonical(&Value::String(k.clone()), out);
                out.push(':');
                if let Some(child) = map.get(k) {
                    canonical(child, out);
                }
            }
            out.push('}');
        }
        Value::Array(items) => {
            out.push('[');
            for (n, item) in items.iter().enumerate() {
                if n > 0 {
                    out.push(',');
                }
                canonical(item, out);
            }
            out.push(']');
        }
        Value::Number(n) => match n.as_f64() {
            // Exact for |f| < 2^53.
            Some(f) if n.is_f64() && f.fract() == 0.0 && f.abs() < 9.0e15 => {
                #[allow(clippy::cast_possible_truncation)]
                let whole = f as i64;
                out.push_str(&whole.to_string());
            }
            _ => out.push_str(&n.to_string()),
        },
        other => out.push_str(&other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn instruction() -> Value {
        json!({
            "schema_version": "1.0", "message_id": "tv-MNQ-1", "sent_at": "2026-10-24T12:00:00Z",
            "source": {"type": "tradingview"}, "signal_type": "trigger", "action": "buy",
            "instrument": {"symbol": "MNQ"}, "quantity": 1,
            "reference_price": "21000.25", "details": {"setup": "A"}
        })
    }

    #[test]
    fn fields_outside_the_list_never_conflict() {
        let first = instruction();
        for (field, value) in [
            ("sent_at", json!("2026-10-24T12:00:05Z")),
            ("reference_price", json!("21001.00")),
            ("details", json!({"setup": "B", "volatile": 3})),
            ("schema_version", json!("1.1")),
            ("message_id", json!("other")),
        ] {
            let mut second = instruction();
            second[field] = value;
            assert_eq!(body_sha256(&first), body_sha256(&second), "{field}");
            assert!(differing(&first, &second).is_empty(), "{field}");
        }
    }

    #[test]
    fn a_compared_field_conflicts_by_name_only() {
        let first = instruction();
        let mut second = instruction();
        second["quantity"] = json!(2);
        assert_ne!(body_sha256(&first), body_sha256(&second));
        assert_eq!(differing(&first, &second), ["quantity"]);

        second["action"] = json!("sell");
        second["instrument"] = json!({"symbol": "MNQ", "contract_month": "2026-12"});
        assert_eq!(
            differing(&first, &second),
            ["action", "instrument", "quantity"]
        );
    }

    #[test]
    fn absent_differs_from_null_and_key_order_and_number_form_do_not_matter() {
        let first = instruction();
        let mut second = instruction();
        second["refs"] = Value::Null;
        assert_ne!(body_sha256(&first), body_sha256(&second));
        assert_eq!(differing(&first, &second), ["refs"]);

        let reordered: Value = serde_json::from_str(
            r#"{"quantity": 1.0, "instrument": {"symbol": "MNQ"}, "action": "buy",
                "signal_type": "trigger", "source": {"type": "tradingview"}}"#,
        )
        .unwrap();
        assert_eq!(body_sha256(&reordered), body_sha256(&first));
        assert!(differing(&reordered, &first).is_empty());
    }
}
