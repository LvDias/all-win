//! Reading a preset release file for `publish-release`: JSON, validated against the contract's
//! schema (contracts/presets/v1/release.schema.json) before anything else, then hashed in its
//! canonical form (the release's idempotency key).

use std::sync::LazyLock;

use serde_json::Value;
use sha2::{Digest, Sha256};

use catalog_application::enterprise::release::canonical_json;
use catalog_application::release::ValidatedRelease;

const SCHEMA_JSON: &str = include_str!("../../../../../contracts/presets/v1/release.schema.json");

static VALIDATOR: LazyLock<Result<jsonschema::Validator, String>> = LazyLock::new(|| {
    let schema: Value = serde_json::from_str(SCHEMA_JSON).map_err(|e| e.to_string())?;
    jsonschema::validator_for(&schema).map_err(|e| e.to_string())
});

/// Why a file cannot be read; each entry names the JSON pointer it is about.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("release file refused: {0:?}")]
pub struct ReleaseFileError(pub Vec<String>);

pub fn parse_release(bytes: &[u8]) -> Result<ValidatedRelease, ReleaseFileError> {
    let raw: Value = serde_json::from_slice(bytes).map_err(|e| {
        ReleaseFileError(vec![format!(
            "not JSON: line {}, column {}",
            e.line(),
            e.column()
        )])
    })?;
    let validator = VALIDATOR
        .as_ref()
        .map_err(|e| ReleaseFileError(vec![format!("embedded schema unusable: {e}")]))?;
    let errors: Vec<String> = validator
        .iter_errors(&raw)
        .map(|e| {
            // Admin tool: the file holds no secrets; the message is cut to keep reports short.
            let mut msg = e.to_string();
            msg.truncate(msg.char_indices().nth(200).map_or(msg.len(), |(i, _)| i));
            format!("{}: {msg}", e.instance_path())
        })
        .collect();
    if !errors.is_empty() {
        return Err(ReleaseFileError(errors));
    }
    let file = serde_json::from_value(raw.clone())
        .map_err(|e| ReleaseFileError(vec![format!("schema-valid file not representable: {e}")]))?;
    let sha256 = Sha256::digest(canonical_json(&raw)).into();
    Ok(ValidatedRelease { file, raw, sha256 })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! example {
        ($name:literal) => {
            include_bytes!(concat!(
                "../../../../../contracts/presets/v1/examples/",
                $name
            ))
        };
    }

    #[test]
    fn valid_examples_parse_and_hash_canonically() {
        let a = parse_release(example!("valid/release-v1-new-firm.json")).unwrap();
        // Key order and whitespace do not change the hash.
        let reordered = serde_json::to_vec_pretty(&a.raw).unwrap();
        assert_eq!(parse_release(&reordered).unwrap().sha256, a.sha256);
        let b = parse_release(example!("valid/release-v2-stance-changed.json")).unwrap();
        assert_ne!(a.sha256, b.sha256);
    }

    #[test]
    fn invalid_examples_are_refused_by_the_schema() {
        for bytes in [
            &example!("invalid/money-as-number.json")[..],
            &example!("invalid/database-id-field.json")[..],
            &example!("invalid/not-confirmed-with-value.json")[..],
            &example!("invalid/http-source-url.json")[..],
            &example!("invalid/stored-only-in-live-phase.json")[..],
        ] {
            assert!(parse_release(bytes).is_err());
        }
        assert!(parse_release(b"{").is_err());
    }
}
