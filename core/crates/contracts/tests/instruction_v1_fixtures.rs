//! Conformance against the shared fixtures in `contracts/instruction/v1/examples`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fs;
use std::path::{Path, PathBuf};

use contracts::instruction::v1::{self, ParseError};
use serde_json::Value;

fn fixtures(kind: &str) -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../contracts/instruction/v1/examples")
        .join(kind);
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no fixtures in {}", dir.display());
    files
}

#[test]
fn every_valid_fixture_parses_and_round_trips_exactly() {
    for path in fixtures("valid") {
        let raw = fs::read(&path).unwrap();
        let original: Value = serde_json::from_slice(&raw).unwrap();
        let parsed = v1::parse(&raw).unwrap_or_else(|e| panic!("{}: {e}", path.display()));

        let mut emitted = serde_json::to_value(&parsed).unwrap();
        let mut expected = original.clone();
        // sent_at is compared as an instant; every other field (decimals included) byte-for-byte.
        let sent_emitted = emitted.as_object_mut().unwrap().remove("sent_at").unwrap();
        let sent_expected = expected.as_object_mut().unwrap().remove("sent_at").unwrap();
        let instant =
            |v: &Value| chrono::DateTime::parse_from_rfc3339(v.as_str().unwrap()).unwrap();
        assert_eq!(
            instant(&sent_emitted),
            instant(&sent_expected),
            "{}",
            path.display()
        );
        assert_eq!(emitted, expected, "{}", path.display());

        // Re-parsing the emitted form is lossless as well.
        let again = v1::parse(&serde_json::to_vec(&parsed).unwrap()).unwrap();
        assert_eq!(again, parsed, "{}", path.display());
    }
}

#[test]
fn every_invalid_fixture_is_rejected_by_the_schema() {
    for path in fixtures("invalid") {
        let raw = fs::read(&path).unwrap();
        match v1::parse(&raw) {
            Err(ParseError::Schema { pointers }) => assert!(!pointers.is_empty()),
            other => panic!("{} not rejected by schema: {other:?}", path.display()),
        }
    }
}

#[test]
fn schema_errors_never_echo_values() {
    let raw = br#"{"schema_version":"1.0","message_id":"m 1","sent_at":"2026-10-22T16:03:20Z",
        "source":{"type":"tradingview"},"signal_type":"trigger","action":"buy",
        "instrument":{"symbol":"MNQ"},"quantity":1,"password":"hunter2-secret"}"#;
    let err = v1::parse(raw).unwrap_err();
    let shown = format!("{err} {err:?}");
    assert!(!shown.contains("hunter2-secret"));
    assert!(!shown.contains("m 1"));
}

#[test]
fn malformed_json_is_distinguished() {
    assert!(matches!(
        v1::parse(b"{not json"),
        Err(ParseError::Malformed)
    ));
}

#[test]
fn pointers_name_the_offending_field_without_echoing_unexpected_keys() {
    let base = r#""schema_version":"1.0","message_id":"m1","sent_at":"2026-10-22T16:03:20Z",
        "source":{"type":"tradingview"},"signal_type":"trigger","action":"buy","instrument":{"symbol":"MNQ"}"#;
    let first = |raw: String| match v1::parse(raw.as_bytes()) {
        Err(ParseError::Schema { pointers }) => pointers,
        other => panic!("{other:?}"),
    };
    assert!(first(format!(r#"{{{base},"quantity":0}}"#)).contains(&"/quantity".to_owned()));
    assert!(first(format!(r#"{{{base}}}"#)).contains(&"/quantity".to_owned()));
    let planted = "awk_AAAAAAAAAAAA_BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";
    let pointers = first(format!(r#"{{{base},"quantity":1,"{planted}":1}}"#));
    assert!(pointers.iter().all(|p| !p.contains("awk_")), "{pointers:?}");
}

#[test]
fn schema_version_is_peeked_before_validation() {
    let v = |s: &str| serde_json::json!({ "schema_version": s });
    assert_eq!(v1::peek_schema_version(&v("1.3")), Some((1, 3)));
    assert_eq!(v1::peek_schema_version(&v("2.0")), Some((2, 0)));
    assert_eq!(v1::peek_schema_version(&v("1.x")), None);
    assert_eq!(v1::peek_schema_version(&serde_json::json!({})), None);
}
