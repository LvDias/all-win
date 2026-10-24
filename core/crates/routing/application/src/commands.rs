//! The gateway commands of a decision (C1 `routing.commands_requested`, frozen with the gateway
//! on 2026-10-24; adapter protocol v1 `command.place` / `command.flatten`). The event goes to
//! the outbox in the decision's transaction, with one `execution.command_requested` audit entry
//! per exit command (HTTP 5.2.0 `DetailsCommandSent`), so a waiting exit is visible even while
//! the gateway or the connection is down.
//!
//! One event per (instruction, account, leg) that has commands: an entry is one `place`; a
//! `close` (and the Q75 flatten) is one `flatten`; a `reverse` against an opposite position is
//! a `flatten` then a `place` that depends on it, in one event when decided at once or in the
//! close leg's and the open leg's events. V1 never emits a `reduce` place, so `refs`
//! (`position_ref`) never goes.

use chrono::{DateTime, SecondsFormat, Utc};
use routing_enterprise::decision::{Action, Decision, Side, Step};
use routing_enterprise::{AccountId, InstructionId, Uuid};
use serde_json::{Map, Value, json};

use crate::decisions::{Contract, InstructionToRoute, Leg};

pub const COMMANDS_REQUESTED: &str = "routing.commands_requested";
pub const COMMAND_REQUESTED: &str = "execution.command_requested";

/// Namespace of the routing command ids (UUIDv5); never changes.
const NAMESPACE: Uuid = Uuid::from_u128(0x1f64_8659_d29b_43bb_9bc3_dc00_2593_a8bf);

/// `UUIDv5(instruction_id ‖ account_id ‖ step)`: a redelivery gets the same id, and the
/// gateway's `ON CONFLICT (id) DO NOTHING` absorbs it.
pub fn command_id(instruction: InstructionId, account: AccountId, step: &str) -> Uuid {
    let mut name = Vec::with_capacity(32 + step.len());
    name.extend_from_slice(instruction.as_uuid().as_bytes());
    name.extend_from_slice(account.as_uuid().as_bytes());
    name.extend_from_slice(step.as_bytes());
    Uuid::new_v5(&NAMESPACE, &name)
}

/// The account's stop/target profile for the contract, in whole ticks (S6 `BracketQuery`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Protection {
    pub stop_ticks: u32,
    pub target_ticks: u32,
}

/// What one decision leg asks the gateway to do: the event payload and the audit envelopes of
/// its exit commands.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandsRequested {
    pub event: Value,
    pub exit_audits: Vec<Value>,
}

/// Everything besides the decision that the commands carry.
#[derive(Debug, Clone, Copy)]
pub struct CommandContext<'a> {
    pub instruction: &'a InstructionToRoute,
    pub account_id: AccountId,
    pub account_name: &'a str,
    pub contract: &'a Contract,
    pub action: Action,
    pub leg: Leg,
    /// The decision instant: the envelope `sent_at`, unchanged on retransmission.
    pub created_at: DateTime<Utc>,
    /// `expiry_base + command_ttl_ms`; only an opening place carries it.
    pub expires_at: DateTime<Utc>,
    pub protection: Option<Protection>,
}

/// A place without a stop/target profile: the engine skips such an entry with
/// `routing.no_bracket_profile` first, so reaching it is a defect. The decision is not
/// committed and the delivery retries, then dead-letters with an alert (Esmeril, C1 note a).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("a place for account {0} has no stop/target profile; decision not committed")]
pub struct MissingProtection(pub AccountId);

/// `Ok(None)` when the decision places nothing.
pub fn commands_for(
    c: &CommandContext<'_>,
    d: &Decision,
) -> Result<Option<CommandsRequested>, MissingProtection> {
    let flatten_id = command_id(c.instruction.id, c.account_id, "flatten");
    // The opening of a reverse waits for its flatten, whether it is in this event or was
    // committed by the close leg.
    let depends_on = (matches!(c.action, Action::Reverse { .. })
        && (c.leg == Leg::Open || d.steps.contains(&Step::Flatten)))
    .then_some(flatten_id);
    let mut commands = Vec::with_capacity(d.steps.len());
    let mut exit_audits = Vec::new();
    for step in &d.steps {
        match *step {
            Step::Flatten => {
                let reason = match c.action {
                    Action::Reverse { .. } if !d.expected_flat => "reverse",
                    _ => "close",
                };
                commands.push(json!({
                    "command_id": flatten_id,
                    "kind": "flatten",
                    "step": "flatten",
                    "instrument": instrument(c.contract),
                    "body": {"reason": reason},
                }));
                exit_audits.push(json!({
                    "actor": "system",
                    "details": {"step": "flatten", "kind": "flatten"},
                    "refs": {
                        "instruction_id": c.instruction.id,
                        "account_id": c.account_id,
                        "account_name": c.account_name,
                        "message_id": c.instruction.message_id,
                        "signal_type": c.instruction.signal_type,
                        "is_exit": true,
                    },
                }));
            }
            Step::Place { side, quantity } => {
                let protection = c.protection.ok_or(MissingProtection(c.account_id))?;
                let mut command = Map::new();
                command.insert(
                    "command_id".into(),
                    json!(command_id(c.instruction.id, c.account_id, "place")),
                );
                command.insert("kind".into(), json!("place"));
                command.insert("step".into(), json!("place"));
                if let Some(d) = depends_on {
                    command.insert("depends_on".into(), json!(d));
                }
                command.insert("expires_at".into(), json!(timestamp(c.expires_at)));
                command.insert("position_effect".into(), json!("open"));
                command.insert("instrument".into(), instrument(c.contract));
                command.insert(
                    "body".into(),
                    json!({
                        "side": match side {
                            Side::Long => "buy",
                            Side::Short => "sell",
                        },
                        "quantity": quantity,
                        "order": order_spec(c.instruction.order.as_ref()),
                        "position_effect": "open",
                        "protection": {
                            "mode": "individual",
                            "stop_ticks": protection.stop_ticks,
                            "target_ticks": protection.target_ticks,
                        },
                    }),
                );
                commands.push(Value::Object(command));
            }
        }
    }
    if commands.is_empty() {
        return Ok(None);
    }
    Ok(Some(CommandsRequested {
        event: json!({
            "instruction_id": c.instruction.id,
            "account_id": c.account_id,
            "created_at": timestamp(c.created_at),
            "commands": commands,
        }),
        exit_audits,
    }))
}

fn instrument(c: &Contract) -> Value {
    json!({"symbol": c.symbol, "contract_month": c.month})
}

/// The instruction's `order` without its `role` (prices stay strings); omitted = market.
fn order_spec(order: Option<&Value>) -> Value {
    let mut spec = match order {
        Some(Value::Object(o)) => o.clone(),
        _ => Map::new(),
    };
    spec.remove("role");
    spec.entry("type").or_insert_with(|| json!("market"));
    Value::Object(spec)
}

fn timestamp(t: DateTime<Utc>) -> String {
    t.to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[cfg(test)]
#[path = "commands_tests.rs"]
mod tests;
