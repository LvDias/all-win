//! The profile HTTP operations end to end: routers, use cases and the PostgreSQL stores (RLS on),
//! including the initial copy inside the account create. The catalog port is a fake with the
//! seeded MNQ row and the standard sizes. Every endpoint has a cross-user test (404).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::sync::Arc;

use accounts_application::AccountsDeps;
use accounts_application::enterprise::AccountId;
use accounts_application::enterprise::SizeAmount;
use accounts_application::enterprise::profile::TickSpec;
use accounts_application::ports::PortError;
use accounts_application::profiles::ports::{EnabledInstrument, ProfileCatalog};
use accounts_application::profiles::{BracketQuery, ProfileDeps, ProfilePlanner, ProfileUseCases};
use accounts_application::testing::*;
use accounts_infra::PgAccountStore;
use accounts_infra::http::{AccountsHttpState, router as accounts_router};
use accounts_infra::profiles::http::{ProfilesHttpState, router};
use accounts_infra::profiles::{PgProfileStore, schemas};
use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use rust_decimal::Decimal;
use serde_json::{Value, json};
use shared_kernel::{SystemClock, UserId};
use tower::ServiceExt;
use uuid::Uuid;

const ORIGIN: &str = "https://app.test";
/// Seeded by the catalog migration.
const MNQ: &str = "01a166e7-a8d0-700f-bade-6d8493132056";

struct Catalog;

#[async_trait]
impl ProfileCatalog for Catalog {
    async fn enabled_instruments(&self) -> Result<Vec<EnabledInstrument>, PortError> {
        Ok(vec![EnabledInstrument {
            id: Uuid::parse_str(MNQ).unwrap(),
            ticks: TickSpec::new(Decimal::new(25, 2), Decimal::new(2, 0)).unwrap(),
        }])
    }

    async fn standard_sizes(&self) -> Result<Vec<SizeAmount>, PortError> {
        Ok([25_000, 50_000, 100_000, 150_000, 200_000, 300_000]
            .map(|v| SizeAmount::new(Decimal::from(v)).unwrap())
            .to_vec())
    }
}

struct Env {
    app: axum::Router,
    db: support::TestDb,
    brackets: BracketQuery,
}

async fn env() -> Option<Env> {
    let db = support::db().await?;
    let store = Arc::new(PgProfileStore::new(db.app.clone()));
    let catalog = Arc::new(Catalog);
    let deps = AccountsDeps {
        store: Arc::new(PgAccountStore::new(db.app.clone())),
        rules: Arc::new(GenericRules),
        presets: Arc::new(NoPresets),
        venues: Arc::new(Venues::default()),
        routes: Arc::new(NoRoutes),
        time_zones: Arc::new(SaoPaulo),
        profiles: Arc::new(ProfilePlanner::new(store.clone(), catalog.clone())),
        step_up: Arc::new(Gate::default()),
        clock: Arc::new(SystemClock),
        production: false,
    };
    let accounts = AccountsHttpState::new(deps, Arc::new(CookieIsUser), ORIGIN.into(), false);
    let profiles = ProfileUseCases::new(ProfileDeps {
        store: store.clone(),
        catalog: catalog.clone(),
        clock: Arc::new(SystemClock),
    });
    let state = ProfilesHttpState::new(profiles, Arc::new(CookieIsUser), ORIGIN.into());
    Some(Env {
        app: accounts_router(Arc::new(accounts)).merge(router(Arc::new(state))),
        db,
        brackets: BracketQuery::new(store, catalog),
    })
}

async fn send(
    env: &Env,
    method: &str,
    uri: &str,
    user: Option<Uuid>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut req = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::ORIGIN, ORIGIN);
    if let Some(u) = user {
        req = req.header(header::COOKIE, format!("__Host-aw_session={u}"));
    }
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

async fn get(env: &Env, uri: &str, user: Uuid) -> (StatusCode, Value) {
    send(env, "GET", uri, Some(user), None).await
}

async fn put(env: &Env, uri: &str, user: Uuid, body: Value) -> (StatusCode, Value) {
    send(env, "PUT", uri, Some(user), Some(body)).await
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

fn error(v: &Value) -> (&str, &str) {
    (
        v["error"]["code"].as_str().unwrap(),
        v["error"]["pointer"].as_str().unwrap_or(""),
    )
}

fn default_uri(size: &str) -> String {
    format!("/api/v1/default-profiles/{MNQ}/{size}")
}

fn profile_uri(account: &str) -> String {
    format!("/api/v1/accounts/{account}/profiles/{MNQ}")
}

async fn create_real(env: &Env, user: Uuid, size: &str, pick: Option<&str>) -> (StatusCode, Value) {
    let mut body = json!({"kind": "real", "display_name": format!("Real {}", Uuid::now_v7().simple()),
                          "size_amount": size, "max_position_qty": 3});
    if let Some(p) = pick {
        body["profile_size"] = json!(p);
    }
    send(env, "POST", "/api/v1/accounts", Some(user), Some(body)).await
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

/// `AuditDetailsByCode[profiles.*]` resolved from the contract bundle.
static AUDIT_DETAILS: &str = include_str!("profiles_audit_details.json");

/// Every `profiles.*` outbox row is in the common audit envelope and its `details` match the
/// contract. Returns `(code, action)` in order.
async fn audit_conforms(env: &Env, user: Uuid) -> Vec<(String, String)> {
    let schemas: Value = serde_json::from_str(AUDIT_DETAILS).unwrap();
    let mut seen = Vec::new();
    for (code, p) in outbox(env, user).await {
        if !code.starts_with("profiles.") {
            continue;
        }
        let keys: Vec<&String> = p.as_object().unwrap().keys().collect();
        assert_eq!(keys, ["actor", "details", "refs"], "{code}: {p}");
        assert_eq!(p["actor"], "user", "{code}");
        let refs = p["refs"].as_object().unwrap();
        if code == "profiles.account_changed" {
            assert!(
                refs.len() == 2
                    && refs["account_id"].is_string()
                    && refs["account_name"].is_string(),
                "{p}"
            );
        } else {
            assert!(refs.is_empty(), "{p}");
        }
        let validator = jsonschema::validator_for(&schemas[&code]).unwrap();
        let errors: Vec<String> = validator
            .iter_errors(&p["details"])
            .map(|e| e.to_string())
            .collect();
        assert!(errors.is_empty(), "{code}: {errors:?}\n{p:#}");
        seen.push((
            code,
            p["details"]["action"].as_str().unwrap_or("").to_owned(),
        ));
    }
    seen
}

#[tokio::test]
async fn defaults_upsert_validate_and_isolate() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let them = support::user(&env.db).await;

    let (s, list) = get(
        &env,
        &format!("/api/v1/default-profiles?instrument_id={MNQ}"),
        me,
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{list}");
    conforms(&list, "DefaultProfileList");
    assert_eq!(list["defaults"].as_array().unwrap().len(), 6);
    assert!(
        list["defaults"]
            .as_array()
            .unwrap()
            .iter()
            .all(|d| d.get("profile").is_none())
    );

    let body = json!({"distance_unit": "usd", "stop_distance": "100.1", "target_distance": "200"});
    let (s, saved) = put(&env, &default_uri("50000"), me, body.clone()).await;
    assert_eq!(s, StatusCode::OK, "{saved}");
    conforms(&saved, "DefaultProfileEntry");
    assert_eq!(saved["size_amount"], "50000.00");
    assert_eq!(
        saved["profile"],
        json!({"distance_unit": "usd", "stop_distance": "100.10", "target_distance": "200.00"})
    );
    // Same values (other spelling): no second audit entry.
    let same =
        json!({"distance_unit": "usd", "stop_distance": "100.10", "target_distance": "200.00"});
    assert_eq!(
        put(&env, &default_uri("50000.00"), me, same).await.0,
        StatusCode::OK
    );
    assert_eq!(
        audit_conforms(&env, me).await,
        [("profiles.default_changed".to_owned(), String::new())]
    );

    // 400 / 404 / 422 with pointers.
    let (s, e) = put(&env, &default_uri("75000"), me, body.clone()).await;
    assert_eq!(
        (s, error(&e).0),
        (StatusCode::NOT_FOUND, "common.not_found"),
        "not a standard size"
    );
    let (s, e) = put(&env, &default_uri("50000.001"), me, body.clone()).await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::BAD_REQUEST,
            ("common.invalid_request", "/path/size_amount")
        )
    );
    let (s, e) = put(
        &env,
        &format!("/api/v1/default-profiles/{}/50000", Uuid::now_v7()),
        me,
        body.clone(),
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND, "{e}");
    let (s, e) = put(
        &env,
        &default_uri("50000"),
        me,
        json!({"distance_unit": "usd", "stop_distance": "1.005", "target_distance": "2"}),
    )
    .await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::BAD_REQUEST,
            ("common.invalid_request", "/stop_distance")
        )
    );
    let (s, e) = put(
        &env,
        &default_uri("50000"),
        me,
        json!({"distance_unit": "ticks", "stop_distance": "4", "target_distance": "8.5"}),
    )
    .await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::BAD_REQUEST,
            ("common.invalid_request", "/target_distance")
        )
    );
    let (s, e) = put(
        &env,
        &default_uri("50000"),
        me,
        json!({"distance_unit": "points", "stop_distance": "0.2", "target_distance": "2"}),
    )
    .await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ("profiles.distance_below_one_tick", "/stop_distance")
        )
    );
    let (s, e) = put(
        &env,
        &default_uri("50000"),
        me,
        json!({"distance_unit": "ticks", "stop_distance": "4", "target_distance": "10001"}),
    )
    .await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ("profiles.distance_too_large", "/target_distance")
        )
    );
    let (s, e) = get(&env, "/api/v1/default-profiles?instrument_id=NOPE", me).await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::BAD_REQUEST,
            ("common.invalid_request", "/query/instrument_id")
        )
    );

    // Isolation: the other user sees nothing of mine and writes only their own.
    let (_, list) = get(
        &env,
        &format!("/api/v1/default-profiles?instrument_id={MNQ}"),
        them,
    )
    .await;
    assert!(
        list["defaults"]
            .as_array()
            .unwrap()
            .iter()
            .all(|d| d.get("profile").is_none())
    );
    assert!(audit_conforms(&env, them).await.is_empty());

    // Auth and origin.
    assert_eq!(
        send(
            &env,
            "GET",
            &format!("/api/v1/default-profiles?instrument_id={MNQ}"),
            None,
            None
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let req = Request::builder()
        .method("PUT")
        .uri(default_uri("50000"))
        .header(header::COOKIE, format!("__Host-aw_session={me}"))
        .header(header::ORIGIN, "https://evil.test")
        .header(header::CONTENT_TYPE, "application/json");
    let res = env
        .app
        .clone()
        .oneshot(req.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn account_profile_copy_edit_restore_and_isolation() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let them = support::user(&env.db).await;
    put(
        &env,
        &default_uri("100000"),
        me,
        json!({"distance_unit": "ticks", "stop_distance": "60", "target_distance": "120"}),
    )
    .await;

    // Q60: a tie needs the user's choice at create.
    let (s, e) = create_real(&env, me, "75000.00", None).await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ("accounts.profile_size_required", "/profile_size")
        )
    );
    let (s, acc) = create_real(&env, me, "75000.00", Some("100000.00")).await;
    assert_eq!(s, StatusCode::CREATED, "{acc}");
    let id = acc["id"].as_str().unwrap().to_owned();

    let (s, list) = get(&env, &format!("/api/v1/accounts/{id}/profiles"), me).await;
    assert_eq!(s, StatusCode::OK, "{list}");
    conforms(&list, "AccountProfileList");
    let p = &list["profiles"][0]["profile"];
    assert_eq!(
        (
            p["origin"].as_str(),
            p["copied_from_size"].as_str(),
            p["customized"].as_bool()
        ),
        (Some("nearest_size_default"), Some("100000.00"), Some(false))
    );
    assert_eq!(
        p["equivalents"]["stop"],
        json!({"ticks": 60, "points": "15.00", "usd": "30.00", "rounded": false})
    );

    // Cross-user: 404 on every account endpoint, nothing written.
    for (m, uri, body) in [
        ("GET", format!("/api/v1/accounts/{id}/profiles"), None),
        (
            "PUT",
            profile_uri(&id),
            Some(
                json!({"distance_unit": "ticks", "stop_distance": "1", "target_distance": "2", "row_version": 1}),
            ),
        ),
        ("POST", format!("{}/restore", profile_uri(&id)), None),
    ] {
        let (s, e) = send(&env, m, &uri, Some(them), body).await;
        assert_eq!(
            (s, error(&e).0),
            (StatusCode::NOT_FOUND, "common.not_found"),
            "{m} {uri}"
        );
    }
    assert!(outbox(&env, them).await.is_empty());

    // Edit: 409 (stale) before 422, then a customized copy.
    let edit = |stop: &str, v: i64| json!({"distance_unit": "points", "stop_distance": stop, "target_distance": "30", "row_version": v});
    let (s, e) = put(&env, &profile_uri(&id), me, edit("0.1", 9)).await;
    assert_eq!(
        (s, error(&e).0),
        (StatusCode::CONFLICT, "common.version_conflict")
    );
    let (s, e) = put(&env, &profile_uri(&id), me, edit("0.1", 1)).await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ("profiles.distance_below_one_tick", "/stop_distance")
        )
    );
    let (s, v) = put(&env, &profile_uri(&id), me, edit("10.1", 1)).await;
    assert_eq!(s, StatusCode::OK, "{v}");
    conforms(&v, "AccountProfile");
    assert_eq!(
        (v["customized"].as_bool(), v["row_version"].as_i64()),
        (Some(true), Some(2))
    );
    assert_eq!(
        v["equivalents"]["stop"],
        json!({"ticks": 40, "points": "10.00", "usd": "20.00", "rounded": true})
    );
    assert_eq!(v["default_now"]["stop_distance"], "60");

    // Restore: stale 409, then the current default, then a no-op.
    let restore = format!("{}/restore", profile_uri(&id));
    let (s, _) = send(
        &env,
        "POST",
        &restore,
        Some(me),
        Some(json!({"row_version": 1})),
    )
    .await;
    assert_eq!(s, StatusCode::CONFLICT);
    let (s, v) = send(&env, "POST", &restore, Some(me), None).await;
    assert_eq!(s, StatusCode::OK, "{v}");
    conforms(&v, "AccountProfile");
    assert_eq!(
        (
            v["customized"].as_bool(),
            v["row_version"].as_i64(),
            v["profile"]["stop_distance"].as_str()
        ),
        (Some(false), Some(3), Some("60"))
    );
    send(&env, "POST", &restore, Some(me), Some(json!({}))).await;

    let codes = audit_conforms(&env, me).await;
    let got: Vec<(&str, &str)> = codes
        .iter()
        .map(|(c, a)| (c.as_str(), a.as_str()))
        .collect();
    assert_eq!(
        got,
        [
            ("profiles.default_changed", ""),
            ("profiles.account_changed", "initial_copy"),
            ("profiles.account_changed", "edited"),
            ("profiles.account_changed", "restored"),
        ]
    );

    // Execution sees the restored profile; another user's call sees nothing.
    let account = AccountId::from_uuid(Uuid::parse_str(&id).unwrap());
    let mnq = Uuid::parse_str(MNQ).unwrap();
    let b = env
        .brackets
        .brackets_for(UserId::from_uuid(me), mnq, &[account])
        .await
        .unwrap();
    assert_eq!(b[&account].equivalents.stop.ticks, 60);
    assert!(
        env.brackets
            .brackets_for(UserId::from_uuid(them), mnq, &[account])
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn account_without_profile_and_restore_rules() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    // No default at 300000: the account starts without a profile (Q43).
    let (_, acc) = create_real(&env, me, "280000.00", None).await;
    let id = acc["id"].as_str().unwrap().to_owned();
    let (_, list) = get(&env, &format!("/api/v1/accounts/{id}/profiles"), me).await;
    conforms(&list, "AccountProfileList");
    assert_eq!(list["profiles"], json!([{"instrument_id": MNQ}]));
    let account = AccountId::from_uuid(Uuid::parse_str(&id).unwrap());
    let missing = env
        .brackets
        .missing_brackets(
            UserId::from_uuid(me),
            &[account],
            &[Uuid::parse_str(MNQ).unwrap()],
        )
        .await
        .unwrap();
    assert_eq!(missing.len(), 1);

    let restore = format!("{}/restore", profile_uri(&id));
    let (s, e) = send(&env, "POST", &restore, Some(me), None).await;
    assert_eq!(
        (s, error(&e).0),
        (StatusCode::UNPROCESSABLE_ENTITY, "profiles.default_missing")
    );
    // A row_version without a profile conflicts; without one it is set_by_user.
    let body = |v: Option<i64>| {
        let mut b =
            json!({"distance_unit": "ticks", "stop_distance": "8", "target_distance": "16"});
        if let Some(v) = v {
            b["row_version"] = json!(v);
        }
        b
    };
    assert_eq!(
        put(&env, &profile_uri(&id), me, body(Some(1))).await.0,
        StatusCode::CONFLICT
    );
    let (s, v) = put(&env, &profile_uri(&id), me, body(None)).await;
    assert_eq!(s, StatusCode::OK, "{v}");
    conforms(&v, "AccountProfile");
    assert_eq!(v["origin"], "set_by_user");
    assert!(v.get("copied_from_size").is_none() && v.get("default_now").is_none());
    // A second create of the same profile without row_version conflicts.
    assert_eq!(
        put(&env, &profile_uri(&id), me, body(None)).await.0,
        StatusCode::CONFLICT
    );

    // Restore of a set_by_user profile on a tie needs profile_size.
    let (_, tie) = create_real(&env, me, "37500.00", Some("25000.00")).await;
    let tie_id = tie["id"].as_str().unwrap().to_owned();
    put(&env, &profile_uri(&tie_id), me, body(None)).await;
    let tie_restore = format!("{}/restore", profile_uri(&tie_id));
    let (s, e) = send(&env, "POST", &tie_restore, Some(me), None).await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ("profiles.profile_size_required", "/profile_size")
        )
    );
    let (s, e) = send(
        &env,
        "POST",
        &tie_restore,
        Some(me),
        Some(json!({"profile_size": "100000.00"})),
    )
    .await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ("profiles.profile_size_invalid", "/profile_size")
        )
    );
    let (s, e) = send(
        &env,
        "POST",
        &tie_restore,
        Some(me),
        Some(json!({"profile_size": "50000.00", "extra": 1})),
    )
    .await;
    assert_eq!(
        (s, error(&e).0),
        (StatusCode::BAD_REQUEST, "common.invalid_request")
    );
}

#[tokio::test]
async fn nearest_and_preview() {
    let Some(env) = env().await else { return };
    let me = support::user(&env.db).await;
    let them = support::user(&env.db).await;
    put(
        &env,
        &default_uri("100000"),
        me,
        json!({"distance_unit": "ticks", "stop_distance": "60", "target_distance": "120"}),
    )
    .await;

    let uri = format!("/api/v1/profile-sizes/nearest?size_amount=75000&instrument_id={MNQ}");
    let (s, n) = get(&env, &uri, me).await;
    assert_eq!(s, StatusCode::OK, "{n}");
    conforms(&n, "NearestProfileSize");
    assert_eq!(
        n,
        json!({"size_amount": "75000.00", "candidates": [
        {"size_amount": "50000.00", "has_default": false}, {"size_amount": "100000.00", "has_default": true}]})
    );
    let (_, other) = get(&env, &uri, them).await;
    assert!(
        other["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .all(|c| c["has_default"] == false),
        "isolation"
    );
    let (s, e) = get(
        &env,
        &format!("/api/v1/profile-sizes/nearest?size_amount=0&instrument_id={MNQ}"),
        me,
    )
    .await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::BAD_REQUEST,
            ("common.invalid_request", "/query/size_amount")
        )
    );

    let preview = |unit: &str, stop: &str, target: &str| {
        format!(
            "/api/v1/profile-preview?instrument_id={MNQ}&distance_unit={unit}&stop_distance={stop}&target_distance={target}"
        )
    };
    let (s, p) = get(&env, &preview("usd", "100.10", "200.00"), me).await;
    assert_eq!(s, StatusCode::OK, "{p}");
    conforms(&p, "ProfileEquivalents");
    assert_eq!(
        p["stop"],
        json!({"ticks": 200, "points": "50.00", "usd": "100.00", "rounded": true})
    );
    assert_eq!(
        p["target"],
        json!({"ticks": 400, "points": "100.00", "usd": "200.00", "rounded": false})
    );
    let (s, e) = get(&env, &preview("ticks", "1.5", "2"), me).await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::BAD_REQUEST,
            ("common.invalid_request", "/query/stop_distance")
        )
    );
    let (s, e) = get(&env, &preview("ticks", "1", "0"), me).await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            ("profiles.distance_below_one_tick", "/query/target_distance")
        )
    );
    let (s, e) = get(&env, &preview("miles", "1", "1"), me).await;
    assert_eq!(
        (s, error(&e)),
        (
            StatusCode::BAD_REQUEST,
            ("common.invalid_request", "/query/distance_unit")
        )
    );
    let (s, _) = get(&env, &format!("/api/v1/profile-preview?instrument_id={}&distance_unit=ticks&stop_distance=1&target_distance=1", Uuid::now_v7()), me).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    // The preview stores and audits nothing.
    assert_eq!(outbox(&env, me).await.len(), 1);
}
