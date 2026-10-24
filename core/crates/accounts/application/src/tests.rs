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
        db.accounts.insert(a.id, a);
        Ok(())
    }
    async fn save(&self, w: AccountWrite) -> Result<(), StoreError> {
        let mut db = self.0.lock().unwrap();
        let current = db.accounts.get(&w.account.id).unwrap();
        if current.row_version != w.expected_row_version {
            return Err(StoreError::VersionConflict);
        }
        db.events.extend(w.events);
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
        }))
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
            account_name: name,
            at: now,
            old_start: None,
            new_start: now.date_naive(),
            old_earned_before: None,
            new_earned_before: None,
            profit_target: MoneyAmount::new(dec("1")).unwrap(),
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
