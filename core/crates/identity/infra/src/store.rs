//! PostgreSQL implementation of [`IdentityStore`] (migrations #2, #2b, #3, #8).
//!
//! - User-owned reads and every change set run in a transaction that first sets
//!   `app.user_id` (RLS, data model §3) on the `allwin_app` pool.
//! - Pre-authentication lookups use the `SECURITY DEFINER` functions (`resolve_*`,
//!   `find_user_for_login`, `device_key_in_use`); `list_live_connections` and the due-hold scan
//!   run on the `allwin_worker` pool.
//! - Code verification uses conditional UPDATEs instead of `SELECT … FOR UPDATE` (Lastro,
//!   2026-10-23): a failure increments and invalidates at 5 only on an open row, a success consumes
//!   only the open row with the HMAC that was checked, and expiry is compared with the same clock
//!   that wrote it (the app clock, passed as a parameter).
//! - A change set is one transaction: its writes in order, then one outbox row per audit
//!   record and event. Conditional writes that touch no row and unique violations become
//!   [`StoreConflict`]s; the transaction is then rolled back.
//! - Errors never carry bound values (sqlx reports statement and constraint names only).

use std::net::IpAddr;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use identity_application::changes::{ChangeSet, CodeTable, StoredKey, Write};
use identity_application::ports::{
    ConnectionRecord, IdentityStore, LoginTarget, StoreConflict, StoreError, UserSummary,
    WindowCount,
};
use identity_enterprise::channel::{Channel, Destination, PhoneNumber};
use identity_enterprise::channel_change::{ChangeStatus, ChannelChange};
use identity_enterprise::connection::{
    AdapterBuild, AdapterInfo, Connection, ConnectionStatus, PairedInstance, PairingCode,
};
use identity_enterprise::device::KnownDevice;
use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::{
    ChannelChangeId, CodeId, ConnectionId, DeviceId, InvitationId, PairingCodeId, SessionId,
    SourceId,
};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::otp::{
    CodeRole, CodeState, DeliveryState, DeviceClass, InvalidReason, OneTimeCode, Purpose,
    SignupCode, StepUpAction,
};
use identity_enterprise::profile::{Locale, TimeZoneName};
use identity_enterprise::session::{ClientInfo, EndReason, Session};
use identity_enterprise::source::{DisplayName, Source, SourceType};
use identity_enterprise::tokens::{CodeHmac, KeyId, TokenHash};
use identity_enterprise::user::{User, UserStatus};
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
            Some("app_user_whatsapp_key") => Some(StoreConflict::WhatsappTaken),
            Some("channel_change_one_open") => Some(StoreConflict::ChannelChangeOpen),
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

struct UserRow {
    id: Uuid,
    email: String,
    email_normalized: String,
    email_verified_at: DateTime<Utc>,
    whatsapp_e164: Option<String>,
    locale: String,
    time_zone: String,
    is_platform_admin: bool,
    status: String,
    created_at: DateTime<Utc>,
    channel_changed_at: Option<DateTime<Utc>>,
}

impl UserRow {
    fn into_user(self) -> Result<User, StoreError> {
        Ok(User {
            id: UserId::from_uuid(self.id),
            email: EmailAddress::from_stored(self.email, self.email_normalized),
            email_verified_at: self.email_verified_at,
            whatsapp: self.whatsapp_e164.map(PhoneNumber::from_stored),
            locale: Locale::parse(&self.locale).ok_or_else(|| corrupt("locale"))?,
            time_zone: TimeZoneName::from_stored(self.time_zone),
            is_platform_admin: self.is_platform_admin,
            status: UserStatus::parse(&self.status).ok_or_else(|| corrupt("user status"))?,
            created_at: self.created_at,
            channel_changed_at: self.channel_changed_at,
        })
    }
}

fn end_reason(raw: Option<String>) -> Result<Option<EndReason>, StoreError> {
    raw.map(|r| EndReason::parse(&r).ok_or_else(|| corrupt("end_reason")))
        .transpose()
}

fn channel(raw: &str) -> Result<Channel, StoreError> {
    Channel::parse(raw).ok_or_else(|| corrupt("channel"))
}

fn device_class(c: DeviceClass) -> &'static str {
    c.as_str()
}

fn ip_of(raw: Option<String>) -> Option<IpAddr> {
    raw.and_then(|s| s.split('/').next().and_then(|a| a.parse().ok()))
}

struct DeviceRow {
    id: Uuid,
    user_id: Uuid,
    device_key_hash: Vec<u8>,
    label: Option<String>,
    first_seen_at: DateTime<Utc>,
    last_seen_at: DateTime<Utc>,
    last_ip: Option<String>,
    revoked_at: Option<DateTime<Utc>>,
}

impl DeviceRow {
    fn into_device(self) -> Result<KnownDevice, StoreError> {
        Ok(KnownDevice {
            id: DeviceId::from_uuid(self.id),
            user_id: UserId::from_uuid(self.user_id),
            key_hash: TokenHash::from_bytes(hash32(self.device_key_hash)?),
            label: self.label,
            first_seen_at: self.first_seen_at,
            last_seen_at: self.last_seen_at,
            last_ip: ip_of(self.last_ip),
            revoked_at: self.revoked_at,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn code_state(
    code_hmac: Vec<u8>,
    pepper_version: i16,
    failed_attempts: i16,
    created_at: DateTime<Utc>,
    sent_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    consumed_at: Option<DateTime<Utc>>,
    invalidated_at: Option<DateTime<Utc>>,
    invalidated_reason: Option<String>,
    delivery_state: &str,
) -> Result<CodeState, StoreError> {
    Ok(CodeState {
        code_hmac: CodeHmac {
            bytes: hash32(code_hmac)?,
            pepper_version,
        },
        failed_attempts,
        created_at,
        sent_at,
        expires_at,
        consumed_at,
        invalidated_at,
        invalidated_reason: invalidated_reason
            .map(|r| InvalidReason::parse(&r).ok_or_else(|| corrupt("invalidated_reason")))
            .transpose()?,
        delivery_state: DeliveryState::parse(delivery_state)
            .ok_or_else(|| corrupt("delivery_state"))?,
    })
}

struct OneTimeRow {
    id: Uuid,
    user_id: Uuid,
    channel_change_id: Option<Uuid>,
    purpose: String,
    channel: String,
    destination: String,
    action: Option<String>,
    target_id: Option<Uuid>,
    session_id: Option<Uuid>,
    code_role: Option<String>,
    code_hmac: Vec<u8>,
    pepper_version: i16,
    failed_attempts: i16,
    created_at: DateTime<Utc>,
    sent_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    consumed_at: Option<DateTime<Utc>>,
    invalidated_at: Option<DateTime<Utc>>,
    invalidated_reason: Option<String>,
    request_ip: Option<String>,
    delivery_state: String,
}

impl OneTimeRow {
    fn into_code(self) -> Result<OneTimeCode, StoreError> {
        Ok(OneTimeCode {
            id: CodeId::from_uuid(self.id),
            user_id: UserId::from_uuid(self.user_id),
            purpose: Purpose::parse(&self.purpose).ok_or_else(|| corrupt("purpose"))?,
            destination: Destination::from_stored(channel(&self.channel)?, self.destination),
            action: self
                .action
                .map(|a| StepUpAction::parse(&a).ok_or_else(|| corrupt("action")))
                .transpose()?,
            target_id: self.target_id,
            session_id: self.session_id.map(SessionId::from_uuid),
            channel_change_id: self.channel_change_id.map(ChannelChangeId::from_uuid),
            role: self
                .code_role
                .map(|r| CodeRole::parse(&r).ok_or_else(|| corrupt("code_role")))
                .transpose()?,
            request_ip: ip_of(self.request_ip),
            state: code_state(
                self.code_hmac,
                self.pepper_version,
                self.failed_attempts,
                self.created_at,
                self.sent_at,
                self.expires_at,
                self.consumed_at,
                self.invalidated_at,
                self.invalidated_reason,
                &self.delivery_state,
            )?,
        })
    }
}

struct ChangeRow {
    id: Uuid,
    user_id: Uuid,
    channel: String,
    new_value: String,
    requested_session_id: Uuid,
    status: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    hold_until: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    cancelled_at: Option<DateTime<Utc>>,
    cancelled_by_session_id: Option<Uuid>,
}

impl ChangeRow {
    fn into_change(self) -> Result<ChannelChange, StoreError> {
        Ok(ChannelChange {
            id: ChannelChangeId::from_uuid(self.id),
            user_id: UserId::from_uuid(self.user_id),
            new_value: Destination::from_stored(channel(&self.channel)?, self.new_value),
            requested_session_id: SessionId::from_uuid(self.requested_session_id),
            status: ChangeStatus::parse(&self.status).ok_or_else(|| corrupt("change status"))?,
            created_at: self.created_at,
            expires_at: self.expires_at,
            hold_until: self.hold_until,
            completed_at: self.completed_at,
            cancelled_at: self.cancelled_at,
            cancelled_by_session_id: self.cancelled_by_session_id.map(SessionId::from_uuid),
        })
    }
}

fn window_start(now: DateTime<Utc>, seconds: i32) -> DateTime<Utc> {
    let s = i64::from(seconds);
    let t = now.timestamp();
    DateTime::from_timestamp(t - t.rem_euclid(s), 0).unwrap_or(now)
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
    async fn login_target(
        &self,
        email_normalized: &str,
    ) -> Result<Option<LoginTarget>, StoreError> {
        let row = sqlx::query!(
            r#"SELECT id AS "id!", status AS "status!", locale AS "locale!", email AS "email!", whatsapp_e164
                 FROM identity.find_user_for_login($1)"#,
            email_normalized
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        row.map(|r| {
            let mut destinations = vec![Destination::Email(EmailAddress::from_stored(
                r.email,
                email_normalized.to_owned(),
            ))];
            if let Some(w) = r.whatsapp_e164 {
                destinations.push(Destination::Whatsapp(PhoneNumber::from_stored(w)));
            }
            Ok(LoginTarget {
                user_id: UserId::from_uuid(r.id),
                status: UserStatus::parse(&r.status).ok_or_else(|| corrupt("user status"))?,
                locale: Locale::parse(&r.locale).ok_or_else(|| corrupt("locale"))?,
                destinations,
            })
        })
        .transpose()
    }

    async fn user(&self, user_id: UserId) -> Result<Option<User>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, email, email_normalized, email_verified_at, whatsapp_e164, locale, time_zone,
                      is_platform_admin, status, created_at, channel_changed_at
                 FROM identity.app_user WHERE id = $1"#,
            user_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(UserRow::into_user).transpose()
    }

    async fn users(&self, admin: UserId) -> Result<Vec<UserSummary>, StoreError> {
        let mut tx = self.scoped(admin).await?;
        let rows = sqlx::query!(
            r#"SELECT l.id AS "id!", l.status AS "status!", l.is_platform_admin AS "is_platform_admin!",
                      l.created_at AS "created_at!", l.whatsapp_verified AS "whatsapp_verified!", l.last_sign_in_at,
                      l.email AS "email!"
                 FROM identity.admin_list_users() l"#
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.into_iter()
            .map(|r| {
                Ok(UserSummary {
                    id: UserId::from_uuid(r.id),
                    email: r.email,
                    status: UserStatus::parse(&r.status).ok_or_else(|| corrupt("user status"))?,
                    is_platform_admin: r.is_platform_admin,
                    whatsapp_verified: r.whatsapp_verified,
                    created_at: r.created_at,
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
            r#"SELECT id AS "id!", user_id AS "user_id!", device_id AS "device_id!", is_persistent AS "is_persistent!",
                      login_channel AS "login_channel!", login_at AS "login_at!", last_activity_at AS "last_activity_at!",
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
                device_id: DeviceId::from_uuid(r.device_id),
                token_hash: *token_hash,
                persistent: r.is_persistent,
                login_channel: channel(&r.login_channel)?,
                login_at: r.login_at,
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

    async fn session(
        &self,
        user_id: UserId,
        session_id: SessionId,
    ) -> Result<Option<Session>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query!(
            r#"SELECT id, user_id, device_id, token_hash, is_persistent, login_channel, login_at, last_activity_at,
                      idle_expires_at, absolute_expires_at, ended_at, end_reason
                 FROM identity.session WHERE user_id = $1 AND id = $2"#,
            user_id.as_uuid(),
            session_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(|r| {
            Ok(Session {
                id: SessionId::from_uuid(r.id),
                user_id: UserId::from_uuid(r.user_id),
                device_id: DeviceId::from_uuid(r.device_id),
                token_hash: TokenHash::from_bytes(hash32(r.token_hash)?),
                persistent: r.is_persistent,
                login_channel: channel(&r.login_channel)?,
                login_at: r.login_at,
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

    async fn device_by_key(
        &self,
        user_id: UserId,
        key_hash: &TokenHash,
    ) -> Result<Option<KnownDevice>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query_as!(
            DeviceRow,
            r#"SELECT id, user_id, device_key_hash, label, first_seen_at, last_seen_at, last_ip::text AS last_ip, revoked_at
                 FROM identity.known_device WHERE user_id = $1 AND device_key_hash = $2
                ORDER BY revoked_at IS NULL DESC, last_seen_at DESC LIMIT 1"#,
            user_id.as_uuid(),
            &key_hash.as_bytes()[..]
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(DeviceRow::into_device).transpose()
    }

    async fn device_key_in_use(&self, key_hash: &TokenHash) -> Result<bool, StoreError> {
        sqlx::query_scalar!(
            r#"SELECT identity.device_key_in_use($1) AS "found!""#,
            &key_hash.as_bytes()[..]
        )
        .fetch_one(&self.app)
        .await
        .map_err(db)
    }

    async fn device(
        &self,
        user_id: UserId,
        device_id: DeviceId,
    ) -> Result<Option<KnownDevice>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query_as!(
            DeviceRow,
            r#"SELECT id, user_id, device_key_hash, label, first_seen_at, last_seen_at, last_ip::text AS last_ip, revoked_at
                 FROM identity.known_device WHERE user_id = $1 AND id = $2"#,
            user_id.as_uuid(),
            device_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(DeviceRow::into_device).transpose()
    }

    async fn devices(&self, user_id: UserId) -> Result<Vec<KnownDevice>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let rows = sqlx::query_as!(
            DeviceRow,
            r#"SELECT id, user_id, device_key_hash, label, first_seen_at, last_seen_at, last_ip::text AS last_ip, revoked_at
                 FROM identity.known_device WHERE user_id = $1 AND revoked_at IS NULL
                ORDER BY last_seen_at DESC"#,
            user_id.as_uuid()
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.into_iter().map(DeviceRow::into_device).collect()
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

    async fn signup_code(&self, code_id: CodeId) -> Result<Option<SignupCode>, StoreError> {
        let r = sqlx::query!(
            r#"SELECT id, invitation_id, code_hmac, pepper_version, failed_attempts, created_at, sent_at, expires_at,
                      consumed_at, invalidated_at, invalidated_reason, request_ip::text AS request_ip, delivery_state
                 FROM identity.signup_code WHERE id = $1"#,
            code_id.as_uuid()
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        r.map(|r| {
            Ok(SignupCode {
                id: CodeId::from_uuid(r.id),
                invitation_id: InvitationId::from_uuid(r.invitation_id),
                request_ip: ip_of(r.request_ip),
                state: code_state(
                    r.code_hmac,
                    r.pepper_version,
                    r.failed_attempts,
                    r.created_at,
                    r.sent_at,
                    r.expires_at,
                    r.consumed_at,
                    r.invalidated_at,
                    r.invalidated_reason,
                    &r.delivery_state,
                )?,
            })
        })
        .transpose()
    }

    async fn one_time_code_owner(
        &self,
        code_id: CodeId,
    ) -> Result<Option<(UserId, Purpose)>, StoreError> {
        let r = sqlx::query!(
            r#"SELECT user_id AS "user_id!", purpose AS "purpose!" FROM identity.resolve_one_time_code($1)"#,
            code_id.as_uuid()
        )
        .fetch_optional(&self.app)
        .await
        .map_err(db)?;
        r.map(|r| {
            Ok((
                UserId::from_uuid(r.user_id),
                Purpose::parse(&r.purpose).ok_or_else(|| corrupt("purpose"))?,
            ))
        })
        .transpose()
    }

    async fn one_time_code(
        &self,
        user_id: UserId,
        code_id: CodeId,
    ) -> Result<Option<OneTimeCode>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query_as!(
            OneTimeRow,
            r#"SELECT id, user_id, channel_change_id, purpose, channel, destination, action, target_id, session_id,
                      code_role, code_hmac, pepper_version, failed_attempts, created_at, sent_at, expires_at,
                      consumed_at, invalidated_at, invalidated_reason, request_ip::text AS request_ip, delivery_state
                 FROM identity.one_time_code WHERE user_id = $1 AND id = $2"#,
            user_id.as_uuid(),
            code_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(OneTimeRow::into_code).transpose()
    }

    async fn change_codes(
        &self,
        user_id: UserId,
        change_id: ChannelChangeId,
    ) -> Result<Vec<OneTimeCode>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let rows = sqlx::query_as!(
            OneTimeRow,
            r#"SELECT id, user_id, channel_change_id, purpose, channel, destination, action, target_id, session_id,
                      code_role, code_hmac, pepper_version, failed_attempts, created_at, sent_at, expires_at,
                      consumed_at, invalidated_at, invalidated_reason, request_ip::text AS request_ip, delivery_state
                 FROM identity.one_time_code WHERE user_id = $1 AND channel_change_id = $2
                ORDER BY created_at, id"#,
            user_id.as_uuid(),
            change_id.as_uuid()
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.into_iter().map(OneTimeRow::into_code).collect()
    }

    async fn signed_in_sends_since(
        &self,
        user_id: UserId,
        since: DateTime<Utc>,
    ) -> Result<i64, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let n = sqlx::query_scalar!(
            r#"SELECT count(*) AS "n!" FROM identity.one_time_code
                WHERE user_id = $1 AND purpose IN ('step_up', 'channel_change') AND created_at >= $2"#,
            user_id.as_uuid(),
            since
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(n)
    }

    async fn channel_change(
        &self,
        user_id: UserId,
        change_id: ChannelChangeId,
    ) -> Result<Option<ChannelChange>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query_as!(
            ChangeRow,
            r#"SELECT id, user_id, channel, new_value, requested_session_id, status, created_at, expires_at,
                      hold_until, completed_at, cancelled_at, cancelled_by_session_id
                 FROM identity.channel_change WHERE user_id = $1 AND id = $2"#,
            user_id.as_uuid(),
            change_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(ChangeRow::into_change).transpose()
    }

    async fn open_channel_change(
        &self,
        user_id: UserId,
    ) -> Result<Option<ChannelChange>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query_as!(
            ChangeRow,
            r#"SELECT id, user_id, channel, new_value, requested_session_id, status, created_at, expires_at,
                      hold_until, completed_at, cancelled_at, cancelled_by_session_id
                 FROM identity.channel_change
                WHERE user_id = $1 AND status IN ('awaiting_codes', 'on_hold')"#,
            user_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(ChangeRow::into_change).transpose()
    }

    async fn latest_channel_change(
        &self,
        user_id: UserId,
    ) -> Result<Option<ChannelChange>, StoreError> {
        let mut tx = self.scoped(user_id).await?;
        let row = sqlx::query_as!(
            ChangeRow,
            r#"SELECT id, user_id, channel, new_value, requested_session_id, status, created_at, expires_at,
                      hold_until, completed_at, cancelled_at, cancelled_by_session_id
                 FROM identity.channel_change WHERE user_id = $1
                ORDER BY created_at DESC, id DESC LIMIT 1"#,
            user_id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(ChangeRow::into_change).transpose()
    }

    async fn due_channel_holds(
        &self,
        now: DateTime<Utc>,
    ) -> Result<Vec<(UserId, ChannelChangeId)>, StoreError> {
        let rows = sqlx::query!(
            r#"SELECT user_id, id FROM identity.channel_change
                WHERE status = 'on_hold' AND hold_until <= $1 ORDER BY hold_until"#,
            now
        )
        .fetch_all(&self.worker)
        .await
        .map_err(db)?;
        Ok(rows
            .into_iter()
            .map(|r| {
                (
                    UserId::from_uuid(r.user_id),
                    ChannelChangeId::from_uuid(r.id),
                )
            })
            .collect())
    }

    async fn count_send(
        &self,
        identifier: &CodeHmac,
        class: DeviceClass,
        windows: &[i32],
        now: DateTime<Utc>,
    ) -> Result<Vec<WindowCount>, StoreError> {
        let mut tx = self.app.begin().await.map_err(db)?;
        let mut out = Vec::with_capacity(windows.len());
        for w in windows {
            let start = window_start(now, *w);
            let sends = sqlx::query_scalar!(
                r#"INSERT INTO identity.code_send_counter
                     (identifier_hmac, pepper_version, device_class, window_seconds, window_start, sends)
                   VALUES ($1, $2, $3, $4, $5, 1)
                   ON CONFLICT (identifier_hmac, pepper_version, device_class, window_seconds, window_start)
                   DO UPDATE SET sends = identity.code_send_counter.sends + 1
                   RETURNING sends"#,
                &identifier.bytes[..],
                identifier.pepper_version,
                device_class(class),
                w,
                start
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
            out.push(WindowCount {
                window_seconds: *w,
                window_start: start,
                sends,
            });
        }
        tx.commit().await.map_err(db)?;
        Ok(out)
    }

    async fn send_counts(
        &self,
        identifier: &CodeHmac,
        class: DeviceClass,
        windows: &[i32],
        now: DateTime<Utc>,
    ) -> Result<Vec<WindowCount>, StoreError> {
        let mut out = Vec::with_capacity(windows.len());
        for w in windows {
            let start = window_start(now, *w);
            let sends = sqlx::query_scalar!(
                r#"SELECT COALESCE(sum(sends), 0)::int AS "sends!" FROM identity.code_send_counter
                    WHERE identifier_hmac = $1 AND device_class = $2 AND window_seconds = $3 AND window_start = $4"#,
                &identifier.bytes[..],
                device_class(class),
                w,
                start
            )
            .fetch_one(&self.app)
            .await
            .map_err(db)?;
            out.push(WindowCount {
                window_seconds: *w,
                window_start: start,
                sends,
            });
        }
        Ok(out)
    }

    async fn login_code_placeholder(&self) -> Result<(), StoreError> {
        // The same scoped transaction and a statement on the code table, writing nothing.
        let mut tx = self.scoped(UserId::new_v7()).await?;
        sqlx::query!(
            "SELECT id FROM identity.one_time_code WHERE id = $1 FOR UPDATE",
            Uuid::now_v7()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(())
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

async fn invalidate_signup_codes(
    tx: &mut Transaction<'static, Postgres>,
    invitation_id: InvitationId,
    at: DateTime<Utc>,
) -> Result<(), StoreError> {
    sqlx::query!(
        r#"UPDATE identity.signup_code SET invalidated_at = $2, invalidated_reason = 'invitation_closed'
            WHERE invitation_id = $1 AND consumed_at IS NULL AND invalidated_at IS NULL"#,
        invitation_id.as_uuid(),
        at
    )
    .execute(&mut **tx)
    .await
    .map_err(db)?;
    Ok(())
}

#[allow(clippy::too_many_lines)]
async fn apply(tx: &mut Transaction<'static, Postgres>, write: Write) -> Result<(), StoreError> {
    match write {
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
        Write::InsertUser(u) => {
            sqlx::query!(
                r#"INSERT INTO identity.app_user
                     (id, email, email_normalized, email_verified_at, whatsapp_e164, whatsapp_verified_at, locale, time_zone,
                      is_platform_admin, status, created_at, updated_at)
                   VALUES ($1, $2, $3, $4, $5, $11, $6, $7, $8, $9, $10, $10)"#,
                u.id.as_uuid(),
                u.email.as_entered(),
                u.email.normalized(),
                u.email_verified_at,
                u.whatsapp.as_ref().map(PhoneNumber::as_str),
                u.locale.as_str(),
                u.time_zone.as_str(),
                u.is_platform_admin,
                u.status.as_str(),
                u.created_at,
                u.whatsapp.as_ref().map(|_| u.email_verified_at)
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::SetChannelValue { user_id, value, at } => match value {
            Destination::Email(e) => {
                sqlx::query!(
                    r#"UPDATE identity.app_user
                          SET email = $2, email_normalized = $3, email_verified_at = $4, channel_changed_at = $4, updated_at = $4
                        WHERE id = $1"#,
                    user_id.as_uuid(),
                    e.as_entered(),
                    e.normalized(),
                    at
                )
                .execute(&mut **tx)
                .await
                .map_err(db)?;
            }
            Destination::Whatsapp(p) => {
                sqlx::query!(
                    r#"UPDATE identity.app_user
                          SET whatsapp_e164 = $2, whatsapp_verified_at = $3, channel_changed_at = $3, updated_at = $3
                        WHERE id = $1"#,
                    user_id.as_uuid(),
                    p.as_str(),
                    at
                )
                .execute(&mut **tx)
                .await
                .map_err(db)?;
            }
        },
        Write::InsertDevice(d) => {
            sqlx::query!(
                r#"INSERT INTO identity.known_device (id, user_id, device_key_hash, label, first_seen_at, last_seen_at, last_ip)
                   VALUES ($1, $2, $3, $4, $5, $6, $7::text::inet)"#,
                d.id.as_uuid(),
                d.user_id.as_uuid(),
                &d.key_hash.as_bytes()[..],
                d.label,
                d.first_seen_at,
                d.last_seen_at,
                ip_text(d.last_ip)
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::TouchDevice {
            user_id,
            device_id,
            ip,
            at,
        } => {
            sqlx::query!(
                "UPDATE identity.known_device SET last_seen_at = $3, last_ip = $4::text::inet WHERE user_id = $1 AND id = $2",
                user_id.as_uuid(),
                device_id.as_uuid(),
                at,
                ip_text(ip)
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::RevokeDevice {
            user_id,
            device_id,
            at,
        } => {
            sqlx::query!(
                "UPDATE identity.known_device SET revoked_at = $3 WHERE user_id = $1 AND id = $2 AND revoked_at IS NULL",
                user_id.as_uuid(),
                device_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::RevokeAllDevices { user_id, at } => {
            sqlx::query!(
                "UPDATE identity.known_device SET revoked_at = $2 WHERE user_id = $1 AND revoked_at IS NULL",
                user_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::InsertSession(s) => {
            sqlx::query!(
                r#"INSERT INTO identity.session
                     (id, user_id, device_id, token_hash, is_persistent, login_channel, created_at, login_at,
                      last_activity_at, idle_expires_at, absolute_expires_at, ip_address, user_agent)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $7, $9, $10, $11::text::inet, $12)"#,
                s.id.as_uuid(),
                s.user_id.as_uuid(),
                s.device_id.as_uuid(),
                &s.token_hash.as_bytes()[..],
                s.persistent,
                s.login_channel.as_str(),
                s.last_activity_at,
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
        } => {
            sqlx::query!(
                r#"UPDATE identity.session SET last_activity_at = $2, idle_expires_at = $3
                    WHERE id = $1 AND ended_at IS NULL"#,
                session_id.as_uuid(),
                last_activity_at,
                idle_expires_at
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
        Write::EndDeviceSessions {
            user_id,
            device_id,
            reason,
            at,
        } => {
            sqlx::query!(
                r#"UPDATE identity.session SET ended_at = $3, end_reason = $4
                    WHERE user_id = $1 AND device_id = $2 AND ended_at IS NULL"#,
                user_id.as_uuid(),
                device_id.as_uuid(),
                at,
                reason.as_str()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::InsertOneTimeCode(c) => {
            sqlx::query!(
                r#"INSERT INTO identity.one_time_code
                     (id, user_id, channel_change_id, purpose, channel, destination, action, target_id, session_id, code_role,
                      code_hmac, pepper_version, failed_attempts, created_at, sent_at, expires_at, request_ip, delivery_state)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17::text::inet, $18)"#,
                c.id.as_uuid(),
                c.user_id.as_uuid(),
                c.channel_change_id.map(|x| x.as_uuid()),
                c.purpose.as_str(),
                c.destination.channel().as_str(),
                c.destination.normalized(),
                c.action.map(StepUpAction::as_str),
                c.target_id,
                c.session_id.map(|x| x.as_uuid()),
                c.role.map(CodeRole::as_str),
                &c.state.code_hmac.bytes[..],
                c.state.code_hmac.pepper_version,
                c.state.failed_attempts,
                c.state.created_at,
                c.state.sent_at,
                c.state.expires_at,
                ip_text(c.request_ip),
                c.state.delivery_state.as_str()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::InsertSignupCode(c) => {
            sqlx::query!(
                r#"INSERT INTO identity.signup_code
                     (id, invitation_id, code_hmac, pepper_version, failed_attempts, created_at, sent_at, expires_at,
                      request_ip, delivery_state)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::text::inet, $10)"#,
                c.id.as_uuid(),
                c.invitation_id.as_uuid(),
                &c.state.code_hmac.bytes[..],
                c.state.code_hmac.pepper_version,
                c.state.failed_attempts,
                c.state.created_at,
                c.state.sent_at,
                c.state.expires_at,
                ip_text(c.request_ip),
                c.state.delivery_state.as_str()
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::ResendCode {
            table,
            code_id,
            code_hmac,
            sent_at,
            expires_at,
        } => {
            let rows = match table {
                CodeTable::OneTime => sqlx::query!(
                    r#"UPDATE identity.one_time_code
                          SET code_hmac = $2, pepper_version = $3, sent_at = $4, expires_at = $5, delivery_state = 'pending'
                        WHERE id = $1 AND consumed_at IS NULL AND invalidated_at IS NULL AND expires_at > $4"#,
                    code_id.as_uuid(),
                    &code_hmac.bytes[..],
                    code_hmac.pepper_version,
                    sent_at,
                    expires_at
                )
                .execute(&mut **tx)
                .await,
                CodeTable::Signup => sqlx::query!(
                    r#"UPDATE identity.signup_code
                          SET code_hmac = $2, pepper_version = $3, sent_at = $4, expires_at = $5, delivery_state = 'pending'
                        WHERE id = $1 AND consumed_at IS NULL AND invalidated_at IS NULL AND expires_at > $4"#,
                    code_id.as_uuid(),
                    &code_hmac.bytes[..],
                    code_hmac.pepper_version,
                    sent_at,
                    expires_at
                )
                .execute(&mut **tx)
                .await,
            }
            .map_err(db)?;
            ensure(rows.rows_affected(), StoreConflict::CodeGone)?;
        }
        Write::FailCode { table, code_id } => {
            match table {
                CodeTable::OneTime => sqlx::query!(
                    r#"UPDATE identity.one_time_code
                          SET failed_attempts = failed_attempts + 1,
                              invalidated_at = CASE WHEN failed_attempts + 1 >= 5 THEN now() END,
                              invalidated_reason = CASE WHEN failed_attempts + 1 >= 5 THEN 'attempts' END
                        WHERE id = $1 AND consumed_at IS NULL AND invalidated_at IS NULL"#,
                    code_id.as_uuid()
                )
                .execute(&mut **tx)
                .await,
                CodeTable::Signup => sqlx::query!(
                    r#"UPDATE identity.signup_code
                          SET failed_attempts = failed_attempts + 1,
                              invalidated_at = CASE WHEN failed_attempts + 1 >= 5 THEN now() END,
                              invalidated_reason = CASE WHEN failed_attempts + 1 >= 5 THEN 'attempts' END
                        WHERE id = $1 AND consumed_at IS NULL AND invalidated_at IS NULL"#,
                    code_id.as_uuid()
                )
                .execute(&mut **tx)
                .await,
            }
            .map_err(db)?;
        }
        Write::ConsumeCode {
            table,
            code_id,
            code_hmac,
            at,
        } => {
            let rows = match table {
                CodeTable::OneTime => sqlx::query!(
                    r#"UPDATE identity.one_time_code SET consumed_at = $3
                        WHERE id = $1 AND code_hmac = $2 AND consumed_at IS NULL AND invalidated_at IS NULL AND expires_at > $3"#,
                    code_id.as_uuid(),
                    &code_hmac.bytes[..],
                    at
                )
                .execute(&mut **tx)
                .await,
                CodeTable::Signup => sqlx::query!(
                    r#"UPDATE identity.signup_code SET consumed_at = $3
                        WHERE id = $1 AND code_hmac = $2 AND consumed_at IS NULL AND invalidated_at IS NULL AND expires_at > $3"#,
                    code_id.as_uuid(),
                    &code_hmac.bytes[..],
                    at
                )
                .execute(&mut **tx)
                .await,
            }
            .map_err(db)?;
            ensure(rows.rows_affected(), StoreConflict::CodeGone)?;
        }
        Write::SetDeliveryState {
            table,
            code_id,
            state,
        } => {
            match table {
                CodeTable::OneTime => {
                    sqlx::query!(
                        "UPDATE identity.one_time_code SET delivery_state = $2 WHERE id = $1",
                        code_id.as_uuid(),
                        state.as_str()
                    )
                    .execute(&mut **tx)
                    .await
                }
                CodeTable::Signup => {
                    sqlx::query!(
                        "UPDATE identity.signup_code SET delivery_state = $2 WHERE id = $1",
                        code_id.as_uuid(),
                        state.as_str()
                    )
                    .execute(&mut **tx)
                    .await
                }
            }
            .map_err(db)?;
        }
        Write::InvalidateUserCodes {
            user_id,
            reason,
            at,
        } => {
            sqlx::query!(
                r#"UPDATE identity.one_time_code SET invalidated_at = $3, invalidated_reason = $2
                    WHERE user_id = $1 AND consumed_at IS NULL AND invalidated_at IS NULL"#,
                user_id.as_uuid(),
                reason.as_str(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::InsertChannelChange(c) => {
            sqlx::query!(
                r#"INSERT INTO identity.channel_change
                     (id, user_id, channel, new_value, requested_session_id, status, created_at, expires_at)
                   VALUES ($1, $2, $3, $4, $5, 'awaiting_codes', $6, $7)"#,
                c.id.as_uuid(),
                c.user_id.as_uuid(),
                c.channel().as_str(),
                c.new_value.normalized(),
                c.requested_session_id.as_uuid(),
                c.created_at,
                c.expires_at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::ExpireStaleChannelChange { user_id, at } => {
            sqlx::query!(
                r#"WITH stale AS (
                     UPDATE identity.channel_change SET status = 'expired'
                      WHERE user_id = $1 AND status = 'awaiting_codes' AND expires_at <= $2
                      RETURNING id
                   )
                   UPDATE identity.one_time_code SET invalidated_at = $2, invalidated_reason = 'change_closed'
                    WHERE channel_change_id IN (SELECT id FROM stale)
                      AND consumed_at IS NULL AND invalidated_at IS NULL"#,
                user_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
        }
        Write::AssertChannelChangeAwaiting { change_id, at } => {
            // A no-op update takes the row lock, so a concurrent cancel or expiry is serialized.
            let rows = sqlx::query!(
                r#"UPDATE identity.channel_change SET status = status
                    WHERE id = $1 AND status = 'awaiting_codes' AND expires_at > $2"#,
                change_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(rows.rows_affected(), StoreConflict::ChannelChangeGone)?;
        }
        Write::CompleteChannelChange { change_id, at } => {
            let rows = sqlx::query!(
                r#"UPDATE identity.channel_change SET status = 'completed', completed_at = $2
                    WHERE id = $1 AND status = 'awaiting_codes' AND expires_at > $2"#,
                change_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(rows.rows_affected(), StoreConflict::ChannelChangeGone)?;
        }
        Write::HoldChannelChange {
            change_id,
            hold_until,
            at,
        } => {
            let rows = sqlx::query!(
                r#"UPDATE identity.channel_change SET status = 'on_hold', hold_until = $2
                    WHERE id = $1 AND status = 'awaiting_codes' AND expires_at > $3"#,
                change_id.as_uuid(),
                hold_until,
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(rows.rows_affected(), StoreConflict::ChannelChangeGone)?;
        }
        Write::FinishHeldChannelChange { change_id, at } => {
            let rows = sqlx::query!(
                r#"UPDATE identity.channel_change SET status = 'completed', completed_at = $2
                    WHERE id = $1 AND status = 'on_hold'"#,
                change_id.as_uuid(),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(rows.rows_affected(), StoreConflict::ChannelChangeGone)?;
        }
        Write::CancelChannelChange {
            change_id,
            by_session,
            at,
        } => {
            let rows = sqlx::query!(
                r#"UPDATE identity.channel_change SET status = 'cancelled', cancelled_at = $3, cancelled_by_session_id = $2
                    WHERE id = $1 AND (status = 'on_hold' OR (status = 'awaiting_codes' AND expires_at > $3))"#,
                change_id.as_uuid(),
                by_session.map(|x| x.as_uuid()),
                at
            )
            .execute(&mut **tx)
            .await
            .map_err(db)?;
            ensure(rows.rows_affected(), StoreConflict::ChannelChangeGone)?;
            sqlx::query!(
                r#"UPDATE identity.one_time_code SET invalidated_at = $2, invalidated_reason = 'change_closed'
                    WHERE channel_change_id = $1 AND consumed_at IS NULL AND invalidated_at IS NULL"#,
                change_id.as_uuid(),
                at
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
            invalidate_signup_codes(tx, invitation_id, at).await?;
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
            invalidate_signup_codes(tx, invitation_id, at).await?;
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
