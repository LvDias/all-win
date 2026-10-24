//! Use-case behaviour through in-memory fakes of every port.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::type_complexity
)]

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use access::{AccessError, Caller, RequestOrigin, StepUpAction, StepUpProof};
use async_trait::async_trait;
use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};
use rust_decimal::Decimal;
use shared_kernel::{Clock, UserId};
use uuid::Uuid;

use accounts_enterprise::*;

use crate::commands::*;
use crate::deps::AccountsDeps;
use crate::error::AccountsError;
use crate::events::AccountsEvent;
use crate::ports::*;
use crate::testing::*;
use crate::usecases::create::{CreateAccountUseCase, CreateOutcome};
use crate::usecases::update::UpdateAccountUseCase;
use crate::usecases::*;

// ---- fakes --------------------------------------------------------------------------------

#[derive(Default)]
struct Db {
    accounts: HashMap<AccountId, Account>,
    keys: HashMap<(UserId, Uuid), (AccountId, [u8; 32])>,
    events: Vec<AccountsEvent>,
    commission_writes: Vec<CommissionWrite>,
    targets: HashMap<AccountId, MoneyAmount>,
    rule_values: HashMap<AccountId, Vec<preset_values::AccountRuleValue>>,
}

impl Db {
    fn write_rule_values(&mut self, id: AccountId, writes: Vec<RuleValueWrite>) {
        let rows = self.rule_values.entry(id).or_default();
        for w in writes {
            rows.retain(|r| r.field != w.field);
            rows.extend(w.row);
        }
    }
}

#[derive(Default, Clone)]
struct MemStore(Arc<Mutex<Db>>);

#[async_trait]
impl AccountStore for MemStore {
    async fn list(&self, user_id: UserId) -> Result<Vec<Account>, StoreError> {
        let db = self.0.lock().unwrap();
        let mut v: Vec<Account> = db
            .accounts
            .values()
            .filter(|a| a.user_id == user_id && a.archived_at.is_none())
            .cloned()
            .collect();
        v.sort_by_key(|a| a.display_name.as_str().to_lowercase());
        Ok(v)
    }
    async fn get(&self, user_id: UserId, id: AccountId) -> Result<Option<Account>, StoreError> {
        let db = self.0.lock().unwrap();
        Ok(db
            .accounts
            .get(&id)
            .filter(|a| a.user_id == user_id && a.archived_at.is_none())
            .cloned())
    }
    async fn find_by_create_key(
        &self,
        user_id: UserId,
        key: Uuid,
    ) -> Result<Option<CreateKeyRecord>, StoreError> {
        let db = self.0.lock().unwrap();
        Ok(db
            .keys
            .get(&(user_id, key))
            .map(|(id, hash)| CreateKeyRecord {
                account: db.accounts[id].clone(),
                request_hash: *hash,
            }))
    }
    async fn name_taken(
        &self,
        user_id: UserId,
        name: &str,
        except: Option<AccountId>,
    ) -> Result<bool, StoreError> {
        let db = self.0.lock().unwrap();
        Ok(db.accounts.values().any(|a| {
            a.user_id == user_id
                && a.archived_at.is_none()
                && Some(a.id) != except
                && a.display_name.as_str().to_lowercase() == name.to_lowercase()
        }))
    }
    async fn binding_taken(
        &self,
        user_id: UserId,
        venue: VenueAccountId,
        except: Option<AccountId>,
    ) -> Result<bool, StoreError> {
        let db = self.0.lock().unwrap();
        Ok(db.accounts.values().any(|a| {
            a.user_id == user_id
                && Some(a.id) != except
                && a.binding.map(|b| b.venue_account_id) == Some(venue)
        }))
    }
    async fn rule_values(
        &self,
        _: UserId,
        id: AccountId,
    ) -> Result<Vec<preset_values::AccountRuleValue>, StoreError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .rule_values
            .get(&id)
            .cloned()
            .unwrap_or_default())
    }
    async fn evaluation_target(
        &self,
        _: UserId,
        id: AccountId,
    ) -> Result<Option<MoneyAmount>, StoreError> {
        Ok(self.0.lock().unwrap().targets.get(&id).copied())
    }
    async fn create(&self, new: NewAccount) -> Result<(), StoreError> {
        let mut db = self.0.lock().unwrap();
        let a = new.account;
        db.keys
            .insert((a.user_id, new.idempotency_key), (a.id, new.request_hash));
        db.events.extend(new.events);
        db.write_rule_values(a.id, new.rule_values);
        db.accounts.insert(a.id, a);
        Ok(())
    }
    async fn save(&self, w: AccountWrite) -> Result<(), StoreError> {
        let mut db = self.0.lock().unwrap();
        let current = db.accounts.get(&w.account.id).unwrap();
        if current.row_version != w.expected_row_version
            || (!w.rule_values.is_empty() && current.limits_version != w.expected_limits_version)
        {
            return Err(StoreError::VersionConflict);
        }
        db.events.extend(w.events);
        db.write_rule_values(w.account.id, w.rule_values);
        db.commission_writes.push(w.commission);
        db.accounts.insert(w.account.id, w.account);
        Ok(())
    }
}

struct MovableClock(Arc<Mutex<DateTime<Utc>>>);

impl Clock for MovableClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

struct World {
    deps: AccountsDeps,
    db: MemStore,
    venues: Venues,
    gate: Gate,
    now: Arc<Mutex<DateTime<Utc>>>,
    me: Caller,
    other: Caller,
}

fn caller() -> Caller {
    Caller {
        user_id: UserId::new_v7(),
        session_id: Uuid::now_v7(),
        device_id: Uuid::now_v7(),
        is_platform_admin: false,
    }
}

fn world() -> World {
    let db = MemStore::default();
    let venues = Venues::default();
    let gate = Gate::default();
    // 01:00 UTC on 24 Oct is still 23 Oct in São Paulo.
    let now = Arc::new(Mutex::new(
        Utc.with_ymd_and_hms(2026, 10, 24, 1, 0, 0).unwrap(),
    ));
    let deps = AccountsDeps {
        store: Arc::new(db.clone()),
        rules: Arc::new(GenericRules),
        presets: Arc::new(NoPresets),
        venues: Arc::new(venues.clone()),
        routes: Arc::new(NoRoutes),
        time_zones: Arc::new(SaoPaulo),
        profiles: Arc::new(NoProfiles),
        step_up: Arc::new(gate.clone()),
        clock: Arc::new(MovableClock(now.clone())),
        production: false,
    };
    World {
        deps,
        db,
        venues,
        gate,
        now,
        me: caller(),
        other: caller(),
    }
}

fn origin() -> RequestOrigin {
    RequestOrigin {
        ip: "127.0.0.1".parse().unwrap(),
    }
}

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

fn real(name: &str) -> CreateAccount {
    CreateAccount {
        idempotency_key: Uuid::now_v7(),
        request_hash: [1; 32],
        kind: AccountKind::Real,
        display_name: AccountName::parse(name).unwrap(),
        selection: SelectionInput::None,
        size_amount: Some(SizeAmount::new(dec("25000")).unwrap()),
        multiplier: Multiplier::ONE,
        max_position_qty: MaxPositionQty::new(3).unwrap(),
        commission_per_contract_side: None,
        evaluation_start: None,
        evaluation_earned_before: None,
        binding: None,
        rule_values: None,
        profile_size: None,
        opt_ins: OptInChoices::default(),
        step_up: None,
    }
}

fn prop(name: &str, choice: Option<Choice>) -> CreateAccount {
    let mut c = real(name);
    c.kind = AccountKind::Prop;
    c.selection = SelectionInput::Manual {
        firm_name: ManualText::parse("Apex").unwrap(),
        plan_name: ManualText::parse("50K").unwrap(),
        phase: ManualText::parse("Evaluation").unwrap(),
    };
    c.size_amount = Some(SizeAmount::new(dec("50000")).unwrap());
    c.opt_ins.automation = choice.map(|choice| ChoiceRequest {
        choice,
        rule_version_id: GENERIC_AUTOMATION_RULE_VERSION_ID,
    });
    c
}

fn proof() -> Option<StepUpProof> {
    Some(StepUpProof {
        challenge_id: "c".into(),
        code: "ok".into(),
    })
}

async fn create(w: &World, cmd: CreateAccount) -> Result<CreateOutcome, AccountsError> {
    CreateAccountUseCase::new(w.deps.clone())
        .handle(&w.me, &origin(), cmd)
        .await
}

async fn created(w: &World, cmd: CreateAccount) -> crate::views::AccountView {
    match create(w, cmd).await.unwrap() {
        CreateOutcome::Created(v) => v,
        CreateOutcome::Replayed(_) => panic!("replayed"),
    }
}

fn update_of(v: &crate::views::AccountView) -> UpdateAccount {
    let a = &v.account;
    UpdateAccount {
        account_id: a.id,
        row_version: a.row_version,
        display_name: a.display_name.clone(),
        selection: match &a.selection {
            None => SelectionInput::None,
            Some(PropSelection::Manual {
                firm_name,
                plan_name,
                phase,
            }) => SelectionInput::Manual {
                firm_name: firm_name.clone(),
                plan_name: plan_name.clone(),
                phase: phase.clone(),
            },
            Some(_) => panic!("preset"),
        },
        size_amount: Some(a.size_amount),
        multiplier: a.multiplier,
        max_position_qty: a.max_position_qty,
        commission_per_contract_side: CommissionInput::Unchanged,
        binding: None,
        rule_values: None,
        opt_ins: OptInChoices::default(),
        opt_ins_sent: false,
        sent: PropFieldsSent {
            firm_name: a.selection.is_some(),
            plan_name: a.selection.is_some(),
            phase: a.selection.is_some(),
            size_amount: true,
            ..PropFieldsSent::default()
        },
        step_up: None,
    }
}

async fn update(w: &World, cmd: UpdateAccount) -> Result<crate::views::AccountView, AccountsError> {
    UpdateAccountUseCase::new(w.deps.clone())
        .handle(&w.me, &origin(), cmd)
        .await
}

fn event_types(w: &World) -> Vec<&'static str> {
    w.db.0
        .lock()
        .unwrap()
        .events
        .iter()
        .map(AccountsEvent::event_type)
        .collect()
}

fn bind(w: &World, user: UserId, platform_account: &str, kind: VenueKind) -> BindingRef {
    let connection_id = ConnectionId::from_uuid(Uuid::now_v7());
    w.venues.known.lock().unwrap().insert(
        (user, connection_id, platform_account.to_owned()),
        VenueAccountFacts {
            venue_account_id: VenueAccountId::from_uuid(Uuid::now_v7()),
            kind,
        },
    );
    BindingRef {
        connection_id,
        platform_account: platform_account.to_owned(),
    }
}

// ---- create -------------------------------------------------------------------------------

#[tokio::test]
async fn real_account_is_created_not_required_and_audited() {
    let w = world();
    let v = created(&w, real("Conta real")).await;
    assert_eq!(v.account.limits_version, 1);
    assert_eq!(v.automation.state, OptInState::NotRequired);
    assert_eq!(v.automation.firm_stance, None);
    assert_eq!(v.automation.current_rule_version_id, None);
    assert_eq!(v.account.evaluation_start, None);
    assert_eq!(
        event_types(&w),
        [
            "accounts.created",
            "account.limits_changed",
            "account.changed"
        ]
    );
    assert!(w.gate.0.lock().unwrap().is_empty());
}

#[tokio::test]
async fn prop_without_preset_needs_a_choice_on_the_generic_rule() {
    let w = world();
    let e = create(&w, prop("Apex 1", None)).await.unwrap_err();
    assert_eq!(
        e,
        AccountsError::OptInRequired {
            kind: OptInKind::Automation
        }
    );
    assert_eq!(e.pointer().as_deref(), Some("/opt_ins/automation"));
    assert!(w.db.0.lock().unwrap().accounts.is_empty(), "nothing saved");

    let v = created(&w, prop("Apex 1", Some(Choice::Declined))).await;
    assert_eq!(v.automation.firm_stance, Some(FirmStance::Unknown));
    assert_eq!(
        v.automation.current_rule_version_id,
        Some(GENERIC_AUTOMATION_RULE_VERSION_ID)
    );
    assert!(matches!(v.automation.state, OptInState::Chosen(c) if c.choice == Choice::Declined));
    assert_eq!(v.copy.state, OptInState::NotRequired);
    assert_eq!(v.copy.firm_stance, Some(FirmStance::Unknown));
    // Generic rules carry no evaluation target: no default evaluation start.
    assert_eq!(v.account.evaluation_start, None);
    assert!(
        w.gate.0.lock().unwrap().is_empty(),
        "declined needs no step-up"
    );
    assert!(event_types(&w).contains(&"accounts.opt_in_chosen"));
}

#[tokio::test]
async fn evaluation_fields_and_rule_values_need_an_evaluation_preset() {
    let w = world();
    let mut cmd = real("Conta real");
    cmd.evaluation_earned_before = Some(MoneyAmount::new(dec("10")).unwrap());
    let e = create(&w, cmd).await.unwrap_err();
    assert_eq!(
        e,
        AccountsError::EvaluationNotApplicable {
            field: Some("/evaluation_earned_before")
        }
    );
    assert_eq!(e.http_status(), 422);
    assert_eq!(e.pointer().as_deref(), Some("/evaluation_earned_before"));

    let mut cmd = prop("Apex 1", Some(Choice::Declined));
    cmd.evaluation_start = NaiveDate::from_ymd_opt(2026, 10, 1);
    cmd.evaluation_earned_before = Some(MoneyAmount::new(dec("10")).unwrap());
    let e = create(&w, cmd).await.unwrap_err();
    assert_eq!(e.pointer().as_deref(), Some("/evaluation_start"));

    // A preset is unknown before S16: answered before looking at the rule values.
    let mut cmd = prop("Apex 2", Some(Choice::Declined));
    cmd.selection = SelectionInput::Preset {
        preset_version_id: PresetVersionId::from_uuid(Uuid::now_v7()),
        firm_platform: None,
    };
    cmd.rule_values = Some(serde_json::json!({"daily_loss_limit": "100.00"}));
    assert_eq!(
        create(&w, cmd).await.unwrap_err(),
        AccountsError::PresetVersionUnknown
    );
    assert!(w.db.0.lock().unwrap().accounts.is_empty(), "nothing saved");
}

#[tokio::test]
async fn accepting_on_create_needs_the_step_up_before_conflicts() {
    let w = world();
    created(&w, real("Apex 1")).await;
    // Name taken (409) but the step-up is checked first.
    let e = create(&w, prop("apex 1", Some(Choice::Accepted)))
        .await
        .unwrap_err();
    assert_eq!(e, AccountsError::Access(AccessError::StepUpRequired));
    let mut cmd = prop("apex 1", Some(Choice::Accepted));
    cmd.step_up = proof();
    assert_eq!(create(&w, cmd).await.unwrap_err(), AccountsError::NameTaken);

    let mut cmd = prop("Apex 2", Some(Choice::Accepted));
    cmd.step_up = proof();
    let v = match create(&w, cmd).await.unwrap() {
        CreateOutcome::Created(v) => v,
        CreateOutcome::Replayed(_) => panic!(),
    };
    assert!(v.automation.state.is_accepted());
    assert!(
        w.gate
            .0
            .lock()
            .unwrap()
            .iter()
            .all(|(a, t)| *a == StepUpAction::AccountCreateOptInAccept && t.is_none())
    );
}

#[tokio::test]
async fn stale_rule_version_and_copy_choice_are_refused() {
    let w = world();
    let mut cmd = prop("Apex", None);
    cmd.opt_ins.automation = Some(ChoiceRequest {
        choice: Choice::Declined,
        rule_version_id: RuleVersionId::from_uuid(Uuid::now_v7()),
    });
    assert_eq!(
        create(&w, cmd).await.unwrap_err(),
        AccountsError::RuleVersionChanged {
            current: GENERIC_AUTOMATION_RULE_VERSION_ID
        }
    );
    let mut cmd = prop("Apex", Some(Choice::Declined));
    cmd.opt_ins.copy = Some(ChoiceRequest {
        choice: Choice::Declined,
        rule_version_id: GENERIC_COPY_RULE_VERSION_ID,
    });
    assert_eq!(
        create(&w, cmd).await.unwrap_err(),
        AccountsError::OptInNotApplicable {
            kind: OptInKind::Copy
        }
    );
}

#[tokio::test]
async fn create_is_idempotent_by_key_for_24_hours() {
    let w = world();
    let cmd = real("Conta");
    let first = created(&w, cmd.clone()).await;
    // Same key, same body → the same account, even after a rename.
    let mut u = update_of(&first);
    u.display_name = AccountName::parse("Renomeada").unwrap();
    update(&w, u).await.unwrap();
    match create(&w, cmd.clone()).await.unwrap() {
        CreateOutcome::Replayed(v) => {
            assert_eq!(v.account.id, first.account.id);
            assert_eq!(v.account.display_name.as_str(), "Renomeada");
        }
        CreateOutcome::Created(_) => panic!("second account"),
    }
    let mut other_body = cmd.clone();
    other_body.request_hash = [2; 32];
    assert_eq!(
        create(&w, other_body).await.unwrap_err(),
        AccountsError::IdempotencyKeyReused
    );
    *w.now.lock().unwrap() += Duration::hours(24);
    assert_eq!(
        create(&w, cmd).await.unwrap_err(),
        AccountsError::IdempotencyKeyReused
    );
    assert_eq!(w.db.0.lock().unwrap().accounts.len(), 1);
}

#[tokio::test]
async fn binding_is_resolved_through_gateway_and_guarded() {
    let w = world();
    let mut cmd = real("A");
    cmd.binding = Some(BindingRef {
        connection_id: ConnectionId::from_uuid(Uuid::now_v7()),
        platform_account: "Sim101".into(),
    });
    assert_eq!(
        create(&w, cmd.clone()).await.unwrap_err(),
        AccountsError::PlatformAccountUnknown
    );
    // Another user's platform account is indistinguishable from an unknown one.
    cmd.binding = Some(bind(&w, w.other.user_id, "Sim101", VenueKind::Simulation));
    assert_eq!(
        create(&w, cmd.clone()).await.unwrap_err(),
        AccountsError::PlatformAccountUnknown
    );
    cmd.binding = Some(bind(&w, w.me.user_id, "Live1", VenueKind::Live));
    assert_eq!(
        create(&w, cmd.clone()).await.unwrap_err(),
        AccountsError::PlatformAccountKindNotAllowed
    );
    let sim = bind(&w, w.me.user_id, "Sim101", VenueKind::Simulation);
    cmd.binding = Some(sim.clone());
    let v = created(&w, cmd).await;
    assert_eq!(
        v.binding.as_ref().map(|b| b.platform_account.as_str()),
        Some("Sim101")
    );
    assert!(event_types(&w).contains(&"account.binding_changed"));
    let mut again = real("B");
    again.binding = Some(sim);
    assert_eq!(
        create(&w, again).await.unwrap_err(),
        AccountsError::PlatformAccountTaken
    );
}

#[tokio::test]
async fn profile_size_tie_is_decided_by_the_user() {
    let w = world();
    let mut cmd = real("Tie");
    cmd.size_amount = Some(SizeAmount::new(dec("37500")).unwrap());
    assert_eq!(
        create(&w, cmd.clone()).await.unwrap_err(),
        AccountsError::ProfileSizeRequired
    );
    cmd.profile_size = Some(SizeAmount::new(dec("100000")).unwrap());
    assert_eq!(
        create(&w, cmd.clone()).await.unwrap_err(),
        AccountsError::ProfileSizeInvalid
    );
    cmd.profile_size = Some(SizeAmount::new(dec("50000")).unwrap());
    created(&w, cmd).await;
}

// ---- update -------------------------------------------------------------------------------

#[tokio::test]
async fn update_bumps_limits_only_for_limits_inputs() {
    let w = world();
    let v = created(&w, real("A")).await;
    let mut u = update_of(&v);
    u.display_name = AccountName::parse("B").unwrap();
    let v = update(&w, u).await.unwrap();
    assert_eq!((v.account.row_version, v.account.limits_version), (2, 1));

    let mut u = update_of(&v);
    u.max_position_qty = MaxPositionQty::new(5).unwrap();
    let v = update(&w, u).await.unwrap();
    assert_eq!((v.account.row_version, v.account.limits_version), (3, 2));

    // A request that changes nothing is not audited and bumps nothing.
    let before = w.db.0.lock().unwrap().events.len();
    let same = update(&w, update_of(&v)).await.unwrap();
    assert_eq!(same.account.row_version, 3);
    assert_eq!(w.db.0.lock().unwrap().events.len(), before);

    let mut stale = update_of(&v);
    stale.row_version = 1;
    stale.display_name = AccountName::parse("C").unwrap();
    assert_eq!(
        update(&w, stale).await.unwrap_err(),
        AccountsError::VersionConflict
    );
}

#[tokio::test]
async fn commission_absent_keeps_and_null_removes() {
    let w = world();
    let mut cmd = real("A");
    cmd.commission_per_contract_side = Some(MoneyAmount::new(dec("0.37")).unwrap());
    let v = created(&w, cmd).await;
    let u = update_of(&v);
    let v = update(&w, u).await.unwrap();
    assert_eq!(
        v.account.commission_per_contract_side,
        Some(MoneyAmount::new(dec("0.37")).unwrap())
    );
    let mut u = update_of(&v);
    u.commission_per_contract_side = CommissionInput::Remove;
    let v = update(&w, u).await.unwrap();
    assert_eq!(v.account.commission_per_contract_side, None);
    assert_eq!(v.account.limits_version, 2);
    assert_eq!(
        w.db.0.lock().unwrap().commission_writes.last(),
        Some(&CommissionWrite::Unset(RuleOrigin::SetByUser))
    );
}

#[tokio::test]
async fn real_account_update_must_fit_its_kind() {
    let w = world();
    let v = created(&w, real("A")).await;
    let mut u = update_of(&v);
    u.sent.firm_name = true;
    assert_eq!(
        update(&w, u).await.unwrap_err(),
        AccountsError::InvalidRequest {
            pointer: "/firm_name".into()
        }
    );
    let mut u = update_of(&v);
    u.opt_ins.automation = Some(ChoiceRequest {
        choice: Choice::Declined,
        rule_version_id: GENERIC_AUTOMATION_RULE_VERSION_ID,
    });
    assert_eq!(
        update(&w, u).await.unwrap_err(),
        AccountsError::OptInNotApplicable {
            kind: OptInKind::Automation
        }
    );
}

#[tokio::test]
async fn binding_change_of_an_automated_account_needs_step_up_and_no_open_position() {
    let w = world();
    let mut cmd = prop("A", Some(Choice::Accepted));
    cmd.step_up = proof();
    let sim = bind(&w, w.me.user_id, "Sim101", VenueKind::Simulation);
    cmd.binding = Some(sim);
    let v = created(&w, cmd).await;
    w.gate.0.lock().unwrap().clear();

    let mut u = update_of(&v);
    u.binding = None; // unbind
    assert_eq!(
        update(&w, u.clone()).await.unwrap_err(),
        AccountsError::Access(AccessError::StepUpRequired)
    );
    assert_eq!(
        w.gate.0.lock().unwrap()[0],
        (
            StepUpAction::AccountBindingChange,
            Some(v.account.id.as_uuid())
        )
    );
    let venue = v.account.binding.unwrap().venue_account_id;
    w.venues.open.lock().unwrap().push(venue);
    u.step_up = proof();
    assert_eq!(
        update(&w, u.clone()).await.unwrap_err(),
        AccountsError::BindingOpenPosition
    );
    w.venues.open.lock().unwrap().clear();
    let read = ReadAccounts::new(w.deps.clone());
    assert_eq!(
        read.binding_of(w.me.user_id, v.account.id).await.unwrap(),
        Some(venue)
    );
    let after = update(&w, u).await.unwrap();
    assert_eq!(
        read.binding_of(w.me.user_id, v.account.id).await.unwrap(),
        None
    );
    assert!(after.account.binding.is_none());
    assert_eq!(after.account.limits_version, 2);
    // Bind on create and unbind: each writes both account.limits_changed (risk, ADR-0006) and
    // account.binding_changed (gateway, ADR-0008).
    let types = event_types(&w);
    for t in ["account.limits_changed", "account.binding_changed"] {
        assert_eq!(types.iter().filter(|x| **x == t).count(), 2, "{t}");
    }
}

#[tokio::test]
async fn changing_the_manual_selection_keeps_the_generic_choice() {
    let w = world();
    let v = created(&w, prop("A", Some(Choice::Declined))).await;
    let mut u = update_of(&v);
    u.selection = SelectionInput::Manual {
        firm_name: ManualText::parse("Topstep").unwrap(),
        plan_name: ManualText::parse("50K").unwrap(),
        phase: ManualText::parse("Funded").unwrap(),
    };
    // Same generic rule version: no new choice needed.
    let after = update(&w, u).await.unwrap();
    assert_eq!(after.automation.state, v.automation.state);
    assert_eq!(after.account.limits_version, 2);
}

// ---- lifecycle, opt-in, archive, evaluation -----------------------------------------------

#[tokio::test]
async fn lifecycle_transitions() {
    let w = world();
    let v = created(&w, real("A")).await;
    let uc = SetAccountLifecycle::new(w.deps.clone());
    let set = |state, row_version| SetLifecycle {
        account_id: v.account.id,
        state,
        row_version,
    };
    let p = uc
        .handle(&w.me, set(LifecycleState::Paused, 1))
        .await
        .unwrap();
    assert_eq!(p.account.lifecycle, LifecycleState::Paused);
    assert_eq!(
        uc.handle(&w.me, set(LifecycleState::Active, 1))
            .await
            .unwrap_err(),
        AccountsError::VersionConflict
    );
    let d = uc
        .handle(&w.me, set(LifecycleState::Disabled, 2))
        .await
        .unwrap();
    assert_eq!(
        uc.handle(&w.me, set(LifecycleState::Paused, 3))
            .await
            .unwrap_err(),
        AccountsError::LifecycleTransitionInvalid
    );
    let same = uc
        .handle(&w.me, set(LifecycleState::Disabled, 3))
        .await
        .unwrap();
    assert_eq!(same.account.row_version, d.account.row_version, "no-op");
    assert_eq!(same.account.limits_version, 1, "not a limits input");
}

#[tokio::test]
async fn set_opt_in_is_idempotent_and_steps_up_on_accept() {
    let w = world();
    let v = created(&w, prop("A", Some(Choice::Declined))).await;
    let uc = SetAccountOptIn::new(w.deps.clone());
    let cmd = |choice, step_up| SetOptIn {
        account_id: v.account.id,
        kind: OptInKind::Automation,
        request: ChoiceRequest {
            choice,
            rule_version_id: GENERIC_AUTOMATION_RULE_VERSION_ID,
        },
        step_up,
    };
    let same = uc
        .handle(&w.me, &origin(), cmd(Choice::Declined, None))
        .await
        .unwrap();
    assert!(!same.state.is_accepted());
    assert!(w.gate.0.lock().unwrap().is_empty());
    assert_eq!(
        uc.handle(&w.me, &origin(), cmd(Choice::Accepted, None))
            .await
            .unwrap_err(),
        AccountsError::Access(AccessError::StepUpRequired)
    );
    let ok = uc
        .handle(&w.me, &origin(), cmd(Choice::Accepted, proof()))
        .await
        .unwrap();
    assert!(ok.state.is_accepted());
    assert_eq!(
        w.gate.0.lock().unwrap().last(),
        Some(&(
            StepUpAction::AccountOptInAccept,
            Some(v.account.id.as_uuid())
        ))
    );
    let copy = SetOptIn {
        kind: OptInKind::Copy,
        request: ChoiceRequest {
            choice: Choice::Declined,
            rule_version_id: GENERIC_COPY_RULE_VERSION_ID,
        },
        ..cmd(Choice::Declined, None)
    };
    assert_eq!(
        uc.handle(&w.me, &origin(), copy).await.unwrap_err(),
        AccountsError::OptInNotApplicable {
            kind: OptInKind::Copy
        }
    );
}

#[tokio::test]
async fn archive_is_refused_with_an_open_position_and_frees_name_and_binding() {
    let w = world();
    let mut cmd = real("A");
    cmd.binding = Some(bind(&w, w.me.user_id, "Sim101", VenueKind::Simulation));
    let v = created(&w, cmd).await;
    let venue = v.account.binding.unwrap().venue_account_id;
    w.venues.open.lock().unwrap().push(venue);
    let uc = ArchiveAccount::new(w.deps.clone());
    assert_eq!(
        uc.handle(&w.me, v.account.id).await.unwrap_err(),
        AccountsError::DeleteOpenPosition
    );
    let impact = ReadAccounts::new(w.deps.clone())
        .archive_impact(&w.me, v.account.id)
        .await
        .unwrap();
    assert!(impact.has_open_position);
    w.venues.open.lock().unwrap().clear();
    uc.handle(&w.me, v.account.id).await.unwrap();
    let read = ReadAccounts::new(w.deps.clone());
    assert_eq!(
        read.get(&w.me, v.account.id).await.unwrap_err(),
        AccountsError::NotFound
    );
    assert!(read.list(&w.me).await.unwrap().is_empty());
    let stored = w.db.0.lock().unwrap().accounts[&v.account.id].clone();
    assert!(stored.binding.is_none() && stored.archived_at.is_some());
    assert!(event_types(&w).contains(&"account.archived"));
    // The name is free again.
    created(&w, real("a")).await;
}

#[tokio::test]
async fn restart_evaluation_rules() {
    let w = world();
    let v = created(&w, prop("A", Some(Choice::Declined))).await;
    let uc = RestartAccountEvaluation::new(w.deps.clone());
    let cmd = |date| RestartEvaluation {
        account_id: v.account.id,
        evaluation_start: date,
        evaluation_earned_before: None,
        row_version: v.account.row_version,
    };
    let today = NaiveDate::from_ymd_opt(2026, 10, 23).unwrap();
    assert_eq!(
        uc.handle(&w.me, cmd(today)).await.unwrap_err(),
        AccountsError::EvaluationNotApplicable { field: None }
    );
    w.db.0
        .lock()
        .unwrap()
        .targets
        .insert(v.account.id, MoneyAmount::new(dec("3000")).unwrap());
    assert_eq!(
        uc.handle(&w.me, cmd(today.succ_opt().unwrap()))
            .await
            .unwrap_err(),
        AccountsError::EvaluationStartInvalid
    );
    let after = uc.handle(&w.me, cmd(today)).await.unwrap();
    assert_eq!(after.account.limits_version, v.account.limits_version + 1);
    assert!(event_types(&w).contains(&"accounts.evaluation_restarted"));
}

#[tokio::test]
async fn another_users_account_is_not_found_everywhere() {
    let w = world();
    let v = created(&w, prop("Mine", Some(Choice::Declined))).await;
    let id = v.account.id;
    let them = &w.other;
    let read = ReadAccounts::new(w.deps.clone());
    assert_eq!(
        read.get(them, id).await.unwrap_err(),
        AccountsError::NotFound
    );
    assert_eq!(
        read.archive_impact(them, id).await.unwrap_err(),
        AccountsError::NotFound
    );
    assert!(read.list(them).await.unwrap().is_empty());
    assert_eq!(read.binding_of(them.user_id, id).await.unwrap(), None);
    assert!(!read.exists(them.user_id, id).await.unwrap());
    let mut u = update_of(&v);
    u.display_name = AccountName::parse("Theirs").unwrap();
    assert_eq!(
        UpdateAccountUseCase::new(w.deps.clone())
            .handle(them, &origin(), u)
            .await
            .unwrap_err(),
        AccountsError::NotFound
    );
    assert_eq!(
        SetAccountLifecycle::new(w.deps.clone())
            .handle(
                them,
                SetLifecycle {
                    account_id: id,
                    state: LifecycleState::Paused,
                    row_version: 1
                }
            )
            .await
            .unwrap_err(),
        AccountsError::NotFound
    );
    assert_eq!(
        SetAccountOptIn::new(w.deps.clone())
            .handle(
                them,
                &origin(),
                SetOptIn {
                    account_id: id,
                    kind: OptInKind::Automation,
                    request: ChoiceRequest {
                        choice: Choice::Accepted,
                        rule_version_id: GENERIC_AUTOMATION_RULE_VERSION_ID
                    },
                    step_up: proof()
                }
            )
            .await
            .unwrap_err(),
        AccountsError::NotFound
    );
    assert_eq!(
        ArchiveAccount::new(w.deps.clone())
            .handle(them, id)
            .await
            .unwrap_err(),
        AccountsError::NotFound
    );
    assert_eq!(
        RestartAccountEvaluation::new(w.deps.clone())
            .handle(
                them,
                RestartEvaluation {
                    account_id: id,
                    evaluation_start: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
                    evaluation_earned_before: None,
                    row_version: 1
                }
            )
            .await
            .unwrap_err(),
        AccountsError::NotFound
    );
    // The same name is free for another user.
    let mut theirs = real("Mine");
    theirs.idempotency_key = Uuid::now_v7();
    CreateAccountUseCase::new(w.deps.clone())
        .handle(them, &origin(), theirs)
        .await
        .unwrap();
    assert!(
        w.gate.0.lock().unwrap().is_empty(),
        "404 comes before the step-up"
    );
}

/// A current preset whose rules carry an evaluation target (S16 shape), with `allowed` rules.
struct TargetPreset(PresetVersionId);

#[async_trait]
impl PresetCatalog for TargetPreset {
    async fn version(&self, id: PresetVersionId) -> Result<Option<PresetFacts>, PortError> {
        Ok((id == self.0).then(|| PresetFacts {
            preset_version_id: id,
            firm_id: FirmId::from_uuid(Uuid::now_v7()),
            firm_name: "Apex".into(),
            plan_name: "50K".into(),
            phase: "evaluation".into(),
            size_amount: SizeAmount::new(dec("50000")).unwrap(),
            max_position_qty: None,
            firm_platforms: vec![],
            is_current: true,
            preset_id: Uuid::now_v7(),
            has_evaluation_target: true,
            max_position_cap: None,
            superseded_by: None,
            manual_only: false,
            release_version_number: 1,
            release_published_at: Utc.with_ymd_and_hms(2026, 10, 1, 12, 0, 0).unwrap(),
            release_verified_on: NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
            rule_sets: vec![target_rule_set()],
        }))
    }
}

fn target_rule_set() -> PresetRuleSet {
    let src =
        serde_json::json!([{"url": "https://firm.example/rules", "accessed_at": "2026-09-30"}]);
    let v = |value: serde_json::Value| serde_json::json!({"status": "verified", "value": value, "sources": src});
    let wire = serde_json::json!({
        "max_position_qty": v(serde_json::json!({"contracts": 50})),
        "drawdown": v(serde_json::json!({"type": "trailing_eod", "amount": "2500.00"})),
        "daily_loss_limit": v(serde_json::json!({"kind": "none"})),
        "trading_day_reset": v(serde_json::json!({"local_time": "17:00", "time_zone": "America/New_York"})),
        "evaluation_profit_target": v(serde_json::json!({"amount": "3000.00"})),
    });
    PresetRuleSet {
        firm_platform: None,
        rules: preset_values::PresetRules::from_wire(&wire).unwrap(),
        wire,
    }
}

struct AllowedPresetRules;

#[async_trait]
impl RuleVersions for AllowedPresetRules {
    async fn current(
        &self,
        selection: &RuleSelection,
        kind: OptInKind,
    ) -> Result<Option<CurrentRule>, PortError> {
        match selection {
            RuleSelection::Generic => GenericRules.current(selection, kind).await,
            RuleSelection::Preset { .. } => Ok(Some(CurrentRule {
                rule: RuleVersionRef {
                    id: RuleVersionId::from_uuid(Uuid::now_v7()),
                    kind,
                    stance: FirmStance::Allowed,
                },
                sources: vec![],
            })),
        }
    }
}

#[tokio::test]
async fn gaining_an_evaluation_target_defaults_the_start_to_the_registration_date() {
    let mut w = world();
    let preset = PresetVersionId::from_uuid(Uuid::now_v7());
    w.deps.presets = Arc::new(TargetPreset(preset));
    w.deps.rules = Arc::new(AllowedPresetRules);
    // Registered on 23 Oct (São Paulo); generic rules have no target, so no start.
    let v = created(&w, prop("Apex 1", Some(Choice::Declined))).await;
    assert_eq!(v.account.evaluation_start, None);

    *w.now.lock().unwrap() = Utc.with_ymd_and_hms(2026, 11, 5, 15, 0, 0).unwrap();
    let mut u = update_of(&v);
    u.selection = SelectionInput::Preset {
        preset_version_id: preset,
        firm_platform: None,
    };
    u.size_amount = None;
    u.sent = PropFieldsSent {
        preset_version_id: true,
        ..PropFieldsSent::default()
    };
    let after = update(&w, u).await.unwrap();
    assert_eq!(
        after.account.evaluation_start,
        NaiveDate::from_ymd_opt(2026, 10, 23),
        "the registration date, not the change date"
    );
    assert!(after.account.limits_version > v.account.limits_version);
}

#[test]
fn emitted_types_cover_every_event() {
    let now = Utc::now();
    let id = AccountId::new_v7();
    let name = AccountName::parse("A").unwrap();
    let events = [
        AccountsEvent::Created {
            account_id: id,
            account_name: name.clone(),
            at: now,
            fields: vec![],
        },
        AccountsEvent::Updated {
            account_id: id,
            account_name: name.clone(),
            at: now,
            changes: vec![],
        },
        AccountsEvent::Archived {
            account_id: id,
            account_name: name.clone(),
            at: now,
            changes: vec![],
            removed_routes: vec![],
        },
        AccountsEvent::LifecycleChanged {
            account_id: id,
            account_name: name.clone(),
            at: now,
            previous: LifecycleState::Active,
            new: LifecycleState::Paused,
            had_open_position: false,
        },
        AccountsEvent::OptInChosen {
            account_id: id,
            account_name: name.clone(),
            at: now,
            kind: OptInKind::Automation,
            choice: Choice::Declined,
            previous_choice: None,
            rule_version_id: GENERIC_AUTOMATION_RULE_VERSION_ID,
            firm_stance: FirmStance::Unknown,
            sources: vec![],
        },
        AccountsEvent::EvaluationRestarted {
            account_id: id,
            account_name: name.clone(),
            at: now,
            old_start: None,
            new_start: now.date_naive(),
            old_earned_before: None,
            new_earned_before: None,
            profit_target: MoneyAmount::new(dec("1")).unwrap(),
        },
        AccountsEvent::PresetVersionApplied {
            account_id: id,
            account_name: name.clone(),
            at: now,
            from_preset_version_id: PresetVersionId::from_uuid(Uuid::now_v7()),
            to_preset_version_id: PresetVersionId::from_uuid(Uuid::now_v7()),
            from_version_number: 1,
            to_version_number: 2,
            changes: vec![],
            kept_fields: vec![],
        },
        AccountsEvent::PresetValueRestored {
            account_id: id,
            account_name: name.clone(),
            at: now,
            change: crate::events::FieldChange {
                field: "drawdown",
                old: None,
                new: None,
            },
        },
        AccountsEvent::LimitsChanged {
            account_id: id,
            at: now,
            limits_version: 1,
        },
        AccountsEvent::BindingChanged {
            account_id: id,
            at: now,
            venue_account_id: None,
            limits_version: 1,
        },
        AccountsEvent::Changed {
            account_id: id,
            at: now,
        },
        AccountsEvent::ArchivedNotice {
            account_id: id,
            at: now,
        },
    ];
    let types: Vec<&str> = events.iter().map(AccountsEvent::event_type).collect();
    assert_eq!(types, crate::EMITTED_EVENT_TYPES);
    assert!(
        crate::AUDIT_EVENT_TYPES
            .iter()
            .all(|t| crate::EMITTED_EVENT_TYPES.contains(t))
    );
}

#[tokio::test]
async fn accounts_for_venues_returns_only_the_users_live_bound_accounts() {
    let w = world();
    let mut a = real("A");
    a.binding = Some(bind(&w, w.me.user_id, "Sim101", VenueKind::Simulation));
    let a = created(&w, a).await;
    let venue = a.account.binding.unwrap().venue_account_id;
    created(&w, real("B")).await;
    let unknown = VenueAccountId::from_uuid(Uuid::now_v7());
    let read = ReadAccounts::new(w.deps.clone());

    let found = read
        .accounts_for_venues(w.me.user_id, &[venue, unknown])
        .await
        .unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(
        found[&venue],
        BoundAccount {
            account_id: a.account.id,
            display_name: AccountName::parse("A").unwrap(),
        }
    );
    assert!(
        read.accounts_for_venues(w.me.user_id, &[])
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        read.accounts_for_venues(w.other.user_id, &[venue])
            .await
            .unwrap()
            .is_empty(),
        "another user's binding is never returned"
    );
    ArchiveAccount::new(w.deps.clone())
        .handle(&w.me, a.account.id)
        .await
        .unwrap();
    assert!(
        read.accounts_for_venues(w.me.user_id, &[venue])
            .await
            .unwrap()
            .is_empty(),
        "archived (and released) accounts are absent"
    );
}

// ---- S16 per-account preset operations (Torno) ------------------------------------------------

mod preset_ops {
    use serde_json::{Value, json};

    use accounts_enterprise::preset_values::account::{ApplyChoice, FieldOrigin};
    use accounts_enterprise::preset_values::{AccountRuleValue, PresetRules};

    use super::*;
    use crate::presets::account_ops::{
        AccountPresetView, ApplyPresetVersionUseCase, GetAccountPreset, GetAccountPresetDiff,
        RestorePresetValueUseCase,
    };

    #[derive(Default, Clone)]
    struct Catalog(Arc<Mutex<HashMap<PresetVersionId, PresetFacts>>>);

    #[async_trait]
    impl PresetCatalog for Catalog {
        async fn version(&self, id: PresetVersionId) -> Result<Option<PresetFacts>, PortError> {
            Ok(self.0.lock().unwrap().get(&id).cloned())
        }
    }

    const AUTOMATION: Uuid = Uuid::from_u128(0xa1);
    const COPY: Uuid = Uuid::from_u128(0xc1);

    /// The same opt-in rule versions for every preset version, unless one overrides automation.
    #[derive(Default, Clone)]
    struct Rules(Arc<Mutex<HashMap<PresetVersionId, (RuleVersionId, FirmStance)>>>);

    #[async_trait]
    impl RuleVersions for Rules {
        async fn current(
            &self,
            selection: &RuleSelection,
            kind: OptInKind,
        ) -> Result<Option<CurrentRule>, PortError> {
            let RuleSelection::Preset {
                preset_version_id, ..
            } = selection
            else {
                return GenericRules.current(selection, kind).await;
            };
            let over = self.0.lock().unwrap().get(preset_version_id).copied();
            let (id, stance) = match (kind, over) {
                (OptInKind::Automation, Some(o)) => o,
                (OptInKind::Automation, None) => {
                    (RuleVersionId::from_uuid(AUTOMATION), FirmStance::Allowed)
                }
                (OptInKind::Copy, _) => (RuleVersionId::from_uuid(COPY), FirmStance::Allowed),
            };
            Ok(Some(CurrentRule {
                rule: RuleVersionRef { id, kind, stance },
                sources: vec![],
            }))
        }
    }

    fn verified(value: Value) -> Value {
        json!({"status": "verified", "value": value,
               "sources": [{"url": "https://firm.example/rules", "accessed_at": "2026-09-30"}]})
    }

    fn conflict(a: Value, b: Value) -> Value {
        let src = json!([{"url": "https://firm.example/rules", "accessed_at": "2026-09-30"}]);
        json!({"status": "not_confirmed",
               "published_values": [{"value": a, "sources": src}, {"value": b, "sources": src}]})
    }

    fn rules(drawdown: Value) -> Value {
        json!({
            "max_position_qty": verified(json!({"contracts": 50})),
            "drawdown": drawdown,
            "daily_loss_limit": verified(json!({"kind": "none"})),
            "trading_day_reset": verified(json!({"local_time": "17:00", "time_zone": "America/New_York"})),
        })
    }

    fn facts(id: PresetVersionId, preset_id: Uuid, number: i32, wire: Value) -> PresetFacts {
        PresetFacts {
            preset_version_id: id,
            firm_id: FirmId::from_uuid(Uuid::from_u128(0xf1)),
            firm_name: "Topstep".into(),
            plan_name: "Trading Combine".into(),
            phase: "evaluation".into(),
            size_amount: SizeAmount::new(dec("50000")).unwrap(),
            max_position_qty: MaxPositionQty::new(50).ok(),
            firm_platforms: vec![],
            is_current: true,
            preset_id,
            has_evaluation_target: false,
            max_position_cap: None,
            superseded_by: None,
            manual_only: false,
            release_version_number: number,
            release_published_at: Utc
                .with_ymd_and_hms(2026, 10, number as u32, 12, 0, 0)
                .unwrap(),
            release_verified_on: NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
            rule_sets: vec![PresetRuleSet {
                firm_platform: None,
                rules: PresetRules::from_wire(&wire).unwrap(),
                wire,
            }],
        }
    }

    fn static_dd(amount: &str) -> Value {
        json!({"type": "static", "amount": amount})
    }

    struct Fixture {
        w: World,
        catalog: Catalog,
        rules: Rules,
        v1: PresetVersionId,
        v2: PresetVersionId,
    }

    /// v1 (drawdown 2000 static) is current; `publish_v2` adds v2 (drawdown 2500 static).
    fn fixture(v1_drawdown: Value) -> Fixture {
        let mut w = world();
        let catalog = Catalog::default();
        let rule_versions = Rules::default();
        let preset = Uuid::from_u128(0x77);
        let v1 = PresetVersionId::from_uuid(Uuid::now_v7());
        let v2 = PresetVersionId::from_uuid(Uuid::now_v7());
        catalog
            .0
            .lock()
            .unwrap()
            .insert(v1, facts(v1, preset, 1, rules(v1_drawdown)));
        w.deps.presets = Arc::new(catalog.clone());
        w.deps.rules = Arc::new(rule_versions.clone());
        Fixture {
            w,
            catalog,
            rules: rule_versions,
            v1,
            v2,
        }
    }

    impl Fixture {
        fn publish_v2(&self) {
            let mut c = self.catalog.0.lock().unwrap();
            let v2 = facts(
                self.v2,
                Uuid::from_u128(0x77),
                2,
                rules(verified(static_dd("2500.00"))),
            );
            let v1 = c.get_mut(&self.v1).unwrap();
            v1.is_current = false;
            v1.superseded_by = Some(self.v2);
            c.insert(self.v2, v2);
        }

        fn create_cmd(&self, rule_values: Option<Value>) -> CreateAccount {
            let mut c = prop("Combine 1", None);
            c.selection = SelectionInput::Preset {
                preset_version_id: self.v1,
                firm_platform: None,
            };
            c.size_amount = None;
            c.rule_values = rule_values;
            c
        }

        fn rows(&self, id: AccountId) -> Vec<AccountRuleValue> {
            self.w
                .db
                .0
                .lock()
                .unwrap()
                .rule_values
                .get(&id)
                .cloned()
                .unwrap_or_default()
        }

        fn events(&self) -> Vec<AccountsEvent> {
            self.w.db.0.lock().unwrap().events.clone()
        }

        async fn get(
            &self,
            caller: &Caller,
            id: AccountId,
        ) -> Result<AccountPresetView, AccountsError> {
            GetAccountPreset::new(self.w.deps.clone())
                .handle(caller, id)
                .await
        }

        fn apply_cmd(&self, a: &Account) -> ApplyPresetVersion {
            ApplyPresetVersion {
                account_id: a.id,
                target_preset_version_id: self.v2,
                row_version: a.row_version,
                firm_platform: None,
                choices: vec![],
                rule_values: None,
                max_position_qty: None,
                opt_ins: OptInChoices::default(),
                step_up: None,
            }
        }

        async fn apply(&self, cmd: ApplyPresetVersion) -> Result<AccountPresetView, AccountsError> {
            ApplyPresetVersionUseCase::new(self.w.deps.clone())
                .handle(&self.w.me, &origin(), cmd)
                .await
        }

        async fn restore(
            &self,
            a: &Account,
            field: RuleField,
        ) -> Result<AccountPresetView, AccountsError> {
            RestorePresetValueUseCase::new(self.w.deps.clone())
                .handle(
                    &self.w.me,
                    RestorePresetValue {
                        account_id: a.id,
                        field,
                        row_version: a.row_version,
                    },
                )
                .await
        }

        fn account(&self, id: AccountId) -> Account {
            self.w
                .db
                .0
                .lock()
                .unwrap()
                .accounts
                .get(&id)
                .cloned()
                .unwrap()
        }
    }

    fn origin_of(v: &AccountPresetView, field: RuleField) -> FieldOrigin {
        v.fields
            .iter()
            .find(|f| f.view.field == field)
            .unwrap()
            .view
            .origin
    }

    /// An account on v1 with an edited drawdown (1500) and its maximum (3 < 50) edited.
    async fn edited(f: &Fixture) -> Account {
        let v = created(
            &f.w,
            f.create_cmd(Some(json!({"drawdown": static_dd("1500.00")}))),
        )
        .await;
        v.account
    }

    #[tokio::test]
    async fn create_with_a_preset_stores_only_the_deviations() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        let mut fields: Vec<(RuleField, RuleOrigin)> =
            f.rows(a.id).iter().map(|r| (r.field, r.origin)).collect();
        fields.sort_by_key(|(field, _)| field.as_str());
        assert_eq!(
            fields,
            [
                (RuleField::Drawdown, RuleOrigin::Edited),
                (RuleField::MaxPositionQty, RuleOrigin::Edited),
            ]
        );

        // The preset's own value is not stored.
        let same = created(
            &f.w,
            CreateAccount {
                display_name: AccountName::parse("Combine 2").unwrap(),
                idempotency_key: Uuid::now_v7(),
                ..f.create_cmd(Some(json!({"drawdown": static_dd("2000.00")})))
            },
        )
        .await;
        assert!(
            f.rows(same.account.id)
                .iter()
                .all(|r| r.field != RuleField::Drawdown)
        );
    }

    #[tokio::test]
    async fn create_refuses_an_unresolved_required_conflict() {
        let f = fixture(conflict(static_dd("2000.00"), static_dd("2500.00")));
        let e = create(&f.w, f.create_cmd(None)).await.unwrap_err();
        assert_eq!(
            e,
            AccountsError::RequiredFieldUnresolved {
                field: RuleField::Drawdown
            }
        );
        assert_eq!(e.pointer().as_deref(), Some("/rule_values/drawdown"));
        assert!(
            f.w.db.0.lock().unwrap().accounts.is_empty(),
            "nothing saved"
        );

        // Resolved with a published value: recorded as chosen.
        let v = created(
            &f.w,
            f.create_cmd(Some(json!({"drawdown": static_dd("2500.00")}))),
        )
        .await;
        let row = f
            .rows(v.account.id)
            .into_iter()
            .find(|r| r.field == RuleField::Drawdown)
            .unwrap();
        assert_eq!(row.origin, RuleOrigin::ChosenFromConflict);
        assert_eq!(
            row.chose.map(|(c, _)| c),
            Some(preset_values::Chose::PublishedValue)
        );
    }

    #[tokio::test]
    async fn get_shows_each_origin_and_the_newer_version() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        let v = f.get(&f.w.me, a.id).await.unwrap();
        assert_eq!(v.preset_version_id, f.v1);
        assert_eq!(origin_of(&v, RuleField::Drawdown), FieldOrigin::Edited);
        assert_eq!(
            origin_of(&v, RuleField::DailyLossLimit),
            FieldOrigin::Preset
        );
        assert!(v.newer_version.is_none());
        let preset_value = &v
            .fields
            .iter()
            .find(|x| x.view.field == RuleField::Drawdown)
            .unwrap()
            .preset_value;
        assert_eq!(preset_value.as_ref().unwrap()["value"]["amount"], "2000.00");

        f.publish_v2();
        let v = f.get(&f.w.me, a.id).await.unwrap();
        let newer = v.newer_version.unwrap();
        assert_eq!(
            (newer.preset_version_id, newer.release.version_number),
            (f.v2, 2)
        );
        assert!(!newer.suspends, "same opt-in rule versions");

        // Another user's account, and an account without a preset: 404.
        assert_eq!(
            f.get(&f.w.other, a.id).await.unwrap_err(),
            AccountsError::NotFound
        );
        let manual = created(&f.w, prop("Manual", Some(Choice::Declined))).await;
        assert_eq!(
            f.get(&f.w.me, manual.account.id).await.unwrap_err(),
            AccountsError::NotFound
        );
    }

    #[tokio::test]
    async fn diff_lists_the_edited_field_and_refuses_an_older_target() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        f.publish_v2();
        let diff = GetAccountPresetDiff::new(f.w.deps.clone());
        let d = diff.handle(&f.w.me, a.id, f.v2, None).await.unwrap();
        let rows: Vec<(RuleField, bool, bool)> = d
            .rows
            .iter()
            .map(|r| (r.row.field, r.row.edited, r.row.requires_choice))
            .collect();
        assert_eq!(rows, [(RuleField::Drawdown, true, true)]);
        assert_eq!(d.opt_in_changes, [None, None]);
        assert_eq!(
            d.rows[0].new_preset_value.as_ref().unwrap()["value"]["amount"],
            "2500.00"
        );

        let e = diff.handle(&f.w.me, a.id, f.v1, None).await.unwrap_err();
        assert_eq!(e.code(), "presets.version_not_newer");
        assert_eq!(
            e.pointer().as_deref(),
            Some("/query/target_preset_version_id")
        );
        assert_eq!(
            diff.handle(&f.w.other, a.id, f.v2, None).await.unwrap_err(),
            AccountsError::NotFound
        );
    }

    #[tokio::test]
    async fn diff_rows_follow_the_platform_of_a_target_with_platform_rules() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        f.publish_v2();
        // v2 has its own rules per platform; the account (on v1) has no platform.
        {
            let mut c = f.catalog.0.lock().unwrap();
            let v2 = c.get_mut(&f.v2).unwrap();
            let set = |name: &str, amount: &str| {
                let wire = rules(verified(static_dd(amount)));
                PresetRuleSet {
                    firm_platform: Some(FirmPlatform::parse(name).unwrap()),
                    rules: PresetRules::from_wire(&wire).unwrap(),
                    wire,
                }
            };
            v2.rule_sets = vec![set("rithmic", "2500.00"), set("tradovate", "3000.00")];
            v2.firm_platforms = v2
                .rule_sets
                .iter()
                .filter_map(|s| s.firm_platform.clone())
                .collect();
        }
        let diff = GetAccountPresetDiff::new(f.w.deps.clone());

        // No platform to use: no rows, the target's platforms to choose from.
        let d = diff.handle(&f.w.me, a.id, f.v2, None).await.unwrap();
        assert!(d.rows.is_empty() && d.firm_platform_required);
        let names: Vec<&str> = d.firm_platforms.iter().map(|p| p.as_str()).collect();
        assert_eq!(names, ["rithmic", "tradovate"]);

        // A named platform: its rows; still required (it is not the account's yet).
        let tradovate = FirmPlatform::parse("tradovate").unwrap();
        let d = diff
            .handle(&f.w.me, a.id, f.v2, Some(tradovate))
            .await
            .unwrap();
        assert!(d.firm_platform_required);
        assert_eq!(
            d.rows[0].new_preset_value.as_ref().unwrap()["value"]["amount"],
            "3000.00"
        );

        // A platform the target does not offer: 422 on the query.
        let e = diff
            .handle(
                &f.w.me,
                a.id,
                f.v2,
                Some(FirmPlatform::parse("cqg").unwrap()),
            )
            .await
            .unwrap_err();
        assert_eq!(e.code(), "accounts.firm_platform_unknown");
        assert_eq!(e.pointer().as_deref(), Some("/query/firm_platform"));
    }

    #[tokio::test]
    async fn apply_needs_a_choice_for_an_edited_field_then_audits_one_save() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        f.publish_v2();
        let before = f.events().len();

        let e = f.apply(f.apply_cmd(&a)).await.unwrap_err();
        assert_eq!(
            e,
            AccountsError::ChoiceRequired {
                field: RuleField::Drawdown
            }
        );
        assert_eq!(e.pointer().as_deref(), Some("/choices/drawdown"));
        assert_eq!(f.events().len(), before, "nothing saved");

        let mut cmd = f.apply_cmd(&a);
        cmd.choices = vec![(RuleField::Drawdown, ApplyChoice::UsePreset)];
        let v = f.apply(cmd).await.unwrap();
        assert_eq!(v.preset_version_id, f.v2);
        assert_eq!(origin_of(&v, RuleField::Drawdown), FieldOrigin::Preset);
        assert!(f.rows(a.id).iter().all(|r| r.field != RuleField::Drawdown));
        let stored = f.account(a.id);
        assert_eq!(stored.limits_version, a.limits_version + 1);
        assert_eq!(stored.row_version, a.row_version + 1);

        let new_events = &f.events()[before..];
        let types: Vec<&str> = new_events.iter().map(AccountsEvent::event_type).collect();
        assert_eq!(
            types,
            [
                "presets.version_applied",
                "account.limits_changed",
                "account.changed"
            ]
        );
        let p = new_events[0].payload();
        assert_eq!(p["actor"], "user");
        assert_eq!(p["details"]["from_version_number"], 1);
        assert_eq!(p["details"]["to_version_number"], 2);
        assert_eq!(
            p["details"]["changes"],
            json!([{"field": "drawdown",
                    "old": "{\"amount\":\"1500.00\",\"type\":\"static\"}",
                    "new": "{\"amount\":\"2500.00\",\"type\":\"static\"}"}])
        );
        assert!(p["details"].get("kept_fields").is_none());
    }

    #[tokio::test]
    async fn apply_keep_mine_keeps_the_row_and_names_it() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        f.publish_v2();
        let mut cmd = f.apply_cmd(&a);
        cmd.choices = vec![(RuleField::Drawdown, ApplyChoice::KeepMine)];
        let v = f.apply(cmd).await.unwrap();
        assert_eq!(origin_of(&v, RuleField::Drawdown), FieldOrigin::Edited);
        let applied = f
            .events()
            .into_iter()
            .find(|e| e.event_type() == "presets.version_applied")
            .unwrap()
            .payload();
        assert_eq!(applied["details"]["kept_fields"], json!(["drawdown"]));
        assert_eq!(applied["details"]["changes"], json!([]));
    }

    #[tokio::test]
    async fn apply_refuses_stale_superseded_and_older_targets() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        f.publish_v2();
        let mut cmd = f.apply_cmd(&a);
        cmd.choices = vec![(RuleField::Drawdown, ApplyChoice::UsePreset)];
        cmd.row_version += 1;
        assert_eq!(
            f.apply(cmd).await.unwrap_err(),
            AccountsError::VersionConflict
        );

        let mut cmd = f.apply_cmd(&a);
        cmd.target_preset_version_id = f.v1;
        assert_eq!(
            f.apply(cmd).await.unwrap_err(),
            AccountsError::VersionNotNewer {
                pointer: "/target_preset_version_id"
            }
        );

        // v3 published meanwhile: v2 is superseded.
        f.catalog
            .0
            .lock()
            .unwrap()
            .get_mut(&f.v2)
            .unwrap()
            .superseded_by = Some(PresetVersionId::from_uuid(Uuid::now_v7()));
        let mut cmd = f.apply_cmd(&a);
        cmd.choices = vec![(RuleField::Drawdown, ApplyChoice::UsePreset)];
        assert_eq!(
            f.apply(cmd).await.unwrap_err(),
            AccountsError::VersionSuperseded
        );

        let mut cmd = f.apply_cmd(&a);
        cmd.account_id = created(&f.w, prop("Manual", Some(Choice::Declined)))
            .await
            .account
            .id;
        assert_eq!(f.apply(cmd).await.unwrap_err(), AccountsError::NotFound);
    }

    #[tokio::test]
    async fn apply_asks_the_opt_in_again_when_the_version_changes_its_rule() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        f.publish_v2();
        let new_rule = RuleVersionId::from_uuid(Uuid::from_u128(0xa2));
        f.rules
            .0
            .lock()
            .unwrap()
            .insert(f.v2, (new_rule, FirmStance::Conditional));
        let d = GetAccountPresetDiff::new(f.w.deps.clone())
            .handle(&f.w.me, a.id, f.v2, None)
            .await
            .unwrap();
        assert_eq!(d.opt_in_changes, [Some(new_rule), None]);

        let mut cmd = f.apply_cmd(&a);
        cmd.choices = vec![(RuleField::Drawdown, ApplyChoice::UsePreset)];
        assert_eq!(
            f.apply(cmd.clone()).await.unwrap_err(),
            AccountsError::OptInRequired {
                kind: OptInKind::Automation
            }
        );

        cmd.opt_ins.automation = Some(ChoiceRequest {
            choice: Choice::Accepted,
            rule_version_id: new_rule,
        });
        assert!(matches!(
            f.apply(cmd.clone()).await.unwrap_err(),
            AccountsError::Access(AccessError::StepUpRequired)
        ));
        cmd.step_up = proof();
        f.apply(cmd).await.unwrap();
        assert_eq!(
            f.w.gate.0.lock().unwrap().last().copied(),
            Some((StepUpAction::AccountOptInAccept, Some(a.id.as_uuid())))
        );
        assert!(f.account(a.id).has_accepted_automation());
        assert!(
            f.events()
                .iter()
                .any(|e| e.event_type() == "accounts.opt_in_chosen")
        );
    }

    #[tokio::test]
    async fn restore_takes_the_preset_value_once_and_audits_it() {
        let f = fixture(verified(static_dd("2000.00")));
        let a = edited(&f).await;
        let v = f.restore(&a, RuleField::Drawdown).await.unwrap();
        assert_eq!(origin_of(&v, RuleField::Drawdown), FieldOrigin::Preset);
        let restored = f.events().last().cloned();
        assert_eq!(restored.map(|e| e.event_type()), Some("account.changed"));
        let audit = f
            .events()
            .into_iter()
            .find(|e| e.event_type() == "presets.value_restored")
            .unwrap()
            .payload();
        assert_eq!(audit["details"]["change"]["field"], "drawdown");
        assert_eq!(
            audit["details"]["change"]["new"],
            "{\"amount\":\"2000.00\",\"type\":\"static\"}"
        );

        // Not edited any more: a no-op, not audited.
        let a = f.account(a.id);
        let count = f.events().len();
        f.restore(&a, RuleField::Drawdown).await.unwrap();
        assert_eq!(f.events().len(), count);

        // The maximum takes the preset's contracts.
        let v = f.restore(&a, RuleField::MaxPositionQty).await.unwrap();
        assert_eq!(f.account(a.id).max_position_qty.value(), 50);
        assert_eq!(
            origin_of(&v, RuleField::MaxPositionQty),
            FieldOrigin::Preset
        );

        // Stale row_version: 409.
        assert_eq!(
            f.restore(&a, RuleField::Drawdown).await.unwrap_err(),
            AccountsError::VersionConflict
        );
    }

    /// Audit (b) A, condition 3: the largest value of each audited rule, with control characters
    /// (the worst escaping case), fits `FieldChange` without truncation.
    #[test]
    fn the_largest_value_of_each_audited_rule_fits_a_field_change() {
        let ctl = |n: usize| "\u{1}".repeat(n);
        let money = "99999999999999.9999";
        let largest = [
            (
                RuleField::Drawdown,
                json!({"type": "trailing_intraday", "amount": money, "stops_trailing_at": ctl(300),
                       "description": ctl(500),
                       "trailing_lock": {"mode": "floor_reaches_start", "offset": money}}),
            ),
            (
                RuleField::DailyLossLimit,
                json!({"kind": "soft_pause", "amount": money}),
            ),
            (RuleField::DailyProfitTarget, json!({"amount": money})),
            (
                RuleField::TradingDayReset,
                json!({"local_time": "23:59", "time_zone": "America/Argentina/ComodRivadavia",
                       "published_text": ctl(100)}),
            ),
            (
                RuleField::ConsistencyRule,
                json!({"text": ctl(2000), "threshold_pct": "100.0"}),
            ),
            (RuleField::EvaluationProfitTarget, json!({"amount": money})),
        ];
        assert_eq!(
            largest.iter().map(|(f, _)| *f).collect::<Vec<_>>(),
            crate::presets::account_ops::AUDITED_RULE_FIELDS
        );
        for (field, v) in largest {
            let value = preset_values::RuleValue::deserialize(field, &v).unwrap();
            let text = crate::presets::account_ops::audit_text(
                &preset_values::account::AccountValue::Rule(value),
            );
            assert!(
                text.chars().count() <= crate::events::FIELD_CHANGE_MAX_LEN,
                "{field:?}: {} chars",
                text.chars().count()
            );
        }
    }

    #[tokio::test]
    async fn create_and_update_audit_the_rule_values_they_change() {
        let f = fixture(verified(static_dd("2000.00")));
        edited(&f).await;
        let created_fields = match &f.events()[0] {
            AccountsEvent::Created { fields, .. } => fields.clone(),
            e => panic!("{e:?}"),
        };
        let dd = created_fields
            .iter()
            .find(|c| c.field == "drawdown")
            .unwrap();
        assert_eq!(
            (dd.old.as_deref(), dd.new.as_deref()),
            (None, Some("{\"amount\":\"1500.00\",\"type\":\"static\"}"))
        );
        // Only the deviations: the preset's own values are not listed.
        assert!(created_fields.iter().all(|c| c.field != "daily_loss_limit"));

        // A manual account that selects the preset with its own drawdown.
        let manual = created(&f.w, prop("Manual", Some(Choice::Declined))).await;
        let before = f.events().len();
        let mut u = update_of(&manual);
        u.selection = SelectionInput::Preset {
            preset_version_id: f.v1,
            firm_platform: None,
        };
        u.size_amount = None;
        u.sent = PropFieldsSent {
            preset_version_id: true,
            ..PropFieldsSent::default()
        };
        u.rule_values = Some(json!({"drawdown": static_dd("1800.00")}));
        update(&f.w, u).await.unwrap();
        let changes = f.events()[before..]
            .iter()
            .find_map(|e| match e {
                AccountsEvent::Updated { changes, .. } => Some(changes.clone()),
                _ => None,
            })
            .unwrap();
        let dd = changes.iter().find(|c| c.field == "drawdown").unwrap();
        assert_eq!(dd.old, None, "a manual account had no preset drawdown");
        assert_eq!(
            dd.new.as_deref(),
            Some("{\"amount\":\"1800.00\",\"type\":\"static\"}")
        );
        assert!(
            changes
                .iter()
                .all(|c| crate::AUDITED_FIELDS.contains(&c.field))
        );
    }

    #[tokio::test]
    async fn restore_refuses_a_not_confirmed_preset_value() {
        let f = fixture(conflict(static_dd("2000.00"), static_dd("2500.00")));
        let v = created(
            &f.w,
            f.create_cmd(Some(json!({"drawdown": static_dd("2500.00")}))),
        )
        .await;
        assert_eq!(
            f.restore(&v.account, RuleField::Drawdown)
                .await
                .unwrap_err(),
            AccountsError::ValueNotConfirmed
        );
    }
}
