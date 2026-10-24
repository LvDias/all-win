//! The shared protocol fixtures (contracts/adapter-protocol/v1/examples) pass and fail exactly as
//! the contract says, and the frames the gateway sends are schema-valid.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use gateway_application::frames::{FrameValidator, ack_frame, error_frame, heartbeat_frame};
use gateway_enterprise::protocol::ProtocolVersion;
use gateway_infra::ProtocolSchema;
use serde_json::Value;
use uuid::Uuid;

fn examples(kind: &str) -> Vec<(String, Value)> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../contracts/adapter-protocol/v1/examples")
        .join(kind);
    let mut out: Vec<(String, Value)> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .map(|p| {
            (
                p.file_name().unwrap().to_string_lossy().into_owned(),
                serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap(),
            )
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!out.is_empty(), "no {kind} fixtures");
    out
}

#[test]
fn valid_fixtures_pass() {
    let schema = ProtocolSchema::load().unwrap();
    for (name, frame) in examples("valid") {
        assert!(
            schema.validate(&frame).is_ok(),
            "{name}: {:?}",
            schema.validate(&frame)
        );
    }
}

#[test]
fn invalid_fixtures_fail() {
    let schema = ProtocolSchema::load().unwrap();
    for (name, frame) in examples("invalid") {
        assert!(schema.validate(&frame).is_err(), "{name} should be invalid");
    }
}

#[test]
fn validator_is_shared_safely_across_threads() {
    let schema = std::sync::Arc::new(ProtocolSchema::load().unwrap());
    let valid = examples("valid");
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let schema = schema.clone();
            let valid = valid.clone();
            std::thread::spawn(move || {
                for _ in 0..20 {
                    for (name, frame) in &valid {
                        assert!(schema.validate(frame).is_ok(), "{name}");
                    }
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn control_frames_the_gateway_sends_are_valid() {
    let schema = ProtocolSchema::load().unwrap();
    let now = chrono::Utc::now();
    for v in [ProtocolVersion::V1_2, ProtocolVersion::V1_3] {
        for frame in [
            ack_frame(v, &[Uuid::now_v7()], now),
            error_frame(v, "protocol.invalid_message", Some(Uuid::now_v7()), now),
            error_frame(v, "protocol.unsupported_value", None, now),
            heartbeat_frame(v, now),
        ] {
            let value: Value = serde_json::from_str(&frame).unwrap();
            assert!(
                schema.validate(&value).is_ok(),
                "{frame}: {:?}",
                schema.validate(&value)
            );
        }
    }
}

/// Each command of routing's C1 fixtures becomes a schema-valid `command` frame.
#[test]
fn routing_c1_fixture_commands_are_valid_protocol_commands() {
    use gateway_application::consume::{CommandsRequested, command_payload};
    let schema = ProtocolSchema::load().unwrap();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../application/fixtures/c1");
    for name in ["entry", "reverse-step1-close-leg", "reverse-step2-open-leg"] {
        let text = std::fs::read_to_string(dir.join(format!("{name}.json"))).unwrap();
        let ev: CommandsRequested = serde_json::from_str(&text).unwrap();
        for c in &ev.commands {
            let frame = serde_json::json!({
                "protocol_version": "1.2", "msg_id": Uuid::now_v7(), "type": "command",
                "sent_at": "2026-10-24T14:31:06.200Z",
                "payload": command_payload(&ev, c, "Sim101"),
            });
            assert!(
                schema.validate(&frame).is_ok(),
                "{name}: {:?}",
                schema.validate(&frame)
            );
        }
    }
}
