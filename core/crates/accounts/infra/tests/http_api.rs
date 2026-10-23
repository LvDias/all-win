//! The accounts HTTP operations end to end: router, use cases and the PostgreSQL store (RLS on);
//! other modules' ports are fakes. Every endpoint has a cross-user test (404).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::sync::Arc;

use accounts_application::enterprise::{ConnectionId, VenueAccountId};
use accounts_application::ports::{VenueAccountFacts, VenueKind};
use accounts_application::testing::*;
use accounts_application::{AUDITED_FIELDS, AccountsDeps};
use accounts_infra::PgAccountStore;
use accounts_infra::http::{AccountsHttpState, router, schemas};
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use shared_kernel::{SystemClock, UserId};
use tower::ServiceExt;
use uuid::Uuid;

const ORIGIN: &str = "https://app.test";
const AUTOMATION_V1: &str = "019a1000-0000-7000-8000-000000000001";
const COPY_V1: &str = "019a1000-0000-7000-8000-000000000002";

struct Env {
    app: axum::Router,
    db: support::TestDb,
    venues: Venues,
}

async fn env() -> Option<Env> {
    let db = support::db().await?;
    let venues = Venues::default();
    let deps = AccountsDeps {
        store: Arc::new(PgAccountStore::new(db.app.clone())),
        rules: Arc::new(GenericRules),
        presets: Arc::new(NoPresets),
        venues: Arc::new(venues.clone()),
        routes: Arc::new(NoRoutes),
        time_zones: Arc::new(SaoPaulo),
        profiles: Arc::new(NoProfiles),
        step_up: Arc::new(Gate::default()),
        clock: Arc::new(SystemClock),
        production: false,
    };
    let state = AccountsHttpState::new(deps, Arc::new(CookieIsUser), ORIGIN.into(), false);
    Some(Env {
        app: router(Arc::new(state)),
        db,
        venues,
    })
}

struct Call<'a> {
    method: &'a str,
    uri: String,
    user: Option<Uuid>,
    origin: Option<&'a str>,
    key: Option<String>,
    body: Option<Value>,
}

impl<'a> Call<'a> {
    fn new(method: &'a str, uri: impl Into<String>, user: Uuid) -> Self {
        Self {
            method,
            uri: uri.into(),
            user: Some(user),
            origin: Some(ORIGIN),
            key: None,
            body: None,
        }
    }

    fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    fn key(mut self, key: &str) -> Self {
        self.key = Some(key.to_owned());
        self
    }
}

async fn send(env: &Env, c: Call<'_>) -> (StatusCode, Value) {
    let mut req = Request::builder().method(c.method).uri(&c.uri);
    if let Some(u) = c.user {
        req = req.header(header::COOKIE, format!("__Host-aw_session={u}"));
    }
    if let Some(o) = c.origin {
        req = req.header(header::ORIGIN, o);
    }
    if let Some(k) = c.key {
        req = req.header("idempotency-key", k);
    }
    let body = match c.body {
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

/// Response bodies must satisfy the contract schema (DTO ↔ contract).
fn conforms(value: &Value, component: &str) {
    let schema = schemas::component(component).unwrap();
    let validator = jsonschema::validator_for(schema).unwrap();
    let errors: Vec<String> = validator
        .iter_errors(value)
        .map(|e| format!("{} at {}", e, e.instance_path()))
        .collect();
    assert!(errors.is_empty(), "{component}: {errors:?}\n{value:#}");
}

fn real(name: &str) -> Value {
    json!({"kind": "real", "display_name": name, "size_amount": "25000.00", "max_position_qty": 3})
}

fn prop(name: &str, choice: Option<&str>) -> Value {
    let mut v = json!({"kind": "prop", "display_name": name, "firm_name": "Apex",
                       "plan_name": "50K", "phase": "Evaluation", "size_amount": "50000.00",
                       "max_position_qty": 2});
    if let Some(c) = choice {
        v["opt_ins"] = json!({"automation": {"choice": c, "rule_version_id": AUTOMATION_V1}});
    }
    v
}

async fn create(env: &Env, user: Uuid, body: Value) -> (StatusCode, Value) {
    let key = Uuid::now_v7().to_string();
    send(
        env,
        Call::new("POST", "/api/v1/accounts", user)
            .key(&key)
            .body(body),
    )
    .await
}

fn edit_body(account: &Value) -> Value {
    let mut b = json!({
        "row_version": account["row_version"],
        "display_name": account["display_name"],
        "size_amount": account["size_amount"],
        "max_position_qty": account["max_position_qty"],
        "multiplier": account["multiplier"],
    });
    if account["kind"] == "prop" {
        for f in ["firm_name", "plan_name", "phase"] {
            b[f] = account[f].clone();
        }
    }
    b
}

async fn outbox(env: &Env, user: Uuid) -> Vec<(String, Value)> {
    sqlx::query_as::<_, (String, Value)>(
        "SELECT event_type, payload FROM app.outbox WHERE user_id = $1 ORDER BY occurred_at, id",
    )
    .bind(user)
    .fetch_all(&env.db.admin)
    .await
    .unwrap()
}

/// `AuditDetailsByCode[accounts.*]` resolved from the contract bundle (evaluation_restarted:
/// the 4.4.0 `DetailsEvaluationRestarted` announced by Compasso; regenerate when 4.4.0 lands).
static AUDIT_DETAILS: &str = include_str!("audit_details.json");

/// Every `accounts.*` row of the user's outbox is in the common audit envelope, its `details`
/// match the contract and its `FieldChange`s name only `AUDITED_FIELDS`. Returns the codes seen.
async fn audit_conforms(env: &Env, user: Uuid) -> Vec<String> {
    let schemas: Value = serde_json::from_str(AUDIT_DETAILS).unwrap();
    let mut seen = Vec::new();
    for (code, p) in outbox(env, user).await {
        if !code.starts_with("accounts.") {
            continue;
        }
        let keys: Vec<&String> = p.as_object().unwrap().keys().collect();
        assert_eq!(keys, ["actor", "details", "refs"], "{code}: {p}");
        assert_eq!(p["actor"], "user", "{code}");
        let refs = p["refs"].as_object().unwrap();
        assert!(
            refs.len() == 2 && refs["account_id"].is_string() && refs["account_name"].is_string(),
            "{code}: {p}"
        );
        let validator = jsonschema::validator_for(&schemas[&code]).unwrap();
        let errors: Vec<String> = validator
            .iter_errors(&p["details"])
            .map(|e| format!("{e} at {}", e.instance_path()))
            .collect();
        assert!(errors.is_empty(), "{code}: {errors:?}\n{p:#}");
        for change in p["details"]["changes"].as_array().into_iter().flatten() {
            let field = change["field"].as_str().unwrap();
            assert!(
                AUDITED_FIELDS.contains(&field),
                "{code}: field {field} not in the closed list"
            );
        }
        seen.push(code);
    }
    seen
}

/// Payloads of the account's `account.limits_changed` / `account.binding_changed` rows, in order.
async fn binding_events(env: &Env, user: Uuid, id: &str) -> Vec<Value> {
    outbox(env, user)
        .await
        .into_iter()
        .filter(|(t, p)| {
            matches!(
                t.as_str(),
                "account.limits_changed" | "account.binding_changed"
            ) && p["account_id"] == id
        })
        .map(|(_, p)| p)
        .collect()
}

#[tokio::test]
async fn real_account_round_trip_matches_the_contract() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let (s, created) = create(&env, me, real("Conta real")).await;
    assert_eq!(s, StatusCode::CREATED, "{created}");
    conforms(&created, "Account");
    assert_eq!(created["size_amount"], "25000.00");
    assert_eq!(created["multiplier"], 1);
    assert_eq!(
        created["opt_ins"]["automation"],
        json!({"kind": "automation", "state": "not_required"})
    );
    let id = created["id"].as_str().unwrap();

    let (s, got) = send(&env, Call::new("GET", format!("/api/v1/accounts/{id}"), me)).await;
    assert_eq!(s, StatusCode::OK);
    conforms(&got, "Account");
    let (s, list) = send(&env, Call::new("GET", "/api/v1/accounts", me)).await;
    assert_eq!(s, StatusCode::OK);
    conforms(&list, "AccountList");
    assert_eq!(list["accounts"].as_array().unwrap().len(), 1);

    let events: Vec<String> = outbox(&env, me).await.into_iter().map(|e| e.0).collect();
    assert_eq!(
        events,
        [
            "accounts.created",
            "account.limits_changed",
            "account.changed"
        ]
    );
}

#[tokio::test]
async fn create_replays_by_key_and_never_creates_twice() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let key = Uuid::now_v7().to_string();
    let call = |body| {
        Call::new("POST", "/api/v1/accounts", me)
            .key(&key)
            .body(body)
    };
    let (s, first) = send(&env, call(real("A"))).await;
    assert_eq!(s, StatusCode::CREATED);
    // Same body with another key order and a step_up: same request.
    let reordered = json!({"max_position_qty": 3, "size_amount": "25000.00", "display_name": "A",
                           "kind": "real", "step_up": {"challenge_id": "0f0e0d0c-0b0a-4908-8706-050403020100", "code": "654321"}});
    let (s, again) = send(&env, call(reordered)).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(again["id"], first["id"]);
    let (s, e) = send(&env, call(real("B"))).await;
    assert_eq!(s, StatusCode::CONFLICT);
    assert_eq!(e["error"]["code"], "accounts.idempotency_key_reused");
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM accounts.account WHERE user_id = $1")
        .bind(me)
        .fetch_one(&env.db.admin)
        .await
        .unwrap();
    assert_eq!(n, 1);
    let (s, _) = send(
        &env,
        Call::new("POST", "/api/v1/accounts", me).body(real("C")),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST, "Idempotency-Key is required");
}

#[tokio::test]
async fn request_errors_follow_the_contract_precedence() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let mut anonymous = Call::new("GET", "/api/v1/accounts", me);
    anonymous.user = None;
    assert_eq!(send(&env, anonymous).await.0, StatusCode::UNAUTHORIZED);
    let mut foreign = Call::new("POST", "/api/v1/accounts", me)
        .key(&Uuid::now_v7().to_string())
        .body(real("x"));
    foreign.origin = Some("https://evil.test");
    let (s, e) = send(&env, foreign).await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (StatusCode::FORBIDDEN, Some("common.forbidden_origin"))
    );

    let cases = [
        (
            json!({"kind": "real", "display_name": "   ", "size_amount": "1.00", "max_position_qty": 1}),
            "/display_name",
        ),
        (
            json!({"kind": "real", "display_name": "x", "size_amount": "1.00", "max_position_qty": 1, "firm_name": "Apex"}),
            "/firm_name",
        ),
        (
            json!({"kind": "real", "display_name": "x", "size_amount": "0", "max_position_qty": 1}),
            "/size_amount",
        ),
        (
            json!({"kind": "real", "display_name": "x", "size_amount": "1.00", "max_position_qty": 1, "multiplier": 2.5}),
            "/multiplier",
        ),
        (
            json!({"kind": "real", "display_name": "x", "max_position_qty": 1}),
            "/size_amount",
        ),
        (
            json!({"kind": "real", "display_name": "x", "size_amount": "1.00"}),
            "/max_position_qty",
        ),
        (
            json!({"kind": "real", "display_name": "x", "size_amount": "1.00", "max_position_qty": 0}),
            "/max_position_qty",
        ),
        (
            json!({"kind": "prop", "display_name": "x", "firm_name": "A", "plan_name": "B", "phase": "C",
                "size_amount": "1.00", "max_position_qty": 1, "evaluation_start": "2026-02-30"}),
            "/evaluation_start",
        ),
    ];
    for (body, pointer) in cases {
        let (s, e) = create(&env, me, body).await;
        assert_eq!(s, StatusCode::BAD_REQUEST, "{pointer}: {e}");
        assert_eq!(e["error"]["pointer"], pointer, "{e}");
    }
    let (s, e) = send(&env, Call::new("GET", "/api/v1/accounts/NOT-A-UUID", me)).await;
    assert_eq!(
        (s, e["error"]["pointer"].as_str()),
        (StatusCode::BAD_REQUEST, Some("/path/account_id"))
    );
    let (s, e) = send(
        &env,
        Call::new("GET", format!("/api/v1/accounts/{}", Uuid::now_v7()), me),
    )
    .await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (StatusCode::NOT_FOUND, Some("common.not_found"))
    );

    // Name taken (case-insensitive) is a 409 even when a 422 also applies.
    create(&env, me, real("Apex 1")).await;
    let (s, e) = create(&env, me, prop("apex 1", None)).await;
    assert_eq!(s, StatusCode::CONFLICT);
    assert_eq!(
        e["error"],
        json!({"code": "accounts.name_taken", "pointer": "/display_name"})
    );
}

#[tokio::test]
async fn prop_opt_in_flow() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let (s, e) = create(&env, me, prop("P", None)).await;
    assert_eq!(s, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        e["error"],
        json!({"code": "accounts.opt_in_required", "pointer": "/opt_ins/automation"})
    );

    let (s, e) = create(&env, me, prop("P", Some("accepted"))).await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (StatusCode::FORBIDDEN, Some("auth.step_up_required"))
    );

    let (s, acc) = create(&env, me, prop("P", Some("declined"))).await;
    assert_eq!(s, StatusCode::CREATED, "{acc}");
    conforms(&acc, "Account");
    let auto = &acc["opt_ins"]["automation"];
    assert_eq!(auto["state"], "chosen");
    assert_eq!(auto["choice"], "declined");
    assert_eq!(auto["firm_stance"], "unknown");
    assert_eq!(auto["current_rule_version_id"], AUTOMATION_V1);
    assert_eq!(acc["opt_ins"]["copy"]["state"], "not_required");
    assert_eq!(acc["opt_ins"]["copy"]["current_rule_version_id"], COPY_V1);
    let id = acc["id"].as_str().unwrap();
    let uri = format!("/api/v1/accounts/{id}/opt-ins/automation");

    let (s, e) = send(
        &env,
        Call::new("PUT", &uri, me)
            .body(json!({"choice": "accepted", "rule_version_id": AUTOMATION_V1})),
    )
    .await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (StatusCode::FORBIDDEN, Some("auth.step_up_required"))
    );
    let stale = Uuid::now_v7().to_string();
    let (s, e) = send(
        &env,
        Call::new("PUT", &uri, me).body(json!({"choice": "accepted", "rule_version_id": stale,
        "step_up": {"challenge_id": "0f0e0d0c-0b0a-4908-8706-050403020100", "code": "123456"}})),
    )
    .await;
    assert_eq!(s, StatusCode::CONFLICT);
    assert_eq!(e["error"]["current_rule_version_id"], AUTOMATION_V1);
    let (s, st) = send(
        &env,
        Call::new("PUT", &uri, me).body(
            json!({"choice": "accepted", "rule_version_id": AUTOMATION_V1,
        "step_up": {"challenge_id": "0f0e0d0c-0b0a-4908-8706-050403020100", "code": "123456"}}),
        ),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{st}");
    conforms(&st, "OptInState");
    assert_eq!(st["choice"], "accepted");
    let (s, e) = send(
        &env,
        Call::new("PUT", format!("/api/v1/accounts/{id}/opt-ins/copy"), me)
            .body(json!({"choice": "declined", "rule_version_id": COPY_V1})),
    )
    .await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Some("accounts.opt_in_not_applicable")
        )
    );

    let audit: Vec<(String, Value)> = outbox(&env, me).await;
    let chosen: Vec<&Value> = audit
        .iter()
        .filter(|e| e.0 == "accounts.opt_in_chosen")
        .map(|e| &e.1)
        .collect();
    assert_eq!(chosen.len(), 2);
    assert_eq!(chosen[1]["details"]["previous_choice"], "declined");
    assert_eq!(chosen[1]["details"]["rule_version_id"], AUTOMATION_V1);
    assert_eq!(chosen[1]["refs"]["account_name"], "P");
    let codes = audit_conforms(&env, me).await;
    assert!(codes.iter().any(|c| c == "accounts.opt_in_chosen"));
}

#[tokio::test]
async fn update_commission_and_limits_version() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let mut body = real("A");
    body["commission_per_contract_side"] = json!("0.37");
    let (_, acc) = create(&env, me, body).await;
    assert_eq!(acc["commission_per_contract_side"], "0.37");
    let id = acc["id"].as_str().unwrap().to_owned();
    let uri = format!("/api/v1/accounts/{id}");

    // Absent: unchanged (and nothing else changed: not audited).
    let (s, same) = send(&env, Call::new("PUT", &uri, me).body(edit_body(&acc))).await;
    assert_eq!(s, StatusCode::OK, "{same}");
    assert_eq!(same["row_version"], 1);
    assert_eq!(same["commission_per_contract_side"], "0.37");

    let mut b = edit_body(&acc);
    b["commission_per_contract_side"] = Value::Null;
    b["max_position_qty"] = json!(5);
    let (s, after) = send(&env, Call::new("PUT", &uri, me).body(b)).await;
    assert_eq!(s, StatusCode::OK, "{after}");
    conforms(&after, "Account");
    assert!(after.get("commission_per_contract_side").is_none());
    let (unset, value): (bool, Option<Value>) = sqlx::query_as(
        "SELECT is_unset, value FROM accounts.account_rule_value
          WHERE account_id = $1 AND field = 'commission_per_contract_side'",
    )
    .bind(Uuid::parse_str(&id).unwrap())
    .fetch_one(&env.db.admin)
    .await
    .unwrap();
    assert!(unset && value.is_none(), "removal is a tombstone");
    let lv: i32 = sqlx::query_scalar("SELECT limits_version FROM accounts.account WHERE id = $1")
        .bind(Uuid::parse_str(&id).unwrap())
        .fetch_one(&env.db.admin)
        .await
        .unwrap();
    assert_eq!(lv, 2);
    let events = outbox(&env, me).await;
    let updated = events.iter().find(|e| e.0 == "accounts.updated").unwrap();
    assert!(
        updated.1["details"]["changes"]
            .as_array()
            .unwrap()
            .contains(&json!({"field": "commission_per_contract_side", "old": "0.37"}))
    );
    assert!(
        audit_conforms(&env, me)
            .await
            .iter()
            .any(|c| c == "accounts.updated")
    );
    assert_eq!(
        events
            .iter()
            .rfind(|e| e.0 == "account.limits_changed")
            .unwrap()
            .1["limits_version"],
        2
    );

    let mut stale = edit_body(&acc);
    stale["display_name"] = json!("B");
    let (s, e) = send(&env, Call::new("PUT", &uri, me).body(stale)).await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (StatusCode::CONFLICT, Some("common.version_conflict"))
    );

    let mut wrong_kind = edit_body(&after);
    wrong_kind["firm_name"] = json!("Apex");
    let (s, e) = send(&env, Call::new("PUT", &uri, me).body(wrong_kind)).await;
    assert_eq!(s, StatusCode::BAD_REQUEST, "{e}");
}

#[tokio::test]
async fn binding_lifecycle_and_archive() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let connection = ConnectionId::from_uuid(Uuid::now_v7());
    let venue = VenueAccountId::from_uuid(Uuid::now_v7());
    env.venues.known.lock().unwrap().insert(
        (UserId::from_uuid(me), connection, "Sim101".into()),
        VenueAccountFacts {
            venue_account_id: venue,
            kind: VenueKind::Simulation,
        },
    );
    let mut body = real("Bound");
    body["binding"] = json!({"connection_id": connection.as_uuid(), "platform_account": "Sim101"});
    let (s, acc) = create(&env, me, body.clone()).await;
    assert_eq!(s, StatusCode::CREATED, "{acc}");
    conforms(&acc, "Account");
    assert_eq!(acc["binding"]["platform_account"], "Sim101");
    let id = acc["id"].as_str().unwrap();
    // Write path (ADR-0006/0008): the create transaction wrote both events.
    assert_eq!(
        binding_events(&env, me, id).await,
        [
            json!({"kind": "account.limits_changed", "account_id": id, "limits_version": 1}),
            json!({"kind": "account.binding_changed", "account_id": id, "venue_account_id": venue.as_uuid(), "limits_version": 1}),
        ]
    );
    body["display_name"] = json!("Other");
    let (s, e) = create(&env, me, body).await;
    assert_eq!(
        e["error"],
        json!({"code": "accounts.platform_account_taken", "pointer": "/binding"}),
        "{s}"
    );

    let lc = format!("/api/v1/accounts/{id}/lifecycle");
    let (s, paused) = send(
        &env,
        Call::new("PUT", &lc, me).body(json!({"state": "paused", "row_version": 1})),
    )
    .await;
    assert_eq!(
        (s, paused["lifecycle_state"].as_str()),
        (StatusCode::OK, Some("paused"))
    );
    let (s, e) = send(
        &env,
        Call::new("PUT", &lc, me).body(json!({"state": "disabled", "row_version": 2})),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{e}");
    let (s, e) = send(
        &env,
        Call::new("PUT", &lc, me).body(json!({"state": "paused", "row_version": 3})),
    )
    .await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (
            StatusCode::CONFLICT,
            Some("accounts.lifecycle_transition_invalid")
        )
    );

    env.venues.open.lock().unwrap().push(venue);
    let (s, impact) = send(
        &env,
        Call::new("GET", format!("/api/v1/accounts/{id}/archive-impact"), me),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    conforms(&impact, "ArchiveImpact");
    assert_eq!(
        impact,
        json!({"has_open_position": true, "routes": [], "follower_links": []})
    );
    let (s, e) = send(
        &env,
        Call::new("DELETE", format!("/api/v1/accounts/{id}"), me),
    )
    .await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (StatusCode::CONFLICT, Some("accounts.delete_open_position"))
    );
    env.venues.open.lock().unwrap().clear();
    let (s, _) = send(
        &env,
        Call::new("DELETE", format!("/api/v1/accounts/{id}"), me),
    )
    .await;
    assert_eq!(s, StatusCode::NO_CONTENT);
    // Archiving releases the binding: a limits bump and both events again.
    assert_eq!(
        binding_events(&env, me, id).await[2..],
        [
            json!({"kind": "account.limits_changed", "account_id": id, "limits_version": 2}),
            json!({"kind": "account.binding_changed", "account_id": id, "venue_account_id": null, "limits_version": 2}),
        ]
    );
    let (s, _) = send(&env, Call::new("GET", format!("/api/v1/accounts/{id}"), me)).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    // The name and the platform account are free again.
    let mut again = real("bound");
    again["binding"] = json!({"connection_id": connection.as_uuid(), "platform_account": "Sim101"});
    let (s, e) = create(&env, me, again).await;
    assert_eq!(s, StatusCode::CREATED, "{e}");
    let archived = outbox(&env, me)
        .await
        .into_iter()
        .find(|e| e.0 == "accounts.archived")
        .unwrap();
    assert_eq!(
        archived.1["details"]["changes"],
        json!([{"field": "binding", "old": "Sim101"}])
    );
    let codes = audit_conforms(&env, me).await;
    for code in [
        "accounts.created",
        "accounts.lifecycle_changed",
        "accounts.archived",
    ] {
        assert!(codes.iter().any(|c| c == code), "{code}");
    }
}

#[tokio::test]
async fn restart_evaluation_needs_a_target() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let (_, acc) = create(&env, me, prop("E", Some("declined"))).await;
    let id = acc["id"].as_str().unwrap();
    let uri = format!("/api/v1/accounts/{id}/evaluation/restart");
    let (s, e) = send(
        &env,
        Call::new("POST", &uri, me)
            .body(json!({"evaluation_start": "2026-10-01", "row_version": 1})),
    )
    .await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Some("accounts.evaluation_not_applicable")
        )
    );
    sqlx::query(
        "INSERT INTO accounts.account_rule_value (user_id, account_id, field, origin, value)
         VALUES ($1, $2, 'evaluation_profit_target', 'set_by_user', '{\"amount\": \"3000.00\"}')",
    )
    .bind(me)
    .bind(Uuid::parse_str(id).unwrap())
    .execute(&env.db.admin)
    .await
    .unwrap();
    // As if created with an evaluation preset (S16): the start defaults to the creation date.
    sqlx::query("UPDATE accounts.account SET evaluation_start = '2026-09-01' WHERE id = $1")
        .bind(Uuid::parse_str(id).unwrap())
        .execute(&env.db.admin)
        .await
        .unwrap();
    let (s, after) = send(
        &env,
        Call::new("POST", &uri, me).body(json!({"evaluation_start": "2026-10-01",
        "evaluation_earned_before": "120.50", "row_version": 1})),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{after}");
    conforms(&after, "Account");
    assert_eq!(after["evaluation_start"], "2026-10-01");
    assert_eq!(after["evaluation_earned_before"], "120.50");
    let restarted = outbox(&env, me)
        .await
        .into_iter()
        .find(|e| e.0 == "accounts.evaluation_restarted")
        .unwrap();
    assert_eq!(
        restarted.1["details"],
        json!({"start": "2026-10-01", "previous_start": "2026-09-01",
               "profit_target": "3000.00", "earned_before": "120.50"})
    );
    assert!(
        audit_conforms(&env, me)
            .await
            .iter()
            .any(|c| c == "accounts.evaluation_restarted")
    );
    let (s, e) = send(
        &env,
        Call::new("POST", &uri, me)
            .body(json!({"evaluation_start": "2999-01-01", "row_version": 2})),
    )
    .await;
    assert_eq!(
        (s, e["error"]["code"].as_str()),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Some("accounts.evaluation_start_invalid")
        )
    );
}

#[tokio::test]
async fn another_users_account_is_not_found_on_every_endpoint() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let them = support::user(&env.db).await;
    let (_, acc) = create(&env, me, prop("Mine", Some("declined"))).await;
    let id = acc["id"].as_str().unwrap();
    let base = format!("/api/v1/accounts/{id}");
    let calls = [
        Call::new("GET", &base, them),
        Call::new("PUT", &base, them).body(edit_body(&acc)),
        Call::new("DELETE", &base, them),
        Call::new("GET", format!("{base}/archive-impact"), them),
        Call::new("PUT", format!("{base}/lifecycle"), them).body(json!({"state": "paused", "row_version": 1})),
        Call::new("POST", format!("{base}/evaluation/restart"), them)
            .body(json!({"evaluation_start": "2026-10-01", "row_version": 1})),
        Call::new("PUT", format!("{base}/opt-ins/automation"), them).body(json!({"choice": "accepted",
            "rule_version_id": AUTOMATION_V1, "step_up": {"challenge_id": "0f0e0d0c-0b0a-4908-8706-050403020100", "code": "123456"}})),
    ];
    for c in calls {
        let label = format!("{} {}", c.method, c.uri);
        let (s, e) = send(&env, c).await;
        assert_eq!(s, StatusCode::NOT_FOUND, "{label}: {e}");
        assert_eq!(e["error"]["code"], "common.not_found");
    }
    let (_, list) = send(&env, Call::new("GET", "/api/v1/accounts", them)).await;
    assert_eq!(list, json!({"accounts": []}));
    // Untouched.
    let (_, mine) = send(&env, Call::new("GET", &base, me)).await;
    assert_eq!(mine["row_version"], 1);
    assert_eq!(mine["lifecycle_state"], "active");
}

#[tokio::test]
async fn rls_is_fail_closed_without_a_user_context() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    create(&env, me, real("Hidden")).await;
    let visible: i64 = sqlx::query_scalar("SELECT count(*) FROM accounts.account")
        .fetch_one(&env.db.app)
        .await
        .unwrap();
    assert_eq!(visible, 0);
    let mut tx = env.db.app.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(Uuid::now_v7().to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let other: i64 = sqlx::query_scalar("SELECT count(*) FROM accounts.account")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(other, 0, "another user's context sees nothing");
    let denied = sqlx::query("DELETE FROM accounts.account")
        .execute(&mut *tx)
        .await;
    assert!(denied.is_err(), "allwin_app cannot delete accounts");
}

#[tokio::test]
async fn a_refused_write_leaves_no_row_and_no_event() {
    use accounts_application::enterprise::{AccountId, Binding, MoneyAmount, RuleOrigin};
    use accounts_application::events::AccountsEvent;
    use accounts_application::ports::{AccountStore, AccountWrite, CommissionWrite, StoreError};

    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let connection = ConnectionId::from_uuid(Uuid::now_v7());
    let venue = VenueAccountId::from_uuid(Uuid::now_v7());
    env.venues.known.lock().unwrap().insert(
        (UserId::from_uuid(me), connection, "Sim101".into()),
        VenueAccountFacts {
            venue_account_id: venue,
            kind: VenueKind::Simulation,
        },
    );
    let mut bound = real("Bound");
    bound["binding"] = json!({"connection_id": connection.as_uuid(), "platform_account": "Sim101"});
    assert_eq!(create(&env, me, bound).await.0, StatusCode::CREATED);
    let (s, other) = create(&env, me, real("Other")).await;
    assert_eq!(s, StatusCode::CREATED);
    let id = AccountId::from_uuid(Uuid::parse_str(other["id"].as_str().unwrap()).unwrap());
    let before = outbox(&env, me).await.len();

    // A concurrent bind of the same platform account (past the use case's check): the unique
    // index refuses it and the whole transaction rolls back.
    let store = PgAccountStore::new(env.db.app.clone());
    let user = UserId::from_uuid(me);
    let stored = store.get(user, id).await.unwrap().unwrap();
    let mut next = stored.clone();
    let now = chrono::Utc::now();
    next.binding = Some(Binding {
        venue_account_id: venue,
        bound_at: now,
    });
    next.commission_per_contract_side = Some(MoneyAmount::new("0.50".parse().unwrap()).unwrap());
    next.row_version += 1;
    next.limits_version += 1;
    let events = vec![
        AccountsEvent::LimitsChanged {
            account_id: id,
            at: now,
            limits_version: next.limits_version,
        },
        AccountsEvent::BindingChanged {
            account_id: id,
            at: now,
            venue_account_id: Some(venue),
            limits_version: next.limits_version,
        },
    ];
    let refused = store
        .save(AccountWrite {
            account: next,
            expected_row_version: stored.row_version,
            commission: CommissionWrite::Set(RuleOrigin::SetByUser),
            events,
        })
        .await;
    assert!(
        matches!(refused, Err(StoreError::BindingTaken)),
        "{refused:?}"
    );
    assert_eq!(outbox(&env, me).await.len(), before, "no event written");
    let after = store.get(user, id).await.unwrap().unwrap();
    assert_eq!(
        (
            after.binding,
            after.limits_version,
            after.commission_per_contract_side
        ),
        (None, 1, None)
    );
}
