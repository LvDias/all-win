//! HTTP adapter tests with fakes only (no database): response DTOs against the contract
//! schemas, the error mapping, the `route_enable` step-up adapter and the request precedence.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Mutex;

use access::http::SESSION_COOKIE;
use access::{Activity, StepUpGrant};
use axum::body::Body;
use axum::http::{Request, header};
use chrono::{TimeZone, Utc};
use serde_json::{Value, json};
use shared_kernel::{SystemClock, UserId};
use tower::ServiceExt;

use routing_application::enterprise::{
    EffectiveStatus, LifecycleState, NotAccepted, OptIn, RouteReason, RouteSignalType, SignalTypes,
};
use routing_application::routes::{
    AccountDirectory, AccountFacts, AccountSizing, Candidate, Candidates, CreateOutcome, Pointer,
    PortError, RouteChange, RouteRecord, RouteStore, RouteUpdate, RouteView, SourceDirectory,
    SourceFacts, UpdateOutcome,
};

use super::*;

const ORIGIN: &str = "https://app.test";

fn conforms(value: &Value, component: &'static str) {
    if let Err(e) = validate::value_against(value, component) {
        panic!("{component} does not match the contract ({e:?}): {value:#}");
    }
}

fn at(s: u32) -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 24, 12, 0, s).unwrap()
}

fn view(reasons: Vec<RouteReason>, is_enabled: bool) -> RouteView {
    RouteView {
        route: RouteRecord {
            id: RouteId::new_v7(),
            user_id: UserId::new_v7(),
            source_id: SourceId::new_v7(),
            account_id: AccountId::new_v7(),
            signal_types: SignalTypes::new([RouteSignalType::NqTrigger, RouteSignalType::Trigger])
                .unwrap(),
            is_enabled,
            row_version: 2,
            created_at: at(0),
            updated_at: at(1),
        },
        effective: EffectiveStatus::from_reasons(reasons),
        computed_at: at(2),
    }
}

fn every_reason() -> Vec<RouteReason> {
    vec![
        RouteReason::AccountInactive(LifecycleState::Paused),
        RouteReason::SourceRevoked,
        RouteReason::PlatformUnbound,
        RouteReason::AutomationNotAccepted(NotAccepted::Declined),
        RouteReason::CopyNotAccepted(NotAccepted::PendingReview),
        RouteReason::ConnectionDisconnected,
        RouteReason::OutOfSync,
        RouteReason::RiskBlocked,
        RouteReason::NoBracketProfile(vec![Uuid::now_v7(), Uuid::now_v7()]),
        RouteReason::LimitsPending,
        RouteReason::RouteDisabled,
    ]
}

// ---- (1) responses ------------------------------------------------------------------------

#[test]
fn responses_match_the_contract_schemas() {
    let receiving = serde_json::to_value(RouteDto::from(&view(vec![], true))).unwrap();
    conforms(&receiving, "Route");
    assert_eq!(receiving["effective"]["status"], "receiving");
    assert_eq!(receiving["signal_types"], json!(["trigger", "nq_trigger"]));
    assert_eq!(receiving["created_at"], "2026-10-24T12:00:00Z");

    let exits_only = view(
        vec![
            RouteReason::OutOfSync,
            RouteReason::NoBracketProfile(vec![Uuid::now_v7()]),
        ],
        true,
    );
    let all = view(every_reason(), false);
    let list = serde_json::to_value(RouteListDto::from(&[exits_only, all][..])).unwrap();
    conforms(&list, "RouteList");
    assert_eq!(list["routes"][0]["effective"]["status"], "exits_only");
    let reasons = &list["routes"][1]["effective"]["reasons"];
    assert_eq!(list["routes"][1]["effective"]["status"], "not_receiving");
    assert_eq!(reasons.as_array().unwrap().len(), every_reason().len());
    assert_eq!(reasons[0]["code"], "routing.source_revoked");
    let inactive = reasons
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["code"] == "routing.account_inactive")
        .unwrap();
    assert_eq!(inactive["lifecycle_state"], "paused");

    let candidates = Candidates {
        source_id: SourceId::new_v7(),
        candidates: vec![
            Candidate {
                account_id: AccountId::new_v7(),
                blocking_reasons: vec![],
                status_if_routed: EffectiveStatus::from_reasons(vec![RouteReason::OutOfSync]),
            },
            Candidate {
                account_id: AccountId::new_v7(),
                blocking_reasons: vec![RouteReason::AutomationNotAccepted(
                    NotAccepted::PendingReview,
                )],
                status_if_routed: EffectiveStatus::from_reasons(vec![
                    RouteReason::AutomationNotAccepted(NotAccepted::PendingReview),
                ]),
            },
        ],
        manual_only: vec![(
            AccountId::new_v7(),
            json!({"text": "Automated trading is not allowed.", "language": "en",
                   "page_title": "Rules", "url": "https://firm.example/rules",
                   "accessed_at": "2026-10-01"}),
        )],
        computed_at: at(3),
    };
    let body = serde_json::to_value(RouteCandidatesDto::from(&candidates)).unwrap();
    conforms(&body, "RouteCandidates");
    assert_eq!(
        body["candidates"][1]["blocking_reasons"][0]["opt_in_state"],
        "pending_review"
    );

    let empty = Candidates {
        source_id: SourceId::new_v7(),
        candidates: vec![],
        manual_only: vec![],
        computed_at: at(3),
    };
    conforms(
        &serde_json::to_value(RouteCandidatesDto::from(&empty)).unwrap(),
        "RouteCandidates",
    );
}

#[test]
fn every_placeholder_has_its_contract_schema() {
    let names: Vec<&str> = schemas::raw_components().iter().map(|(n, _)| *n).collect();
    assert_eq!(
        names,
        [
            "Route",
            "RouteList",
            "RouteCreateRequest",
            "RouteUpdateRequest",
            "RouteCandidates"
        ]
    );
}

// ---- (2) requests -------------------------------------------------------------------------

#[test]
fn request_bodies_map_to_commands() {
    let b: dto::CreateBody = serde_json::from_value(json!({
        "source_id": "3c4d5e6f-7a8b-4c9d-8e0f-1a2b3c4d5e6f",
        "account_id": "0b9f4c1e-7d2a-4e7b-9a52-3c1d8e6f2a10",
        "signal_types": ["trade_plan", "trigger"]}))
    .unwrap();
    let (cmd, proof) = dto::create_command(b).unwrap();
    assert!(cmd.is_enabled, "is_enabled defaults to true");
    assert!(proof.is_none());
    assert!(cmd.signal_types.contains(RouteSignalType::TradePlan));

    let b: dto::UpdateBody = serde_json::from_value(json!({
        "signal_types": ["trigger"], "is_enabled": true, "row_version": 4_000_000_000_u64,
        "step_up": {"challenge_id": "abcdefghijklmnop", "code": "ABCD1234"}}))
    .unwrap();
    assert_eq!(
        dto::update_command(b).err(),
        Some(BadField("/row_version".into()))
    );
    assert_eq!(
        dto::signal_types(&["trigger".into(), "master_account".into()]).err(),
        Some(BadField("/signal_types/1".into()))
    );
    assert_eq!(
        dto::signal_types(&[]).err(),
        Some(BadField("/signal_types".into()))
    );
}

// ---- (3) errors ---------------------------------------------------------------------------

async fn parts(r: Response) -> (StatusCode, Value, Option<String>) {
    let status = r.status();
    let retry = r
        .headers()
        .get(header::RETRY_AFTER)
        .map(|v| v.to_str().unwrap().to_owned());
    let bytes = axum::body::to_bytes(r.into_body(), 64 * 1024)
        .await
        .unwrap();
    let body = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, body, retry)
}

#[tokio::test]
async fn route_errors_map_to_the_contract() {
    let table: Vec<(RouteError, u16, &str, Option<&str>)> = vec![
        (RouteError::NotFound, 404, "common.not_found", None),
        (RouteError::RouteExists, 409, "routing.route_exists", None),
        (
            RouteError::VersionConflict,
            409,
            "common.version_conflict",
            None,
        ),
        (
            RouteError::SourceRevoked { on_create: false },
            409,
            "routing.source_revoked",
            None,
        ),
        (
            RouteError::SourceRevoked { on_create: true },
            422,
            "routing.source_revoked",
            Some("/source_id"),
        ),
        (
            RouteError::SourceUnknown,
            422,
            "routing.source_unknown",
            Some("/source_id"),
        ),
        (
            RouteError::AccountUnknown,
            422,
            "routing.account_unknown",
            Some("/account_id"),
        ),
        (
            RouteError::AccountManualOnly,
            422,
            "routing.account_manual_only",
            Some("/account_id"),
        ),
        (
            RouteError::NotEligible {
                reason: RouteReason::AutomationNotAccepted(NotAccepted::Declined),
                pointer: Pointer::AccountId,
            },
            422,
            "routing.automation_not_accepted",
            Some("/account_id"),
        ),
        (
            RouteError::NotEligible {
                reason: RouteReason::PlatformUnbound,
                pointer: Pointer::IsEnabled,
            },
            422,
            "routing.platform_unbound",
            Some("/is_enabled"),
        ),
        (
            RouteError::StepUp(StepUpDenied::Required),
            403,
            "auth.step_up_required",
            None,
        ),
        (
            RouteError::StepUp(StepUpDenied::CodeInvalid),
            403,
            "auth.code_invalid",
            None,
        ),
        (
            RouteError::StepUp(StepUpDenied::Unavailable),
            500,
            "common.internal",
            None,
        ),
        (
            RouteError::Port(PortError("db down".into())),
            500,
            "common.internal",
            None,
        ),
    ];
    for (error, status, code, pointer) in table {
        let label = format!("{error:?}");
        let (s, body, _) = parts(from_error(error)).await;
        assert_eq!(s.as_u16(), status, "{label}");
        assert_eq!(body["error"]["code"], code, "{label}");
        assert_eq!(body["error"]["pointer"].as_str(), pointer, "{label}");
        assert!(
            !body.to_string().contains("db down"),
            "{label}: detail leaked"
        );
    }
    let (s, body, retry) = parts(from_error(RouteError::StepUp(StepUpDenied::RateLimited {
        retry_after_s: 30,
    })))
    .await;
    assert_eq!(s, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(body["error"]["code"], "auth.rate_limited");
    assert_eq!(retry.as_deref(), Some("30"));
}

// ---- fakes --------------------------------------------------------------------------------

fn caller() -> Caller {
    Caller {
        user_id: UserId::from_uuid(Uuid::from_u128(7)),
        session_id: Uuid::from_u128(8),
        device_id: Uuid::from_u128(9),
        is_platform_admin: false,
    }
}

struct Sessions;

#[async_trait]
impl SessionAuthenticator for Sessions {
    async fn authenticate(&self, c: Option<&str>, _: Activity) -> Result<Caller, AccessError> {
        match c {
            Some("good") => Ok(caller()),
            _ => Err(AccessError::Unauthenticated),
        }
    }
}

/// Refuses a missing proof, accepts code `GOOD1234`, refuses any other; records each call.
#[derive(Default)]
struct Gate {
    calls: Mutex<Vec<(StepUpAction, Option<Uuid>)>>,
    unavailable: bool,
}

#[async_trait]
impl StepUpGate for Gate {
    async fn authorize(
        &self,
        caller: &Caller,
        action: StepUpAction,
        target_id: Option<Uuid>,
        proof: Option<StepUpProof>,
        _: &RequestOrigin,
    ) -> Result<StepUpGrant, AccessError> {
        self.calls.lock().unwrap().push((action, target_id));
        if self.unavailable {
            return Err(AccessError::Unavailable);
        }
        match proof {
            None => Err(AccessError::StepUpRequired),
            Some(p) if p.code == "GOOD1234" => {
                Ok(StepUpGrant::issue(caller, action, target_id, true))
            }
            Some(_) => Err(AccessError::CodeInvalid),
        }
    }
}

const SOURCE: Uuid = Uuid::from_u128(0x3c4d_5e6f_7a8b_4c9d_8e0f_1a2b_3c4d_5e6f);
const ACCOUNT: Uuid = Uuid::from_u128(0x0b9f_4c1e_7d2a_4e7b_9a52_3c1d_8e6f_2a10);

/// No stored route; a create always succeeds.
struct Routes;

#[async_trait]
impl RouteStore for Routes {
    async fn list(&self, _: UserId, _: RouteFilter) -> Result<Vec<RouteRecord>, PortError> {
        Ok(vec![])
    }
    async fn get(&self, _: UserId, _: RouteId) -> Result<Option<RouteRecord>, PortError> {
        Ok(None)
    }
    async fn create(&self, _: &RouteRecord, _: &RouteChange) -> Result<CreateOutcome, PortError> {
        Ok(CreateOutcome::Created)
    }
    async fn update(
        &self,
        _: &RouteUpdate,
        _: Option<&RouteChange>,
    ) -> Result<UpdateOutcome, PortError> {
        Ok(UpdateOutcome::NotFound)
    }
    async fn delete(&self, _: UserId, _: RouteId, _: &RouteChange) -> Result<bool, PortError> {
        Ok(false)
    }
}

/// One eligible account (`ACCOUNT`).
struct Accounts;

#[async_trait]
impl AccountDirectory for Accounts {
    async fn accounts(&self, _: UserId) -> Result<Vec<AccountFacts>, PortError> {
        Ok(vec![AccountFacts {
            id: AccountId::from_uuid(ACCOUNT),
            display_name: "Sim".into(),
            manual_only: None,
            lifecycle: None,
            platform_bound: true,
            automation: OptIn::NotRequired,
            connection_connected: true,
            sizing: AccountSizing {
                multiplier: 1,
                max_position_qty: 1,
                command_ttl_ms: 5000,
            },
        }])
    }
}

/// One live source (`SOURCE`).
struct Sources;

#[async_trait]
impl SourceDirectory for Sources {
    async fn source(&self, _: UserId, id: SourceId) -> Result<Option<SourceFacts>, PortError> {
        Ok((id.as_uuid() == SOURCE).then(|| SourceFacts {
            revoked: false,
            automated: true,
            name: "TV".into(),
        }))
    }
}

fn app(gate: Arc<Gate>) -> Router {
    let deps = RoutingDeps {
        routes: Arc::new(Routes),
        accounts: Arc::new(Accounts),
        sources: Arc::new(Sources),
        gates: None,
        brackets: None,
        clock: Arc::new(SystemClock),
    };
    router(Arc::new(RoutesHttpState::new(
        deps,
        Arc::new(Sessions),
        gate,
        ORIGIN.into(),
        false,
    )))
}

async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    cookie: Option<&str>,
    origin: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    if let Some(c) = cookie {
        req = req.header(header::COOKIE, format!("{SESSION_COOKIE}={c}"));
    }
    if let Some(o) = origin {
        req = req.header(header::ORIGIN, o);
    }
    let req = match body {
        Some(b) => req
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(b.to_string())),
        None => req.body(Body::empty()),
    }
    .unwrap();
    let (s, body, _) = parts(app.clone().oneshot(req).await.unwrap()).await;
    (s, body)
}

fn code_and_pointer(body: &Value) -> (&str, Option<&str>) {
    (
        body["error"]["code"].as_str().unwrap_or(""),
        body["error"]["pointer"].as_str(),
    )
}

// ---- step-up adapter and precedence -------------------------------------------------------

#[tokio::test]
async fn enable_authorizer_asks_route_enable_for_the_account() {
    let gate = Gate::default();
    let c = caller();
    let origin = RequestOrigin {
        ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };
    let account = AccountId::from_uuid(ACCOUNT);
    let missing = RouteEnableStepUp {
        gate: &gate,
        caller: &c,
        proof: None,
        origin: &origin,
    };
    assert_eq!(
        missing.authorize(account).await,
        Err(StepUpDenied::Required)
    );
    let wrong = RouteEnableStepUp {
        proof: Some(StepUpProof {
            challenge_id: "abcdefghijklmnop".into(),
            code: "BAD".into(),
        }),
        ..missing
    };
    assert_eq!(
        wrong.authorize(account).await,
        Err(StepUpDenied::CodeInvalid)
    );
    let good = RouteEnableStepUp {
        proof: Some(StepUpProof {
            challenge_id: "abcdefghijklmnop".into(),
            code: "GOOD1234".into(),
        }),
        ..wrong
    };
    assert_eq!(good.authorize(account).await, Ok(()));
    assert!(
        gate.calls
            .lock()
            .unwrap()
            .iter()
            .all(|c| *c == (StepUpAction::RouteEnable, Some(ACCOUNT)))
    );
    let down = Gate {
        unavailable: true,
        ..Gate::default()
    };
    let on_down = RouteEnableStepUp {
        gate: &down,
        ..good
    };
    assert_eq!(
        on_down.authorize(account).await,
        Err(StepUpDenied::Unavailable)
    );
    assert_eq!(
        denied(AccessError::RateLimited { retry_after_s: 5 }),
        StepUpDenied::RateLimited { retry_after_s: 5 }
    );
}

#[tokio::test]
async fn requests_follow_the_contract_precedence() {
    let gate = Arc::new(Gate::default());
    let app = app(gate.clone());
    let good = Some("good");
    let o = Some(ORIGIN);
    let route = json!({"source_id": SOURCE, "account_id": ACCOUNT, "signal_types": ["trigger"]});
    let id = Uuid::now_v7();

    // 401 before everything, 403 origin before the body.
    let (s, _) = send(&app, "GET", "/api/v1/routes", None, None, None).await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
    let (s, b) = send(
        &app,
        "POST",
        "/api/v1/routes",
        good,
        Some("https://evil.test"),
        Some(json!({})),
    )
    .await;
    assert_eq!(
        (s, code_and_pointer(&b).0),
        (StatusCode::FORBIDDEN, "common.forbidden_origin")
    );
    let (s, _) = send(
        &app,
        "DELETE",
        &format!("/api/v1/routes/{id}"),
        good,
        None,
        None,
    )
    .await;
    assert_eq!(s, StatusCode::FORBIDDEN);

    // 400: query, path, body.
    for (method, uri, body, pointer) in [
        (
            "GET",
            "/api/v1/routes?source_id=NOT-A-UUID".to_owned(),
            None,
            Some("/query/source_id"),
        ),
        (
            "GET",
            format!(
                "/api/v1/routes?account_id={}",
                ACCOUNT.to_string().to_uppercase()
            ),
            None,
            Some("/query/account_id"),
        ),
        (
            "GET",
            "/api/v1/route-candidates".to_owned(),
            None,
            Some("/query/source_id"),
        ),
        (
            "PUT",
            "/api/v1/routes/xyz".to_owned(),
            Some(json!({})),
            Some("/path/route_id"),
        ),
        (
            "DELETE",
            "/api/v1/routes/xyz".to_owned(),
            None,
            Some("/path/route_id"),
        ),
        (
            "POST",
            "/api/v1/routes".to_owned(),
            Some(json!({"source_id": SOURCE, "account_id": ACCOUNT, "signal_types": []})),
            Some("/signal_types"),
        ),
        (
            "PUT",
            format!("/api/v1/routes/{id}"),
            Some(json!({"signal_types": ["trigger"], "is_enabled": true})),
            Some("/row_version"),
        ),
    ] {
        let (s, b) = send(&app, method, &uri, good, o, body).await;
        assert_eq!(s, StatusCode::BAD_REQUEST, "{method} {uri}");
        assert_eq!(
            code_and_pointer(&b),
            ("common.invalid_request", pointer),
            "{method} {uri}"
        );
    }

    // 404 before the step-up.
    let update = json!({"signal_types": ["trigger"], "is_enabled": true, "row_version": 1});
    let (s, _) = send(
        &app,
        "PUT",
        &format!("/api/v1/routes/{id}"),
        good,
        o,
        Some(update),
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    let (s, _) = send(
        &app,
        "DELETE",
        &format!("/api/v1/routes/{id}"),
        good,
        o,
        None,
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    let (s, _) = send(
        &app,
        "GET",
        &format!("/api/v1/route-candidates?source_id={id}"),
        good,
        None,
        None,
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    assert!(gate.calls.lock().unwrap().is_empty());

    // Unknown source: 422 before the step-up.
    let mut unknown = route.clone();
    unknown["source_id"] = json!(id);
    let (s, b) = send(&app, "POST", "/api/v1/routes", good, o, Some(unknown)).await;
    assert_eq!(s, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        code_and_pointer(&b),
        ("routing.source_unknown", Some("/source_id"))
    );
    assert!(gate.calls.lock().unwrap().is_empty());

    // Enabled by default: the step-up is asked for the account.
    let (s, b) = send(&app, "POST", "/api/v1/routes", good, o, Some(route.clone())).await;
    assert_eq!(
        (s, code_and_pointer(&b).0),
        (StatusCode::FORBIDDEN, "auth.step_up_required")
    );
    let mut proven = route.clone();
    proven["step_up"] = json!({"challenge_id": "abcdefghijklmnop", "code": "GOOD1234"});
    let (s, b) = send(&app, "POST", "/api/v1/routes", good, o, Some(proven)).await;
    assert_eq!(s, StatusCode::CREATED);
    conforms(&b, "Route");
    assert_eq!(b["is_enabled"], true);
    assert_eq!(b["effective"]["status"], "receiving");
    assert_eq!(
        gate.calls.lock().unwrap().as_slice(),
        [
            (StepUpAction::RouteEnable, Some(ACCOUNT)),
            (StepUpAction::RouteEnable, Some(ACCOUNT))
        ]
    );

    // Created off: no step-up.
    let mut off = route.clone();
    off["is_enabled"] = json!(false);
    let (s, b) = send(&app, "POST", "/api/v1/routes", good, o, Some(off)).await;
    assert_eq!(s, StatusCode::CREATED);
    assert_eq!(
        b["effective"]["reasons"][0]["code"],
        "routing.route_disabled"
    );
    assert_eq!(gate.calls.lock().unwrap().len(), 2);

    // Reads.
    let (s, b) = send(
        &app,
        "GET",
        &format!("/api/v1/routes?source_id={SOURCE}"),
        good,
        None,
        None,
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    conforms(&b, "RouteList");
    let (s, b) = send(
        &app,
        "GET",
        &format!("/api/v1/route-candidates?source_id={SOURCE}"),
        good,
        None,
        None,
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    conforms(&b, "RouteCandidates");
    assert_eq!(b["candidates"][0]["account_id"], json!(ACCOUNT));
}
