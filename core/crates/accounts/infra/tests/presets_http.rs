//! The per-account preset operations end to end (S16, Torno): routers, use cases and the
//! PostgreSQL store (RLS on); the catalog and the opt-in rules are fakes. Every endpoint has a
//! cross-user test (404); responses and audit details are checked against the contract.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use accounts_application::AccountsDeps;
use accounts_application::enterprise::preset_values::PresetRules;
use accounts_application::enterprise::{
    AccountId, FirmId, FirmStance, MaxPositionQty, OptInKind, PresetVersionId, RuleField,
    RuleOrigin, RuleVersionId, RuleVersionRef, SizeAmount,
};
use accounts_application::ports::{
    AccountStore, AccountWrite, CommissionWrite, CurrentRule, PortError, PresetCatalog,
    PresetFacts, PresetRuleSet, RuleSelection, RuleValueWrite, RuleVersions, StoreError,
};
use accounts_application::presets::PRESET_AUDITED_FIELDS;
use accounts_application::testing::*;
use accounts_infra::PgAccountStore;
use accounts_infra::http::presets::{self, AccountPresetsHttpState};
use accounts_infra::http::{AccountsHttpState, router};
use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use chrono::{NaiveDate, TimeZone, Utc};
use http_body_util::BodyExt;
use rust_decimal::Decimal;
use serde_json::{Value, json};
use shared_kernel::{SystemClock, UserId};
use tower::ServiceExt;
use uuid::Uuid;

const ORIGIN: &str = "https://app.test";

#[derive(Default, Clone)]
struct Catalog(Arc<Mutex<HashMap<PresetVersionId, PresetFacts>>>);

#[async_trait]
impl PresetCatalog for Catalog {
    async fn version(&self, id: PresetVersionId) -> Result<Option<PresetFacts>, PortError> {
        Ok(self.0.lock().unwrap().get(&id).cloned())
    }
}

/// Stable opt-in rule versions for every preset version (stance allowed: nothing to choose).
struct Rules;

#[async_trait]
impl RuleVersions for Rules {
    async fn current(
        &self,
        selection: &RuleSelection,
        kind: OptInKind,
    ) -> Result<Option<CurrentRule>, PortError> {
        if matches!(selection, RuleSelection::Generic) {
            return GenericRules.current(selection, kind).await;
        }
        let id = match kind {
            OptInKind::Automation => Uuid::from_u128(0xa1),
            OptInKind::Copy => Uuid::from_u128(0xc1),
        };
        Ok(Some(CurrentRule {
            rule: RuleVersionRef {
                id: RuleVersionId::from_uuid(id),
                kind,
                stance: FirmStance::Allowed,
            },
            sources: vec![],
        }))
    }
}

fn verified(value: Value) -> Value {
    json!({"status": "verified", "value": value,
           "sources": [{"url": "https://firm.example/rules", "accessed_at": "2026-09-30",
                        "title_note": "title not retrieved"}]})
}

fn rules(drawdown: &str) -> Value {
    json!({
        "max_position_qty": verified(json!({"contracts": 50})),
        "drawdown": verified(json!({"type": "trailing_eod", "amount": drawdown,
                                    "trailing_lock": {"mode": "floor_reaches_start"}})),
        "daily_loss_limit": verified(json!({"kind": "none"})),
        "trading_day_reset": verified(json!({"local_time": "17:00", "time_zone": "America/New_York"})),
    })
}

fn facts(id: PresetVersionId, number: i32, drawdown: &str) -> PresetFacts {
    let wire = rules(drawdown);
    PresetFacts {
        preset_version_id: id,
        firm_id: FirmId::from_uuid(Uuid::from_u128(0xf1)),
        firm_name: "Topstep".into(),
        plan_name: "Trading Combine".into(),
        phase: "evaluation".into(),
        size_amount: SizeAmount::new(Decimal::new(50_000, 0)).unwrap(),
        max_position_qty: MaxPositionQty::new(50).ok(),
        firm_platforms: vec![],
        is_current: true,
        preset_id: Uuid::from_u128(0x77),
        has_evaluation_target: false,
        max_position_cap: None,
        superseded_by: None,
        manual_only: false,
        release_version_number: number,
        release_published_at: Utc.with_ymd_and_hms(2026, 10, 1, 12, 0, 0).unwrap(),
        release_verified_on: NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
        rule_sets: vec![PresetRuleSet {
            firm_platform: None,
            rules: PresetRules::from_wire(&wire).unwrap(),
            wire,
        }],
    }
}

struct Env {
    app: axum::Router,
    db: support::TestDb,
    catalog: Catalog,
    store: PgAccountStore,
    v1: PresetVersionId,
    v2: PresetVersionId,
}

async fn env() -> Option<Env> {
    let db = support::db().await?;
    let catalog = Catalog::default();
    let (v1, v2) = (
        PresetVersionId::from_uuid(Uuid::now_v7()),
        PresetVersionId::from_uuid(Uuid::now_v7()),
    );
    catalog
        .0
        .lock()
        .unwrap()
        .insert(v1, facts(v1, 1, "2000.00"));
    let store = PgAccountStore::new(db.app.clone());
    let deps = AccountsDeps {
        store: Arc::new(store.clone()),
        rules: Arc::new(Rules),
        presets: Arc::new(catalog.clone()),
        venues: Arc::new(Venues::default()),
        routes: Arc::new(NoRoutes),
        time_zones: Arc::new(SaoPaulo),
        profiles: Arc::new(NoProfiles),
        step_up: Arc::new(Gate::default()),
        clock: Arc::new(SystemClock),
        production: false,
    };
    let accounts =
        AccountsHttpState::new(deps.clone(), Arc::new(CookieIsUser), ORIGIN.into(), false);
    let ops = AccountPresetsHttpState::new(deps, Arc::new(CookieIsUser), ORIGIN.into(), false);
    Some(Env {
        app: router(Arc::new(accounts)).merge(presets::router(Arc::new(ops))),
        db,
        catalog,
        store,
        v1,
        v2,
    })
}

impl Env {
    fn publish_v2(&self) {
        let mut c = self.catalog.0.lock().unwrap();
        let v1 = c.get_mut(&self.v1).unwrap();
        v1.is_current = false;
        v1.superseded_by = Some(self.v2);
        c.insert(self.v2, facts(self.v2, 2, "2500.00"));
    }
}

async fn send(
    env: &Env,
    method: &str,
    uri: &str,
    user: Uuid,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut req = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, format!("__Host-aw_session={user}"))
        .header(header::ORIGIN, ORIGIN);
    if method == "POST" && uri == "/api/v1/accounts" {
        req = req.header("idempotency-key", Uuid::now_v7().to_string());
    }
    let body = match body {
        Some(b) => {
            req = req.header(header::CONTENT_TYPE, "application/json");
            Body::from(b.to_string())
        }
        None => Body::empty(),
    };
    let res = env
        .app
        .clone()
        .oneshot(req.body(body).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, value)
}

fn conforms(value: &Value, component: &str) {
    let validator = jsonschema::validator_for(presets::component(component).unwrap()).unwrap();
    let errors: Vec<String> = validator
        .iter_errors(value)
        .map(|e| format!("{e} at {}", e.instance_path()))
        .collect();
    assert!(errors.is_empty(), "{component}: {errors:?}\n{value:#}");
}

static AUDIT_DETAILS: &str = include_str!("preset_audit_details.json");

/// The user's `presets.*` outbox rows: the common audit envelope, contract details, and
/// `FieldChange`s that name only preset fields.
async fn preset_audit(env: &Env, user: Uuid) -> Vec<(String, Value)> {
    let schemas: Value = serde_json::from_str(AUDIT_DETAILS).unwrap();
    let rows = sqlx::query_as::<_, (String, Value)>(
        "SELECT event_type, payload FROM app.outbox
         WHERE user_id = $1 AND event_type LIKE 'presets.%' ORDER BY occurred_at, id",
    )
    .bind(user)
    .fetch_all(&env.db.admin)
    .await
    .unwrap();
    for (code, p) in &rows {
        assert_eq!(p["actor"], "user", "{code}");
        assert!(p["refs"]["account_id"].is_string() && p["refs"]["account_name"].is_string());
        let validator = jsonschema::validator_for(&schemas[code]).unwrap();
        let errors: Vec<String> = validator
            .iter_errors(&p["details"])
            .map(|e| e.to_string())
            .collect();
        assert!(errors.is_empty(), "{code}: {errors:?}\n{p:#}");
        let changes = p["details"]["changes"]
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![p["details"]["change"].clone()]);
        for c in changes {
            assert!(
                PRESET_AUDITED_FIELDS.contains(&c["field"].as_str().unwrap()),
                "{c}"
            );
        }
    }
    rows
}

async fn rows(env: &Env, account: &str) -> Vec<(String, String)> {
    sqlx::query_as::<_, (String, String)>(
        "SELECT field, origin FROM accounts.account_rule_value WHERE account_id = $1::uuid ORDER BY field",
    )
    .bind(account)
    .fetch_all(&env.db.admin)
    .await
    .unwrap()
}

async fn limits_version(env: &Env, account: &str) -> i32 {
    sqlx::query_scalar("SELECT limits_version FROM accounts.account WHERE id = $1::uuid")
        .bind(account)
        .fetch_one(&env.db.admin)
        .await
        .unwrap()
}

/// A Topstep account on v1 with its drawdown edited (1500) and its maximum (3 < 50).
async fn preset_account(env: &Env, user: Uuid) -> Value {
    let (status, a) = send(
        env,
        "POST",
        "/api/v1/accounts",
        user,
        Some(
            json!({"kind": "prop", "display_name": "Combine", "preset_version_id": env.v1.as_uuid(),
                    "max_position_qty": 3,
                    "rule_values": {"drawdown": {"type": "trailing_eod", "amount": "1500.00"}}}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{a}");
    a
}

#[tokio::test]
async fn a_preset_account_is_read_compared_applied_and_restored() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let a = preset_account(&env, me).await;
    let id = a["id"].as_str().unwrap();
    assert_eq!(
        rows(&env, id).await,
        [
            ("drawdown".into(), "edited".into()),
            ("max_position_qty".into(), "edited".into())
        ]
    );

    let (status, state) = send(
        &env,
        "GET",
        &format!("/api/v1/accounts/{id}/preset"),
        me,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{state}");
    conforms(&state, "AccountPresetState");
    assert!(state.get("newer_version").is_none());

    env.publish_v2();
    let (_, state) = send(
        &env,
        "GET",
        &format!("/api/v1/accounts/{id}/preset"),
        me,
        None,
    )
    .await;
    conforms(&state, "AccountPresetState");
    assert_eq!(
        state["newer_version"]["preset_version_id"],
        env.v2.as_uuid().to_string()
    );

    let (status, diff) = send(
        &env,
        "GET",
        &format!(
            "/api/v1/accounts/{id}/preset-diff?target_preset_version_id={}",
            env.v2.as_uuid()
        ),
        me,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{diff}");
    conforms(&diff, "PresetDiff");
    assert_eq!(diff["rows"][0]["field"], "drawdown");
    assert_eq!(diff["rows"][0]["requires_choice"], true);

    let before = limits_version(&env, id).await;
    let apply = |choices: Value, row_version: &Value| {
        json!({"target_preset_version_id": env.v2.as_uuid(), "row_version": row_version,
               "choices": choices})
    };
    let (status, e) = send(
        &env,
        "POST",
        &format!("/api/v1/accounts/{id}/preset-version"),
        me,
        Some(apply(json!({}), &state["row_version"])),
    )
    .await;
    assert_eq!(
        (status, e["error"]["code"].as_str()),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Some("presets.choice_required")
        )
    );
    assert_eq!(e["error"]["pointer"], "/choices/drawdown");

    let (status, applied) = send(
        &env,
        "POST",
        &format!("/api/v1/accounts/{id}/preset-version"),
        me,
        Some(apply(
            json!({"drawdown": "use_preset"}),
            &state["row_version"],
        )),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{applied}");
    conforms(&applied, "AccountPresetState");
    assert_eq!(applied["preset_version_id"], env.v2.as_uuid().to_string());
    assert_eq!(
        rows(&env, id).await,
        [("max_position_qty".into(), "edited".into())]
    );
    assert_eq!(limits_version(&env, id).await, before + 1);

    let (status, restored) = send(
        &env,
        "POST",
        &format!("/api/v1/accounts/{id}/preset-values/max_position_qty/restore"),
        me,
        Some(json!({"row_version": applied["row_version"]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{restored}");
    conforms(&restored, "AccountPresetState");
    assert!(rows(&env, id).await.is_empty());
    let (_, account) = send(&env, "GET", &format!("/api/v1/accounts/{id}"), me, None).await;
    assert_eq!(account["max_position_qty"], 50);

    let codes: Vec<String> = preset_audit(&env, me)
        .await
        .into_iter()
        .map(|(c, _)| c)
        .collect();
    assert_eq!(codes, ["presets.version_applied", "presets.value_restored"]);
}

#[tokio::test]
async fn every_operation_answers_404_for_another_users_account() {
    let Some(env) = env().await else { return };
    let (me, other) = (support::user(&env.db).await, support::user(&env.db).await);
    let a = preset_account(&env, me).await;
    let id = a["id"].as_str().unwrap();
    env.publish_v2();
    let calls = [
        ("GET", format!("/api/v1/accounts/{id}/preset"), None),
        (
            "GET",
            format!(
                "/api/v1/accounts/{id}/preset-diff?target_preset_version_id={}",
                env.v2.as_uuid()
            ),
            None,
        ),
        (
            "POST",
            format!("/api/v1/accounts/{id}/preset-version"),
            Some(
                json!({"target_preset_version_id": env.v2.as_uuid(), "row_version": 1,
                        "choices": {"drawdown": "use_preset"}}),
            ),
        ),
        (
            "POST",
            format!("/api/v1/accounts/{id}/preset-values/drawdown/restore"),
            Some(json!({"row_version": 1})),
        ),
    ];
    for (method, uri, body) in calls {
        let (status, e) = send(&env, method, &uri, other, body).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{method} {uri}: {e}");
        assert_eq!(e["error"]["code"], "common.not_found");
    }
    assert_eq!(rows(&env, id).await.len(), 2, "nothing changed");
    assert!(preset_audit(&env, me).await.is_empty());
    assert!(preset_audit(&env, other).await.is_empty());
}

#[tokio::test]
async fn malformed_requests_are_400_with_their_pointer() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let a = preset_account(&env, me).await;
    let id = a["id"].as_str().unwrap();
    let cases = [
        (
            "GET",
            format!("/api/v1/accounts/{id}/preset-diff"),
            None,
            "/query/target_preset_version_id",
        ),
        (
            "POST",
            format!("/api/v1/accounts/{id}/preset-values/commission/restore"),
            Some(json!({"row_version": 1})),
            "/path/field",
        ),
        (
            "POST",
            format!("/api/v1/accounts/{id}/preset-values/drawdown/restore"),
            Some(json!({})),
            "/row_version",
        ),
    ];
    for (method, uri, body, pointer) in cases {
        let (status, e) = send(&env, method, &uri, me, body).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}: {e}");
        assert_eq!(e["error"]["pointer"], pointer, "{uri}");
    }
}

/// S13's `putAccountRiskRules` writes rule values without `row_version` (5.4.0): a save with rows
/// computed from an older `limits_version` is refused, and nothing is written.
#[tokio::test]
async fn a_save_with_rule_values_requires_the_limits_version_it_read() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let a = preset_account(&env, me).await;
    let id = AccountId::from_uuid(a["id"].as_str().unwrap().parse().unwrap());
    let user = UserId::from_uuid(me);
    let stored = env.store.get(user, id).await.unwrap().unwrap();
    // A concurrent rule-value write bumped the limits version.
    sqlx::query("UPDATE accounts.account SET limits_version = limits_version + 1 WHERE id = $1")
        .bind(id.as_uuid())
        .execute(&env.db.admin)
        .await
        .unwrap();
    let write = |rule_values: Vec<RuleValueWrite>| AccountWrite {
        account: stored.clone(),
        expected_row_version: stored.row_version,
        expected_limits_version: stored.limits_version,
        commission: CommissionWrite::Unchanged,
        rule_values,
        events: vec![],
    };
    let refused = env
        .store
        .save(write(vec![RuleValueWrite {
            field: RuleField::Drawdown,
            row: None,
        }]))
        .await;
    assert!(
        matches!(refused, Err(StoreError::VersionConflict)),
        "{refused:?}"
    );
    assert_eq!(env.store.rule_values(user, id).await.unwrap().len(), 2);

    // Without rule values only row_version guards the save (S5 behaviour unchanged).
    env.store.save(write(vec![])).await.unwrap();
    let rows = env.store.rule_values(user, id).await.unwrap();
    assert!(
        rows.iter()
            .any(|r| r.field == RuleField::Drawdown && r.origin == RuleOrigin::Edited)
    );
}
