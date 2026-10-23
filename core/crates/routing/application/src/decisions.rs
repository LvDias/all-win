//! Per-instruction routing decision (S7 AC-2…AC-5, AC-8; ADR-0003 §4; HTTP 5.1.0 Q74). For each
//! enabled route of the instruction's source, the facts are read once through the ports, the
//! pure engine (`routing_enterprise::decision`) decides, and every decision is stored with its
//! snapshots and its audit event in one transaction (`routing.routing_decision`).
//!
//! Provisional (Maestro, 2026-10-23): no command is emitted until the gateway consumes routing
//! decisions, and the use case is wired once the owner of the expected position exists
//! (`PositionFor`, gateway M1). Notifications are derived from the audit entries by S18
//! (s18 §3), so the event carries everything `routing.reverse_close_only` needs.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use routing_enterprise::decision::{self, Action, Decision, DecisionFacts, Outcome, Side};
use routing_enterprise::{AccountId, InstructionId, RouteSignalType, SourceId, Uuid};
use serde_json::{Map, Value, json};
use shared_kernel::{Clock, UserId};

use crate::routes::{
    AccountDirectory, AccountFacts, PortError, RouteFilter, RouteStore, SourceDirectory,
    SourceFacts,
};

// ---------------------------------------------------------------------------------------------
// Ports
// ---------------------------------------------------------------------------------------------

/// The Core's expected position of one account in one instrument (gateway).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedPosition {
    /// Signed quantity: positive long, negative short, 0 flat.
    pub quantity: i64,
    /// The All Win identity of the open position (U13; ADR-0004 §5), when one is held.
    pub position_ref: Option<String>,
}

/// Query port owned by the gateway, read once per (instruction, account) at the decision.
#[async_trait]
pub trait PositionFor: Send + Sync {
    async fn position(
        &self,
        user: UserId,
        account: AccountId,
        instrument_symbol: &str,
    ) -> Result<ExpectedPosition, PortError>;
}

/// One entry-only gate of the decision (ADR-0003 §4.1 gates 6–9), answered per account:
/// `true` = barred for entries. Required: every gate must answer for every account, or the
/// instruction's decision fails and is retried (fail closed; Architect, 2026-10-23). Until its
/// owning module exists, `crates/app` wires a stand-in that answers `Err` outside release
/// builds only.
#[async_trait]
pub trait EntryGate: Send + Sync {
    async fn barred(
        &self,
        user: UserId,
        instrument_symbol: &str,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, bool>, PortError>;
}

/// The four entry gates, in ADR-0003 §4.1 order.
#[derive(Clone)]
pub struct EntryGatePorts {
    /// 6: the account is flagged out of sync (S11, gateway).
    pub out_of_sync: Arc<dyn EntryGate>,
    /// 7: risk block (S13/S14).
    pub risk_blocked: Arc<dyn EntryGate>,
    /// 8: limits not yet derived (ADR-0006 §3.3).
    pub limits_pending: Arc<dyn EntryGate>,
    /// 9: no stop/target profile for the instrument (S6, Q43); the app adapter resolves the
    /// symbol through the catalog.
    pub no_bracket_profile: Arc<dyn EntryGate>,
}

/// Gates 6–9 for the accounts, in ADR-0003 §4.1 order.
async fn entry_gates(
    g: &EntryGatePorts,
    user: UserId,
    symbol: &str,
    accounts: &[AccountId],
) -> Result<[HashMap<AccountId, bool>; 4], PortError> {
    Ok([
        ask(&*g.out_of_sync, "out_of_sync", user, symbol, accounts).await?,
        ask(&*g.risk_blocked, "risk_blocked", user, symbol, accounts).await?,
        ask(&*g.limits_pending, "limits_pending", user, symbol, accounts).await?,
        ask(
            &*g.no_bracket_profile,
            "no_bracket_profile",
            user,
            symbol,
            accounts,
        )
        .await?,
    ])
}

/// Every account's answer from one gate; a missing answer is an error, never "not barred".
async fn ask(
    gate: &dyn EntryGate,
    name: &str,
    user: UserId,
    symbol: &str,
    accounts: &[AccountId],
) -> Result<HashMap<AccountId, bool>, PortError> {
    let answers = gate.barred(user, symbol, accounts).await?;
    if accounts.iter().all(|a| answers.contains_key(a)) {
        Ok(answers)
    } else {
        Err(PortError(format!(
            "gate {name} did not answer for every account"
        )))
    }
}

/// A stored instruction, as the decision needs it (`routing.instruction`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionToRoute {
    pub id: InstructionId,
    pub source_id: SourceId,
    pub message_id: String,
    pub signal_type: String,
    pub action: String,
    pub position_effect: Option<String>,
    pub side: Option<String>,
    pub instrument_symbol: String,
    pub quantity: Option<i32>,
    /// `min(sent_at, received_at)` (ADR-0003 §7).
    pub expiry_base: DateTime<Utc>,
}

/// One decision with its snapshots (`routing.routing_decision`) and its audit event.
#[derive(Debug, Clone, PartialEq)]
pub struct DecisionRecord {
    pub id: Uuid,
    pub user_id: UserId,
    pub instruction_id: InstructionId,
    pub account_id: AccountId,
    pub account_name: String,
    pub outcome: Outcome,
    pub reason_code: Option<&'static str>,
    pub multiplier: u32,
    pub max_position_qty: u32,
    pub position_before: i64,
    pub requested_qty: Option<u32>,
    pub executed_qty: Option<u32>,
    pub decided_at: DateTime<Utc>,
    /// The audit code (`reason_code`, or `routing.routed`).
    pub event_type: &'static str,
    /// The audit envelope `{actor, details, refs}`.
    pub payload: Value,
}

#[async_trait]
pub trait DecisionStore: Send + Sync {
    /// The caller's instruction; `None` for an unknown id or another user's.
    async fn instruction(
        &self,
        user: UserId,
        id: InstructionId,
    ) -> Result<Option<InstructionToRoute>, PortError>;

    /// In one user-scoped transaction, inserts each decision once per (instruction, account)
    /// and appends its audit event only when the row is new, so a redelivery never audits
    /// twice. Returns how many were new.
    async fn record(&self, user: UserId, decisions: &[DecisionRecord]) -> Result<usize, PortError>;
}

// ---------------------------------------------------------------------------------------------
// Use case
// ---------------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum DecideError {
    #[error("instruction not found")]
    NotFound,
    /// The stored instruction is not a V1 routable action; it never reaches the engine.
    #[error("instruction action is not routable")]
    NotRoutable,
    #[error(transparent)]
    Port(#[from] PortError),
}

#[derive(Clone)]
pub struct DecisionDeps {
    pub store: Arc<dyn DecisionStore>,
    pub routes: Arc<dyn RouteStore>,
    pub accounts: Arc<dyn AccountDirectory>,
    pub sources: Arc<dyn SourceDirectory>,
    pub positions: Arc<dyn PositionFor>,
    pub gates: EntryGatePorts,
    pub clock: Arc<dyn Clock>,
}

/// Decides one instruction for every enabled route of its source.
pub struct DecideInstruction {
    deps: DecisionDeps,
}

impl DecideInstruction {
    pub fn new(deps: DecisionDeps) -> Self {
        Self { deps }
    }

    /// Returns the decisions taken; a redelivery decides again but stores and audits nothing
    /// new (the first decision per account stands).
    pub async fn handle(
        &self,
        user: UserId,
        instruction_id: InstructionId,
    ) -> Result<Vec<DecisionRecord>, DecideError> {
        let d = &self.deps;
        let i = d
            .store
            .instruction(user, instruction_id)
            .await?
            .ok_or(DecideError::NotFound)?;
        // `master_account` follower subscriptions belong to S19.
        let Some(signal_type) = RouteSignalType::parse(&i.signal_type) else {
            return Ok(Vec::new());
        };
        let action = Action::from_instruction(
            &i.action,
            i.position_effect.as_deref(),
            i.side.as_deref(),
            i.quantity,
        )
        .ok_or(DecideError::NotRoutable)?;
        // A revoked source never passes ingress; an unknown one has nothing to route.
        let Some(source) = d.sources.source(user, i.source_id).await? else {
            return Ok(Vec::new());
        };

        let routes: Vec<_> = d
            .routes
            .list(
                user,
                RouteFilter {
                    source_id: Some(i.source_id),
                    account_id: None,
                },
            )
            .await?
            .into_iter()
            .filter(|r| r.is_enabled)
            .collect();
        let accounts: HashMap<AccountId, AccountFacts> = d
            .accounts
            .accounts(user)
            .await?
            .into_iter()
            .map(|a| (a.id, a))
            .collect();
        // An archived account has no routes left; a route without its account is skipped.
        let targets: Vec<(&AccountFacts, _)> = routes
            .iter()
            .filter_map(|r| accounts.get(&r.account_id).map(|a| (a, &r.signal_types)))
            .collect();
        if targets.is_empty() {
            return Ok(Vec::new());
        }
        let ids: Vec<AccountId> = targets.iter().map(|(a, _)| a.id).collect();
        // Any error from here on returns before the store: nothing of this instruction is
        // committed, and the outbox delivery retries it. An exit never consults the entry
        // gates (P1), so their outage never delays it; a `reverse` needs their answer to choose
        // between close-only and flatten + open, so it waits whole.
        let gates = if action.increases_position() {
            Some(entry_gates(&d.gates, user, &i.instrument_symbol, &ids).await?)
        } else {
            None
        };
        // Unread (exit) or unanswered: barred.
        let barred = |n: usize, a: &AccountId| {
            gates
                .as_ref()
                .and_then(|g| g[n].get(a).copied())
                .unwrap_or(true)
        };

        let now = d.clock.now();
        let mut out = Vec::with_capacity(targets.len());
        for (account, signal_types) in targets {
            let position = d
                .positions
                .position(user, account.id, &i.instrument_symbol)
                .await?;
            let ttl_ms = account.sizing.command_ttl_ms;
            let expires_at = i.expiry_base + Duration::milliseconds(i64::from(ttl_ms));
            let facts = DecisionFacts {
                account_active: account.lifecycle.is_none(),
                platform_bound: account.platform_bound,
                automation_not_accepted: source.automated
                    && account.automation.not_accepted().is_some(),
                signal_types: signal_types.clone(),
                out_of_sync: barred(0, &account.id),
                risk_blocked: barred(1, &account.id),
                expired: now >= expires_at,
                limits_pending: barred(2, &account.id),
                has_bracket_profile: !barred(3, &account.id),
                multiplier: account.sizing.multiplier,
                max_position: account.sizing.max_position_qty,
                position: position.quantity,
            };
            let decision = decision::decide(signal_type, action, &facts);
            out.push(record(
                user, &i, &source, account, action, &facts, &decision, now,
            ));
        }
        d.store.record(user, &out).await?;
        Ok(out)
    }
}

/// The decision's row and audit event (`DetailsRoutingDecision`, HTTP 5.1.0; s10 §5: actor
/// `system`). `refs.outcome` follows the Architect's per-code table (decision `capped` is
/// audited `partial`).
#[allow(clippy::too_many_arguments)]
fn record(
    user: UserId,
    i: &InstructionToRoute,
    source: &SourceFacts,
    account: &AccountFacts,
    action: Action,
    f: &DecisionFacts,
    d: &Decision,
    now: DateTime<Utc>,
) -> DecisionRecord {
    let code = d.reason_code.unwrap_or("routing.routed");
    let outcome = match d.outcome {
        Outcome::Routed => "routed",
        Outcome::Capped => "partial",
        Outcome::Skipped => "skipped",
        Outcome::Refused => "refused",
        Outcome::Expired => "expired",
    };

    let mut sizing = Map::new();
    if let Action::Open { quantity, .. } | Action::Reverse { quantity, .. } = action {
        sizing.insert("instruction_qty".into(), json!(quantity));
    }
    sizing.insert("multiplier".into(), json!(f.multiplier));
    sizing.insert("max_position_qty".into(), json!(f.max_position));
    sizing.insert("position_before".into(), json!(f.position));
    if let Some(q) = d.requested_qty {
        sizing.insert("requested_qty".into(), json!(q));
    }
    if let Some(q) = d.executed_qty {
        sizing.insert("executed_qty".into(), json!(q));
    }
    if let (Some(r), Some(e)) = (d.requested_qty, d.executed_qty) {
        sizing.insert("skipped_qty".into(), json!(r.saturating_sub(e)));
    }

    let mut details = Map::new();
    details.insert("sizing".into(), Value::Object(sizing));
    details.insert("position_side".into(), json!(Side::of_position(f.position)));
    match code {
        "routing.account_inactive" => {
            if let Some(state) = account.lifecycle {
                details.insert("lifecycle_state".into(), json!(state.as_str()));
            }
        }
        "routing.automation_not_accepted" => {
            if let Some(n) = account.automation.not_accepted() {
                details.insert("opt_in_state".into(), json!(n.as_str()));
            }
        }
        "routing.expired" => {
            details.insert("ttl_ms".into(), json!(account.sizing.command_ttl_ms));
            details.insert(
                "expiry_base".into(),
                json!(
                    i.expiry_base
                        .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
                ),
            );
        }
        _ => {}
    }
    if let Some(q) = d.reverse_closed_qty {
        details.insert("reverse_closed_qty".into(), json!(q));
    }

    let payload = json!({
        "actor": "system",
        "details": details,
        "refs": {
            "outcome": outcome,
            "instruction_id": i.id,
            "source_id": i.source_id,
            "source_name": source.name,
            "account_id": account.id,
            "account_name": account.display_name,
            "message_id": i.message_id,
            "signal_type": i.signal_type,
            "is_exit": !action.increases_position(),
        },
    });
    DecisionRecord {
        id: Uuid::now_v7(),
        user_id: user,
        instruction_id: i.id,
        account_id: account.id,
        account_name: account.display_name.clone(),
        outcome: d.outcome,
        reason_code: d.reason_code,
        multiplier: f.multiplier,
        max_position_qty: f.max_position,
        position_before: f.position,
        requested_qty: d.requested_qty,
        executed_qty: d.executed_qty,
        decided_at: now,
        event_type: code,
        payload,
    }
}

#[cfg(test)]
#[path = "decisions_tests.rs"]
mod tests;
