//! PostgreSQL integration tests for the identity store (data model §9), run as `allwin_app`
//! (never as superuser: it would bypass RLS). Each test creates a fresh database from
//! `TEST_DATABASE_ADMIN_URL`, migrates it as `allwin_owner` and drops it at the end.
//! Without `TEST_DATABASE_ADMIN_URL` the tests are skipped (with a note on stderr).
//! `ALLWIN_TEST_PRE_MIGRATIONS` (`:`-separated .sql files) supplies migrations that live on
//! another branch until integration. Role URLs derive from
//! `TEST_DATABASE_ADMIN_URL` with the fixed test-only passwords (common/mod.rs).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

mod common;

use chrono::Utc;
use common::*;
use identity_application::auth::{Activity, AuthenticateSession};
use identity_application::changes::{ChangeSet, CodeTable, Write};
use identity_application::error::IdentityError;
use identity_application::ports::*;
use identity_application::usecases::channel_change::*;
use identity_application::usecases::connections::*;
use identity_application::usecases::invitations::*;
use identity_application::usecases::pairing::{PairAdapter, PairAdapterCommand};
use identity_application::usecases::signin::*;
use identity_application::usecases::signup::*;
use identity_application::usecases::sources::*;
use identity_enterprise::channel::Channel;
use identity_enterprise::channel_change::ChangeStatus;
use identity_enterprise::connection::{AdapterBuild, AdapterInfo, LiveState};
use identity_enterprise::otp::{CodeRole, DeviceClass, StepUpAction};
use identity_enterprise::profile::Locale;
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
async fn rls_lint_sources_connections_and_pairing() {
    let Some(db) = setup().await else { return };
    let (deps, store, delivery) = deps(&db);

    // RLS lint (§3): every identity table with user_id forces RLS and has a policy.
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
    let (_, a) = sign_in(&db, &deps, &delivery, "alice@example.com", None).await;
    let (_, b) = sign_in(&db, &deps, &delivery, "Bob@Example.com", None).await;

    // Sources: step-up, isolation, rotation, revocation, resolve_source_key.
    let p = step_up(&db, &deps, &delivery, &a, StepUpAction::SourceCreate, None).await;
    let created = CreateSource::new(deps.clone())
        .handle(&a, "Main", SourceType::TradingView, Some(&p), &meta())
        .await
        .unwrap();
    let source_id = created.source.source.id;
    let first_key = created.source.source.key_id.clone();
    assert!(
        matches!(
            CreateSource::new(deps.clone())
                .handle(&a, "Other", SourceType::TradingView, Some(&p), &meta())
                .await,
            Err(IdentityError::CodeInvalid)
        ),
        "the code was consumed with the action"
    );
    assert!(
        store.source(bob.id, source_id).await.unwrap().is_none(),
        "B cannot read A's source"
    );
    assert!(matches!(
        RotateSource::new(deps.clone())
            .handle(&b, source_id, None, &meta())
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
            .is_none()
    );

    let p = step_up(
        &db,
        &deps,
        &delivery,
        &a,
        StepUpAction::SourceRotate,
        Some(source_id.as_uuid()),
    )
    .await;
    let rotated = RotateSource::new(deps.clone())
        .handle(&a, source_id, Some(&p), &meta())
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
        deps.hasher.sha256(secret.as_bytes()).as_bytes().to_vec()
    );
    RevokeSource::new(deps.clone())
        .handle(&a, source_id, &meta())
        .await
        .unwrap();
    assert!(resolve(new_key.as_str().into()).await.1);

    // Connections and pairing, including two concurrent redemptions of one code.
    let p = step_up(
        &db,
        &deps,
        &delivery,
        &a,
        StepUpAction::ConnectionCreate,
        None,
    )
    .await;
    let conn = CreateConnection::new(deps.clone())
        .handle(&a, "Desk A", Some(&p), &meta())
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
    assert_eq!(
        [r1.is_ok(), r2.is_ok()].iter().filter(|x| **x).count(),
        1,
        "exactly one wins"
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
    let p = step_up(
        &db,
        &deps,
        &delivery,
        &a,
        StepUpAction::ConnectionUnpair,
        Some(conn_id.as_uuid()),
    )
    .await;
    UnpairConnection::new(deps.clone())
        .handle(&a, conn_id, true, Some(&p), &meta())
        .await
        .unwrap();
    let (_, dev_revoked): (Uuid, bool) =
        sqlx::query_as("SELECT user_id, revoked FROM identity.resolve_device_token($1)")
            .bind(&token[4..16])
            .fetch_one(&db.app)
            .await
            .unwrap();
    assert!(dev_revoked);

    // Admin listing and the e-mail oracle gate.
    assert_eq!(
        store.users(alice.id).await.unwrap().len(),
        2,
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
            .unwrap()
    );

    // Outbox: contract audit codes, never a token or a code.
    let rows = outbox_types(&db).await;
    let types: Vec<&str> = rows.iter().map(|(_, t, _)| t.as_str()).collect();
    for expected in [
        "security.signed_in",
        "security.new_device_notice_sent",
        "identity.notice_requested",
        "security.step_up_succeeded",
        "sources.created",
        "sources.rotated",
        "sources.revoked",
        "connections.created",
        "connections.paired",
        "connection.status_changed",
        "connections.unpaired",
    ] {
        assert!(types.contains(&expected), "missing {expected}: {types:?}");
    }
    let codes: Vec<String> = delivery
        .take()
        .into_iter()
        .map(|j| j.delivery.code.expose().replace('-', ""))
        .collect();
    assert!(
        rows.iter()
            .all(|(_, _, p)| !p.contains("awk_") && !p.contains("awd_"))
    );
    assert!(
        rows.iter()
            .all(|(_, _, p)| codes.iter().all(|c| !p.contains(c.as_str()))),
        "no code in the outbox"
    );
    drop((deps, store));
    db.teardown().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn login_devices_and_sign_out_everywhere() {
    let Some(db) = setup().await else { return };
    let (deps, store, delivery) = deps(&db);
    let user = seed_user(&deps, "ana@example.com", false).await;
    // Unknown identifier: same answer, a placeholder statement, no row.
    let unknown = RequestLoginCode::new(deps.clone())
        .handle(RequestLoginCodeCommand {
            email: "nobody@example.com".into(),
            channel: Channel::Email,
            device_cookie: None,
            meta: meta(),
        })
        .await
        .unwrap();
    assert!(!unknown.challenge_id.expose().is_empty());
    let (first, ctx) = sign_in(&db, &deps, &delivery, "ana@example.com", None).await;
    assert!(first.new_device);
    reset_send_limits(&db).await;
    let cookie = first.device.value.expose().to_owned();
    let (second, ctx2) = sign_in(&db, &deps, &delivery, "ana@example.com", Some(&cookie)).await;
    assert!(!second.new_device);
    assert_eq!(ctx.session.device_id, ctx2.session.device_id);
    let devices = store.devices(user.id).await.unwrap();
    assert_eq!(devices.len(), 1);
    SignOutEverywhere::new(deps.clone())
        .handle(&ctx, &meta())
        .await
        .unwrap();
    for s in [&first, &second] {
        assert!(
            AuthenticateSession::new(deps.clone())
                .handle(Some(s.session.value.expose()), Activity::UserInitiated)
                .await
                .is_err()
        );
    }
    assert!(store.devices(user.id).await.unwrap().is_empty());
    reset_send_limits(&db).await;
    let (third, _) = sign_in(&db, &deps, &delivery, "ana@example.com", Some(&cookie)).await;
    assert!(third.new_device, "a revoked cookie counts as none");
    assert_ne!(third.device.value.expose(), cookie);
    // Durable send counters: one row per window, no address.
    let rows: Vec<(i32, i32)> = sqlx::query_as(
        "SELECT window_seconds, sends FROM identity.code_send_counter ORDER BY window_seconds",
    )
    .fetch_all(&db.inspect)
    .await
    .unwrap();
    assert_eq!(rows.len(), 3);
    let ident = deps.hasher.identifier_hmac("nobody@example.com");
    let counts = store
        .send_counts(&ident, DeviceClass::Unknown, &[86_400], Utc::now())
        .await
        .unwrap();
    assert_eq!(
        counts[0].sends, 0,
        "counters were reset above; unknown identifiers are counted like any"
    );
    drop((deps, store));
    db.teardown().await;
}

/// Lastro: two concurrent wrong guesses at 4 failures end at 5, never 6; a concurrent success and
/// a wrong guess leave the code consumed with the attempts unchanged.
#[tokio::test(flavor = "multi_thread")]
async fn code_verification_is_serialized_by_conditional_updates() {
    let Some(db) = setup().await else { return };
    let (deps, store, delivery) = deps(&db);
    let user = seed_user(&deps, "ana@example.com", false).await;
    let request = || {
        let d = deps.clone();
        async move {
            RequestLoginCode::new(d)
                .handle(RequestLoginCodeCommand {
                    email: "ana@example.com".into(),
                    channel: Channel::Email,
                    device_cookie: None,
                    meta: meta(),
                })
                .await
        }
    };
    let ch = request().await.unwrap();
    let claims = deps
        .hasher
        .open_challenge(ch.challenge_id.expose())
        .unwrap();
    let fail = || {
        store.commit(ChangeSet::new(user.id).write(Write::FailCode {
            table: CodeTable::OneTime,
            code_id: claims.code_id,
        }))
    };
    for _ in 0..4 {
        fail().await.unwrap();
    }
    let (x, y) = tokio::join!(fail(), fail());
    x.unwrap();
    y.unwrap();
    let row = store
        .one_time_code(user.id, claims.code_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.state.failed_attempts, 5);
    assert!(row.state.invalidated_at.is_some());

    reset_send_limits(&db).await;
    let ch = request().await.unwrap();
    let claims = deps
        .hasher
        .open_challenge(ch.challenge_id.expose())
        .unwrap();
    let row = store
        .one_time_code(user.id, claims.code_id)
        .await
        .unwrap()
        .unwrap();
    let consume = store.commit(ChangeSet::new(user.id).write(Write::ConsumeCode {
        table: CodeTable::OneTime,
        code_id: claims.code_id,
        code_hmac: row.state.code_hmac,
        at: Utc::now(),
    }));
    let wrong = store.commit(ChangeSet::new(user.id).write(Write::FailCode {
        table: CodeTable::OneTime,
        code_id: claims.code_id,
    }));
    let (c, w) = tokio::join!(consume, wrong);
    w.unwrap();
    match c {
        Ok(()) => {
            let row = store
                .one_time_code(user.id, claims.code_id)
                .await
                .unwrap()
                .unwrap();
            assert!(row.state.consumed_at.is_some());
            assert!(
                row.state.failed_attempts <= 1,
                "at most the one guess that ran first"
            );
        }
        Err(StoreError::Conflict(StoreConflict::CodeGone)) => {
            panic!("one wrong guess cannot close the code")
        }
        Err(e) => panic!("{e}"),
    }
    // A consumed code never accepts a failure or a second consume.
    let again = store
        .commit(ChangeSet::new(user.id).write(Write::ConsumeCode {
            table: CodeTable::OneTime,
            code_id: claims.code_id,
            code_hmac: row.state.code_hmac,
            at: Utc::now(),
        }))
        .await;
    assert!(matches!(
        again,
        Err(StoreError::Conflict(StoreConflict::CodeGone))
    ));
    let _ = delivery.take();
    drop((deps, store));
    db.teardown().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn sign_up_creates_one_user_even_with_concurrent_confirmations() {
    let Some(db) = setup().await else { return };
    let (deps, store, delivery) = deps(&db);
    seed_user(&deps, "admin@example.com", true).await;
    let (_, admin) = sign_in(&db, &deps, &delivery, "admin@example.com", None).await;
    let p = step_up(
        &db,
        &deps,
        &delivery,
        &admin,
        StepUpAction::AdminInvitationCreate,
        None,
    )
    .await;
    let created = CreateInvitation::new(deps.clone())
        .handle(
            &admin,
            CreateInvitationCommand {
                email: "carol@example.com".into(),
                locale: Locale::PtBr,
                send_email: true,
                step_up: Some(p),
                meta: meta(),
            },
        )
        .await
        .unwrap();
    assert!(!created.email_sent);
    let token = created
        .invite_url
        .expose()
        .split("#t=")
        .nth(1)
        .unwrap()
        .to_owned();
    let ch = StartSignup::new(deps.clone())
        .handle(&token, &meta())
        .await
        .unwrap();
    let code = delivery.last_code_to("carol@example.com").unwrap();
    let complete = || {
        let (d, challenge_id, code) = (
            deps.clone(),
            ch.challenge_id.expose().to_owned(),
            code.clone(),
        );
        async move {
            CompleteSignup::new(d)
                .handle(CompleteSignupCommand {
                    challenge_id,
                    code,
                    locale: Locale::PtBr,
                    time_zone: "America/Sao_Paulo".into(),
                    remember_me: false,
                    device_cookie: None,
                    meta: meta(),
                })
                .await
        }
    };
    let (r1, r2) = tokio::join!(complete(), complete());
    assert_eq!([r1.is_ok(), r2.is_ok()].iter().filter(|x| **x).count(), 1);
    assert!(
        [&r1, &r2]
            .iter()
            .any(|r| matches!(r, Err(IdentityError::CodeInvalid)))
    );
    let n: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM identity.app_user WHERE email_normalized = 'carol@example.com'",
    )
    .fetch_one(&db.inspect)
    .await
    .unwrap();
    assert_eq!(n, 1);
    let rows = outbox_types(&db).await;
    assert!(rows.iter().any(|(_, t, _)| t == "security.account_created"));
    assert!(
        rows.iter()
            .filter(|(_, t, _)| t == "security.invitation_issued")
            .all(|(u, _, p)| *u == admin.user.id.as_uuid()
                && p.contains("c***@e***.com")
                && !p.contains("carol@"))
    );
    drop((deps, store));
    db.teardown().await;
}

/// Lastro: a change code is refused once the change expired; holds apply through the worker.
#[tokio::test(flavor = "multi_thread")]
async fn channel_change_expiry_hold_and_worker() {
    let Some(db) = setup().await else { return };
    let (deps, store, delivery) = deps(&db);
    let user = seed_user(&deps, "ana@example.com", false).await;
    let (_, c) = sign_in(&db, &deps, &delivery, "ana@example.com", None).await;
    let start = |value: &str| {
        let (d, c, value) = (deps.clone(), c.clone(), value.to_owned());
        async move {
            StartChannelChange::new(d)
                .handle(
                    &c,
                    StartChannelChangeCommand {
                        channel: Channel::Email,
                        new_value: value,
                        meta: meta(),
                    },
                )
                .await
        }
    };
    reset_send_limits(&db).await;
    let view = start("new@example.com").await.unwrap();
    // Expire the change behind the code's back (a resend may outlive the change).
    sqlx::query("UPDATE identity.channel_change SET expires_at = created_at + interval '1 second' WHERE id = $1")
        .bind(view.id.as_uuid())
        .execute(&db.inspect)
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
    let current = view
        .codes
        .iter()
        .find(|x| x.role == CodeRole::CurrentValue)
        .unwrap();
    let r = VerifyChannelChangeCode::new(deps.clone())
        .handle(
            &c,
            VerifyChannelChangeCodeCommand {
                change_id: view.id,
                challenge_id: current.challenge_id.expose().to_owned(),
                code: delivery.last_code_to("ana@example.com").unwrap(),
                meta: meta(),
            },
        )
        .await;
    assert!(matches!(r, Err(IdentityError::ChannelChangeClosed)));

    // A new change from this (new) device goes on hold after both codes.
    reset_send_limits(&db).await;
    let view = start("new@example.com").await.unwrap();
    for (role, to) in [
        (CodeRole::CurrentValue, "ana@example.com"),
        (CodeRole::NewValue, "new@example.com"),
    ] {
        let code = view.codes.iter().find(|x| x.role == role).unwrap();
        VerifyChannelChangeCode::new(deps.clone())
            .handle(
                &c,
                VerifyChannelChangeCodeCommand {
                    change_id: view.id,
                    challenge_id: code.challenge_id.expose().to_owned(),
                    code: delivery.last_code_to(to).unwrap(),
                    meta: meta(),
                },
            )
            .await
            .unwrap();
    }
    let held = store
        .channel_change(user.id, view.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(held.status, ChangeStatus::OnHold);
    assert_eq!(
        ApplyDueChannelHolds::new(deps.clone())
            .handle()
            .await
            .unwrap(),
        0
    );
    sqlx::query(
        "UPDATE identity.channel_change SET hold_until = now() - interval '1 minute' WHERE id = $1",
    )
    .bind(view.id.as_uuid())
    .execute(&db.inspect)
    .await
    .unwrap();
    assert_eq!(
        ApplyDueChannelHolds::new(deps.clone())
            .handle()
            .await
            .unwrap(),
        1
    );
    let u = store.user(user.id).await.unwrap().unwrap();
    assert_eq!(u.email.normalized(), "new@example.com");
    assert!(
        store.devices(user.id).await.unwrap().is_empty(),
        "M2: devices revoked"
    );
    assert!(
        AuthenticateSession::new(deps.clone())
            .handle(None, Activity::UserInitiated)
            .await
            .is_err()
    );
    drop((deps, store));
    db.teardown().await;
}
