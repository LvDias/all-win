//! Preset releases (V1-S16): the release file, the Core's import checks and the publication plan.

pub mod file;
pub mod import;

use serde_json::Value;

/// Canonical JSON: object keys sorted at every level, no insignificant whitespace. Its sha256 is
/// the release's idempotency key; the same bytes compare changes order-insensitively.
pub fn canonical_json(v: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    write_canonical(v, &mut out);
    out
}

fn write_canonical(v: &Value, out: &mut Vec<u8>) {
    match v {
        Value::Object(m) => {
            out.push(b'{');
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort();
            for (i, k) in keys.into_iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_canonical(&Value::String(k.clone()), out);
                out.push(b':');
                if let Some(child) = m.get(k) {
                    write_canonical(child, out);
                }
            }
            out.push(b'}');
        }
        Value::Array(items) => {
            out.push(b'[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_canonical(item, out);
            }
            out.push(b']');
        }
        // Scalars: serde_json's own (escaped strings, exact numbers as parsed).
        scalar => out.extend_from_slice(scalar.to_string().as_bytes()),
    }
}

#[cfg(test)]
mod tests;
