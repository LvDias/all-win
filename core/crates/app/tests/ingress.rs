//! S1 ingress against PostgreSQL: AC-1–AC-5, AC-7, idempotency, isolation and the
//! empty-404 rule (s1-webhook-ingest §2, §5).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

mod support;

use std::path::Path;

use axum::body::Body;
use axum::http::Request;
use serde_json::Value;
use support::*;

macro_rules! db_or_skip {
    () => {
        &match db().await {
            Some(db) => db,
            None => return,
        }
    };
}

fn json(body: &[u8]) -> Value {
    serde_json::from_slice(body).unwrap()
}

async fn count(db: &TestDb, sql: &str, user: uuid::Uuid) -> i64 {
    let sql = sql.to_owned();
    as_user(db, user, move |c| {
        Box::pin(async move {
            sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(sql))
                .fetch_one(c)
                .await
                .unwrap()
        })
    })
    .await
}

#[tokio::test]
async fn accepted_then_duplicate_with_audit_entries() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    let msg = unique_message_id();

    let (status, headers, body) = send(
        &app.state,
        post_json(&ingress_path(&s.token), instruction(&msg)),
    )
    .await;
    assert_eq!(status, 200);
    let first = json(&body);
    assert_eq!(first["status"], "accepted");
    assert_eq!(headers["x-content-type-options"], "nosniff");
    assert_eq!(headers["cache-control"], "no-store");

    let (status, _, body) = send(
        &app.state,
        post_json(&ingress_path(&s.token), instruction(&msg)),
    )
    .await;
    assert_eq!(status, 200);
    let second = json(&body);
    assert_eq!(second["status"], "duplicate");
    assert_eq!(second["instruction_id"], first["instruction_id"]);

    let iid: uuid::Uuid = first["instruction_id"].as_str().unwrap().parse().unwrap();
    let row: (String, Option<String>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>) =
        as_user(db, s.user_id, move |c| {
            Box::pin(async move {
                sqlx::query_as(
                    "SELECT message_id, source_key_id, sent_at, received_at, expiry_base FROM routing.instruction WHERE id = $1",
                )
                .bind(iid)
                .fetch_one(c)
                .await
                .unwrap()
            })
        })
        .await;
    assert_eq!(row.0, msg);
    assert_eq!(row.1.as_deref(), Some(s.key_id.as_str()));
    assert_eq!(
        row.4,
        row.2.min(row.3),
        "expiry_base = min(sent_at, received_at)"
    );

    drain(&app, db, s.user_id).await;
    type Row = (
        String,
        Option<String>,
        Value,
        Option<String>,
        String,
        String,
        bool,
        i64,
    );
    let codes: Vec<Row> = as_user(db, s.user_id, |c| {
        Box::pin(async move {
            sqlx::query_as(
                "SELECT code, source_key_id, details, message_id, source_name, actor || '/' || outcome,
                        safe_to_display, event_seq
                   FROM audit.audit_entry ORDER BY event_seq",
            )
                .fetch_all(c)
                .await
                .unwrap()
        })
    })
    .await;
    let kinds: Vec<&str> = codes.iter().map(|c| c.0.as_str()).collect();
    assert_eq!(kinds, ["ingress.received", "ingress.duplicate_ignored"]);
    assert!(
        codes
            .iter()
            .all(|c| c.1.as_deref() == Some(s.key_id.as_str()))
    );
    // Columns from refs; details exactly DetailsIngressReceived / DetailsIngressDuplicate.
    assert!(codes.iter().all(|c| c.3.as_deref() == Some(msg.as_str())));
    assert!(codes.iter().all(|c| !c.4.is_empty() && c.6));
    assert_eq!(
        codes.iter().map(|c| c.5.as_str()).collect::<Vec<_>>(),
        ["system/received", "system/skipped"]
    );
    assert!(
        codes[0].7 > 0 && codes[1].7 > codes[0].7,
        "event_seq is causal"
    );
    assert!(codes[0].2.get("message_id").is_none());
    assert_eq!(codes[0].2["action"], "buy");
    assert!(codes[0].2.get("sent_at").is_some());
    assert!(codes[1].2.get("original_instruction_id").is_some());
    // Redelivery is a no-op.
    sqlx::query("UPDATE app.outbox SET published_at = NULL WHERE user_id = $1")
        .bind(s.user_id)
        .execute(&db.worker)
        .await
        .unwrap();
    drain(&app, db, s.user_id).await;
    assert_eq!(
        count(db, "SELECT count(*) FROM audit.audit_entry", s.user_id).await,
        2
    );
}

#[tokio::test]
async fn twenty_parallel_duplicates_store_exactly_one_instruction() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    let msg = unique_message_id();
    let tasks: Vec<_> = (0..20)
        .map(|_| {
            let state = app.state.clone();
            let req = post_json(&ingress_path(&s.token), instruction(&msg));
            tokio::spawn(async move { send(&state, req).await })
        })
        .collect();
    let mut statuses = Vec::new();
    for t in tasks {
        let (status, _, body) = t.await.unwrap();
        assert_eq!(status, 200);
        statuses.push(json(&body)["status"].as_str().unwrap().to_owned());
    }
    assert_eq!(statuses.iter().filter(|s| *s == "accepted").count(), 1);
    assert_eq!(statuses.iter().filter(|s| *s == "duplicate").count(), 19);
    assert_eq!(
        count(db, "SELECT count(*) FROM routing.instruction", s.user_id).await,
        1
    );
    assert_eq!(
        count(
            db,
            "SELECT count(*) FROM routing.message_receipt",
            s.user_id
        )
        .await,
        1
    );
}

#[tokio::test]
async fn same_message_id_for_two_users_and_rls_isolation() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let (a, b) = (seed(db).await, seed(db).await);
    let msg = unique_message_id();
    for s in [&a, &b] {
        let (status, _, body) = send(
            &app.state,
            post_json(&ingress_path(&s.token), instruction(&msg)),
        )
        .await;
        assert_eq!(
            (status, json(&body)["status"].as_str().unwrap()),
            (200, "accepted")
        );
    }
    assert_eq!(
        count(db, "SELECT count(*) FROM routing.instruction", a.user_id).await,
        1
    );
    assert_eq!(
        count(db, "SELECT count(*) FROM routing.instruction", b.user_id).await,
        1
    );
    // RLS on its own, without an application filter: no context → no rows.
    let unscoped: i64 = sqlx::query_scalar("SELECT count(*) FROM routing.instruction")
        .fetch_one(&db.app)
        .await
        .unwrap();
    assert_eq!(unscoped, 0);
    // A's context cannot write a row for B.
    let b_id = b.user_id;
    let denied = as_user(db, a.user_id, move |c| {
        Box::pin(async move {
            sqlx::query("INSERT INTO routing.message_receipt (user_id, message_id, instruction_id, received_at) VALUES ($1, 'x', $2, now())")
                .bind(b_id)
                .bind(uuid::Uuid::now_v7())
                .execute(c)
                .await
        })
    })
    .await;
    assert!(denied.is_err());
}

fn fixtures(kind: &str) -> Vec<(String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../contracts/instruction/v1/examples")
        .join(kind);
    let mut out: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .map(|p| {
            (
                p.file_name().unwrap().to_string_lossy().into_owned(),
                std::fs::read_to_string(&p).unwrap(),
            )
        })
        .collect();
    out.sort();
    out
}

#[tokio::test]
async fn contract_fixtures_get_the_documented_answers() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    for (name, body) in fixtures("valid") {
        // Each run is a fresh user, so fixture message_ids never collide across runs.
        let v: Value = serde_json::from_str(&body).unwrap();
        let (status, _, resp) =
            send(&app.state, post_json(&ingress_path(&s.token), body.clone())).await;
        let resp = json(&resp);
        let expected = if v["source"]["type"] != "tradingview" {
            (422, Some("ingress.source_mismatch"))
        } else if name == "future-minor-version.json" {
            (400, Some("ingress.unsupported_version"))
        } else {
            (200, None)
        };
        assert_eq!(status, expected.0, "{name}: {resp}");
        assert_eq!(resp["error"]["code"].as_str(), expected.1, "{name}");
    }
    for (name, body) in fixtures("invalid") {
        let (status, _, resp) = send(&app.state, post_json(&ingress_path(&s.token), body)).await;
        let resp = json(&resp);
        assert_eq!(status, 400, "{name}: {resp}");
        let code = resp["error"]["code"].as_str().unwrap();
        assert!(
            ["ingress.schema_violation", "ingress.unsupported_version"].contains(&code),
            "{name}: {code}"
        );
        assert_eq!(
            resp.as_object().unwrap().len(),
            2,
            "{name}: only status/error"
        );
    }
}

#[tokio::test]
async fn policy_and_catalog_rejections_are_422_and_audited() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    let base = |extra: &str| {
        format!(
            r#"{{"schema_version":"1.0","message_id":"{}","sent_at":"2026-10-22T16:03:20Z",
                "source":{{"type":"tradingview"}}{extra}}}"#,
            unique_message_id()
        )
    };
    let cases = [
        (
            r#","signal_type":"trigger","action":"modify","instrument":{"symbol":"MNQ"},"refs":{"order_ref":"o1"},"changes":{"quantity":2}"#,
            "ingress.action_not_allowed_for_source",
            "/action",
        ),
        (
            r#","signal_type":"trigger","action":"sell","position_effect":"reduce","quantity":1,"instrument":{"symbol":"MNQ"}"#,
            "ingress.action_not_allowed_for_source",
            "/position_effect",
        ),
        (
            r#","signal_type":"trigger","action":"buy","quantity":1,"instrument":{"symbol":"MNQ"},"order":{"type":"limit","limit_price":"21000.25"}"#,
            "ingress.order_type_not_allowed_for_source",
            "/order/type",
        ),
        (
            r#","signal_type":"master_account","action":"buy","quantity":1,"instrument":{"symbol":"MNQ"}"#,
            "ingress.signal_type_not_allowed_for_source",
            "/signal_type",
        ),
        (
            r#","signal_type":"trigger","action":"buy","quantity":1,"instrument":{"symbol":"ES"}"#,
            "ingress.unsupported_instrument",
            "/instrument/symbol",
        ),
    ];
    for (extra, code, pointer) in cases {
        let (status, _, resp) =
            send(&app.state, post_json(&ingress_path(&s.token), base(extra))).await;
        let resp = json(&resp);
        assert_eq!(status, 422, "{code}: {resp}");
        assert_eq!(resp["error"]["code"], code);
        assert_eq!(resp["error"]["pointer"], pointer);
    }
    assert_eq!(
        count(db, "SELECT count(*) FROM routing.instruction", s.user_id).await,
        0
    );
    drain(&app, db, s.user_id).await;
    let rejected = count(
        db,
        "SELECT count(*) FROM audit.audit_entry WHERE code = 'ingress.rejected'",
        s.user_id,
    )
    .await;
    assert_eq!(rejected, 5);
    let sized = count(
        db,
        "SELECT count(*) FROM audit.audit_entry WHERE code = 'ingress.rejected' AND outcome = 'rejected'
           AND (details->>'body_size')::int > 0 AND details ? 'error_code' AND details ? 'pointer'
           AND message_id IS NOT NULL AND NOT details ? 'message_id'",
        s.user_id,
    )
    .await;
    assert_eq!(
        sized, 5,
        "rejections carry code, pointer, message_id and body size"
    );
}

#[tokio::test]
async fn enabling_an_instrument_is_data_only() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    let body = |m: &str| instruction(m).replace("\"MNQ\"", "\"MES\"");
    let (status, _, _) = send(
        &app.state,
        post_json(&ingress_path(&s.token), body(&unique_message_id())),
    )
    .await;
    assert_eq!(status, 422);
    // Reference data change (as an admin migration would do); no code change.
    sqlx::query(
        "INSERT INTO catalog.instrument (id, symbol, name, exchange, tick_size, point_value, session_code, is_enabled)
         VALUES ($1, 'MES', 'Micro E-mini S&P 500', 'CME', 0.25, 5.0000, 'cme_equity_index', true)
         ON CONFLICT (symbol) DO UPDATE SET is_enabled = true",
    )
    .bind(uuid::Uuid::now_v7())
    .execute(&db.owner)
    .await
    .unwrap();
    let (status, _, _) = send(
        &app.state,
        post_json(&ingress_path(&s.token), body(&unique_message_id())),
    )
    .await;
    assert_eq!(status, 200);
}

#[tokio::test]
async fn transport_rejections_with_a_valid_token() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    let path = ingress_path(&s.token);
    let xml = Request::post(path.as_str())
        .header("content-type", "application/xml")
        .body(Body::from("<a/>"))
        .unwrap();
    let (status, _, resp) = send(&app.state, xml).await;
    assert_eq!(
        (status, json(&resp)["error"]["code"].as_str()),
        (415, Some("ingress.unsupported_media_type"))
    );

    let big = format!(r#"{{"pad":"{}"}}"#, "x".repeat(20 * 1024));
    let (status, _, resp) = send(&app.state, post_json(&path, big)).await;
    assert_eq!(
        (status, json(&resp)["error"]["code"].as_str()),
        (413, Some("ingress.payload_too_large"))
    );

    let text = Request::post(path.as_str())
        .header("content-type", "text/plain")
        .body(Body::from(instruction(&unique_message_id())))
        .unwrap();
    assert_eq!(
        send(&app.state, text).await.0,
        200,
        "text/plain is parsed like JSON"
    );

    let impossible_date =
        instruction(&unique_message_id()).replace("2026-10-22T16:03:20Z", "2026-02-30T16:03:20Z");
    let (status, _, resp) = send(&app.state, post_json(&path, impossible_date)).await;
    assert_eq!(
        (status, json(&resp)["error"]["code"].as_str()),
        (400, Some("ingress.schema_violation"))
    );

    let (status, _, resp) = send(&app.state, post_json(&path, "{not json")).await;
    assert_eq!(
        (status, json(&resp)["error"]["code"].as_str()),
        (400, Some("ingress.invalid_json"))
    );

    let (status, _, resp) = send(
        &app.state,
        post_json(
            &path,
            instruction(&unique_message_id()).replace("\"quantity\":1", "\"quantity\":0"),
        ),
    )
    .await;
    let resp = json(&resp);
    assert_eq!(status, 400);
    assert_eq!(resp["error"]["code"], "ingress.schema_violation");
    assert_eq!(resp["error"]["pointer"], "/quantity");
}

#[tokio::test]
async fn bad_tokens_and_near_misses_get_an_empty_404() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    let (k, sec) = (&s.key_id, &s.secret);
    let mut encoded_one = sec.clone();
    let c = encoded_one.remove(5);
    encoded_one.insert_str(5, &format!("%{:02X}", c as u8));
    let paths = [
        format!("/ingress/v1/tradingview/awk_{k}_{sec}X"),
        format!("/ingress/v1/tradingview/awk_{k}_{}", &sec[..30]),
        format!("/ingress/v1/tradingview//awk_{k}_{sec}"),
        format!("//ingress/v1/tradingview/awk_{k}_{sec}"),
        format!("/Ingress/v1/TradingView/awk_{k}_{sec}"),
        format!("/INGRESS/V1/TRADINGVIEW/awk_{k}_{sec}"),
        format!("/ingress/v1/tradingview/awk_{k}%5F{sec}"),
        format!(
            "/ingress/v1/tradingview/awk_{k}_{}%2B{}",
            &sec[..20],
            &sec[20..]
        ),
        format!("/ingress/v1/tradingview/awk_{k}_{encoded_one}"),
        format!("/ingress/v1/tradingview/{sec}"),
        format!("/ingress/v1/tradingview?t=awk_{k}_{sec}"),
        format!("/%69ngress/v1/tradingview/awk_{k}_{sec}"),
        format!("/ingress/v1/tradingview/awk_{k}_{sec}/"),
        format!("/ingress/v1/tradingview/awk_AAAAAAAAAAAA_{sec}"),
        format!("/api/v1/awk_{k}_{sec}"),
    ];
    for p in paths {
        let (status, _, body) =
            send(&app.state, post_json(&p, instruction(&unique_message_id()))).await;
        assert_eq!(
            (status, body.len()),
            (404, 0),
            "{}",
            p.replace(sec.as_str(), "<secret>")
        );
    }
    let get = Request::get(ingress_path(&s.token))
        .body(Body::empty())
        .unwrap();
    let (status, _, body) = send(&app.state, get).await;
    assert_eq!((status, body.len()), (404, 0));

    // Revoked (M5): the same empty 404.
    assert!(app::dev::revoke_key(&db.owner, &s.key_id).await.unwrap());
    let (status, _, body) = send(
        &app.state,
        post_json(&ingress_path(&s.token), instruction(&unique_message_id())),
    )
    .await;
    assert_eq!((status, body.len()), (404, 0));
    assert_eq!(
        count(db, "SELECT count(*) FROM routing.instruction", s.user_id).await,
        0
    );
}

#[tokio::test]
async fn per_key_rate_limit_answers_429_with_retry_after() {
    let db = db_or_skip!();
    let mut cfg = config();
    cfg.rate_per_key_per_min = 3;
    let app = wire(db, &cfg);
    let s = seed(db).await;
    for _ in 0..3 {
        assert_eq!(
            send(
                &app.state,
                post_json(&ingress_path(&s.token), instruction(&unique_message_id()))
            )
            .await
            .0,
            200
        );
    }
    let (status, headers, body) = send(
        &app.state,
        post_json(&ingress_path(&s.token), instruction(&unique_message_id())),
    )
    .await;
    assert_eq!(status, 429);
    assert_eq!(json(&body)["error"]["code"], "ingress.rate_limited");
    assert!(
        headers["retry-after"]
            .to_str()
            .unwrap()
            .parse::<u64>()
            .unwrap()
            >= 1
    );
    // Audited as its own code with DetailsIngressRateLimited.
    drain(&app, db, s.user_id).await;
    let limited = count(
        db,
        "SELECT count(*) FROM audit.audit_entry WHERE code = 'ingress.rate_limited' AND outcome = 'rejected'
           AND actor = 'system' AND (details->>'retry_after_s')::int >= 1 AND source_name IS NOT NULL",
        s.user_id,
    )
    .await;
    assert_eq!(limited, 1);
}

fn from_ip(request: Request<Body>, ip: &str) -> Request<Body> {
    let (mut parts, body) = request.into_parts();
    parts.headers.insert("x-forwarded-for", ip.parse().unwrap());
    Request::from_parts(parts, body)
}

#[tokio::test]
async fn flood_cap_counts_every_request_per_ip_before_the_token() {
    let db = db_or_skip!();
    let mut cfg = config();
    cfg.ip_flood_per_min = 2;
    let app = wire(db, &cfg);
    let s = seed(db).await;
    let ok = || post_json(&ingress_path(&s.token), instruction(&unique_message_id()));
    assert_eq!(send(&app.state, ok()).await.0, 200);
    assert_eq!(
        send(&app.state, post_json("/ingress/v1/tradingview/nope", "{}"))
            .await
            .0,
        404
    );
    let (status, headers, body) = send(&app.state, ok()).await;
    assert_eq!(status, 429, "the flood cap applies to valid tokens too");
    assert_eq!(json(&body)["error"]["code"], "ingress.rate_limited");
    assert!(headers.contains_key("retry-after"));
}

#[tokio::test]
async fn tradingview_ranges_get_the_higher_flood_cap() {
    let db = db_or_skip!();
    let mut cfg = config();
    cfg.trust_forwarded_for = true;
    cfg.ip_flood_per_min = 2;
    cfg.tradingview_flood_per_min = 50;
    cfg.tradingview_source_ranges = vec!["52.89.214.0/24".parse().unwrap()];
    let app = wire(db, &cfg);
    let s = seed(db).await;
    let ok = |ip: &str| {
        from_ip(
            post_json(&ingress_path(&s.token), instruction(&unique_message_id())),
            ip,
        )
    };
    for _ in 0..10 {
        assert_eq!(send(&app.state, ok("52.89.214.238")).await.0, 200);
    }
    assert_eq!(send(&app.state, ok("198.51.100.20")).await.0, 200);
    assert_eq!(send(&app.state, ok("198.51.100.20")).await.0, 200);
    assert_eq!(send(&app.state, ok("198.51.100.20")).await.0, 429);
    // IPv6 is keyed by /64: another address in the same /64 shares the bucket.
    assert_eq!(send(&app.state, ok("2001:db8:1:2::1")).await.0, 200);
    assert_eq!(send(&app.state, ok("2001:db8:1:2::2")).await.0, 200);
    assert_eq!(send(&app.state, ok("2001:db8:1:2::3")).await.0, 429);
    assert_eq!(send(&app.state, ok("2001:db8:1:3::1")).await.0, 200);
}

#[tokio::test]
async fn token_failure_budget_answers_an_identical_429_and_never_blocks_valid_tokens() {
    let db = db_or_skip!();
    let mut cfg = config();
    cfg.ip_token_failures_per_min = 3;
    let app = wire(db, &cfg);
    let s = seed(db).await;
    let revoked = seed(db).await;
    assert!(
        app::dev::revoke_key(&db.owner, &revoked.key_id)
            .await
            .unwrap()
    );
    let unknown = format!("awk_ZZZZZZZZZZZZ_{}", s.secret);
    let mismatched = format!("awk_{}_{}", s.key_id, revoked.secret);
    let failures = [
        "/ingress/v1/tradingview/malformed".to_owned(),
        ingress_path(&unknown),
        ingress_path(&revoked.token),
    ];
    for path in &failures {
        let (status, _, body) = send(&app.state, post_json(path, "{}")).await;
        assert_eq!((status, body.len()), (404, 0));
    }
    // Budget exhausted: every failure kind, including near-miss paths, gets the same 429.
    let mut answers = Vec::new();
    for path in failures
        .iter()
        .cloned()
        .chain([ingress_path(&mismatched), "/Ingress/v1/x".to_owned()])
    {
        let (status, headers, body) = send(&app.state, post_json(&path, "{}")).await;
        assert_eq!(
            status,
            429,
            "{}",
            path.replace(s.secret.as_str(), "<secret>")
        );
        answers.push((
            body,
            headers
                .get("retry-after")
                .map(|v| v.to_str().unwrap().to_owned()),
        ));
    }
    assert!(
        answers.windows(2).all(|w| w[0] == w[1]),
        "429s must be identical"
    );
    // A valid token from the same IP is never refused by the failure budget.
    for _ in 0..3 {
        let (status, _, _) = send(
            &app.state,
            post_json(&ingress_path(&s.token), instruction(&unique_message_id())),
        )
        .await;
        assert_eq!(status, 200);
    }
}

#[tokio::test]
async fn only_unknown_key_ids_are_cached() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let cache = &app.state.ingress.unknown_keys;
    let now = std::time::Instant::now;
    let s = seed(db).await;
    let post = |token: &str| post_json(&ingress_path(token), instruction(&unique_message_id()));

    // An unknown key_id is remembered.
    let ghost = format!("awk_ZZZZZZZZZZZZ_{}", s.secret);
    assert_eq!(send(&app.state, post(&ghost)).await.0, 404);
    assert!(cache.is_unknown("ZZZZZZZZZZZZ", now()));

    // A revoked key is never cached: un-revoking takes effect on the next request.
    assert!(app::dev::revoke_key(&db.owner, &s.key_id).await.unwrap());
    assert_eq!(send(&app.state, post(&s.token)).await.0, 404);
    assert!(!cache.is_unknown(&s.key_id, now()));
    let mut tx = db.owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(s.user_id.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE identity.source_key SET revoked_at = NULL WHERE key_id = $1")
        .bind(&s.key_id)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(send(&app.state, post(&s.token)).await.0, 200);

    // The cache is consulted before the database; creating a key invalidates its entry.
    cache.remember_unknown(&s.key_id, now());
    assert_eq!(send(&app.state, post(&s.token)).await.0, 404);
    cache.invalidate(&s.key_id);
    assert_eq!(send(&app.state, post(&s.token)).await.0, 200);
}

#[tokio::test]
async fn metrics_count_by_result_only() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    send(
        &app.state,
        post_json(&ingress_path(&s.token), instruction(&unique_message_id())),
    )
    .await;
    send(&app.state, post_json("/ingress/v1/tradingview/nope", "{}")).await;
    let (status, _, body) = send(
        &app.state,
        Request::get("/metrics").body(Body::empty()).unwrap(),
    )
    .await;
    let text = String::from_utf8(body).unwrap();
    assert_eq!(status, 200);
    assert!(text.contains(r#"result="accepted"} 1"#), "{text}");
    assert!(text.contains(r#"result="not_found"} 1"#), "{text}");
    assert!(!text.contains(&s.key_id));
    assert!(text.contains("allwin_outbox_dead_lettered "), "{text}");
}

/// Wall-clock latency depends on host load, so it is not a gate here; the composed-stack
/// p95 (Sentinela) is. Run with `cargo test -- --ignored` on an idle host.
#[tokio::test]
#[ignore = "performance check, not a gate: run on an idle host"]
async fn p95_latency_is_well_under_budget() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    let mut samples = Vec::new();
    for _ in 0..60 {
        let started = std::time::Instant::now();
        let (status, _, _) = send(
            &app.state,
            post_json(&ingress_path(&s.token), instruction(&unique_message_id())),
        )
        .await;
        samples.push(started.elapsed());
        assert_eq!(status, 200);
    }
    samples.sort();
    let p95 = samples[samples.len() * 95 / 100];
    eprintln!("ingress p95 in-process: {p95:?}");
    assert!(p95 < std::time::Duration::from_millis(300), "p95 {p95:?}");
}

/// Crivo L1 on 196a30a (Architect): a source whose name passes the identity rules at its edge
/// (60 characters, non-ASCII) is audited with that exact name, and its events are never
/// dead-lettered.
#[tokio::test]
async fn an_edge_case_valid_source_name_is_audited_and_never_dead_lettered() {
    let db = db_or_skip!();
    let app = wire(db, &config());
    let s = seed(db).await;
    let name = "é".repeat(60);
    let mut tx = db.owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(s.user_id.to_string())
        .execute(&mut *tx)
        .await
        .unwrap();
    let renamed = sqlx::query("UPDATE identity.source SET name = $1 WHERE user_id = $2")
        .bind(&name)
        .bind(s.user_id)
        .execute(&mut *tx)
        .await
        .unwrap();
    assert_eq!(renamed.rows_affected(), 1);
    tx.commit().await.unwrap();

    let (status, _, _) = send(
        &app.state,
        post_json(&ingress_path(&s.token), instruction(&unique_message_id())),
    )
    .await;
    assert_eq!(status, 200);
    drain(&app, db, s.user_id).await;

    let (dead, published): (i64, i64) = sqlx::query_as(
        "SELECT count(*) FILTER (WHERE dead_lettered_at IS NOT NULL),
                count(*) FILTER (WHERE published_at IS NOT NULL)
           FROM app.outbox WHERE user_id = $1",
    )
    .bind(s.user_id)
    .fetch_one(&db.worker)
    .await
    .unwrap();
    assert_eq!((dead, published), (0, 1));
    let audited: Vec<(String, Option<String>)> = as_user(db, s.user_id, |c| {
        Box::pin(async move {
            sqlx::query_as("SELECT code, source_name FROM audit.audit_entry")
                .fetch_all(c)
                .await
                .unwrap()
        })
    })
    .await;
    assert_eq!(audited, [("ingress.received".to_owned(), Some(name))]);
}
