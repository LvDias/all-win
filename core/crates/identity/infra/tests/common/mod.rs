//! Shared setup of the PostgreSQL-backed integration tests (see `pg_store.rs`).
#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use std::collections::{BTreeMap, HashMap};
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use identity_application::auth::{Activity, AuthContext, AuthenticateSession};
use identity_application::changes::{ChangeSet, Write};
use identity_application::deps::{IdentityDeps, IdentitySettings, RequestMeta};
use identity_application::ports::*;
use identity_application::usecases::delivery::DeliveryJob;
use identity_application::usecases::signin::{
    RequestLoginCode, RequestLoginCodeCommand, VerifyLoginCode, VerifyLoginCodeCommand,
};
use identity_application::views::SignedIn;
use identity_enterprise::channel::Channel;
use identity_enterprise::connection::AdapterInfo;
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::SourceId;
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::secret::SecretString;
use identity_enterprise::user::{User, UserStatus};
use identity_infra::crypto::{OsRandom, PepperedHasher, VersionedKeys};
use identity_infra::store::PgIdentityStore;
use identity_infra::throttle::{
    CodeThrottlePolicy, LogPairingAlerts, LogSecurityAlerts, PairingThrottlePolicy,
    PerIpPublicThrottle, PrefixCodeThrottle, PrefixPairingThrottle,
};
use shared_kernel::{SystemClock, UserId};
use sqlx::postgres::PgPoolOptions;
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;

pub struct Db {
    pub admin_url: String,
    pub name: String,
    pub app: PgPool,
    pub worker: PgPool,
    pub owner: PgPool,
    /// Superuser on the test database, for assertions only (bypasses RLS).
    pub inspect: PgPool,
}

pub fn with_db(url: &str, db: &str) -> String {
    let (base, query) = url
        .split_once('?')
        .map_or((url, None), |(b, q)| (b, Some(q)));
    let cut = base.rfind('/').expect("database url without path");
    let mut out = format!("{}/{db}", &base[..cut]);
    if let Some(q) = query {
        out.push('?');
        out.push_str(q);
    }
    out
}

/// Cluster-wide roles with the fixed test-only passwords the app tests also set
/// (`crates/app/tests/support/mod.rs`): every test binary converges on the same credentials,
/// whatever ran before it.
const ROLES: [(&str, &str, &str); 4] = [
    ("allwin_owner", "owner-test-only", "NOBYPASSRLS"),
    ("allwin_app", "app-test-only", "NOBYPASSRLS"),
    ("allwin_worker", "worker-test-only", "BYPASSRLS"),
    ("allwin_maint", "maint-test-only", "BYPASSRLS"),
];

/// `admin` with another user, password and database (same host, port and query).
fn as_role(admin: &str, role: &str, db: &str) -> String {
    let password = ROLES
        .iter()
        .find(|(r, _, _)| *r == role)
        .map(|(_, p, _)| *p)
        .unwrap();
    let url = with_db(admin, db);
    let (scheme, rest) = url.split_once("://").unwrap();
    let host = rest.rsplit_once('@').map_or(rest, |(_, h)| h);
    format!("{scheme}://{role}:{password}@{host}")
}

async fn ensure_roles(admin: &PgPool) {
    // Parallel tests would otherwise ALTER the same role at once ("tuple concurrently updated").
    // `admin` has one connection, so lock and unlock run on the same session.
    sqlx::query("SELECT pg_advisory_lock(7320301)")
        .execute(admin)
        .await
        .unwrap();
    for (role, password, rls) in ROLES {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = $1)")
                .bind(role)
                .fetch_one(admin)
                .await
                .unwrap();
        let verb = if exists { "ALTER" } else { "CREATE" };
        // Constants only, never input.
        let sql = format!("{verb} ROLE {role} WITH LOGIN PASSWORD '{password}' {rls}");
        if let Err(e) = sqlx::raw_sql(AssertSqlSafe(sql)).execute(admin).await {
            // A concurrent binary created it first.
            assert!(e.to_string().contains("already exists"), "{e}");
        }
    }
    sqlx::query("SELECT pg_advisory_unlock(7320301)")
        .execute(admin)
        .await
        .unwrap();
}

pub async fn setup() -> Option<Db> {
    let Ok(admin_url) = std::env::var("TEST_DATABASE_ADMIN_URL") else {
        eprintln!("skipped: TEST_DATABASE_ADMIN_URL not set");
        return None;
    };
    let name = format!("allwin_s3_t_{}", Uuid::now_v7().simple());
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_url)
        .await
        .unwrap();
    ensure_roles(&admin).await;
    // `name` is generated here from a UUID, never from input.
    sqlx::raw_sql(AssertSqlSafe(format!(
        "CREATE DATABASE {name} OWNER allwin_owner"
    )))
    .execute(&admin)
    .await
    .unwrap();
    admin.close().await;

    let owner = PgPoolOptions::new()
        .max_connections(2)
        .connect(&as_role(&admin_url, "allwin_owner", &name))
        .await
        .unwrap();
    let mut files: Vec<PathBuf> = std::env::var("ALLWIN_TEST_PRE_MIGRATIONS")
        .map(|v| {
            v.split(':')
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .collect()
        })
        .unwrap_or_default();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../migrations");
    let mut own: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "sql"))
        .collect();
    own.sort();
    files.extend(own);
    for f in files {
        let sql = std::fs::read_to_string(&f).unwrap();
        let mut tx = owner.begin().await.unwrap();
        sqlx::raw_sql(AssertSqlSafe(sql))
            .execute(&mut *tx)
            .await
            .unwrap_or_else(|e| panic!("{}: {e}", f.display()));
        tx.commit().await.unwrap();
    }
    let app = PgPoolOptions::new()
        .max_connections(8)
        .connect(&as_role(&admin_url, "allwin_app", &name))
        .await
        .unwrap();
    let worker = PgPoolOptions::new()
        .max_connections(2)
        .connect(&as_role(&admin_url, "allwin_worker", &name))
        .await
        .unwrap();
    let inspect = PgPoolOptions::new()
        .max_connections(1)
        .connect(&with_db(&admin_url, &name))
        .await
        .unwrap();
    Some(Db {
        admin_url,
        name,
        app,
        worker,
        owner,
        inspect,
    })
}

impl Db {
    pub async fn teardown(self) {
        self.app.close().await;
        self.worker.close().await;
        self.owner.close().await;
        self.inspect.close().await;
        let admin = PgPoolOptions::new()
            .max_connections(1)
            .connect(&self.admin_url)
            .await
            .unwrap();
        sqlx::raw_sql(AssertSqlSafe(format!(
            "DROP DATABASE {} WITH (FORCE)",
            self.name
        )))
        .execute(&admin)
        .await
        .unwrap();
    }
}

pub struct NoMail;
#[async_trait]
impl Mailer for NoMail {
    async fn send_invitation(&self, _: &str, _: Locale, _: &SecretString) -> Result<(), MailError> {
        Err(MailError)
    }
}
pub struct NoPresence;
#[async_trait]
impl ConnectionPresence for NoPresence {
    async fn accounts(
        &self,
        _: UserId,
        _: &[identity_enterprise::ids::ConnectionId],
    ) -> Result<HashMap<identity_enterprise::ids::ConnectionId, ConnectionAccounts>, String> {
        Ok(HashMap::new())
    }
}
pub struct NoActivity;
#[async_trait]
impl SourceActivity for NoActivity {
    async fn last_received(
        &self,
        _: UserId,
        _: &[SourceId],
    ) -> Result<HashMap<SourceId, DateTime<Utc>>, String> {
        Ok(HashMap::new())
    }
}
pub struct NoKeyListener;
impl SourceKeyListener for NoKeyListener {
    fn key_activated(&self, _: &identity_enterprise::tokens::KeyId) {}
}
pub struct AllAdapters;
impl AdapterCompatibility for AllAdapters {
    fn is_supported(&self, _: &AdapterInfo) -> bool {
        true
    }
}
pub struct NoAccounts;
#[async_trait]
impl AccountTargets for NoAccounts {
    async fn owned_by(&self, _: UserId, _: Uuid) -> Result<bool, String> {
        Ok(false)
    }
}

/// Captures deliveries (the plaintext code lives only here, as in memory) and records each
/// outcome as `sent` through the real use case, like the detached worker would.
#[derive(Default)]
pub struct CapturingDelivery {
    pub jobs: Mutex<Vec<DeliveryJob>>,
}

impl DeliveryQueue for CapturingDelivery {
    fn dispatch(&self, job: DeliveryJob) {
        self.jobs.lock().unwrap().push(job);
    }
}

impl CapturingDelivery {
    pub fn last_code_to(&self, destination: &str) -> Option<String> {
        self.jobs
            .lock()
            .unwrap()
            .iter()
            .rev()
            .find(|j| j.delivery.destination.normalized() == destination && !j.suppressed)
            .map(|j| j.delivery.code.expose().to_owned())
    }

    pub fn take(&self) -> Vec<DeliveryJob> {
        std::mem::take(&mut *self.jobs.lock().unwrap())
    }
}

pub fn keys() -> VersionedKeys {
    VersionedKeys::new(1, BTreeMap::from([(1, vec![7u8; 32])])).unwrap()
}

pub fn deps(db: &Db) -> (IdentityDeps, Arc<PgIdentityStore>, Arc<CapturingDelivery>) {
    deps_with(db, true)
}

/// `q35`: the `step_up.q35_persistence_actions` setting.
pub fn deps_with(
    db: &Db,
    q35: bool,
) -> (IdentityDeps, Arc<PgIdentityStore>, Arc<CapturingDelivery>) {
    let store = Arc::new(PgIdentityStore::new(db.app.clone(), db.worker.clone()));
    let delivery = Arc::new(CapturingDelivery::default());
    let deps = IdentityDeps {
        store: store.clone(),
        clock: Arc::new(SystemClock),
        hasher: Arc::new(PepperedHasher::new(keys())),
        random: Arc::new(OsRandom),
        mailer: Arc::new(NoMail),
        presence: Arc::new(NoPresence),
        source_activity: Arc::new(NoActivity),
        source_keys: Arc::new(NoKeyListener),
        adapters: Arc::new(AllAdapters),
        pairing_throttle: Arc::new(PrefixPairingThrottle::new(
            PairingThrottlePolicy {
                per_ip_per_minute: 1000,
                per_prefix_per_minute: 1000,
                ..PairingThrottlePolicy::default()
            },
            Arc::new(LogPairingAlerts),
        )),
        public_throttle: Arc::new(PerIpPublicThrottle::new(1000)),
        code_throttle: Arc::new(PrefixCodeThrottle::new(
            CodeThrottlePolicy {
                per_ip_15min: 1000,
                per_ip_24h: 1000,
                per_prefix_hour: 1000,
                ..CodeThrottlePolicy::default()
            },
            Arc::new(LogSecurityAlerts),
        )),
        delivery: delivery.clone(),
        alerts: Arc::new(LogSecurityAlerts),
        account_targets: Arc::new(NoAccounts),
        settings: IdentitySettings {
            public_origin: "https://app.test".into(),
            step_up_q35_persistence_actions: q35,
            whatsapp_enabled: false,
        },
    };
    (deps, store, delivery)
}

pub fn meta() -> RequestMeta {
    RequestMeta {
        ip: IpAddr::V4(Ipv4Addr::new(203, 0, 113, 9)),
        user_agent: Some("Mozilla/5.0 (X11; Linux x86_64) Firefox/131.0".into()),
    }
}

/// Seeds a user through the store (sign-up is covered by its own test).
pub async fn seed_user(deps: &IdentityDeps, email: &str, admin: bool) -> User {
    let now = Utc::now();
    let user = User {
        id: UserId::new_v7(),
        email: EmailAddress::parse(email).unwrap(),
        email_verified_at: now,
        whatsapp: None,
        locale: Locale::En,
        time_zone: TimeZoneName::parse("UTC").unwrap(),
        is_platform_admin: admin,
        status: UserStatus::Active,
        created_at: now,
        channel_changed_at: None,
    };
    deps.store
        .commit(ChangeSet::new(user.id).write(Write::InsertUser(user.clone())))
        .await
        .unwrap();
    user
}

/// Signs in by code (request, captured code, verify) and returns the cookies and context. The
/// durable send counters are cleared first, so tests can sign in again within the cooldown.
pub async fn sign_in(
    db: &Db,
    deps: &IdentityDeps,
    delivery: &CapturingDelivery,
    email: &str,
    device_cookie: Option<&str>,
) -> (SignedIn, AuthContext) {
    reset_send_limits(db).await;
    let normalized = EmailAddress::parse(email).unwrap().normalized().to_owned();
    let ch = RequestLoginCode::new(deps.clone())
        .handle(RequestLoginCodeCommand {
            email: email.into(),
            channel: Channel::Email,
            device_cookie: device_cookie.map(str::to_owned),
            meta: meta(),
        })
        .await
        .unwrap();
    let code = delivery.last_code_to(&normalized).unwrap();
    let signed = VerifyLoginCode::new(deps.clone())
        .handle(VerifyLoginCodeCommand {
            challenge_id: ch.challenge_id.expose().to_owned(),
            code,
            remember_me: false,
            device_cookie: device_cookie.map(str::to_owned),
            meta: meta(),
        })
        .await
        .unwrap();
    let ctx = AuthenticateSession::new(deps.clone())
        .handle(Some(signed.session.value.expose()), Activity::UserInitiated)
        .await
        .unwrap();
    (signed, ctx)
}

/// Requests a step-up code by e-mail and returns the proof for the action.
pub async fn step_up(
    db: &Db,
    deps: &IdentityDeps,
    delivery: &CapturingDelivery,
    ctx: &AuthContext,
    action: identity_enterprise::otp::StepUpAction,
    target: Option<Uuid>,
) -> identity_application::usecases::step_up::StepUpProof {
    reset_send_limits(db).await;
    let issued = identity_application::usecases::step_up::RequestStepUp::new(deps.clone())
        .handle(
            ctx,
            identity_application::usecases::step_up::RequestStepUpCommand {
                action,
                target_id: target,
                channel: Channel::Email,
                meta: meta(),
            },
        )
        .await
        .unwrap();
    identity_application::usecases::step_up::StepUpProof {
        challenge_id: issued.challenge_id.expose().to_owned(),
        code: delivery.last_code_to(ctx.user.email.normalized()).unwrap(),
    }
}

/// Clears the durable send counters (tests sign in repeatedly within the 60-s cooldown).
pub async fn reset_send_limits(db: &Db) {
    sqlx::query("DELETE FROM identity.code_send_counter")
        .execute(&db.inspect)
        .await
        .unwrap();
}
