//! Use-case behaviour through fakes: authorization, isolation (X-2), step-up, throttling
//! seams, one-time secrets and atomic effects. The passwordless flows are in `tests_auth.rs`.

use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration as StdDuration;

use chrono::Duration;
use identity_enterprise::channel::Channel;
use identity_enterprise::connection::{AdapterBuild, AdapterInfo, ConnectionStatus, LiveState};
use identity_enterprise::device::KnownDevice;
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::{ConnectionId, DeviceId, SessionId, SourceId};
use identity_enterprise::otp::StepUpAction;
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::session::{ClientInfo, EndReason, NewSession, Session};
use identity_enterprise::source::SourceType;
use identity_enterprise::tokens::TokenHash;
use identity_enterprise::user::{User, UserStatus};
use shared_kernel::{Clock, UserId};
use uuid::Uuid;

use crate::auth::{Activity, AuthContext, AuthenticateSession};
use crate::changes::{ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::RequestMeta;
use crate::error::IdentityError;
use crate::ports::ConnectionAccounts;
use crate::testing::{ORIGIN, World};
use crate::throttle::PairingDecision;
use crate::usecases::admin::ListUsers;
use crate::usecases::connections::*;
use crate::usecases::invitations::*;
use crate::usecases::me::*;
use crate::usecases::pairing::{PairAdapter, PairAdapterCommand};
use crate::usecases::signin::*;
use crate::usecases::sources::*;
use crate::usecases::step_up::{RequestStepUp, RequestStepUpCommand, StepUpProof};

pub(crate) fn meta() -> RequestMeta {
    RequestMeta {
        ip: IpAddr::V4(Ipv4Addr::new(203, 0, 113, 7)),
        user_agent: Some(
            "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 Chrome/131.0 Safari/537.36".into(),
        ),
    }
}

pub(crate) fn add_user(w: &World, email: &str, admin: bool) -> User {
    let email = EmailAddress::parse(email).unwrap();
    let user = User {
        id: UserId::new_v7(),
        email,
        email_verified_at: w.clock.now(),
        whatsapp: None,
        locale: Locale::PtBr,
        time_zone: TimeZoneName::parse("America/Sao_Paulo").unwrap(),
        is_platform_admin: admin,
        status: UserStatus::Active,
        created_at: w.clock.now(),
        channel_changed_at: None,
    };
    w.store.insert_user(user.clone());
    user
}

/// Starts a session directly on a device first seen `device_age` ago.
pub(crate) async fn login_on(w: &World, email: &str, device_age: Duration) -> String {
    let user = w.store.user_by_email(email).expect("test user exists");
    let now = w.clock.now();
    let device = KnownDevice::new(
        DeviceId::new_v7(),
        user.id,
        TokenHash::from_bytes([0xAB; 32]),
        meta().user_agent.as_deref(),
        Some(meta().ip),
        now - device_age,
    );
    let token = w.deps.issue_token();
    let session = Session::start(
        NewSession {
            id: SessionId::new_v7(),
            user_id: user.id,
            device_id: device.id,
            token_hash: token.hash,
            persistent: false,
            login_channel: Channel::Email,
            client: ClientInfo {
                ip: Some(meta().ip),
                user_agent: meta().user_agent,
            },
        },
        now,
    );
    w.deps
        .store
        .commit(
            ChangeSet::new(user.id)
                .write(Write::InsertDevice(device))
                .write(Write::InsertSession(session)),
        )
        .await
        .unwrap();
    token.value().expose().to_owned()
}

/// A session on a device known for 30 days.
pub(crate) async fn login(w: &World, email: &str) -> String {
    login_on(w, email, Duration::days(30)).await
}

pub(crate) async fn ctx(w: &World, cookie: &str) -> Result<AuthContext, IdentityError> {
    AuthenticateSession::new(w.deps.clone())
        .handle(Some(cookie), Activity::UserInitiated)
        .await
}

pub(crate) async fn signed_in(w: &World, email: &str, admin: bool) -> (User, String, AuthContext) {
    let user = add_user(w, email, admin);
    let cookie = login(w, email).await;
    let c = ctx(w, &cookie).await.unwrap();
    (user, cookie, c)
}

/// Requests a step-up code by e-mail and returns the proof the action needs.
pub(crate) async fn proof(
    w: &World,
    c: &AuthContext,
    action: StepUpAction,
    target: Option<Uuid>,
) -> StepUpProof {
    let issued = RequestStepUp::new(w.deps.clone())
        .handle(
            c,
            RequestStepUpCommand {
                action,
                target_id: target,
                channel: Channel::Email,
                meta: meta(),
            },
        )
        .await
        .unwrap();
    w.clock.advance(Duration::seconds(61));
    StepUpProof {
        challenge_id: issued.challenge_id.expose().to_owned(),
        code: w.delivery.last_code_to(c.user.email.normalized()).unwrap(),
    }
}

pub(crate) fn adapter(version: &str) -> AdapterInfo {
    AdapterInfo {
        adapter_type: "ninjatrader8".into(),
        version: version.into(),
        build: AdapterBuild::Release,
    }
}

pub(crate) fn pair_cmd(code: &str) -> PairAdapterCommand {
    PairAdapterCommand {
        pairing_code: code.into(),
        instance_id: Uuid::now_v7(),
        instance_name: "Desk PC".into(),
        adapter: adapter("1.0.0"),
        ip: meta().ip,
    }
}

pub(crate) fn debug_of_everything(w: &World) -> String {
    format!("{:?}", w.store.committed())
}

// ---------- sign-in ----------

#[tokio::test]
async fn session_expires_when_idle_and_ignores_background_activity() {
    let w = World::new();
    let (_, cookie, _) = signed_in(&w, "Ana@Example.com", false).await;
    w.clock.advance(Duration::hours(11));
    AuthenticateSession::new(w.deps.clone())
        .handle(Some(&cookie), Activity::Background)
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
            auth.handle(cookie, Activity::UserInitiated).await,
            Err(IdentityError::Unauthenticated)
        ));
    }
}

#[tokio::test]
async fn list_users_is_admin_only() {
    let w = World::new();
    let (_, _, admin) = signed_in(&w, "admin@example.com", true).await;
    let (_, _, plain) = signed_in(&w, "ana@example.com", false).await;
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

// ---------- invitations ----------

#[tokio::test]
async fn invitation_flow_admin_only_one_time_and_reinvite_after_expiry() {
    let w = World::with_settings(false);
    let (_, _, admin) = signed_in(&w, "admin@example.com", true).await;
    let (_, _, plain) = signed_in(&w, "plain@example.com", false).await;
    let create = CreateInvitation::new(w.deps.clone());
    let cmd = |email: &str| CreateInvitationCommand {
        email: email.into(),
        locale: Locale::En,
        send_email: true,
        step_up: None,
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
    let w = World::with_settings(false);
    let (_, _, admin) = signed_in(&w, "admin@example.com", true).await;
    *w.mailer.fail.lock().unwrap() = true;
    let created = CreateInvitation::new(w.deps.clone())
        .handle(
            &admin,
            CreateInvitationCommand {
                email: "x@example.com".into(),
                locale: Locale::PtBr,
                send_email: true,
                step_up: None,
                meta: meta(),
            },
        )
        .await
        .unwrap();
    assert!(!created.email_sent);
    assert!(created.invite_url.expose().contains("/pt-BR/invite#t="));
}

// ---------- sources ----------

#[tokio::test]
async fn sources_lifecycle_and_one_time_url() {
    let w = World::with_settings(false);
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateSource::new(w.deps.clone())
        .handle(&c, "Main", SourceType::TradingView, None, &meta())
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
            .handle(&c, "MAIN", SourceType::TradingView, None, &meta())
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
        .handle(&c, id, None, &meta())
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
            .handle(&c, "MAIN", SourceType::TradingView, None, &meta())
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
            .handle(&c, id, None, &meta())
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
        .handle(&c, "Main", SourceType::TradingView, None, &meta())
        .await
        .unwrap();
}

#[tokio::test]
async fn sources_of_another_user_are_not_found() {
    let w = World::with_settings(false);
    let (_, _, a) = signed_in(&w, "a@example.com", false).await;
    let (_, _, b) = signed_in(&w, "b@example.com", false).await;
    let id = CreateSource::new(w.deps.clone())
        .handle(&a, "Main", SourceType::TradingView, None, &meta())
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
            .handle(&b, id, None, &meta())
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
        .handle(&b, "Main", SourceType::TradingView, None, &meta())
        .await
        .unwrap();
}

// ---------- connections and pairing ----------

#[tokio::test]
async fn pairing_flow_single_use_repairing_and_unpair() {
    let w = World::with_settings(false);
    let (user, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateConnection::new(w.deps.clone())
        .handle(&c, "Desk A", None, &meta())
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
        .handle(&c, id, None, &meta())
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
    let st = w.store.state.lock().unwrap().device_tokens.clone();
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
        unpair.handle(&c, id, true, None, &meta()).await,
        Err(IdentityError::StepUpRequired)
    ));
    let p = proof(&w, &c, StepUpAction::ConnectionUnpair, Some(id.as_uuid())).await;
    assert!(matches!(
        unpair.handle(&c, id, false, Some(&p), &meta()).await,
        Err(IdentityError::OpenPositionsNotAcknowledged)
    ));
    unpair
        .handle(&c, id, true, Some(&p), &meta())
        .await
        .unwrap();
    unpair.handle(&c, id, true, None, &meta()).await.unwrap();
    assert!(
        w.store
            .state
            .lock()
            .unwrap()
            .device_tokens
            .iter()
            .all(|d| d.revoked)
    );
    assert!(matches!(
        NewPairingCode::new(w.deps.clone())
            .handle(&c, id, None, &meta())
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
        .handle(&c, "desk a", None, &meta())
        .await
        .unwrap();
}

#[tokio::test]
async fn superseded_and_expired_codes_are_one_invalid_code() {
    let w = World::with_settings(false);
    let (_, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateConnection::new(w.deps.clone())
        .handle(&c, "Desk A", None, &meta())
        .await
        .unwrap();
    let id = created.connection.connection.id;
    let newer = NewPairingCode::new(w.deps.clone())
        .handle(&c, id, None, &meta())
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
    let w = World::with_settings(false);
    let (_, _, a) = signed_in(&w, "a@example.com", false).await;
    let (_, _, b) = signed_in(&w, "b@example.com", false).await;
    let id = CreateConnection::new(w.deps.clone())
        .handle(&a, "Desk", None, &meta())
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
            .handle(&b, id, None, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        UnpairConnection::new(w.deps.clone())
            .handle(&b, id, true, None, &meta())
            .await,
        Err(IdentityError::NotFound)
    ));
    assert!(matches!(
        UnpairConnection::new(w.deps.clone())
            .handle(&b, ConnectionId::new_v7(), true, None, &meta())
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
    let w = World::with_settings(false);
    let (user, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateConnection::new(w.deps.clone())
        .handle(&c, "Desk", None, &meta())
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

/// S2 adapter authentication: the device token authenticates (with the user's locale) until a
/// re-pair or an unpair revokes it, and each revocation emits `identity.device_token_revoked`.
#[tokio::test]
async fn device_tokens_authenticate_until_revoked() {
    use crate::changes::TokenRevokedReason;
    use crate::usecases::adapter_auth::{AuthenticateDeviceToken, DeviceTokenStatus};
    use identity_enterprise::secret::SecretString;

    let w = World::with_settings(false);
    let (user, _, c) = signed_in(&w, "ana@example.com", false).await;
    let created = CreateConnection::new(w.deps.clone())
        .handle(&c, "Desk A", None, &meta())
        .await
        .unwrap();
    let pair = PairAdapter::new(w.deps.clone());
    let first = pair
        .handle(pair_cmd(created.pairing.pairing_code.expose()))
        .await
        .unwrap();
    let id = first.connection_id;
    let auth = AuthenticateDeviceToken::new(w.deps.clone());
    let status = DeviceTokenStatus::new(w.deps.clone());
    let p = auth.handle(&first.device_token).await.unwrap().unwrap();
    assert_eq!(
        (p.user_id, p.connection_id, p.locale),
        (user.id, id, user.locale)
    );
    assert!(status.handle(&p.key_id).await.unwrap());
    // A wrong secret or a malformed token: nothing, alike.
    let mut wrong = first.device_token.expose().to_owned();
    let last = if wrong.ends_with('A') { 'B' } else { 'A' };
    wrong.pop();
    wrong.push(last);
    for t in [wrong.as_str(), "awd_nope", ""] {
        assert!(
            auth.handle(&SecretString::new(t)).await.unwrap().is_none(),
            "{t}"
        );
    }
    let revoked = |reason: TokenRevokedReason| {
        w.store.committed().iter().any(|c| {
            c.events.iter().any(|e| {
                matches!(e, IdentityEvent::DeviceTokenRevoked { connection_id, reason: r, .. }
                    if *connection_id == id && *r == reason)
            })
        })
    };
    assert!(!revoked(TokenRevokedReason::Repaired));
    // Re-pair: the first token stops working.
    let again = NewPairingCode::new(w.deps.clone())
        .handle(&c, id, None, &meta())
        .await
        .unwrap();
    let second = pair
        .handle(pair_cmd(again.pairing_code.expose()))
        .await
        .unwrap();
    assert!(auth.handle(&first.device_token).await.unwrap().is_none());
    assert!(!status.handle(&p.key_id).await.unwrap());
    assert!(revoked(TokenRevokedReason::Repaired));
    let p2 = auth.handle(&second.device_token).await.unwrap().unwrap();
    // Unpair: the second token stops working too.
    let proof = proof(&w, &c, StepUpAction::ConnectionUnpair, Some(id.as_uuid())).await;
    UnpairConnection::new(w.deps.clone())
        .handle(&c, id, true, Some(&proof), &meta())
        .await
        .unwrap();
    assert!(auth.handle(&second.device_token).await.unwrap().is_none());
    assert!(!status.handle(&p2.key_id).await.unwrap());
    assert!(revoked(TokenRevokedReason::Unpaired));
}
