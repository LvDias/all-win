//! HTTP adapter end to end over PostgreSQL (as `allwin_app`): contract check precedence,
//! `Origin`, schema errors with `pointer`, session cookie, one-time secret headers and the
//! two-user isolation rule (another user's resource is 404, never 403). Skipped without
//! `TEST_DATABASE_ADMIN_URL` (see `pg_store.rs`).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

mod common;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Method, Request, StatusCode, header};
use common::*;
use http_body_util::BodyExt;
use identity_infra::http::{IdentityHttpState, SseHub, router};
use serde_json::{Value, json};
use tower::ServiceExt;

const ORIGIN: &str = "https://app.test";

struct Api {
    app: Router,
    delivery: Arc<CapturingDelivery>,
}

struct Reply {
    status: StatusCode,
    headers: axum::http::HeaderMap,
    body: Value,
}

impl Reply {
    fn code(&self) -> &str {
        self.body["error"]["code"].as_str().unwrap_or_default()
    }
    fn pointer(&self) -> &str {
        self.body["error"]["pointer"].as_str().unwrap_or_default()
    }
}

impl Api {
    async fn call(
        &self,
        method: Method,
        path: &str,
        cookie: Option<&str>,
        origin: Option<&str>,
        body: Option<Value>,
    ) -> Reply {
        self.call_with(
            method,
            path,
            cookie.map(|c| format!("__Host-aw_session={c}")),
            origin,
            body,
        )
        .await
    }

    async fn call_with(
        &self,
        method: Method,
        path: &str,
        cookie_header: Option<String>,
        origin: Option<&str>,
        body: Option<Value>,
    ) -> Reply {
        let mut req = Request::builder().method(method).uri(path).header(
            header::USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64) Firefox/131.0",
        );
        if let Some(c) = cookie_header {
            req = req.header(header::COOKIE, c);
        }
        if let Some(o) = origin {
            req = req.header(header::ORIGIN, o);
        }
        let mut req = match body {
            Some(b) => req
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(b.to_string()))
                .unwrap(),
            None => req.body(Body::empty()).unwrap(),
        };
        req.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([198, 51, 100, 20], 4000))));
        let res = self.app.clone().oneshot(req).await.unwrap();
        let status = res.status();
        let headers = res.headers().clone();
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = if bytes.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&bytes).unwrap_or(Value::Null)
        };
        Reply {
            status,
            headers,
            body,
        }
    }

    async fn post(&self, path: &str, cookie: Option<&str>, body: Value) -> Reply {
        self.call(Method::POST, path, cookie, Some(ORIGIN), Some(body))
            .await
    }

    async fn get(&self, path: &str, cookie: Option<&str>) -> Reply {
        self.call(Method::GET, path, cookie, None, None).await
    }

    /// Signs in over HTTP: request a code, read it from the capture, verify. Returns the session
    /// cookie value and the full reply.
    async fn sign_in(&self, db: &Db, email: &str, device: Option<&str>) -> (String, Reply) {
        reset_send_limits(db).await;
        let cookie = device.map(|d| format!("__Host-aw_device={d}"));
        let ch = self
            .call_with(
                Method::POST,
                "/api/v1/auth/login/challenges",
                cookie.clone(),
                Some(ORIGIN),
                Some(json!({"email": email, "channel": "email"})),
            )
            .await;
        assert_eq!(ch.status, StatusCode::ACCEPTED, "{:?}", ch.body);
        let code = self.delivery.last_code_to(&email.to_lowercase()).unwrap();
        let r = self
            .call_with(
                Method::POST,
                "/api/v1/auth/login/verify",
                cookie,
                Some(ORIGIN),
                Some(json!({"challenge_id": ch.body["challenge_id"], "code": code, "remember_me": false})),
            )
            .await;
        assert_eq!(r.status, StatusCode::OK, "{:?}", r.body);
        (set_cookie(&r, "__Host-aw_session").unwrap(), r)
    }

    async fn login(&self, db: &Db, email: &str) -> String {
        self.sign_in(db, email, None).await.0
    }
}

fn set_cookie(r: &Reply, name: &str) -> Option<String> {
    r.headers
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .find(|v| v.starts_with(&format!("{name}=")))
        .and_then(|v| v.split(';').next())
        .map(|kv| kv[name.len() + 1..].to_owned())
}

#[tokio::test(flavor = "multi_thread")]
async fn http_contract_behaviour() {
    let Some(db) = setup().await else { return };
    // Q35 off here: the step-up flows have their own test below.
    let (deps, _store, delivery) = deps_with(&db, false);
    let state = Arc::new(IdentityHttpState {
        deps: deps.clone(),
        public_origin: ORIGIN.into(),
        trust_forwarded_for: false,
        events: Arc::new(SseHub::new()),
    });
    let api = Api {
        app: router(state),
        delivery,
    };
    seed_user(&deps, "admin@example.com", true).await;
    seed_user(&deps, "ana@example.com", false).await;
    seed_user(&deps, "bob@example.com", false).await;

    let ana = api.login(&db, "ana@example.com").await;

    // Precedence: 401 before Origin; Origin before 400; admin 403 before 400.
    let r = api
        .call(Method::POST, "/api/v1/sources", None, None, Some(json!({})))
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::UNAUTHORIZED, "auth.unauthenticated")
    );
    let r = api
        .call(
            Method::POST,
            "/api/v1/sources",
            Some(&ana),
            None,
            Some(json!({})),
        )
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::FORBIDDEN, "common.forbidden_origin")
    );
    let r = api
        .call(
            Method::POST,
            "/api/v1/sources",
            Some(&ana),
            Some("https://evil.test"),
            Some(json!({})),
        )
        .await;
    assert_eq!(r.code(), "common.forbidden_origin");
    let r = api
        .call(
            Method::POST,
            "/api/v1/auth/invitations/lookup",
            None,
            None,
            Some(json!({})),
        )
        .await;
    assert_eq!(
        r.code(),
        "common.forbidden_origin",
        "public operations check Origin too"
    );
    let r = api
        .post("/api/v1/admin/invitations", Some(&ana), json!({"bogus": 1}))
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::FORBIDDEN, "auth.forbidden"),
        "non-admin never sees 400 on admin ops"
    );
    // 400 with pointers (body, path, query); framework bodies never leak.
    let r = api
        .post(
            "/api/v1/sources",
            Some(&ana),
            json!({"name": "Main", "source_type": "tradingview", "extra": true}),
        )
        .await;
    assert_eq!(
        (r.status, r.code(), r.pointer()),
        (StatusCode::BAD_REQUEST, "common.invalid_request", "/extra")
    );
    let r = api
        .post(
            "/api/v1/sources",
            Some(&ana),
            json!({"name": "", "source_type": "tradingview"}),
        )
        .await;
    assert_eq!(r.pointer(), "/name");
    let r = api
        .post("/api/v1/sources", Some(&ana), json!({"name": "Main"}))
        .await;
    assert_eq!(r.pointer(), "/source_type");
    let r = api
        .post(
            "/api/v1/sources",
            Some(&ana),
            json!({"name": "Main", "source_type": "metatrader"}),
        )
        .await;
    assert_eq!(r.pointer(), "/source_type");
    let r = api
        .post("/api/v1/sources/NOT-A-UUID/rotate", Some(&ana), json!({}))
        .await;
    assert_eq!(
        (r.status, r.pointer()),
        (StatusCode::BAD_REQUEST, "/path/source_id")
    );
    let r = api
        .get("/api/v1/sources?include_revoked=yes", Some(&ana))
        .await;
    assert_eq!(r.pointer(), "/query/include_revoked");
    let raw = api
        .call(
            Method::POST,
            "/api/v1/auth/invitations/lookup",
            None,
            Some(ORIGIN),
            None,
        )
        .await;
    assert_eq!(
        (raw.status, raw.code()),
        (StatusCode::BAD_REQUEST, "common.invalid_request")
    );

    // Source lifecycle: one-time URL with no-store; another user's source is 404.
    let r = api
        .post(
            "/api/v1/sources",
            Some(&ana),
            json!({"name": "Main", "source_type": "tradingview"}),
        )
        .await;
    assert_eq!(r.status, StatusCode::CREATED, "{:?}", r.body);
    assert_eq!(r.headers.get(header::CACHE_CONTROL).unwrap(), "no-store");
    let url = r.body["webhook_url"].as_str().unwrap().to_owned();
    assert!(url.starts_with(&format!("{ORIGIN}/ingress/v1/tradingview/awk_")));
    let source_id = r.body["source"]["id"].as_str().unwrap().to_owned();
    assert_eq!(r.body["source"]["is_automated"], true);
    // DisplayName (Esquadro, Crivo L1): trimmed; blank, control characters, a token shape
    // anywhere and more than 60 characters are 400 on /name, on create and on rename.
    let token = format!("awk_Abc123Def456_{}", "s".repeat(43));
    for bad in [
        String::new(),
        "   ".to_owned(),
        "Main\tfeed".to_owned(),
        token.clone(),
        format!("feed {token} copy"),
        "a".repeat(61),
    ] {
        let r = api
            .post(
                "/api/v1/sources",
                Some(&ana),
                json!({"name": bad, "source_type": "tradingview"}),
            )
            .await;
        assert_eq!(
            (r.status, r.code(), r.pointer()),
            (StatusCode::BAD_REQUEST, "common.invalid_request", "/name"),
            "create {bad:?}"
        );
        let r = api
            .call(
                Method::PATCH,
                &format!("/api/v1/sources/{source_id}"),
                Some(&ana),
                Some(ORIGIN),
                Some(json!({"name": bad})),
            )
            .await;
        assert_eq!(
            (r.status, r.code(), r.pointer()),
            (StatusCode::BAD_REQUEST, "common.invalid_request", "/name"),
            "rename {bad:?}"
        );
    }
    for (name, stored) in [("  Feed  ", "Feed"), (&*"a".repeat(60), &*"a".repeat(60))] {
        let r = api
            .call(
                Method::PATCH,
                &format!("/api/v1/sources/{source_id}"),
                Some(&ana),
                Some(ORIGIN),
                Some(json!({"name": name})),
            )
            .await;
        assert_eq!(r.status, StatusCode::OK, "{:?}", r.body);
        assert_eq!(r.body["name"], stored, "stored without the outer spaces");
    }
    let list = api.get("/api/v1/sources", Some(&ana)).await;
    assert!(
        !list.body.to_string().contains(&url[url.len() - 43..]),
        "list never shows the secret"
    );
    let bob = api.login(&db, "bob@example.com").await;
    for (m, path, body) in [
        (
            Method::PATCH,
            format!("/api/v1/sources/{source_id}"),
            Some(json!({"name": "Mine"})),
        ),
        (
            Method::POST,
            format!("/api/v1/sources/{source_id}/rotate"),
            None,
        ),
        (
            Method::POST,
            format!("/api/v1/sources/{source_id}/revoke"),
            None,
        ),
    ] {
        let r = api
            .call(m, &path, Some(&bob), Some(ORIGIN), body.or(Some(json!({}))))
            .await;
        assert_eq!(
            (r.status, r.code()),
            (StatusCode::NOT_FOUND, "common.not_found"),
            "{path}"
        );
    }
    assert_eq!(
        api.get("/api/v1/sources?include_revoked=true", Some(&bob))
            .await
            .body,
        json!([])
    );

    // Connections: create returns the code once; pairing returns the device token; bob gets 404.
    let r = api
        .post("/api/v1/connections", Some(&ana), json!({"name": "Desk A"}))
        .await;
    assert_eq!(r.status, StatusCode::CREATED, "{:?}", r.body);
    assert_eq!(r.body["connection"]["status"], "awaiting_pairing");
    let conn_id = r.body["connection"]["id"].as_str().unwrap().to_owned();
    let code = r.body["pairing"]["pairing_code"]
        .as_str()
        .unwrap()
        .to_owned();
    let r = api
        .call(
            Method::PATCH,
            &format!("/api/v1/connections/{conn_id}"),
            Some(&bob),
            Some(ORIGIN),
            Some(json!({"name": "x"})),
        )
        .await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    let pairing = json!({"pairing_code": code.to_lowercase(), "instance_id": uuid::Uuid::now_v7().to_string(), "instance_name": "PC",
        "adapter": {"type": "ninjatrader8", "version": "1.0.0", "build": "release"}});
    let r = api
        .call(
            Method::POST,
            "/adapter/v1/pairings",
            None,
            None,
            Some(pairing.clone()),
        )
        .await;
    assert_eq!(
        r.status,
        StatusCode::CREATED,
        "adapter endpoint has no Origin rule: {:?}",
        r.body
    );
    assert!(r.body["device_token"].as_str().unwrap().starts_with("awd_"));
    let r = api
        .call(
            Method::POST,
            "/adapter/v1/pairings",
            None,
            None,
            Some(pairing),
        )
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::BAD_REQUEST, "pairing.invalid_code")
    );
    let listed = api.get("/api/v1/connections", Some(&ana)).await;
    assert_eq!(listed.body[0]["status"], "disconnected");
    assert!(listed.body[0]["instance_id"].is_string());

    // Me, security, logout.
    let me = api.get("/api/v1/me", Some(&ana)).await;
    assert_eq!(
        me.body,
        json!({"id": me.body["id"], "email": "ana@example.com", "locale": "en", "time_zone": "UTC", "is_platform_admin": false})
    );
    let sec = api.get("/api/v1/me/security", Some(&ana)).await;
    assert_eq!(
        sec.body["session"]["idle_timeout_s"], 43_200,
        "non-persistent session"
    );
    let r = api
        .call(
            Method::PATCH,
            "/api/v1/me",
            Some(&ana),
            Some(ORIGIN),
            Some(json!({})),
        )
        .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST, "minProperties 1");
    let r = api
        .call(
            Method::PATCH,
            "/api/v1/me",
            Some(&ana),
            Some(ORIGIN),
            Some(json!({"time_zone": "Mars/Base"})),
        )
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::UNPROCESSABLE_ENTITY, "me.invalid_time_zone")
    );
    let r = api.post("/api/v1/auth/logout", Some(&ana), json!({})).await;
    assert_eq!(r.status, StatusCode::NO_CONTENT);
    assert!(
        r.headers
            .get(header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .contains("Max-Age=0")
    );
    let r = api.post("/api/v1/auth/logout", None, json!({})).await;
    assert_eq!(
        r.status,
        StatusCode::NO_CONTENT,
        "idempotent without a session"
    );
    assert_eq!(
        api.get("/api/v1/me", Some(&ana)).await.status,
        StatusCode::UNAUTHORIZED
    );

    // Admin: invitation link uses the fragment and is no-store.
    let admin = api.login(&db, "admin@example.com").await;
    let r = api
        .post(
            "/api/v1/admin/invitations",
            Some(&admin),
            json!({"email": "new@example.com", "locale": "pt-BR", "send_email": false}),
        )
        .await;
    assert_eq!(r.status, StatusCode::CREATED, "{:?}", r.body);
    assert!(
        r.body["invite_url"]
            .as_str()
            .unwrap()
            .starts_with(&format!("{ORIGIN}/pt-BR/invite#t="))
    );
    assert_eq!(r.headers.get(header::CACHE_CONTROL).unwrap(), "no-store");
    let token = r.body["invite_url"]
        .as_str()
        .unwrap()
        .split("#t=")
        .nth(1)
        .unwrap()
        .to_owned();
    let r = api
        .post(
            "/api/v1/auth/invitations/lookup",
            None,
            json!({"token": token}),
        )
        .await;
    assert_eq!(
        (r.status, r.body["email"].as_str()),
        (StatusCode::OK, Some("new@example.com"))
    );
    let users = api.get("/api/v1/admin/users", Some(&admin)).await;
    assert_eq!(users.body.as_array().unwrap().len(), 3);

    db.teardown().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn http_passwordless_flows() {
    let Some(db) = setup().await else { return };
    let (deps, _store, delivery) = deps(&db);
    let state = Arc::new(IdentityHttpState {
        deps: deps.clone(),
        public_origin: ORIGIN.into(),
        trust_forwarded_for: false,
        events: Arc::new(SseHub::new()),
    });
    let api = Api {
        app: router(state),
        delivery,
    };
    seed_user(&deps, "ana@example.com", false).await;
    seed_user(&deps, "bob@example.com", false).await;

    // Login request: identical 202 for a registered and an unknown e-mail.
    let known = api
        .post(
            "/api/v1/auth/login/challenges",
            None,
            json!({"email": "ana@example.com", "channel": "email"}),
        )
        .await;
    let unknown = api
        .post(
            "/api/v1/auth/login/challenges",
            None,
            json!({"email": "zz@example.com", "channel": "email"}),
        )
        .await;
    assert_eq!(
        (known.status, unknown.status),
        (StatusCode::ACCEPTED, StatusCode::ACCEPTED)
    );
    let keys = |r: &Reply| {
        let mut k: Vec<String> = r.body.as_object().unwrap().keys().cloned().collect();
        k.sort();
        k
    };
    assert_eq!(keys(&known), keys(&unknown));
    assert_eq!(known.body["resend_after_s"], 60);
    // Wrong code: the same 401 for both.
    let wrong = |ch: &Reply| json!({"challenge_id": ch.body["challenge_id"], "code": "0000-0000", "remember_me": false});
    let a = api
        .post("/api/v1/auth/login/verify", None, wrong(&known))
        .await;
    let b = api
        .post("/api/v1/auth/login/verify", None, wrong(&unknown))
        .await;
    assert_eq!(
        (a.status, a.code()),
        (StatusCode::UNAUTHORIZED, "auth.code_invalid")
    );
    assert_eq!((b.status, b.body.clone()), (a.status, a.body.clone()));
    // Resend within the cooldown: 429 with Retry-After for both.
    for ch in [&known, &unknown] {
        let r = api
            .post(
                "/api/v1/auth/challenges/resend",
                None,
                json!({"challenge_id": ch.body["challenge_id"]}),
            )
            .await;
        assert_eq!(
            (r.status, r.code()),
            (StatusCode::TOO_MANY_REQUESTS, "auth.rate_limited")
        );
        assert!(r.headers.get(header::RETRY_AFTER).is_some());
    }
    let d = api
        .post(
            "/api/v1/auth/challenges/delivery",
            None,
            json!({"challenge_id": known.body["challenge_id"]}),
        )
        .await;
    assert_eq!(d.body, json!({"delivery": "not_reported"}));
    let r = api
        .post(
            "/api/v1/auth/login/verify",
            None,
            json!({"challenge_id": "x".repeat(20), "code": "1".repeat(33), "remember_me": false}),
        )
        .await;
    assert_eq!(
        (r.status, r.pointer()),
        (StatusCode::BAD_REQUEST, "/code"),
        "more than 32 characters is 400"
    );

    // Full sign-in: session and device cookies with the contract attributes.
    let (ana, reply) = api.sign_in(&db, "ana@example.com", None).await;
    assert_eq!(reply.body["new_device"], true);
    let cookies: Vec<String> = reply
        .headers
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap().to_owned())
        .collect();
    assert!(cookies.iter().any(|c| c.starts_with("__Host-aw_session=")
        && c.contains("HttpOnly; Secure; SameSite=Lax; Path=/")));
    assert!(
        cookies
            .iter()
            .any(|c| c.starts_with("__Host-aw_device=") && c.ends_with("Max-Age=34560000"))
    );
    let device = set_cookie(&reply, "__Host-aw_device").unwrap();
    let (_, again) = api.sign_in(&db, "ana@example.com", Some(&device)).await;
    assert_eq!(again.body["new_device"], false, "known browser");

    // Step-up: required, bound and single use.
    let r = api
        .post(
            "/api/v1/sources",
            Some(&ana),
            json!({"name": "Main", "source_type": "tradingview"}),
        )
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::FORBIDDEN, "auth.step_up_required")
    );
    reset_send_limits(&db).await;
    let ch = api
        .post(
            "/api/v1/auth/step-up",
            Some(&ana),
            json!({"action": "source_create", "channel": "email"}),
        )
        .await;
    assert_eq!(ch.status, StatusCode::ACCEPTED, "{:?}", ch.body);
    let d = api
        .post(
            "/api/v1/auth/challenges/delivery",
            Some(&ana),
            json!({"challenge_id": ch.body["challenge_id"]}),
        )
        .await;
    assert_eq!(
        d.body,
        json!({"delivery": "pending"}),
        "a step-up challenge reports its delivery"
    );
    let proof = json!({"challenge_id": ch.body["challenge_id"], "code": api.delivery.last_code_to("ana@example.com").unwrap()});
    let r = api
        .post(
            "/api/v1/sources",
            Some(&ana),
            json!({"name": "Main", "source_type": "tradingview", "step_up": proof}),
        )
        .await;
    assert_eq!(r.status, StatusCode::CREATED, "{:?}", r.body);
    let source_id = r.body["source"]["id"].as_str().unwrap().to_owned();
    let r = api
        .post(
            "/api/v1/sources",
            Some(&ana),
            json!({"name": "Other", "source_type": "tradingview", "step_up": proof}),
        )
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::FORBIDDEN, "auth.code_invalid")
    );
    let bob = api.login(&db, "bob@example.com").await;
    let r = api
        .post(
            "/api/v1/auth/step-up",
            Some(&bob),
            json!({"action": "source_rotate", "target_id": source_id, "channel": "email"}),
        )
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::NOT_FOUND, "common.not_found")
    );
    let r = api
        .post(
            "/api/v1/auth/step-up",
            Some(&bob),
            json!({"action": "source_rotate", "channel": "email"}),
        )
        .await;
    assert_eq!(
        (r.status, r.pointer()),
        (StatusCode::BAD_REQUEST, "/target_id")
    );
    let r = api
        .post(
            "/api/v1/auth/step-up",
            Some(&bob),
            json!({"action": "source_create", "channel": "whatsapp"}),
        )
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::UNPROCESSABLE_ENTITY, "auth.channel_unavailable")
    );

    // Channel change: two codes, cancel from the same user, 404 for another.
    reset_send_limits(&db).await;
    let r = api
        .post(
            "/api/v1/me/channel-changes",
            Some(&ana),
            json!({"channel": "email", "new_value": "new@example.com"}),
        )
        .await;
    assert_eq!(r.status, StatusCode::ACCEPTED, "{:?}", r.body);
    assert_eq!(r.body["codes"].as_array().unwrap().len(), 2);
    assert_eq!(r.body["masked_new_value"], "n***@e***.com");
    let change = r.body["id"].as_str().unwrap().to_owned();
    let sec = api.get("/api/v1/me/security", Some(&ana)).await;
    assert_eq!(sec.body["pending_channel_change"]["id"], change.as_str());
    let r = api
        .post(
            &format!("/api/v1/me/channel-changes/{change}/cancel"),
            Some(&bob),
            json!({}),
        )
        .await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    let r = api
        .post(
            &format!("/api/v1/me/channel-changes/{change}/cancel"),
            Some(&ana),
            json!({}),
        )
        .await;
    assert_eq!(r.status, StatusCode::NO_CONTENT);
    let r = api
        .post(
            "/api/v1/me/channel-changes",
            Some(&ana),
            json!({"channel": "email", "new_value": "ana@example.com"}),
        )
        .await;
    assert_eq!(
        (r.status, r.code()),
        (StatusCode::UNPROCESSABLE_ENTITY, "me.channel_unchanged")
    );

    // Devices and sign out everywhere.
    let devices = api.get("/api/v1/me/devices", Some(&ana)).await;
    assert_eq!(devices.body.as_array().unwrap().len(), 1);
    assert_eq!(devices.body[0]["label"], "Firefox on Linux");
    assert_eq!(devices.body[0]["current"], true);
    let r = api
        .post("/api/v1/auth/logout-everywhere", Some(&ana), json!({}))
        .await;
    assert_eq!(r.status, StatusCode::NO_CONTENT);
    let cleared: Vec<String> = r
        .headers
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap().to_owned())
        .collect();
    assert!(cleared.iter().any(|c| c.starts_with("__Host-aw_session=;")));
    assert!(cleared.iter().any(|c| c.starts_with("__Host-aw_device=;")));
    assert_eq!(
        api.get("/api/v1/me", Some(&ana)).await.status,
        StatusCode::UNAUTHORIZED
    );
    db.teardown().await;
}
