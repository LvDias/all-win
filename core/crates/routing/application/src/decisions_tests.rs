use std::collections::BTreeMap;
use std::sync::Mutex;

use chrono::TimeZone;
use routing_enterprise::{LifecycleState, NotAccepted, OptIn, RouteId, SignalTypes};
use shared_kernel::clock::FixedClock;

use super::*;
use crate::routes::{
    AccountSizing, CreateOutcome, RouteChange, RouteRecord, RouteUpdate, UpdateOutcome,
};

fn at(ms: i64) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 23, 12, 0, 0).unwrap() + Duration::milliseconds(ms)
}

#[derive(Default)]
struct Store {
    instructions: Mutex<HashMap<(UserId, InstructionId), InstructionToRoute>>,
    /// (instruction, account) → the stored decision; first one wins.
    rows: Mutex<BTreeMap<(Uuid, Uuid), DecisionRecord>>,
    events: Mutex<Vec<(&'static str, Value)>>,
}

#[async_trait]
impl DecisionStore for Store {
    async fn instruction(
        &self,
        user: UserId,
        id: InstructionId,
    ) -> Result<Option<InstructionToRoute>, PortError> {
        Ok(self.instructions.lock().unwrap().get(&(user, id)).cloned())
    }

    async fn record(&self, _: UserId, decisions: &[DecisionRecord]) -> Result<usize, PortError> {
        let mut rows = self.rows.lock().unwrap();
        let mut new = 0;
        for d in decisions {
            let key = (d.instruction_id.as_uuid(), d.account_id.as_uuid());
            if rows.contains_key(&key) {
                continue;
            }
            rows.insert(key, d.clone());
            self.events
                .lock()
                .unwrap()
                .push((d.event_type, d.payload.clone()));
            new += 1;
        }
        Ok(new)
    }
}

#[derive(Default)]
struct Routes(Mutex<Vec<RouteRecord>>);

#[async_trait]
impl RouteStore for Routes {
    async fn list(&self, user: UserId, f: RouteFilter) -> Result<Vec<RouteRecord>, PortError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.user_id == user && f.source_id.is_none_or(|s| s == r.source_id))
            .cloned()
            .collect())
    }
    async fn get(&self, _: UserId, _: RouteId) -> Result<Option<RouteRecord>, PortError> {
        unreachable!()
    }
    async fn create(&self, _: &RouteRecord, _: &RouteChange) -> Result<CreateOutcome, PortError> {
        unreachable!()
    }
    async fn update(
        &self,
        _: &RouteUpdate,
        _: Option<&RouteChange>,
    ) -> Result<UpdateOutcome, PortError> {
        unreachable!()
    }
    async fn delete(&self, _: UserId, _: RouteId, _: &RouteChange) -> Result<bool, PortError> {
        unreachable!()
    }
}

#[derive(Default)]
struct Facts {
    accounts: Mutex<HashMap<UserId, Vec<AccountFacts>>>,
    sources: Mutex<HashMap<(UserId, SourceId), SourceFacts>>,
    positions: Mutex<HashMap<AccountId, i64>>,
    position_reads: Mutex<Vec<(UserId, AccountId, String)>>,
}

#[async_trait]
impl AccountDirectory for Facts {
    async fn accounts(&self, user: UserId) -> Result<Vec<AccountFacts>, PortError> {
        Ok(self
            .accounts
            .lock()
            .unwrap()
            .get(&user)
            .cloned()
            .unwrap_or_default())
    }
}

#[async_trait]
impl SourceDirectory for Facts {
    async fn source(&self, user: UserId, id: SourceId) -> Result<Option<SourceFacts>, PortError> {
        Ok(self.sources.lock().unwrap().get(&(user, id)).cloned())
    }
}

#[async_trait]
impl PositionFor for Facts {
    async fn position(
        &self,
        user: UserId,
        account: AccountId,
        symbol: &str,
    ) -> Result<ExpectedPosition, PortError> {
        self.position_reads
            .lock()
            .unwrap()
            .push((user, account, symbol.into()));
        Ok(ExpectedPosition {
            quantity: self
                .positions
                .lock()
                .unwrap()
                .get(&account)
                .copied()
                .unwrap_or(0),
            position_ref: None,
        })
    }
}

/// A gate fake with explicit answers: every account is "not barred" unless set, the whole
/// call can fail (`Unavailable`), or one account can be left without an answer.
#[derive(Default)]
struct Gate {
    barred: Mutex<HashMap<AccountId, bool>>,
    unavailable: Mutex<bool>,
    silent_for: Mutex<Option<AccountId>>,
}

#[async_trait]
impl EntryGate for Gate {
    async fn barred(
        &self,
        _: UserId,
        symbol: &str,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, bool>, PortError> {
        assert_eq!(symbol, "MNQ");
        if *self.unavailable.lock().unwrap() {
            return Err(PortError("unavailable".into()));
        }
        let barred = self.barred.lock().unwrap();
        let silent = *self.silent_for.lock().unwrap();
        Ok(accounts
            .iter()
            .filter(|a| Some(**a) != silent)
            .map(|a| (*a, barred.get(a).copied().unwrap_or(false)))
            .collect())
    }
}

/// Gates 6–9 in ADR-0003 §4.1 order.
#[derive(Default)]
struct Gates([Arc<Gate>; 4]);

impl Gates {
    fn ports(&self) -> EntryGatePorts {
        let [sync, risk, limits, bracket] = self.0.clone();
        EntryGatePorts {
            out_of_sync: sync,
            risk_blocked: risk,
            limits_pending: limits,
            no_bracket_profile: bracket,
        }
    }
}

struct World {
    store: Arc<Store>,
    routes: Arc<Routes>,
    facts: Arc<Facts>,
    user: UserId,
    source: SourceId,
    account: AccountId,
    instruction: InstructionId,
    now_ms: i64,
    gates: Gates,
}

fn account(id: AccountId, name: &str) -> AccountFacts {
    AccountFacts {
        id,
        display_name: name.into(),
        manual_only: None,
        lifecycle: None,
        platform_bound: true,
        automation: OptIn::Accepted { with_step_up: true },
        connection_connected: true,
        sizing: AccountSizing {
            multiplier: 2,
            max_position_qty: 5,
            command_ttl_ms: 5000,
        },
    }
}

fn world() -> World {
    let w = World {
        store: Arc::default(),
        routes: Arc::default(),
        facts: Arc::default(),
        user: UserId::new_v7(),
        source: SourceId::new_v7(),
        account: AccountId::new_v7(),
        instruction: InstructionId::new_v7(),
        now_ms: 300,
        gates: Gates::default(),
    };
    w.facts
        .accounts
        .lock()
        .unwrap()
        .insert(w.user, vec![account(w.account, "Apex 50k")]);
    w.facts.sources.lock().unwrap().insert(
        (w.user, w.source),
        SourceFacts {
            revoked: false,
            automated: true,
            name: "TV main".into(),
        },
    );
    w.add_route(w.account, true);
    w.set_instruction("buy", None, Some(1));
    w
}

impl World {
    fn add_route(&self, account: AccountId, enabled: bool) {
        self.routes.0.lock().unwrap().push(RouteRecord {
            id: RouteId::new_v7(),
            user_id: self.user,
            source_id: self.source,
            account_id: account,
            signal_types: SignalTypes::new([RouteSignalType::Trigger]).unwrap(),
            is_enabled: enabled,
            row_version: 1,
            created_at: at(0),
            updated_at: at(0),
        });
    }

    fn set_instruction(&self, action: &str, side: Option<&str>, quantity: Option<i32>) {
        self.store.instructions.lock().unwrap().insert(
            (self.user, self.instruction),
            InstructionToRoute {
                id: self.instruction,
                source_id: self.source,
                message_id: "tv-MNQ-trigger-1".into(),
                signal_type: "trigger".into(),
                action: action.into(),
                position_effect: None,
                side: side.map(Into::into),
                instrument_symbol: "MNQ".into(),
                quantity,
                expiry_base: at(0),
            },
        );
    }

    fn set_account(&self, f: impl FnOnce(&mut AccountFacts)) {
        let mut all = self.facts.accounts.lock().unwrap();
        f(all
            .get_mut(&self.user)
            .and_then(|v| v.iter_mut().find(|a| a.id == self.account))
            .unwrap());
    }

    fn set_position(&self, q: i64) {
        self.facts.positions.lock().unwrap().insert(self.account, q);
    }

    /// Bars the account at gate `n` (0 = out_of_sync … 3 = no_bracket_profile).
    fn bar(&self, n: usize) {
        self.gates.0[n]
            .barred
            .lock()
            .unwrap()
            .insert(self.account, true);
    }

    fn use_case(&self) -> DecideInstruction {
        DecideInstruction::new(DecisionDeps {
            store: self.store.clone(),
            routes: self.routes.clone(),
            accounts: self.facts.clone(),
            sources: self.facts.clone(),
            positions: self.facts.clone(),
            gates: self.gates.ports(),
            clock: Arc::new(FixedClock(at(self.now_ms))),
        })
    }

    async fn decide(&self) -> Vec<DecisionRecord> {
        self.use_case()
            .handle(self.user, self.instruction)
            .await
            .unwrap()
    }

    /// The one audit event of the one decision.
    async fn only_event(&self) -> (&'static str, Value) {
        let d = self.decide().await;
        assert_eq!(d.len(), 1);
        let events = self.store.events.lock().unwrap();
        assert_eq!(events.len(), 1);
        events[0].clone()
    }
}

#[tokio::test]
async fn an_entry_is_routed_with_the_multiplier_and_audited() {
    let w = world();
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.routed");
    assert_eq!(p["actor"], "system");
    assert_eq!(
        p["details"],
        json!({
            "sizing": {"instruction_qty": 1, "multiplier": 2, "max_position_qty": 5,
                       "position_before": 0, "requested_qty": 2, "executed_qty": 2,
                       "skipped_qty": 0},
            "position_side": "flat",
        })
    );
    assert_eq!(
        p["refs"],
        json!({
            "outcome": "routed", "instruction_id": w.instruction, "source_id": w.source,
            "source_name": "TV main", "account_id": w.account, "account_name": "Apex 50k",
            "message_id": "tv-MNQ-trigger-1", "signal_type": "trigger", "is_exit": false,
        })
    );
    let rows = w.store.rows.lock().unwrap();
    let row = rows.values().next().unwrap();
    assert_eq!((row.outcome, row.reason_code), (Outcome::Routed, None));
    assert_eq!((row.requested_qty, row.executed_qty), (Some(2), Some(2)));
    assert_eq!((row.multiplier, row.max_position_qty), (2, 5));
    assert_eq!(row.decided_at, at(300));
    // The position is read once, for this user, account and instrument.
    assert_eq!(
        *w.facts.position_reads.lock().unwrap(),
        [(w.user, w.account, "MNQ".to_owned())]
    );
}

#[tokio::test]
async fn a_capped_entry_is_partial() {
    let w = world();
    w.set_instruction("buy", None, Some(2));
    w.set_position(3);
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.quantity_capped");
    assert_eq!(p["refs"]["outcome"], "partial");
    assert_eq!(p["details"]["sizing"]["executed_qty"], 2);
    assert_eq!(p["details"]["sizing"]["skipped_qty"], 2);
    assert_eq!(p["details"]["position_side"], "long");
}

#[tokio::test]
async fn only_enabled_routes_of_the_source_are_decided() {
    let w = world();
    let off = AccountId::new_v7();
    w.facts
        .accounts
        .lock()
        .unwrap()
        .get_mut(&w.user)
        .unwrap()
        .push(account(off, "Off"));
    w.add_route(off, false);
    // Another source's route to the same account.
    {
        let mut rows = w.routes.0.lock().unwrap();
        let other_source = RouteRecord {
            source_id: SourceId::new_v7(),
            ..rows[0].clone()
        };
        rows.push(other_source);
    }
    let d = w.decide().await;
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].account_id, w.account);
}

#[tokio::test]
async fn a_barred_reverse_closes_only_and_carries_what_the_notification_needs() {
    let w = world();
    w.set_instruction("reverse", Some("short"), Some(1));
    w.set_position(3);
    w.bar(0);
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.out_of_sync");
    assert_eq!(p["refs"]["outcome"], "partial");
    assert_eq!(p["details"]["reverse_closed_qty"], 3);
    assert_eq!(p["details"]["position_side"], "long");
    assert_eq!(p["details"]["sizing"]["executed_qty"], 0);
}

#[tokio::test]
async fn a_late_entry_expires_but_an_exit_never_does() {
    let w = world();
    w.set_position(1);
    let mut w = w;
    w.now_ms = 5000; // expiry_base + ttl
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.expired");
    assert_eq!(p["refs"]["outcome"], "expired");
    assert_eq!(p["details"]["ttl_ms"], 5000);
    assert_eq!(p["details"]["expiry_base"], "2026-10-23T12:00:00Z");

    let exit = world();
    exit.set_position(1);
    exit.set_instruction("close", None, None);
    let mut exit = exit;
    exit.now_ms = 60_000;
    let (code, p) = exit.only_event().await;
    assert_eq!(code, "routing.routed");
    assert_eq!(p["refs"]["is_exit"], true);
    assert!(p["details"]["sizing"].get("requested_qty").is_none());
}

#[tokio::test]
async fn a_reverse_expired_at_routing_closes_only_with_routing_expired() {
    let mut w = world();
    w.set_instruction("reverse", Some("long"), Some(1));
    w.set_position(-2);
    w.now_ms = 9000;
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.expired");
    assert_eq!(p["refs"]["outcome"], "partial");
    assert_eq!(p["details"]["reverse_closed_qty"], 2);
    assert_eq!(p["details"]["position_side"], "short");
}

#[tokio::test]
async fn gate_details_carry_the_state_that_failed() {
    let w = world();
    w.set_account(|a| a.lifecycle = Some(LifecycleState::Paused));
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.account_inactive");
    assert_eq!(p["refs"]["outcome"], "skipped");
    assert_eq!(p["details"]["lifecycle_state"], "paused");

    let w = world();
    w.set_account(|a| a.automation = OptIn::NotAccepted(NotAccepted::PendingReview));
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.automation_not_accepted");
    assert_eq!(p["details"]["opt_in_state"], "pending_review");

    let w = world();
    w.set_instruction("sell", None, Some(1));
    w.set_position(1);
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.position_conflict");
    assert_eq!(p["refs"]["outcome"], "refused");
}

#[tokio::test]
async fn each_entry_gate_bars_with_its_code() {
    // Protective gates refuse, configuration gates skip (HTTP 5.1.1 `AuditOutcome`).
    for (n, code, outcome) in [
        (0, "routing.out_of_sync", "refused"),
        (1, "routing.risk_blocked", "refused"),
        (2, "routing.limits_pending", "skipped"),
        (3, "routing.no_bracket_profile", "skipped"),
    ] {
        let w = world();
        w.bar(n);
        let (got, p) = w.only_event().await;
        assert_eq!(got, code);
        assert_eq!(p["refs"]["outcome"], outcome, "{code}");
    }
}

/// Fail closed (Architect, 2026-10-23): a gate that cannot answer, for the whole call or for
/// one account, stops the instruction's decision. Nothing is stored or audited, so the
/// outbox delivery retries it; an exit of the same instruction waits too.
#[tokio::test]
async fn a_gate_that_cannot_answer_decides_nothing() {
    for n in 0..4 {
        for (action, side, quantity) in
            [("buy", None, Some(1)), ("reverse", Some("short"), Some(1))]
        {
            let w = world();
            w.set_position(1);
            w.set_instruction(action, side, quantity);
            *w.gates.0[n].unavailable.lock().unwrap() = true;
            let err = w
                .use_case()
                .handle(w.user, w.instruction)
                .await
                .unwrap_err();
            assert!(matches!(err, DecideError::Port(_)), "gate {n} {action}");
            assert!(w.store.rows.lock().unwrap().is_empty());
            assert!(w.store.events.lock().unwrap().is_empty());

            // An answer missing for one account is the same as no answer.
            let w = world();
            *w.gates.0[n].silent_for.lock().unwrap() = Some(w.account);
            assert!(w.use_case().handle(w.user, w.instruction).await.is_err());
            assert!(w.store.events.lock().unwrap().is_empty());
        }
    }
}

/// Esmeril (P1 review): an exit never consults the entry gates, so their outage never delays
/// it.
#[tokio::test]
async fn an_exit_is_decided_while_the_entry_gates_are_down() {
    let w = world();
    w.set_position(2);
    w.set_instruction("close", None, None);
    for g in &w.gates.0 {
        *g.unavailable.lock().unwrap() = true;
    }
    let (code, p) = w.only_event().await;
    assert_eq!(code, "routing.routed");
    assert_eq!(p["refs"]["is_exit"], true);
}

/// The retry after a gate outage decides with the clock of the retry: a late entry expires
/// (never executes late).
#[tokio::test]
async fn an_entry_retried_after_its_ttl_expires() {
    let mut w = world();
    *w.gates.0[1].unavailable.lock().unwrap() = true;
    assert!(w.use_case().handle(w.user, w.instruction).await.is_err());
    *w.gates.0[1].unavailable.lock().unwrap() = false;
    w.now_ms = 7000;
    assert_eq!(w.only_event().await.0, "routing.expired");
}

#[tokio::test]
async fn a_redelivery_stores_and_audits_nothing_new() {
    let w = world();
    w.decide().await;
    w.set_position(5); // the first decision stands
    w.decide().await;
    assert_eq!(w.store.rows.lock().unwrap().len(), 1);
    let events = w.store.events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].0, "routing.routed");
}

#[tokio::test]
async fn another_users_instruction_is_not_found_and_nothing_is_written() {
    let w = world();
    let err = w
        .use_case()
        .handle(UserId::new_v7(), w.instruction)
        .await
        .unwrap_err();
    assert!(matches!(err, DecideError::NotFound));
    assert!(w.store.events.lock().unwrap().is_empty());
    assert!(w.facts.position_reads.lock().unwrap().is_empty());
}

#[tokio::test]
async fn unroutable_actions_and_follower_signals_never_reach_the_engine() {
    let w = world();
    w.set_instruction("modify", None, None);
    let err = w
        .use_case()
        .handle(w.user, w.instruction)
        .await
        .unwrap_err();
    assert!(matches!(err, DecideError::NotRoutable));

    let w = world();
    w.store
        .instructions
        .lock()
        .unwrap()
        .get_mut(&(w.user, w.instruction))
        .unwrap()
        .signal_type = "master_account".into();
    assert!(w.decide().await.is_empty());
    assert!(w.store.events.lock().unwrap().is_empty());
}

// ---------------------------------------------------------------------------------------------
// Contract: every emitted decision code has `details` that match `AuditDetailsByCode[code]` of
// the bundled HTTP contract; `safe_to_display = true` depends on these two tests (Architect,
// 2026-10-23).
// ---------------------------------------------------------------------------------------------

fn bundle() -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../contracts/http/openapi.bundled.yaml");
    yaml_serde::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn details_ref(bundle: &Value, code: &str) -> String {
    bundle["components"]["schemas"]["AuditDetailsByCode"]["properties"][code]["$ref"]
        .as_str()
        .unwrap_or_else(|| panic!("{code} has no AuditDetailsByCode entry"))
        .to_owned()
}

/// One scenario per code the decision emits.
async fn emitted_events() -> Vec<(&'static str, Value)> {
    type Setup = fn(&mut World);
    let scenarios: [Setup; 15] = [
        |_| {},
        |w| {
            w.set_instruction("buy", None, Some(2));
            w.set_position(3);
        },
        |w| w.set_position(5),
        |w| {
            w.set_instruction("reverse", Some("short"), Some(1));
            w.set_position(2);
        },
        |w| {
            w.store
                .instructions
                .lock()
                .unwrap()
                .get_mut(&(w.user, w.instruction))
                .unwrap()
                .signal_type = "nq_trigger".into();
        },
        |w| w.set_account(|a| a.automation = OptIn::NotAccepted(NotAccepted::Declined)),
        |w| w.set_account(|a| a.lifecycle = Some(LifecycleState::Disabled)),
        |w| w.set_account(|a| a.platform_bound = false),
        |w| w.bar(3),
        |w| w.bar(2),
        |w| w.bar(0),
        |w| w.bar(1),
        |w| {
            w.set_instruction("sell", None, Some(1));
            w.set_position(1);
        },
        |w| w.set_instruction("close", None, None),
        |w| {
            w.set_instruction("reverse", Some("long"), Some(1));
            w.set_position(-2);
            w.now_ms = 9000;
        },
    ];
    let mut out = Vec::new();
    for setup in scenarios {
        let mut w = world();
        setup(&mut w);
        out.push(w.only_event().await);
    }
    let mut w = world();
    w.now_ms = 9000;
    out.push(w.only_event().await);
    out
}

#[tokio::test]
async fn emitted_decision_details_match_the_contract() {
    let bundle = bundle();
    let events = emitted_events().await;
    let codes: std::collections::BTreeSet<&str> = events.iter().map(|(c, _)| *c).collect();
    assert_eq!(
        codes.into_iter().collect::<Vec<_>>(),
        [
            "routing.account_inactive",
            "routing.automation_not_accepted",
            "routing.expired",
            "routing.limits_pending",
            "routing.max_position_reached",
            "routing.no_bracket_profile",
            "routing.nothing_to_close",
            "routing.out_of_sync",
            "routing.platform_unbound",
            "routing.position_conflict",
            "routing.quantity_capped",
            "routing.risk_blocked",
            "routing.routed",
            "routing.signal_type_not_selected",
        ]
    );
    for (code, payload) in &events {
        let schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$ref": details_ref(&bundle, code),
            "components": bundle["components"],
        });
        let validator = jsonschema::validator_for(&schema).unwrap();
        let errors: Vec<String> = validator
            .iter_errors(&payload["details"])
            .map(|e| e.to_string())
            .collect();
        assert!(
            errors.is_empty(),
            "{code}: {errors:?} in {}",
            payload["details"]
        );
        // The validator really checks: an unknown key fails (closed schema).
        let mut extra = payload["details"].clone();
        extra["free_text"] = json!("x");
        assert!(!validator.is_valid(&extra), "{code}");
    }
}

/// No free text can reach the displayed `details`: every string in the decision schema
/// (followed through `$ref`) is closed by `enum`, `const` or `pattern`.
#[test]
fn decision_details_schema_has_no_free_strings() {
    fn resolve<'a>(bundle: &'a Value, node: &'a Value) -> &'a Value {
        match node.get("$ref").and_then(Value::as_str) {
            Some(r) => {
                let name = r.rsplit('/').next().unwrap();
                resolve(bundle, &bundle["components"]["schemas"][name])
            }
            None => node,
        }
    }
    fn walk(bundle: &Value, node: &Value, path: &str, depth: usize, free: &mut Vec<String>) {
        assert!(depth < 16, "schema too deep at {path}");
        let node = resolve(bundle, node);
        let is_string = match &node["type"] {
            Value::String(t) => t == "string",
            Value::Array(ts) => ts.iter().any(|t| t == "string"),
            _ => false,
        };
        if is_string
            && ["enum", "const", "pattern"]
                .iter()
                .all(|k| node.get(k).is_none())
        {
            free.push(path.to_owned());
        }
        if let Some(props) = node["properties"].as_object() {
            for (k, v) in props {
                walk(bundle, v, &format!("{path}/{k}"), depth + 1, free);
            }
        }
        for key in ["items", "additionalProperties"] {
            if let Some(child) = node.get(key).filter(|c| c.is_object()) {
                walk(bundle, child, &format!("{path}/{key}"), depth + 1, free);
            }
        }
        for key in ["allOf", "anyOf", "oneOf"] {
            for (n, child) in node[key].as_array().into_iter().flatten().enumerate() {
                walk(bundle, child, &format!("{path}/{key}/{n}"), depth + 1, free);
            }
        }
    }
    let bundle = bundle();
    let root = json!({ "$ref": details_ref(&bundle, "routing.routed") });
    let mut free = Vec::new();
    walk(&bundle, &root, "DetailsRoutingDecision", 0, &mut free);
    assert!(free.is_empty(), "free strings: {free:?}");
}
