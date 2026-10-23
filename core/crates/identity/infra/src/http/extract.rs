//! Request context extractors. Argument order in handlers follows the contract precedence:
//! session (401) → `Origin` (403) → admin (403) → body/parameters (400) → use case
//! (404 → step-up 403 → 409/422).

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use identity_application::auth::{Activity, AuthContext, AuthenticateSession};
use identity_application::deps::RequestMeta;
use identity_application::error::IdentityError;

use super::IdentityHttpState;
use super::error::ApiError;
use super::guard::{Rejection, check_origin};

/// Client facts for throttling and audit. Behind Caddy, the last `X-Forwarded-For` entry.
pub struct Client(pub RequestMeta);

fn client_ip(parts: &Parts, trust_forwarded_for: bool) -> Option<IpAddr> {
    if trust_forwarded_for
        && let Some(ip) = parts
            .headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit(',').next())
            .and_then(|v| v.trim().parse().ok())
    {
        return Some(ip);
    }
    parts
        .extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|c| c.0.ip())
}

impl FromRequestParts<Arc<IdentityHttpState>> for Client {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        s: &Arc<IdentityHttpState>,
    ) -> Result<Self, Self::Rejection> {
        let ip = client_ip(parts, s.trust_forwarded_for)
            .ok_or_else(|| Rejection::internal().into_response())?;
        let user_agent = parts
            .headers
            .get(axum::http::header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.chars().take(256).collect());
        Ok(Self(RequestMeta { ip, user_agent }))
    }
}

/// Public operation: only the `Origin` check (state-changing methods).
pub struct PublicOrigin;

impl FromRequestParts<Arc<IdentityHttpState>> for PublicOrigin {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        s: &Arc<IdentityHttpState>,
    ) -> Result<Self, Self::Rejection> {
        check_origin(&parts.method, &parts.headers, &s.public_origin)
            .map_err(IntoResponse::into_response)?;
        Ok(Self)
    }
}

// axum rejections are responses; boxing them would only move the allocation.
#[allow(clippy::result_large_err)]
async fn authenticate(
    parts: &Parts,
    s: &Arc<IdentityHttpState>,
    activity: Activity,
) -> Result<AuthContext, Response> {
    let cookie = super::cookie::read(&parts.headers);
    let ctx = AuthenticateSession::new(s.deps.clone())
        .handle(cookie.as_deref(), activity)
        .await
        .map_err(|e| ApiError(e).into_response())?;
    check_origin(&parts.method, &parts.headers, &s.public_origin)
        .map_err(IntoResponse::into_response)?;
    Ok(ctx)
}

/// Full session (every operation except the login verify endpoints).
pub struct Session(pub AuthContext);

impl FromRequestParts<Arc<IdentityHttpState>> for Session {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        s: &Arc<IdentityHttpState>,
    ) -> Result<Self, Self::Rejection> {
        authenticate(parts, s, Activity::UserInitiated)
            .await
            .map(Self)
    }
}

/// Full session of a platform admin; `auth.forbidden` before anything else can fail.
pub struct AdminSession(pub AuthContext);

impl FromRequestParts<Arc<IdentityHttpState>> for AdminSession {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        s: &Arc<IdentityHttpState>,
    ) -> Result<Self, Self::Rejection> {
        let ctx = authenticate(parts, s, Activity::UserInitiated).await?;
        ctx.require_admin()
            .map_err(|e| ApiError(e).into_response())?;
        Ok(Self(ctx))
    }
}

/// SSE: a full session that the stream itself never refreshes (idle timeout keeps running).
pub struct StreamSession(pub AuthContext, pub String);

impl FromRequestParts<Arc<IdentityHttpState>> for StreamSession {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        s: &Arc<IdentityHttpState>,
    ) -> Result<Self, Self::Rejection> {
        let cookie = super::cookie::read(&parts.headers)
            .ok_or_else(|| ApiError(IdentityError::Unauthenticated).into_response())?;
        let ctx = authenticate(parts, s, Activity::Background).await?;
        Ok(Self(ctx, cookie))
    }
}
