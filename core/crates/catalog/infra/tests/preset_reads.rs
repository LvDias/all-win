//! listFirms / getFirm / listFirmReleases / getPresetVersion over releases published by the real
//! publisher; every response is checked against the HTTP contract schema (5.1.1).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::sync::Arc;

use access::{AccessError, Activity, Caller, SessionAuthenticator};
use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use catalog_application::enterprise::release::import::ImportPolicy;
use catalog_application::preset::PresetQueries;
use catalog_application::release::{PublishRelease, PublishedBy};
use catalog_infra::PgInstrumentRepository;
use catalog_infra::preset_http::{PresetsHttpState, component, router};
use catalog_infra::release_file::parse_release;
use catalog_infra::release_store::PgReleaseStore;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use shared_kernel::UserId;
use tower::ServiceExt;
use uuid::Uuid;

macro_rules! example {
    ($name:literal) => {
        include_str!(concat!(
            "../../../../../contracts/presets/v1/examples/",
            $name
        ))
    };
}

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

fn conforms(value: &Value, name: &str) {
    let validator = jsonschema::validator_for(component(name).unwrap()).unwrap();
    let errors: Vec<String> = validator
        .iter_errors(value)
        .map(|e| format!("{e} at {}", e.instance_path()))
        .collect();
    assert!(errors.is_empty(), "{name}: {errors:?}\n{value:#}");
}

fn with_slug(raw: &str, slug: &str) -> Value {
    let mut v: Value = serde_json::from_str(raw).unwrap();
    v["firm"]["slug"] = json!(slug);
    v
}

async fn publish(db: &support::TestDb, v: &Value) {
    let p = PublishRelease::new(
        Arc::new(PgReleaseStore::new(db.publisher.clone())),
        ImportPolicy {
            accept_personal_device_not_confirmed: false,
        },
    );
    p.handle(
        &parse_release(&serde_json::to_vec(v).unwrap()).unwrap(),
        &PublishedBy::parse("torno").unwrap(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn firm_reads_match_the_contract() {
    let Some(db) = support::db().await else {
        return;
    };
    let slug = format!("r-{}", Uuid::now_v7().simple());
    publish(
        &db,
        &with_slug(example!("valid/release-v1-new-firm.json"), &slug),
    )
    .await;
    let mut v2 = with_slug(example!("valid/release-v2-stance-changed.json"), &slug);
    v2["plans"][0]["phases"][0]["sizes"][0]["rule_sets"][0]["rules"]["drawdown"]["value"]["amount"] =
        json!("2500.00");
    publish(&db, &v2).await;
    let per_platform = format!("p-{}", Uuid::now_v7().simple());
    publish(
        &db,
        &with_slug(
            example!("valid/release-not-confirmed-per-platform.json"),
            &per_platform,
        ),
    )
    .await;

    let app = router(Arc::new(PresetsHttpState::new(
        PresetQueries::new(Arc::new(PgInstrumentRepository::new(db.app.clone()))),
        Arc::new(AnySession),
        "https://app.test".into(),
    )));

    assert_eq!(
        get(&app, "/api/v1/firms", false).await.0,
        StatusCode::UNAUTHORIZED
    );
    let (s, list) = get(&app, "/api/v1/firms", true).await;
    assert_eq!(s, StatusCode::OK, "{list}");
    conforms(&list, "FirmList");
    let firm = list["firms"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["slug"] == slug)
        .unwrap()
        .clone();
    assert_eq!(firm["release"]["version_number"], 2);
    assert_eq!(firm["stances"]["automation"], "forbidden");
    let other = list["firms"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["slug"] == per_platform)
        .unwrap();
    assert_eq!(other["stances"]["automation"], "varies_by_platform");
    assert!(other["not_confirmed_count"].as_u64().unwrap() >= 1);

    let id = firm["id"].as_str().unwrap();
    let (s, detail) = get(&app, &format!("/api/v1/firms/{id}"), true).await;
    assert_eq!(s, StatusCode::OK, "{detail}");
    conforms(&detail, "FirmDetail");
    assert_eq!(detail["excluded_plans"], json!(["Instant Funding"]));
    assert_eq!(detail["stances"]["automation"][0]["version_number"], 2);

    let (s, releases) = get(&app, &format!("/api/v1/firms/{id}/releases"), true).await;
    assert_eq!(s, StatusCode::OK, "{releases}");
    conforms(&releases, "FirmReleaseList");
    let newest = &releases["releases"][0];
    assert_eq!(newest["version_number"], 2);
    // The file's preset key became the preset id.
    let changed = newest["changes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["kind"] == "rule_value_changed")
        .unwrap();
    assert!(changed["preset_id"].is_string() && changed.get("preset").is_none());
    // Unchanged copy text keeps its id across releases; automation has a new one.
    let ids = |r: &Value, kind: &str| {
        r["rule_versions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["kind"] == kind)
            .unwrap()["id"]
            .clone()
    };
    let oldest = &releases["releases"][1];
    assert_eq!(ids(newest, "copy"), ids(oldest, "copy"));
    assert_ne!(ids(newest, "automation"), ids(oldest, "automation"));

    // Preset versions: the old one is superseded by the new one.
    let option = &detail["plans"][0]["phases"][0]["sizes"][0];
    let new_version = option["preset_version_id"].as_str().unwrap().to_owned();
    let (s, pv) = get(
        &app,
        &format!("/api/v1/preset-versions/{new_version}"),
        true,
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{pv}");
    conforms(&pv, "PresetVersion");
    assert!(pv.get("superseded_by").is_none());
    let old_version: Uuid = sqlx::query_scalar(
        "SELECT v.id FROM catalog.preset_version v JOIN catalog.firm_release r ON r.id = v.release_id
         WHERE v.preset_id = $1 AND r.version_number = 1",
    )
    .bind(Uuid::parse_str(option["preset_id"].as_str().unwrap()).unwrap())
    .fetch_one(&db.admin)
    .await
    .unwrap();
    let (_, old) = get(
        &app,
        &format!("/api/v1/preset-versions/{old_version}"),
        true,
    )
    .await;
    conforms(&old, "PresetVersion");
    assert_eq!(old["superseded_by"], json!(new_version));

    // 400 / 404.
    let (s, e) = get(&app, "/api/v1/firms/NOPE", true).await;
    assert_eq!(
        (s, e["error"]["pointer"].as_str()),
        (StatusCode::BAD_REQUEST, Some("/path/firm_id"))
    );
    assert_eq!(
        get(&app, &format!("/api/v1/firms/{}", Uuid::now_v7()), true)
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        get(
            &app,
            &format!("/api/v1/firms/{}/releases", Uuid::now_v7()),
            true
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        get(
            &app,
            &format!("/api/v1/preset-versions/{}", Uuid::now_v7()),
            true
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
}
