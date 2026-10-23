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
use identity_application::auth::{Activity, AuthContext, AuthenticateSession, Requirement};
use identity_application::changes::{ChangeSet, Write};
use identity_application::deps::{IdentityDeps, IdentitySettings, RequestMeta};
use identity_application::ports::*;
use identity_application::usecases::signin::*;
use identity_application::views::LoginOutcome;
use identity_enterprise::connection::AdapterInfo;
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::SourceId;
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::secret::SecretString;
use identity_enterprise::user::{User, UserStatus};
use identity_infra::crypto::{
    AesGcmCipher, Argon2Cost, Argon2Passwords, OsRandom, PepperedHasher, VersionedKeys,
};
use identity_infra::store::PgIdentityStore;
use identity_infra::throttle::{
    LogPairingAlerts, LoginPolicy, PairingThrottlePolicy, PerIpPublicThrottle, PolicyLoginThrottle,
    PrefixPairingThrottle,
};
use shared_kernel::{SystemClock, UserId};
use sqlx::postgres::PgPoolOptions;
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;

pub const PASSWORD: &str = "correct horse battery";

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
    async fn send_password_reset(
        &self,
        _: &str,
        _: Locale,
        _: &SecretString,
    ) -> Result<(), MailError> {
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
pub struct NotBreached;
impl BreachedPasswords for NotBreached {
    fn contains(&self, _: &str) -> bool {
        false
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
        passwords: Arc::new(Argon2Passwords::new(Argon2Cost::MINIMUM).unwrap()),
        hasher: Arc::new(PepperedHasher::new(keys())),
        cipher: Arc::new(AesGcmCipher::new(keys()).unwrap()),
        random: Arc::new(OsRandom),
        breached: Arc::new(NotBreached),
        mailer: Arc::new(NoMail),
        presence: Arc::new(NoPresence),
        source_activity: Arc::new(NoActivity),
        source_keys: Arc::new(NoKeyListener),
        adapters: Arc::new(AllAdapters),
        login_throttle: Arc::new(PolicyLoginThrottle::new(LoginPolicy {
            per_ip_per_minute: 1000,
            ..LoginPolicy::default()
        })),
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

/// Seeds a user directly (users are normally created by accepting an invitation).
pub async fn seed_user(deps: &IdentityDeps, email: &str, admin: bool) -> User {
    let user = User {
        id: UserId::new_v7(),
        email: EmailAddress::parse(email).unwrap(),
        password_hash: deps
            .passwords
            .hash(&SecretString::new(PASSWORD))
            .await
            .unwrap(),
        locale: Locale::En,
        time_zone: TimeZoneName::parse("UTC").unwrap(),
        is_platform_admin: admin,
        status: UserStatus::Active,
        created_at: Utc::now(),
    };
    deps.store
        .commit(ChangeSet::new(user.id).write(Write::InsertUser(user.clone())))
        .await
        .unwrap();
    user
}

pub async fn sign_in(deps: &IdentityDeps, email: &str) -> (String, AuthContext) {
    let out = Login::new(deps.clone())
        .handle(LoginCommand {
            email: email.into(),
            password: SecretString::new(PASSWORD),
            remember_me: false,
            meta: meta(),
        })
        .await
        .unwrap();
    let LoginOutcome::Authenticated { cookie, .. } = out else {
        panic!("expected full session")
    };
    let value = cookie.value.expose().to_owned();
    let ctx = AuthenticateSession::new(deps.clone())
        .handle(Some(&value), Requirement::Full, Activity::UserInitiated)
        .await
        .unwrap();
    (value, ctx)
}
