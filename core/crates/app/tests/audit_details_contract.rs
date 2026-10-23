//! Every audit event the Core emits today carries `details` that match
//! `AuditDetailsByCode[code]` of the bundled HTTP contract (closed schemas), so a new field
//! fails here instead of leaking; `safe_to_display = true` depends on it (Architect,
//! 2026-10-23). Events are produced through the real ingress path and read from the outbox.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

mod support;

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::{Value, json};
use support::*;

macro_rules! db_or_skip {
    () => {
        &match db().await {
            Some(db) => db,
            None => return,
        }
    };
}

fn bundle() -> Value {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../contracts/http/openapi.bundled.yaml");
    yaml_serde::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// A validator for `AuditDetailsByCode[code]`, resolving `#/components/...` in the bundle.
fn details_validator(bundle: &Value, code: &str) -> jsonschema::Validator {
    let reference =
        bundle["components"]["schemas"]["AuditDetailsByCode"]["properties"][code]["$ref"]
            .as_str()
            .unwrap_or_else(|| panic!("{code} has no AuditDetailsByCode entry"));
    let schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$ref": reference,
        "components": bundle["components"],
    });
    jsonschema::validator_for(&schema).unwrap()
}

#[tokio::test]
async fn emitted_ingress_details_match_the_contract_and_never_carry_body_values() {
    let db = db_or_skip!();
    let mut cfg = config();
    cfg.rate_per_key_per_min = 3;
    let app = wire(db, &cfg);
    let s = seed(db).await;
    let path = ingress_path(&s.token);
    let msg = unique_message_id();
    // received, duplicate_ignored
    for _ in 0..2 {
        assert_eq!(
            send(&app.state, post_json(&path, instruction(&msg)))
                .await
                .0,
            200
        );
    }
    // rejected: an unknown key whose value looks like a secret
    let secret_like = "s3cr3t-Value-0123456789-should-never-be-stored";
    let body = instruction(&unique_message_id()).replacen(
        '{',
        &format!(r#"{{"token":"{secret_like}","#),
        1,
    );
    assert_eq!(send(&app.state, post_json(&path, body)).await.0, 400);
    // rate_limited: the key's own limit (3/min)
    assert_eq!(
        send(
            &app.state,
            post_json(&path, instruction(&unique_message_id()))
        )
        .await
        .0,
        429
    );

    let events: Vec<(String, Value)> = as_user(db, s.user_id, |c| {
        Box::pin(async move {
            sqlx::query_as("SELECT event_type, payload FROM app.outbox ORDER BY seq")
                .fetch_all(c)
                .await
                .unwrap()
        })
    })
    .await;
    let bundle = bundle();
    // The validator really enforces the closed schema.
    let received = details_validator(&bundle, "ingress.received");
    let ok =
        json!({"action": "buy", "instrument_symbol": "MNQ", "sent_at": "2026-10-23T12:00:00Z"});
    assert!(received.is_valid(&ok));
    let mut extra = ok.clone();
    extra["message_id"] = json!("m-1");
    assert!(!received.is_valid(&extra), "an unknown field must fail");
    let mut seen = BTreeSet::new();
    for (code, payload) in &events {
        let text = payload.to_string();
        assert!(!text.contains(secret_like), "{code}: body value in payload");
        assert!(!text.contains(&s.secret), "{code}: secret in payload");
        assert!(!text.contains("awk_"), "{code}: token in payload");
        assert_eq!(payload["actor"], "system", "{code}");
        let details = &payload["details"];
        let v = details_validator(&bundle, code);
        let errors: Vec<String> = v.iter_errors(details).map(|e| e.to_string()).collect();
        assert!(errors.is_empty(), "{code}: {errors:?}");
        let refs = &payload["refs"];
        assert_eq!(refs["source_key_id"], s.key_id.as_str(), "{code}");
        assert!(
            refs["source_name"].as_str().is_some_and(|n| !n.is_empty()),
            "{code}"
        );
        seen.insert(code.clone());
    }
    let expected: BTreeSet<String> = adapter_tradingview_types().into_iter().collect();
    assert_eq!(seen, expected, "every ingress code was exercised");

    // The rejection's pointer is a JSON path only (RFC 6901), here the document root.
    let rejected = events.iter().find(|e| e.0 == "ingress.rejected").unwrap();
    assert_eq!(
        rejected.1["details"]["error_code"],
        "ingress.schema_violation"
    );
    assert_eq!(rejected.1["details"]["pointer"], "");
    assert!(rejected.1["details"]["body_size"].as_u64().unwrap() > 0);

    // And all of them become audit rows with the contract's columns.
    drain(&app, db, s.user_id).await;
    let rows: i64 = as_user(db, s.user_id, |c| {
        Box::pin(async move {
            sqlx::query_scalar(
                "SELECT count(*) FROM audit.audit_entry
                  WHERE actor = 'system' AND safe_to_display AND source_name IS NOT NULL AND event_seq > 0",
            )
            .fetch_one(c)
            .await
            .unwrap()
        })
    })
    .await;
    assert_eq!(rows, i64::try_from(events.len()).unwrap());
}

fn adapter_tradingview_types() -> Vec<String> {
    routing_application::enterprise::IngressEvent::TYPES
        .iter()
        .map(|t| (*t).to_owned())
        .collect()
}
