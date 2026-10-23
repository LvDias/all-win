//! `getOptInRules` / `getRuleVersion` against the migrated catalog (generic rule versions, Q40).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::sync::Arc;

use access::{AccessError, Activity, Caller, SessionAuthenticator};
use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use catalog_application::rule_version::RuleVersionQueries;
use catalog_infra::rule_version::PgRuleVersionRepository;
use catalog_infra::rule_version_http::{RuleVersionHttpState, router};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use shared_kernel::UserId;
use tower::ServiceExt;
use uuid::Uuid;

const AUTOMATION_V1: &str = "019a1000-0000-7000-8000-000000000001";
const COPY_V1: &str = "019a1000-0000-7000-8000-000000000002";

struct AnySession;

#[async_trait]
impl SessionAuthenticator for AnySession {
    async fn authenticate(&self, c: Option<&str>, _: Activity) -> Result<Caller, AccessError> {
        c.ok_or(AccessError::Unauthenticated)?;
        Ok(Caller {
            user_id: UserId::new_v7(),
            session_id: Uuid::nil(),
            device_id: Uuid::nil(),
            is_platform_admin: false,
        })
    }
}

async fn get(app: &axum::Router, uri: &str, signed_in: bool) -> (StatusCode, Value) {
    let mut req = Request::builder().uri(uri);
    if signed_in {
        req = req.header(header::COOKIE, "__Host-aw_session=s");
    }
    let res = app
        .clone()
        .oneshot(req.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn generic_rule_versions_are_served() {
    let Some(db) = support::db().await else {
        return;
    };
    let queries = RuleVersionQueries::new(Arc::new(PgRuleVersionRepository::new(db.app.clone())));
    let app = router(Arc::new(RuleVersionHttpState::new(
        queries,
        Arc::new(AnySession),
        "https://app.test".into(),
    )));
    assert_eq!(
        get(&app, "/api/v1/opt-in-rules", false).await.0,
        StatusCode::UNAUTHORIZED
    );

    let (s, rules) = get(&app, "/api/v1/opt-in-rules", true).await;
    assert_eq!(s, StatusCode::OK, "{rules}");
    assert_eq!(rules["automation"]["id"], AUTOMATION_V1);
    assert_eq!(rules["copy"]["id"], COPY_V1);
    assert_eq!(rules["automation"]["firm_stance"], "unknown");
    assert_eq!(rules["automation"]["is_generic"], true);
    assert_eq!(rules["automation"]["quotes"], json!([]));
    assert_eq!(rules["automation"]["published_at"], "2026-10-23T00:00:00Z");
    assert!(rules["automation"].get("firm_id").is_none());

    let (s, one) = get(&app, &format!("/api/v1/rule-versions/{COPY_V1}"), true).await;
    assert_eq!((s, one["kind"].as_str()), (StatusCode::OK, Some("copy")));
    let (s, _) = get(
        &app,
        &format!("/api/v1/rule-versions/{}", Uuid::now_v7()),
        true,
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    let (s, e) = get(&app, "/api/v1/rule-versions/nope", true).await;
    assert_eq!(
        (s, e["error"]["pointer"].as_str()),
        (StatusCode::BAD_REQUEST, Some("/path/rule_version_id"))
    );

    let (s, _) = get(
        &app,
        &format!("/api/v1/opt-in-rules?preset_version_id={}", Uuid::now_v7()),
        true,
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND, "no preset catalog before S16");
    let (s, e) = get(&app, "/api/v1/opt-in-rules?firm_platform=rithmic", true).await;
    assert_eq!(s, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        e["error"],
        json!({"code": "accounts.firm_platform_unknown", "pointer": "/query/firm_platform"})
    );
    let (s, e) = get(&app, "/api/v1/opt-in-rules?firm_platform=Bad!", true).await;
    assert_eq!(
        (s, e["error"]["pointer"].as_str()),
        (StatusCode::BAD_REQUEST, Some("/query/firm_platform"))
    );
}
