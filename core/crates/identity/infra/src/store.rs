//! PostgreSQL implementation of [`IdentityStore`] (migrations #2, #3, #8).
//!
//! - User-owned reads and every change set run in a transaction that first sets
//!   `app.user_id` (RLS, data model §3) on the `allwin_app` pool.
//! - Pre-authentication lookups use the `SECURITY DEFINER` functions (`resolve_*`,
//!   `find_user_for_login`); `list_live_connections` runs on the `allwin_worker` pool.
//! - A change set is one transaction: its writes in order, then one outbox row per audit
//!   record and event. Conditional writes that touch no row and unique violations become
//!   [`StoreConflict`]s; the transaction is then rolled back.
//! - Errors never carry bound values (sqlx reports statement and constraint names only).

use std::net::IpAddr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use identity_application::changes::{ChangeSet, StoredKey, Write};
use identity_application::ports::{
    ConnectionRecord, IdentityStore, StoreConflict, StoreError, UserSummary,
};
use identity_enterprise::connection::{
    AdapterBuild, AdapterInfo, Connection, ConnectionStatus, PairedInstance, PairingCode,
};
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::{
    ConnectionId, InvitationId, PairingCodeId, PasswordResetId, SessionId, SourceId,
};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::password_reset::PasswordReset;
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::session::{ClientInfo, EndReason, MfaState, Session};
use identity_enterprise::source::{DisplayName, Source, SourceType};
use identity_enterprise::tokens::{CodeHmac, KeyId, TokenHash};
use identity_enterprise::totp::TotpCredential;
use identity_enterprise::user::{PasswordHash, User, UserStatus};
use shared_kernel::UserId;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::outbox::{self, Envelope};

pub struct PgIdentityStore {
    app: PgPool,
    worker: PgPool,
}

impl PgIdentityStore {
    /// `app`: `allwin_app` (NOBYPASSRLS). `worker`: `allwin_worker`, only for the startup scan.
    pub fn new(app: PgPool, worker: PgPool) -> Self {
        Self { app, worker }
    }

    async fn scoped(&self, user: UserId) -> Result<Transaction<'static, Postgres>, StoreError> {
        let mut tx = self.app.begin().await.map_err(db)?;
        set_scope(&mut tx, user).await?;
        Ok(tx)
    }
}

async fn set_scope(
    tx: &mut Transaction<'static, Postgres>,
    user: UserId,
) -> Result<(), StoreError> {
    sqlx::query!(
        "SELECT set_config('app.user_id', $1, true)",
        user.as_uuid().to_string()
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(db)?;
    Ok(())
}

fn db(e: sqlx::Error) -> StoreError {
    if let sqlx::Error::Database(d) = &e {
        let conflict = match d.constraint() {
            Some("app_user_email_normalized_key" | "invitation_open_email_key") => {
                Some(StoreConflict::EmailTaken)
            }
            Some("source_user_name_key" | "adapter_connection_user_name_key") => {
                Some(StoreConflict::NameTaken)
            }
            Some("source_key_key_id_key" | "device_token_key_id_key") => {
                Some(StoreConflict::KeyIdTaken)
            }
            Some("pairing_code_code_hmac_key") => Some(StoreConflict::CodeHmacTaken),
            _ => None,
        };
        if let Some(c) = conflict {
            return StoreError::Conflict(c);
        }
    }
    StoreError::Unavailable(e.to_string())
}

fn corrupt(what: &str) -> StoreError {
    StoreError::Unavailable(format!("unexpected stored value: {what}"))
}

fn hash32(bytes: Vec<u8>) -> Result<[u8; 32], StoreError> {
    bytes.try_into().map_err(|_| corrupt("hash length"))
}

fn ensure(rows: u64, conflict: StoreConflict) -> Result<(), StoreError> {
    if rows == 0 {
        Err(StoreError::Conflict(conflict))
    } else {
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn user_row(
    id: Uuid,
    email: String,
    email_normalized: String,
    password_hash: String,
    locale: String,
    time_zone: String,
    is_platform_admin: bool,
    status: String,
    created_at: DateTime<Utc>,
) -> Result<User, StoreError> {
    Ok(User {
        id: UserId::from_uuid(id),
        email: EmailAddress::from_stored(email, email_normalized),
        password_hash: PasswordHash::new(password_hash),
        locale: Locale::parse(&locale).ok_or_else(|| corrupt("locale"))?,
        time_zone: TimeZoneName::from_stored(time_zone),
        is_platform_admin,
        status: match status.as_str() {
            "active" => UserStatus::Active,
            "closed" => UserStatus::Closed,
            _ => return Err(corrupt("user status")),
        },
        created_at,
    })
}

fn mfa(raw: &str) -> Result<MfaState, StoreError> {
    match raw {
        "not_required" => Ok(MfaState::NotRequired),
        "pending" => Ok(MfaState::Pending),
        "verified" => Ok(MfaState::Verified),
        _ => Err(corrupt("mfa_state")),
    }
}

fn end_reason(raw: Option<String>) -> Result<Option<EndReason>, StoreError> {
    Ok(match raw.as_deref() {
        None => None,
        Some("logout") => Some(EndReason::Logout),
        Some("expired") => Some(EndReason::Expired),
        Some("password_reset") => Some(EndReason::PasswordReset),
        Some("password_changed") => Some(EndReason::PasswordChanged),
        Some("totp_changed") => Some(EndReason::TotpChanged),
        Some("admin_totp_reset") => Some(EndReason::AdminTotpReset),
        Some("rotated") => Some(EndReason::Rotated),
        Some(_) => return Err(corrupt("end_reason")),
    })
}

#[allow(clippy::too_many_arguments)]
fn invitation_row(
    id: Uuid,
    email_normalized: String,
    locale: String,
    issued_by: Uuid,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    accepted_at: Option<DateTime<Utc>>,
    accepted_user_id: Option<Uuid>,
    revoked_at: Option<DateTime<Utc>>,
    superseded_at: Option<DateTime<Utc>>,
) -> Result<Invitation, StoreError> {
    Ok(Invitation {
        id: InvitationId::from_uuid(id),
        email: email_normalized,
        locale: Locale::parse(&locale).ok_or_else(|| corrupt("locale"))?,
        issued_by: UserId::from_uuid(issued_by),
        created_at,
        expires_at,
        accepted_at,
        accepted_user_id: accepted_user_id.map(UserId::from_uuid),
        revoked_at,
        superseded_at,
    })
}

#[allow(clippy::too_many_arguments)]
fn source_row(
    id: Uuid,
    user_id: Uuid,
    source_type: String,
    name: String,
    created_at: DateTime<Utc>,
    archived_at: Option<DateTime<Utc>>,
    key_id: String,
    key_created_at: DateTime<Utc>,
    key_count: i64,
) -> Result<Source, StoreError> {
    Ok(Source {
        id: SourceId::from_uuid(id),
        user_id: UserId::from_uuid(user_id),
        source_type: SourceType::parse(&source_type).ok_or_else(|| corrupt("source_type"))?,
        name: DisplayName::from_stored(name),
        created_at,
        revoked_at: archived_at,
        key_id: KeyId::parse(&key_id).ok_or_else(|| corrupt("key_id"))?,
        rotated_at: (key_count > 1).then_some(key_created_at),
    })
}

struct ConnectionRow {
    id: Uuid,
    user_id: Uuid,
    name: String,
    adapter_type: Option<String>,
    instance_id: Option<Uuid>,
    instance_name: Option<String>,
    adapter_version: Option<String>,
    status: String,
    status_changed_at: DateTime<Utc>,
    paired_at: Option<DateTime<Utc>>,
    last_seen_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    archived_at: Option<DateTime<Utc>>,
    active_code_expires_at: Option<DateTime<Utc>>,
}

impl ConnectionRow {
    fn into_record(self) -> Result<ConnectionRecord, StoreError> {
        let instance = match (self.instance_id, self.paired_at) {
            (Some(instance_id), Some(paired_at)) => Some(PairedInstance {
                instance_id,
                instance_name: self.instance_name.unwrap_or_default(),
                adapter: AdapterInfo {
                    adapter_type: self.adapter_type.unwrap_or_default(),
                    version: self.adapter_version.unwrap_or_default(),
                    // The build flavour is checked at pairing and not stored.
                    build: AdapterBuild::Release,
                },
                paired_at,
            }),
            _ => None,
        };
        Ok(ConnectionRecord {
            connection: Connection {
                id: ConnectionId::from_uuid(self.id),
                user_id: UserId::from_uuid(self.user_id),
                name: DisplayName::from_stored(self.name),
                created_at: self.created_at,
                instance,
                status: ConnectionStatus::parse(&self.status)
                    .ok_or_else(|| corrupt("connection status"))?,
                status_changed_at: self.status_changed_at,
                last_seen_at: self.last_seen_at,
                unpaired_at: self.archived_at,
            },
            active_code_expires_at: self.active_code_expires_at,
        })
    }
}

fn ip_text(ip: Option<IpAddr>) -> Option<String> {
    ip.map(|ip| ip.to_string())
}

#[async_trait]
impl IdentityStore for PgIdentityStore {
    async fn user_for_login(&self, email_normalized: &str) -> Result<Option<User>, StoreError> {
        let row = sqlx::query!(
            r#"SELECT id AS "id!", email AS "email!", email_normalized AS "email_normalized!",
                      password_hash AS "password_hash!", locale AS "locale!", time_zone AS "time_zone!",
                      is_platform_admin AS "is_platform_admin!", status AS "status!", created_at AS "created_at!"
                 FROM identity.find_user_for_login($1)"#,
            email_normalized
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        row.map(|r| {
            user_row(
                r.id,
                r.email,
                r.email_normalized,
                r.password_hash,
                r.locale,
                r.time_zone,
                r.is_platform_admin,
                r.status,
                r.created_at,
            )
        })
        .transpose()
    }

    async fn user(&self, user_id: UserId) -> Result<Option<User>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query!(
            r#"SELECT id, email, email_normalized, password_hash, locale, time_zone, is_platform_admin, status, created_at
                 FROM identity.app_user WHERE id = $1"#,
            user_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(|r| {
            user_row(
                r.id,
                r.email,
                r.email_normalized,
                r.password_hash,
                r.locale,
                r.time_zone,
                r.is_platform_admin,
                r.status,
                r.created_at,
            )
        })
        .transpose()
    }

    async fn users(&self, admin: UserId) -> Result<Vec<UserSummary>, StoreError> {
        let mut tx = self.scoped(admin).await?;
        let rows = sqlx::query!(
            r#"SELECT l.id AS "id!", l.status AS "status!", l.is_platform_admin AS "is_platform_admin!",
                      l.created_at AS "created_at!", l.totp_enabled AS "totp_enabled!", l.last_sign_in_at,
                      l.email AS "email!"
                 FROM identity.admin_list_users() l"#
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.into_iter()
            .map(|r| {
                let normalized = r.email.trim().to_lowercase();
                Ok(UserSummary {
                    user: user_row(
                        r.id,
                        r.email,
                        normalized,
                        String::new(),
                        "en".into(),
                        "UTC".into(),
                        r.is_platform_admin,
                        r.status,
                        r.created_at,
                    )?,
                    totp_enabled: r.totp_enabled,
                    last_sign_in_at: r.last_sign_in_at,
                })
            })
            .collect()
    }

    async fn email_registered(
        &self,
        admin: UserId,
        email_normalized: &str,
    ) -> Result<bool, StoreError> {
        let mut tx = self.scoped(admin).await?;
        let found = sqlx::query_scalar!(
            r#"SELECT identity.email_registered($1) AS "found!""#,
            email_normalized
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(found)
    }

    async fn session_by_token(
        &self,
        token_hash: &TokenHash,
    ) -> Result<Option<Session>, StoreError> {
        let row = sqlx::query!(
            r#"SELECT id AS "id!", user_id AS "user_id!", is_persistent AS "is_persistent!", mfa_state AS "mfa_state!",
                      login_at AS "login_at!", authenticated_at AS "authenticated_at!", last_activity_at AS "last_activity_at!",
                      idle_expires_at AS "idle_expires_at!", absolute_expires_at AS "absolute_expires_at!",
                      ended_at, end_reason
                 FROM identity.resolve_session($1)"#,
            &token_hash.as_bytes()[..]
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        row.map(|r| {
            Ok(Session {
                id: SessionId::from_uuid(r.id),
                user_id: UserId::from_uuid(r.user_id),
                token_hash: *token_hash,
                persistent: r.is_persistent,
                mfa_state: mfa(&r.mfa_state)?,
                login_at: r.login_at,
                authenticated_at: r.authenticated_at,
                last_activity_at: r.last_activity_at,
                idle_expires_at: r.idle_expires_at,
                absolute_expires_at: r.absolute_expires_at,
                ended_at: r.ended_at,
                end_reason: end_reason(r.end_reason)?,
                client: ClientInfo::default(),
            })
        })
        .transpose()
    }

    async fn signed_in_from(
        &self,
        user_id: UserId,
        ip: IpAddr,
        since: DateTime<Utc>,
    ) -> Result<bool, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let found = sqlx::query_scalar!(
            r#"SELECT EXISTS (
                 SELECT 1 FROM identity.session
                  WHERE user_id = $1 AND ip_address = $2::text::inet AND login_at >= $3 AND mfa_state <> 'pending'
               ) AS "found!""#,
            user_id.as_uuid(),
            ip.to_string(),
            since
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(found)
    }

    async fn invitation(
        &self,
        invitation_id: InvitationId,
    ) -> Result<Option<Invitation>, StoreError> {
        let r = sqlx::query!(
            r#"SELECT id, email_normalized, locale, issued_by, created_at, expires_at, accepted_at, accepted_user_id,
                      revoked_at, superseded_at
                 FROM identity.invitation WHERE id = $1"#,
            invitation_id.as_uuid()
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        r.map(|r| {
            invitation_row(
                r.id,
                r.email_normalized,
                r.locale,
                r.issued_by,
                r.created_at,
                r.expires_at,
                r.accepted_at,
                r.accepted_user_id,
                r.revoked_at,
                r.superseded_at,
            )
        })
        .transpose()
    }

    async fn invitation_by_token(
        &self,
        token_hash: &TokenHash,
    ) -> Result<Option<Invitation>, StoreError> {
        let r = sqlx::query!(
            r#"SELECT id AS "id!", email_normalized AS "email_normalized!", locale AS "locale!", issued_by AS "issued_by!",
                      created_at AS "created_at!", expires_at AS "expires_at!", accepted_at, accepted_user_id,
                      revoked_at, superseded_at
                 FROM identity.resolve_invitation($1)"#,
            &token_hash.as_bytes()[..]
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        r.map(|r| {
            invitation_row(
                r.id,
                r.email_normalized,
                r.locale,
                r.issued_by,
                r.created_at,
                r.expires_at,
                r.accepted_at,
                r.accepted_user_id,
                r.revoked_at,
                r.superseded_at,
            )
        })
        .transpose()
    }

    async fn open_invitation_for(
        &self,
        email_normalized: &str,
    ) -> Result<Option<Invitation>, StoreError> {
        let r = sqlx::query!(
            r#"SELECT id, email_normalized, locale, issued_by, created_at, expires_at, accepted_at, accepted_user_id,
                      revoked_at, superseded_at
                 FROM identity.invitation
                WHERE email_normalized = $1 AND accepted_at IS NULL AND revoked_at IS NULL AND superseded_at IS NULL"#,
            email_normalized
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        r.map(|r| {
            invitation_row(
                r.id,
                r.email_normalized,
                r.locale,
                r.issued_by,
                r.created_at,
                r.expires_at,
                r.accepted_at,
                r.accepted_user_id,
                r.revoked_at,
                r.superseded_at,
            )
        })
        .transpose()
    }

    async fn invitations(&self) -> Result<Vec<Invitation>, StoreError> {
        let rows = sqlx::query!(
            r#"SELECT id, email_normalized, locale, issued_by, created_at, expires_at, accepted_at, accepted_user_id,
                      revoked_at, superseded_at
                 FROM identity.invitation ORDER BY created_at DESC"#
        )
        .fetch_all(&self.app)
        .await
        .map_err(db)?;
        rows.into_iter()
            .map(|r| {
                invitation_row(
                    r.id,
                    r.email_normalized,
                    r.locale,
                    r.issued_by,
                    r.created_at,
                    r.expires_at,
                    r.accepted_at,
                    r.accepted_user_id,
                    r.revoked_at,
                    r.superseded_at,
                )
            })
            .collect()
    }

    async fn password_reset_by_token(
        &self,
        token_hash: &TokenHash,
    ) -> Result<Option<PasswordReset>, StoreError> {
        let r = sqlx::query!(
            r#"SELECT id AS "id!", user_id AS "user_id!", created_at AS "created_at!", expires_at AS "expires_at!",
                      used_at, invalidated_at
                 FROM identity.resolve_password_reset($1)"#,
            &token_hash.as_bytes()[..]
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        Ok(r.map(|r| PasswordReset {
            id: PasswordResetId::from_uuid(r.id),
            user_id: UserId::from_uuid(r.user_id),
            token_hash: *token_hash,
            created_at: r.created_at,
            expires_at: r.expires_at,
            used_at: r.used_at,
            invalidated_at: r.invalidated_at,
        }))
    }

    async fn totp(&self, user_id: UserId) -> Result<Option<TotpCredential>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let r = sqlx::query!(
            "SELECT secret_encrypted, key_version, created_at, enabled_at, last_used_step FROM identity.totp_credential WHERE user_id = $1",
            user_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(r.map(|r| TotpCredential {
            secret_encrypted: r.secret_encrypted,
            key_version: r.key_version,
            created_at: r.created_at,
            enabled_at: r.enabled_at,
            last_used_step: r.last_used_step,
        }))
    }

    async fn recovery_codes_remaining(&self, user_id: UserId) -> Result<u8, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let n = sqlx::query_scalar!(
            r#"SELECT count(*) AS "n!" FROM identity.recovery_code WHERE user_id = $1 AND used_at IS NULL"#,
            user_id.as_uuid()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        u8::try_from(n).map_err(|_| corrupt("recovery code count"))
    }

    async fn sources(
        &self,
        user_id: UserId,
        include_revoked: bool,
    ) -> Result<Vec<Source>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let rows = sqlx::query!(
            r#"SELECT s.id, s.user_id, s.source_type, s.name, s.created_at, s.archived_at,
                      k.key_id AS "key_id!", k.created_at AS "key_created_at!", k.key_count AS "key_count!"
                 FROM identity.source s
                 JOIN LATERAL (
                   SELECT key_id, created_at, count(*) OVER () AS key_count
                     FROM identity.source_key WHERE source_id = s.id
                    ORDER BY created_at DESC, id DESC LIMIT 1
                 ) k ON true
                WHERE s.user_id = $1 AND ($2 OR s.archived_at IS NULL)
                ORDER BY s.created_at, s.id"#,
            user_id.as_uuid(),
            include_revoked
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.into_iter()
            .map(|r| {
                source_row(
                    r.id,
                    r.user_id,
                    r.source_type,
                    r.name,
                    r.created_at,
                    r.archived_at,
                    r.key_id,
                    r.key_created_at,
                    r.key_count,
                )
            })
            .collect()
    }

    async fn source(
        &self,
        user_id: UserId,
        source_id: SourceId,
    ) -> Result<Option<Source>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let r = sqlx::query!(
            r#"SELECT s.id, s.user_id, s.source_type, s.name, s.created_at, s.archived_at,
                      k.key_id AS "key_id!", k.created_at AS "key_created_at!", k.key_count AS "key_count!"
                 FROM identity.source s
                 JOIN LATERAL (
                   SELECT key_id, created_at, count(*) OVER () AS key_count
                     FROM identity.source_key WHERE source_id = s.id
                    ORDER BY created_at DESC, id DESC LIMIT 1
                 ) k ON true
                WHERE s.user_id = $1 AND s.id = $2"#,
            user_id.as_uuid(),
            source_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        r.map(|r| {
            source_row(
                r.id,
                r.user_id,
                r.source_type,
                r.name,
                r.created_at,
                r.archived_at,
                r.key_id,
                r.key_created_at,
                r.key_count,
            )
        })
        .transpose()
    }

    async fn connections(
        &self,
        user_id: UserId,
        include_unpaired: bool,
    ) -> Result<Vec<ConnectionRecord>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let rows = sqlx::query_as!(
            ConnectionRow,
            r#"SELECT c.id, c.user_id, c.name, c.adapter_type, c.instance_id, c.instance_name, c.adapter_version,
                      c.status, c.status_changed_at, c.paired_at, c.last_seen_at, c.created_at, c.archived_at,
                      (SELECT max(p.expires_at) FROM identity.pairing_code p
                        WHERE p.connection_id = c.id AND p.used_at IS NULL AND p.invalidated_at IS NULL) AS active_code_expires_at
                 FROM identity.adapter_connection c
                WHERE c.user_id = $1 AND ($2 OR c.archived_at IS NULL)
                ORDER BY c.created_at, c.id"#,
            user_id.as_uuid(),
            include_unpaired
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.into_iter().map(ConnectionRow::into_record).collect()
    }

    async fn connection(
        &self,
        user_id: UserId,
        connection_id: ConnectionId,
    ) -> Result<Option<ConnectionRecord>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query_as!(
            ConnectionRow,
            r#"SELECT c.id, c.user_id, c.name, c.adapter_type, c.instance_id, c.instance_name, c.adapter_version,
                      c.status, c.status_changed_at, c.paired_at, c.last_seen_at, c.created_at, c.archived_at,
                      (SELECT max(p.expires_at) FROM identity.pairing_code p
                        WHERE p.connection_id = c.id AND p.used_at IS NULL AND p.invalidated_at IS NULL) AS active_code_expires_at
                 FROM identity.adapter_connection c
                WHERE c.user_id = $1 AND c.id = $2"#,
            user_id.as_uuid(),
            connection_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(ConnectionRow::into_record).transpose()
    }

    async fn redeemable_pairing_code(
        &self,
        candidates: &[CodeHmac],
        now: DateTime<Utc>,
    ) -> Result<Option<PairingCode>, StoreError> {
        let hmacs: Vec<Vec<u8>> = candidates.iter().map(|c| c.bytes.to_vec()).collect();
        let rows = sqlx::query!(
            r#"SELECT id AS "id!", user_id AS "user_id!", connection_id AS "connection_id!", code_hmac AS "code_hmac!",
                      pepper_version AS "pepper_version!", created_at AS "created_at!", expires_at AS "expires_at!",
                      used_at, invalidated_at
                 FROM identity.resolve_pairing_code($1)"#,
            &hmacs[..]
        )
        .fetch_all(&self.app)
        .await
        .map_err(db)?;
        let codes = rows
            .into_iter()
            .map(|r| {
                Ok(PairingCode {
                    id: PairingCodeId::from_uuid(r.id),
                    user_id: UserId::from_uuid(r.user_id),
                    connection_id: ConnectionId::from_uuid(r.connection_id),
                    code_hmac: CodeHmac {
                        bytes: hash32(r.code_hmac)?,
                        pepper_version: r.pepper_version,
                    },
                    created_at: r.created_at,
                    expires_at: r.expires_at,
                    used_at: r.used_at,
                    invalidated_at: r.invalidated_at,
                })
            })
            .collect::<Result<Vec<_>, StoreError>>()?;
        Ok(codes.into_iter().find(|c| c.is_redeemable(now)))
    }

    async fn live_connections(&self) -> Result<Vec<(UserId, ConnectionId)>, StoreError> {
        let rows = sqlx::query!(r#"SELECT user_id AS "user_id!", connection_id AS "connection_id!" FROM identity.list_live_connections()"#)
            .fetch_all(&self.worker)
            .await
            .map_err(db)?;
        Ok(rows
            .into_iter()
            .map(|r| {
                (
                    UserId::from_uuid(r.user_id),
                    ConnectionId::from_uuid(r.connection_id),
                )
            })
            .collect())
    }

    async fn commit(&self, changes: ChangeSet) -> Result<(), StoreError> {
        let mut tx = self.scoped(changes.scope).await?;
        for write in changes.writes {
            apply(&mut tx, write).await?;
        }
        let envelopes = changes
            .audit
            .iter()
            .map(outbox::audit)
            .chain(changes.events.iter().map(outbox::event));
        let mut current = changes.scope;
        for envelope in envelopes {
            // An admin-trail row belongs to the admin; switch the RLS scope just for that insert.
            if envelope.user_id != current {
                set_scope(&mut tx, envelope.user_id).await?;
                current = envelope.user_id;
            }
            append_outbox(&mut tx, &envelope).await?;
        }
        tx.commit().await.map_err(db)
    }
}

async fn append_outbox(
    tx: &mut Transaction<'static, Postgres>,
    e: &Envelope,
) -> Result<(), StoreError> {
    sqlx::query!(
        "INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload) VALUES ($1, $2, $3, $4, $5)",
        Uuid::now_v7(),
        e.user_id.as_uuid(),
        e.event_type,
        e.occurred_at,
        e.payload
    )
    .execute(&mut **tx)
    .await
    .map_err(db)?;
    Ok(())
}

async fn insert_source_key(
    tx: &mut Transaction<'static, Postgres>,
    user: UserId,
    source: SourceId,
    key: &StoredKey,
    at: DateTime<Utc>,
) -> Result<(), StoreError> {
    sqlx::query!(
        "INSERT INTO identity.source_key (id, user_id, source_id, key_id, secret_hash, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        Uuid::now_v7(),
        user.as_uuid(),
        source.as_uuid(),
        key.key_id.as_str(),
        &key.secret_hash.as_bytes()[..],
        at
    )
    .execute(&mut **tx)
    .await
    .map_err(db)?;
    Ok(())
}

async fn apply(tx: &mut Transaction<'static, Postgres>, write: Write) -> Result<(), StoreError> {
    match write {
        Write::InsertUser(u) => {
            sqlx::query!(
                r#"INSERT INTO identity.app_user
                     (id, email, email_normalized, password_hash, locale, time_zone, is_platform_admin, status, created_at, updated_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)"#,
                u.id.as_uuid(),
                u.email.as_entered(),
                u.email.normalized(),
                u.password_hash.as_str(),
                u.locale.as_str(),
                u.time_zone.as_str(),
                u.is_platform_admin,
                u.status.as_str(),
                u.created_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::UpdateProfile {
            user_id,
            locale,
            time_zone,
        } => {
            sqlx::query!(
                r#"UPDATE identity.app_user
                      SET locale = COALESCE($2, locale), time_zone = COALESCE($3, time_zone), updated_at = now()
                    WHERE id = $1"#,
                user_id.as_uuid(),
                locale.map(Locale::as_str),
                time_zone.as_ref().map(TimeZoneName::as_str)
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::SetPasswordHash {
            user_id,
            password_hash,
        } => {
            sqlx::query!(
                "UPDATE identity.app_user SET password_hash = $2, updated_at = now() WHERE id = $1",
                user_id.as_uuid(),
                password_hash.as_str()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::InsertSession(s) => {
            sqlx::query!(
                r#"INSERT INTO identity.session
                     (id, user_id, token_hash, is_persistent, mfa_state, created_at, login_at, authenticated_at,
                      last_activity_at, idle_expires_at, absolute_expires_at, ip_address, user_agent)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $6, $6, $8, $9, $10::text::inet, $11)"#,
                s.id.as_uuid(),
                s.user_id.as_uuid(),
                &s.token_hash.as_bytes()[..],
                s.persistent,
                s.mfa_state.as_str(),
                s.authenticated_at,
                s.login_at,
                s.idle_expires_at,
                s.absolute_expires_at,
                ip_text(s.client.ip),
                s.client.user_agent
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::UpdateSessionActivity {
            session_id,
            last_activity_at,
            idle_expires_at,
            authenticated_at,
        } => {
            sqlx::query!(
                r#"UPDATE identity.session SET last_activity_at = $2, idle_expires_at = $3, authenticated_at = $4
                    WHERE id = $1 AND ended_at IS NULL"#,
                session_id.as_uuid(),
                last_activity_at,
                idle_expires_at,
                authenticated_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::EndSession {
            session_id,
            reason,
            at,
        } => {
            sqlx::query!(
                "UPDATE identity.session SET ended_at = $2, end_reason = $3 WHERE id = $1 AND ended_at IS NULL",
                session_id.as_uuid(),
                at,
                reason.as_str()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::EndUserSessions {
            user_id,
            except,
            reason,
            at,
        } => {
            sqlx::query!(
                r#"UPDATE identity.session SET ended_at = $3, end_reason = $4
                    WHERE user_id = $1 AND ended_at IS NULL AND ($2::uuid IS NULL OR id <> $2)"#,
                user_id.as_uuid(),
                except.map(|s| s.as_uuid()),
                at,
                reason.as_str()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::InsertInvitation {
            invitation: i,
            token_hash,
        } => {
            sqlx::query!(
                r#"INSERT INTO identity.invitation (id, email_normalized, locale, token_hash, issued_by, created_at, expires_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
                i.id.as_uuid(),
                i.email,
                i.locale.as_str(),
                &token_hash.as_bytes()[..],
                i.issued_by.as_uuid(),
                i.created_at,
                i.expires_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::SupersedeExpiredInvitation { invitation_id, at } => {
            sqlx::query!(
                r#"UPDATE identity.invitation SET superseded_at = $2
                    WHERE id = $1 AND accepted_at IS NULL AND revoked_at IS NULL AND superseded_at IS NULL AND expires_at <= $2"#,
                invitation_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::AcceptInvitation {
            invitation_id,
            user_id,
            at,
        } => {
            let done = sqlx::query!(
                r#"UPDATE identity.invitation SET accepted_at = $3, accepted_user_id = $2
                    WHERE id = $1 AND accepted_at IS NULL AND revoked_at IS NULL AND superseded_at IS NULL AND expires_at > $3"#,
                invitation_id.as_uuid(),
                user_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(done.rows_affected(), StoreConflict::InvitationGone)?;
        }
        Write::RevokeInvitation { invitation_id, at } => {
            sqlx::query!(
                r#"UPDATE identity.invitation SET revoked_at = $2
                    WHERE id = $1 AND accepted_at IS NULL AND revoked_at IS NULL AND superseded_at IS NULL"#,
                invitation_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::InsertPasswordReset(r) => {
            sqlx::query!(
                r#"UPDATE identity.password_reset SET invalidated_at = $2
                    WHERE user_id = $1 AND used_at IS NULL AND invalidated_at IS NULL"#,
                r.user_id.as_uuid(),
                r.created_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            sqlx::query!(
                "INSERT INTO identity.password_reset (id, user_id, token_hash, created_at, expires_at) VALUES ($1, $2, $3, $4, $5)",
                r.id.as_uuid(),
                r.user_id.as_uuid(),
                &r.token_hash.as_bytes()[..],
                r.created_at,
                r.expires_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::UsePasswordReset { reset_id, at } => {
            let done = sqlx::query!(
                r#"UPDATE identity.password_reset SET used_at = $2
                    WHERE id = $1 AND used_at IS NULL AND invalidated_at IS NULL AND expires_at > $2"#,
                reset_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(done.rows_affected(), StoreConflict::ResetGone)?;
        }
        Write::UpsertTotpSetup {
            user_id,
            credential: c,
        } => {
            sqlx::query!(
                r#"INSERT INTO identity.totp_credential (user_id, secret_encrypted, key_version, created_at)
                   VALUES ($1, $2, $3, $4)
                   ON CONFLICT (user_id) DO UPDATE
                     SET secret_encrypted = EXCLUDED.secret_encrypted, key_version = EXCLUDED.key_version,
                         created_at = EXCLUDED.created_at, enabled_at = NULL, last_used_step = NULL
                   WHERE identity.totp_credential.enabled_at IS NULL"#,
                user_id.as_uuid(),
                c.secret_encrypted,
                c.key_version,
                c.created_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::EnableTotp {
            user_id,
            at,
            used_step,
        } => {
            let done = sqlx::query!(
                "UPDATE identity.totp_credential SET enabled_at = $2, last_used_step = $3 WHERE user_id = $1 AND enabled_at IS NULL",
                user_id.as_uuid(),
                at,
                used_step
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(done.rows_affected(), StoreConflict::TotpNotPending)?;
        }
        Write::ConsumeTotpStep { user_id, step } => {
            let done = sqlx::query!(
                r#"UPDATE identity.totp_credential SET last_used_step = $2
                    WHERE user_id = $1 AND enabled_at IS NOT NULL AND (last_used_step IS NULL OR last_used_step < $2)"#,
                user_id.as_uuid(),
                step
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(done.rows_affected(), StoreConflict::TotpStepReplayed)?;
        }
        Write::DeleteTotp { user_id } => {
            sqlx::query!(
                "DELETE FROM identity.recovery_code WHERE user_id = $1",
                user_id.as_uuid()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            sqlx::query!(
                "DELETE FROM identity.totp_credential WHERE user_id = $1",
                user_id.as_uuid()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::ReplaceRecoveryCodes {
            user_id,
            batch_id,
            codes,
            at,
        } => {
            sqlx::query!(
                "DELETE FROM identity.recovery_code WHERE user_id = $1",
                user_id.as_uuid()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            let ids: Vec<Uuid> = codes.iter().map(|_| Uuid::now_v7()).collect();
            let hmacs: Vec<Vec<u8>> = codes.iter().map(|c| c.bytes.to_vec()).collect();
            let versions: Vec<i16> = codes.iter().map(|c| c.pepper_version).collect();
            sqlx::query!(
                r#"INSERT INTO identity.recovery_code (id, user_id, batch_id, code_hmac, pepper_version, created_at)
                   SELECT id, $2, $3, code_hmac, pepper_version, $4
                     FROM UNNEST($1::uuid[], $5::bytea[], $6::smallint[]) AS t(id, code_hmac, pepper_version)"#,
                &ids[..],
                user_id.as_uuid(),
                batch_id.as_uuid(),
                at,
                &hmacs[..],
                &versions[..]
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::ConsumeRecoveryCode {
            user_id,
            candidates,
            at,
        } => {
            let hmacs: Vec<Vec<u8>> = candidates.iter().map(|c| c.bytes.to_vec()).collect();
            let done = sqlx::query!(
                r#"UPDATE identity.recovery_code SET used_at = $3
                    WHERE id = (SELECT id FROM identity.recovery_code
                                 WHERE user_id = $1 AND used_at IS NULL AND code_hmac = ANY($2)
                                 LIMIT 1 FOR UPDATE)
                      AND used_at IS NULL"#,
                user_id.as_uuid(),
                &hmacs[..],
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(done.rows_affected(), StoreConflict::RecoveryCodeInvalid)?;
        }
        Write::InsertSource { source: s, key } => {
            sqlx::query!(
                "INSERT INTO identity.source (id, user_id, source_type, name, created_at) VALUES ($1, $2, $3, $4, $5)",
                s.id.as_uuid(),
                s.user_id.as_uuid(),
                s.source_type.as_str(),
                s.name.as_str(),
                s.created_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            insert_source_key(tx, s.user_id, s.id, &key, s.created_at).await?;
        }
        Write::RenameSource {
            user_id,
            source_id,
            name,
        } => {
            let done = sqlx::query!(
                "UPDATE identity.source SET name = $3 WHERE user_id = $1 AND id = $2 AND archived_at IS NULL",
                user_id.as_uuid(),
                source_id.as_uuid(),
                name.as_str()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(done.rows_affected(), StoreConflict::SourceRevoked)?;
        }
        Write::RotateSourceKey {
            user_id,
            source_id,
            key,
            at,
        } => {
            let active = sqlx::query_scalar!(
                "SELECT id FROM identity.source WHERE user_id = $1 AND id = $2 AND archived_at IS NULL FOR UPDATE",
                user_id.as_uuid(),
                source_id.as_uuid()
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(db)?;
            if active.is_none() {
                return Err(StoreError::Conflict(StoreConflict::SourceRevoked));
            }
            sqlx::query!(
                "UPDATE identity.source_key SET revoked_at = $3 WHERE user_id = $1 AND source_id = $2 AND revoked_at IS NULL",
                user_id.as_uuid(),
                source_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            insert_source_key(tx, user_id, source_id, &key, at).await?;
        }
        Write::RevokeSource {
            user_id,
            source_id,
            at,
        } => {
            sqlx::query!(
                "UPDATE identity.source SET archived_at = $3 WHERE user_id = $1 AND id = $2 AND archived_at IS NULL",
                user_id.as_uuid(),
                source_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            sqlx::query!(
                "UPDATE identity.source_key SET revoked_at = $3 WHERE user_id = $1 AND source_id = $2 AND revoked_at IS NULL",
                user_id.as_uuid(),
                source_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::InsertConnection(c) => {
            sqlx::query!(
                r#"INSERT INTO identity.adapter_connection (id, user_id, name, status, status_changed_at, created_at)
                   VALUES ($1, $2, $3, $4, $5, $6)"#,
                c.id.as_uuid(),
                c.user_id.as_uuid(),
                c.name.as_str(),
                c.status.as_str(),
                c.status_changed_at,
                c.created_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::RenameConnection {
            user_id,
            connection_id,
            name,
        } => {
            let done = sqlx::query!(
                "UPDATE identity.adapter_connection SET name = $3 WHERE user_id = $1 AND id = $2 AND archived_at IS NULL",
                user_id.as_uuid(),
                connection_id.as_uuid(),
                name.as_str()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(done.rows_affected(), StoreConflict::ConnectionUnpaired)?;
        }
        Write::IssuePairingCode(p) => {
            let active = sqlx::query_scalar!(
                "SELECT id FROM identity.adapter_connection WHERE user_id = $1 AND id = $2 AND archived_at IS NULL FOR UPDATE",
                p.user_id.as_uuid(),
                p.connection_id.as_uuid()
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(db)?;
            if active.is_none() {
                return Err(StoreError::Conflict(StoreConflict::ConnectionUnpaired));
            }
            sqlx::query!(
                r#"UPDATE identity.pairing_code SET invalidated_at = $3
                    WHERE user_id = $1 AND connection_id = $2 AND used_at IS NULL AND invalidated_at IS NULL"#,
                p.user_id.as_uuid(),
                p.connection_id.as_uuid(),
                p.created_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            sqlx::query!(
                r#"INSERT INTO identity.pairing_code (id, user_id, connection_id, code_hmac, pepper_version, created_at, expires_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
                p.id.as_uuid(),
                p.user_id.as_uuid(),
                p.connection_id.as_uuid(),
                &p.code_hmac.bytes[..],
                p.code_hmac.pepper_version,
                p.created_at,
                p.expires_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::UsePairingCode { code_id, at } => {
            // Two concurrent redemptions: the second sees used_at set (READ COMMITTED re-check) and gets 0 rows.
            let used = sqlx::query_scalar!(
                r#"UPDATE identity.pairing_code SET used_at = $2
                    WHERE id = $1 AND used_at IS NULL AND invalidated_at IS NULL AND expires_at > $2
                   RETURNING id"#,
                code_id.as_uuid(),
                at
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(db)?;
            if used.is_none() {
                return Err(StoreError::Conflict(StoreConflict::PairingCodeGone));
            }
        }
        Write::PairConnection {
            user_id,
            connection_id,
            instance,
            device_key,
        } => {
            let done = sqlx::query!(
                r#"UPDATE identity.adapter_connection
                      SET adapter_type = $3, instance_id = $4, instance_name = $5, adapter_version = $6,
                          paired_at = $7, status = 'disconnected', status_changed_at = $7
                    WHERE user_id = $1 AND id = $2 AND archived_at IS NULL"#,
                user_id.as_uuid(),
                connection_id.as_uuid(),
                instance.adapter.adapter_type,
                instance.instance_id,
                instance.instance_name,
                instance.adapter.version,
                instance.paired_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(done.rows_affected(), StoreConflict::ConnectionUnpaired)?;
            sqlx::query!(
                "UPDATE identity.device_token SET revoked_at = $3 WHERE user_id = $1 AND connection_id = $2 AND revoked_at IS NULL",
                user_id.as_uuid(),
                connection_id.as_uuid(),
                instance.paired_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            sqlx::query!(
                r#"INSERT INTO identity.device_token (id, user_id, connection_id, key_id, token_hash, created_at)
                   VALUES ($1, $2, $3, $4, $5, $6)"#,
                Uuid::now_v7(),
                user_id.as_uuid(),
                connection_id.as_uuid(),
                device_key.key_id.as_str(),
                &device_key.secret_hash.as_bytes()[..],
                instance.paired_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::UpdateConnectionStatus {
            user_id,
            connection_id,
            status,
            status_changed_at,
            last_seen_at,
        } => {
            sqlx::query!(
                r#"UPDATE identity.adapter_connection SET status = $3, status_changed_at = $4, last_seen_at = $5
                    WHERE user_id = $1 AND id = $2 AND archived_at IS NULL"#,
                user_id.as_uuid(),
                connection_id.as_uuid(),
                status.as_str(),
                status_changed_at,
                last_seen_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::DisconnectLiveConnection {
            user_id,
            connection_id,
            at,
        } => {
            sqlx::query!(
                r#"UPDATE identity.adapter_connection SET status = 'disconnected', status_changed_at = $3
                    WHERE user_id = $1 AND id = $2 AND status IN ('connected', 'outdated')"#,
                user_id.as_uuid(),
                connection_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::UnpairConnection {
            user_id,
            connection_id,
            at,
        } => {
            sqlx::query!(
                r#"UPDATE identity.adapter_connection SET status = 'unpaired', status_changed_at = $3, archived_at = $3
                    WHERE user_id = $1 AND id = $2 AND archived_at IS NULL"#,
                user_id.as_uuid(),
                connection_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            sqlx::query!(
                "UPDATE identity.device_token SET revoked_at = $3 WHERE user_id = $1 AND connection_id = $2 AND revoked_at IS NULL",
                user_id.as_uuid(),
                connection_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            sqlx::query!(
                r#"UPDATE identity.pairing_code SET invalidated_at = $3
                    WHERE user_id = $1 AND connection_id = $2 AND used_at IS NULL AND invalidated_at IS NULL"#,
                user_id.as_uuid(),
                connection_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
    }
    Ok(())
}
