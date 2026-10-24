//! PostgreSQL tests of the admin recovery (S3 AC-12; migration `…061800`, data model rev. 16.8):
//! the admin's step-up and the target's change in one transaction under two RLS scopes, the
//! stored row, the outbox actor, the worker's effect, and the new CHECKs in both directions.
//! Same setup as `pg_store.rs` (`TEST_DATABASE_ADMIN_URL`, skipped without it).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

mod common;

use common::*;
use identity_application::usecases::admin_recovery::*;
use identity_application::usecases::channel_change::ApplyDueChannelHolds;
use identity_enterprise::channel_change::{ChangeStatus, VerificationRef, VerifiedBy};
use identity_enterprise::user::User;
use sqlx::Row;
use uuid::Uuid;

async fn resolve(
    deps: &identity_application::deps::IdentityDeps,
    admin: Option<&str>,
    glass: bool,
    user: &str,
) -> (Operator, User) {
    ResolveOperator::new(deps.clone())
        .handle(ResolveOperatorCommand {
            admin_email: admin.map(str::to_owned),
            break_glass: glass,
            user: UserRef::Email(user.into()),
            host: identity_application::changes::HostReport::parse(Some("ops"), None, None),
        })
        .await
        .unwrap()
}

async fn admin_proof(
    db: &Db,
    deps: &identity_application::deps::IdentityDeps,
    delivery: &CapturingDelivery,
    op: &Operator,
    admin_email: &str,
    target: Uuid,
) -> StepUpProof {
    reset_send_limits(db).await;
    let ch = RequestOperatorStepUp::new(deps.clone())
        .handle(op, target)
        .await
        .unwrap();
    StepUpProof {
        challenge_id: ch.challenge_id.expose().to_owned(),
        code: delivery.last_code_to(admin_email).unwrap(),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn recovery_runs_under_both_scopes_and_the_worker_applies_it() {
    let Some(db) = setup().await else { return };
    let (deps, _, delivery) = deps(&db);
    let admin = seed_user(&deps, "root@example.com", true).await;
    let user = seed_user(&deps, "ana@example.com", false).await;
    let (signed, _) = sign_in(&db, &deps, &delivery, "ana@example.com", None).await;
    let (op, target) = resolve(&deps, Some("root@example.com"), false, "ana@example.com").await;

    let p = admin_proof(
        &db,
        &deps,
        &delivery,
        &op,
        "root@example.com",
        user.id.as_uuid(),
    )
    .await;
    reset_send_limits(&db).await;
    let view = StartAdminRecovery::new(deps.clone())
        .handle(
            &op,
            &target,
            StartRecoveryCommand {
                new_email: "ana.new@example.com".into(),
                verified_by: VerifiedBy::PhoneKnownNumber,
                verification_ref: VerificationRef::parse("ticket 4521").unwrap(),
                step_up: Some(p),
            },
        )
        .await
        .unwrap();
    let row = sqlx::query(
        "SELECT kind, requested_session_id, admin_user_id, verified_by, verification_ref, break_glass FROM identity.channel_change WHERE id = $1",
    )
    .bind(view.change_id.as_uuid())
    .fetch_one(&db.inspect)
    .await
    .unwrap();
    assert_eq!(row.get::<String, _>("kind"), "admin_recovery");
    assert_eq!(row.get::<Option<Uuid>, _>("requested_session_id"), None);
    assert_eq!(row.get::<Uuid, _>("admin_user_id"), admin.id.as_uuid());
    assert_eq!(row.get::<String, _>("verified_by"), "phone_known_number");
    assert_eq!(row.get::<String, _>("verification_ref"), "ticket 4521");
    assert!(!row.get::<bool, _>("break_glass"));
    // The admin's code is consumed in the same commit.
    let consumed: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM identity.one_time_code WHERE user_id = $1 AND action = 'admin_recovery' AND consumed_at IS NOT NULL",
    )
    .bind(admin.id.as_uuid())
    .fetch_one(&db.inspect)
    .await
    .unwrap();
    assert_eq!(consumed, 1);

    let p = admin_proof(
        &db,
        &deps,
        &delivery,
        &op,
        "root@example.com",
        view.change_id.as_uuid(),
    )
    .await;
    let code = delivery.last_code_to("ana.new@example.com").unwrap();
    let held = ConfirmAdminRecovery::new(deps.clone())
        .handle(
            &op,
            &target,
            ConfirmRecoveryCommand {
                change_id: view.change_id,
                code,
                step_up: Some(p),
            },
        )
        .await
        .unwrap();
    assert_eq!(held.status, ChangeStatus::OnHold);

    // The target's trail says platform_admin and never names the admin; the admin's has the step-ups.
    let rows = sqlx::query("SELECT user_id, event_type, payload FROM app.outbox ORDER BY id")
        .fetch_all(&db.inspect)
        .await
        .unwrap();
    let of = |user: Uuid, ty: &str| {
        rows.iter()
            .filter(|r| {
                r.get::<Uuid, _>("user_id") == user && r.get::<String, _>("event_type") == ty
            })
            .map(|r| r.get::<serde_json::Value, _>("payload"))
            .collect::<Vec<_>>()
    };
    let requested = of(user.id.as_uuid(), "security.channel_change_requested");
    assert_eq!(requested.len(), 1);
    assert_eq!(requested[0]["actor"], "platform_admin");
    assert_eq!(requested[0]["details"]["kind"], "admin_recovery");
    assert_eq!(requested[0]["details"]["verified_by"], "phone_known_number");
    // The user's trail = the audit codes in their outbox; the operator event is never audited.
    for r in rows.iter().filter(|r| {
        r.get::<Uuid, _>("user_id") == user.id.as_uuid()
            && identity_application::changes::AUDIT_EVENT_TYPES
                .contains(&r.get::<String, _>("event_type").as_str())
    }) {
        let text = r.get::<serde_json::Value, _>("payload").to_string();
        assert!(
            !text.contains(&admin.id.as_uuid().to_string()),
            "admin id in the user's trail"
        );
    }
    assert_eq!(
        of(admin.id.as_uuid(), "security.step_up_succeeded").len(),
        2
    );
    // The operator log rides on each commit, owned by the target, never as an audit code.
    let ops = of(user.id.as_uuid(), "identity.operator_recovery");
    assert_eq!(ops.len(), 2, "start and confirm");
    assert_eq!(ops[0]["step"], "start");
    assert_eq!(ops[1]["step"], "confirm");
    assert_eq!(ops[0]["host_reported"]["login"], "ops");
    assert_eq!(ops[0]["admin_user_id"], admin.id.as_uuid().to_string());

    // Make the hold due and let the worker apply it.
    sqlx::query(
        "UPDATE identity.channel_change SET hold_until = now() - interval '1 second' WHERE id = $1",
    )
    .bind(view.change_id.as_uuid())
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
    let email: String = sqlx::query_scalar("SELECT email FROM identity.app_user WHERE id = $1")
        .bind(user.id.as_uuid())
        .fetch_one(&db.inspect)
        .await
        .unwrap();
    assert_eq!(email, "ana.new@example.com");
    let open_sessions: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM identity.session WHERE user_id = $1 AND ended_at IS NULL",
    )
    .bind(user.id.as_uuid())
    .fetch_one(&db.inspect)
    .await
    .unwrap();
    assert_eq!(open_sessions, 0);
    let live_devices: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM identity.known_device WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(user.id.as_uuid())
    .fetch_one(&db.inspect)
    .await
    .unwrap();
    assert_eq!(live_devices, 0);
    drop(signed);
    db.teardown().await;
}

/// Inserts a channel_change row as superuser and reports whether the CHECKs accepted it.
async fn change_ok(db: &Db, user: Uuid, cols: &str, vals: &str) -> bool {
    let sql = format!(
        "INSERT INTO identity.channel_change (id, user_id, channel, new_value, expires_at, {cols})
         VALUES (gen_random_uuid(), '{user}', 'email', 'x@example.com', now() + interval '5 minutes', {vals})"
    );
    let mut tx = db.inspect.begin().await.unwrap();
    let ok = sqlx::query(sqlx::AssertSqlSafe(sql))
        .execute(&mut *tx)
        .await
        .is_ok();
    tx.rollback().await.unwrap();
    ok
}

/// Inserts a one_time_code row as superuser and reports whether the CHECKs accepted it.
async fn code_ok(db: &Db, user: Uuid, cols: &str, vals: &str) -> bool {
    let sql = format!(
        "INSERT INTO identity.one_time_code (id, user_id, channel, destination, code_hmac, pepper_version, expires_at, {cols})
         VALUES (gen_random_uuid(), '{user}', 'email', 'x@example.com', decode(repeat('ab', 32), 'hex'), 1, now() + interval '5 minutes', {vals})"
    );
    let mut tx = db.inspect.begin().await.unwrap();
    let ok = sqlx::query(sqlx::AssertSqlSafe(sql))
        .execute(&mut *tx)
        .await
        .is_ok();
    tx.rollback().await.unwrap();
    ok
}

#[tokio::test(flavor = "multi_thread")]
async fn migration_checks_hold_in_both_directions() {
    let Some(db) = setup().await else { return };
    let (deps, _, delivery) = deps(&db);
    let admin = seed_user(&deps, "root@example.com", true).await;
    let user = seed_user(&deps, "ana@example.com", false).await;
    let (_, c) = sign_in(&db, &deps, &delivery, "ana@example.com", None).await;
    let (a, u, s) = (
        admin.id.as_uuid(),
        user.id.as_uuid(),
        c.session.id.as_uuid(),
    );
    let rec = |extra: &str| format!("'admin_recovery', '{a}', 'video_call', 'ticket 1'{extra}");
    let cols = "kind, admin_user_id, verified_by, verification_ref";

    // channel_change
    assert!(change_ok(&db, u, cols, &rec("")).await);
    assert!(
        change_ok(&db, u, "requested_session_id", &format!("'{s}'")).await,
        "user_change"
    );
    assert!(
        !change_ok(
            &db,
            u,
            "kind, verified_by, verification_ref",
            "'admin_recovery', 'video_call', 'x'"
        )
        .await,
        "no admin"
    );
    assert!(
        !change_ok(
            &db,
            u,
            "kind, admin_user_id, verification_ref",
            &format!("'admin_recovery', '{a}', 'x'")
        )
        .await,
        "no verified_by"
    );
    assert!(
        !change_ok(
            &db,
            u,
            &format!("{cols}, requested_session_id"),
            &format!("{}, '{s}'", rec(""))
        )
        .await,
        "recovery with a session"
    );
    assert!(
        !change_ok(
            &db,
            u,
            "requested_session_id, admin_user_id",
            &format!("'{s}', '{a}'")
        )
        .await,
        "user_change with recovery fields"
    );
    assert!(
        !change_ok(
            &db,
            u,
            "requested_session_id, break_glass",
            &format!("'{s}', true")
        )
        .await,
        "user_change break_glass"
    );
    assert!(
        !change_ok(&db, u, &format!("{cols}, break_glass"), &rec(", true")).await,
        "break_glass on someone else"
    );
    let own = format!("'admin_recovery', '{a}', 'video_call', 'ticket 1'");
    assert!(
        change_ok(
            &db,
            a,
            &format!("{cols}, break_glass"),
            &format!("{own}, true")
        )
        .await,
        "break_glass on self"
    );
    assert!(
        !change_ok(&db, a, cols, &own).await,
        "self without break_glass"
    );
    let token = format!("awk_{}_{}", "A".repeat(12), "b".repeat(43));
    assert!(
        !change_ok(
            &db,
            u,
            cols,
            &format!("'admin_recovery', '{a}', 'video_call', 'x {token}'")
        )
        .await,
        "token in ref"
    );
    assert!(
        !change_ok(
            &db,
            u,
            cols,
            &format!(
                "'admin_recovery', '{a}', 'video_call', '{}'",
                "a".repeat(201)
            )
        )
        .await,
        "ref > 200"
    );

    // one_time_code: the admin's step-up has no session, only for admin_recovery.
    let t = Uuid::now_v7();
    assert!(
        code_ok(
            &db,
            a,
            "purpose, action, target_id",
            &format!("'step_up', 'admin_recovery', '{t}'")
        )
        .await
    );
    assert!(
        !code_ok(
            &db,
            a,
            "purpose, action, target_id, session_id",
            &format!("'step_up', 'admin_recovery', '{t}', '{s}'")
        )
        .await,
        "admin_recovery with a session"
    );
    assert!(
        !code_ok(&db, a, "purpose, action", "'step_up', 'source_create'").await,
        "other step-up without a session"
    );
    assert!(
        !code_ok(&db, a, "purpose, action", "'step_up', 'admin_recovery'").await,
        "admin_recovery without target"
    );
    assert!(
        code_ok(
            &db,
            u,
            "purpose, action, session_id",
            &format!("'step_up', 'source_create', '{s}'")
        )
        .await
    );

    // A session-less channel-change code is only the new-value one.
    let change = Uuid::now_v7();
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "INSERT INTO identity.channel_change (id, user_id, channel, new_value, expires_at, {cols})
         VALUES ('{change}', '{u}', 'email', 'x@example.com', now() + interval '5 minutes', {})",
        rec("")
    )))
    .execute(&db.inspect)
    .await
    .unwrap();
    let cc = |role: &str| format!("'channel_change', '{change}', '{role}'");
    let cc_cols = "purpose, channel_change_id, code_role";
    assert!(code_ok(&db, u, cc_cols, &cc("new_value")).await);
    assert!(
        !code_ok(&db, u, cc_cols, &cc("current_value")).await,
        "current_value without a session"
    );
    assert!(
        !code_ok(&db, u, cc_cols, &cc("other_channel")).await,
        "other_channel without a session"
    );
    assert!(
        !code_ok(&db, u, "purpose, session_id", &format!("'login', '{s}'")).await,
        "login with a session"
    );
    db.teardown().await;
}
