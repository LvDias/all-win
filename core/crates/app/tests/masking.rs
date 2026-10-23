//! Core side of the masking gate (s1-webhook-ingest §4.3, M1–M8, M10): with every
//! target at `trace`, no part of a source secret reaches logs, spans, metrics,
//! responses or audit rows. Caddy and the composed stack are covered by QA E2E.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod support;

use std::io::Write;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::Request;
use support::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Vec<u8>>>);

impl Write for Capture {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Capture {
    type Writer = Capture;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

/// The full secret and every 16-character window of it (28 windows for 43 chars).
fn search_terms(secret: &str) -> Vec<String> {
    let mut terms: Vec<String> = (0..=secret.len() - 16)
        .map(|i| secret[i..i + 16].to_owned())
        .collect();
    terms.push(secret.to_owned());
    terms
}

fn assert_absent(haystack: &str, terms: &[String], what: &str) {
    for t in terms {
        assert!(
            !haystack.contains(t.as_str()),
            "secret window found in {what}"
        );
    }
}

#[tokio::test]
async fn no_secret_fragment_reaches_logs_spans_metrics_responses_or_audit() {
    let Some(db) = db().await else { return };
    let db = &db;
    let capture = Capture::default();
    app::telemetry::init_with(EnvFilter::new("trace"), capture.clone());

    let mut cfg = config();
    cfg.rate_per_key_per_min = 30;
    let app = wire(db, &cfg);
    let s = seed(db).await;
    let (k, sec) = (s.key_id.clone(), s.secret.clone());
    let terms = search_terms(&sec);
    let mut responses = String::new();
    let post = |uri: String, body: String, ct: &'static str| {
        let state = app.state.clone();
        async move {
            let req = Request::post(uri)
                .header("content-type", ct)
                .body(Body::from(body))
                .unwrap();
            send(&state, req).await
        }
    };
    let ok_body = instruction(&unique_message_id());
    let canonical = ingress_path(&s.token);

    // M1 accepted, M2 duplicate, M3 schema, M4 policy.
    for body in [
        ok_body.clone(),
        ok_body.clone(),
        ok_body
            .replace(r#""quantity":1"#, r#""quantity":0"#)
            .replace("tv-MNQ", "tv-MNQ-m3"),
        format!(
            r#"{{"schema_version":"1.0","message_id":"{}","sent_at":"2026-10-22T16:03:20Z","source":{{"type":"tradingview"}},
               "signal_type":"trigger","action":"modify","instrument":{{"symbol":"MNQ"}},"refs":{{"order_ref":"o1"}},"changes":{{"quantity":2}}}}"#,
            unique_message_id()
        ),
    ] {
        let (_, h, b) = post(canonical.clone(), body, "application/json").await;
        responses.push_str(&format!("{h:?}{}", String::from_utf8_lossy(&b)));
    }
    // M7 oversized / wrong content type.
    for (body, ct) in [
        ("x".repeat(20 * 1024), "application/json"),
        ("<a/>".to_owned(), "application/xml"),
    ] {
        let (_, h, b) = post(canonical.clone(), body, ct).await;
        responses.push_str(&format!("{h:?}{}", String::from_utf8_lossy(&b)));
    }
    // M6 near-misses and R2 paths.
    let mut one_encoded = sec.clone();
    let c = one_encoded.remove(7);
    one_encoded.insert_str(7, &format!("%{:02X}", c as u8));
    let near = [
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
        format!("/ingress/v1/tradingview/awk_{k}_{one_encoded}"),
        format!("/ingress/v1/tradingview/{sec}"),
        format!("/ingress/v1/tradingview?t=awk_{k}_{sec}"),
        format!("/%69ngress/v1/tradingview/awk_{k}_{sec}"),
        format!("/ingress/v1/tradingview/awk_{k}_{sec}/"),
        format!("/api/v1/awk_{k}_{sec}"),
        format!("/x/awk_{k}/{}/{}", &sec[..20], &sec[20..]),
        format!(
            "/pt-BR/{}.{}/{}?q={sec}",
            &sec[..10],
            &sec[10..20],
            &sec[20..]
        ),
    ];
    for p in near {
        let (status, h, b) = post(p, ok_body.clone(), "application/json").await;
        assert_eq!((status, b.len()), (404, 0));
        responses.push_str(&format!("{h:?}"));
    }
    // M6-m: GET on the canonical URL.
    let get = Request::get(canonical.clone()).body(Body::empty()).unwrap();
    assert_eq!(send(&app.state, get).await.0, 404);
    // M8: burst with the valid token until 429.
    let mut limited = false;
    for _ in 0..40 {
        let (status, h, b) = post(
            canonical.clone(),
            instruction(&unique_message_id()),
            "application/json",
        )
        .await;
        responses.push_str(&format!("{h:?}{}", String::from_utf8_lossy(&b)));
        limited |= status == 429;
    }
    assert!(limited);
    // M5: revoked.
    assert!(app::dev::revoke_key(&db.owner, &k).await.unwrap());
    let (status, _, b) = post(
        canonical.clone(),
        instruction(&unique_message_id()),
        "application/json",
    )
    .await;
    assert_eq!((status, b.len()), (404, 0));

    while app.dispatcher.dispatch_batch().await.unwrap().0 > 0 {}
    let (_, _, metrics) = send(
        &app.state,
        Request::get("/metrics").body(Body::empty()).unwrap(),
    )
    .await;
    let user = s.user_id;
    let audit: Vec<String> = as_user(db, user, |c| {
        Box::pin(async move {
            sqlx::query_scalar("SELECT row_to_json(a)::text FROM audit.audit_entry a")
                .fetch_all(c)
                .await
                .unwrap()
        })
    })
    .await;
    let logs = String::from_utf8(capture.0.lock().unwrap().clone()).unwrap();

    assert!(
        !audit.is_empty() && audit.iter().any(|r| r.contains(&k)),
        "audit carries key_id"
    );
    assert!(
        logs.contains(&format!(r#""key_id":"{k}""#)),
        "request log carries key_id"
    );
    assert!(logs.contains(r#""http.route":"/ingress/v1/tradingview/{token}""#));
    assert!(logs.contains(r#""http.route":"<unmatched-ingress>""#));
    assert!(logs.contains(r#""http.route":"<unmatched>""#));
    assert_absent(&logs, &terms, "logs");
    assert_absent(&responses, &terms, "responses");
    assert_absent(&String::from_utf8_lossy(&metrics), &terms, "metrics");
    assert_absent(&audit.join("\n"), &terms, "audit");
    // M6-o pieces of ≥ 8 characters are searched as well.
    for piece in [&sec[..10], &sec[10..20], &sec[20..]] {
        assert!(!logs.contains(piece), "split secret piece found in logs");
    }
}
