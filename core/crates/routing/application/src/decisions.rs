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

use crate::commands::{CommandContext, CommandsRequested, Protection, commands_for};
use crate::routes::{
    AccountDirectory, AccountFacts, PortError, RouteFilter, RouteStore, SourceDirectory,
    SourceFacts,
};

// ---------------------------------------------------------------------------------------------
// Ports
// ---------------------------------------------------------------------------------------------

/// The contract a decision is about, resolved once per instruction (Esquadro D2): the
/// instruction's month, or else the rollover catalog's active contract (P5). Every port and
/// command uses it, as the gateway's key (data model 16.6 G3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    pub symbol: String,
    /// `YYYY-MM`.
    pub month: String,
}

/// The rollover catalog (P5): the active contract month of a symbol at an instant; `None`
/// when the instrument has no active contract then. The adapter lives in `crates/app`.
#[async_trait]
pub trait ActiveContract: Send + Sync {
    async fn active(&self, symbol: &str, at: DateTime<Utc>) -> Result<Option<String>, PortError>;
}

/// The Core's expected position of one account in one contract (gateway).
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
        contract: &Contract,
    ) -> Result<ExpectedPosition, PortError>;
}

/// The kind of the platform account behind a routed account, from its latest handshake
/// (gateway `venue_account.kind`). No binding or no venue account reads `Unknown`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountKind {
    Simulation,
    Playback,
    Live,
    Unknown,
}

/// Query port owned by the gateway, read at each decision that needs it, never cached
/// (Vigia C2). An error never reads as a simulation account.
#[async_trait]
pub trait AccountKindOf: Send + Sync {
    async fn kind(&self, user: UserId, account: AccountId) -> Result<AccountKind, PortError>;
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
        contract: &Contract,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, bool>, PortError>;

    /// The marker of a test-build stand-in ([`crate::dev_stubs`]); `None` for a real gate.
    fn stand_in(&self) -> Option<&'static str> {
        None
    }
}

/// Gate 9 and the commands' protection (S6 `BracketQuery`, adapter in `crates/app`): each
/// account's stop/target profile for the contract in whole ticks. An absent account has no
/// usable profile, and its entries are skipped with `routing.no_bracket_profile`.
#[async_trait]
pub trait BracketFor: Send + Sync {
    async fn brackets(
        &self,
        user: UserId,
        contract: &Contract,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, Protection>, PortError>;

    /// The marker of a test-build fixed profile; `None` for the S6 query.
    fn stand_in(&self) -> Option<&'static str> {
        None
    }
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
    /// symbol through the catalog. Its answer is also the commands' protection.
    pub no_bracket_profile: Arc<dyn BracketFor>,
}

impl EntryGatePorts {
    /// The gates answered by a test-build stand-in, by name: the app's startup warning and
    /// its release checks (Vigia C3/C5).
    pub fn stand_ins(&self) -> Vec<&'static str> {
        [
            ("out_of_sync", self.out_of_sync.stand_in()),
            ("risk_blocked", self.risk_blocked.stand_in()),
            ("limits_pending", self.limits_pending.stand_in()),
            ("no_bracket_profile", self.no_bracket_profile.stand_in()),
        ]
        .into_iter()
        .filter(|(_, marker)| marker.is_some())
        .map(|(name, _)| name)
        .collect()
    }
}

/// The answers of gates 6–9 for the accounts, in ADR-0003 §4.1 order, and the profiles that
/// gate 9 found (the commands' protection).
struct GateAnswers {
    barred: [HashMap<AccountId, bool>; 4],
    protection: HashMap<AccountId, Protection>,
}

/// Gates 6–9 for the accounts, in ADR-0003 §4.1 order.
async fn entry_gates(
    g: &EntryGatePorts,
    user: UserId,
    contract: &Contract,
    accounts: &[AccountId],
) -> Result<GateAnswers, PortError> {
    let out_of_sync = ask(&*g.out_of_sync, "out_of_sync", user, contract, accounts).await?;
    let risk_blocked = ask(&*g.risk_blocked, "risk_blocked", user, contract, accounts).await?;
    let limits_pending = ask(
        &*g.limits_pending,
        "limits_pending",
        user,
        contract,
        accounts,
    )
    .await?;
    // An account without a usable profile is absent: that is its answer (barred).
    let protection = g
        .no_bracket_profile
        .brackets(user, contract, accounts)
        .await?;
    let no_bracket = accounts
        .iter()
        .map(|a| (*a, !protection.contains_key(a)))
        .collect();
    Ok(GateAnswers {
        barred: [out_of_sync, risk_blocked, limits_pending, no_bracket],
        protection,
    })
}

/// Every account's answer from one gate; a missing answer is an error, never "not barred".
async fn ask(
    gate: &dyn EntryGate,
    name: &str,
    user: UserId,
    contract: &Contract,
    accounts: &[AccountId],
) -> Result<HashMap<AccountId, bool>, PortError> {
    let answers = gate.barred(user, contract, accounts).await?;
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
    /// `YYYY-MM`, when the instruction names its contract.
    pub contract_month: Option<String>,
    pub quantity: Option<i32>,
    /// The instruction's `order` object; `None` = market.
    pub order: Option<Value>,
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
    /// `None` when the decision never read a position (`routing.instrument_unavailable`).
    pub position_before: Option<i64>,
    pub requested_qty: Option<u32>,
    pub executed_qty: Option<u32>,
    pub decided_at: DateTime<Utc>,
    pub leg: Leg,
    /// Decided after the TTL without answers from gates 6–9 (Architect, N1; details
    /// `gates_unknown`).
    pub gates_unknown: bool,
    /// Q75: a flatten sent against a flat expected position on an out-of-sync account (details
    /// `expected_flat`).
    pub expected_flat: bool,
    /// The audit code (`reason_code`, or `routing.routed`) and envelope `{actor, details,
    /// refs}`. A `close` leg has none: the pair gets its one decision entry with the opening
    /// (Compasso, 2026-10-23).
    pub audit: Option<(&'static str, Value)>,
    /// C1: the leg's gateway commands (`routing.commands_requested`) and the
    /// `execution.command_requested` entries of its exit commands, appended with the row.
    pub commands: Option<CommandsRequested>,
}

/// Which part of a decision a row holds (`routing_decision.leg`; ADR-0003 §4.1 r4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Leg {
    /// The whole decision, in one step.
    Whole,
    /// Step 1 of a `reverse` against an opposite position: the flatten, decided without the
    /// entry gates.
    Close,
    /// Step 2: the opening, decided when the entry gates answer or the TTL passes.
    Open,
}

impl Leg {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Whole => "whole",
            Self::Close => "close",
            Self::Open => "open",
        }
    }
}

/// A row already stored for the instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecidedLeg {
    pub account_id: AccountId,
    pub leg: Leg,
    pub position_before: i64,
}

#[async_trait]
pub trait DecisionStore: Send + Sync {
    /// The caller's instruction; `None` for an unknown id or another user's.
    async fn instruction(
        &self,
        user: UserId,
        id: InstructionId,
    ) -> Result<Option<InstructionToRoute>, PortError>;

    /// The rows already stored for the instruction.
    async fn decided(
        &self,
        user: UserId,
        instruction: InstructionId,
    ) -> Result<Vec<DecidedLeg>, PortError>;

    /// In one user-scoped transaction, inserts each row once (`whole` or `close`, then at
    /// most one `open`, and an `open` only after a `close` of the same pair) and appends its
    /// audit event only when the row is new, so a redelivery never audits twice. Returns how
    /// many were new.
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
    /// Some accounts wait for a gate that did not answer; what could be decided (exits, close
    /// legs) is committed, and the delivery is retried. The text names the instruction, each
    /// waiting account and why, so the dead-letter alert says what is stuck (Esmeril, 5.2.0),
    /// and the contract, so a stalled-exit alert names the instrument (Architect, L4).
    #[error(
        "instruction {instruction} {} {} waiting ({cause}): {}",
        .contract.symbol,
        .contract.month,
        waiting_list(.accounts)
    )]
    GatesUnavailable {
        instruction: InstructionId,
        contract: Contract,
        accounts: Vec<(AccountId, Waiting)>,
        cause: PortError,
    },
}

impl DecideError {
    /// The exits still waiting (a close against a flat expected position, without gate 6).
    pub fn waiting_exits(&self) -> Vec<AccountId> {
        match self {
            Self::GatesUnavailable { accounts, .. } => accounts
                .iter()
                .filter(|(_, w)| *w == Waiting::CloseForSyncState)
                .map(|(a, _)| *a)
                .collect(),
            _ => Vec::new(),
        }
    }
}

/// Retry ceiling of the decision consumer while an exit waits (Architect, L4): exits are
/// retried without an attempt limit, never dead-lettered by count.
pub const EXIT_RETRY_CAP: Duration = Duration::seconds(30);
const EXIT_ALERT_AFTER: Duration = Duration::seconds(60);
const EXIT_CRITICAL_AFTER: Duration = Duration::minutes(5);
const EXIT_CRITICAL_EVERY: Duration = Duration::minutes(30);

/// The operator alert of a waiting exit (Architect, L4): a warning at 60 s, then a critical
/// alert at 5 min repeated every 30 min. The user-facing codes come with HTTP 6.3.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitStallAlert {
    Waiting,
    Stalled,
}

/// Edge-triggered (Esmeril, WIP #13 M1): the alert of the highest threshold crossed in
/// `(previous, stalled]`, the times since the instruction was received at the previous attempt
/// and at this one, so each threshold alerts exactly once whatever the retry cadence; a gap
/// that skips several 30-min periods alerts once. `previous = None` (first attempt seen by
/// this process, e.g. after a restart) counts every threshold up to `stalled`, so an alert may
/// repeat after a restart but is never missed.
pub fn exit_stall_alert(previous: Option<Duration>, stalled: Duration) -> Option<ExitStallAlert> {
    let crossed = |t: Duration| previous.is_none_or(|p| p < t) && t <= stalled;
    if stalled >= EXIT_CRITICAL_AFTER {
        // The latest critical threshold at or before `stalled`.
        let periods = (stalled - EXIT_CRITICAL_AFTER).num_milliseconds()
            / EXIT_CRITICAL_EVERY.num_milliseconds();
        let latest = EXIT_CRITICAL_AFTER
            + Duration::milliseconds(periods * EXIT_CRITICAL_EVERY.num_milliseconds());
        if crossed(latest) {
            return Some(ExitStallAlert::Stalled);
        }
    }
    crossed(EXIT_ALERT_AFTER).then_some(ExitStallAlert::Waiting)
}

/// Why one account of an instruction is not decided yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waiting {
    /// A close against a flat expected position needs gate 6 (Q75); exits never expire.
    CloseForSyncState,
    /// An entry (or the opening of a reverse) needs gates 6–9; it expires by its TTL.
    EntryForGates,
}

impl Waiting {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CloseForSyncState => "close waiting for sync state",
            Self::EntryForGates => "entry waiting for gates 6-9",
        }
    }
}

fn waiting_list(accounts: &[(AccountId, Waiting)]) -> String {
    accounts
        .iter()
        .map(|(a, w)| format!("account {} {}", a.as_uuid(), w.as_str()))
        .collect::<Vec<_>>()
        .join("; ")
}

#[derive(Clone)]
pub struct DecisionDeps {
    pub store: Arc<dyn DecisionStore>,
    pub routes: Arc<dyn RouteStore>,
    pub accounts: Arc<dyn AccountDirectory>,
    pub sources: Arc<dyn SourceDirectory>,
    pub positions: Arc<dyn PositionFor>,
    pub contracts: Arc<dyn ActiveContract>,
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
        // What a redelivery already decided stands; a close leg keeps the position it read.
        let decided = d.store.decided(user, instruction_id).await?;
        let done =
            |a: AccountId, leg: Leg| decided.iter().any(|x| x.account_id == a && x.leg == leg);
        let targets: Vec<_> = targets
            .into_iter()
            .filter(|(a, _)| !done(a.id, Leg::Whole) && !done(a.id, Leg::Open))
            .collect();
        if targets.is_empty() {
            return Ok(Vec::new());
        }

        // One contract for the whole decision (D2). The instant is the instruction's expiry
        // base, not the retry's clock, so every attempt and both legs of a reverse resolve the
        // same contract across a roll.
        let month = match &i.contract_month {
            Some(m) => Some(m.clone()),
            None => {
                d.contracts
                    .active(&i.instrument_symbol, i.expiry_base)
                    .await?
            }
        };
        // In the catalog but no active contract now: skipped before any gate, for every
        // account, without reading a position (HTTP 5.2.0 `routing.instrument_unavailable`).
        let Some(month) = month else {
            let now = d.clock.now();
            let out: Vec<DecisionRecord> = targets
                .iter()
                .map(|(account, signal_types)| {
                    let facts = DecisionFacts {
                        account_active: account.lifecycle.is_none(),
                        platform_bound: account.platform_bound,
                        automation_not_accepted: source.automated
                            && account.automation.not_accepted().is_some(),
                        signal_types: (*signal_types).clone(),
                        out_of_sync: true,
                        risk_blocked: true,
                        expired: false,
                        limits_pending: true,
                        has_bracket_profile: false,
                        multiplier: account.sizing.multiplier,
                        max_position: account.sizing.max_position_qty,
                        position: 0,
                    };
                    let decision =
                        decision::instrument_unavailable(action, account.sizing.multiplier);
                    record(
                        user,
                        &i,
                        &source,
                        account,
                        action,
                        &facts,
                        &decision,
                        now,
                        Leg::Whole,
                        Known {
                            month: None,
                            position: None,
                            gates_unknown: false,
                        },
                        None,
                    )
                })
                .collect();
            d.store.record(user, &out).await?;
            return Ok(out);
        };
        let contract = Contract {
            symbol: i.instrument_symbol.clone(),
            month,
        };

        // The expected position decides what is an exit, so the whole instruction waits for
        // it (no partial commit); a close leg already stored keeps its own.
        let mut positions = HashMap::new();
        for (account, _) in &targets {
            let stored = decided
                .iter()
                .find(|x| x.account_id == account.id && x.leg == Leg::Close)
                .map(|x| x.position_before);
            let position = match stored {
                Some(p) => p,
                None => {
                    d.positions
                        .position(user, account.id, &contract)
                        .await?
                        .quantity
                }
            };
            positions.insert(account.id, (position, stored.is_some()));
        }

        // Exits never consult the entry gates (P1). For entries, an outage is remembered and
        // only what does not need the answer is decided: close legs at once, and anything
        // whose TTL has passed (never executes late, whatever the gates would say).
        let ids: Vec<AccountId> = targets.iter().map(|(a, _)| a.id).collect();
        let gates = if action.increases_position() {
            Some(entry_gates(&d.gates, user, &contract, &ids).await)
        } else {
            None
        };
        let answers = gates.as_ref().and_then(|g| g.as_ref().ok());
        // A close against a flat expected position reads gate 6 alone (Q75, ADR-0003 r4): in
        // sync it is nothing to close, out of sync a flatten. Without the answer it waits; a
        // close against a non-flat position never asks.
        let flat_closes: Vec<AccountId> = match action {
            Action::Close => ids
                .iter()
                .copied()
                .filter(|a| positions[a].0 == 0)
                .collect(),
            _ => Vec::new(),
        };
        let flat_sync = if flat_closes.is_empty() {
            None
        } else {
            ask(
                &*d.gates.out_of_sync,
                "out_of_sync",
                user,
                &contract,
                &flat_closes,
            )
            .await
            .ok()
        };

        let now = d.clock.now();
        let mut out = Vec::with_capacity(targets.len());
        let mut waiting: Vec<(AccountId, Waiting)> = Vec::new();
        for (account, signal_types) in targets {
            let (position, has_close_leg) = positions[&account.id];
            let ttl_ms = account.sizing.command_ttl_ms;
            let expires_at = i.expiry_base + Duration::milliseconds(i64::from(ttl_ms));
            let expired = now >= expires_at;
            // Without an answer a gate reads as barred: an exit and a close leg never look at
            // it, and the expired case below overrides it explicitly.
            let barred = |n: usize| {
                answers
                    .and_then(|g| g.barred[n].get(&account.id).copied())
                    .unwrap_or(true)
            };
            let facts = DecisionFacts {
                account_active: account.lifecycle.is_none(),
                platform_bound: account.platform_bound,
                automation_not_accepted: source.automated
                    && account.automation.not_accepted().is_some(),
                signal_types: signal_types.clone(),
                out_of_sync: barred(0),
                risk_blocked: barred(1),
                expired,
                limits_pending: barred(2),
                has_bracket_profile: !barred(3),
                multiplier: account.sizing.multiplier,
                max_position: account.sizing.max_position_qty,
                position,
            };
            let mut facts = facts;
            if action == Action::Close && position == 0 {
                match flat_sync.as_ref().and_then(|m| m.get(&account.id)) {
                    Some(out_of_sync) => facts.out_of_sync = *out_of_sync,
                    None => {
                        waiting.push((account.id, Waiting::CloseForSyncState));
                        continue;
                    }
                }
            }
            let commands = |leg: Leg, decision: &Decision| {
                commands_for(
                    &CommandContext {
                        instruction: &i,
                        account_id: account.id,
                        account_name: &account.display_name,
                        contract: &contract,
                        action,
                        leg,
                        created_at: now,
                        expires_at,
                        protection: answers.and_then(|g| g.protection.get(&account.id).copied()),
                    },
                    decision,
                )
                .map_err(|e| DecideError::Port(PortError(e.to_string())))
            };
            // Entry gates asked but not answered (an exit never asks).
            let unanswered = gates.is_some() && answers.is_none();
            if unanswered && !expired {
                // Only the close leg of a reverse can go now; the rest waits.
                if !has_close_leg
                    && let Some(close) = decision::reverse_close_leg(signal_type, action, &facts)
                {
                    out.push(record(
                        user,
                        &i,
                        &source,
                        account,
                        action,
                        &facts,
                        &close,
                        now,
                        Leg::Close,
                        Known {
                            month: Some(&contract.month),
                            position: Some(position),
                            gates_unknown: false,
                        },
                        commands(Leg::Close, &close)?,
                    ));
                }
                waiting.push((account.id, Waiting::EntryForGates));
                continue;
            }
            let leg = if has_close_leg { Leg::Open } else { Leg::Whole };
            let mut decision = if unanswered {
                // TTL passed without answers (N1): never names a gate that did not answer and
                // never places — never routes late.
                decision::decide_expired_unanswered(signal_type, action, &facts)
            } else if has_close_leg {
                decision::reverse_open_leg(signal_type, action, &facts)
            } else {
                decision::decide(signal_type, action, &facts)
            };
            if has_close_leg {
                decision.steps.retain(|s| *s != decision::Step::Flatten);
            }
            let leg_commands = commands(leg, &decision)?;
            out.push(record(
                user,
                &i,
                &source,
                account,
                action,
                &facts,
                &decision,
                now,
                leg,
                Known {
                    month: Some(&contract.month),
                    position: Some(position),
                    gates_unknown: unanswered,
                },
                leg_commands,
            ));
        }
        d.store.record(user, &out).await?;
        if !waiting.is_empty() {
            let cause = match gates {
                Some(Err(e)) => e,
                _ => PortError("gate 6 (sync state) unavailable".into()),
            };
            return Err(DecideError::GatesUnavailable {
                instruction: instruction_id,
                contract,
                accounts: waiting,
                cause,
            });
        }
        Ok(out)
    }
}

/// The decision's row and audit event (`DetailsRoutingDecision`, HTTP 5.1.0; s10 §5: actor
/// `system`). `refs.outcome` follows the Architect's per-code table (decision `capped` is
/// audited `partial`).
/// What the decision knew besides the facts: the resolved contract month, the position it
/// read (none when it never read one), and whether gates 6–9 never answered (N1).
#[derive(Debug, Clone, Copy)]
struct Known<'a> {
    month: Option<&'a str>,
    position: Option<i64>,
    gates_unknown: bool,
}

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
    leg: Leg,
    known: Known<'_>,
    commands: Option<CommandsRequested>,
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
    if let Some(p) = known.position {
        sizing.insert("position_before".into(), json!(p));
    }
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
    if let Some(p) = known.position {
        details.insert("position_side".into(), json!(Side::of_position(p)));
    }
    // The contract the decision used (HTTP 5.2.0, D2).
    details.insert("instrument_symbol".into(), json!(i.instrument_symbol));
    if let Some(month) = known.month {
        details.insert("contract_month".into(), json!(month));
    }
    if known.gates_unknown {
        details.insert("gates_unknown".into(), json!(true));
    }
    if d.expected_flat {
        details.insert("expected_flat".into(), json!(true));
    }
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
        position_before: known.position,
        requested_qty: d.requested_qty,
        executed_qty: d.executed_qty,
        decided_at: now,
        leg,
        gates_unknown: known.gates_unknown,
        expected_flat: d.expected_flat,
        audit: (leg != Leg::Close).then_some((code, payload)),
        commands,
    }
}

#[cfg(test)]
#[path = "decisions_tests.rs"]
mod tests;
