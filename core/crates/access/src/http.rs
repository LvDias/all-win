//! axum extractor for other modules' handlers (feature `axum`): resolves the session cookie
//! (401 `auth.unauthenticated`), then checks `Origin` on state-changing methods
//! (403 `common.forbidden_origin`), in the contract's precedence order. Bodies follow the contract
//! `Error` schema: `{"error": {"code": …}}`.

use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};

use crate::{AccessError, Activity, Caller, SessionAuthenticator};

pub const SESSION_COOKIE: &str = "__Host-aw_session";

/// What a module's router state must provide to authenticate callers.
pub trait AccessState: Send + Sync {
    fn authenticator(&self) -> &Arc<dyn SessionAuthenticator>;
    /// Public origin of the app (`https://host`, no trailing slash).
    fn public_origin(&self) -> &str;
}

/// The authenticated caller of a user-initiated request (extends the idle timeout).
pub struct Authenticated(pub Caller);

impl<S> FromRequestParts<Arc<S>> for Authenticated
where
    S: AccessState + 'static,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<S>,
    ) -> Result<Self, Self::Rejection> {
        let cookie = session_cookie(&parts.headers);
        let caller = state
            .authenticator()
            .authenticate(cookie.as_deref(), Activity::UserInitiated)
            .await
            .map_err(error_response)?;
        if !origin_allowed(&parts.method, &parts.headers, state.public_origin()) {
            return Err(json_error(StatusCode::FORBIDDEN, "common.forbidden_origin"));
        }
        Ok(Self(caller))
    }
}

/// The `__Host-aw_session` value, if present.
pub fn session_cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(name, _)| *name == SESSION_COOKIE)
        .map(|(_, value)| value.to_owned())
}

/// State-changing methods need `Origin` equal to the public origin.
pub fn origin_allowed(method: &Method, headers: &HeaderMap, allowed: &str) -> bool {
    let safe = matches!(*method, Method::GET | Method::HEAD | Method::OPTIONS);
    safe || headers
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|origin| origin == allowed)
}

/// Contract response for an [`AccessError`] (with `Retry-After` on 429).
pub fn error_response(e: AccessError) -> Response {
    let status = StatusCode::from_u16(e.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut r = json_error(status, e.code());
    if let AccessError::RateLimited { retry_after_s } = e
        && let Ok(v) = HeaderValue::from_str(&retry_after_s.max(1).to_string())
    {
        r.headers_mut().insert(header::RETRY_AFTER, v);
    }
    r
}

fn json_error(status: StatusCode, code: &str) -> Response {
    let body = serde_json::json!({ "error": { "code": code } });
    (status, axum::Json(body)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use axum::Router;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::{get, post};
    use shared_kernel::UserId;
    use tower::ServiceExt;
    use uuid::Uuid;

    struct OnlyGood;

    #[async_trait]
    impl SessionAuthenticator for OnlyGood {
        async fn authenticate(&self, c: Option<&str>, _: Activity) -> Result<Caller, AccessError> {
            match c {
                Some("good") => Ok(Caller {
                    user_id: UserId::new_v7(),
                    session_id: Uuid::nil(),
                    device_id: Uuid::nil(),
                    is_platform_admin: false,
                }),
                _ => Err(AccessError::Unauthenticated),
            }
        }
    }

    struct St(Arc<dyn SessionAuthenticator>);

    impl AccessState for St {
        fn authenticator(&self) -> &Arc<dyn SessionAuthenticator> {
            &self.0
        }
        fn public_origin(&self) -> &str {
            "https://app.test"
        }
    }

    async fn call(
        method: &str,
        cookie: Option<&str>,
        origin: Option<&str>,
    ) -> (StatusCode, String) {
        let app = Router::new()
            .route("/x", get(|_: Authenticated| async { "ok" }))
            .route("/x", post(|_: Authenticated| async { "ok" }))
            .with_state(Arc::new(St(Arc::new(OnlyGood))));
        let mut req = Request::builder().method(method).uri("/x");
        if let Some(c) = cookie {
            req = req.header(header::COOKIE, format!("a=b; {SESSION_COOKIE}={c}"));
        }
        if let Some(o) = origin {
            req = req.header(header::ORIGIN, o);
        }
        let res = app.oneshot(req.body(Body::empty()).unwrap()).await.unwrap();
        let status = res.status();
        let body = axum::body::to_bytes(res.into_body(), 1024).await.unwrap();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn precedence_is_401_then_origin() {
        let (s, b) = call("POST", None, None).await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        assert!(b.contains("auth.unauthenticated"));
        let (s, b) = call("POST", Some("good"), Some("https://evil.test")).await;
        assert_eq!(s, StatusCode::FORBIDDEN);
        assert!(b.contains("common.forbidden_origin"));
        assert_eq!(
            call("POST", Some("good"), Some("https://app.test")).await.0,
            StatusCode::OK
        );
        assert_eq!(call("GET", Some("good"), None).await.0, StatusCode::OK);
    }
}
