//! `IdentityError` → contract status and `Error` body. Internal details go to the technical
//! log only (never secrets: errors carry no bound values or tokens).

use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use identity_application::error::IdentityError;

use super::guard::Rejection;

pub struct ApiError(pub IdentityError);

impl From<IdentityError> for ApiError {
    fn from(e: IdentityError) -> Self {
        Self(e)
    }
}

pub fn status_of(e: &IdentityError) -> StatusCode {
    use IdentityError as E;
    match e {
        E::Unauthenticated
        | E::InvalidCredentials
        | E::InvalidCode
        | E::PendingExpired
        | E::ReauthFailed => StatusCode::UNAUTHORIZED,
        E::Forbidden | E::StepUpRequired => StatusCode::FORBIDDEN,
        E::RateLimited { .. } | E::PairingRateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
        E::InvitationInvalid | E::ResetTokenInvalid | E::NotFound => StatusCode::NOT_FOUND,
        E::PasswordRejected { .. } | E::TotpSetupExpired | E::InvalidTimeZone => {
            StatusCode::UNPROCESSABLE_ENTITY
        }
        E::InvalidRequest { .. } | E::PairingInvalidCode => StatusCode::BAD_REQUEST,
        E::TotpAlreadyEnabled
        | E::InvitationEmailTaken
        | E::SourceNameTaken
        | E::SourceRevoked
        | E::ConnectionNameTaken
        | E::ConnectionUnpaired
        | E::OpenPositionsNotAcknowledged => StatusCode::CONFLICT,
        E::AdapterVersionUnsupported => StatusCode::UPGRADE_REQUIRED,
        E::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        respond(&self.0, status_of(&self.0))
    }
}

/// Same body, explicit status (e.g. `confirmTotpSetup` answers `auth.invalid_code` with 422).
pub fn respond(e: &IdentityError, status: StatusCode) -> Response {
    if let IdentityError::Internal(detail) = e {
        tracing::error!(error = %detail, "identity request failed");
        return Rejection::internal().into_response();
    }
    let mut body = contracts::http::Error::new(&e.code());
    if let Some(pointer) = e.pointer() {
        body = body.with_pointer(pointer);
    }
    let retry = e
        .retry_after()
        .map(|d| u32::try_from(d.as_secs().max(1)).unwrap_or(u32::MAX));
    body.error.retry_after_s = retry;
    let mut response = (status, axum::Json(body)).into_response();
    if let Some(secs) = retry
        && let Ok(v) = HeaderValue::from_str(&secs.to_string())
    {
        response.headers_mut().insert(header::RETRY_AFTER, v);
    }
    response
}
