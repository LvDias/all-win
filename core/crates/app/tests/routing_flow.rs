//! Routing through the mounted app against PostgreSQL: a TradingView alert becomes the
//! gateway's `routing.commands_requested` for a simulation account (and never for a live one),
//! the accounts module reads the real routes (Esmeril F1: archiving names the routes it
//! removes; a binding change under an enabled automated route needs step-up), and the stalled
//! exit alert is one log line with its fields and nothing of the alert's body (Sonda L-RW-2).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::body::{Body, to_bytes};
use axum::extract::ConnectInfo;
use axum::http::{Request, header};
use serde_json::{Value, json};
use sqlx::PgPool;
use support::{TestDb, ingress_path, post_json, send, unique_message_id};
use tower::ServiceExt;
use uuid::Uuid;

const PEPPER: &str = "1:AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA"; // test-only, 32 bytes
const ORIGIN: &str = "https://app.test";
/// MNQ, from the catalog migration.
const MNQ: &str = "01a166e7-a8d0-700f-bade-6d8493132056";

fn config() -> app::config::Config {
    app::config::Config::from_lookup(|k| match k {
        "DATABASE_URL" => Some("postgres://unused".into()),
        "APP_ENV" => Some("test".into()),
        "IDENTITY_PEPPERS" => Some(PEPPER.into()),
        "APP_PUBLIC_ORIGIN" => Some(ORIGIN.into()),
        "INGRESS_RATE_PER_KEY_PER_MIN" => Some("1000".into()),
        "INGRESS_IP_FLOOD_PER_MIN" => Some("100000".into()),
        "INGRESS_IP_TOKEN_FAILURES_PER_MIN" => Some("100000".into()),
        _ => None,
    })
    .unwrap()
}

/// Runs `sql` binding `binds` as the owner, scoped to `user` (RLS forced on every user table).
async fn exec_as(owner: &PgPool, user: Uuid, sql: &'static str, binds: &[Value]) {
    let mut tx = owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let mut q = sqlx::query(sql);
    for b in binds {
        q = match b {
            Value::String(s) => match Uuid::parse_str(s) {
                Ok(u) => q.bind(u),
                Err(_) => q.bind(s.clone()),
            },
            other => q.bind(other.clone()),
        };
    }
    q.execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
}

/// A seeded TradingView source routed to one account bound to a paired platform account of
/// `kind`, with a 40/80-tick MNQ profile, and MNQ 2026-12 active.
struct World {
    token: String,
    user: Uuid,
    source: Uuid,
    source_name: String,
    account: Uuid,
}

async fn world(db: &TestDb, kind: &str) -> World {
    let s = support::seed(db).await;
    let user = s.user_id;
    // The seed's only source.
    let [(source, source_name)]: [(Uuid, String); 1] = rows_as(
        &db.owner,
        user,
        "SELECT id, name FROM identity.source WHERE user_id = $1",
        user,
    )
    .await
    .try_into()
    .unwrap();
    let (connection, venue, account) = (Uuid::now_v7(), Uuid::now_v7(), Uuid::now_v7());
    exec_as(
        &db.owner,
        user,
        "INSERT INTO identity.adapter_connection (id, user_id, name, adapter_type, status, paired_at)
         VALUES ($1, $2, 'Desk A', 'ninjatrader8', 'connected', now())",
        &[json!(connection), json!(user)],
    )
    .await;
    exec_as(
        &db.owner,
        user,
        "INSERT INTO gateway.venue_account (id, user_id, connection_id, platform_account, kind, oco_location)
         VALUES ($1, $2, $3, 'Sim101', $4, 'local')",
        &[json!(venue), json!(user), json!(connection), json!(kind)],
    )
    .await;
    exec_as(
        &db.owner,
        user,
        "INSERT INTO accounts.account (id, user_id, display_name, kind, size_amount, max_position_qty, venue_account_id, bound_at)
         VALUES ($1, $2, 'Sim account', 'real', 50000, 1, $3, now())",
        &[json!(account), json!(user), json!(venue)],
    )
    .await;
    exec_as(
        &db.owner,
        user,
        "INSERT INTO accounts.account_bracket_profile (user_id, account_id, instrument_id, distance_unit, stop_distance, target_distance, origin)
         VALUES ($1, $2, $3, 'ticks', 40, 80, 'set_by_user')",
        &[json!(user), json!(account), json!(MNQ)],
    )
    .await;
    exec_as(
        &db.owner,
        user,
        "INSERT INTO routing.subscription (id, user_id, source_id, account_id, signal_types)
         VALUES ($1, $2, $3, $4, ARRAY['trigger'])",
        &[
            json!(Uuid::now_v7()),
            json!(user),
            json!(source),
            json!(account),
        ],
    )
    .await;
    sqlx::query(
        "INSERT INTO catalog.instrument_contract (instrument_id, contract_month, active_from)
         VALUES ($1, DATE '2026-12-01', TIMESTAMPTZ '2026-09-18 00:00:00Z') ON CONFLICT DO NOTHING",
    )
    .bind(Uuid::parse_str(MNQ).unwrap())
    .execute(&db.owner)
    .await
    .unwrap();
    World {
        token: s.token,
        user,
        source,
        source_name,
        account,
    }
}

/// An alert sent now (an entry expires from its `sent_at`).
fn alert(message_id: &str, action: &str) -> String {
    alert_at(message_id, action, chrono::Utc::now())
}

/// An alert sent at `sent_at` (whole seconds on the wire).
fn alert_at(message_id: &str, action: &str, sent_at: chrono::DateTime<chrono::Utc>) -> String {
    let sent_at = sent_at.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let quantity = if action == "close" {
        ""
    } else {
        r#","quantity":1"#
    };
    format!(
        r#"{{"schema_version":"1.0","message_id":"{message_id}","sent_at":"{sent_at}",
            "source":{{"type":"tradingview"}},"signal_type":"trigger","action":"{action}",
            "instrument":{{"symbol":"MNQ"}}{quantity}}}"#
    )
}

/// The rows of `sql` binding `id`, read as the owner scoped to `user` (RLS forced).
async fn rows_as<T>(owner: &PgPool, user: Uuid, sql: &'static str, id: Uuid) -> Vec<T>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let mut tx = owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let rows = sqlx::query_as(sql)
        .bind(id)
        .fetch_all(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    rows
}

/// The outbox events of `user` of type `event_type`.
async fn events(db: &TestDb, user: Uuid, event_type: &str) -> Vec<Value> {
    let all: Vec<(String, Value)> = rows_as(
        &db.owner,
        user,
        "SELECT event_type, payload FROM app.outbox WHERE user_id = $1 ORDER BY occurred_at",
        user,
    )
    .await;
    all.into_iter()
        .filter(|(t, _)| t == event_type)
        .map(|(_, p)| p)
        .collect()
}

/// The routing decisions of `user`'s `account`: `(outcome, reason_code)`.
#[cfg(feature = "dev-stubs")]
async fn decisions(db: &TestDb, user: Uuid, account: Uuid) -> Vec<(String, Option<String>)> {
    rows_as(
        &db.owner,
        user,
        "SELECT outcome, reason_code FROM routing.routing_decision WHERE account_id = $1 ORDER BY decided_at",
        account,
    )
    .await
}

/// Dispatches until `account` has a decision (the decision and its events commit together).
#[cfg(feature = "dev-stubs")]
async fn until_decided(app: &app::App, db: &TestDb, user: Uuid, account: Uuid) {
    for _ in 0..200 {
        app.dispatcher.dispatch_all().await.unwrap();
        if !decisions(db, user, account).await.is_empty() {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
    }
    panic!("no routing decision for {account}");
}

#[cfg(feature = "dev-stubs")]
#[tokio::test]
async fn an_alert_becomes_one_bracketed_gateway_command_for_a_simulation_account() {
    use routing_application::commands::{COMMANDS_REQUESTED, command_id};
    use routing_application::enterprise::{AccountId, InstructionId};

    let Some(db) = support::db().await else {
        return;
    };
    let w = world(&db, "simulation").await;
    let app = support::wire(&db, &config());
    // `routing.decide` starts from now: registered before the alert arrives.
    app.dispatcher.register().await.unwrap();
    let message_id = unique_message_id();
    let sent_at = chrono::DateTime::from_timestamp(chrono::Utc::now().timestamp(), 0).unwrap();
    let (status, _, body) = send(
        &app.state,
        post_json(
            &ingress_path(&w.token),
            alert_at(&message_id, "buy", sent_at),
        ),
    )
    .await;
    assert_eq!(status, 200, "{}", String::from_utf8_lossy(&body));
    until_decided(&app, &db, w.user, w.account).await;

    assert_eq!(
        decisions(&db, w.user, w.account).await,
        [("routed".to_owned(), None)]
    );
    let routed = events(&db, w.user, "routing.routed").await;
    assert_eq!(routed.len(), 1, "{routed:?}");
    let commands = events(&db, w.user, COMMANDS_REQUESTED).await;
    assert_eq!(commands.len(), 1, "{commands:?}");
    let event = &commands[0];
    assert_eq!(event["account_id"], json!(w.account));
    let instruction = Uuid::parse_str(event["instruction_id"].as_str().unwrap()).unwrap();
    let [command] = event["commands"].as_array().unwrap().as_slice() else {
        panic!("one command: {event}");
    };
    let expected_id = command_id(
        InstructionId::from_uuid(instruction),
        AccountId::from_uuid(w.account),
        "place",
    );
    assert_eq!(command["command_id"], json!(expected_id));
    assert_eq!(
        (
            &command["kind"],
            &command["step"],
            &command["position_effect"],
            &command["instrument"]
        ),
        (
            &json!("place"),
            &json!("place"),
            &json!("open"),
            &json!({"symbol": "MNQ", "contract_month": "2026-12"})
        ),
        "{command}"
    );
    // The account's command TTL (5 s) from the alert's `sent_at`, never later.
    let expires_at: chrono::DateTime<chrono::Utc> = command["expires_at"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("{command}"));
    assert!(
        sent_at < expires_at && expires_at <= chrono::Utc::now() + chrono::Duration::seconds(5),
        "{command}"
    );
    assert_eq!(
        command["body"],
        json!({
            "side": "buy",
            "quantity": 1,
            "order": {"type": "market"},
            "position_effect": "open",
            "protection": {"mode": "individual", "stop_ticks": 40, "target_ticks": 80}
        })
    );

    // The same alert again is a duplicate: never a second instruction, never a second command.
    let (status, _, body) = send(
        &app.state,
        post_json(&ingress_path(&w.token), alert(&message_id, "buy")),
    )
    .await;
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        (status, &body["status"]),
        (200, &json!("duplicate")),
        "{body}"
    );
    assert_eq!(events(&db, w.user, "ingress.received").await.len(), 1);
    for _ in 0..5 {
        app.dispatcher.dispatch_all().await.unwrap();
    }
    assert_eq!(events(&db, w.user, COMMANDS_REQUESTED).await.len(), 1);
    assert_eq!(decisions(&db, w.user, w.account).await.len(), 1);
}

/// The test-build gates 7–8 let entries through only for simulation and playback accounts.
#[cfg(feature = "dev-stubs")]
#[tokio::test]
async fn an_alert_never_becomes_a_command_for_a_live_platform_account() {
    let Some(db) = support::db().await else {
        return;
    };
    let w = world(&db, "live").await;
    let app = support::wire(&db, &config());
    app.dispatcher.register().await.unwrap();
    let (status, _, _) = send(
        &app.state,
        post_json(&ingress_path(&w.token), alert(&unique_message_id(), "buy")),
    )
    .await;
    assert_eq!(status, 200);
    until_decided(&app, &db, w.user, w.account).await;

    let [(outcome, reason)] = decisions(&db, w.user, w.account).await.try_into().unwrap();
    assert_ne!(outcome, "routed");
    assert_eq!(reason.as_deref(), Some("routing.risk_blocked"));
    assert!(
        events(&db, w.user, "routing.commands_requested")
            .await
            .is_empty()
    );
}

// ---- accounts over the real routes (Esmeril F1) -----------------------------------------------

/// The session cookie is the user id (requests never reach identity's sessions).
struct CookieIsUser;

#[async_trait::async_trait]
impl access::SessionAuthenticator for CookieIsUser {
    async fn authenticate(
        &self,
        cookie: Option<&str>,
        _: access::Activity,
    ) -> Result<access::Caller, access::AccessError> {
        let user = cookie
            .and_then(|c| Uuid::parse_str(c).ok())
            .ok_or(access::AccessError::Unauthenticated)?;
        Ok(access::Caller {
            user_id: shared_kernel::UserId::from_uuid(user),
            session_id: Uuid::now_v7(),
            device_id: Uuid::now_v7(),
            is_platform_admin: false,
        })
    }
}

/// Every step-up is asked for and never given.
struct NoStepUp;

#[async_trait::async_trait]
impl access::StepUpGate for NoStepUp {
    async fn authorize(
        &self,
        _: &access::Caller,
        _: access::StepUpAction,
        _: Option<Uuid>,
        _: Option<access::StepUpProof>,
        _: &access::RequestOrigin,
    ) -> Result<access::StepUpGrant, access::AccessError> {
        Err(access::AccessError::StepUpRequired)
    }
}

/// The accounts router over the gateway's venues and routing's real `AccountRoutes`.
fn accounts_router(db: &TestDb) -> axum::Router {
    let identity_store: Arc<dyn identity_application::ports::IdentityStore> = Arc::new(
        identity_infra::store::PgIdentityStore::new(db.app.clone(), db.worker.clone()),
    );
    let reads: app::gateway::AccountsReads = Arc::default();
    let mut consumers = app::outbox::Consumers::default();
    let audit: Arc<dyn app::outbox::EventConsumer> = Arc::new(app::outbox::AuditConsumer(
        audit_application::RecordAuditEntry::new(Arc::new(audit_infra::PgAuditStore::new(
            db.app.clone(),
        ))),
    ));
    let wired = app::accounts::wire(
        &db.app,
        app::accounts::store(&db.app),
        Arc::new(shared_kernel::SystemClock),
        app::accounts::SessionPorts {
            authenticator: Arc::new(CookieIsUser),
            step_up: Arc::new(NoStepUp),
            identity_store: identity_store.clone(),
        },
        Arc::new(app::gateway::GatewayVenues {
            store: app::gateway::store(&db.app),
            identity: identity_store.clone(),
            accounts: reads.clone(),
        }),
        app::routing::account_routes(&db.app, identity_store),
        ORIGIN.into(),
        false,
        false,
        audit,
        &mut consumers,
    );
    assert!(reads.set(wired.reads).is_ok());
    wired.router
}

async fn call(
    router: &axum::Router,
    user: Uuid,
    method: &str,
    uri: String,
    body: Option<Value>,
) -> (u16, Value) {
    let mut b = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, format!("__Host-aw_session={user}"))
        .header(header::ORIGIN, ORIGIN);
    let body = match body {
        Some(v) => {
            b = b.header(header::CONTENT_TYPE, "application/json");
            Body::from(v.to_string())
        }
        None => Body::empty(),
    };
    let mut request = b.body(body).unwrap();
    request
        .extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([198, 51, 100, 9], 5000))));
    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status().as_u16();
    let bytes = to_bytes(response.into_body(), 1 << 20).await.unwrap();
    (
        status,
        serde_json::from_slice::<Value>(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn archiving_an_account_names_the_routes_it_removes() {
    let Some(db) = support::db().await else {
        return;
    };
    let w = world(&db, "simulation").await;
    let router = accounts_router(&db);
    let route = json!({"source_id": w.source, "source_name": w.source_name});

    let (status, impact) = call(
        &router,
        w.user,
        "GET",
        format!("/api/v1/accounts/{}/archive-impact", w.account),
        None,
    )
    .await;
    assert_eq!(status, 200, "{impact}");
    assert_eq!(impact["routes"], json!([route]), "{impact}");

    let (status, body) = call(
        &router,
        w.user,
        "DELETE",
        format!("/api/v1/accounts/{}", w.account),
        None,
    )
    .await;
    assert_eq!(status, 204, "{body}");
    // The audit entry of the archive names the routes as they were.
    let archived = events(&db, w.user, "accounts.archived").await;
    assert_eq!(archived.len(), 1, "{archived:?}");
    assert_eq!(
        archived[0]["details"]["removed_routes"],
        json!([route]),
        "{}",
        archived[0]
    );
}

#[tokio::test]
async fn a_binding_change_under_an_enabled_automated_route_needs_step_up() {
    let Some(db) = support::db().await else {
        return;
    };
    let w = world(&db, "simulation").await;
    let router = accounts_router(&db);
    // Unbinding (no `binding`) is a binding change.
    let unbind = |row_version: i32| {
        json!({
            "row_version": row_version,
            "display_name": "Sim account",
            "size_amount": "50000",
            "max_position_qty": 1
        })
    };
    let uri = format!("/api/v1/accounts/{}", w.account);

    let (status, refused) = call(&router, w.user, "PUT", uri.clone(), Some(unbind(1))).await;
    assert_eq!(
        (status, refused["error"]["code"].as_str()),
        (403, Some("auth.step_up_required")),
        "{refused}"
    );
    let bound: Vec<(Option<Uuid>,)> = rows_as(
        &db.owner,
        w.user,
        "SELECT venue_account_id FROM accounts.account WHERE id = $1",
        w.account,
    )
    .await;
    assert!(bound[0].0.is_some(), "nothing saved without the step-up");

    // The same change without an enabled route needs none.
    exec_as(
        &db.owner,
        w.user,
        "UPDATE routing.subscription SET is_enabled = false WHERE account_id = $1",
        &[json!(w.account)],
    )
    .await;
    let (status, saved) = call(&router, w.user, "PUT", uri, Some(unbind(1))).await;
    assert_eq!(status, 200, "{saved}");
    assert!(saved["binding"].is_null(), "{saved}");
}

// ---- the stalled exit alert (Sonda L-RW-2) ----------------------------------------------------

/// The log output, one JSON object per line.
#[derive(Clone, Default)]
struct Captured(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for Captured {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Captured {
    fn text(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }

    /// The captured lines whose message is `message`.
    fn lines(&self, message: &str) -> Vec<Value> {
        self.text()
            .lines()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .filter(|v| v["fields"]["message"] == message)
            .collect()
    }
}

/// Captures WARN and above as JSON on this thread (a current-thread test runtime) while the
/// guard lives.
fn capture() -> (Captured, tracing::subscriber::DefaultGuard) {
    let captured = Captured::default();
    let writer = captured.clone();
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::WARN)
        .with_writer(move || writer.clone())
        .finish();
    (captured, tracing::subscriber::set_default(subscriber))
}

/// A close against a flat position whose sync check (gate 6) cannot be read, 90 s after its
/// receipt: one WARN line for the operator with the instruction, the contract, the waiting
/// account and the time waited — never the alert's body, its message id or the webhook token.
#[tokio::test(flavor = "current_thread")]
async fn a_waiting_exit_logs_one_operator_alert_without_the_alert_body() {
    // Its own database: gate 6 is made unreadable for the app role.
    let Some(db) = support::fresh_db().await else {
        return;
    };
    let w = world(&db, "simulation").await;
    sqlx::query("REVOKE SELECT ON gateway.account_sync FROM allwin_app")
        .execute(&db.owner)
        .await
        .unwrap();
    let app = support::wire(&db, &config());
    app.dispatcher.register().await.unwrap();
    let message_id = unique_message_id();
    let (status, _, _) = send(
        &app.state,
        post_json(&ingress_path(&w.token), alert(&message_id, "close")),
    )
    .await;
    assert_eq!(status, 200);
    exec_as(
        &db.owner,
        w.user,
        "UPDATE app.outbox SET occurred_at = occurred_at - interval '90 seconds'
         WHERE user_id = $1 AND event_type = 'ingress.received'",
        &[json!(w.user)],
    )
    .await;

    let (captured, _guard) = capture();
    let line = {
        let mut found = None;
        for _ in 0..40 {
            app.dispatcher.dispatch_all().await.unwrap();
            found = captured.text().lines().find_map(|l| {
                let v: Value = serde_json::from_str(l).ok()?;
                (v["fields"]["message"] == "ALERT exit decision waiting for account sync check")
                    .then_some(v)
            });
            if found.is_some() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(25)).await;
        }
        found.unwrap_or_else(|| panic!("no exit alert in {}", captured.text()))
    };

    assert_eq!(
        (&line["level"], &line["target"]),
        (&json!("WARN"), &json!("app::routing")),
        "{line}"
    );
    let fields = &line["fields"];
    let received = events(&db, w.user, "ingress.received").await;
    let instruction = &received[0]["refs"]["instruction_id"];
    assert_eq!(&fields["instruction_id"], instruction);
    assert_eq!(fields["instrument"], json!("MNQ 2026-12"));
    assert_eq!(fields["account_ids"], json!(format!("[{}]", w.account)));
    let stalled = fields["stalled_s"].as_i64().unwrap();
    assert!((60..300).contains(&stalled), "{line}");
    assert!(fields["error"].as_str().is_some_and(|e| !e.is_empty()));
    let mut keys: Vec<&str> = fields
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "account_ids",
            "error",
            "instruction_id",
            "instrument",
            "message",
            "stalled_s"
        ]
    );
    // Nothing of the alert itself reaches the log.
    let all = captured.text();
    for (label, secret) in [
        ("the token", w.token.as_str()),
        ("the token secret", &w.token[17..]),
        ("the message id", message_id.as_str()),
        ("the alert body", "schema_version"),
        ("a payload", "\"payload\""),
        ("a body", "\"body\""),
    ] {
        assert!(!all.contains(secret), "the log carries {label}");
    }
}

// ---- the give-up (Vigia M1/M2) ----------------------------------------------------------------

const STALLED: &str =
    "ALERT routing.decide stalled: not decided and not given up, retrying, never routed late";
const GAVE_UP: &str =
    "routing.decide gave up: entry expired with gates unknown, audited, never routed late";

/// `routing.decide`'s delivery of each `ingress.received` of `user`, in order:
/// `(attempts, delivered, dead-lettered)`.
async fn deliveries(db: &TestDb, user: Uuid) -> Vec<(i32, bool, bool)> {
    rows_as(
        &db.owner,
        user,
        "SELECT d.attempts, d.delivered_at IS NOT NULL, d.dead_lettered_at IS NOT NULL
         FROM app.outbox o
         JOIN app.outbox_delivery d ON d.event_id = o.id AND d.consumer = 'routing.decide'
         WHERE o.user_id = $1 AND o.event_type = 'ingress.received' ORDER BY o.seq",
        user,
    )
    .await
}

/// Ends the backoff of every undelivered `routing.decide` delivery.
async fn retry_now(db: &TestDb) {
    let n = sqlx::query(
        "UPDATE app.outbox_delivery SET next_attempt_at = now()
         WHERE consumer = 'routing.decide' AND delivered_at IS NULL AND dead_lettered_at IS NULL",
    )
    .execute(&db.owner)
    .await
    .unwrap()
    .rows_affected();
    assert!(n > 0, "nothing in backoff");
}

/// Dispatches until `user` has `n` deliveries of `routing.decide` with `attempts` or more.
async fn until_attempted(app: &app::App, db: &TestDb, user: Uuid, n: usize, attempts: i32) {
    for _ in 0..200 {
        app.dispatcher.dispatch_all().await.unwrap();
        let d = deliveries(db, user).await;
        if d.len() == n && d.iter().all(|(a, done, _)| *done || *a >= attempts) {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
    }
    panic!("not attempted: {:?}", deliveries(db, user).await);
}

/// The receipts of `user`'s alerts as if they were `11 minutes` (or `2 hours`) old.
async fn age_receipts(db: &TestDb, user: Uuid, by: &str) {
    let sql: &'static str = match by {
        "11 minutes" => {
            "UPDATE app.outbox SET occurred_at = occurred_at - interval '11 minutes'
             WHERE user_id = $1 AND event_type = 'ingress.received'"
        }
        "2 hours" => {
            "UPDATE app.outbox SET occurred_at = occurred_at - interval '2 hours'
             WHERE user_id = $1 AND event_type = 'ingress.received'"
        }
        other => panic!("{other}"),
    };
    exec_as(&db.owner, user, sql, &[json!(user)]).await;
}

/// A proven entry whose decision keeps failing (its expected position unreadable) is given up
/// past the horizon as `routing.expired` with `gates_unknown` in the user's audit. While that
/// write fails, the operator gets the stalled ERROR and the delivery is retried, never dropped.
#[tokio::test(flavor = "current_thread")]
async fn a_stalled_entry_is_given_up_into_the_audit_and_a_failed_write_is_retried() {
    let Some(db) = support::fresh_db().await else {
        return;
    };
    let w = world(&db, "simulation").await;
    for sql in [
        "REVOKE SELECT ON gateway.execution_report FROM allwin_app",
        "REVOKE INSERT ON routing.routing_decision FROM allwin_app",
    ] {
        sqlx::query(sql).execute(&db.owner).await.unwrap();
    }
    let app = support::wire(&db, &config());
    app.dispatcher.register().await.unwrap();
    let (status, _, _) = send(
        &app.state,
        post_json(&ingress_path(&w.token), alert(&unique_message_id(), "buy")),
    )
    .await;
    assert_eq!(status, 200);
    age_receipts(&db, w.user, "11 minutes").await;
    exec_as(
        &db.owner,
        w.user,
        "UPDATE routing.instruction SET sent_at = sent_at - interval '11 minutes',
                expiry_base = LEAST(sent_at - interval '11 minutes', received_at)
         WHERE user_id = $1",
        &[json!(w.user)],
    )
    .await;

    let (captured, _guard) = capture();
    until_attempted(&app, &db, w.user, 1, 1).await;
    assert_eq!(deliveries(&db, w.user).await, [(1, false, false)]);
    let stalled = captured.lines(STALLED);
    assert_eq!(stalled.len(), 1, "{}", captured.text());
    assert_eq!(stalled[0]["level"], "ERROR");
    assert!(events(&db, w.user, "routing.expired").await.is_empty());

    sqlx::query("GRANT INSERT ON routing.routing_decision TO allwin_app")
        .execute(&db.owner)
        .await
        .unwrap();
    retry_now(&db).await;
    until_attempted(&app, &db, w.user, 1, 2).await;
    // The second attempt delivered it.
    assert_eq!(deliveries(&db, w.user).await, [(2, true, false)]);
    let expired = events(&db, w.user, "routing.expired").await;
    let [entry] = expired.as_slice() else {
        panic!("one routing.expired: {expired:?}");
    };
    assert_eq!(entry["details"]["gates_unknown"], true, "{entry}");
    assert_eq!(entry["refs"]["outcome"], "expired", "{entry}");
    assert!(
        events(&db, w.user, "routing.commands_requested")
            .await
            .is_empty()
    );
    let gave_up = captured.lines(GAVE_UP);
    assert_eq!(gave_up.len(), 1, "{}", captured.text());
    assert_eq!(
        gave_up[0]["fields"]["account_ids"],
        json!(format!("[{}]", w.account))
    );
}

/// A reduce, a cancel and a modify whose instruction cannot be read are never classified as
/// entries: past the horizon, and hours later, each keeps waiting with the stalled ERROR; once
/// readable they end as not routable in V1.
#[tokio::test(flavor = "current_thread")]
async fn reduce_cancel_and_modify_with_a_port_error_keep_waiting_past_the_horizon() {
    let Some(db) = support::fresh_db().await else {
        return;
    };
    let w = world(&db, "simulation").await;
    let app = support::wire(&db, &config());
    app.dispatcher.register().await.unwrap();
    let as_stored = [
        (
            "sell",
            json!({"action": "sell", "position_effect": "reduce"}),
        ),
        ("cancel", json!({"action": "cancel"})),
        ("modify", json!({"action": "modify"})),
    ];
    for (action, payload) in &as_stored {
        let message_id = unique_message_id();
        let (status, _, _) = send(
            &app.state,
            post_json(&ingress_path(&w.token), alert(&message_id, "buy")),
        )
        .await;
        assert_eq!(status, 200);
        exec_as(
            &db.owner,
            w.user,
            "UPDATE routing.instruction SET action = $1, payload = payload || $2 WHERE message_id = $3",
            &[json!(action), payload.clone(), json!(message_id)],
        )
        .await;
    }
    sqlx::query("REVOKE SELECT ON routing.instruction FROM allwin_app")
        .execute(&db.owner)
        .await
        .unwrap();
    age_receipts(&db, w.user, "11 minutes").await;

    let (captured, _guard) = capture();
    until_attempted(&app, &db, w.user, 3, 1).await;
    assert_eq!(deliveries(&db, w.user).await, [(1, false, false); 3]);
    assert_eq!(captured.lines(STALLED).len(), 3, "{}", captured.text());

    age_receipts(&db, w.user, "2 hours").await;
    retry_now(&db).await;
    until_attempted(&app, &db, w.user, 3, 2).await;
    assert_eq!(deliveries(&db, w.user).await, [(2, false, false); 3]);
    assert!(captured.lines(GAVE_UP).is_empty());

    sqlx::query("GRANT SELECT ON routing.instruction TO allwin_app")
        .execute(&db.owner)
        .await
        .unwrap();
    retry_now(&db).await;
    until_attempted(&app, &db, w.user, 3, 3).await;
    assert_eq!(deliveries(&db, w.user).await, [(3, true, false); 3]);
    assert!(
        events(&db, w.user, "routing.commands_requested")
            .await
            .is_empty()
    );
}
