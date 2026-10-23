//! Use-case tests with in-memory fakes (ADR-0004; Esquadro B1 acceptance criteria).

use std::str::FromStr;
use std::sync::Arc;

use chrono::{DateTime, Duration, TimeZone, Utc};
use gateway_enterprise::command::CommandState;
use gateway_enterprise::ids::{AccountId, CommandId, ConnectionId};
use gateway_enterprise::sync::{SyncReason, SyncState};
use gateway_enterprise::triggers::SessionEnd;
use rust_decimal::Decimal;
use serde_json::{Value, json};
use shared_kernel::{Money, UserId};
use uuid::Uuid;

use crate::consume::{
    CommandsOutcome, CommandsRequested, ConsumeCommandsRequested, ConsumeLimitsDerived,
    LimitsDerived,
};
use crate::deliver::DeliverCommands;
use crate::disconnect::HandleDisconnect;
use crate::hello::{HandleHello, HelloOutcome};
use crate::intake::{HandleExecutionReport, IntakeOutcome};
use crate::ports::{AdapterPrincipal, DerivedLimits, VenueAccount};
use crate::session::{Session, SessionSettings, Sessions};
use crate::snapshot::HandlePositionSnapshot;
use crate::testing::{FakeBindings, FakeRisk, FakeStore};
use crate::wire::{ExecutionReport, Hello, PositionSnapshot};

fn t(ms: i64) -> DateTime<Utc> {
    Utc.timestamp_millis_opt(1_800_000_000_000 + ms).unwrap()
}

struct World {
    store: Arc<FakeStore>,
    bindings: Arc<FakeBindings>,
    risk: Arc<FakeRisk>,
    sessions: Sessions,
    principal: AdapterPrincipal,
    account: AccountId,
}

impl World {
    fn new() -> Self {
        let store = Arc::new(FakeStore::default());
        let bindings = Arc::new(FakeBindings::default());
        let connection = ConnectionId::from_uuid(Uuid::from_u128(77));
        let venue = VenueAccount {
            id: gateway_enterprise::ids::VenueAccountId::from_uuid(Uuid::from_u128(5)),
            platform_account: "Sim101".into(),
            kind: "simulation".into(),
            oco_location: "local".into(),
            previous_kind: None,
        };
        store.add_venue(connection, venue.clone());
        let account = AccountId::from_uuid(Uuid::from_u128(9));
        bindings.bind(venue.id, account, "Conta A");
        Self {
            store,
            bindings,
            risk: Arc::new(FakeRisk::default()),
            sessions: Sessions::default(),
            principal: AdapterPrincipal {
                user_id: UserId::from_uuid(Uuid::from_u128(1)),
                connection_id: connection,
                key_id: "AbCdEf123456".into(),
                locale: "pt-BR".into(),
            },
            account,
        }
    }

    fn hello_uc(&self) -> HandleHello {
        HandleHello::new(
            self.store.clone(),
            self.bindings.clone(),
            self.risk.clone(),
            SessionSettings::default(),
        )
    }

    fn deliver_uc(&self) -> DeliverCommands {
        DeliverCommands::new(
            self.store.clone(),
            SessionSettings::default(),
            Duration::seconds(30),
        )
    }

    fn intake_uc(&self) -> HandleExecutionReport {
        HandleExecutionReport::new(self.store.clone(), self.bindings.clone(), self.risk.clone())
    }

    fn snapshot_uc(&self) -> HandlePositionSnapshot {
        HandlePositionSnapshot::new(self.store.clone(), self.bindings.clone())
    }

    async fn connect(
        &self,
        versions: &[&str],
        kind: &str,
        now: DateTime<Utc>,
    ) -> Option<(Arc<Session>, String)> {
        let hello: Hello = serde_json::from_value(json!({
            "instance_id": Uuid::from_u128(3),
            "adapter": {"type": "ninjatrader8", "version": "1.3.0", "build": "development"},
            "supported_protocol_versions": versions,
            "accounts": [{"platform_account": "Sim101", "kind": kind, "oco_location": "local"}],
        }))
        .unwrap();
        match self
            .hello_uc()
            .handle(&self.principal, &hello, now)
            .await
            .unwrap()
        {
            HelloOutcome::Welcome {
                session,
                welcome_frame,
                ..
            } => {
                self.sessions.register(session.clone());
                Some((session, welcome_frame))
            }
            HelloOutcome::Outdated => None,
        }
    }

    async fn reconnect_snapshot(
        &self,
        session: &Session,
        positions: Value,
        orders: Value,
        now: DateTime<Utc>,
    ) {
        let snap: PositionSnapshot = serde_json::from_value(json!({
            "trigger": "reconnect",
            "taken_at": crate::wire::timestamp(now),
            "accounts": [{
                "account_id": self.account, "platform_account": "Sim101", "connected": true,
                "positions": positions, "working_orders": orders,
            }],
        }))
        .unwrap();
        self.snapshot_uc()
            .handle(session, Uuid::now_v7(), &snap, now)
            .await
            .unwrap();
    }

    async fn request(&self, commands: Value, created_at: DateTime<Utc>) -> CommandsOutcome {
        let ev: CommandsRequested = serde_json::from_value(json!({
            "instruction_id": Uuid::from_u128(100),
            "account_id": self.account,
            "created_at": crate::wire::timestamp(created_at),
            "commands": commands,
        }))
        .unwrap();
        ConsumeCommandsRequested::new(self.store.clone(), self.bindings.clone())
            .handle(self.principal.user_id, &ev)
            .await
            .unwrap()
    }

    fn sync_state(&self) -> (SyncState, Vec<SyncReason>) {
        let s = self.store.state();
        let sync = s
            .sync
            .get(&self.account)
            .cloned()
            .unwrap_or_else(gateway_enterprise::sync::AccountSync::new_in_sync);
        (sync.state(), sync.active().iter().map(|r| r.code).collect())
    }

    fn command_state(&self, id: u128) -> CommandState {
        self.store.state().commands[&CommandId::from_uuid(Uuid::from_u128(id))]
            .command
            .state
    }
}

fn entry(id: u128, expires: DateTime<Utc>) -> Value {
    json!({
        "command_id": Uuid::from_u128(id), "kind": "place", "step": "place", "position_effect": "open",
        "expires_at": crate::wire::timestamp(expires),
        "instrument": {"symbol": "MNQ", "contract_month": "2026-12"},
        "body": {"side": "buy", "quantity": 1, "order": {"type": "market"}, "position_effect": "open",
                 "protection": {"mode": "individual", "stop_ticks": 40, "target_ticks": 80}},
    })
}

fn flatten(id: u128) -> Value {
    json!({
        "command_id": Uuid::from_u128(id), "kind": "flatten", "step": "flatten",
        "instrument": {"symbol": "MNQ", "contract_month": "2026-12"},
        "body": {"reason": "close"},
    })
}

fn report(w: &World, extra: Value) -> (ExecutionReport, Value) {
    let mut v = json!({
        "event_id": Uuid::now_v7(), "account_id": w.account, "platform_account": "Sim101",
        "instrument": {"symbol": "MNQ", "contract_month": "2026-12"},
        "origin": "all_win", "status": "working", "occurred_at": crate::wire::timestamp(t(0)),
    });
    for (k, val) in extra.as_object().unwrap() {
        v[k] = val.clone();
    }
    (serde_json::from_value(v.clone()).unwrap(), v)
}

// ---- handshake -----------------------------------------------------------------------------

#[tokio::test]
async fn adapter_below_1_2_is_outdated_with_e3() {
    let w = World::new();
    assert!(
        w.connect(&["1.0", "1.1"], "simulation", t(0))
            .await
            .is_none()
    );
    let events = w.store.state().events.clone();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].payload["status"], "outdated");
    assert_eq!(events[0].payload["disconnect_reason"], "outdated");
}

#[tokio::test]
async fn welcome_negotiates_and_maps_bound_simulation_accounts() {
    let w = World::new();
    let (session, frame) = w
        .connect(&["1.2", "1.3"], "simulation", t(0))
        .await
        .unwrap();
    assert_eq!(session.version.to_string(), "1.3");
    let env: Value = serde_json::from_str(&frame).unwrap();
    assert_eq!(env["type"], "welcome");
    assert_eq!(env["payload"]["accounts"][0]["platform_account"], "Sim101");
    assert_eq!(env["payload"]["accounts"][0]["display_name"], "Conta A");
    assert_eq!(env["payload"]["locale"], "pt-BR");
    assert!(
        w.store
            .event_types()
            .contains(&"gateway.connection_session_changed")
    );
}

#[tokio::test]
async fn live_account_is_left_out_and_its_pending_commands_are_refused() {
    let w = World::new();
    w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    let (session, frame) = w.connect(&["1.2"], "live", t(1)).await.unwrap();
    let env: Value = serde_json::from_str(&frame).unwrap();
    assert_eq!(env["payload"]["accounts"], json!([]));
    assert!(
        w.store
            .event_types()
            .contains(&"connection.account_kind_changed")
    );
    w.request(json!([flatten(1)]), t(2)).await;
    let out = w.deliver_uc().handle(&session, t(3)).await.unwrap();
    assert!(out.is_empty());
    assert_eq!(w.command_state(1), CommandState::Rejected);
    assert_eq!(
        w.sync_state(),
        (SyncState::OutOfSync, vec![SyncReason::CommandRejected])
    );
    assert!(w.store.event_types().contains(&"execution.rejected"));
}

#[tokio::test]
async fn limits_only_for_derived_accounts_and_never_older_than_sent() {
    let w = World::new();
    w.risk.limits.lock().unwrap().insert(
        w.account,
        DerivedLimits {
            version: 4,
            body: json!({"enforcement_enabled": true}),
        },
    );
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    assert_eq!(w.store.state().outbound.len(), 1);
    assert_eq!(
        w.store.state().outbound[0].payload["accounts"][0]["version"],
        4
    );
    let c = ConsumeLimitsDerived::new(w.store.clone(), w.sessions.clone());
    let older = LimitsDerived {
        account_id: w.account.as_uuid(),
        derived_from_version: 3,
        limits: json!({}),
    };
    assert_eq!(c.handle(&older, t(1)).await.unwrap(), None);
    let newer = LimitsDerived {
        account_id: w.account.as_uuid(),
        derived_from_version: 5,
        limits: json!({}),
    };
    assert_eq!(
        c.handle(&newer, t(2)).await.unwrap(),
        Some(session.connection_id)
    );
    assert_eq!(w.store.state().outbound.len(), 2);
}

// ---- delivery ------------------------------------------------------------------------------

#[tokio::test]
async fn no_command_before_the_reconnect_snapshot_is_evaluated() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    w.request(json!([entry(1, t(5_000))]), t(0)).await;
    assert!(
        w.deliver_uc()
            .handle(&session, t(10))
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(w.command_state(1), CommandState::Pending);
    w.reconnect_snapshot(&session, json!([]), json!([]), t(20))
        .await;
    let out = w.deliver_uc().handle(&session, t(30)).await.unwrap();
    assert_eq!(out.len(), 1);
    assert_eq!(w.command_state(1), CommandState::Sent);
    assert!(w.store.event_types().contains(&"execution.command_sent"));
}

#[tokio::test]
async fn crash_between_commit_and_write_retransmits_the_same_bytes() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    w.reconnect_snapshot(&session, json!([]), json!([]), t(1))
        .await;
    w.request(json!([flatten(2), entry(1, t(5_000))]), t(2))
        .await;
    // First pass commits `sent` and hands the frames over; the "process" dies before writing.
    let first = w.deliver_uc().handle(&session, t(3)).await.unwrap();
    assert_eq!(first.len(), 2);
    assert_eq!(w.command_state(1), CommandState::Sent);
    // Restart: a new session after reconnect, past the entry's expiry.
    w.sessions.remove(&session);
    let (session2, _) = w.connect(&["1.2"], "simulation", t(10_000)).await.unwrap();
    w.reconnect_snapshot(&session2, json!([]), json!([]), t(10_001))
        .await;
    let again = w.deliver_uc().handle(&session2, t(10_002)).await.unwrap();
    assert_eq!(
        again, first,
        "byte-identical retransmission, in creation order"
    );
    // Sent never qualifies for any pending-only rule: still sent, not expired, not exempt.
    assert_eq!(w.command_state(1), CommandState::Sent);
    // Not rewritten before the ack timeout.
    assert!(
        w.deliver_uc()
            .handle(&session2, t(10_003))
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn pending_entry_found_expired_is_expired_flagged_and_audited() {
    let w = World::new();
    w.request(json!([entry(1, t(5_000))]), t(0)).await;
    let (session, _) = w.connect(&["1.2"], "simulation", t(60_000)).await.unwrap();
    w.reconnect_snapshot(&session, json!([]), json!([]), t(60_001))
        .await;
    assert!(
        w.deliver_uc()
            .handle(&session, t(60_002))
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(w.command_state(1), CommandState::Expired);
    assert_eq!(
        w.sync_state(),
        (SyncState::OutOfSync, vec![SyncReason::CommandExpired])
    );
    let types = w.store.event_types();
    for t in [
        "execution.expired",
        "sync.flagged",
        "gateway.account_sync_changed",
    ] {
        assert!(types.contains(&t), "{t}");
    }
}

#[tokio::test]
async fn exits_past_the_horizon_become_undelivered() {
    let w = World::new();
    w.request(json!([flatten(2)]), t(0)).await;
    let (session, _) = w
        .connect(&["1.2"], "simulation", t(0) + Duration::hours(72))
        .await
        .unwrap();
    w.reconnect_snapshot(&session, json!([]), json!([]), t(1) + Duration::hours(72))
        .await;
    w.deliver_uc()
        .handle(&session, t(2) + Duration::hours(72))
        .await
        .unwrap();
    assert_eq!(w.command_state(2), CommandState::Undelivered);
    assert_eq!(w.sync_state().1, vec![SyncReason::CommandUndelivered]);
    assert!(w.store.event_types().contains(&"execution.undelivered"));
}

#[tokio::test]
async fn commands_are_idempotent_by_command_id() {
    let w = World::new();
    assert!(matches!(
        w.request(json!([flatten(2)]), t(0)).await,
        CommandsOutcome::Enqueued { new: 1, .. }
    ));
    assert!(matches!(
        w.request(json!([flatten(2)]), t(0)).await,
        CommandsOutcome::Enqueued { new: 0, .. }
    ));
    w.bindings.unbind_all();
    assert_eq!(
        w.request(json!([flatten(3)]), t(0)).await,
        CommandsOutcome::Unbound
    );
}

#[tokio::test]
async fn command_frame_is_a_1_2_envelope_created_at_decision_time() {
    let w = World::new();
    w.request(json!([entry(1, t(5_000))]), t(0)).await;
    let frame = w
        .store
        .state()
        .commands
        .values()
        .next()
        .unwrap()
        .frame
        .clone();
    let env: Value = serde_json::from_str(&frame).unwrap();
    assert_eq!(env["protocol_version"], "1.2");
    assert_eq!(env["sent_at"], crate::wire::timestamp(t(0)));
    assert_eq!(env["payload"]["platform_account"], "Sim101");
    assert_eq!(env["payload"]["place"]["position_effect"], "open");
}

// ---- intake --------------------------------------------------------------------------------

#[tokio::test]
async fn fill_is_stored_once_with_effective_commission_and_e1() {
    let w = World::new();
    *w.risk.rate.lock().unwrap() = Some(Money::new(Decimal::from_str("1.25").unwrap()));
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    let (r, p) = report(
        &w,
        json!({"status": "filled", "side": "buy",
        "fill": {"fill_id": "F1", "quantity": 2, "price": "21450.25"}, "position_after": {"quantity": 2}}),
    );
    let msg = Uuid::now_v7();
    assert_eq!(
        w.intake_uc()
            .handle(&session, msg, &r, &p, t(1))
            .await
            .unwrap(),
        IntakeOutcome::Stored
    );
    assert_eq!(
        w.intake_uc()
            .handle(&session, msg, &r, &p, t(2))
            .await
            .unwrap(),
        IntakeOutcome::Duplicate
    );
    // Same platform fill under a new event id (an adapter that lost its outbox).
    let (r2, p2) = report(
        &w,
        json!({"status": "filled", "fill": {"fill_id": "F1", "quantity": 2, "price": "21450.25"}}),
    );
    assert_eq!(
        w.intake_uc()
            .handle(&session, Uuid::now_v7(), &r2, &p2, t(3))
            .await
            .unwrap(),
        IntakeOutcome::Duplicate
    );
    let s = w.store.state();
    assert_eq!(s.reports.len(), 1);
    let e1 = s
        .events
        .iter()
        .find(|e| e.event_type == "gateway.execution_reported")
        .unwrap();
    assert_eq!(e1.payload["fill"]["reported_commission"], Value::Null);
    assert_eq!(e1.payload["fill"]["commission_effective"], "2.50");
    assert_eq!(e1.payload["fill"]["commission_source"], "configured");
    assert!(s.events.iter().any(|e| e.event_type == "execution.filled"));
}

#[tokio::test]
async fn reported_zero_commission_is_not_absent() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    let (r, p) = report(
        &w,
        json!({"status": "filled", "fill": {"fill_id": "F9", "quantity": 1, "price": "1", "commission": "0"}}),
    );
    w.intake_uc()
        .handle(&session, Uuid::now_v7(), &r, &p, t(1))
        .await
        .unwrap();
    let s = w.store.state();
    let e1 = s
        .events
        .iter()
        .find(|e| e.event_type == "gateway.execution_reported")
        .unwrap();
    assert_eq!(e1.payload["fill"]["reported_commission"], "0");
    assert_eq!(e1.payload["fill"]["commission_source"], "platform");
}

#[tokio::test]
async fn a_failing_port_writes_nothing() {
    let w = World::new();
    *w.risk.fail.lock().unwrap() = true;
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    let before = w.store.state().events.len();
    let (r, p) = report(
        &w,
        json!({"status": "filled", "fill": {"fill_id": "F1", "quantity": 1, "price": "1"}}),
    );
    assert!(
        w.intake_uc()
            .handle(&session, Uuid::now_v7(), &r, &p, t(1))
            .await
            .is_err()
    );
    let s = w.store.state();
    assert!(s.reports.is_empty() && s.receipts.is_empty());
    assert_eq!(s.events.len(), before);
}

#[tokio::test]
async fn rejected_command_flags_with_the_adapter_code() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    w.reconnect_snapshot(&session, json!([]), json!([]), t(1))
        .await;
    w.request(json!([entry(1, t(5_000))]), t(1)).await;
    w.deliver_uc().handle(&session, t(2)).await.unwrap();
    let (r, p) = report(
        &w,
        json!({"command_id": Uuid::from_u128(1), "child_role": "entry", "status": "rejected",
        "reason_code": "order.platform_rejected"}),
    );
    w.intake_uc()
        .handle(&session, Uuid::now_v7(), &r, &p, t(3))
        .await
        .unwrap();
    assert_eq!(w.command_state(1), CommandState::Rejected);
    assert_eq!(
        w.sync_state(),
        (SyncState::OutOfSync, vec![SyncReason::CommandRejected])
    );
    let s = w.store.state();
    let flagged = s
        .events
        .iter()
        .find(|e| e.event_type == "sync.flagged")
        .unwrap();
    assert_eq!(
        flagged.payload["details"]["reason"]["adapter_reason_code"],
        "order.platform_rejected"
    );
}

#[tokio::test]
async fn already_flat_completes_without_flagging() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    w.reconnect_snapshot(&session, json!([]), json!([]), t(1))
        .await;
    w.request(json!([flatten(2)]), t(1)).await;
    w.deliver_uc().handle(&session, t(2)).await.unwrap();
    let (r, p) = report(
        &w,
        json!({"command_id": Uuid::from_u128(2), "child_role": "flatten", "status": "cancelled",
        "reason_code": "command.already_flat"}),
    );
    w.intake_uc()
        .handle(&session, Uuid::now_v7(), &r, &p, t(3))
        .await
        .unwrap();
    assert_eq!(w.command_state(2), CommandState::Completed);
    assert_eq!(w.sync_state().0, SyncState::InSync);
}

#[tokio::test]
async fn report_for_an_account_outside_the_welcome_is_refused() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    let (mut r, p) = report(&w, json!({}));
    r.account_id = Uuid::from_u128(4242);
    assert!(
        w.intake_uc()
            .handle(&session, Uuid::now_v7(), &r, &p, t(1))
            .await
            .is_err()
    );
}

// ---- snapshots and disconnects -------------------------------------------------------------

#[tokio::test]
async fn reconnect_snapshot_mismatch_flags_per_contract() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    w.reconnect_snapshot(&session, json!([{"instrument": {"symbol": "MNQ", "contract_month": "2026-12"}, "position": {"quantity": 1}}]), json!([]), t(1)).await;
    assert_eq!(
        w.sync_state(),
        (SyncState::OutOfSync, vec![SyncReason::SnapshotMismatch])
    );
    assert!(session.snapshot_evaluated(w.account));
}

#[tokio::test]
async fn disconnect_flat_only_reports_the_session_change() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    HandleDisconnect::new(w.store.clone(), w.sessions.clone())
        .handle(&session, SessionEnd::Network, false, t(1))
        .await
        .unwrap();
    assert_eq!(w.sync_state().0, SyncState::InSync);
    let s = w.store.state();
    let last = s.events.last().unwrap();
    assert_eq!(
        (
            last.payload["status"].as_str(),
            last.payload["disconnect_reason"].as_str()
        ),
        (Some("disconnected"), Some("network"))
    );
}

#[tokio::test]
async fn disconnect_in_trade_flags_and_supersede_records_nothing() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    let (r, p) = report(
        &w,
        json!({"status": "filled", "fill": {"fill_id": "F1", "quantity": 1, "price": "1"}, "position_after": {"quantity": 1}}),
    );
    w.intake_uc()
        .handle(&session, Uuid::now_v7(), &r, &p, t(1))
        .await
        .unwrap();
    let uc = HandleDisconnect::new(w.store.clone(), w.sessions.clone());
    let before = w.store.state().events.len();
    uc.handle(&session, SessionEnd::Network, true, t(2))
        .await
        .unwrap();
    assert_eq!(w.store.state().events.len(), before);
    uc.handle(&session, SessionEnd::ByUser, false, t(3))
        .await
        .unwrap();
    assert_eq!(
        w.sync_state(),
        (SyncState::OutOfSync, vec![SyncReason::DisconnectedByUser])
    );
}

// ---- frames --------------------------------------------------------------------------------

struct AcceptAll;

impl crate::frames::FrameValidator for AcceptAll {
    fn validate(&self, _frame: &Value) -> Result<(), String> {
        Ok(())
    }
}

struct RejectAll;

impl crate::frames::FrameValidator for RejectAll {
    fn validate(&self, _frame: &Value) -> Result<(), String> {
        Err("schema violation at /payload".into())
    }
}

fn frames_uc(
    w: &World,
    validator: Arc<dyn crate::frames::FrameValidator>,
) -> crate::frames::HandleFrame {
    crate::frames::HandleFrame::new(validator, w.store.clone(), w.intake_uc(), w.snapshot_uc())
}

fn frame(kind: &str, version: &str, msg_id: Uuid, payload: Value) -> String {
    json!({"protocol_version": version, "msg_id": msg_id, "type": kind,
           "sent_at": crate::wire::timestamp(t(0)), "payload": payload})
    .to_string()
}

#[tokio::test]
async fn deferred_frames_are_stored_once_and_acked() {
    let w = World::new();
    let (session, _) = w.connect(&["1.3"], "simulation", t(0)).await.unwrap();
    let uc = frames_uc(&w, Arc::new(AcceptAll));
    let msg = Uuid::now_v7();
    let text = frame(
        "risk_event",
        "1.3",
        msg,
        json!({"account_id": w.account, "kind": "warning"}),
    );
    for _ in 0..2 {
        let out = uc.handle(&session, &text, t(1)).await;
        assert_eq!(out.close, None);
        let ack: Value = serde_json::from_str(&out.replies[0]).unwrap();
        assert_eq!(
            (ack["type"].as_str(), ack["payload"]["msg_ids"][0].as_str()),
            (Some("ack"), Some(msg.to_string().as_str()))
        );
    }
    let s = w.store.state();
    assert_eq!(s.deferred.len(), 1, "same msg_id is not stored twice");
    assert_eq!(s.deferred[0].account_id, Some(w.account));
    assert_eq!(s.deferred[0].kind, "risk_event");
}

#[tokio::test]
async fn invalid_or_newer_frames_are_not_acked() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    let msg = Uuid::now_v7();
    let out = frames_uc(&w, Arc::new(RejectAll))
        .handle(&session, &frame("risk_event", "1.2", msg, json!({})), t(1))
        .await;
    let err: Value = serde_json::from_str(&out.replies[0]).unwrap();
    assert_eq!(
        (err["type"].as_str(), err["payload"]["code"].as_str()),
        (Some("error"), Some("protocol.invalid_message"))
    );
    assert!(w.store.state().deferred.is_empty());
    let out = frames_uc(&w, Arc::new(AcceptAll))
        .handle(
            &session,
            &frame("trade_completed", "1.3", msg, json!({})),
            t(1),
        )
        .await;
    let err: Value = serde_json::from_str(&out.replies[0]).unwrap();
    assert_eq!(err["payload"]["code"], "protocol.unsupported_value");
    assert_eq!(err["payload"]["ref_msg_id"], json!(msg));
}

#[tokio::test]
async fn unexpected_types_close_with_4006_and_store_failures_with_4000() {
    let w = World::new();
    let (session, _) = w.connect(&["1.2"], "simulation", t(0)).await.unwrap();
    let uc = frames_uc(&w, Arc::new(AcceptAll));
    let out = uc
        .handle(
            &session,
            &frame("hello", "1.2", Uuid::now_v7(), json!({})),
            t(1),
        )
        .await;
    assert_eq!(
        out.close,
        Some((
            crate::frames::CloseCode::ProtocolViolation,
            "unexpected_type"
        ))
    );
    assert_eq!(
        uc.handle(&session, "not json", t(1))
            .await
            .close
            .map(|c| c.0),
        Some(crate::frames::CloseCode::ProtocolViolation)
    );
    *w.store.fail_commits.lock().unwrap() = true;
    let out = uc
        .handle(
            &session,
            &frame("risk_event", "1.2", Uuid::now_v7(), json!({})),
            t(1),
        )
        .await;
    assert_eq!(
        out.close,
        Some((crate::frames::CloseCode::Normal, "intake_retry"))
    );
    assert!(out.replies.is_empty(), "no ack when nothing was stored");
}
