//! Use-case behaviour through fakes: authorization, isolation (X-2), step-up, throttling
//! seams, one-time secrets and atomic effects.

use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration as StdDuration;

use chrono::Duration;
use identity_enterprise::connection::{AdapterBuild, AdapterInfo, ConnectionStatus, LiveState};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::{ConnectionId, SourceId};
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::EndReason;
use identity_enterprise::source::SourceType;
use identity_enterprise::totp;
use identity_enterprise::user::{PasswordHash, User, UserStatus};
use shared_kernel::{Clock, UserId};
use uuid::Uuid;

use crate::auth::{Activity, AuthContext, AuthenticateSession, Requirement};
use crate::changes::{EndedSessions, IdentityEvent, Write};
use crate::deps::RequestMeta;
use crate::error::IdentityError;
use crate::ports::ConnectionAccounts;
use crate::testing::{ORIGIN, World};
use crate::throttle::PairingDecision;
use crate::usecases::admin::{AdminResetTotp, ListUsers};
use crate::usecases::connections::*;
use crate::usecases::invitations::*;
use crate::usecases::me::*;
use crate::usecases::pairing::{PairAdapter, PairAdapterCommand};
use crate::usecases::password_reset::*;
use crate::usecases::signin::*;
use crate::usecases::sources::*;
use crate::usecases::totp::*;
use crate::views::LoginOutcome;

const PASSWORD: &str = "correct horse battery";

fn meta() -> RequestMeta {
    RequestMeta {
        ip: IpAddr::V4(Ipv4Addr::new(203, 0, 113, 7)),
        user_agent: Some("test".into()),
    }
}

fn add_user(w: &World, email: &str, admin: bool) -> User {
    let email = EmailAddress::parse(email).unwrap();
    let user = User {
        id: UserId::new_v7(),
        email,
        password_hash: PasswordHash::new(format!("h:{PASSWORD}")),
        locale: Locale::PtBr,
        time_zone: TimeZoneName::parse("America/Sao_Paulo").unwrap(),
        is_platform_admin: admin,
        status: UserStatus::Active,
        created_at: w.clock.now(),
    };
    w.store.insert_user(user.clone());
    user
}

async fn login(w: &World, email: &str) -> String {
    let out = Login::new(w.deps.clone())
        .handle(LoginCommand {
            email: email.into(),
            password: SecretString::new(PASSWORD),
            remember_me: false,
            meta: meta(),
        })
        .await
        .unwrap();
    match out {
        LoginOutcome::Authenticated { cookie, .. } | LoginOutcome::Pending2fa { cookie, .. } => {
            cookie.value.expose().to_owned()
        }
    }
}

async fn ctx(w: &World, cookie: &str) -> Result<AuthContext, IdentityError> {
    AuthenticateSession::new(w.deps.clone())
        .handle(Some(cookie), Requirement::Full, Activity::UserInitiated)
        .await
}

async fn signed_in(w: &World, email: &str, admin: bool) -> (User, String, AuthContext) {
    let user = add_user(w, email, admin);
    let cookie = login(w, email).await;
    let c = ctx(w, &cookie).await.unwrap();
    (user, cookie, c)
}

fn adapter(version: &str) -> AdapterInfo {
    AdapterInfo {
        adapter_type: "ninjatrader8".into(),
        version: version.into(),
        build: AdapterBuild::Release,
    }
}

fn pair_cmd(code: &str) -> PairAdapterCommand {
    PairAdapterCommand {
        pairing_code: code.into(),
        instance_id: Uuid::now_v7(),
        instance_name: "Desk PC".into(),
        adapter: adapter("1.0.0"),
        ip: meta().ip,
    }
}

fn debug_of_everything(w: &World) -> String {
    format!("{:?}", w.store.committed())
}

// ---------- sign-in ----------

#[tokio::test]
async fn login_failures_are_identical_and_cost_one_hash_each() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    let closed = add_user(&w, "closed@example.com", false);
    w.store
        .state
        .lock()
        .unwrap()
        .users
        .get_mut(&closed.id)
        .unwrap()
        .status = UserStatus::Closed;
    let login = Login::new(w.deps.clone());
    for (email, pw) in [
        ("nobody@example.com", PASSWORD),
        ("ana@example.com", "wrong password!!"),
        ("closed@example.com", PASSWORD),
    ] {
        let err = login
            .handle(LoginCommand {
                email: email.into(),
                password: SecretString::new(pw),
                remember_me: false,
                meta: meta(),
            })
            .await
            .unwrap_err();
        assert!(matches!(err, IdentityError::InvalidCredentials), "{email}");
    }
    assert_eq!(
        *w.passwords.dummy_verifications.lock().unwrap(),
        1,
        "unknown e-mail runs the dummy hash"
    );
    assert_eq!(*w.passwords.verifications.lock().unwrap(), 2);
    assert_eq!(w.login_throttle.failures.lock().unwrap().len(), 3);
    assert!(w.login_throttle.failures.lock().unwrap()[0].user.is_none());
}

#[tokio::test]
async fn throttled_login_does_not_verify_the_password() {
    let w = World::new();
    add_user(&w, "ana@example.com", false);
    *w.login_throttle.limited.lock().unwrap() = true;
    let err = Login::new(w.deps.clone())
        .handle(LoginCommand {
            email: "ana@example.com".into(),
            password: SecretString::new(PASSWORD),
            remember_me: false,
            meta: meta(),
        })
        .await
        .unwrap_err();
    assert!(
        matches!(err, IdentityError::RateLimited { retry_after } if retry_after == StdDuration::from_secs(30))
    );
    assert_eq!(*w.passwords.verifications.lock().unwrap(), 0);
}

#[tokio::test]
async fn login_starts_a_full_session_that_expires_when_idle_and_ignores_background_activity() {
    let w = World::new();
    let (_, cookie, _) = signed_in(&w, "Ana@Example.com", false).await;
    w.clock.advance(Duration::hours(11));
    AuthenticateSession::new(w.deps.clone())
        .handle(Some(&cookie), Requirement::Full, Activity::Background)
        .await
        .unwrap();
    w.clock.advance(Duration::hours(1));
    assert!(matches!(
        ctx(&w, &cookie).await,
        Err(IdentityError::Unauthenticated)
    ));
    let ended = w
        .store
        .committed()
        .into_iter()
        .flat_map(|c| c.events)
        .any(|e| {
            matches!(
                e,
                IdentityEvent::SessionsEnded {
                    reason: EndReason::Expired,
                    sessions: EndedSessions::One(_),
                    ..
                }
            )
        });
    assert!(ended, "expiry is announced on the SSE stream");
}

#[tokio::test]
async fn garbage_or_missing_cookie_is_unauthenticated() {
    let w = World::new();
    let auth = AuthenticateSession::new(w.deps.clone());
    for cookie in [
        None,
        Some(""),
        Some("not-a-token"),
        Some("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
    ] {
        assert!(matches!(
            auth.handle(cookie, Requirement::Full, Activity::UserInitiated)
                .await,
            Err(IdentityError::Unauthenticated)
        ));
    }
}

async fn enable_totp(w: &World, c: &AuthContext) -> (Vec<u8>, Vec<SecretString>) {
    let setup = StartTotpSetup::new(w.deps.clone()).handle(c).await.unwrap();
    let secret = base32_decode(setup.secret.expose());
    assert!(
        setup
            .otpauth_uri
            .expose()
            .starts_with("otpauth://totp/All%20Win:")
    );
    let code = totp::code_at(&secret, w.clock.now()).unwrap();
    let codes = ConfirmTotpSetup::new(w.deps.clone())
        .handle(c, &code, &meta())
        .await
        .unwrap();
    (secret, codes.codes)
}

fn base32_decode(s: &str) -> Vec<u8> {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let (mut buf, mut bits, mut out) = (0u32, 0u32, Vec::new());
    for c in s.bytes() {
        buf = (buf << 5) | alphabet.iter().position(|a| *a == c).unwrap() as u32;
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    out
}

#[tokio::test]
async fn totp_sign_in_is_pending_then_rotated_and_replay_is_rejected() {
    let w = World::new();
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let (secret, recovery) = enable_totp(&w, &c).await;
    assert_eq!(recovery.len(), 10);
    w.clock.advance(Duration::seconds(90));

    let pending = login(&w, "ana@example.com").await;
    assert!(
        matches!(ctx(&w, &pending).await, Err(IdentityError::Unauthenticated)),
        "pending grants nothing else"
    );
    let auth = AuthenticateSession::new(w.deps.clone());
    let pctx = auth
        .handle(
            Some(&pending),
            Requirement::Pending,
            Activity::UserInitiated,
        )
        .await
        .unwrap();
    let code = totp::code_at(&secret, w.clock.now()).unwrap();
    let verify = VerifyLoginTotp::new(w.deps.clone());
    assert!(matches!(
        verify.handle(&pctx, "000000", &meta()).await,
        Err(IdentityError::InvalidCode)
    ));
    let out = verify.handle(&pctx, &code, &meta()).await.unwrap();
    let LoginOutcome::Authenticated { cookie, .. } = out else {
        panic!("expected full session")
    };
    assert!(ctx(&w, cookie.value.expose()).await.is_ok());
    assert!(
        auth.handle(
            Some(&pending),
            Requirement::Pending,
            Activity::UserInitiated
        )
        .await
        .is_err(),
        "rotated"
    );

    // Same code again on a new pending session: replay.
    let pending2 = login(&w, "ana@example.com").await;
    let pctx2 = auth
        .handle(
            Some(&pending2),
            Requirement::Pending,
            Activity::UserInitiated,
        )
        .await
        .unwrap();
    assert!(matches!(
        verify.handle(&pctx2, &code, &meta()).await,
        Err(IdentityError::InvalidCode)
    ));

    // Recovery code: works once.
    let rc = VerifyLoginRecoveryCode::new(w.deps.clone());
    let first = recovery[0].expose().to_lowercase().replace('-', " ");
    let out = rc.handle(&pctx2, &first, &meta()).await.unwrap();
    assert!(matches!(
        out,
        LoginOutcome::Authenticated {
            recovery_codes_remaining: Some(9),
            ..
        }
    ));
    let pending3 = login(&w, "ana@example.com").await;
    let pctx3 = auth
        .handle(
            Some(&pending3),
            Requirement::Pending,
            Activity::UserInitiated,
        )
        .await
        .unwrap();
    assert!(matches!(
        rc.handle(&pctx3, &first, &meta()).await,
        Err(IdentityError::InvalidCode)
    ));
}

#[tokio::test]
async fn pending_session_expires_after_five_minutes() {
    let w = World::new();
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    enable_totp(&w, &c).await;
    let pending = login(&w, "ana@example.com").await;
    w.clock.advance(Duration::minutes(5));
    let r = AuthenticateSession::new(w.deps.clone())
        .handle(
            Some(&pending),
            Requirement::Pending,
            Activity::UserInitiated,
        )
        .await;
    assert!(matches!(r, Err(IdentityError::PendingExpired)));
}

#[tokio::test]
async fn enabling_totp_ends_other_sessions_only() {
    let w = World::new();
    let (_, cookie, c) = signed_in(&w, "ana@example.com", false).await;
    let other = login(&w, "ana@example.com").await;
    enable_totp(&w, &c).await;
    assert!(ctx(&w, &cookie).await.is_ok());
    assert!(ctx(&w, &other).await.is_err());
}

#[tokio::test]
async fn step_up_window_and_reauthentication() {
    let w = World::new();
    let (_, cookie, _) = signed_in(&w, "ana@example.com", false).await;
    w.clock.advance(Duration::minutes(16));
    let c = ctx(&w, &cookie).await.unwrap();
    let create = CreateSource::new(w.deps.clone());
    assert!(matches!(
        create
            .handle(&c, "Main", SourceType::TradingView, &meta())
            .await,
        Err(IdentityError::StepUpRequired)
    ));
    let step = StepUp::new(w.deps.clone());
    let bad = StepUpCommand {
        password: SecretString::new("nope nope nope"),
        code: None,
        meta: meta(),
    };
    assert!(matches!(
        step.handle(&c, bad).await,
        Err(IdentityError::ReauthFailed)
    ));
    step.handle(
        &c,
        StepUpCommand {
            password: SecretString::new(PASSWORD),
            code: None,
            meta: meta(),
        },
    )
    .await
    .unwrap();
    let c = ctx(&w, &cookie).await.unwrap();
    assert!(
        create
            .handle(&c, "Main", SourceType::TradingView, &meta())
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn step_up_for_totp_user_requires_the_code() {
    let w = World::new();
    let (_, cookie, c) = signed_in(&w, "ana@example.com", false).await;
    let (secret, _) = enable_totp(&w, &c).await;
    w.clock.advance(Duration::minutes(1));
    let c = ctx(&w, &cookie).await.unwrap();
    let step = StepUp::new(w.deps.clone());
    let no_code = StepUpCommand {
        password: SecretString::new(PASSWORD),
        code: None,
        meta: meta(),
    };
    assert!(matches!(
        step.handle(&c, no_code).await,
        Err(IdentityError::InvalidCode)
    ));
    let code = totp::code_at(&secret, w.clock.now()).unwrap();
    step.handle(
        &c,
        StepUpCommand {
            password: SecretString::new(PASSWORD),
            code: Some(code),
            meta: meta(),
        },
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn change_password_rotates_this_session_and_ends_the_others() {
    let w = World::new();
    let (_, cookie, c) = signed_in(&w, "ana@example.com", false).await;
    let other = login(&w, "ana@example.com").await;
    let cmd = ChangePasswordCommand {
        current_password: SecretString::new(PASSWORD),
        new_password: SecretString::new("a brand new passphrase"),
        meta: meta(),
    };
    let new_cookie = ChangePassword::new(w.deps.clone())
        .handle(&c, cmd)
        .await
        .unwrap();
    assert!(ctx(&w, &cookie).await.is_err());
    assert!(ctx(&w, &other).await.is_err());
    assert!(ctx(&w, new_cookie.value.expose()).await.is_ok());
    let weak = ChangePasswordCommand {
        current_password: SecretString::new("a brand new passphrase"),
        new_password: SecretString::new("short"),
        meta: meta(),
    };
    let c2 = ctx(&w, new_cookie.value.expose()).await.unwrap();
    assert!(matches!(
        ChangePassword::new(w.deps.clone()).handle(&c2, weak).await,
        Err(IdentityError::PasswordRejected {
            pointer: "/new_password",
            ..
        })
    ));
}

#[tokio::test]
async fn logout_is_idempotent() {
    let w = World::new();
    let (_, cookie, _) = signed_in(&w, "ana@example.com", false).await;
    let logout = Logout::new(w.deps.clone());
    logout.handle(Some(&cookie)).await.unwrap();
    logout.handle(Some(&cookie)).await.unwrap();
    logout.handle(None).await.unwrap();
    assert!(ctx(&w, &cookie).await.is_err());
}

// ---------- invitations and reset ----------

#[tokio::test]
async fn invitation_flow_admin_only_one_time_and_reinvite_after_expiry() {
    let w = World::new();
    let (_, _, admin) = signed_in(&w, "admin@example.com", true).await;
    let (_, _, plain) = signed_in(&w, "plain@example.com", false).await;
    let create = CreateInvitation::new(w.deps.clone());
    let cmd = |email: &str| CreateInvitationCommand {
        email: email.into(),
        locale: Locale::En,
        send_email: true,
        meta: meta(),
    };
    assert!(matches!(
        create.handle(&plain, cmd("new@example.com")).await,
        Err(IdentityError::Forbidden)
    ));
    assert!(matches!(
        create.handle(&admin, cmd("plain@example.com")).await,
        Err(IdentityError::InvitationEmailTaken)
    ));

    let created = create.handle(&admin, cmd("New@Example.com")).await.unwrap();
    let url = created.invite_url.expose().to_owned();
    assert!(
        url.starts_with(&format!("{ORIGIN}/en/invite#t=")),
        "token in the fragment"
    );
    assert!(created.email_sent);
    assert!(matches!(
        create.handle(&admin, cmd("new@example.com")).await,
        Err(IdentityError::InvitationEmailTaken)
    ));
    let token = url.split("#t=").nth(1).unwrap().to_owned();

    let lookup = LookupInvitation::new(w.deps.clone())
        .handle(&token, &meta())
        .await
        .unwrap();
    assert_eq!(lookup.email, "new@example.com");
    let accept = AcceptInvitation::new(w.deps.clone());
    let acmd = |tz: &str, pw: &str| AcceptInvitationCommand {
        token: token.clone(),
        password: SecretString::new(pw),
        locale: Locale::En,
        time_zone: tz.into(),
        meta: meta(),
    };
    assert!(matches!(
        accept.handle(acmd("Mars/Base", PASSWORD)).await,
        Err(IdentityError::InvalidRequest {
            pointer: "/time_zone"
        })
    ));
    assert!(
        matches!(
            accept.handle(acmd("UTC", "new-password-xx")).await,
            Err(IdentityError::PasswordRejected { .. })
        ),
        "contains local part"
    );
    let out = accept.handle(acmd("UTC", PASSWORD)).await.unwrap();
    let LoginOutcome::Authenticated { cookie, user, .. } = out else {
        panic!()
    };
    assert!(!user.is_platform_admin);
    assert!(ctx(&w, cookie.value.expose()).await.is_ok());
    assert!(matches!(
        accept.handle(acmd("UTC", PASSWORD)).await,
        Err(IdentityError::InvitationInvalid)
    ));
    assert!(matches!(
        LookupInvitation::new(w.deps.clone())
            .handle(&token, &meta())
            .await,
        Err(IdentityError::InvitationInvalid)
    ));

    // Expired open invitation is superseded by a new one.
    let late = create
        .handle(&admin, cmd("late@example.com"))
        .await
        .unwrap();
    w.clock.advance(Duration::days(8));
    let admin = ctx(&w, &login(&w, "admin@example.com").await)
        .await
        .unwrap();
    create
        .handle(&admin, cmd("late@example.com"))
        .await
        .unwrap();
    let list = ListInvitations::new(w.deps.clone())
        .handle(&admin)
        .await
        .unwrap();
    let old = list
        .iter()
        .find(|v| v.invitation.id == late.invitation.invitation.id)
        .unwrap();
    assert_eq!(
        old.status,
        identity_enterprise::invitation::InvitationStatus::Expired
    );
    assert!(old.invitation.superseded_at.is_some());
}

#[tokio::test]
async fn invitation_mail_failure_still_returns_the_link() {
    let w = World::new();
    let (_, _, admin) = signed_in(&w, "admin@example.com", true).await;
    *w.mailer.fail.lock().unwrap() = true;
    let created = CreateInvitation::new(w.deps.clone())
        .handle(
            &admin,
            CreateInvitationCommand {
                email: "x@example.com".into(),
                locale: Locale::PtBr,
                send_email: true,
                meta: meta(),
            },
        )
        .await
        .unwrap();
    assert!(!created.email_sent);
    assert!(created.invite_url.expose().contains("/pt-BR/invite#t="));
}

#[tokio::test]
async fn password_reset_never_reveals_accounts_and_ends_every_session() {
    let w = World::new();
    let (_, cookie, _) = signed_in(&w, "ana@example.com", false).await;
    let request = RequestPasswordReset::new(w.deps.clone());
    request.handle("nobody@example.com", &meta()).await.unwrap();
    assert!(w.mailer.sent.lock().unwrap().is_empty());
    request.handle("ana@example.com", &meta()).await.unwrap();
    request.handle("ana@example.com", &meta()).await.unwrap();
    let sent = w.mailer.sent.lock().unwrap().clone();
    assert_eq!(sent.len(), 2);
    let first = sent[0].1.split("#t=").nth(1).unwrap().to_owned();
    let latest = sent[1].1.split("#t=").nth(1).unwrap().to_owned();
    assert!(
        sent[1]
            .1
            .starts_with(&format!("{ORIGIN}/pt-BR/reset-password#t="))
    );

    let confirm = ConfirmPasswordReset::new(w.deps.clone());
    let pw = SecretString::new("another long passphrase");
    assert!(
        matches!(
            confirm.handle(&first, &pw, &meta()).await,
            Err(IdentityError::ResetTokenInvalid)
        ),
        "superseded"
    );
    confirm.handle(&latest, &pw, &meta()).await.unwrap();
    assert!(ctx(&w, &cookie).await.is_err(), "all sessions ended");
    assert!(
        matches!(
            confirm.handle(&latest, &pw, &meta()).await,
            Err(IdentityError::ResetTokenInvalid)
        ),
        "single use"
    );
    w.clock.advance(Duration::minutes(1));
    request.handle("ana@example.com", &meta()).await.unwrap();
    let t = w.mailer.sent.lock().unwrap()[2]
        .1
        .split("#t=")
        .nth(1)
        .unwrap()
        .to_owned();
    w.clock.advance(Duration::minutes(30));
    assert!(
        matches!(
            confirm.handle(&t, &pw, &meta()).await,
            Err(IdentityError::ResetTokenInvalid)
        ),
        "expired"
    );
}

#[tokio::test]
async fn admin_operations_check_admin_before_step_up_and_audit_the_target() {
    let w = World::new();
    let (admin_user, admin_cookie, _) = signed_in(&w, "admin@example.com", true).await;
    let (target, target_cookie, tctx) = signed_in(&w, "ana@example.com", false).await;
    enable_totp(&w, &tctx).await;
    w.clock.advance(Duration::minutes(20));
    let plain = ctx(&w, &target_cookie).await.unwrap();
    let reset = AdminResetTotp::new(w.deps.clone());
    assert!(
        matches!(
            reset.handle(&plain, admin_user.id, &meta()).await,
            Err(IdentityError::Forbidden)
        ),
        "no step-up prompt for non-admins"
    );
    let admin = ctx(&w, &admin_cookie).await.unwrap();
    assert!(matches!(
        reset.handle(&admin, UserId::new_v7(), &meta()).await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        reset.handle(&admin, target.id, &meta()).await,
        Err(IdentityError::StepUpRequired)
    ));
    StepUp::new(w.deps.clone())
        .handle(
            &admin,
            StepUpCommand {
                password: SecretString::new(PASSWORD),
                code: None,
                meta: meta(),
            },
        )
        .await
        .unwrap();
    let admin = ctx(&w, &admin_cookie).await.unwrap();
    reset.handle(&admin, target.id, &meta()).await.unwrap();
    assert!(ctx(&w, &target_cookie).await.is_err());
    let last = w.store.committed().pop().unwrap();
    assert!(
        last.audit
            .iter()
            .any(|a| a.trail == crate::changes::Trail::User(target.id))
    );
    assert!(
        last.audit
            .iter()
            .any(|a| matches!(a.trail, crate::changes::Trail::Admin(_)))
    );

    let link = AdminCreateResetLink::new(w.deps.clone())
        .handle(&admin, target.id, &meta())
        .await
        .unwrap();
    assert!(link.reset_url.expose().contains("/pt-BR/reset-password#t="));
    assert!(
        !debug_of_everything(&w).contains(link.reset_url.expose().split("#t=").nth(1).unwrap())
    );
    assert!(matches!(
        ListUsers::new(w.deps.clone()).handle(&plain).await,
        Err(IdentityError::Forbidden)
    ));
    assert_eq!(
        ListUsers::new(w.deps.clone())
            .handle(&admin)
            .await
            .unwrap()
            .len(),
        2
    );
}

// ---------- sources ----------

#[tokio::test]
async fn sources_lifecycle_and_one_time_url() {
    let w = World::new();
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateSource::new(w.deps.clone())
        .handle(&c, "Main", SourceType::TradingView, &meta())
        .await
        .unwrap();
    let url = created.webhook_url.expose().to_owned();
    let prefix = format!("{ORIGIN}/ingress/v1/tradingview/awk_");
    assert!(url.starts_with(&prefix));
    assert_eq!(url.len(), prefix.len() + 12 + 1 + 43);
    assert!(created.source.is_automated);
    let secret = &url[url.len() - 43..];
    assert!(
        created.source.masked_url.ends_with('…') && !created.source.masked_url.contains(secret)
    );
    assert!(
        !debug_of_everything(&w).contains(secret),
        "secret never reaches the store or audit"
    );

    assert!(matches!(
        CreateSource::new(w.deps.clone())
            .handle(&c, "MAIN", SourceType::TradingView, &meta())
            .await,
        Err(IdentityError::SourceNameTaken)
    ));
    let id = created.source.source.id;
    assert_eq!(
        *w.activated_keys.0.lock().unwrap(),
        vec![created.source.source.key_id.as_str().to_owned()],
        "fresh key leaves the ingress negative cache"
    );
    let rotated = RotateSource::new(w.deps.clone())
        .handle(&c, id, &meta())
        .await
        .unwrap();
    assert_ne!(rotated.webhook_url.expose(), url);
    assert_ne!(rotated.source.source.key_id, created.source.source.key_id);
    assert!(rotated.source.source.rotated_at.is_some());
    assert_eq!(
        w.activated_keys
            .0
            .lock()
            .unwrap()
            .last()
            .map(String::as_str),
        Some(rotated.source.source.key_id.as_str())
    );
    assert!(matches!(
        CreateSource::new(w.deps.clone())
            .handle(&c, "MAIN", SourceType::TradingView, &meta())
            .await,
        Err(IdentityError::SourceNameTaken)
    ));
    assert_eq!(
        w.activated_keys.0.lock().unwrap().len(),
        2,
        "no notification without a commit"
    );

    RevokeSource::new(w.deps.clone())
        .handle(&c, id, &meta())
        .await
        .unwrap();
    RevokeSource::new(w.deps.clone())
        .handle(&c, id, &meta())
        .await
        .unwrap();
    assert!(matches!(
        RenameSource::new(w.deps.clone())
            .handle(&c, id, "X", &meta())
            .await,
        Err(IdentityError::SourceRevoked)
    ));
    assert!(matches!(
        RotateSource::new(w.deps.clone())
            .handle(&c, id, &meta())
            .await,
        Err(IdentityError::SourceRevoked)
    ));
    assert!(
        ListSources::new(w.deps.clone())
            .handle(&c, false)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        ListSources::new(w.deps.clone())
            .handle(&c, true)
            .await
            .unwrap()
            .len(),
        1
    );
    // The revoked name is free again among active sources.
    CreateSource::new(w.deps.clone())
        .handle(&c, "Main", SourceType::TradingView, &meta())
        .await
        .unwrap();
}

#[tokio::test]
async fn sources_of_another_user_are_not_found() {
    let w = World::new();
    let (_, _, a) = signed_in(&w, "a@example.com", false).await;
    let (_, _, b) = signed_in(&w, "b@example.com", false).await;
    let id = CreateSource::new(w.deps.clone())
        .handle(&a, "Main", SourceType::TradingView, &meta())
        .await
        .unwrap()
        .source
        .source
        .id;
    assert!(matches!(
        RenameSource::new(w.deps.clone())
            .handle(&b, id, "Mine", &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        RotateSource::new(w.deps.clone())
            .handle(&b, id, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        RevokeSource::new(w.deps.clone())
            .handle(&b, id, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        RenameSource::new(w.deps.clone())
            .handle(&b, SourceId::new_v7(), "x", &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(
        ListSources::new(w.deps.clone())
            .handle(&b, true)
            .await
            .unwrap()
            .is_empty()
    );
    // Same name for another user is fine.
    CreateSource::new(w.deps.clone())
        .handle(&b, "Main", SourceType::TradingView, &meta())
        .await
        .unwrap();
}

// ---------- connections and pairing ----------

#[tokio::test]
async fn pairing_flow_single_use_repairing_and_unpair() {
    let w = World::new();
    let (user, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateConnection::new(w.deps.clone())
        .handle(&c, "Desk A", &meta())
        .await
        .unwrap();
    assert_eq!(created.connection.status, ConnectionStatus::AwaitingPairing);
    let code = created.pairing.pairing_code.expose().to_owned();
    assert_eq!(code.len(), 9);
    assert!(
        !debug_of_everything(&w).contains(&code.replace('-', "")),
        "code stored only as HMAC"
    );

    let pair = PairAdapter::new(w.deps.clone());
    let first = pair.handle(pair_cmd(&code.to_lowercase())).await.unwrap();
    assert_eq!(first.connection_id, created.connection.connection.id);
    let token = first.device_token.expose().to_owned();
    assert!(token.starts_with("awd_") && token.len() == 4 + 12 + 1 + 43);
    assert!(!debug_of_everything(&w).contains(&token[17..]));
    assert!(
        matches!(
            pair.handle(pair_cmd(&code)).await,
            Err(IdentityError::PairingInvalidCode)
        ),
        "single use"
    );
    assert_eq!(*w.pairing_throttle.failures.lock().unwrap(), 1);

    let id = first.connection_id;
    let listed = ListConnections::new(w.deps.clone())
        .handle(&c, false)
        .await
        .unwrap();
    assert_eq!(listed[0].status, ConnectionStatus::Disconnected);
    RecordConnectionLiveState::new(w.deps.clone())
        .handle(user.id, id, LiveState::Connected, w.clock.now())
        .await
        .unwrap();

    // Re-pairing: old instance keeps working until the new code is used.
    let again = NewPairingCode::new(w.deps.clone())
        .handle(&c, id, &meta())
        .await
        .unwrap();
    let listed = ListConnections::new(w.deps.clone())
        .handle(&c, false)
        .await
        .unwrap();
    assert!(listed[0].repairing_pending);
    assert_eq!(listed[0].status, ConnectionStatus::Connected);
    pair.handle(pair_cmd(again.pairing_code.expose()))
        .await
        .unwrap();
    let st = w.store.state.lock().unwrap().devices.clone();
    assert_eq!(
        st.iter().filter(|d| !d.revoked).count(),
        1,
        "previous device token revoked"
    );

    // Unpair with an open position needs the acknowledgement.
    w.presence.0.lock().unwrap().insert(
        id,
        ConnectionAccounts {
            visible_accounts: vec![],
            open_position_accounts: vec!["Sim101".into()],
        },
    );
    let unpair = UnpairConnection::new(w.deps.clone());
    assert!(matches!(
        unpair.handle(&c, id, false, &meta()).await,
        Err(IdentityError::OpenPositionsNotAcknowledged)
    ));
    unpair.handle(&c, id, true, &meta()).await.unwrap();
    unpair.handle(&c, id, true, &meta()).await.unwrap();
    assert!(
        w.store
            .state
            .lock()
            .unwrap()
            .devices
            .iter()
            .all(|d| d.revoked)
    );
    assert!(matches!(
        NewPairingCode::new(w.deps.clone())
            .handle(&c, id, &meta())
            .await,
        Err(IdentityError::ConnectionUnpaired)
    ));
    assert!(matches!(
        RenameConnection::new(w.deps.clone())
            .handle(&c, id, "B", &meta())
            .await,
        Err(IdentityError::ConnectionUnpaired)
    ));
    // The unpaired name is free again.
    CreateConnection::new(w.deps.clone())
        .handle(&c, "desk a", &meta())
        .await
        .unwrap();
}

#[tokio::test]
async fn superseded_and_expired_codes_are_one_invalid_code() {
    let w = World::new();
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateConnection::new(w.deps.clone())
        .handle(&c, "Desk A", &meta())
        .await
        .unwrap();
    let id = created.connection.connection.id;
    let newer = NewPairingCode::new(w.deps.clone())
        .handle(&c, id, &meta())
        .await
        .unwrap();
    let superseded = w.store.committed().pop().unwrap().events;
    assert!(
        superseded
            .iter()
            .any(|e| matches!(e, IdentityEvent::PairingCodeSuperseded { .. }))
    );
    let pair = PairAdapter::new(w.deps.clone());
    assert!(matches!(
        pair.handle(pair_cmd(created.pairing.pairing_code.expose()))
            .await,
        Err(IdentityError::PairingInvalidCode)
    ));
    w.clock.advance(Duration::minutes(10));
    assert!(matches!(
        pair.handle(pair_cmd(newer.pairing_code.expose())).await,
        Err(IdentityError::PairingInvalidCode)
    ));
    assert!(matches!(
        pair.handle(pair_cmd("not a code")).await,
        Err(IdentityError::PairingInvalidCode)
    ));
    assert_eq!(*w.pairing_throttle.failures.lock().unwrap(), 3);
}

#[tokio::test]
async fn pairing_throttle_and_version_gate() {
    let w = World::new();
    let pair = PairAdapter::new(w.deps.clone());
    let mut cmd = pair_cmd("K7Q2-0111");
    cmd.adapter = adapter("0.0.1");
    assert!(matches!(
        pair.handle(cmd).await,
        Err(IdentityError::AdapterVersionUnsupported)
    ));
    *w.pairing_throttle.decision.lock().unwrap() = Some(PairingDecision::RateLimited {
        retry_after: StdDuration::from_secs(9),
    });
    assert!(matches!(
        pair.handle(pair_cmd("K7Q2-0111")).await,
        Err(IdentityError::PairingRateLimited { .. })
    ));
    assert_eq!(*w.pairing_throttle.failures.lock().unwrap(), 0);
}

#[tokio::test]
async fn connections_of_another_user_are_not_found() {
    let w = World::new();
    let (_, _, a) = signed_in(&w, "a@example.com", false).await;
    let (_, _, b) = signed_in(&w, "b@example.com", false).await;
    let id = CreateConnection::new(w.deps.clone())
        .handle(&a, "Desk", &meta())
        .await
        .unwrap()
        .connection
        .connection
        .id;
    assert!(matches!(
        RenameConnection::new(w.deps.clone())
            .handle(&b, id, "x", &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        NewPairingCode::new(w.deps.clone())
            .handle(&b, id, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        UnpairConnection::new(w.deps.clone())
            .handle(&b, id, true, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        UnpairConnection::new(w.deps.clone())
            .handle(&b, ConnectionId::new_v7(), true, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(
        ListConnections::new(w.deps.clone())
            .handle(&b, true)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn startup_reset_disconnects_live_connections_with_audit_per_user() {
    let w = World::new();
    let (user, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateConnection::new(w.deps.clone())
        .handle(&c, "Desk", &meta())
        .await
        .unwrap();
    PairAdapter::new(w.deps.clone())
        .handle(pair_cmd(created.pairing.pairing_code.expose()))
        .await
        .unwrap();
    let id = created.connection.connection.id;
    RecordConnectionLiveState::new(w.deps.clone())
        .handle(user.id, id, LiveState::Connected, w.clock.now())
        .await
        .unwrap();
    let n = DisconnectLiveConnectionsAtStartup::new(w.deps.clone())
        .handle()
        .await
        .unwrap();
    assert_eq!(n, 1);
    let last = w.store.committed().pop().unwrap();
    assert!(
        last.writes
            .iter()
            .any(|x| matches!(x, Write::DisconnectLiveConnection { .. }))
    );
    assert_eq!(last.audit.len(), 1);
    assert_eq!(
        DisconnectLiveConnectionsAtStartup::new(w.deps.clone())
            .handle()
            .await
            .unwrap(),
        0,
        "idempotent"
    );
}

#[tokio::test]
async fn profile_update_validates_time_zone() {
    let w = World::new();
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let update = UpdateMe::new(w.deps.clone());
    assert!(matches!(
        update.handle(&c, None, Some("Nowhere/City")).await,
        Err(IdentityError::InvalidTimeZone)
    ));
    let me = update
        .handle(&c, Some(Locale::En), Some("Europe/Lisbon"))
        .await
        .unwrap();
    assert_eq!(
        (me.locale, me.time_zone.as_str()),
        (Locale::En, "Europe/Lisbon")
    );
    let sec = GetMySecurity::new(w.deps.clone()).handle(&c).await.unwrap();
    assert_eq!(
        (sec.session.idle_timeout_s, sec.session.absolute_timeout_s),
        (43_200, 86_400)
    );
}
