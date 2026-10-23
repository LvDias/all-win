//! HTTP adapter of the identity module (V1-S3): `/api/v1` auth, me, admin, sources,
//! connections and events, plus the plugin's `POST /adapter/v1/pairings`.
//!
//! `app` merges [`router`] and [`IdentityApi`] (one line each). Handlers follow the contract
//! precedence through their extractor order (see [`extract`]).

pub mod cookie;
pub mod error;
pub mod extract;
pub mod guard;
mod handlers;
pub mod schemas;
pub mod sse;

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, patch, post};
use identity_application::deps::IdentityDeps;
use utoipa::OpenApi;

pub use sse::SseHub;

pub struct IdentityHttpState {
    pub deps: IdentityDeps,
    /// `APP_PUBLIC_ORIGIN`: the only accepted `Origin` on state-changing `/api/v1` requests.
    pub public_origin: String,
    /// Take the client IP from the last `X-Forwarded-For` entry (set by Caddy).
    pub trust_forwarded_for: bool,
    pub events: Arc<SseHub>,
}

pub fn router(state: Arc<IdentityHttpState>) -> Router {
    guard::register_components(&IdentityApi::openapi());
    use handlers::*;
    Router::new()
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/invitations/lookup", post(lookup_invitation))
        .route("/api/v1/me", get(get_me).patch(update_me))
        .route("/api/v1/me/security", get(get_my_security))
        .route(
            "/api/v1/admin/invitations",
            get(list_invitations).post(create_invitation),
        )
        .route(
            "/api/v1/admin/invitations/{invitation_id}/revoke",
            post(revoke_invitation),
        )
        .route("/api/v1/admin/users", get(list_users))
        .route("/api/v1/sources", get(list_sources).post(create_source))
        .route("/api/v1/sources/{source_id}", patch(rename_source))
        .route("/api/v1/sources/{source_id}/rotate", post(rotate_source))
        .route("/api/v1/sources/{source_id}/revoke", post(revoke_source))
        .route(
            "/api/v1/connections",
            get(list_connections).post(create_connection),
        )
        .route(
            "/api/v1/connections/{connection_id}",
            patch(rename_connection),
        )
        .route(
            "/api/v1/connections/{connection_id}/pairing-code",
            post(new_pairing_code),
        )
        .route(
            "/api/v1/connections/{connection_id}/unpair",
            post(unpair_connection),
        )
        .route("/adapter/v1/pairings", post(pair_adapter))
        .route("/api/v1/events", get(event_stream))
        .with_state(state)
}

/// OpenAPI of the identity module; `app` merges it into the Core document.
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::logout,
        handlers::lookup_invitation,
        handlers::get_me,
        handlers::update_me,
        handlers::get_my_security,
        handlers::list_invitations,
        handlers::create_invitation,
        handlers::revoke_invitation,
        handlers::list_users,
        handlers::list_sources,
        handlers::create_source,
        handlers::rename_source,
        handlers::rotate_source,
        handlers::revoke_source,
        handlers::list_connections,
        handlers::create_connection,
        handlers::rename_connection,
        handlers::new_pairing_code,
        handlers::unpair_connection,
        handlers::pair_adapter,
        handlers::event_stream,
    ),
    components(schemas(contracts::http::RetryAfterSeconds, schemas::NoStore))
)]
pub struct IdentityApi;
