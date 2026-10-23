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
use identity_application::usecases::admin::{AdminResetTotp, ListUsers};
use identity_application::usecases::connections::*;
use identity_application::usecases::invitations::*;
use identity_application::usecases::me::*;
use identity_application::usecases::pairing::{PairAdapter, PairAdapterCommand};
use identity_application::usecases::password_reset::*;
use identity_application::usecases::signin::*;
use identity_application::usecases::sources::*;
use identity_application::usecases::totp::*;
use identity_application::views::{
    ConnectionView, InvitationView, LoginOutcome, SessionCookie, SourceView,
};
use identity_enterprise::connection::{AdapterBuild, AdapterInfo, ConnectionStatus};
use identity_enterprise::ids::{ConnectionId, InvitationId, SourceId};
use identity_enterprise::invitation::InvitationStatus;
use identity_enterprise::secret::SecretString;
use identity_enterprise::source::{SourceStatus, SourceType};
use identity_enterprise::user::UserStatus;
use serde::Serialize;
use shared_kernel::UserId;

use super::IdentityHttpState;
use super::error::{ApiError, respond};
use super::extract::{AdminSession, Client, PendingSession, PublicOrigin, Session, StreamSession};
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

fn with_cookie(mut r: Response, cookie: &SessionCookie, s: &IdentityHttpState) -> Response {
    if let Some(v) = super::cookie::set(cookie, s.deps.clock.now()) {
        r.headers_mut().append(header::SET_COOKIE, v);
    }
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

fn login_response(s: &IdentityHttpState, status: StatusCode, outcome: LoginOutcome) -> Response {
    let (body, cookie) = match outcome {
        LoginOutcome::Authenticated {
            user,
            cookie,
            recovery_codes_remaining,
        } => (
            LoginResult {
                state: LoginState::Authenticated,
                user: Some(Me::from(&user)),
                pending_expires_at: None,
                recovery_codes_remaining,
            },
            cookie,
        ),
        LoginOutcome::Pending2fa {
            cookie,
            pending_expires_at,
        } => (
            LoginResult {
                state: LoginState::Pending2fa,
                user: None,
                pending_expires_at: Some(ts(pending_expires_at)),
                recovery_codes_remaining: None,
            },
            cookie,
        ),
    };
    with_cookie(json(status, &body), &cookie, s)
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
        id: u.user.id.to_string(),
        email: u.user.email.as_entered().to_owned(),
        status: match u.user.status {
            UserStatus::Active => UserStatusDto::Active,
            UserStatus::Closed => UserStatusDto::Closed,
        },
        totp_enabled: u.totp_enabled,
        is_platform_admin: Some(u.user.is_platform_admin),
        created_at: ts(u.user.created_at),
        last_sign_in_at: u.last_sign_in_at.map(ts),
    }
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

/// Sign in with e-mail and password.
#[utoipa::path(post, path = "/api/v1/auth/login", operation_id = "login", tag = "auth", security(()),
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authenticated, or second factor required.", body = LoginResult, headers(("Set-Cookie" = String))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.invalid_credentials.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn login(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    ValidJson(r): ValidJson<LoginRequest>,
) -> Response {
    let cmd = LoginCommand {
        email: r.email,
        password: SecretString::new(r.password),
        remember_me: r.remember_me,
        meta,
    };
    match Login::new(s.deps.clone()).handle(cmd).await {
        Ok(out) => login_response(&s, StatusCode::OK, out),
        Err(e) => err(e),
    }
}

/// Complete sign-in with a TOTP code.
#[utoipa::path(post, path = "/api/v1/auth/login/totp", operation_id = "verifyLoginTotp", tag = "auth",
    security(("pendingSession" = [])),
    request_body = TotpCodeRequest,
    responses(
        (status = 200, description = "Authenticated; session id rotated.", body = LoginResult, headers(("Set-Cookie" = String))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.invalid_code or auth.pending_expired.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn verify_login_totp(
    State(s): S,
    PendingSession(ctx): PendingSession,
    Client(meta): Client,
    ValidJson(r): ValidJson<TotpCodeRequest>,
) -> Response {
    match VerifyLoginTotp::new(s.deps.clone())
        .handle(&ctx, &r.code, &meta)
        .await
    {
        Ok(out) => login_response(&s, StatusCode::OK, out),
        Err(e) => err(e),
    }
}

/// Complete sign-in with a single-use recovery code (Q22).
#[utoipa::path(post, path = "/api/v1/auth/login/recovery-code", operation_id = "verifyLoginRecoveryCode", tag = "auth",
    security(("pendingSession" = [])),
    request_body = RecoveryCodeRequest,
    responses(
        (status = 200, description = "Authenticated; recovery_codes_remaining included.", body = LoginResult, headers(("Set-Cookie" = String))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.invalid_code or auth.pending_expired.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn verify_login_recovery_code(
    State(s): S,
    PendingSession(ctx): PendingSession,
    Client(meta): Client,
    ValidJson(r): ValidJson<RecoveryCodeRequest>,
) -> Response {
    match VerifyLoginRecoveryCode::new(s.deps.clone())
        .handle(&ctx, &r.code, &meta)
        .await
    {
        Ok(out) => login_response(&s, StatusCode::OK, out),
        Err(e) => err(e),
    }
}

/// End the current session (server-side). Idempotent.
#[utoipa::path(post, path = "/api/v1/auth/logout", operation_id = "logout", tag = "auth",
    security(("session" = []), ("pendingSession" = []), ()),
    responses(
        (status = 204, description = "Session ended; cookie cleared.", headers(("Set-Cookie" = String))),
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

/// Re-authenticate before a sensitive action (15-minute rule).
#[utoipa::path(post, path = "/api/v1/auth/step-up", operation_id = "stepUp", tag = "auth",
    request_body = StepUpRequest,
    responses(
        (status = 204, description = "Re-authenticated."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated, auth.reauth_failed or auth.invalid_code.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn step_up(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<StepUpRequest>,
) -> Response {
    let cmd = StepUpCommand {
        password: SecretString::new(r.password),
        code: r.code,
        meta,
    };
    match StepUp::new(s.deps.clone()).handle(&ctx, cmd).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
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

/// Create the account from an invitation and sign in (S3 AC-1).
#[utoipa::path(post, path = "/api/v1/auth/invitations/accept", operation_id = "acceptInvitation", tag = "auth", security(()),
    request_body = AcceptInvitationRequest,
    responses(
        (status = 201, description = "Account created; session started.", body = LoginResult, headers(("Set-Cookie" = String))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "auth.invitation_invalid.", body = Error),
        (status = 422, description = "Password rejected.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn accept_invitation(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    ValidJson(r): ValidJson<AcceptInvitationRequest>,
) -> Response {
    let cmd = AcceptInvitationCommand {
        token: r.token,
        password: SecretString::new(r.password),
        locale: locale_in(r.locale),
        time_zone: r.time_zone,
        meta,
    };
    match AcceptInvitation::new(s.deps.clone()).handle(cmd).await {
        Ok(out) => login_response(&s, StatusCode::CREATED, out),
        Err(e) => err(e),
    }
}

/// Request a reset link by e-mail (always 202, Q20).
#[utoipa::path(post, path = "/api/v1/auth/password-reset/request", operation_id = "requestPasswordReset", tag = "auth", security(()),
    request_body = EmailRequest,
    responses(
        (status = 202, description = "Returned for every well-formed same-origin request."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn request_password_reset(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    ValidJson(r): ValidJson<EmailRequest>,
) -> Response {
    match RequestPasswordReset::new(s.deps.clone())
        .handle(&r.email, &meta)
        .await
    {
        Ok(()) => StatusCode::ACCEPTED.into_response(),
        Err(e) => err(e),
    }
}

/// Set a new password with a reset token; ends ALL sessions.
#[utoipa::path(post, path = "/api/v1/auth/password-reset/confirm", operation_id = "confirmPasswordReset", tag = "auth", security(()),
    request_body = ConfirmPasswordResetRequest,
    responses(
        (status = 204, description = "Password changed; every session ended."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 404, description = "auth.reset_token_invalid.", body = Error),
        (status = 422, description = "Password rejected.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn confirm_password_reset(
    State(s): S,
    _o: PublicOrigin,
    Client(meta): Client,
    ValidJson(r): ValidJson<ConfirmPasswordResetRequest>,
) -> Response {
    match ConfirmPasswordReset::new(s.deps.clone())
        .handle(&r.token, &SecretString::new(r.new_password), &meta)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
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
                totp_enabled: v.totp_enabled,
                recovery_codes_remaining: v.recovery_codes_remaining,
                session: SessionInfo {
                    persistent: v.session.persistent,
                    idle_timeout_s: v.session.idle_timeout_s,
                    absolute_timeout_s: v.session.absolute_timeout_s,
                    idle_expires_at: ts(v.session.idle_expires_at),
                    absolute_expires_at: ts(v.session.absolute_expires_at),
                    step_up_valid_until: ts(v.session.step_up_valid_until),
                },
            },
        ),
        Err(e) => err(e),
    }
}

/// Start TOTP enrolment (step-up).
#[utoipa::path(post, path = "/api/v1/me/totp/setup", operation_id = "startTotpSetup", tag = "me",
    responses(
        (status = 200, description = "Enrolment secret, shown once.", body = TotpSetup, headers(("Cache-Control" = NoStore))),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required or common.forbidden_origin.", body = Error),
        (status = 409, description = "auth.totp_already_enabled.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn start_totp_setup(State(s): S, Session(ctx): Session) -> Response {
    match StartTotpSetup::new(s.deps.clone()).handle(&ctx).await {
        Ok(v) => no_store(json(
            StatusCode::OK,
            &TotpSetup {
                otpauth_uri: v.otpauth_uri.expose().to_owned(),
                secret: v.secret.expose().to_owned(),
                expires_at: ts(v.expires_at),
            },
        )),
        Err(e) => err(e),
    }
}

/// Confirm enrolment with a code; returns recovery codes once; ends other sessions.
#[utoipa::path(post, path = "/api/v1/me/totp/confirm", operation_id = "confirmTotpSetup", tag = "me",
    request_body = TotpCodeRequest,
    responses(
        (status = 200, description = "TOTP enabled.", body = RecoveryCodes, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 422, description = "auth.invalid_code or auth.totp_setup_expired.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn confirm_totp_setup(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<TotpCodeRequest>,
) -> Response {
    match ConfirmTotpSetup::new(s.deps.clone())
        .handle(&ctx, &r.code, &meta)
        .await
    {
        Ok(v) => no_store(json(
            StatusCode::OK,
            &RecoveryCodes {
                recovery_codes: v.codes.iter().map(|c| c.expose().to_owned()).collect(),
            },
        )),
        // The contract answers a wrong enrolment code with 422, not 401.
        Err(e @ IdentityError::InvalidCode) => respond(&e, StatusCode::UNPROCESSABLE_ENTITY),
        Err(e) => err(e),
    }
}

/// Disable TOTP (password + current code, Q22); ends other sessions.
#[utoipa::path(post, path = "/api/v1/me/totp/disable", operation_id = "disableTotp", tag = "me",
    request_body = PasswordAndCode,
    responses(
        (status = 204, description = "TOTP disabled; recovery codes deleted."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated, auth.reauth_failed or auth.invalid_code.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn disable_totp(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<PasswordAndCode>,
) -> Response {
    match DisableTotp::new(s.deps.clone())
        .handle(&ctx, &SecretString::new(r.password), &r.code, &meta)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e),
    }
}

/// Replace the recovery code batch (password + current code).
#[utoipa::path(post, path = "/api/v1/me/recovery-codes/regenerate", operation_id = "regenerateRecoveryCodes", tag = "me",
    request_body = PasswordAndCode,
    responses(
        (status = 200, description = "New batch, shown once.", body = RecoveryCodes, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated, auth.reauth_failed or auth.invalid_code.", body = Error),
        (status = 403, description = "common.forbidden_origin.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn regenerate_recovery_codes(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<PasswordAndCode>,
) -> Response {
    match RegenerateRecoveryCodes::new(s.deps.clone())
        .handle(&ctx, &SecretString::new(r.password), &r.code, &meta)
        .await
    {
        Ok(v) => no_store(json(
            StatusCode::OK,
            &RecoveryCodes {
                recovery_codes: v.codes.iter().map(|c| c.expose().to_owned()).collect(),
            },
        )),
        Err(e) => err(e),
    }
}

/// Change password (step-up); ends other sessions.
#[utoipa::path(post, path = "/api/v1/me/password", operation_id = "changePassword", tag = "me",
    request_body = ChangePasswordRequest,
    responses(
        (status = 204, description = "Changed; current session rotated, other sessions ended."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated or auth.reauth_failed.", body = Error),
        (status = 403, description = "auth.step_up_required or common.forbidden_origin.", body = Error),
        (status = 422, description = "Password rejected.", body = Error),
        (status = 429, description = "auth.rate_limited.", body = Error, headers(("Retry-After" = RetryAfterSeconds))),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn change_password(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<ChangePasswordRequest>,
) -> Response {
    let cmd = ChangePasswordCommand {
        current_password: SecretString::new(r.current_password),
        new_password: SecretString::new(r.new_password),
        meta,
    };
    match ChangePassword::new(s.deps.clone()).handle(&ctx, cmd).await {
        Ok(cookie) => with_cookie(StatusCode::NO_CONTENT.into_response(), &cookie, &s),
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
        (status = 403, description = "auth.forbidden or common.forbidden_origin.", body = Error),
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

/// Generate a reset link when e-mail is unavailable (Q20 fallback; step-up).
#[utoipa::path(post, path = "/api/v1/admin/users/{user_id}/password-reset-link", operation_id = "adminCreateResetLink", tag = "admin",
    params(UserIdParam),
    responses(
        (status = 201, description = "Link returned once.", body = ResetLink, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.forbidden, auth.step_up_required or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn admin_create_reset_link(
    State(s): S,
    AdminSession(ctx): AdminSession,
    Client(meta): Client,
    p: PathParams,
) -> Response {
    let id = match path_id(p, "user_id") {
        Ok(id) => UserId::from_uuid(id),
        Err(r) => return r,
    };
    match AdminCreateResetLink::new(s.deps.clone())
        .handle(&ctx, id, &meta)
        .await
    {
        Ok(v) => no_store(json(
            StatusCode::CREATED,
            &ResetLink {
                reset_url: v.reset_url.expose().to_owned(),
                expires_at: ts(v.expires_at),
            },
        )),
        Err(e) => err(e),
    }
}

/// Remove a user's TOTP after an identity check (Q22; step-up); ends that user's sessions.
#[utoipa::path(post, path = "/api/v1/admin/users/{user_id}/totp-reset", operation_id = "adminResetTotp", tag = "admin",
    params(UserIdParam),
    responses(
        (status = 204, description = "TOTP and recovery codes removed."),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.forbidden, auth.step_up_required or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn admin_reset_totp(
    State(s): S,
    AdminSession(ctx): AdminSession,
    Client(meta): Client,
    p: PathParams,
) -> Response {
    let id = match path_id(p, "user_id") {
        Ok(id) => UserId::from_uuid(id),
        Err(r) => return r,
    };
    match AdminResetTotp::new(s.deps.clone())
        .handle(&ctx, id, &meta)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
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
        (status = 403, description = "auth.step_up_required or common.forbidden_origin.", body = Error),
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
        .handle(&ctx, &r.name, source_type, &meta)
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
    responses(
        (status = 200, description = "New URL returned once.", body = SourceWithUrl, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "sources.revoked.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn rotate_source(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
) -> Response {
    let id = match path_id(p, "source_id") {
        Ok(id) => SourceId::from_uuid(id),
        Err(r) => return r,
    };
    match RotateSource::new(s.deps.clone())
        .handle(&ctx, id, &meta)
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
    request_body = RenameRequest,
    responses(
        (status = 201, description = "Created, awaiting pairing.", body = CreatedConnection, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required or common.forbidden_origin.", body = Error),
        (status = 409, description = "connections.name_taken.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn create_connection(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    ValidJson(r): ValidJson<RenameRequest>,
) -> Response {
    match CreateConnection::new(s.deps.clone())
        .handle(&ctx, &r.name, &meta)
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
    responses(
        (status = 201, description = "New code.", body = PairingCode, headers(("Cache-Control" = NoStore))),
        (status = 400, description = "Invalid request.", body = Error),
        (status = 401, description = "auth.unauthenticated.", body = Error),
        (status = 403, description = "auth.step_up_required or common.forbidden_origin.", body = Error),
        (status = 404, description = "common.not_found.", body = Error),
        (status = 409, description = "connections.unpaired.", body = Error),
        (status = 500, description = "Internal error.", body = Error),
    ))]
pub(super) async fn new_pairing_code(
    State(s): S,
    Session(ctx): Session,
    Client(meta): Client,
    p: PathParams,
) -> Response {
    let id = match path_id(p, "connection_id") {
        Ok(id) => ConnectionId::from_uuid(id),
        Err(r) => return r,
    };
    match NewPairingCode::new(s.deps.clone())
        .handle(&ctx, id, &meta)
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
        (status = 403, description = "auth.step_up_required or common.forbidden_origin.", body = Error),
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
        .handle(&ctx, id, r.acknowledge_open_positions, &meta)
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
        cookie,
    )
}
