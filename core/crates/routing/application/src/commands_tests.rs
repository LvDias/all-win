use chrono::{TimeZone, Utc};
use routing_enterprise::SourceId;
use routing_enterprise::decision::Outcome;

use super::*;

fn instruction(order: Option<Value>) -> InstructionToRoute {
    InstructionToRoute {
        id: InstructionId::from_uuid(Uuid::from_u128(0x0192_b3c4_0000_7000_8000_0000_0000_0006)),
        source_id: SourceId::from_uuid(Uuid::from_u128(1)),
        message_id: "tv-MNQ-1".into(),
        signal_type: "trigger".into(),
        action: "buy".into(),
        position_effect: None,
        side: None,
        instrument_symbol: "MNQ".into(),
        contract_month: None,
        quantity: Some(1),
        order,
        expiry_base: at(5_000),
    }
}

fn account() -> AccountId {
    AccountId::from_uuid(Uuid::from_u128(0x0192_b3c4_0000_7000_8000_0000_0000_0001))
}

/// 2026-10-24T14:31:00Z + `ms`.
fn at(ms: i64) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 24, 14, 31, 0).unwrap() + chrono::Duration::milliseconds(ms)
}

fn contract() -> Contract {
    Contract {
        symbol: "MNQ".into(),
        month: "2026-12".into(),
    }
}

fn decision(steps: Vec<Step>) -> Decision {
    Decision {
        outcome: Outcome::Routed,
        reason_code: None,
        requested_qty: Some(2),
        executed_qty: Some(2),
        reverse_closed_qty: None,
        steps,
        expected_flat: false,
    }
}

fn context<'a>(
    i: &'a InstructionToRoute,
    contract: &'a Contract,
    action: Action,
    leg: Leg,
    created_ms: i64,
) -> CommandContext<'a> {
    CommandContext {
        instruction: i,
        account_id: account(),
        account_name: "Sim101",
        contract,
        action,
        leg,
        created_at: at(created_ms),
        expires_at: at(10_000),
        protection: Some(Protection {
            stop_ticks: 40,
            target_ticks: 80,
        }),
    }
}

fn fixture(name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/c1")
        .join(name);
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

const SELL_2: Step = Step::Place {
    side: Side::Short,
    quantity: 2,
};
const REVERSE: Action = Action::Reverse {
    side: Side::Short,
    quantity: 1,
};

/// The fixtures shared with the gateway's consumer test (Bigorna, 2026-10-24).
#[test]
fn the_events_match_the_fixtures_frozen_with_the_gateway() {
    let c = contract();
    let market = instruction(None);
    let buy = Action::Open {
        side: Side::Long,
        quantity: 1,
    };
    let entry = commands_for(
        &context(&market, &c, buy, Leg::Whole, 6_120),
        &decision(vec![Step::Place {
            side: Side::Long,
            quantity: 2,
        }]),
    )
    .unwrap()
    .unwrap();
    assert_eq!(entry.event, fixture("entry.json"));
    assert!(entry.exit_audits.is_empty(), "an entry is not an exit");

    let limit = instruction(Some(json!({
        "type": "limit", "role": "entry", "limit_price": "21000.25", "time_in_force": "day"
    })));
    let close_leg = commands_for(
        &context(&limit, &c, REVERSE, Leg::Close, 6_120),
        &decision(vec![Step::Flatten]),
    )
    .unwrap()
    .unwrap();
    assert_eq!(close_leg.event, fixture("reverse-step1-close-leg.json"));
    let open_leg = commands_for(
        &context(&limit, &c, REVERSE, Leg::Open, 6_320),
        &decision(vec![SELL_2]),
    )
    .unwrap()
    .unwrap();
    assert_eq!(open_leg.event, fixture("reverse-step2-open-leg.json"));
    assert!(open_leg.exit_audits.is_empty());

    // The close leg's flatten is an exit: one `execution.command_requested` entry.
    assert_eq!(
        close_leg.exit_audits,
        [json!({
            "actor": "system",
            "details": {"step": "flatten", "kind": "flatten"},
            "refs": {"instruction_id": limit.id, "account_id": account(), "account_name": "Sim101",
                     "message_id": "tv-MNQ-1", "signal_type": "trigger", "is_exit": true},
        })]
    );
}

#[test]
fn a_reverse_decided_at_once_is_one_event_with_the_place_after_its_flatten() {
    let c = contract();
    let i = instruction(None);
    let both = commands_for(
        &context(&i, &c, REVERSE, Leg::Whole, 6_120),
        &decision(vec![Step::Flatten, SELL_2]),
    )
    .unwrap()
    .unwrap();
    let commands = both.event["commands"].as_array().unwrap();
    assert_eq!(commands.len(), 2);
    assert_eq!(commands[0]["body"], json!({"reason": "reverse"}));
    assert_eq!(commands[1]["depends_on"], commands[0]["command_id"]);
    assert_eq!(both.exit_audits.len(), 1);

    // A reverse on a flat position (Q75 in sync) only opens: nothing to wait for.
    let open = commands_for(
        &context(&i, &c, REVERSE, Leg::Whole, 6_120),
        &decision(vec![SELL_2]),
    )
    .unwrap()
    .unwrap();
    assert!(open.event["commands"][0].get("depends_on").is_none());
}

#[test]
fn a_close_and_the_q75_flatten_flatten_with_reason_close() {
    let c = contract();
    let i = instruction(None);
    let close = commands_for(
        &context(&i, &c, Action::Close, Leg::Whole, 6_120),
        &decision(vec![Step::Flatten]),
    )
    .unwrap()
    .unwrap();
    let command = &close.event["commands"][0];
    assert_eq!(command["body"], json!({"reason": "close"}));
    // Exits carry no expiry, no position effect and no refs.
    for absent in ["expires_at", "position_effect", "refs", "depends_on"] {
        assert!(command.get(absent).is_none(), "{absent}");
    }
    // The flat out-of-sync reverse flattens only (Q75): not a reverse's first leg.
    let mut flat = decision(vec![Step::Flatten]);
    flat.expected_flat = true;
    let q75 = commands_for(&context(&i, &c, REVERSE, Leg::Whole, 6_120), &flat)
        .unwrap()
        .unwrap();
    assert_eq!(q75.event["commands"][0]["body"], json!({"reason": "close"}));
}

#[test]
fn nothing_is_requested_without_a_step_and_ids_are_stable_per_step() {
    let c = contract();
    let i = instruction(None);
    let buy = Action::Open {
        side: Side::Long,
        quantity: 1,
    };
    assert!(
        commands_for(&context(&i, &c, buy, Leg::Whole, 0), &decision(Vec::new()))
            .unwrap()
            .is_none()
    );
    assert_eq!(
        command_id(i.id, account(), "place"),
        command_id(i.id, account(), "place")
    );
    assert_ne!(
        command_id(i.id, account(), "place"),
        command_id(i.id, account(), "flatten")
    );
    assert_eq!(command_id(i.id, account(), "place").get_version_num(), 5);
}

/// The protocol schema refers to the instruction schema by its `$id`: read it from the repo.
struct LocalContracts;

impl jsonschema::Retrieve for LocalContracts {
    fn retrieve(
        &self,
        uri: &jsonschema::Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let path = uri
            .as_str()
            .strip_prefix("https://allwin.app/contracts/")
            .ok_or_else(|| format!("not a local contract: {}", uri.as_str()))?;
        let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../contracts")
            .join(path);
        Ok(serde_json::from_str(&std::fs::read_to_string(file)?)?)
    }
}

/// Every command the fixtures hold is a valid adapter-protocol `command` once the gateway adds
/// `platform_account` and puts the body under its kind (gateway `command_payload`).
#[test]
fn the_fixture_commands_are_valid_protocol_commands() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../contracts/adapter-protocol/v1/messages.schema.json");
    let protocol: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$ref": "#/$defs/command",
        "$defs": protocol["$defs"],
    });
    let validator = jsonschema::options()
        .with_retriever(LocalContracts)
        .build(&schema)
        .unwrap();
    for name in [
        "entry.json",
        "reverse-step1-close-leg.json",
        "reverse-step2-open-leg.json",
    ] {
        let event = fixture(name);
        for c in event["commands"].as_array().unwrap() {
            let kind = c["kind"].as_str().unwrap();
            let mut payload = json!({
                "command_id": c["command_id"],
                "instruction_id": event["instruction_id"],
                "account_id": event["account_id"],
                "platform_account": "Sim101",
                "instrument": c["instrument"],
                "kind": kind,
            });
            for optional in ["expires_at", "depends_on", "refs"] {
                if let Some(v) = c.get(optional) {
                    payload[optional] = v.clone();
                }
            }
            payload[kind] = c["body"].clone();
            let errors: Vec<String> = validator
                .iter_errors(&payload)
                .map(|e| e.to_string())
                .collect();
            assert!(errors.is_empty(), "{name}: {errors:?}");
            // The gateway refuses an event whose two position effects differ.
            if kind == "place" {
                assert_eq!(c["position_effect"], c["body"]["position_effect"], "{name}");
            }
        }
    }
}

/// `execution.command_requested` details are the contract's `DetailsCommandSent` (HTTP 5.2.0).
#[test]
fn exit_audits_match_the_contract() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../contracts/http/openapi.bundled.yaml");
    let bundle: Value = yaml_serde::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let reference = bundle["components"]["schemas"]["AuditDetailsByCode"]["properties"]
        [COMMAND_REQUESTED]["$ref"]
        .as_str()
        .unwrap()
        .to_owned();
    let schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$ref": reference,
        "components": bundle["components"],
    });
    let validator = jsonschema::validator_for(&schema).unwrap();
    let c = contract();
    let i = instruction(None);
    let close = commands_for(
        &context(&i, &c, Action::Close, Leg::Whole, 0),
        &decision(vec![Step::Flatten]),
    )
    .unwrap()
    .unwrap();
    for audit in &close.exit_audits {
        let errors: Vec<String> = validator
            .iter_errors(&audit["details"])
            .map(|e| e.to_string())
            .collect();
        assert!(errors.is_empty(), "{errors:?}");
    }
}

/// Esmeril C1 note (a): a place without protection fails the decision, never drops silently.
#[test]
fn a_place_without_protection_fails_loudly() {
    let c = contract();
    let i = instruction(None);
    let buy = Action::Open {
        side: Side::Long,
        quantity: 1,
    };
    let mut ctx = context(&i, &c, buy, Leg::Whole, 0);
    ctx.protection = None;
    let place = Step::Place {
        side: Side::Long,
        quantity: 2,
    };
    assert_eq!(
        commands_for(&ctx, &decision(vec![place])),
        Err(MissingProtection(account()))
    );
    // An exit never needs a profile.
    let close = context(&i, &c, Action::Close, Leg::Whole, 0);
    let close = CommandContext {
        protection: None,
        ..close
    };
    assert!(
        commands_for(&close, &decision(vec![Step::Flatten]))
            .unwrap()
            .is_some()
    );
}
