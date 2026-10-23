//! PostgreSQL integration tests for the identity store (data model §9), run as `allwin_app`
//! (never as superuser: it would bypass RLS). Each run creates a fresh database from
//! `TEST_DATABASE_ADMIN_URL`, migrates it as `allwin_owner` and drops it at the end.
//! Without `TEST_DATABASE_ADMIN_URL` the tests are skipped (with a note on stderr).
//! `ALLWIN_TEST_PRE_MIGRATIONS` (`:`-separated .sql files) supplies migrations that live on
//! another branch until integration (the #1 bootstrap). The `allwin_app` URL is
//! `IDENTITY_TEST_APP_URL`, else `DATABASE_URL` (which sqlx macros may need at build time).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

mod common;

use chrono::Utc;
use common::*;
use identity_application::auth::{Activity, AuthenticateSession, Requirement};
use identity_application::changes::{ChangeSet, Write};
use identity_application::error::IdentityError;
use identity_application::ports::*;
use identity_application::usecases::connections::*;
use identity_application::usecases::invitations::*;
use identity_application::usecases::pairing::{PairAdapter, PairAdapterCommand};
use identity_application::usecases::signin::*;
use identity_application::usecases::sources::*;
use identity_application::views::LoginOutcome;
use identity_enterprise::connection::{AdapterBuild, AdapterInfo, LiveState};
use identity_enterprise::profile::Locale;
use identity_enterprise::secret::SecretString;
use identity_enterprise::source::SourceType;
use uuid::Uuid;

async fn outbox_types(db: &Db) -> Vec<(Uuid, String, String)> {
    sqlx::query_as::<_, (Uuid, String, String)>(
        "SELECT user_id, event_type, payload::text FROM app.outbox ORDER BY occurred_at, id",
    )
    .fetch_all(&db.inspect)
    .await
    .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn identity_store_against_postgres() {
    let Some(db) = setup().await else { return };
    let (deps, store) = deps(&db);

    // --- RLS lint (§3): every identity table with user_id forces RLS and has a policy.
    let unprotected: Vec<String> = sqlx::query_scalar(
        r#"SELECT c.relname::text FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
            WHERE n.nspname = 'identity' AND c.relkind = 'r'
              AND EXISTS (SELECT 1 FROM pg_attribute a WHERE a.attrelid = c.oid AND a.attname = 'user_id' AND NOT a.attisdropped)
              AND NOT (c.relrowsecurity AND c.relforcerowsecurity AND EXISTS (SELECT 1 FROM pg_policy p WHERE p.polrelid = c.oid))"#,
    )
    .fetch_all(&db.owner)
    .await
    .unwrap();
    assert!(
        unprotected.is_empty(),
        "tables without forced RLS: {unprotected:?}"
    );

    let alice = seed_user(&deps, "alice@example.com", true).await;
    let bob = seed_user(&deps, "bob@example.com", false).await;
    let (cookie, a) = sign_in(&deps, "alice@example.com").await;
    let (_, b) = sign_in(&deps, "Bob@Example.com").await;

    // --- Login failures look the same.
    let err = Login::new(deps.clone())
        .handle(LoginCommand {
            email: "nobody@example.com".into(),
            password: SecretString::new(PASSWORD),
            remember_me: false,
            meta: meta(),
        })
        .await
        .unwrap_err();
    assert!(matches!(err, IdentityError::InvalidCredentials));

    // --- Sources: create, isolation, rotation, revocation, resolve_source_key.
    let created = CreateSource::new(deps.clone())
        .handle(&a, "Main", SourceType::TradingView, &meta())
        .await
        .unwrap();
    let source_id = created.source.source.id;
    let first_key = created.source.source.key_id.clone();
    assert!(matches!(
        CreateSource::new(deps.clone())
            .handle(&a, "MAIN", SourceType::TradingView, &meta())
            .await,
        Err(IdentityError::SourceNameTaken)
    ));
    assert!(
        store.source(bob.id, source_id).await.unwrap().is_none(),
        "B cannot read A's source"
    );
    assert!(matches!(
        RotateSource::new(deps.clone())
            .handle(&b, source_id, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    let cross = store
        .commit(ChangeSet::new(bob.id).write(Write::RevokeSource {
            user_id: alice.id,
            source_id,
            at: Utc::now(),
        }))
        .await;
    assert!(
        cross.is_ok(),
        "an UPDATE under B's scope matches no A row (RLS)"
    );
    assert!(
        store
            .source(alice.id, source_id)
            .await
            .unwrap()
            .unwrap()
            .revoked_at
            .is_none(),
        "A's source untouched"
    );

    let rotated = RotateSource::new(deps.clone())
        .handle(&a, source_id, &meta())
        .await
        .unwrap();
    let new_key = rotated.source.source.key_id.clone();
    let resolve = |k: String| {
        let pool = db.app.clone();
        async move {
            sqlx::query_as::<_, (Uuid, bool, Vec<u8>)>(
                "SELECT user_id, revoked, secret_hash FROM identity.resolve_source_key($1)",
            )
            .bind(k)
            .fetch_one(&pool)
            .await
            .unwrap()
        }
    };
    assert!(
        resolve(first_key.as_str().into()).await.1,
        "old key revoked at once"
    );
    let (owner, revoked, hash) = resolve(new_key.as_str().into()).await;
    assert_eq!((owner, revoked), (alice.id.as_uuid(), false));
    let url = rotated.webhook_url.expose();
    let secret = identity_enterprise::tokens::SecretBytes::decode(&url[url.len() - 43..]).unwrap();
    assert_eq!(
        hash,
        deps.hasher.sha256(secret.as_bytes()).as_bytes().to_vec(),
        "SHA-256 of the 32 decoded bytes"
    );
    RevokeSource::new(deps.clone())
        .handle(&a, source_id, &meta())
        .await
        .unwrap();
    assert!(resolve(new_key.as_str().into()).await.1);

    // --- Connections and pairing, including two concurrent redemptions of one code.
    let conn = CreateConnection::new(deps.clone())
        .handle(&a, "Desk A", &meta())
        .await
        .unwrap();
    let conn_id = conn.connection.connection.id;
    let code = conn.pairing.pairing_code.expose().to_owned();
    let cmd = || PairAdapterCommand {
        pairing_code: code.clone(),
        instance_id: Uuid::now_v7(),
        instance_name: "PC".into(),
        adapter: AdapterInfo {
            adapter_type: "ninjatrader8".into(),
            version: "1.0.0".into(),
            build: AdapterBuild::Release,
        },
        ip: meta().ip,
    };
    let (p1, p2) = (
        PairAdapter::new(deps.clone()),
        PairAdapter::new(deps.clone()),
    );
    let (r1, r2) = tokio::join!(p1.handle(cmd()), p2.handle(cmd()));
    let oks = [r1.is_ok(), r2.is_ok()].iter().filter(|x| **x).count();
    assert_eq!(oks, 1, "exactly one concurrent redemption wins");
    assert!(
        [&r1, &r2]
            .iter()
            .any(|r| matches!(r, Err(IdentityError::PairingInvalidCode)))
    );
    let token = r1.or(r2).unwrap().device_token.expose().to_owned();
    let (dev_user, dev_revoked): (Uuid, bool) =
        sqlx::query_as("SELECT user_id, revoked FROM identity.resolve_device_token($1)")
            .bind(&token[4..16])
            .fetch_one(&db.app)
            .await
            .unwrap();
    assert_eq!((dev_user, dev_revoked), (alice.id.as_uuid(), false));
    assert!(store.connection(bob.id, conn_id).await.unwrap().is_none());

    // Live state and the startup reset.
    RecordConnectionLiveState::new(deps.clone())
        .handle(alice.id, conn_id, LiveState::Connected, Utc::now())
        .await
        .unwrap();
    assert_eq!(
        DisconnectLiveConnectionsAtStartup::new(deps.clone())
            .handle()
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        DisconnectLiveConnectionsAtStartup::new(deps.clone())
            .handle()
            .await
            .unwrap(),
        0
    );
    UnpairConnection::new(deps.clone())
        .handle(&a, conn_id, true, &meta())
        .await
        .unwrap();
    let (_, dev_revoked): (Uuid, bool) =
        sqlx::query_as("SELECT user_id, revoked FROM identity.resolve_device_token($1)")
            .bind(&token[4..16])
            .fetch_one(&db.app)
            .await
            .unwrap();
    assert!(dev_revoked);

    // --- Invitations: admin creates, token holder accepts, single use, expired one superseded.
    let created = CreateInvitation::new(deps.clone())
        .handle(
            &a,
            CreateInvitationCommand {
                email: "carol@example.com".into(),
                locale: Locale::PtBr,
                send_email: true,
                meta: meta(),
            },
        )
        .await
        .unwrap();
    assert!(!created.email_sent);
    assert!(matches!(
        CreateInvitation::new(deps.clone())
            .handle(
                &b,
                CreateInvitationCommand {
                    email: "x@example.com".into(),
                    locale: Locale::En,
                    send_email: false,
                    meta: meta()
                }
            )
            .await,
        Err(IdentityError::Forbidden)
    ));
    assert!(matches!(
        CreateInvitation::new(deps.clone())
            .handle(
                &a,
                CreateInvitationCommand {
                    email: "bob@example.com".into(),
                    locale: Locale::En,
                    send_email: false,
                    meta: meta()
                }
            )
            .await,
        Err(IdentityError::InvitationEmailTaken)
    ));
    let token = created
        .invite_url
        .expose()
        .split("#t=")
        .nth(1)
        .unwrap()
        .to_owned();
    let accept = |t: String| AcceptInvitationCommand {
        token: t,
        password: SecretString::new(PASSWORD),
        locale: Locale::PtBr,
        time_zone: "America/Sao_Paulo".into(),
        meta: meta(),
    };
    let out = AcceptInvitation::new(deps.clone())
        .handle(accept(token.clone()))
        .await
        .unwrap();
    assert!(matches!(out, LoginOutcome::Authenticated { .. }));
    assert!(matches!(
        AcceptInvitation::new(deps.clone())
            .handle(accept(token))
            .await,
        Err(IdentityError::InvitationInvalid)
    ));
    // Expired open invitation for dave, then re-invite.
    sqlx::query("INSERT INTO identity.invitation (id, email_normalized, locale, token_hash, issued_by, expires_at) VALUES ($1, 'dave@example.com', 'en', $2, $3, now() - interval '1 day')")
        .bind(Uuid::now_v7())
        .bind(vec![9u8; 32])
        .bind(alice.id.as_uuid())
        .execute(&db.app)
        .await
        .unwrap();
    CreateInvitation::new(deps.clone())
        .handle(
            &a,
            CreateInvitationCommand {
                email: "dave@example.com".into(),
                locale: Locale::En,
                send_email: false,
                meta: meta(),
            },
        )
        .await
        .unwrap();
    let superseded: i64 = sqlx::query_scalar("SELECT count(*) FROM identity.invitation WHERE email_normalized = 'dave@example.com' AND superseded_at IS NOT NULL")
        .fetch_one(&db.app)
        .await
        .unwrap();
    assert_eq!(superseded, 1);
    assert_eq!(
        store.users(alice.id).await.unwrap().len(),
        3,
        "admin lists everyone"
    );
    assert!(
        store.users(bob.id).await.unwrap().is_empty(),
        "non-admin gets nothing"
    );
    assert!(
        !store
            .email_registered(bob.id, "alice@example.com")
            .await
            .unwrap(),
        "no e-mail oracle for non-admins"
    );

    // --- TOTP step replay is refused by the conditional update.
    store
        .commit(
            ChangeSet::new(bob.id)
                .write(Write::UpsertTotpSetup {
                    user_id: bob.id,
                    credential: identity_enterprise::totp::TotpCredential {
                        secret_encrypted: vec![1; 48],
                        key_version: 1,
                        created_at: Utc::now(),
                        enabled_at: None,
                        last_used_step: None,
                    },
                })
                .write(Write::EnableTotp {
                    user_id: bob.id,
                    at: Utc::now(),
                    used_step: 100,
                }),
        )
        .await
        .unwrap();
    let replay = store
        .commit(ChangeSet::new(bob.id).write(Write::ConsumeTotpStep {
            user_id: bob.id,
            step: 100,
        }))
        .await;
    assert!(matches!(
        replay,
        Err(StoreError::Conflict(StoreConflict::TotpStepReplayed))
    ));
    store
        .commit(ChangeSet::new(bob.id).write(Write::ConsumeTotpStep {
            user_id: bob.id,
            step: 101,
        }))
        .await
        .unwrap();

    // --- Sessions: logout ends it; outbox has audit + SSE events, never a token.
    Logout::new(deps.clone())
        .handle(Some(&cookie))
        .await
        .unwrap();
    assert!(
        AuthenticateSession::new(deps.clone())
            .handle(Some(&cookie), Requirement::Full, Activity::UserInitiated)
            .await
            .is_err()
    );
    let rows = outbox_types(&db).await;
    let types: Vec<&str> = rows.iter().map(|(_, t, _)| t.as_str()).collect();
    for expected in [
        "config.source_created",
        "config.source_rotated",
        "config.source_revoked",
        "connection.paired",
        "connection.disconnected_by_restart",
        "security.invitation_issued",
        "security.account_created",
        "identity.sessions_ended",
    ] {
        assert!(types.contains(&expected), "missing {expected}: {types:?}");
    }
    assert!(
        rows.iter()
            .all(|(_, _, p)| !p.contains("awk_") && !p.contains("awd_"))
    );
    assert!(
        rows.iter()
            .filter(|(_, t, _)| t == "security.invitation_issued")
            .all(|(u, _, _)| *u == alice.id.as_uuid()),
        "admin trail in the admin's log"
    );

    // Idle refresh is written for user-initiated requests only.
    let (cookie_b, _) = sign_in(&deps, "alice@example.com").await;
    AuthenticateSession::new(deps.clone())
        .handle(Some(&cookie_b), Requirement::Full, Activity::Background)
        .await
        .unwrap();

    drop(deps);
    drop(store);
    db.teardown().await;
}
