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
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use identity_application::auth::{Activity, AuthContext, AuthenticateSession};
use identity_application::changes::{ChangeSet, Write};
use identity_application::deps::{IdentityDeps, IdentitySettings, RequestMeta};
use identity_application::ports::*;
use identity_enterprise::connection::AdapterInfo;
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::{SessionId, SourceId};
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::{ClientInfo, NewSession, Session};
use identity_enterprise::tokens::SecretBytes;
use identity_enterprise::user::{User, UserStatus};
use identity_infra::crypto::{OsRandom, PepperedHasher, VersionedKeys};
use identity_infra::store::PgIdentityStore;
use identity_infra::throttle::{
    LogPairingAlerts, PairingThrottlePolicy, PerIpPublicThrottle, PrefixPairingThrottle,
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
        .connect(&with_db(
            &std::env::var("MIGRATION_DATABASE_URL").unwrap(),
            &name,
        ))
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
        .connect(&with_db(
            &std::env::var("IDENTITY_TEST_APP_URL")
                .or_else(|_| std::env::var("DATABASE_URL"))
                .unwrap(),
            &name,
        ))
        .await
        .unwrap();
    let worker = PgPoolOptions::new()
        .max_connections(2)
        .connect(&with_db(
            &std::env::var("WORKER_DATABASE_URL").unwrap(),
            &name,
        ))
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
pub fn keys() -> VersionedKeys {
    VersionedKeys::new(1, BTreeMap::from([(1, vec![7u8; 32])])).unwrap()
}

pub fn deps(db: &Db) -> (IdentityDeps, Arc<PgIdentityStore>) {
    let store = Arc::new(PgIdentityStore::new(db.app.clone(), db.worker.clone()));
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
        settings: IdentitySettings {
            public_origin: "https://app.test".into(),
        },
    };
    (deps, store)
}

pub fn meta() -> RequestMeta {
    RequestMeta {
        ip: IpAddr::V4(Ipv4Addr::new(203, 0, 113, 9)),
        user_agent: Some("it".into()),
    }
}

/// Seeds a user directly (sign-up is being replaced, contract 2.0.0). Superuser insert: the row
/// is test data, not a use-case write.
pub async fn seed_user(db: &Db, email: &str, admin: bool) -> User {
    let user = User {
        id: UserId::new_v7(),
        email: EmailAddress::parse(email).unwrap(),
        locale: Locale::En,
        time_zone: TimeZoneName::parse("UTC").unwrap(),
        is_platform_admin: admin,
        status: UserStatus::Active,
        created_at: Utc::now(),
    };
    sqlx::query(
        "INSERT INTO identity.app_user (id, email, email_normalized, password_hash, locale, time_zone, is_platform_admin, created_at)
         VALUES ($1, $2, $3, 'unused', 'en', 'UTC', $4, $5)",
    )
    .bind(user.id.as_uuid())
    .bind(user.email.as_entered())
    .bind(user.email.normalized())
    .bind(admin)
    .bind(user.created_at)
    .execute(&db.inspect)
    .await
    .unwrap();
    user
}

/// Starts a session through the store, as the sign-in use case would.
pub async fn sign_in(deps: &IdentityDeps, email: &str) -> (String, AuthContext) {
    let normalized = EmailAddress::parse(email).unwrap();
    let user = deps
        .store
        .user_for_login(normalized.normalized())
        .await
        .unwrap()
        .unwrap();
    let mut secret = [0u8; 32];
    deps.random.fill(&mut secret);
    let secret = SecretBytes::from_random(secret);
    let new = NewSession {
        id: SessionId::new_v7(),
        user_id: user.id,
        token_hash: deps.hasher.sha256(secret.as_bytes()),
        persistent: false,
        client: ClientInfo {
            ip: Some(meta().ip),
            user_agent: meta().user_agent,
        },
    };
    let session = Session::start(new, false, Utc::now());
    deps.store
        .commit(ChangeSet::new(user.id).write(Write::InsertSession(session)))
        .await
        .unwrap();
    let value = secret.encode().expose().to_owned();
    let ctx = AuthenticateSession::new(deps.clone())
        .handle(Some(&value), Activity::UserInitiated)
        .await
        .unwrap();
    (value, ctx)
}
