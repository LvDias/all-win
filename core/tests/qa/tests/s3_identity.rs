//! S3 passwordless identity, black-box (slice-plan S3 AC-1..17, HTTP contract 2.x).
//!
//! Drives `app::http::router` over a real PostgreSQL 16 with the identity routes mounted
//! (`IDENTITY_PEPPERS`). Codes are read from a fake SMTP server the Core delivers to
//! (`CODE_SENDER=dev_smtp`, `APP_ENV=test`): the plaintext code exists nowhere else.
//! Each test uses its own e-mail addresses and client IP so send limits never collide.
//! `TEST_DATABASE_ADMIN_URL` is required (`ALLWIN_REQUIRE_DB_TESTS=1` turns a skip into a failure).

#![cfg(test)]
#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use app::config::Config;
use app::{App, AppState};
use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Request, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tower::ServiceExt;

const ORIGIN: &str = "https://app.qa.test";
const PEPPER: &str = "AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA"; // 32 bytes, test only

// ---------------------------------------------------------------- database

const ROLES: [(&str, &str, &str); 4] = [
    ("allwin_owner", "owner-test-only", "NOBYPASSRLS"),
    ("allwin_app", "app-test-only", "NOBYPASSRLS"),
    ("allwin_worker", "worker-test-only", "BYPASSRLS"),
    ("allwin_maint", "maint-test-only", "BYPASSRLS"),
];

static DATABASE: OnceLock<Option<(String, String)>> = OnceLock::new();

fn admin_url() -> Option<String> {
    std::env::var("TEST_DATABASE_ADMIN_URL").ok()
}

fn url_for(admin: &str, user: &str, password: &str, db: &str) -> String {
    let rest = admin.split_once("://").map_or(admin, |(_, r)| r);
    let host = rest.rsplit_once('@').map_or(rest, |(_, h)| h);
    let host = host.split('/').next().unwrap_or(host);
    format!("postgres://{user}:{password}@{host}/{db}")
}

async fn create_database() -> Option<(String, String)> {
    let Some(admin) = admin_url() else {
        assert!(
            std::env::var("ALLWIN_REQUIRE_DB_TESTS").as_deref() != Ok("1"),
            "TEST_DATABASE_ADMIN_URL is required"
        );
        eprintln!("SKIPPED: TEST_DATABASE_ADMIN_URL not set");
        return None;
    };
    let mut conn = PgConnection::connect(&admin).await.expect("admin");
    for (role, password, rls) in ROLES {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = $1)")
                .bind(role)
                .fetch_one(&mut conn)
                .await
                .unwrap();
        let verb = if exists { "ALTER" } else { "CREATE" };
        let sql = format!("{verb} ROLE {role} WITH LOGIN PASSWORD '{password}' {rls}");
        if let Err(e) = conn.execute(sqlx::AssertSqlSafe(sql)).await {
            assert!(e.to_string().contains("already exists"), "{e}");
        }
    }
    let name = format!("allwin_qa_s3_{}", uuid::Uuid::now_v7().simple());
    conn.execute(sqlx::AssertSqlSafe(format!(
        "CREATE DATABASE {name} OWNER allwin_owner"
    )))
    .await
    .unwrap();
    let owner = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url_for(&admin, "allwin_owner", "owner-test-only", &name))
        .await
        .unwrap();
    app::MIGRATOR.run(&owner).await.expect("migrations");
    owner.close().await;
    Some((admin, name))
}

struct Pools {
    admin: PgPool,
    owner: PgPool,
    app: PgPool,
    worker: PgPool,
}

async fn pools() -> Option<Pools> {
    let (admin, name) = DATABASE
        .get_or_init(|| {
            std::thread::spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(create_database())
            })
            .join()
            .unwrap()
        })
        .as_ref()?;
    let pool = |user: &str, pw: &str, max: u32| {
        let url = url_for(admin, user, pw, name);
        async move {
            PgPoolOptions::new()
                .max_connections(max)
                .acquire_timeout(Duration::from_secs(120))
                .connect(&url)
                .await
                .unwrap()
        }
    };
    let admin_db = {
        let base = admin.split('?').next().unwrap_or(admin);
        let head = base.rsplit_once('/').map_or(base, |(h, _)| h);
        format!("{head}/{name}")
    };
    Some(Pools {
        admin: PgPoolOptions::new()
            .max_connections(2)
            .connect(&admin_db)
            .await
            .unwrap(),
        owner: pool("allwin_owner", "owner-test-only", 2).await,
        app: pool("allwin_app", "app-test-only", 8).await,
        worker: pool("allwin_worker", "worker-test-only", 2).await,
    })
}

// ---------------------------------------------------------------- fake SMTP

#[derive(Debug, Clone)]
struct Mail {
    to: String,
    subject: String,
    body: String,
}

#[derive(Clone, Default)]
struct Inbox(
    Arc<Mutex<Vec<Mail>>>,
    Arc<Mutex<std::collections::HashMap<String, usize>>>,
);

impl Inbox {
    fn to(&self, addr: &str) -> Vec<Mail> {
        self.0
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.to.eq_ignore_ascii_case(addr))
            .cloned()
            .collect()
    }

    /// Waits for the delivery worker to hand over the `n`-th mail to `addr` that carries a code.
    async fn nth_code(&self, addr: &str, n: usize) -> String {
        for _ in 0..100 {
            let codes: Vec<String> = self
                .to(addr)
                .iter()
                .filter_map(|m| find_code(&m.body))
                .collect();
            if let Some(c) = codes.get(n) {
                return c.clone();
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        panic!("no code mail #{n} for {addr}: {:?}", self.to(addr));
    }

    /// The next code mail to `addr` not yet handed to a test.
    async fn code(&self, addr: &str) -> String {
        let key = addr.to_ascii_lowercase();
        let n = *self.1.lock().unwrap().get(&key).unwrap_or(&0);
        let code = self.nth_code(addr, n).await;
        self.1.lock().unwrap().insert(key, n + 1);
        code
    }

    /// Code of the most recent code mail already delivered to `addr`.
    async fn latest_code(&self, addr: &str) -> String {
        for _ in 0..100 {
            if let Some(c) = self.to(addr).iter().rev().find_map(|m| find_code(&m.body)) {
                return c;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        panic!("no code mail for {addr}");
    }
}

/// `XXXX-XXXX` in the Crockford Base32 alphabet (no I, L, O, U).
fn find_code(body: &str) -> Option<String> {
    const ALPHA: &str = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let b: Vec<char> = body.chars().collect();
    (0..b.len().saturating_sub(8)).find_map(|i| {
        let w = &b[i..i + 9];
        let ok = w[4] == '-'
            && w.iter()
                .enumerate()
                .all(|(k, c)| k == 4 || ALPHA.contains(*c))
            && (i == 0 || !b[i - 1].is_ascii_alphanumeric())
            && b.get(i + 9).is_none_or(|c| !c.is_ascii_alphanumeric());
        ok.then(|| w.iter().collect())
    })
}

async fn fake_smtp() -> (SocketAddr, Inbox) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let inbox = Inbox::default();
    let sink = inbox.clone();
    tokio::spawn(async move {
        loop {
            let Ok((sock, _)) = listener.accept().await else {
                return;
            };
            let sink = sink.clone();
            tokio::spawn(async move {
                let (r, mut w) = sock.into_split();
                let mut r = BufReader::new(r);
                w.write_all(b"220 qa\r\n").await.ok();
                let (mut to, mut data, mut in_data) = (String::new(), String::new(), false);
                let mut line = String::new();
                loop {
                    line.clear();
                    if r.read_line(&mut line).await.unwrap_or(0) == 0 {
                        return;
                    }
                    if in_data {
                        if line == ".\r\n" {
                            in_data = false;
                            let (head, body) = data.split_once("\r\n\r\n").unwrap_or((&data, ""));
                            let subject = head
                                .lines()
                                .find_map(|l| l.strip_prefix("Subject: "))
                                .unwrap_or("")
                                .to_owned();
                            sink.0.lock().unwrap().push(Mail {
                                to: to.clone(),
                                subject,
                                body: body.to_owned(),
                            });
                            data.clear();
                            w.write_all(b"250 queued\r\n").await.ok();
                        } else {
                            data.push_str(&line);
                        }
                        continue;
                    }
                    let up = line.to_ascii_uppercase();
                    if let Some(rest) = up.strip_prefix("RCPT TO:") {
                        let start = line.len() - rest.len();
                        to = line[start..]
                            .trim()
                            .trim_matches(|c| c == '<' || c == '>')
                            .to_owned();
                        w.write_all(b"250 ok\r\n").await.ok();
                    } else if up.starts_with("DATA") {
                        in_data = true;
                        w.write_all(b"354 go\r\n").await.ok();
                    } else if up.starts_with("QUIT") {
                        w.write_all(b"221 bye\r\n").await.ok();
                        return;
                    } else {
                        w.write_all(b"250 ok\r\n").await.ok();
                    }
                }
            });
        }
    });
    (addr, inbox)
}

// ---------------------------------------------------------------- app + client

/// Send limits are one code per 60 s per identifier (policy §4). Tests that need several
/// codes for one identity clear the counters as the migration owner; `ac6` holds the write
/// side for its whole run so it never races with a reset.
static LIMITS: tokio::sync::RwLock<()> = tokio::sync::RwLock::const_new(());

async fn reset_limits(env: &Env) {
    let _g = LIMITS.read().await;
    sqlx::query("DELETE FROM identity.code_send_counter")
        .execute(&env.pools.owner)
        .await
        .unwrap();
}

/// A clock the test moves; sessions and holds in the Core read it, codes read the DB clock.
struct TestClock(Mutex<chrono::Duration>);

impl shared_kernel::Clock for TestClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now() + *self.0.lock().unwrap()
    }
}

impl TestClock {
    fn advance(&self, by: chrono::Duration) {
        let mut offset = self.0.lock().unwrap();
        *offset += by;
    }
}

struct Env {
    clock: Arc<TestClock>,
    state: AppState,
    inbox: Inbox,
    pools: Pools,
}

async fn env(q35: bool) -> Option<Env> {
    let pools = pools().await?;
    let (smtp, inbox) = fake_smtp().await;
    let q35 = q35.to_string();
    let config = Config::from_lookup(move |k| match k {
        "DATABASE_URL" => Some("postgres://unused".into()),
        "APP_ENV" => Some("test".into()),
        "APP_PUBLIC_ORIGIN" => Some(ORIGIN.into()),
        "IDENTITY_PEPPERS" => Some(format!("1:{PEPPER}")),
        "CODE_SENDER" => Some("dev_smtp".into()),
        "DEV_SMTP_ADDR" => Some(smtp.to_string()),
        "STEP_UP_Q35_PERSISTENCE_ACTIONS" => Some(q35.clone()),
        "INGRESS_RATE_PER_KEY_PER_MIN" => Some("1000".into()),
        "INGRESS_IP_FLOOD_PER_MIN" => Some("100000".into()),
        "INGRESS_IP_TOKEN_FAILURES_PER_MIN" => Some("100000".into()),
        _ => None,
    })
    .expect("config");
    let clock = Arc::new(TestClock(Mutex::new(chrono::Duration::zero())));
    let mut app = App::wire(
        &config,
        pools.app.clone(),
        pools.worker.clone(),
        clock.clone(),
    )
    .expect("wire");
    let identity = app.identity.take().expect("identity wired");
    identity.start().await.expect("identity start");
    // The outbox dispatcher delivers the notices and writes the audit entries.
    tokio::spawn(app.dispatcher.run());
    Some(Env {
        clock,
        state: app.state,
        inbox,
        pools,
    })
}

static NEXT: AtomicU32 = AtomicU32::new(1);

fn unique(prefix: &str) -> String {
    format!(
        "{prefix}{}x{}",
        NEXT.fetch_add(1, Ordering::Relaxed),
        &uuid::Uuid::now_v7().simple().to_string()[20..]
    )
}

struct Reply {
    status: u16,
    headers: axum::http::HeaderMap,
    body: Value,
}

impl Reply {
    fn code(&self) -> &str {
        self.body["error"]["code"].as_str().unwrap_or("")
    }

    fn cookie(&self, name: &str) -> Option<String> {
        self.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .find_map(|c| {
                let (n, rest) = c.split_once('=')?;
                (n == name).then(|| rest.split(';').next().unwrap_or("").to_owned())
            })
            .filter(|v| !v.is_empty())
    }
}

/// One browser: its own cookies and client address.
struct Client {
    state: AppState,
    owner: PgPool,
    ip: [u8; 4],
    session: Option<String>,
    device: Option<String>,
}

impl Client {
    fn new(env: &Env) -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        Self {
            state: env.state.clone(),
            owner: env.pools.owner.clone(),
            ip: [198, 51, (n >> 8) as u8, n as u8],
            session: None,
            device: None,
        }
    }

    async fn call(&mut self, method: &str, uri: &str, body: Option<Value>) -> Reply {
        let mut req = Request::builder().method(method).uri(uri);
        if method != "GET" {
            req = req.header(header::ORIGIN, ORIGIN);
        }
        let mut cookies = Vec::new();
        if let Some(s) = &self.session {
            cookies.push(format!("__Host-aw_session={s}"));
        }
        if let Some(d) = &self.device {
            cookies.push(format!("__Host-aw_device={d}"));
        }
        if !cookies.is_empty() {
            req = req.header(header::COOKIE, cookies.join("; "));
        }
        let req = match body {
            Some(b) => req
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(b.to_string())),
            None => req.body(Body::empty()),
        };
        let mut req = req.unwrap();
        req.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from((self.ip, 40000))));
        let resp = app::http::router(self.state.clone())
            .oneshot(req)
            .await
            .unwrap();
        let status = resp.status().as_u16();
        let headers = resp.headers().clone();
        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        let reply = Reply {
            status,
            headers,
            body,
        };
        if reply.headers.get_all(header::SET_COOKIE).iter().any(|v| {
            v.to_str()
                .is_ok_and(|c| c.starts_with("__Host-aw_session="))
        }) {
            self.session = reply.cookie("__Host-aw_session");
        }
        if reply
            .headers
            .get_all(header::SET_COOKIE)
            .iter()
            .any(|v| v.to_str().is_ok_and(|c| c.starts_with("__Host-aw_device=")))
        {
            self.device = reply.cookie("__Host-aw_device");
        }
        reply
    }

    async fn get(&mut self, uri: &str) -> Reply {
        self.call("GET", uri, None).await
    }

    async fn post(&mut self, uri: &str, body: Value) -> Reply {
        self.call("POST", uri, Some(body)).await
    }

    /// Request a login code, read it from the inbox, verify it. Returns the verify reply.
    async fn sign_in(&mut self, env: &Env, email: &str) -> Reply {
        reset_limits(env).await;
        let issued = self
            .post(
                "/api/v1/auth/login/challenges",
                json!({"email": email, "channel": "email"}),
            )
            .await;
        assert_eq!(issued.status, 202, "{:?}", issued.body);
        let code = env.inbox.code(email).await;
        self.post(
            "/api/v1/auth/login/verify",
            json!({
                "challenge_id": issued.body["challenge_id"],
                "code": code,
                "remember_me": false
            }),
        )
        .await
    }

    /// Step-up for `action` by e-mail; returns the `StepUpProof` to put in the action body.
    async fn step_up(
        &mut self,
        env: &Env,
        email: &str,
        action: &str,
        target: Option<&str>,
    ) -> Value {
        reset_limits(env).await;
        let mut body = json!({"action": action, "channel": "email"});
        if let Some(t) = target {
            body["target_id"] = json!(t);
        }
        let issued = self.post("/api/v1/auth/step-up", body).await;
        assert_eq!(issued.status, 202, "{:?}", issued.body);
        let code = env.inbox.code(email).await;
        json!({"challenge_id": issued.body["challenge_id"], "code": code})
    }
}

/// An active user inserted as the migration owner (the way the dev seed does).
async fn seed_user(env: &Env, email: &str, admin: bool) -> uuid::Uuid {
    let id = uuid::Uuid::now_v7();
    let mut tx = env.pools.owner.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(id.to_string())
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO identity.app_user (id, email, email_normalized, email_verified_at, locale, \
         time_zone, is_platform_admin) VALUES ($1, $2, $2, now(), 'en', 'UTC', $3)",
    )
    .bind(id)
    .bind(email)
    .bind(admin)
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    id
}

macro_rules! env_or_skip {
    ($q35:expr) => {
        match env($q35).await {
            Some(e) => e,
            None => return,
        }
    };
}

// ---------------------------------------------------------------- AC-4, AC-5

#[tokio::test(flavor = "multi_thread")]
async fn ac4_sign_in_by_email_code_then_sign_out() {
    let env = env_or_skip!(true);
    let email = format!("{}@qa.test", unique("login"));
    seed_user(&env, &email, false).await;
    let mut c = Client::new(&env);

    let issued = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": email, "channel": "email"}),
        )
        .await;
    assert_eq!(issued.status, 202, "{:?}", issued.body);
    assert_eq!(issued.body["resend_after_s"], 60);
    let code = env.inbox.code(&email).await;
    // AC-5 format: XXXX-XXXX; entry ignores case, spaces and hyphens.
    assert_eq!(code.len(), 9);
    let sloppy = format!(" {} ", code.replace('-', "").to_lowercase());

    let verified = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": issued.body["challenge_id"], "code": sloppy, "remember_me": false}),
        )
        .await;
    assert_eq!(verified.status, 200, "{:?}", verified.body);
    assert_eq!(verified.body["user"]["email"], email);
    assert_eq!(verified.body["new_device"], true);
    assert!(c.session.is_some() && c.device.is_some());
    assert_eq!(c.get("/api/v1/me").await.status, 200);

    // AC-5 single use: the same code never signs in a second browser.
    let mut other = Client::new(&env);
    let again = other
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": issued.body["challenge_id"], "code": code, "remember_me": false}),
        )
        .await;
    assert_eq!((again.status, again.code()), (401, "auth.code_invalid"));

    // AC-4 sign out ends the current session.
    assert_eq!(c.post("/api/v1/auth/logout", json!({})).await.status, 204);
    c.session = None;
    let me = c.get("/api/v1/me").await;
    assert_eq!(me.status, 401);
}

#[tokio::test(flavor = "multi_thread")]
async fn ac5_five_wrong_attempts_invalidate_the_code() {
    let env = env_or_skip!(true);
    let email = format!("{}@qa.test", unique("wrong"));
    seed_user(&env, &email, false).await;
    let mut c = Client::new(&env);
    let issued = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": email, "channel": "email"}),
        )
        .await;
    let code = env.inbox.code(&email).await;
    let wrong = if code.starts_with('0') {
        "1234-5678"
    } else {
        "0000-0000"
    };
    for _ in 0..5 {
        let r = c
            .post(
                "/api/v1/auth/login/verify",
                json!({"challenge_id": issued.body["challenge_id"], "code": wrong, "remember_me": false}),
            )
            .await;
        assert_eq!((r.status, r.code()), (401, "auth.code_invalid"));
    }
    let late = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": issued.body["challenge_id"], "code": code, "remember_me": false}),
        )
        .await;
    assert_eq!(
        (late.status, late.code()),
        (401, "auth.code_invalid"),
        "the right code after 5 wrong attempts must be refused"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn ac5_over_long_code_is_a_400_and_counts_no_attempt() {
    let env = env_or_skip!(true);
    let email = format!("{}@qa.test", unique("long"));
    seed_user(&env, &email, false).await;
    let mut c = Client::new(&env);
    let issued = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": email, "channel": "email"}),
        )
        .await;
    let code = env.inbox.code(&email).await;
    for _ in 0..6 {
        let r = c
            .post(
                "/api/v1/auth/login/verify",
                json!({"challenge_id": issued.body["challenge_id"], "code": "A".repeat(33), "remember_me": false}),
            )
            .await;
        assert_eq!((r.status, r.code()), (400, "common.invalid_request"));
    }
    let ok = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": issued.body["challenge_id"], "code": code, "remember_me": false}),
        )
        .await;
    assert_eq!(ok.status, 200, "{:?}", ok.body);
}

// ---------------------------------------------------------------- AC-7

#[tokio::test(flavor = "multi_thread")]
async fn ac7_unknown_email_looks_identical_and_receives_nothing() {
    let env = env_or_skip!(true);
    let known = format!("{}@qa.test", unique("known"));
    let unknown = format!("{}@qa.test", unique("ghost"));
    seed_user(&env, &known, false).await;
    let mut c = Client::new(&env);
    let a = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": known, "channel": "email"}),
        )
        .await;
    let b = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": unknown, "channel": "email"}),
        )
        .await;
    assert_eq!((a.status, b.status), (202, 202));
    let keys = |v: &Value| {
        let mut k: Vec<_> = v.as_object().unwrap().keys().cloned().collect();
        k.sort();
        k
    };
    assert_eq!(keys(&a.body), keys(&b.body));
    assert_eq!(a.body["resend_after_s"], b.body["resend_after_s"]);
    env.inbox.code(&known).await;
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(
        env.inbox.to(&unknown).is_empty(),
        "no code to an unregistered e-mail"
    );

    // Wrong code for the unknown challenge answers exactly like a wrong code for the known one.
    let wrong_unknown = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": b.body["challenge_id"], "code": "0000-0000", "remember_me": false}),
        )
        .await;
    let wrong_known = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": a.body["challenge_id"], "code": "0000-0000", "remember_me": false}),
        )
        .await;
    assert_eq!(wrong_unknown.status, wrong_known.status);
    assert_eq!(wrong_unknown.body, wrong_known.body);
    let forged = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": "x".repeat(40), "code": "0000-0000", "remember_me": false}),
        )
        .await;
    assert_eq!(forged.body, wrong_known.body);
}

// ---------------------------------------------------------------- AC-1..3

#[tokio::test(flavor = "multi_thread")]
async fn ac1_ac3_invitation_signup_creates_the_account_only_after_the_code() {
    let env = env_or_skip!(true);
    let admin_email = format!("{}@qa.test", unique("admin"));
    seed_user(&env, &admin_email, true).await;
    let mut admin = Client::new(&env);
    assert_eq!(admin.sign_in(&env, &admin_email).await.status, 200);

    let invitee = format!("{}@qa.test", unique("invitee"));
    let proof = admin
        .step_up(&env, &admin_email, "admin_invitation_create", None)
        .await;
    let created = admin
        .post(
            "/api/v1/admin/invitations",
            json!({"email": invitee, "locale": "en", "send_email": false, "step_up": proof}),
        )
        .await;
    assert_eq!(created.status, 201, "{:?}", created.body);
    let url = created.body["invite_url"].as_str().unwrap();
    let token = url.split("#t=").nth(1).expect("token in the fragment");

    let mut guest = Client::new(&env);
    let looked = guest
        .post("/api/v1/auth/invitations/lookup", json!({"token": token}))
        .await;
    assert_eq!(looked.status, 200, "{:?}", looked.body);
    assert_eq!(looked.body["email"], invitee);

    // AC-2: a failed confirmation leaves the invitation usable and creates no account.
    let started = guest
        .post("/api/v1/auth/signup/challenges", json!({"token": token}))
        .await;
    assert_eq!(started.status, 202, "{:?}", started.body);
    let bad = guest
        .post(
            "/api/v1/auth/signup/verify",
            json!({"challenge_id": started.body["challenge_id"], "code": "0000-0000",
                   "locale": "en", "time_zone": "UTC", "remember_me": false}),
        )
        .await;
    assert_eq!((bad.status, bad.code()), (401, "auth.code_invalid"));
    let n: i64 =
        sqlx::query_scalar("SELECT count(*) FROM identity.app_user WHERE email_normalized = $1")
            .bind(&invitee)
            .fetch_one(&env.pools.owner)
            .await
            .unwrap();
    assert_eq!(n, 0, "no account before the code is confirmed");

    let code = env.inbox.code(&invitee).await; // the invited address, never another
    let done = guest
        .post(
            "/api/v1/auth/signup/verify",
            json!({"challenge_id": started.body["challenge_id"], "code": code,
                   "locale": "pt-BR", "time_zone": "America/Sao_Paulo", "remember_me": false}),
        )
        .await;
    assert_eq!(done.status, 201, "{:?}", done.body);
    assert_eq!(done.body["user"]["email"], invitee);
    assert_eq!(guest.get("/api/v1/me").await.status, 200);

    // AC-1: the invitation is single-use.
    let mut late = Client::new(&env);
    let reuse = late
        .post("/api/v1/auth/signup/challenges", json!({"token": token}))
        .await;
    assert_eq!(reuse.status, 404, "{:?}", reuse.body);

    // AC-3: no password column or field anywhere in the account.
    let cols: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM information_schema.columns \
         WHERE table_schema = 'identity' AND column_name ILIKE '%password%'",
    )
    .fetch_one(&env.pools.owner)
    .await
    .unwrap();
    assert_eq!(cols, 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn ac1_no_invitation_no_signup() {
    let env = env_or_skip!(true);
    let mut c = Client::new(&env);
    let r = c
        .post(
            "/api/v1/auth/signup/challenges",
            json!({"token": "not-a-real-invitation-token"}),
        )
        .await;
    assert_eq!(r.status, 404, "{:?}", r.body);
}

// ---------------------------------------------------------------- AC-10

#[tokio::test(flavor = "multi_thread")]
async fn ac10_admin_invitation_needs_a_fresh_step_up_when_q35_is_on() {
    let env = env_or_skip!(true);
    let admin_email = format!("{}@qa.test", unique("admin"));
    seed_user(&env, &admin_email, true).await;
    let mut admin = Client::new(&env);
    assert_eq!(admin.sign_in(&env, &admin_email).await.status, 200);
    let invitee = format!("{}@qa.test", unique("noproof"));

    let denied = admin
        .post(
            "/api/v1/admin/invitations",
            json!({"email": invitee, "locale": "en", "send_email": false}),
        )
        .await;
    assert!(
        denied.status >= 400 && denied.status < 500,
        "{} {:?}",
        denied.status,
        denied.body
    );
    let n: i64 =
        sqlx::query_scalar("SELECT count(*) FROM identity.invitation WHERE email_normalized = $1")
            .bind(&invitee)
            .fetch_one(&env.pools.owner)
            .await
            .unwrap();
    assert_eq!(n, 0, "no invitation without the code");

    // The proof authorizes one action only: it cannot be reused.
    let proof = admin
        .step_up(&env, &admin_email, "admin_invitation_create", None)
        .await;
    let first = admin
        .post(
            "/api/v1/admin/invitations",
            json!({"email": invitee, "locale": "en", "send_email": false, "step_up": proof}),
        )
        .await;
    assert_eq!(first.status, 201, "{:?}", first.body);
    let second = admin
        .post(
            "/api/v1/admin/invitations",
            json!({"email": format!("{}@qa.test", unique("again")), "locale": "en",
                   "send_email": false, "step_up": proof}),
        )
        .await;
    assert!(
        second.status >= 400 && second.status < 500,
        "proof reused: {:?}",
        second.body
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn ac10_non_admin_cannot_request_an_admin_step_up() {
    let env = env_or_skip!(true);
    let email = format!("{}@qa.test", unique("plain"));
    seed_user(&env, &email, false).await;
    let mut c = Client::new(&env);
    assert_eq!(c.sign_in(&env, &email).await.status, 200);
    let r = c
        .post(
            "/api/v1/auth/step-up",
            json!({"action": "admin_invitation_create", "channel": "email"}),
        )
        .await;
    assert_eq!((r.status, r.code()), (403, "auth.forbidden"));
    tokio::time::sleep(Duration::from_millis(500)).await;
    // Only the sign-in code was mailed.
    let codes = env
        .inbox
        .to(&email)
        .iter()
        .filter(|m| find_code(&m.body).is_some())
        .count();
    assert_eq!(codes, 1, "no step-up code was sent");
}

#[tokio::test(flavor = "multi_thread")]
async fn ac10_whatsapp_is_unavailable_before_s3b() {
    let env = env_or_skip!(true);
    let email = format!("{}@qa.test", unique("wa"));
    seed_user(&env, &email, true).await;
    let mut c = Client::new(&env);
    assert_eq!(c.sign_in(&env, &email).await.status, 200);
    let r = c
        .post(
            "/api/v1/auth/step-up",
            json!({"action": "admin_invitation_create", "channel": "whatsapp"}),
        )
        .await;
    assert_eq!((r.status, r.code()), (422, "auth.channel_unavailable"));
}

// ---------------------------------------------------------------- helpers (batch 2)

async fn signed_in(env: &Env, prefix: &str, admin: bool) -> (Client, String) {
    let email = format!("{}@qa.test", unique(prefix));
    seed_user(env, &email, admin).await;
    let mut c = Client::new(env);
    let r = c.sign_in(env, &email).await;
    assert_eq!(r.status, 200, "{:?}", r.body);
    (c, email)
}

async fn create_source(c: &mut Client, env: &Env, email: &str, name: &str) -> Reply {
    let proof = c.step_up(env, email, "source_create", None).await;
    c.post(
        "/api/v1/sources",
        json!({"name": name, "source_type": "tradingview", "step_up": proof}),
    )
    .await
}

async fn create_connection(c: &mut Client, env: &Env, email: &str, name: &str) -> Reply {
    let proof = c.step_up(env, email, "connection_create", None).await;
    c.post(
        "/api/v1/connections",
        json!({"name": name, "step_up": proof}),
    )
    .await
}

// ---------------------------------------------------------------- AC-9, AC-17

#[tokio::test(flavor = "multi_thread")]
async fn ac9_new_device_notice_only_for_a_device_not_seen_before() {
    let env = env_or_skip!(true);
    let (mut c, email) = signed_in(&env, "nd", false).await;
    let notices = |env: &Env, email: &str| {
        env.inbox
            .to(email)
            .into_iter()
            .filter(|m| find_code(&m.body).is_none())
            .collect::<Vec<_>>()
    };
    for _ in 0..50 {
        if !notices(&env, &email).is_empty() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let first = notices(&env, &email);
    let outbox: Vec<(String, bool, i32, Option<String>)> = sqlx::query_as(
        "SELECT event_type, published_at IS NOT NULL, attempts, last_error FROM app.outbox \
         WHERE event_type LIKE '%notice%' AND payload::text LIKE '%' || $1 || '%'",
    )
    .bind(c.ip[3].to_string())
    .fetch_all(&env.pools.worker)
    .await
    .unwrap();
    assert_eq!(
        first.len(),
        1,
        "one new-device notice: {first:?}; outbox: {outbox:?}"
    );
    let ip = format!("{}.{}.{}.{}", c.ip[0], c.ip[1], c.ip[2], c.ip[3]);
    assert!(
        first[0].body.contains(&ip),
        "notice carries the IP: {}",
        first[0].body
    );

    // The same browser (device cookie kept) signs in again: not a new device, no new notice.
    c.session = None;
    let again = c.sign_in(&env, &email).await;
    assert_eq!(again.status, 200);
    assert_eq!(again.body["new_device"], false);
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert_eq!(
        notices(&env, &email).len(),
        1,
        "no second notice for a known device"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn ac9_signup_session_sends_no_new_device_notice() {
    let env = env_or_skip!(true);
    let admin_email = format!("{}@qa.test", unique("admin"));
    seed_user(&env, &admin_email, true).await;
    let mut admin = Client::new(&env);
    assert_eq!(admin.sign_in(&env, &admin_email).await.status, 200);
    let invitee = format!("{}@qa.test", unique("fresh"));
    let proof = admin
        .step_up(&env, &admin_email, "admin_invitation_create", None)
        .await;
    let created = admin
        .post(
            "/api/v1/admin/invitations",
            json!({"email": invitee, "locale": "en", "send_email": false, "step_up": proof}),
        )
        .await;
    let token = created.body["invite_url"]
        .as_str()
        .unwrap()
        .split("#t=")
        .nth(1)
        .unwrap()
        .to_owned();
    let mut guest = Client::new(&env);
    let started = guest
        .post("/api/v1/auth/signup/challenges", json!({"token": token}))
        .await;
    let code = env.inbox.code(&invitee).await;
    let done = guest
        .post(
            "/api/v1/auth/signup/verify",
            json!({"challenge_id": started.body["challenge_id"], "code": code,
                   "locale": "en", "time_zone": "UTC", "remember_me": false}),
        )
        .await;
    assert_eq!(done.status, 201, "{:?}", done.body);
    tokio::time::sleep(Duration::from_secs(2)).await;
    let others = env
        .inbox
        .to(&invitee)
        .into_iter()
        .filter(|m| find_code(&m.body).is_none())
        .count();
    assert_eq!(others, 0, "the sign-up session itself sends no notice");
}

#[tokio::test(flavor = "multi_thread")]
async fn ac17_sign_out_everywhere_ends_every_session_and_revokes_devices() {
    let env = env_or_skip!(true);
    let (mut a1, email) = signed_in(&env, "everywhere", false).await;
    let mut a2 = Client::new(&env);
    assert_eq!(a2.sign_in(&env, &email).await.status, 200);
    assert_eq!(a2.get("/api/v1/me").await.status, 200);
    let devices = a1.get("/api/v1/me/devices").await;
    assert_eq!(devices.status, 200);
    assert!(
        devices.body.as_array().is_some_and(|d| d.len() >= 2),
        "{:?}",
        devices.body
    );

    // No step-up needed.
    assert_eq!(
        a1.post("/api/v1/auth/logout-everywhere", json!({}))
            .await
            .status,
        204
    );
    a1.session = None;
    assert_eq!(a1.get("/api/v1/me").await.status, 401);
    assert_eq!(
        a2.get("/api/v1/me").await.status,
        401,
        "the other browser is signed out too"
    );

    // Devices were revoked: the next sign-in with the old device cookie is a new device.
    let again = a2.sign_in(&env, &email).await;
    assert_eq!(again.status, 200);
    assert_eq!(again.body["new_device"], true);
}

// ---------------------------------------------------------------- AC-6

#[tokio::test(flavor = "multi_thread")]
async fn ac6_send_limit_answers_429_with_retry_after_and_sends_nothing_more() {
    let env = env_or_skip!(true);
    let _exclusive = LIMITS.write().await;
    let known = format!("{}@qa.test", unique("limk"));
    let unknown = format!("{}@qa.test", unique("limu"));
    seed_user(&env, &known, false).await;
    let first_429 = |email: String| {
        let owner = env.pools.owner.clone();
        let env_state = env.state.clone();
        let inbox = env.inbox.clone();
        async move {
            let mut c = Client {
                state: env_state,
                owner,
                ip: [
                    192,
                    0,
                    2,
                    (NEXT.fetch_add(1, Ordering::Relaxed) % 250) as u8,
                ],
                session: None,
                device: None,
            };
            for n in 1..=60 {
                let r = c
                    .post(
                        "/api/v1/auth/login/challenges",
                        json!({"email": email, "channel": "email"}),
                    )
                    .await;
                if r.status == 429 {
                    assert!(r.headers.contains_key(header::RETRY_AFTER));
                    assert_eq!(r.code(), "auth.rate_limited");
                    return (n, inbox.to(&email).len());
                }
                assert_eq!(r.status, 202);
            }
            panic!("no send limit within 60 requests");
        }
    };
    let (n_known, _) = first_429(known.clone()).await;
    let (n_unknown, mails_unknown) = first_429(unknown.clone()).await;
    assert_eq!(
        n_known, n_unknown,
        "known and unknown identifiers share the same budget"
    );
    assert_eq!(mails_unknown, 0);
    tokio::time::sleep(Duration::from_secs(2)).await;
    let sent = env
        .inbox
        .to(&known)
        .iter()
        .filter(|m| find_code(&m.body).is_some())
        .count();
    assert!(
        sent < n_known as usize,
        "requests over the limit send nothing ({sent} of {n_known})"
    );
}

// ---------------------------------------------------------------- AC-13, AC-14

#[tokio::test(flavor = "multi_thread")]
async fn ac13_two_users_never_see_or_touch_each_others_sources_and_connections() {
    let env = env_or_skip!(true);
    let (mut a, a_mail) = signed_in(&env, "iso-a", false).await;
    let (mut b, b_mail) = signed_in(&env, "iso-b", false).await;

    let src = create_source(&mut a, &env, &a_mail, "A source").await;
    assert_eq!(src.status, 201, "{:?}", src.body);
    let src_id = src.body["source"]["id"].as_str().unwrap().to_owned();
    let conn = create_connection(&mut a, &env, &a_mail, "A box").await;
    assert_eq!(conn.status, 201, "{:?}", conn.body);
    let conn_id = conn.body["connection"]["id"].as_str().unwrap().to_owned();

    // B lists nothing of A's.
    let bs = b.get("/api/v1/sources").await;
    assert_eq!(bs.status, 200);
    assert!(!bs.body.to_string().contains(&src_id));
    let bc = b.get("/api/v1/connections").await;
    assert!(!bc.body.to_string().contains(&conn_id));

    // B cannot rotate, revoke, re-pair or unpair A's objects: 404, and nothing changes.
    for (path, body) in [
        (format!("/api/v1/sources/{src_id}/revoke"), json!({})),
        (
            format!("/api/v1/connections/{conn_id}/pairing-code"),
            json!({}),
        ),
        (
            format!("/api/v1/connections/{conn_id}/unpair"),
            json!({"acknowledge_open_positions": true}),
        ),
    ] {
        let r = b.post(&path, body).await;
        assert!(
            r.status == 404 || r.status == 403,
            "{path}: B got {} {:?}",
            r.status,
            r.body
        );
        assert_ne!(r.status, 200);
        assert_ne!(r.status, 204);
    }
    // A step-up aimed at A's source from B is refused and mails nothing.
    let before = env.inbox.to(&b_mail).len();
    let r = b
        .post(
            "/api/v1/auth/step-up",
            json!({"action": "source_rotate", "channel": "email", "target_id": src_id}),
        )
        .await;
    assert_eq!((r.status, r.code()), (404, "common.not_found"));
    tokio::time::sleep(Duration::from_millis(800)).await;
    assert_eq!(env.inbox.to(&b_mail).len(), before);

    let still = a.get("/api/v1/sources").await;
    let list = still.body.as_array().unwrap();
    let mine = list.iter().find(|s| s["id"] == src_id).unwrap();
    assert_eq!(mine["status"], "active");
}

#[tokio::test(flavor = "multi_thread")]
async fn ac14_source_secret_is_shown_once_stored_hashed_and_dead_after_revoke() {
    let env = env_or_skip!(true);
    let (mut a, mail) = signed_in(&env, "src", false).await;
    let created = create_source(&mut a, &env, &mail, "TV").await;
    assert_eq!(created.status, 201, "{:?}", created.body);
    let url = created.body["webhook_url"].as_str().unwrap().to_owned();
    let token = url.rsplit('/').next().unwrap().to_owned();
    let secret = token.splitn(3, '_').nth(2).unwrap().to_owned();
    let id = created.body["source"]["id"].as_str().unwrap().to_owned();
    assert!(
        !created.body["source"]["masked_url"]
            .as_str()
            .unwrap()
            .contains(&secret)
    );

    // Never returned again.
    let listed = a.get("/api/v1/sources").await;
    assert!(!listed.body.to_string().contains(&secret));

    // Not stored in any text column of the identity or audit schemas.
    let cols: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT table_schema::text, table_name::text, column_name::text \
         FROM information_schema.columns WHERE table_schema IN ('identity','audit') \
         AND data_type IN ('text','character varying','jsonb','json')",
    )
    .fetch_all(&env.pools.owner)
    .await
    .unwrap();
    for (s, t, col) in cols {
        let n: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "SELECT count(*) FROM {s}.{t} WHERE {col}::text LIKE '%' || $1 || '%'"
        )))
        .bind(&secret)
        .fetch_one(&env.pools.owner)
        .await
        .unwrap();
        assert_eq!(n, 0, "{s}.{t}.{col} holds the plaintext secret");
    }

    // The key works before the revoke and is rejected after it (S1 AC-2).
    let path = format!("/ingress/v1/tradingview/{token}");
    let body = r#"{"schema_version":"1.0","message_id":"qa-s3-ac14-1","sent_at":"2026-10-22T16:03:20Z",
        "source":{"type":"tradingview"},"signal_type":"trigger","action":"buy",
        "instrument":{"symbol":"MNQ"},"quantity":1}"#;
    let send = |state: AppState, path: String, body: String| async move {
        let mut req = Request::post(path)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap();
        req.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([203, 0, 113, 9], 40000))));
        app::http::router(state)
            .oneshot(req)
            .await
            .unwrap()
            .status()
            .as_u16()
    };
    let before = send(env.state.clone(), path.clone(), body.to_owned()).await;
    assert!(before < 300, "live key accepted, got {before}");
    let revoke = a
        .post(&format!("/api/v1/sources/{id}/revoke"), json!({}))
        .await;
    assert!(revoke.status < 300, "{:?}", revoke.body);
    let after = send(
        env.state.clone(),
        path,
        body.replace("qa-s3-ac14-1", "qa-s3-ac14-2"),
    )
    .await;
    assert!(after >= 400, "revoked key must be rejected, got {after}");
}

// ---------------------------------------------------------------- AC-15

#[tokio::test(flavor = "multi_thread")]
async fn ac15_pairing_is_per_instance_and_unpairing_one_leaves_the_others() {
    let env = env_or_skip!(true);
    let (mut a, mail) = signed_in(&env, "pair", false).await;
    let mut ids = Vec::new();
    for name in ["Desk", "Laptop"] {
        let conn = create_connection(&mut a, &env, &mail, name).await;
        assert_eq!(conn.status, 201, "{:?}", conn.body);
        let id = conn.body["connection"]["id"].as_str().unwrap().to_owned();
        let code = conn.body["pairing"]["pairing_code"]
            .as_str()
            .unwrap()
            .to_owned();
        let mut plugin = Client::new(&env);
        let paired = plugin
            .post(
                "/adapter/v1/pairings",
                json!({"pairing_code": code, "instance_id": uuid::Uuid::now_v7().to_string(),
                       "instance_name": name,
                       "adapter": {"type": "ninjatrader8", "version": "1.0.0", "build": "development"}}),
            )
            .await;
        assert_eq!(paired.status, 201, "{:?}", paired.body);
        assert_eq!(paired.body["connection_id"], id);
        assert!(
            paired.body["device_token"]
                .as_str()
                .is_some_and(|t| !t.is_empty())
        );
        ids.push(id);
    }
    // A pairing code is single use.
    let reuse = create_connection(&mut a, &env, &mail, "Third").await;
    let code = reuse.body["pairing"]["pairing_code"]
        .as_str()
        .unwrap()
        .to_owned();
    let mut p1 = Client::new(&env);
    let pair = |code: String, inst: &str| {
        json!({"pairing_code": code, "instance_id": inst, "instance_name": inst,
               "adapter": {"type": "ninjatrader8", "version": "1.0.0", "build": "development"}})
    };
    assert_eq!(
        p1.post(
            "/adapter/v1/pairings",
            pair(code.clone(), &uuid::Uuid::now_v7().to_string())
        )
        .await
        .status,
        201
    );
    let second = Client::new(&env)
        .post(
            "/adapter/v1/pairings",
            pair(code, &uuid::Uuid::now_v7().to_string()),
        )
        .await;
    assert!(second.status >= 400, "a used code must not pair again");

    // Unpair the first one: needs the code; the other stays paired.
    let proof = a
        .step_up(&env, &mail, "connection_unpair", Some(&ids[0]))
        .await;
    let un = a
        .post(
            &format!("/api/v1/connections/{}/unpair", ids[0]),
            json!({"acknowledge_open_positions": false, "step_up": proof}),
        )
        .await;
    assert_eq!(un.status, 204, "{:?}", un.body);
    let list = a.get("/api/v1/connections").await;
    let status = |id: &str| {
        list.body
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == id)
            .map(|c| c["status"].as_str().unwrap().to_owned())
            .unwrap_or_default()
    };
    // An unpaired connection is either listed as `unpaired` or no longer listed.
    let listed = |id: &str| list.body.as_array().unwrap().iter().any(|c| c["id"] == id);
    assert!(!listed(&ids[0]) || status(&ids[0]) == "unpaired");
    assert!(listed(&ids[1]) && status(&ids[1]) != "unpaired");

    // Unpair without a code is refused.
    let no_code = a
        .post(
            &format!("/api/v1/connections/{}/unpair", ids[1]),
            json!({"acknowledge_open_positions": false}),
        )
        .await;
    assert!(
        no_code.status >= 400 && no_code.status < 500,
        "{:?}",
        no_code.body
    );
}

// ---------------------------------------------------------------- AC-16

#[tokio::test(flavor = "multi_thread")]
async fn ac16_no_code_and_no_plain_channel_value_in_any_stored_text() {
    let env = env_or_skip!(true);
    let email = format!("{}@qa.test", unique("audit"));
    seed_user(&env, &email, false).await;
    let mut c = Client::new(&env);
    let issued = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": email, "channel": "email"}),
        )
        .await;
    let code = env.inbox.code(&email).await;
    // One failed and one successful attempt.
    c.post(
        "/api/v1/auth/login/verify",
        json!({"challenge_id": issued.body["challenge_id"], "code": "0000-0000", "remember_me": false}),
    )
    .await;
    let ok = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": issued.body["challenge_id"], "code": code, "remember_me": false}),
        )
        .await;
    assert_eq!(ok.status, 200);
    tokio::time::sleep(Duration::from_secs(1)).await;
    let plain = code.replace('-', "");
    for needle in [code.as_str(), plain.as_str()] {
        let cols: Vec<(String, String, String)> = sqlx::query_as(
            "SELECT table_schema::text, table_name::text, column_name::text \
             FROM information_schema.columns WHERE table_schema IN ('identity','audit','public') \
             AND data_type IN ('text','character varying','jsonb','json')",
        )
        .fetch_all(&env.pools.owner)
        .await
        .unwrap();
        for (s, t, col) in cols {
            let n: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
                "SELECT count(*) FROM {s}.{t} WHERE {col}::text LIKE '%' || $1 || '%'"
            )))
            .bind(needle)
            .fetch_one(&env.pools.owner)
            .await
            .unwrap();
            assert_eq!(n, 0, "{s}.{t}.{col} holds the one-time code");
        }
    }
    // No audit row holds the unmasked e-mail.
    let n: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM audit.audit_entry WHERE audit_entry::text LIKE '%' || $1 || '%'",
    )
    .bind(&email)
    .fetch_one(&env.pools.owner)
    .await
    .unwrap();
    assert_eq!(n, 0, "audit rows must not carry the unmasked channel value");
}

// ---------------------------------------------------------------- AC-8 (test clock)

async fn sign_in_remember(c: &mut Client, env: &Env, email: &str, remember: bool) {
    reset_limits(env).await;
    let issued = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": email, "channel": "email"}),
        )
        .await;
    assert_eq!(issued.status, 202, "{:?}", issued.body);
    let code = env.inbox.code(email).await;
    let r = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": issued.body["challenge_id"], "code": code, "remember_me": remember}),
        )
        .await;
    assert_eq!(r.status, 200, "{:?}", r.body);
}

async fn user_with_clock(env: &Env, prefix: &str) -> (Client, String) {
    let email = format!("{}@qa.test", unique(prefix));
    seed_user(env, &email, false).await;
    (Client::new(env), email)
}

#[tokio::test(flavor = "multi_thread")]
async fn ac8_session_expires_after_12h_idle() {
    let env = env_or_skip!(true);
    let (mut c, email) = user_with_clock(&env, "idle").await;
    sign_in_remember(&mut c, &env, &email, false).await;
    env.clock
        .advance(chrono::Duration::hours(11) + chrono::Duration::minutes(59));
    assert_eq!(
        c.get("/api/v1/me").await.status,
        200,
        "still inside 12 h idle"
    );
    // Activity extended the idle window; 12 h 1 min of silence ends it.
    env.clock
        .advance(chrono::Duration::hours(12) + chrono::Duration::minutes(1));
    let late = c.get("/api/v1/me").await;
    assert_eq!(late.status, 401, "{:?}", late.body);
}

#[tokio::test(flavor = "multi_thread")]
async fn ac8_session_ends_at_24h_absolute_even_when_active() {
    let env = env_or_skip!(true);
    let (mut c, email) = user_with_clock(&env, "abs").await;
    sign_in_remember(&mut c, &env, &email, false).await;
    for _ in 0..2 {
        env.clock.advance(chrono::Duration::hours(11));
        assert_eq!(c.get("/api/v1/me").await.status, 200);
    }
    // t = 22 h; the next touch is past 24 h absolute although idle is fresh.
    env.clock.advance(chrono::Duration::hours(3));
    assert_eq!(c.get("/api/v1/me").await.status, 401);
}

#[tokio::test(flavor = "multi_thread")]
async fn ac8_keep_me_signed_in_is_7_days_idle_and_30_days_absolute() {
    let env = env_or_skip!(true);
    let (mut idle, e1) = user_with_clock(&env, "rem-i").await;
    sign_in_remember(&mut idle, &env, &e1, true).await;
    env.clock.advance(chrono::Duration::days(6));
    assert_eq!(idle.get("/api/v1/me").await.status, 200, "inside 7 d idle");
    env.clock
        .advance(chrono::Duration::days(7) + chrono::Duration::minutes(1));
    assert_eq!(idle.get("/api/v1/me").await.status, 401, "past 7 d idle");

    let (mut abs, e2) = user_with_clock(&env, "rem-a").await;
    sign_in_remember(&mut abs, &env, &e2, true).await;
    for _ in 0..4 {
        env.clock.advance(chrono::Duration::days(6));
        assert_eq!(abs.get("/api/v1/me").await.status, 200);
    }
    // t = 24 d; the next touch lands at 31 d, past the 30 d absolute limit.
    env.clock.advance(chrono::Duration::days(7));
    assert_eq!(abs.get("/api/v1/me").await.status, 401);
}

#[tokio::test(flavor = "multi_thread")]
async fn ac8_after_expiry_the_user_can_sign_in_again() {
    let env = env_or_skip!(true);
    let (mut c, email) = user_with_clock(&env, "relog").await;
    sign_in_remember(&mut c, &env, &email, false).await;
    env.clock.advance(chrono::Duration::hours(25));
    assert_eq!(c.get("/api/v1/me").await.status, 401);
    c.session = None;
    sign_in_remember(&mut c, &env, &email, false).await;
    assert_eq!(
        c.get("/api/v1/me").await.status,
        200,
        "codes still work after the jump"
    );
}

// ---------------------------------------------------------------- AC-11 channel change

async fn start_change(c: &mut Client, new_value: &str) -> Reply {
    c.post(
        "/api/v1/me/channel-changes",
        json!({"channel": "email", "new_value": new_value}),
    )
    .await
}

/// Confirms every code of a started change (old and new address), returns the last reply.
async fn confirm_change(c: &mut Client, env: &Env, started: &Reply, old: &str, new: &str) -> Reply {
    let id = started.body["id"].as_str().unwrap().to_owned();
    let mut last = None;
    for (role, addr) in [("current_value", old), ("new_value", new)] {
        let challenge = started.body["codes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|k| k["role"] == role)
            .unwrap()["challenge_id"]
            .clone();
        let code = env.inbox.code(addr).await;
        last = Some(
            c.post(
                &format!("/api/v1/me/channel-changes/{id}/verify"),
                json!({"challenge_id": challenge, "code": code}),
            )
            .await,
        );
    }
    last.unwrap()
}

async fn age_device(env: &Env, email: &str) {
    sqlx::query(
        "UPDATE identity.known_device SET first_seen_at = now() - interval '8 days' \
         WHERE user_id = (SELECT id FROM identity.app_user WHERE email_normalized = $1)",
    )
    .bind(email)
    .execute(&env.pools.admin)
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn ac11_change_needs_a_code_on_the_old_and_on_the_new_address() {
    let env = env_or_skip!(true);
    let (mut c, old) = signed_in(&env, "chg-old", false).await;
    age_device(&env, &old).await;
    let new = format!("{}@qa.test", unique("chg-new"));
    reset_limits(&env).await;
    let started = start_change(&mut c, &new).await;
    assert_eq!(started.status, 202, "{:?}", started.body);
    assert_eq!(started.body["status"], "awaiting_codes");
    // One code goes to the current address and one to the new one.
    env.inbox.nth_code(&old, 1).await;
    env.inbox.nth_code(&new, 0).await;
    let masked = started.body["masked_new_value"].as_str().unwrap();
    assert!(!masked.contains(&new), "masked: {masked}");

    // Only the old code: nothing replaced yet.
    let id = started.body["id"].as_str().unwrap().to_owned();
    let challenge_of = |role: &str| {
        started.body["codes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|k| k["role"] == role)
            .unwrap()["challenge_id"]
            .clone()
    };
    let old_code = env.inbox.code(&old).await; // the sign-in code, already consumed
    let _ = old_code;
    let fresh_old = env.inbox.latest_code(&old).await;
    let half = c
        .post(
            &format!("/api/v1/me/channel-changes/{id}/verify"),
            json!({"challenge_id": challenge_of("current_value"), "code": fresh_old}),
        )
        .await;
    assert_eq!(half.status, 200, "{:?}", half.body);
    assert_eq!(half.body["status"], "awaiting_codes");
    assert_eq!(c.get("/api/v1/me").await.body["email"], old);

    let new_code = env.inbox.code(&new).await;
    let done = c
        .post(
            &format!("/api/v1/me/channel-changes/{id}/verify"),
            json!({"challenge_id": challenge_of("new_value"), "code": new_code}),
        )
        .await;
    assert_eq!(done.status, 200, "{:?}", done.body);
    assert_eq!(done.body["status"], "completed");
    assert_eq!(c.get("/api/v1/me").await.body["email"], new);
}

#[tokio::test(flavor = "multi_thread")]
async fn ac11_completed_change_ends_other_sessions_and_freezes_changes_for_24h() {
    let env = env_or_skip!(true);
    let (mut c, old) = signed_in(&env, "frz", false).await;
    let mut other = Client::new(&env);
    assert_eq!(other.sign_in(&env, &old).await.status, 200);
    age_device(&env, &old).await;
    let new = format!("{}@qa.test", unique("frz-new"));
    reset_limits(&env).await;
    let started = start_change(&mut c, &new).await;
    assert_eq!(started.status, 202, "{:?}", started.body);
    let done = confirm_change(&mut c, &env, &started, &old, &new).await;
    assert_eq!(done.body["status"], "completed", "{:?}", done.body);

    assert_eq!(
        other.get("/api/v1/me").await.status,
        401,
        "other sessions end"
    );
    assert_eq!(
        c.get("/api/v1/me").await.status,
        200,
        "the session that changed it stays"
    );
    reset_limits(&env).await;
    let again = start_change(&mut c, &format!("{}@qa.test", unique("frz-2"))).await;
    assert_eq!(
        (again.status, again.code()),
        (409, "me.channel_change_too_soon")
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn ac11_m2_new_device_holds_the_change_72h_and_cancel_keeps_the_old_value() {
    let env = env_or_skip!(true);
    let (mut c, old) = signed_in(&env, "hold", false).await; // device first seen just now
    let new = format!("{}@qa.test", unique("hold-new"));
    reset_limits(&env).await;
    let started = start_change(&mut c, &new).await;
    assert_eq!(started.status, 202, "{:?}", started.body);
    let done = confirm_change(&mut c, &env, &started, &old, &new).await;
    assert_eq!(done.status, 200, "{:?}", done.body);
    assert_eq!(done.body["status"], "on_hold", "{:?}", done.body);
    let hold = done.body["hold_until"].as_str().expect("hold_until");
    let until = chrono::DateTime::parse_from_rfc3339(hold).unwrap();
    let hours =
        (until.with_timezone(&chrono::Utc) - shared_kernel::Clock::now(&*env.clock)).num_hours();
    assert!((71..=72).contains(&hours), "hold is 72 h, got {hours} h");
    assert_eq!(
        c.get("/api/v1/me").await.body["email"],
        old,
        "not replaced during the hold"
    );

    // "Cancelar troca" from any session; signing in again does not cancel.
    let id = started.body["id"].as_str().unwrap();
    let cancel = c
        .post(
            &format!("/api/v1/me/channel-changes/{id}/cancel"),
            json!({}),
        )
        .await;
    assert_eq!(cancel.status, 204, "{:?}", cancel.body);
    assert_eq!(c.get("/api/v1/me").await.body["email"], old);
}

#[tokio::test(flavor = "multi_thread")]
async fn ac11_channel_value_of_another_account_is_refused_without_saying_who_owns_it() {
    let env = env_or_skip!(true);
    let (mut c, old) = signed_in(&env, "clash", false).await;
    age_device(&env, &old).await;
    let taken = format!("{}@qa.test", unique("clash-owner"));
    seed_user(&env, &taken, false).await;
    let free = format!("{}@qa.test", unique("clash-free"));
    reset_limits(&env).await;
    let a = start_change(&mut c, &taken).await;
    let mut c2 = {
        let (c2, _) = signed_in(&env, "clash2", false).await;
        c2
    };
    let _ = &mut c2;
    // Never "this e-mail belongs to someone else": the clash is not announced at start.
    assert_ne!(a.code(), "me.channel_taken");
    assert!(
        !a.body.to_string().to_lowercase().contains("belong")
            && !a.body.to_string().contains(&taken),
        "{:?}",
        a.body
    );
    let _ = free;
    tokio::time::sleep(Duration::from_secs(1)).await;
    // The owner of that address receives no code for somebody else's change.
    let codes = env
        .inbox
        .to(&taken)
        .iter()
        .filter(|m| find_code(&m.body).is_some())
        .count();
    assert_eq!(
        codes, 0,
        "no code is sent to an address owned by another account"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn ac2_ac11_a_code_is_dead_after_10_minutes() {
    let env = env_or_skip!(true);
    let (mut c, old) = signed_in(&env, "ttl", false).await;
    age_device(&env, &old).await;
    let new = format!("{}@qa.test", unique("ttl-new"));
    reset_limits(&env).await;
    let started = start_change(&mut c, &new).await;
    assert_eq!(started.status, 202, "{:?}", started.body);
    // The DB clock decides code expiry: age this change and its codes by 12 minutes.
    sqlx::query(
        "UPDATE identity.channel_change SET created_at = now() - interval '12 minutes', \
         expires_at = now() - interval '2 minutes' WHERE new_value = $1",
    )
    .bind(&new)
    .execute(&env.pools.admin)
    .await
    .unwrap();
    sqlx::query(
        "UPDATE identity.one_time_code SET created_at = now() - interval '12 minutes', \
         sent_at = now() - interval '12 minutes', expires_at = now() - interval '2 minutes' \
         WHERE destination IN ($1, $2) AND purpose = 'channel_change'",
    )
    .bind(&old)
    .bind(&new)
    .execute(&env.pools.admin)
    .await
    .unwrap();
    let id = started.body["id"].as_str().unwrap();
    let challenge = started.body["codes"][0]["challenge_id"].clone();
    let code = env.inbox.latest_code(&old).await;
    let late = c
        .post(
            &format!("/api/v1/me/channel-changes/{id}/verify"),
            json!({"challenge_id": challenge, "code": code}),
        )
        .await;
    assert!(
        (late.status == 409 && late.code() == "me.channel_change_closed")
            || (late.status == 401 && late.code() == "auth.code_invalid"),
        "{} {:?}",
        late.status,
        late.body
    );
    assert_eq!(c.get("/api/v1/me").await.body["email"], old);
}

#[tokio::test(flavor = "multi_thread")]
async fn ac2_login_code_is_dead_after_10_minutes() {
    let env = env_or_skip!(true);
    let email = format!("{}@qa.test", unique("exp"));
    seed_user(&env, &email, false).await;
    let mut c = Client::new(&env);
    let issued = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": email, "channel": "email"}),
        )
        .await;
    let code = env.inbox.code(&email).await;
    sqlx::query(
        "UPDATE identity.one_time_code SET created_at = now() - interval '12 minutes', \
         sent_at = now() - interval '12 minutes', expires_at = now() - interval '2 minutes' \
         WHERE destination = $1",
    )
    .bind(&email)
    .execute(&env.pools.admin)
    .await
    .unwrap();
    let late = c
        .post(
            "/api/v1/auth/login/verify",
            json!({"challenge_id": issued.body["challenge_id"], "code": code, "remember_me": false}),
        )
        .await;
    assert_eq!((late.status, late.code()), (401, "auth.code_invalid"));
}

// ---------------------------------------------------------------- M2: send limits under concurrency

/// Fires `n` login-code requests for `email` at once, each from its own client address.
async fn burst(env: &Env, email: &str, n: usize) -> Vec<u16> {
    let mut set = tokio::task::JoinSet::new();
    for _ in 0..n {
        let mut c = Client::new(env);
        let email = email.to_owned();
        set.spawn(async move {
            c.post(
                "/api/v1/auth/login/challenges",
                json!({"email": email, "channel": "email"}),
            )
            .await
            .status
        });
    }
    let mut statuses = Vec::new();
    while let Some(r) = set.join_next().await {
        statuses.push(r.unwrap());
    }
    statuses
}

async fn code_mails(env: &Env, email: &str) -> usize {
    tokio::time::sleep(Duration::from_secs(2)).await;
    env.inbox
        .to(email)
        .iter()
        .filter(|m| find_code(&m.body).is_some())
        .count()
}

async fn clear_windows(env: &Env, seconds: &[i32]) {
    sqlx::query("DELETE FROM identity.code_send_counter WHERE window_seconds = ANY($1)")
        .bind(seconds)
        .execute(&env.pools.admin)
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn m2_parallel_requests_for_one_identifier_send_at_most_one_code_per_60s() {
    let env = env_or_skip!(true);
    let _exclusive = LIMITS.write().await;
    let email = format!("{}@qa.test", unique("race60"));
    seed_user(&env, &email, false).await;
    let statuses = burst(&env, &email, 24).await;
    assert!(
        statuses.iter().all(|s| *s == 202 || *s == 429),
        "{statuses:?}"
    );
    let sent = code_mails(&env, &email).await;
    assert_eq!(
        sent, 1,
        "24 parallel requests inside one 60 s window: {sent} codes sent"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn m2_parallel_requests_never_exceed_5_codes_per_15_minutes() {
    let env = env_or_skip!(true);
    let _exclusive = LIMITS.write().await;
    let email = format!("{}@qa.test", unique("race900"));
    seed_user(&env, &email, false).await;
    // Four sends spread over distinct 60 s windows (only the 60 s counters are cleared).
    for _ in 0..4 {
        clear_windows(&env, &[60]).await;
        assert_eq!(burst(&env, &email, 1).await, vec![202]);
    }
    clear_windows(&env, &[60]).await;
    burst(&env, &email, 20).await;
    let sent = code_mails(&env, &email).await;
    assert!(sent <= 5, "{sent} codes inside 15 minutes (limit 5)");
    assert_eq!(sent, 5, "the fifth send is still allowed");
}

#[tokio::test(flavor = "multi_thread")]
async fn m2_parallel_requests_never_exceed_20_codes_per_24_hours() {
    let env = env_or_skip!(true);
    let _exclusive = LIMITS.write().await;
    let email = format!("{}@qa.test", unique("race86400"));
    seed_user(&env, &email, false).await;
    for _ in 0..19 {
        clear_windows(&env, &[60, 900]).await;
        assert_eq!(burst(&env, &email, 1).await, vec![202]);
    }
    clear_windows(&env, &[60, 900]).await;
    burst(&env, &email, 20).await;
    let sent = code_mails(&env, &email).await;
    assert_eq!(
        sent, 20,
        "{sent} codes inside 24 h (limit 20, the 20th allowed)"
    );
}

// ---------------------------------------------------------------- M1: rotation after a channel change

#[tokio::test(flavor = "multi_thread")]
async fn m1_channel_change_does_not_end_the_session_it_rotated_into() {
    let env = env_or_skip!(true);
    let (mut c, old) = signed_in(&env, "m1", false).await;
    age_device(&env, &old).await;
    let new = format!("{}@qa.test", unique("m1-new"));
    reset_limits(&env).await;
    let started = start_change(&mut c, &new).await;
    assert_eq!(started.status, 202, "{:?}", started.body);
    let done = confirm_change(&mut c, &env, &started, &old, &new).await;
    assert_eq!(done.body["status"], "completed", "{:?}", done.body);
    assert_eq!(
        c.get("/api/v1/me").await.status,
        200,
        "the rotated session works"
    );

    // The `sessions_ended` event for the other sessions must spare the session the caller now holds.
    let live: uuid::Uuid = sqlx::query_scalar(
        "SELECT s.id FROM identity.session s JOIN identity.app_user u ON u.id = s.user_id \
         WHERE u.email_normalized = $1 AND s.ended_at IS NULL",
    )
    .bind(&new)
    .fetch_one(&env.pools.admin)
    .await
    .unwrap();
    let payloads: Vec<Value> = sqlx::query_scalar(
        "SELECT o.payload FROM app.outbox o JOIN identity.app_user u ON u.id = o.user_id \
         WHERE u.email_normalized = $1 AND o.event_type = 'identity.sessions_ended' \
         AND o.payload->>'reason' = 'channel_changed'",
    )
    .bind(&new)
    .fetch_all(&env.pools.admin)
    .await
    .unwrap();
    assert_eq!(payloads.len(), 1, "{payloads:?}");
    let spared: Vec<String> = payloads[0]["all_except"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str().map(str::to_owned))
        .collect();
    assert!(
        spared.contains(&live.to_string()),
        "session.ended would also close the caller's own rotated session {live}: spared {spared:?}"
    );
}

// ---------------------------------------------------------------- code attempts under concurrency

async fn verify_all(env: &Env, challenge: &Value, codes: Vec<String>) -> Vec<u16> {
    let mut set = tokio::task::JoinSet::new();
    for code in codes {
        let mut c = Client::new(env);
        let challenge = challenge.clone();
        set.spawn(async move {
            c.post(
                "/api/v1/auth/login/verify",
                json!({"challenge_id": challenge, "code": code, "remember_me": false}),
            )
            .await
            .status
        });
    }
    let mut out = Vec::new();
    while let Some(r) = set.join_next().await {
        out.push(r.unwrap());
    }
    out
}

fn wrong_for(code: &str) -> String {
    if code.starts_with('0') {
        "1111-1111".into()
    } else {
        "0000-0000".into()
    }
}

async fn issued_login(env: &Env, prefix: &str) -> (Value, String, String) {
    let email = format!("{}@qa.test", unique(prefix));
    seed_user(env, &email, false).await;
    let mut c = Client::new(env);
    let issued = c
        .post(
            "/api/v1/auth/login/challenges",
            json!({"email": email, "channel": "email"}),
        )
        .await;
    assert_eq!(issued.status, 202);
    let code = env.inbox.code(&email).await;
    (issued.body["challenge_id"].clone(), code, email)
}

async fn live_sessions(env: &Env, email: &str) -> i64 {
    sqlx::query_scalar(
        "SELECT count(*) FROM identity.session s JOIN identity.app_user u ON u.id = s.user_id \
         WHERE u.email_normalized = $1 AND s.ended_at IS NULL",
    )
    .bind(email)
    .fetch_one(&env.pools.admin)
    .await
    .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn concurrent_wrong_guesses_at_four_attempts_end_at_five() {
    let env = env_or_skip!(true);
    let (challenge, code, email) = issued_login(&env, "cwrong").await;
    let wrong = wrong_for(&code);
    assert_eq!(
        verify_all(&env, &challenge, vec![wrong.clone(); 4]).await,
        vec![401; 4]
    );
    let burst = verify_all(&env, &challenge, vec![wrong; 12]).await;
    assert!(burst.iter().all(|s| *s == 401), "{burst:?}");
    let attempts: i16 = sqlx::query_scalar(
        "SELECT max(failed_attempts) FROM identity.one_time_code WHERE destination = $1",
    )
    .bind(&email)
    .fetch_one(&env.pools.admin)
    .await
    .unwrap();
    assert_eq!(attempts, 5, "attempts stop at 5");
    assert_eq!(
        verify_all(&env, &challenge, vec![code]).await,
        vec![401],
        "the right code is dead after 5 wrong attempts"
    );
    assert_eq!(live_sessions(&env, &email).await, 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn concurrent_success_and_wrong_guess_leave_the_code_unusable() {
    let env = env_or_skip!(true);
    let (challenge, code, email) = issued_login(&env, "cmix").await;
    let wrong = wrong_for(&code);
    assert_eq!(
        verify_all(&env, &challenge, vec![wrong.clone(); 4]).await,
        vec![401; 4]
    );
    let mut codes = vec![code.clone()];
    codes.extend(vec![wrong; 4]);
    let results = verify_all(&env, &challenge, codes).await;
    assert!(
        results.iter().filter(|s| **s == 200).count() <= 1,
        "{results:?}"
    );
    assert!(
        results.iter().all(|s| *s == 200 || *s == 401),
        "{results:?}"
    );
    // Whatever the order, the code is now spent or invalidated, and at most one session exists.
    assert_eq!(verify_all(&env, &challenge, vec![code]).await, vec![401]);
    assert!(live_sessions(&env, &email).await <= 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn one_code_verified_in_parallel_signs_in_exactly_once() {
    let env = env_or_skip!(true);
    let (challenge, code, email) = issued_login(&env, "cone").await;
    let results = verify_all(&env, &challenge, vec![code; 10]).await;
    assert_eq!(
        results.iter().filter(|s| **s == 200).count(),
        1,
        "{results:?}"
    );
    assert_eq!(live_sessions(&env, &email).await, 1);
}

// ---------------------------------------------------------------- Q35 setting (server config)

fn refused(status: u16) -> bool {
    (400..500).contains(&status)
}

#[tokio::test(flavor = "multi_thread")]
async fn q35_on_the_conditional_actions_are_refused_without_a_code() {
    let env = env_or_skip!(true);
    let (mut a, mail) = signed_in(&env, "q35on", true).await;
    // One source and one connection exist (with codes) so rotate / pairing-code have a target.
    let src = create_source(&mut a, &env, &mail, "S").await;
    let src_id = src.body["source"]["id"].as_str().unwrap().to_owned();
    let conn = create_connection(&mut a, &env, &mail, "C").await;
    let conn_id = conn.body["connection"]["id"].as_str().unwrap().to_owned();

    let sources_before = a
        .get("/api/v1/sources")
        .await
        .body
        .as_array()
        .unwrap()
        .len();
    let r = a
        .post(
            "/api/v1/sources",
            json!({"name": "NoCode", "source_type": "tradingview"}),
        )
        .await;
    assert!(
        refused(r.status),
        "create source: {} {:?}",
        r.status,
        r.body
    );
    assert_eq!(
        a.get("/api/v1/sources")
            .await
            .body
            .as_array()
            .unwrap()
            .len(),
        sources_before
    );
    let r = a
        .post(&format!("/api/v1/sources/{src_id}/rotate"), json!({}))
        .await;
    assert!(refused(r.status), "rotate: {} {:?}", r.status, r.body);
    let r = a
        .post("/api/v1/connections", json!({"name": "NoCode"}))
        .await;
    assert!(
        refused(r.status),
        "create connection: {} {:?}",
        r.status,
        r.body
    );
    let r = a
        .post(
            &format!("/api/v1/connections/{conn_id}/pairing-code"),
            json!({}),
        )
        .await;
    assert!(refused(r.status), "pairing code: {} {:?}", r.status, r.body);
}

#[tokio::test(flavor = "multi_thread")]
async fn q35_off_the_conditional_actions_need_no_code() {
    let env = env_or_skip!(false);
    let (mut a, _mail) = signed_in(&env, "q35off", true).await;

    let src = a
        .post(
            "/api/v1/sources",
            json!({"name": "NoCode", "source_type": "tradingview"}),
        )
        .await;
    assert_eq!(src.status, 201, "{:?}", src.body);
    let src_id = src.body["source"]["id"].as_str().unwrap().to_owned();
    let rot = a
        .post(&format!("/api/v1/sources/{src_id}/rotate"), json!({}))
        .await;
    assert_eq!(rot.status, 200, "{:?}", rot.body);
    let conn = a
        .post("/api/v1/connections", json!({"name": "NoCode"}))
        .await;
    assert_eq!(conn.status, 201, "{:?}", conn.body);
    let conn_id = conn.body["connection"]["id"].as_str().unwrap().to_owned();
    let pairing = a
        .post(
            &format!("/api/v1/connections/{conn_id}/pairing-code"),
            json!({}),
        )
        .await;
    assert_eq!(pairing.status, 201, "{:?}", pairing.body);
    let invitation = a
        .post(
            "/api/v1/admin/invitations",
            json!({"email": format!("{}@qa.test", unique("q35inv")), "locale": "en", "send_email": false}),
        )
        .await;
    assert_eq!(invitation.status, 201, "{:?}", invitation.body);
}

#[tokio::test(flavor = "multi_thread")]
async fn q35_off_unpair_and_channel_change_still_need_a_code() {
    let env = env_or_skip!(false);
    let (mut a, mail) = signed_in(&env, "q35keep", false).await;
    let conn = a.post("/api/v1/connections", json!({"name": "C"})).await;
    assert_eq!(conn.status, 201, "{:?}", conn.body);
    let conn_id = conn.body["connection"]["id"].as_str().unwrap().to_owned();
    let r = a
        .post(
            &format!("/api/v1/connections/{conn_id}/unpair"),
            json!({"acknowledge_open_positions": false}),
        )
        .await;
    assert!(
        refused(r.status),
        "unpair without a code: {} {:?}",
        r.status,
        r.body
    );
    let list = a.get("/api/v1/connections").await;
    let c = list
        .body
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == conn_id)
        .unwrap();
    assert_ne!(c["status"], "unpaired");

    // The channel change keeps its own two-code flow regardless of the setting.
    age_device(&env, &mail).await;
    let new = format!("{}@qa.test", unique("q35-new"));
    let started = start_change(&mut a, &new).await;
    assert_eq!(started.status, 202, "{:?}", started.body);
    assert_eq!(started.body["status"], "awaiting_codes");
    assert_eq!(a.get("/api/v1/me").await.body["email"], mail);
}
