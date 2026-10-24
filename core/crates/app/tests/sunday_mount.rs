//! The Sunday composition against PostgreSQL: the gateway's production `DeviceTokens` over
//! identity (a real device token row, hashed as identity stores it), and accounts over the
//! gateway's real `VenueAccounts` with a bound account that is out of sync (Grosa: listing and
//! pausing it still answer 200), and identity's connections view over the gateway's platform
//! accounts, scoped to the caller.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::extract::ConnectInfo;
use axum::http::{Request, header};
use gateway_application::ports::{DeviceTokens, PresentedToken};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

const PEPPER: &str = "1:AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA"; // test-only, 32 bytes
const SECRET: &str = "AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA"; // test-only device secret
const ORIGIN: &str = "https://app.test";

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

/// A user with a paired, connected connection.
async fn user_with_connection(owner: &PgPool) -> (Uuid, Uuid) {
    let (user, connection) = (Uuid::now_v7(), Uuid::now_v7());
    exec_as(
        owner,
        user,
        "INSERT INTO identity.app_user (id, email, email_normalized, email_verified_at, locale, time_zone)
         VALUES ($1, $2, $2, now(), 'en', 'UTC')",
        &[json!(user), json!(format!("{}@example.test", user.simple()))],
    )
    .await;
    exec_as(
        owner,
        user,
        "INSERT INTO identity.adapter_connection (id, user_id, name, adapter_type, status, paired_at)
         VALUES ($1, $2, 'Desk A', 'ninjatrader8', 'connected', now())",
        &[json!(connection), json!(user)],
    )
    .await;
    (user, connection)
}

fn config() -> app::config::Config {
    app::config::Config::from_lookup(|k| match k {
        "DATABASE_URL" => Some("postgres://unused".into()),
        "APP_ENV" => Some("test".into()),
        "IDENTITY_PEPPERS" => Some(PEPPER.into()),
        "APP_PUBLIC_ORIGIN" => Some(ORIGIN.into()),
        _ => None,
    })
    .unwrap()
}

#[tokio::test]
async fn device_tokens_authenticate_through_identity() {
    let Some(db) = support::db().await else {
        return;
    };
    let (user, connection) = user_with_connection(&db.owner).await;
    let key_id: String = Uuid::now_v7().simple().to_string()[20..].to_owned();
    let secret = identity_enterprise::tokens::SecretBytes::decode(SECRET).unwrap();
    let hash = Sha256::digest(secret.as_bytes()).to_vec();
    let mut tx = db.owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO identity.device_token (id, user_id, connection_id, key_id, token_hash)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Uuid::now_v7())
    .bind(user)
    .bind(connection)
    .bind(&key_id)
    .bind(&hash)
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();

    let wired = support::wire(&db, &config());
    let tokens = app::identity::IdentityDeviceTokens(wired.identity.unwrap().deps);
    let token = format!("awd_{key_id}_{SECRET}");
    let principal = tokens
        .authenticate(&PresentedToken::new(token.clone()))
        .await
        .unwrap()
        .expect("the stored token authenticates");
    assert_eq!(principal.user_id.as_uuid(), user);
    assert_eq!(principal.connection_id.as_uuid(), connection);
    assert_eq!(principal.key_id, key_id);
    assert_eq!(principal.locale, "en");
    assert!(tokens.still_valid(&key_id).await.unwrap());

    // A wrong secret, an unknown key id and a malformed token are all refused alike.
    let wrong = format!("awd_{key_id}_{}", "B".repeat(43));
    let unknown = format!("awd_ZZZZZZZZZZZZ_{SECRET}");
    for presented in [wrong, unknown, "not-a-token".to_owned()] {
        assert!(
            tokens
                .authenticate(&PresentedToken::new(presented.clone()))
                .await
                .unwrap()
                .is_none(),
            "{presented:.8}"
        );
    }
    assert!(!tokens.still_valid("bad key").await.unwrap());

    // Revoked: refused at connect and on revalidation.
    exec_as(
        &db.owner,
        user,
        "UPDATE identity.device_token SET revoked_at = now() WHERE key_id = $1",
        &[json!(key_id)],
    )
    .await;
    assert!(
        tokens
            .authenticate(&PresentedToken::new(token))
            .await
            .unwrap()
            .is_none()
    );
    assert!(!tokens.still_valid(&key_id).await.unwrap());
}

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

/// Pausing and listing need no step-up.
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

#[tokio::test]
async fn a_bound_out_of_sync_account_is_listed_and_paused() {
    let Some(db) = support::db().await else {
        return;
    };
    let (user, connection) = user_with_connection(&db.owner).await;
    let (venue, account) = (Uuid::now_v7(), Uuid::now_v7());
    exec_as(
        &db.owner,
        user,
        "INSERT INTO gateway.venue_account (id, user_id, connection_id, platform_account, kind, oco_location)
         VALUES ($1, $2, $3, 'Sim101', 'simulation', 'local')",
        &[json!(venue), json!(user), json!(connection)],
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
        "INSERT INTO gateway.account_sync (user_id, account_id, sync_state, flagged_at)
         VALUES ($1, $2, 'out_of_sync', now())",
        &[json!(user), json!(account)],
    )
    .await;

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
    let send = |method: &str, uri: String, body: Option<Value>| {
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
        let router = wired.router.clone();
        async move {
            let response = router.oneshot(request).await.unwrap();
            let status = response.status().as_u16();
            let bytes = to_bytes(response.into_body(), 1 << 20).await.unwrap();
            (
                status,
                serde_json::from_slice::<Value>(&bytes).unwrap_or(Value::Null),
            )
        }
    };

    let (status, list) = send("GET", "/api/v1/accounts".into(), None).await;
    assert_eq!(status, 200, "{list}");
    let listed = list["accounts"]
        .as_array()
        .expect("a list")
        .iter()
        .find(|a| a["id"] == json!(account))
        .cloned()
        .expect("the account is listed");
    assert_eq!(
        (
            &listed["binding"]["platform_account"],
            &listed["binding"]["connection_status"]
        ),
        (&json!("Sim101"), &json!("connected")),
        "{listed}"
    );

    let (status, paused) = send(
        "PUT",
        format!("/api/v1/accounts/{account}/lifecycle"),
        Some(json!({"state": "paused", "row_version": 1})),
    )
    .await;
    assert_eq!(
        (status, paused["lifecycle_state"].as_str()),
        (200, Some("paused")),
        "{paused}"
    );
}

/// The whole app wires with identity (gateway, routing and their consumers), serves the
/// adapters' WebSocket outside the session routes, and `/metrics` carries the gateway counters.
#[tokio::test]
async fn the_mounted_app_serves_the_gateway_and_its_metrics() {
    let Some(db) = support::db().await else {
        return;
    };
    let wired = support::wire(&db, &config());
    let router = app::http::router(wired.state);
    let request = |uri: &str| {
        let mut r = Request::get(uri).body(Body::empty()).unwrap();
        r.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([198, 51, 100, 9], 5000))));
        r
    };
    let response = router.clone().oneshot(request("/metrics")).await.unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let text = String::from_utf8(
        to_bytes(response.into_body(), 1 << 20)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(
        text.contains("allwin_gateway_commands_dropped_total{reason="),
        "{text}"
    );
    // Without a WebSocket upgrade and a device token the adapter route refuses; it is mounted
    // (not the empty 404 of an unknown path) and needs no session cookie.
    let response = router.oneshot(request("/adapter/v1")).await.unwrap();
    assert_ne!(response.status().as_u16(), 404);
    assert_ne!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn the_connections_view_lists_the_gateway_platform_accounts_of_the_caller_only() {
    let Some(db) = support::db().await else {
        return;
    };
    let (user, connection) = user_with_connection(&db.owner).await;
    let (sim101, sim102, account) = (Uuid::now_v7(), Uuid::now_v7(), Uuid::now_v7());
    for (venue, name) in [(sim101, "Sim101"), (sim102, "Sim102")] {
        exec_as(
            &db.owner,
            user,
            "INSERT INTO gateway.venue_account (id, user_id, connection_id, platform_account, kind, oco_location)
             VALUES ($1, $2, $3, $4, 'simulation', 'local')",
            &[json!(venue), json!(user), json!(connection), json!(name)],
        )
        .await;
    }
    // Sim102 is bound to an account the gateway flagged out of sync: possibly open.
    exec_as(
        &db.owner,
        user,
        "INSERT INTO accounts.account (id, user_id, display_name, kind, size_amount, max_position_qty, venue_account_id, bound_at)
         VALUES ($1, $2, 'Sim account', 'real', 50000, 1, $3, now())",
        &[json!(account), json!(user), json!(sim102)],
    )
    .await;
    exec_as(
        &db.owner,
        user,
        "INSERT INTO gateway.account_sync (user_id, account_id, sync_state, flagged_at)
         VALUES ($1, $2, 'out_of_sync', now())",
        &[json!(user), json!(account)],
    )
    .await;
    let (other, other_connection) = user_with_connection(&db.owner).await;

    // The port `App::wire` hands identity (the connections view and the unpair confirmation).
    let wired = support::wire(&db, &config());
    let presence = wired.identity.unwrap().deps.presence;
    let id = identity_enterprise::ids::ConnectionId::from_uuid(connection);
    let mut seen = presence
        .accounts(shared_kernel::UserId::from_uuid(user), &[id])
        .await
        .unwrap();
    let accounts = seen
        .remove(&id)
        .expect("the connection's platform accounts");
    let visible: Vec<_> = accounts
        .visible_accounts
        .iter()
        .map(|a| {
            (
                a.platform_account.as_str(),
                a.kind.as_str(),
                a.oco_location.as_str(),
                a.account_id,
            )
        })
        .collect();
    assert_eq!(
        visible,
        [
            ("Sim101", "simulation", "local", None),
            ("Sim102", "simulation", "local", Some(account)),
        ]
    );
    assert_eq!(accounts.open_position_accounts, ["Sim102"]);

    // Cross-user: another user asking for this connection sees nothing, and the owner's own
    // view never lists another user's connection.
    let other_id = identity_enterprise::ids::ConnectionId::from_uuid(other_connection);
    let theirs = presence
        .accounts(shared_kernel::UserId::from_uuid(other), &[id, other_id])
        .await
        .unwrap();
    assert!(theirs.is_empty(), "{theirs:?}");
    let mine = presence
        .accounts(shared_kernel::UserId::from_uuid(user), &[other_id])
        .await
        .unwrap();
    assert!(mine.is_empty(), "{mine:?}");
}
