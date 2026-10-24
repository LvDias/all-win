use std::sync::Mutex;

use chrono::TimeZone;
use routing_enterprise::{NotAccepted, Status};
use shared_kernel::clock::FixedClock;

use super::*;

#[derive(Default)]
struct FakeRoutes {
    rows: Mutex<Vec<RouteRecord>>,
    changes: Mutex<Vec<RouteChange>>,
    /// The next N updates race with a user edit: it lands first (signal types change, version
    /// bumps) and the update gets `VersionConflict`.
    concurrent_edits: Mutex<usize>,
}

#[async_trait]
impl RouteStore for FakeRoutes {
    async fn list(&self, user: UserId, f: RouteFilter) -> Result<Vec<RouteRecord>, PortError> {
        Ok(self
            .rows
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.user_id == user)
            .filter(|r| f.source_id.is_none_or(|s| s == r.source_id))
            .filter(|r| f.account_id.is_none_or(|a| a == r.account_id))
            .cloned()
            .collect())
    }

    async fn get(&self, user: UserId, id: RouteId) -> Result<Option<RouteRecord>, PortError> {
        Ok(self
            .rows
            .lock()
            .unwrap()
            .iter()
            .find(|r| r.user_id == user && r.id == id)
            .cloned())
    }

    async fn create(
        &self,
        route: &RouteRecord,
        change: &RouteChange,
    ) -> Result<CreateOutcome, PortError> {
        let mut rows = self.rows.lock().unwrap();
        if rows.iter().any(|r| {
            r.user_id == route.user_id
                && r.source_id == route.source_id
                && r.account_id == route.account_id
        }) {
            return Ok(CreateOutcome::Exists);
        }
        rows.push(route.clone());
        self.changes.lock().unwrap().push(change.clone());
        Ok(CreateOutcome::Created)
    }

    async fn update(
        &self,
        u: &RouteUpdate,
        change: Option<&RouteChange>,
    ) -> Result<UpdateOutcome, PortError> {
        let mut rows = self.rows.lock().unwrap();
        let Some(row) = rows
            .iter_mut()
            .find(|r| r.user_id == u.user_id && r.id == u.id)
        else {
            return Ok(UpdateOutcome::NotFound);
        };
        let mut edits = self.concurrent_edits.lock().unwrap();
        if *edits > 0 {
            *edits -= 1;
            row.signal_types =
                SignalTypes::new([RouteSignalType::Trigger, RouteSignalType::TradePlan]).unwrap();
            row.row_version += 1;
        }
        if row.row_version != u.expected_version {
            return Ok(UpdateOutcome::VersionConflict);
        }
        row.signal_types = u.signal_types.clone();
        row.is_enabled = u.is_enabled;
        row.row_version += 1;
        row.updated_at = u.at;
        if let Some(c) = change {
            self.changes.lock().unwrap().push(c.clone());
        }
        Ok(UpdateOutcome::Updated(row.clone()))
    }

    async fn delete(
        &self,
        user: UserId,
        id: RouteId,
        change: &RouteChange,
    ) -> Result<bool, PortError> {
        let mut rows = self.rows.lock().unwrap();
        let before = rows.len();
        rows.retain(|r| !(r.user_id == user && r.id == id));
        let removed = rows.len() < before;
        if removed {
            self.changes.lock().unwrap().push(change.clone());
        }
        Ok(removed)
    }
}

#[derive(Default)]
struct Directory {
    accounts: Mutex<HashMap<UserId, Vec<AccountFacts>>>,
    sources: Mutex<HashMap<(UserId, SourceId), SourceFacts>>,
}

#[async_trait]
impl AccountDirectory for Directory {
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
impl SourceDirectory for Directory {
    async fn source(&self, user: UserId, id: SourceId) -> Result<Option<SourceFacts>, PortError> {
        Ok(self.sources.lock().unwrap().get(&(user, id)).cloned())
    }
}

/// Records each authorization; answers with `result`.
struct Authorizer {
    calls: Mutex<Vec<AccountId>>,
    result: Result<(), StepUpDenied>,
}

impl Authorizer {
    fn allowing() -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            result: Ok(()),
        }
    }

    fn denying(d: StepUpDenied) -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            result: Err(d),
        }
    }

    fn calls(&self) -> usize {
        self.calls.lock().unwrap().len()
    }
}

#[async_trait]
impl EnableAuthorizer for Authorizer {
    async fn authorize(&self, account: AccountId) -> Result<(), StepUpDenied> {
        self.calls.lock().unwrap().push(account);
        self.result
    }
}

struct World {
    routes: Arc<FakeRoutes>,
    dir: Arc<Directory>,
    deps: RoutingDeps,
    user: UserId,
    source: SourceId,
    account: AccountId,
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
            multiplier: 1,
            max_position_qty: 2,
            command_ttl_ms: 5000,
        },
    }
}

fn world() -> World {
    let routes = Arc::new(FakeRoutes::default());
    let dir = Arc::new(Directory::default());
    let user = UserId::new_v7();
    let source = SourceId::new_v7();
    let account_id = AccountId::new_v7();
    dir.accounts
        .lock()
        .unwrap()
        .insert(user, vec![account(account_id, "Apex 50k")]);
    dir.sources.lock().unwrap().insert(
        (user, source),
        SourceFacts {
            revoked: false,
            automated: true,
            name: "TV main".into(),
        },
    );
    let clock = FixedClock(Utc.with_ymd_and_hms(2026, 10, 23, 12, 0, 0).unwrap());
    let deps = RoutingDeps {
        routes: routes.clone(),
        accounts: dir.clone(),
        sources: dir.clone(),
        gates: None,
        brackets: None,
        clock: Arc::new(clock),
    };
    World {
        routes,
        dir,
        deps,
        user,
        source,
        account: account_id,
    }
}

impl World {
    fn set_account(&self, f: impl FnOnce(&mut AccountFacts)) {
        let mut all = self.dir.accounts.lock().unwrap();
        let a = all
            .get_mut(&self.user)
            .and_then(|v| v.iter_mut().find(|a| a.id == self.account))
            .unwrap();
        f(a);
    }

    fn set_source(&self, revoked: bool, automated: bool) {
        self.dir.sources.lock().unwrap().insert(
            (self.user, self.source),
            SourceFacts {
                revoked,
                automated,
                name: "TV main".into(),
            },
        );
    }

    fn create_cmd(&self, enabled: bool) -> CreateRouteCommand {
        CreateRouteCommand {
            source_id: self.source,
            account_id: self.account,
            signal_types: SignalTypes::new([RouteSignalType::Trigger]).unwrap(),
            is_enabled: enabled,
        }
    }

    async fn create(&self, enabled: bool) -> RouteView {
        CreateRoute::new(self.deps.clone())
            .handle(self.user, self.create_cmd(enabled), &Authorizer::allowing())
            .await
            .unwrap()
    }

    fn actions(&self) -> Vec<(&'static str, ChangeCause)> {
        self.routes
            .changes
            .lock()
            .unwrap()
            .iter()
            .map(|c| (c.action.as_str(), c.cause))
            .collect()
    }
}

fn update(types: &[RouteSignalType], enabled: bool, version: i32) -> UpdateRouteCommand {
    UpdateRouteCommand {
        signal_types: SignalTypes::new(types.iter().copied()).unwrap(),
        is_enabled: enabled,
        row_version: version,
    }
}

#[tokio::test]
async fn create_is_audited_and_returns_the_effective_status() {
    let w = world();
    let v = w.create(true).await;
    assert_eq!(v.effective.status, Status::Receiving);
    assert_eq!(v.route.row_version, 1);
    assert_eq!(w.actions(), [("created", ChangeCause::User)]);
    let payload = w.routes.changes.lock().unwrap()[0].payload();
    assert_eq!(payload["actor"], "user");
    let d = &payload["details"];
    assert_eq!(d["action"], "created");
    assert_eq!(d["source_name"], "TV main");
    assert_eq!(d["signal_types"], json!(["trigger"]));
    assert_eq!(d["is_enabled"], true);
    assert_eq!(d["cause"], "user");
    assert!(d.get("previous_signal_types").is_none());
    assert!(d.get("account_id").is_none());
    assert_eq!(payload["refs"]["account_name"], "Apex 50k");
}

#[tokio::test]
async fn second_create_for_the_pair_is_route_exists() {
    let w = world();
    w.create(true).await;
    let err = CreateRoute::new(w.deps.clone())
        .handle(w.user, w.create_cmd(false), &Authorizer::allowing())
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::RouteExists));
    assert_eq!(w.actions().len(), 1);
}

#[tokio::test]
async fn foreign_source_and_account_are_unknown_and_never_reach_the_step_up() {
    let w = world();
    let other = world();
    let auth = Authorizer::allowing();
    let mut cmd = w.create_cmd(true);
    cmd.source_id = other.source;
    let err = CreateRoute::new(w.deps.clone())
        .handle(w.user, cmd, &auth)
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::SourceUnknown));
    let mut cmd = w.create_cmd(true);
    cmd.account_id = other.account;
    let err = CreateRoute::new(w.deps.clone())
        .handle(w.user, cmd, &auth)
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::AccountUnknown));
    assert_eq!(auth.calls(), 0);
}

#[tokio::test]
async fn ineligible_account_cannot_get_a_route() {
    let w = world();
    w.set_account(|a| a.automation = OptIn::NotAccepted(NotAccepted::Declined));
    let err = CreateRoute::new(w.deps.clone())
        .handle(w.user, w.create_cmd(true), &Authorizer::allowing())
        .await
        .unwrap_err();
    match err {
        RouteError::NotEligible { reason, pointer } => {
            assert_eq!(reason.code(), "routing.automation_not_accepted");
            assert_eq!(pointer, Pointer::AccountId);
        }
        e => panic!("{e:?}"),
    }
    w.set_account(|a| {
        a.automation = OptIn::Accepted { with_step_up: true };
        a.platform_bound = false;
    });
    let err = CreateRoute::new(w.deps.clone())
        .handle(w.user, w.create_cmd(false), &Authorizer::allowing())
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        RouteError::NotEligible {
            reason: RouteReason::PlatformUnbound,
            ..
        }
    ));
    assert!(w.actions().is_empty());
}

#[tokio::test]
async fn non_automated_source_ignores_the_automation_opt_in_but_not_the_step_up() {
    let w = world();
    w.set_source(false, false);
    w.set_account(|a| a.automation = OptIn::NotAccepted(NotAccepted::PendingReview));
    let auth = Authorizer::allowing();
    let v = CreateRoute::new(w.deps.clone())
        .handle(w.user, w.create_cmd(true), &auth)
        .await
        .unwrap();
    assert_eq!(v.effective.status, Status::Receiving);
    assert_eq!(auth.calls(), 1);
}

#[tokio::test]
async fn manual_only_and_revoked_are_refused_on_create() {
    let w = world();
    w.set_account(|a| a.manual_only = Some(json!({"text": "x"})));
    let err = CreateRoute::new(w.deps.clone())
        .handle(w.user, w.create_cmd(true), &Authorizer::allowing())
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::AccountManualOnly));
    w.set_account(|a| a.manual_only = None);
    w.set_source(true, true);
    let err = CreateRoute::new(w.deps.clone())
        .handle(w.user, w.create_cmd(true), &Authorizer::allowing())
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::SourceRevoked { on_create: true }));
}

#[tokio::test]
async fn step_up_is_asked_for_every_switch_on() {
    // Q35 (HTTP 5.2.0): even an automation acceptance confirmed with a step-up, or a real
    // account, needs a fresh `route_enable` step-up to switch a route on.
    for automation in [OptIn::Accepted { with_step_up: true }, OptIn::NotRequired] {
        let w = world();
        w.set_account(|a| a.automation = automation);
        let auth = Authorizer::denying(StepUpDenied::Required);
        let err = CreateRoute::new(w.deps.clone())
            .handle(w.user, w.create_cmd(true), &auth)
            .await
            .unwrap_err();
        assert!(matches!(err, RouteError::StepUp(StepUpDenied::Required)));
        assert_eq!(auth.calls(), 1);
        assert!(w.actions().is_empty());
    }
    let w = world();
    // Created off: no step-up.
    let auth = Authorizer::denying(StepUpDenied::Required);
    let off = CreateRoute::new(w.deps.clone())
        .handle(w.user, w.create_cmd(false), &auth)
        .await
        .unwrap();
    assert_eq!(auth.calls(), 0);
    // Switching on: the proof is required, and a refusal changes nothing.
    let err = UpdateRoute::new(w.deps.clone())
        .handle(
            w.user,
            off.route.id,
            update(&[RouteSignalType::Trigger], true, 1),
            &auth,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::StepUp(StepUpDenied::Required)));
    assert_eq!(auth.calls(), 1);
    assert_eq!(w.actions().len(), 1);
    let ok = Authorizer::allowing();
    let on = UpdateRoute::new(w.deps.clone())
        .handle(
            w.user,
            off.route.id,
            update(&[RouteSignalType::Trigger], true, 1),
            &ok,
        )
        .await
        .unwrap();
    assert!(on.route.is_enabled);
    assert_eq!(ok.calls.lock().unwrap().as_slice(), [w.account]);
    // Changing signal types of an enabled route never asks for a step-up.
    let auth = Authorizer::denying(StepUpDenied::Required);
    UpdateRoute::new(w.deps.clone())
        .handle(
            w.user,
            off.route.id,
            update(
                &[RouteSignalType::Trigger, RouteSignalType::TradePlan],
                true,
                2,
            ),
            &auth,
        )
        .await
        .unwrap();
    assert_eq!(auth.calls(), 0);
    assert_eq!(
        w.actions(),
        [
            ("created", ChangeCause::User),
            ("enabled", ChangeCause::User),
            ("updated", ChangeCause::User)
        ]
    );
}

#[tokio::test]
async fn switching_on_an_ineligible_route_is_refused_but_off_is_always_allowed() {
    let w = world();
    let v = w.create(true).await;
    w.set_account(|a| a.automation = OptIn::NotAccepted(NotAccepted::PendingReview));
    let off = UpdateRoute::new(w.deps.clone())
        .handle(
            w.user,
            v.route.id,
            update(&[RouteSignalType::Trigger], false, 1),
            &Authorizer::allowing(),
        )
        .await
        .unwrap();
    assert!(!off.route.is_enabled);
    let err = UpdateRoute::new(w.deps.clone())
        .handle(
            w.user,
            v.route.id,
            update(&[RouteSignalType::Trigger], true, 2),
            &Authorizer::allowing(),
        )
        .await
        .unwrap_err();
    match err {
        RouteError::NotEligible { reason, pointer } => {
            assert_eq!(
                reason,
                RouteReason::AutomationNotAccepted(NotAccepted::PendingReview)
            );
            assert_eq!(pointer, Pointer::IsEnabled);
        }
        e => panic!("{e:?}"),
    }
}

#[tokio::test]
async fn update_order_is_not_found_then_revoked_then_version() {
    let w = world();
    let v = w.create(true).await;
    let uc = UpdateRoute::new(w.deps.clone());
    let auth = Authorizer::allowing();
    let err = uc
        .handle(
            w.user,
            RouteId::new_v7(),
            update(&[RouteSignalType::Trigger], true, 1),
            &auth,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::NotFound));
    // Another user's route is a 404, too.
    let other = world();
    let err = uc
        .handle(
            other.user,
            v.route.id,
            update(&[RouteSignalType::Trigger], true, 1),
            &auth,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::NotFound));
    let err = uc
        .handle(
            w.user,
            v.route.id,
            update(&[RouteSignalType::NqTrigger], true, 7),
            &auth,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::VersionConflict));
    w.set_source(true, true);
    let err = uc
        .handle(
            w.user,
            v.route.id,
            update(&[RouteSignalType::NqTrigger], true, 7),
            &auth,
        )
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        RouteError::SourceRevoked { on_create: false }
    ));
    let err = DeleteRoute::new(w.deps.clone())
        .handle(w.user, v.route.id)
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        RouteError::SourceRevoked { on_create: false }
    ));
}

#[tokio::test]
async fn an_update_that_changes_nothing_is_not_audited_nor_versioned() {
    let w = world();
    let v = w.create(true).await;
    let same = UpdateRoute::new(w.deps.clone())
        .handle(
            w.user,
            v.route.id,
            update(&[RouteSignalType::Trigger], true, 1),
            &Authorizer::allowing(),
        )
        .await
        .unwrap();
    assert_eq!(same.route.row_version, 1);
    assert_eq!(w.actions().len(), 1);
}

#[tokio::test]
async fn delete_is_audited_and_another_users_route_is_not_found() {
    let w = world();
    let v = w.create(true).await;
    let other = world();
    let err = DeleteRoute::new(other.deps.clone())
        .handle(other.user, v.route.id)
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::NotFound));
    DeleteRoute::new(w.deps.clone())
        .handle(w.user, v.route.id)
        .await
        .unwrap();
    assert_eq!(
        w.actions(),
        [
            ("created", ChangeCause::User),
            ("removed", ChangeCause::User)
        ]
    );
}

#[tokio::test]
async fn ineligible_account_routes_are_switched_off_once_and_stay_off() {
    let w = world();
    let v = w.create(true).await;
    let uc = SwitchOffIneligibleRoutes::new(w.deps.clone());
    assert!(uc.handle(w.user, w.account).await.unwrap().is_empty());
    w.set_account(|a| a.automation = OptIn::NotAccepted(NotAccepted::Declined));
    assert_eq!(uc.handle(w.user, w.account).await.unwrap(), [v.route.id]);
    assert!(uc.handle(w.user, w.account).await.unwrap().is_empty());
    w.set_account(|a| a.automation = OptIn::Accepted { with_step_up: true });
    let routes = ListRoutes::new(w.deps.clone())
        .handle(w.user, RouteFilter::default())
        .await
        .unwrap();
    assert!(!routes[0].route.is_enabled);
    assert_eq!(routes[0].effective.reasons, [RouteReason::RouteDisabled]);
    let changes = w.routes.changes.lock().unwrap();
    let last = changes.last().unwrap();
    assert_eq!(last.action, ChangeAction::Disabled);
    let p = last.payload();
    assert_eq!(p["actor"], "system");
    assert_eq!(p["details"]["cause"], "account_ineligible");
    assert_eq!(p["details"]["previous_is_enabled"], true);
    assert_eq!(p["details"]["is_enabled"], false);
}

/// Esmeril P4: a user edit racing the Q67 b switch-off never leaves the route on.
#[tokio::test]
async fn the_switch_off_retries_a_concurrent_edit_and_gives_up_loudly() {
    let w = world();
    let v = w.create(true).await;
    w.set_account(|a| a.automation = OptIn::NotAccepted(NotAccepted::Declined));
    *w.routes.concurrent_edits.lock().unwrap() = 1;
    let uc = SwitchOffIneligibleRoutes::new(w.deps.clone());
    assert_eq!(uc.handle(w.user, w.account).await.unwrap(), [v.route.id]);
    let row = w.routes.rows.lock().unwrap()[0].clone();
    assert!(!row.is_enabled);
    assert_eq!(row.row_version, 3);
    // The audit carries the signal types the user's edit left.
    let last = w.routes.changes.lock().unwrap().last().unwrap().clone();
    assert_eq!(last.previous.unwrap().0.iter().count(), 2);

    // A route that keeps changing under it: the error goes back to the event delivery.
    let w = world();
    w.create(true).await;
    w.set_account(|a| a.automation = OptIn::NotAccepted(NotAccepted::Declined));
    *w.routes.concurrent_edits.lock().unwrap() = 10;
    let err = SwitchOffIneligibleRoutes::new(w.deps.clone())
        .handle(w.user, w.account)
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::VersionConflict));
    assert!(w.routes.rows.lock().unwrap()[0].is_enabled);
}

#[tokio::test]
async fn list_is_sorted_by_account_name_and_filters_by_source_and_account() {
    let w = world();
    let b = AccountId::new_v7();
    w.dir
        .accounts
        .lock()
        .unwrap()
        .get_mut(&w.user)
        .unwrap()
        .push(account(b, "A first"));
    w.create(true).await;
    let mut cmd = w.create_cmd(true);
    cmd.account_id = b;
    CreateRoute::new(w.deps.clone())
        .handle(w.user, cmd, &Authorizer::allowing())
        .await
        .unwrap();
    let all = ListRoutes::new(w.deps.clone())
        .handle(w.user, RouteFilter::default())
        .await
        .unwrap();
    assert_eq!(
        all.iter().map(|v| v.route.account_id).collect::<Vec<_>>(),
        [b, w.account]
    );
    let only = ListRoutes::new(w.deps.clone())
        .handle(
            w.user,
            RouteFilter {
                source_id: None,
                account_id: Some(w.account),
            },
        )
        .await
        .unwrap();
    assert_eq!(only.len(), 1);
    let foreign = ListRoutes::new(w.deps.clone())
        .handle(
            w.user,
            RouteFilter {
                source_id: Some(SourceId::new_v7()),
                account_id: None,
            },
        )
        .await
        .unwrap();
    assert!(foreign.is_empty());
}

#[tokio::test]
async fn candidates_exclude_routed_accounts_and_list_manual_only_apart() {
    let w = world();
    let blocked = AccountId::new_v7();
    let manual = AccountId::new_v7();
    {
        let mut all = w.dir.accounts.lock().unwrap();
        let v = all.get_mut(&w.user).unwrap();
        let mut b = account(blocked, "Blocked");
        b.automation = OptIn::NotAccepted(NotAccepted::Declined);
        b.lifecycle = Some(LifecycleState::Paused);
        v.push(b);
        let mut m = account(manual, "Topstep");
        m.manual_only = Some(json!({"text": "manual only"}));
        v.push(m);
    }
    w.create(true).await;
    let c = ListRouteCandidates::new(w.deps.clone())
        .handle(w.user, w.source)
        .await
        .unwrap();
    assert_eq!(c.candidates.len(), 1);
    assert_eq!(c.candidates[0].account_id, blocked);
    assert_eq!(
        c.candidates[0].blocking_reasons,
        [RouteReason::AutomationNotAccepted(NotAccepted::Declined)]
    );
    assert_eq!(
        c.candidates[0].status_if_routed.status,
        Status::NotReceiving
    );
    assert_eq!(c.manual_only.len(), 1);
    assert_eq!(c.manual_only[0].0, manual);
    let err = ListRouteCandidates::new(w.deps.clone())
        .handle(w.user, SourceId::new_v7())
        .await
        .unwrap_err();
    assert!(matches!(err, RouteError::NotFound));
}
