//! Handlers of the identity HTTP adapter, one per contract `operationId`.
//! Extractor order implements the check precedence (401 → 403 origin → 403 admin → 400).

#![allow(clippy::too_many_lines)]

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::rejection::RawPathParamsRejection;
use axum::extract::{RawPathParams, RawQuery, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use contracts::http::{Error, RetryAfterSeconds};
use identity_application::error::IdentityError;
use identity_application::ports::UserSummary;
use identity_application::usecases::admin::ListUsers;
use identity_application::usecases::channel_change::*;
use identity_application::usecases::connections::*;
use identity_application::usecases::devices::*;
use identity_application::usecases::invitations::*;
use identity_application::usecases::me::*;
use identity_application::usecases::pairing::{PairAdapter, PairAdapterCommand};
use identity_application::usecases::signin::*;
use identity_application::usecases::signup::*;
use identity_application::usecases::sources::*;
use identity_application::usecases::step_up::{RequestStepUp, RequestStepUpCommand};
use identity_application::views::{
    ChannelChangeView, ConnectionView, DeliveryView, InvitationView, SignedIn, SourceView,
};
use identity_enterprise::channel_change::ChangeStatus;
use identity_enterprise::connection::{AdapterBuild, AdapterInfo, ConnectionStatus};
use identity_enterprise::ids::{ChannelChangeId, DeviceId};
use identity_enterprise::ids::{ConnectionId, InvitationId, SourceId};
use identity_enterprise::invitation::InvitationStatus;
use identity_enterprise::otp::CodeRole;
use identity_enterprise::source::{SourceStatus, SourceType};
use identity_enterprise::user::UserStatus;
use serde::Serialize;

use super::IdentityHttpState;
use super::error::{ApiError, respond};
use super::extract::{
    AdminSession, Client, DeviceCookieValue, OptionalSession, PublicOrigin, Session, StreamSession,
};
use super::guard::{Rejection, ValidJson, path_uuid, query_bool};
use super::schemas::*;

type S = State<Arc<IdentityHttpState>>;
type PathParams = Result<RawPathParams, RawPathParamsRejection>;

fn json<T: Serialize>(status: StatusCode, body: &T) -> Response {
    (status, axum::Json(body)).into_response()
}

fn no_store(mut r: Response) -> Response {
    r.headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    r
}

fn err(e: IdentityError) -> Response {
    ApiError(e).into_response()
}

// axum rejections are responses; boxing them would only move the allocation.
#[allow(clippy::result_large_err)]
fn path_id(params: PathParams, name: &str) -> Result<uuid::Uuid, Response> {
    let params =
        params.map_err(|_| Rejection::invalid(&format!("/path/{name}")).into_response())?;
    let map: HashMap<String, String> = params
        .iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();
    path_uuid(&map, name).map_err(IntoResponse::into_response)
}

fn invitation_dto(v: &InvitationView) -> Invitation {
    Invitation {
        id: v.invitation.id.as_uuid().to_string(),
        email: v.invitation.email.clone(),
        status: match v.status {
            InvitationStatus::Pending => InvitationStatusDto::Pending,
            InvitationStatus::Accepted => InvitationStatusDto::Accepted,
            InvitationStatus::Revoked => InvitationStatusDto::Revoked,
            InvitationStatus::Expired => InvitationStatusDto::Expired,
        },
        created_at: ts(v.invitation.created_at),
        expires_at: ts(v.invitation.expires_at),
        accepted_at: v.invitation.accepted_at.map(ts),
        revoked_at: v.invitation.revoked_at.map(ts),
    }
}

fn admin_user(u: &UserSummary) -> AdminUser {
    AdminUser {
        id: u.id.to_string(),
        email: u.email.clone(),
        status: match u.status {
            UserStatus::Active => UserStatusDto::Active,
            UserStatus::Closed => UserStatusDto::Closed,
        },
        whatsapp_verified: u.whatsapp_verified,
        is_platform_admin: Some(u.is_platform_admin),
        created_at: ts(u.created_at),
        last_sign_in_at: u.last_sign_in_at.map(ts),
    }
}

fn change_dto(v: &ChannelChangeView) -> ChannelChange {
    ChannelChange {
        id: v.id.as_uuid().to_string(),
        channel: CodeChannel::of(v.channel),
        status: match v.status {
            ChangeStatus::AwaitingCodes => ChangeStatusDto::AwaitingCodes,
            ChangeStatus::OnHold => ChangeStatusDto::OnHold,
            ChangeStatus::Completed => ChangeStatusDto::Completed,
            ChangeStatus::Cancelled => ChangeStatusDto::Cancelled,
            ChangeStatus::Expired => ChangeStatusDto::Expired,
        },
        masked_new_value: v.masked_new_value.clone(),
        codes: v
            .codes
            .iter()
            .map(|c| ChangeCode {
                role: match c.role {
                    CodeRole::CurrentValue => CodeRoleDto::CurrentValue,
                    CodeRole::NewValue => CodeRoleDto::NewValue,
                    CodeRole::OtherChannel => CodeRoleDto::OtherChannel,
                },
                challenge_id: c.challenge_id.expose().to_owned(),
                confirmed: c.confirmed,
            })
            .collect(),
        expires_at: ts(v.expires_at),
        hold_until: v.hold_until.map(ts),
    }
}

fn issued_dto(v: &identity_application::views::ChallengeIssued) -> ChallengeIssued {
    ChallengeIssued {
        challenge_id: v.challenge_id.expose().to_owned(),
        expires_at: ts(v.expires_at),
        resend_after_s: v.resend_after_s,
    }
}

/// Sign-in or sign-up result: body, session cookie and (renewed) device cookie.
fn signed_in_response(
    status: StatusCode,
    v: &SignedIn,
    now: chrono::DateTime<chrono::Utc>,
) -> Response {
    let mut r = json(
        status,
        &LoginResult {
            user: Me::from(&v.user),
            new_device: v.new_device,
        },
    );
    if let Some(c) = super::cookie::set(&v.session, now) {
        r.headers_mut().append(header::SET_COOKIE, c);
    }
    if let Some(c) = super::cookie::set_device(&v.device) {
        r.headers_mut().append(header::SET_COOKIE, c);
    }
    r
}

fn proof(p: Option<StepUpProof>) -> Option<identity_application::usecases::step_up::StepUpProof> {
    p.map(StepUpProof::domain)
}

fn source_dto(v: &SourceView) -> Source {
    let s = &v.source;
    Source {
        id: s.id.as_uuid().to_string(),
        name: s.name.as_str().to_owned(),
        source_type: match s.source_type {
            SourceType::TradingView => SourceTypeDto::Tradingview,
        },
        is_automated: v.is_automated,
        status: match s.status() {
            SourceStatus::Active => SourceStatusDto::Active,
            SourceStatus::Revoked => SourceStatusDto::Revoked,
        },
        key_id: s.key_id.as_str().to_owned(),
        masked_url: v.masked_url.clone(),
        created_at: ts(s.created_at),
        rotated_at: s.rotated_at.map(ts),
        revoked_at: s.revoked_at.map(ts),
        last_received_at: v.last_received_at.map(ts),
    }
}

fn kind(raw: &str) -> VisibleAccountKind {
    match raw {
        "simulation" => VisibleAccountKind::Simulation,
        "playback" => VisibleAccountKind::Playback,
        "live" => VisibleAccountKind::Live,
        _ => VisibleAccountKind::Unknown,
    }
}

fn oco(raw: &str) -> OcoLocation {
    match raw {
        "local" => OcoLocation::Local,
        "server" => OcoLocation::Server,
        _ => OcoLocation::Unknown,
    }
}

fn connection_dto(v: &ConnectionView) -> Connection {
    let c = &v.connection;
    let instance = c.instance.as_ref();
    Connection {
        id: c.id.as_uuid().to_string(),
        name: c.name.as_str().to_owned(),
        status: match v.status {
            ConnectionStatus::AwaitingPairing => ConnectionStatusDto::AwaitingPairing,
            ConnectionStatus::Connected => ConnectionStatusDto::Connected,
            ConnectionStatus::Disconnected => ConnectionStatusDto::Disconnected,
            ConnectionStatus::Outdated => ConnectionStatusDto::Outdated,
            ConnectionStatus::Unpaired => ConnectionStatusDto::Unpaired,
        },
        status_changed_at: ts(v.status_changed_at),
        adapter_type: instance.map(|i| i.adapter.adapter_type.clone()),
        adapter_version: instance.map(|i| i.adapter.version.clone()),
        instance_id: instance.map(|i| i.instance_id.to_string()),
        instance_name: instance.map(|i| i.instance_name.clone()),
        paired_at: instance.map(|i| ts(i.paired_at)),
        last_seen_at: v.last_seen_at.map(ts),
        pairing_code_expires_at: v.pairing_code_expires_at.map(ts),
        repairing_pending: Some(v.repairing_pending),
        visible_accounts: Some(
            v.visible_accounts
                .iter()
                .map(|a| VisibleAccount {
                    platform_account: a.platform_account.clone(),
                    kind: kind(&a.kind),
                    oco_location: oco(&a.oco_location),
                    account_id: a.account_id.map(|id| id.to_string()),
                })
                .collect(),
        ),
        open_position_accounts: Some(v.open_position_accounts.clone()),
        created_at: ts(c.created_at),
        unpaired_at: c.unpaired_at.map(ts),
    }
}

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Path)]
#[allow(dead_code)]
struct InvitationIdParam {
    #[param(value_type = contracts::http::Uuid)]
    invitation_id: String,
}

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Path)]
#[allow(dead_code)]
struct UserIdParam {
    #[param(value_type = contracts::http::Uuid)]
    user_id: String,
}

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Path)]
#[allow(dead_code)]
struct SourceIdParam {
    #[param(value_type = contracts::http::Uuid)]
    source_id: String,
}

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Path)]
#[allow(dead_code)]
struct ConnectionIdParam {
    #[param(value_type = contracts::http::Uuid)]
    connection_id: String,
}

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
#[allow(dead_code)]
struct IncludeRevokedParam {
    #[param(default = false, nullable = false)]
    include_revoked: Option<bool>,
}

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
#[allow(dead_code)]
struct IncludeUnpairedParam {
    #[param(default = false, nullable = false)]
    include_unpaired: Option<bool>,
}

// ---------------- auth ----------------

/// End the current session (server-side). Idempotent.
#[utoipa::path(post, path = "/api/v1/auth/logout", operation_id = "logout", tag = "auth",
    security(("session" = []), ()),
    responses(
        (status = 204, description = "Session ended; session cookie cleared.", headers(("Set-Cookie" = String))),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn logout(
    State(s): S,
    _o: PublicOrigin,
    headers: axum::http::HeaderMap,
) -> Response {
    let cookie = super::cookie::read(&headers);
    match Logout::new(s.deps.clone()).handle(cookie.as_deref()).await {
        Ok(()) => {
            let mut r = StatusCode::NO_CONTENT.into_response();
            r.headers_mut()
                .insert(header::SET_COOKIE, super::cookie::clear());
            r
        }
        Err(e) => err(e),
    }
}

/// Read an invitation before accepting it (token in the body, never in a URL).
#[utoipa::path(post, path = "/api/v1/auth/invitations/lookup", operation_id = "lookupInvitation", tag = "auth", security(()),
    request_body = TokenBody,
    responses(
        (status = 200, description = "Valid invitation.", body = InvitationLookup),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "auth.invitation_invalid.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn lookup_invitation(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    ValidJson(r): ValidJson<TokenBody>,
) -> Response {
    match LookupInvitation::new(s.deps.clone())
        .handle(&r.token, &meta)
        .await
    {
        Ok(v) => json(
            StatusCode::OK,
            &InvitationLookup {
                email: v.email,
                locale: Some(locale_out(v.locale)),
                expires_at: ts(v.expires_at),
            },
        ),
        Err(e) => err(e),
    }
}

/// Request a sign-in code on a channel (always 202; auth policy §3).
#[utoipa::path(post, path = "/api/v1/auth/login/challenges", operation_id = "requestLoginCode", tag = "auth", security(()),
    request_body = RequestLoginCodeRequest,
    responses(
        (status = 202, description = "Challenge issued (a code may or may not have been sent).", body = ChallengeIssued),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn request_login_code(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    DeviceCookieValue(device): DeviceCookieValue,
    ValidJson(r): ValidJson<RequestLoginCodeRequest>,
) -> Response {
    let cmd = RequestLoginCodeCommand {
        email: r.email,
        channel: r.channel.domain(),
        device_cookie: device,
        meta,
    };
    match RequestLoginCode::new(s.deps.clone()).handle(cmd).await {
        Ok(v) => json(StatusCode::ACCEPTED, &issued_dto(&v)),
        Err(e) => err(e),
    }
}

/// Sign in with the code of a login challenge.
#[utoipa::path(post, path = "/api/v1/auth/login/verify", operation_id = "verifyLoginCode", tag = "auth", security(()),
    request_body = VerifyLoginCodeRequest,
    responses(
        (status = 200, description = "Authenticated.", body = LoginResult, headers(("Set-Cookie" = String))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.code_invalid.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn verify_login_code(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    DeviceCookieValue(device): DeviceCookieValue,
    ValidJson(r): ValidJson<VerifyLoginCodeRequest>,
) -> Response {
    let cmd = VerifyLoginCodeCommand {
        challenge_id: r.challenge_id,
        code: r.code,
        remember_me: r.remember_me,
        device_cookie: device,
        meta,
    };
    match VerifyLoginCode::new(s.deps.clone()).handle(cmd).await {
        Ok(v) => signed_in_response(StatusCode::OK, &v, s.deps.clock.now()),
        Err(e @ IdentityError::CodeInvalid) => respond(&e, StatusCode::UNAUTHORIZED),
        Err(e) => err(e),
    }
}

/// Send a new code for an open challenge (60-s cooldown; auth policy §2.2).
#[utoipa::path(post, path = "/api/v1/auth/challenges/resend", operation_id = "resendCode", tag = "auth",
    security(("session" = []), ()),
    request_body = ChallengeBody,
    responses(
        (status = 202, description = "Accepted (a code may or may not have been sent).", body = ChallengeIssued),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn resend_code(
    State(s): S,
    OptionalSession(ctx): OptionalSession,
    Client(meta): Client,
    ValidJson(r): ValidJson<ChallengeBody>,
) -> Response {
    let cmd = ResendCodeCommand {
        challenge_id: r.challenge_id,
        meta,
    };
    match ResendCode::new(s.deps.clone())
        .handle(ctx.as_ref(), cmd)
        .await
    {
        Ok(v) => json(StatusCode::ACCEPTED, &issued_dto(&v)),
        Err(e) => err(e),
    }
}

/// Delivery state of a sign-up, step-up or channel-change code (never for login).
#[utoipa::path(post, path = "/api/v1/auth/challenges/delivery", operation_id = "getChallengeDelivery", tag = "auth",
    security(("session" = []), ()),
    request_body = ChallengeBody,
    responses(
        (status = 200, description = "Delivery state.", body = ChallengeDelivery),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn get_challenge_delivery(
    State(s): S,
    OptionalSession(ctx): OptionalSession,
    ValidJson(r): ValidJson<ChallengeBody>,
) -> Response {
    match GetChallengeDelivery::new(s.deps.clone())
        .handle(ctx.as_ref(), &r.challenge_id)
        .await
    {
        Ok(v) => json(
            StatusCode::OK,
            &ChallengeDelivery {
                delivery: match v {
                    DeliveryView::Pending => DeliveryStateDto::Pending,
                    DeliveryView::Sent => DeliveryStateDto::Sent,
                    DeliveryView::Failed => DeliveryStateDto::Failed,
                    DeliveryView::NotReported => DeliveryStateDto::NotReported,
                },
            },
        ),
        Err(e) => err(e),
    }
}

/// Sair de todos os dispositivos — end every session and revoke every device (no step-up).
#[utoipa::path(post, path = "/api/v1/auth/logout-everywhere", operation_id = "signOutEverywhere", tag = "auth",
    responses(
        (status = 204, description = "Done; session and device cookies cleared.", headers(("Set-Cookie" = String))),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn sign_out_everywhere(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
) -> Response {
    match SignOutEverywhere::new(s.deps.clone())
        .handle(&ctx, &meta)
        .await
    {
        Ok(()) => {
            let mut r = StatusCode::NO_CONTENT.into_response();
            r.headers_mut()
                .append(header::SET_COOKIE, super::cookie::clear());
            r.headers_mut()
                .append(header::SET_COOKIE, super::cookie::clear_device());
            r
        }
        Err(e) => err(e),
    }
}

/// Request a step-up code for one sensitive action (auth policy §6).
#[utoipa::path(post, path = "/api/v1/auth/step-up", operation_id = "requestStepUp", tag = "auth",
    request_body = RequestStepUpRequest,
    responses(
        (status = 202, description = "Code sent.", body = ChallengeIssued),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.forbidden or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 422, description = "auth.channel_unavailable or auth.step_up_channel_not_allowed.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn request_step_up(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<RequestStepUpRequest>,
) -> Response {
    let target_id = match r
        .target_id
        .as_deref()
        .map(uuid::Uuid::parse_str)
        .transpose()
    {
        Ok(t) => t,
        Err(_) => return Rejection::invalid("/target_id").into_response(),
    };
    let cmd = RequestStepUpCommand {
        action: r.action.domain(),
        target_id,
        channel: r.channel.domain(),
        meta,
    };
    match RequestStepUp::new(s.deps.clone()).handle(&ctx, cmd).await {
        Ok(v) => json(StatusCode::ACCEPTED, &issued_dto(&v)),
        Err(e) => err(e),
    }
}

/// Send the sign-up code to the invited e-mail (S3 AC-1/AC-2).
#[utoipa::path(post, path = "/api/v1/auth/signup/challenges", operation_id = "startSignup", tag = "auth", security(()),
    request_body = TokenBody,
    responses(
        (status = 202, description = "Code sent to the invited e-mail.", body = ChallengeIssued),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "auth.invitation_invalid.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn start_signup(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    ValidJson(r): ValidJson<TokenBody>,
) -> Response {
    match StartSignup::new(s.deps.clone())
        .handle(&r.token, &meta)
        .await
    {
        Ok(v) => json(StatusCode::ACCEPTED, &issued_dto(&v)),
        Err(e) => err(e),
    }
}

/// Confirm the sign-up code; create the account and sign in.
#[utoipa::path(post, path = "/api/v1/auth/signup/verify", operation_id = "completeSignup", tag = "auth", security(()),
    request_body = CompleteSignupRequest,
    responses(
        (status = 201, description = "Account created; session started.", body = LoginResult, headers(("Set-Cookie" = String))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.code_invalid.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 422, description = "me.invalid_time_zone.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn complete_signup(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    DeviceCookieValue(device): DeviceCookieValue,
    ValidJson(r): ValidJson<CompleteSignupRequest>,
) -> Response {
    let cmd = CompleteSignupCommand {
        challenge_id: r.challenge_id,
        code: r.code,
        locale: locale_in(r.locale),
        time_zone: r.time_zone,
        remember_me: r.remember_me,
        device_cookie: device,
        meta,
    };
    match CompleteSignup::new(s.deps.clone()).handle(cmd).await {
        Ok(v) => signed_in_response(StatusCode::CREATED, &v, s.deps.clock.now()),
        Err(e @ IdentityError::CodeInvalid) => respond(&e, StatusCode::UNAUTHORIZED),
        Err(e) => err(e),
    }
}

// ---------------- me ----------------

#[utoipa::path(get, path = "/api/v1/me", operation_id = "getMe", tag = "me",
    responses(
        (status = 200, description = "Signed-in user.", body = Me),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn get_me(Session(ctx): Session) -> Response {
    json(StatusCode::OK, &Me::from(&GetMe::new().handle(&ctx)))
}

/// Update profile preferences (locale, time zone; Q23).
#[utoipa::path(patch, path = "/api/v1/me", operation_id = "updateMe", tag = "me",
    request_body = UpdateMeRequest,
    responses(
        (status = 200, description = "Updated profile.", body = Me),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 422, description = "me.invalid_time_zone.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn update_me(
    State(s): S,
    Session(ctx): Session,
    ValidJson(r): ValidJson<UpdateMeRequest>,
) -> Response {
    match UpdateMe::new(s.deps.clone())
        .handle(&ctx, r.locale.map(locale_in), r.time_zone.as_deref())
        .await
    {
        Ok(user) => json(StatusCode::OK, &Me::from(&user)),
        Err(e) => err(e),
    }
}

#[utoipa::path(get, path = "/api/v1/me/security", operation_id = "getMySecurity", tag = "me",
    responses(
        (status = 200, description = "Security state for the settings page.", body = SecurityState),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn get_my_security(State(s): S, Session(ctx): Session) -> Response {
    match GetMySecurity::new(s.deps.clone()).handle(&ctx).await {
        Ok(v) => json(
            StatusCode::OK,
            &SecurityState {
                email: v.email,
                whatsapp: v.whatsapp,
                channel_change_allowed_at: ts(v.channel_change_allowed_at),
                pending_channel_change: v.pending_channel_change.as_ref().map(change_dto),
                session: SessionInfo {
                    persistent: v.session.persistent,
                    idle_timeout_s: v.session.idle_timeout_s,
                    absolute_timeout_s: v.session.absolute_timeout_s,
                    idle_expires_at: ts(v.session.idle_expires_at),
                    absolute_expires_at: ts(v.session.absolute_expires_at),
                },
            },
        ),
        Err(e) => err(e),
    }
}

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Path)]
#[allow(dead_code)]
struct DeviceIdParam {
    #[param(value_type = contracts::http::Uuid)]
    device_id: String,
}

#[derive(utoipa::IntoParams)]
#[into_params(parameter_in = Path)]
#[allow(dead_code)]
struct ChangeIdParam {
    #[param(value_type = contracts::http::Uuid)]
    change_id: String,
}

#[utoipa::path(get, path = "/api/v1/me/devices", operation_id = "listDevices", tag = "me",
    responses(
        (status = 200, description = "Browsers that signed in to this account and are not revoked.", body = Vec<Device>),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn list_devices(State(s): S, Session(ctx): Session) -> Response {
    match ListDevices::new(s.deps.clone()).handle(&ctx).await {
        Ok(list) => json(
            StatusCode::OK,
            &list
                .iter()
                .map(|d| Device {
                    id: d.id.as_uuid().to_string(),
                    label: d.label.clone(),
                    first_seen_at: ts(d.first_seen_at),
                    last_seen_at: ts(d.last_seen_at),
                    current: d.current,
                })
                .collect::<Vec<_>>(),
        ),
        Err(e) => err(e),
    }
}

/// Forget a device — revokes it and ends its sessions (no step-up).
#[utoipa::path(post, path = "/api/v1/me/devices/{device_id}/forget", operation_id = "forgetDevice", tag = "me",
    params(DeviceIdParam),
    responses(
        (status = 204, description = "Revoked."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn forget_device(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
) -> Response {
    let id = match path_id(p, "device_id") {
        Ok(id) => DeviceId::from_uuid(id),
        Err(r) => return r,
    };
    match ForgetDevice::new(s.deps.clone())
        .handle(&ctx, id, &meta)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e),
    }
}

/// Start changing the e-mail or adding/changing WhatsApp (S3 AC-11).
#[utoipa::path(post, path = "/api/v1/me/channel-changes", operation_id = "startChannelChange", tag = "me",
    request_body = StartChannelChangeRequest,
    responses(
        (status = 202, description = "Codes sent.", body = ChannelChange),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 409, description = "me.channel_change_too_soon or me.channel_change_pending.", body = Error),
        (status = 422, description = "me.channel_unchanged, auth.channel_unavailable, or me.invalid_channel_value.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn start_channel_change(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<StartChannelChangeRequest>,
) -> Response {
    let cmd = StartChannelChangeCommand {
        channel: r.channel.domain(),
        new_value: r.new_value,
        meta,
    };
    match StartChannelChange::new(s.deps.clone())
        .handle(&ctx, cmd)
        .await
    {
        Ok(v) => json(StatusCode::ACCEPTED, &change_dto(&v)),
        Err(e) => err(e),
    }
}

/// Confirm one code of a channel change.
#[utoipa::path(post, path = "/api/v1/me/channel-changes/{change_id}/verify", operation_id = "verifyChannelChangeCode", tag = "me",
    params(ChangeIdParam),
    request_body = VerifyChannelChangeCodeRequest,
    responses(
        (status = 200, description = "Updated state of the change.", body = ChannelChange, headers(("Set-Cookie" = String))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.code_invalid or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "me.channel_change_closed.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn verify_channel_change_code(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
    ValidJson(r): ValidJson<VerifyChannelChangeCodeRequest>,
) -> Response {
    let id = match path_id(p, "change_id") {
        Ok(id) => ChannelChangeId::from_uuid(id),
        Err(r) => return r,
    };
    let cmd = VerifyChannelChangeCodeCommand {
        change_id: id,
        challenge_id: r.challenge_id,
        code: r.code,
        meta,
    };
    match VerifyChannelChangeCode::new(s.deps.clone())
        .handle(&ctx, cmd)
        .await
    {
        Ok(v) => {
            let mut resp = json(StatusCode::OK, &change_dto(&v.change));
            if let Some(c) = v
                .session
                .as_ref()
                .and_then(|c| super::cookie::set(c, s.deps.clock.now()))
            {
                resp.headers_mut().append(header::SET_COOKIE, c);
            }
            resp
        }
        Err(e) => err(e),
    }
}

/// Cancelar troca — cancel an open or on-hold channel change from any session of the user.
#[utoipa::path(post, path = "/api/v1/me/channel-changes/{change_id}/cancel", operation_id = "cancelChannelChange", tag = "me",
    params(ChangeIdParam),
    responses(
        (status = 204, description = "Cancelled."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "me.channel_change_closed.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn cancel_channel_change(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
) -> Response {
    let id = match path_id(p, "change_id") {
        Ok(id) => ChannelChangeId::from_uuid(id),
        Err(r) => return r,
    };
    match CancelChannelChange::new(s.deps.clone())
        .handle(&ctx, id, &meta)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e),
    }
}

// ---------------- admin ----------------

#[utoipa::path(get, path = "/api/v1/admin/invitations", operation_id = "listInvitations", tag = "admin",
    responses(
        (status = 200, description = "All invitations.", body = Vec<Invitation>),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.forbidden.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn list_invitations(State(s): S, AdminSession(ctx): AdminSession) -> Response {
    match ListInvitations::new(s.deps.clone()).handle(&ctx).await {
        Ok(list) => json(
            StatusCode::OK,
            &list.iter().map(invitation_dto).collect::<Vec<_>>(),
        ),
        Err(e) => err(e),
    }
}

/// Issue an invitation bound to one e-mail (Q19); link returned once.
#[utoipa::path(post, path = "/api/v1/admin/invitations", operation_id = "createInvitation", tag = "admin",
    request_body = CreateInvitationRequest,
    responses(
        (status = 201, description = "Created.", body = CreatedInvitation, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.forbidden, auth.step_up_required, auth.code_invalid or common.forbidden_origin.", body = Error),
        (status = 409, description = "invites.email_taken.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn create_invitation(
    State(s): S,
    AdminSession(ctx): AdminSession,
    Client(meta): Client,
    ValidJson(r): ValidJson<CreateInvitationRequest>,
) -> Response {
    let cmd = CreateInvitationCommand {
        email: r.email,
        locale: locale_in(r.locale),
        send_email: r.send_email.unwrap_or(true),
        step_up: proof(r.step_up),
        meta,
    };
    match CreateInvitation::new(s.deps.clone())
        .handle(&ctx, cmd)
        .await
    {
        Ok(v) => {
            if !v.email_sent {
                tracing::warn!(code = "invites.email_send_failed", invitation_id = %v.invitation.invitation.id.as_uuid(), "invitation not e-mailed");
            }
            no_store(json(
                StatusCode::CREATED,
                &CreatedInvitation {
                    invitation: invitation_dto(&v.invitation),
                    invite_url: v.invite_url.expose().to_owned(),
                    email_sent: v.email_sent,
                },
            ))
        }
        Err(e) => err(e),
    }
}

#[utoipa::path(post, path = "/api/v1/admin/invitations/{invitation_id}/revoke", operation_id = "revokeInvitation", tag = "admin",
    params(InvitationIdParam),
    responses(
        (status = 204, description = "Revoked (idempotent)."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.forbidden or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn revoke_invitation(
    State(s): S,
    AdminSession(ctx): AdminSession,
    Client(meta): Client,
    p: PathParams,
) -> Response {
    let id = match path_id(p, "invitation_id") {
        Ok(id) => InvitationId::from_uuid(id),
        Err(r) => return r,
    };
    match RevokeInvitation::new(s.deps.clone())
        .handle(&ctx, id, &meta)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e),
    }
}

#[utoipa::path(get, path = "/api/v1/admin/users", operation_id = "listUsers", tag = "admin",
    responses(
        (status = 200, description = "Identity fields only.", body = Vec<AdminUser>),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.forbidden.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn list_users(State(s): S, AdminSession(ctx): AdminSession) -> Response {
    match ListUsers::new(s.deps.clone()).handle(&ctx).await {
        Ok(list) => json(
            StatusCode::OK,
            &list.iter().map(admin_user).collect::<Vec<_>>(),
        ),
        Err(e) => err(e),
    }
}

// ---------------- sources ----------------

#[utoipa::path(get, path = "/api/v1/sources", operation_id = "listSources", tag = "sources",
    params(IncludeRevokedParam),
    responses(
        (status = 200, description = "Sources of the signed-in user.", body = Vec<Source>),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn list_sources(
    State(s): S,
    Session(ctx): Session,
    RawQuery(q): RawQuery,
) -> Response {
    let include = match query_bool(q.as_deref(), "include_revoked", false) {
        Ok(v) => v,
        Err(r) => return r.into_response(),
    };
    match ListSources::new(s.deps.clone()).handle(&ctx, include).await {
        Ok(list) => json(
            StatusCode::OK,
            &list.iter().map(source_dto).collect::<Vec<_>>(),
        ),
        Err(e) => err(e),
    }
}

/// Create a named source; webhook URL returned once (S3 AC-3, Q17; step-up).
#[utoipa::path(post, path = "/api/v1/sources", operation_id = "createSource", tag = "sources",
    request_body = CreateSourceRequest,
    responses(
        (status = 201, description = "Created.", body = SourceWithUrl, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required, auth.code_invalid or common.forbidden_origin.", body = Error),
        (status = 409, description = "sources.name_taken.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn create_source(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<CreateSourceRequest>,
) -> Response {
    let source_type = match r.source_type {
        SourceTypeDto::Tradingview => SourceType::TradingView,
    };
    match CreateSource::new(s.deps.clone())
        .handle(&ctx, &r.name, source_type, proof(r.step_up).as_ref(), &meta)
        .await
    {
        Ok(v) => no_store(json(
            StatusCode::CREATED,
            &SourceWithUrl {
                source: source_dto(&v.source),
                webhook_url: v.webhook_url.expose().to_owned(),
            },
        )),
        Err(e) => err(e),
    }
}

#[utoipa::path(patch, path = "/api/v1/sources/{source_id}", operation_id = "renameSource", tag = "sources",
    params(SourceIdParam),
    request_body = RenameRequest,
    responses(
        (status = 200, description = "Renamed.", body = Source),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "sources.name_taken or sources.revoked.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn rename_source(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
    ValidJson(r): ValidJson<RenameRequest>,
) -> Response {
    let id = match path_id(p, "source_id") {
        Ok(id) => SourceId::from_uuid(id),
        Err(r) => return r,
    };
    match RenameSource::new(s.deps.clone())
        .handle(&ctx, id, &r.name, &meta)
        .await
    {
        Ok(v) => json(StatusCode::OK, &source_dto(&v)),
        Err(e) => err(e),
    }
}

/// Replace the token immediately (old URL stops working at once; step-up).
#[utoipa::path(post, path = "/api/v1/sources/{source_id}/rotate", operation_id = "rotateSource", tag = "sources",
    params(SourceIdParam),
    request_body = StepUpBody,
    responses(
        (status = 200, description = "New URL returned once.", body = SourceWithUrl, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required, auth.code_invalid or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "sources.revoked.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn rotate_source(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
    ValidJson(b): ValidJson<StepUpBody>,
) -> Response {
    let id = match path_id(p, "source_id") {
        Ok(id) => SourceId::from_uuid(id),
        Err(r) => return r,
    };
    match RotateSource::new(s.deps.clone())
        .handle(&ctx, id, proof(b.step_up).as_ref(), &meta)
        .await
    {
        Ok(v) => no_store(json(
            StatusCode::OK,
            &SourceWithUrl {
                source: source_dto(&v.source),
                webhook_url: v.webhook_url.expose().to_owned(),
            },
        )),
        Err(e) => err(e),
    }
}

/// Revoke the source; it stays listed as revoked (no step-up).
#[utoipa::path(post, path = "/api/v1/sources/{source_id}/revoke", operation_id = "revokeSource", tag = "sources",
    params(SourceIdParam),
    responses(
        (status = 204, description = "Revoked (idempotent)."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn revoke_source(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
) -> Response {
    let id = match path_id(p, "source_id") {
        Ok(id) => SourceId::from_uuid(id),
        Err(r) => return r,
    };
    match RevokeSource::new(s.deps.clone())
        .handle(&ctx, id, &meta)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e),
    }
}

// ---------------- connections ----------------

#[utoipa::path(get, path = "/api/v1/connections", operation_id = "listConnections", tag = "connections",
    params(IncludeUnpairedParam),
    responses(
        (status = 200, description = "Connections of the signed-in user.", body = Vec<Connection>),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn list_connections(
    State(s): S,
    Session(ctx): Session,
    RawQuery(q): RawQuery,
) -> Response {
    let include = match query_bool(q.as_deref(), "include_unpaired", false) {
        Ok(v) => v,
        Err(r) => return r.into_response(),
    };
    match ListConnections::new(s.deps.clone())
        .handle(&ctx, include)
        .await
    {
        Ok(list) => json(
            StatusCode::OK,
            &list.iter().map(connection_dto).collect::<Vec<_>>(),
        ),
        Err(e) => err(e),
    }
}

/// Create a connection and its first pairing code (step-up).
#[utoipa::path(post, path = "/api/v1/connections", operation_id = "createConnection", tag = "connections",
    request_body = CreateConnectionRequest,
    responses(
        (status = 201, description = "Created, awaiting pairing.", body = CreatedConnection, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required, auth.code_invalid or common.forbidden_origin.", body = Error),
        (status = 409, description = "connections.name_taken.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn create_connection(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<CreateConnectionRequest>,
) -> Response {
    match CreateConnection::new(s.deps.clone())
        .handle(&ctx, &r.name, proof(r.step_up).as_ref(), &meta)
        .await
    {
        Ok(v) => no_store(json(
            StatusCode::CREATED,
            &CreatedConnection {
                connection: connection_dto(&v.connection),
                pairing: PairingCode {
                    pairing_code: v.pairing.pairing_code.expose().to_owned(),
                    expires_at: ts(v.pairing.expires_at),
                },
            },
        )),
        Err(e) => err(e),
    }
}

#[utoipa::path(patch, path = "/api/v1/connections/{connection_id}", operation_id = "renameConnection", tag = "connections",
    params(ConnectionIdParam),
    request_body = RenameRequest,
    responses(
        (status = 200, description = "Renamed.", body = Connection),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "connections.name_taken or connections.unpaired.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn rename_connection(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
    ValidJson(r): ValidJson<RenameRequest>,
) -> Response {
    let id = match path_id(p, "connection_id") {
        Ok(id) => ConnectionId::from_uuid(id),
        Err(r) => return r,
    };
    match RenameConnection::new(s.deps.clone())
        .handle(&ctx, id, &r.name, &meta)
        .await
    {
        Ok(v) => json(StatusCode::OK, &connection_dto(&v)),
        Err(e) => err(e),
    }
}

/// New pairing code (expired code, or "move to another machine"; step-up).
#[utoipa::path(post, path = "/api/v1/connections/{connection_id}/pairing-code", operation_id = "newPairingCode", tag = "connections",
    params(ConnectionIdParam),
    request_body = StepUpBody,
    responses(
        (status = 201, description = "New code.", body = PairingCode, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required, auth.code_invalid or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "connections.unpaired.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn new_pairing_code(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
    ValidJson(b): ValidJson<StepUpBody>,
) -> Response {
    let id = match path_id(p, "connection_id") {
        Ok(id) => ConnectionId::from_uuid(id),
        Err(r) => return r,
    };
    match NewPairingCode::new(s.deps.clone())
        .handle(&ctx, id, proof(b.step_up).as_ref(), &meta)
        .await
    {
        Ok(v) => no_store(json(
            StatusCode::CREATED,
            &PairingCode {
                pairing_code: v.pairing_code.expose().to_owned(),
                expires_at: ts(v.expires_at),
            },
        )),
        Err(e) => err(e),
    }
}

/// Unpair (Q18): revokes the device token; nothing is closed (step-up).
#[utoipa::path(post, path = "/api/v1/connections/{connection_id}/unpair", operation_id = "unpairConnection", tag = "connections",
    params(ConnectionIdParam),
    request_body = UnpairRequest,
    responses(
        (status = 204, description = "Unpaired; WebSocket closed with 4002; audited."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required, auth.code_invalid or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "connections.open_positions_not_acknowledged.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn unpair_connection(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
    ValidJson(r): ValidJson<UnpairRequest>,
) -> Response {
    let id = match path_id(p, "connection_id") {
        Ok(id) => ConnectionId::from_uuid(id),
        Err(r) => return r,
    };
    match UnpairConnection::new(s.deps.clone())
        .handle(
            &ctx,
            id,
            r.acknowledge_open_positions,
            proof(r.step_up).as_ref(),
            &meta,
        )
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e),
    }
}

/// Exchange a pairing code for a device token (called by the plugin; ADR-0004 §8).
#[utoipa::path(post, path = "/adapter/v1/pairings", operation_id = "pairAdapter", tag = "pairing", security(()),
    request_body = PairAdapterRequest,
    responses(
        (status = 201, description = "Paired; device_token returned exactly once.", body = PairingResult, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "pairing.invalid_code or common.invalid_request.", body = Error),
        (status = 426, description = "pairing.adapter_version_unsupported.", body = Error),
        (status = 429, description = "pairing.rate_limited or pairing.temporarily_locked.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn pair_adapter(
    State(s): S,
    Client(meta): Client,
    ValidJson(r): ValidJson<PairAdapterRequest>,
) -> Response {
    let Ok(instance_id) = uuid::Uuid::parse_str(&r.instance_id) else {
        return Rejection::invalid("/instance_id").into_response();
    };
    let cmd = PairAdapterCommand {
        pairing_code: r.pairing_code,
        instance_id,
        instance_name: r.instance_name,
        adapter: AdapterInfo {
            adapter_type: r.adapter.adapter_type,
            version: r.adapter.version,
            build: match r.adapter.build {
                AdapterBuildDto::Release => AdapterBuild::Release,
                AdapterBuildDto::Development => AdapterBuild::Development,
            },
        },
        ip: meta.ip,
    };
    match PairAdapter::new(s.deps.clone()).handle(cmd).await {
        Ok(v) => no_store(json(
            StatusCode::CREATED,
            &PairingResult {
                connection_id: v.connection_id.as_uuid().to_string(),
                device_token: v.device_token.expose().to_owned(),
            },
        )),
        Err(e) => err(e),
    }
}

// ---------------- events ----------------

/// Server-Sent Events stream for the signed-in user (does not extend the idle timeout).
#[utoipa::path(get, path = "/api/v1/events", operation_id = "eventStream", tag = "events",
    responses(
        (status = 200, description = "Stream.", body = String, content_type = "text/event-stream"),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn event_stream(
    State(s): S,
    StreamSession(ctx, cookie): StreamSession,
) -> Response {
    super::sse::open(
        &s.events,
        s.deps.clone(),
        ctx.user.id,
        ctx.session.id,
        ctx.session.device_id,
        cookie,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use identity_enterprise::channel::Channel;

    #[test]
    fn on_hold_changes_always_carry_hold_until() {
        let at = chrono::Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap();
        let v = ChannelChangeView {
            id: ChannelChangeId::new_v7(),
            channel: Channel::Email,
            status: ChangeStatus::OnHold,
            masked_new_value: "n***@e***.com".into(),
            codes: vec![],
            expires_at: at,
            hold_until: Some(at + chrono::Duration::hours(72)),
        };
        let json = serde_json::to_value(change_dto(&v)).unwrap();
        assert_eq!(json["status"], "on_hold");
        assert_eq!(json["hold_until"], "2026-10-26T09:00:00.000Z");
    }
}
