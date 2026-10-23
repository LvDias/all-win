//! `ProfileStore` on `accounts.default_bracket_profile` / `accounts.account_bracket_profile`.
//! Every statement runs in a transaction with the caller's `app.user_id` (RLS, fail-closed) and
//! also filters by `user_id` explicitly. Writes are conditional (the values or `row_version` read
//! by the use case) and append their audit event to the outbox in the same transaction.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
use uuid::Uuid;

use accounts_application::enterprise::profile::{
    AccountBracketProfile, CopiedFrom, DefaultProfile, DistanceProfile, DistanceUnit, ProfileOrigin,
};
use accounts_application::enterprise::{AccountId, AccountKind, AccountName, SizeAmount};
use accounts_application::profiles::ports::{
    AccountFacts, AccountProfileWrite, DefaultWrite, LiveAccountProfiles, ProfileStore,
    ProfileStoreError,
};
use shared_kernel::UserId;

use super::outbox;
use crate::store::set_user;

#[derive(Clone)]
pub struct PgProfileStore {
    pool: PgPool,
}

impl PgProfileStore {
    /// `pool` connects as `allwin_app` (RLS enforced).
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn begin_for(
        &self,
        user_id: UserId,
    ) -> Result<Transaction<'static, Postgres>, ProfileStoreError> {
        let mut tx = self.pool.begin().await.map_err(other)?;
        set_user(&mut tx, user_id)
            .await
            .map_err(|e| ProfileStoreError::Other(e.to_string()))?;
        Ok(tx)
    }
}

/// A live account of the caller (`None`: unknown, archived or another user's). `lock` takes a
/// share lock, so the account cannot be archived while its profile is written.
pub(crate) async fn account_facts(
    conn: &mut PgConnection,
    user_id: UserId,
    account_id: AccountId,
    lock: bool,
) -> Result<Option<AccountFacts>, ProfileStoreError> {
    let row = if lock {
        sqlx::query!(
            r#"SELECT kind, size_amount, display_name FROM accounts.account
               WHERE user_id = $1 AND id = $2 AND archived_at IS NULL FOR SHARE"#,
            user_id.as_uuid(),
            account_id.as_uuid(),
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(other)?
        .map(|r| (r.kind, r.size_amount, r.display_name))
    } else {
        sqlx::query!(
            r#"SELECT kind, size_amount, display_name FROM accounts.account
               WHERE user_id = $1 AND id = $2 AND archived_at IS NULL"#,
            user_id.as_uuid(),
            account_id.as_uuid(),
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(other)?
        .map(|r| (r.kind, r.size_amount, r.display_name))
    };
    row.map(|(kind, size, name)| {
        Ok(AccountFacts {
            kind: AccountKind::parse(&kind).ok_or_else(|| corrupt("account kind"))?,
            size_amount: SizeAmount::new(size).map_err(|_| corrupt("account size"))?,
            display_name: AccountName::parse(&name).map_err(|_| corrupt("account name"))?,
        })
    })
    .transpose()
}

struct ProfileRow {
    account_id: Uuid,
    instrument_id: Uuid,
    distance_unit: String,
    stop_distance: Decimal,
    target_distance: Decimal,
    copied_from_size: Option<Decimal>,
    origin: String,
    copied_at: Option<DateTime<Utc>>,
    customized: bool,
    row_version: i32,
    updated_at: DateTime<Utc>,
}

fn distance(
    unit: &str,
    stop: Decimal,
    target: Decimal,
) -> Result<DistanceProfile, ProfileStoreError> {
    let unit = DistanceUnit::parse(unit).ok_or_else(|| corrupt("distance unit"))?;
    DistanceProfile::from_decimals(unit, stop, target).map_err(|_| corrupt("distance"))
}

fn to_profile(r: &ProfileRow) -> Result<AccountBracketProfile, ProfileStoreError> {
    let copied_from = match (r.copied_from_size, r.copied_at) {
        (Some(size), Some(at)) => Some(CopiedFrom {
            size_amount: SizeAmount::new(size).map_err(|_| corrupt("copied_from_size"))?,
            at,
        }),
        (None, None) => None,
        _ => return Err(corrupt("provenance")),
    };
    AccountBracketProfile::restore_from_storage(
        r.instrument_id,
        distance(&r.distance_unit, r.stop_distance, r.target_distance)?,
        ProfileOrigin::parse(&r.origin).ok_or_else(|| corrupt("origin"))?,
        copied_from,
        r.customized,
        r.row_version,
        r.updated_at,
    )
    .map_err(|_| corrupt("provenance"))
}

async fn profiles_of(
    conn: &mut PgConnection,
    user_id: UserId,
    account_ids: &[Uuid],
    instrument_ids: Option<&[Uuid]>,
) -> Result<Vec<ProfileRow>, ProfileStoreError> {
    sqlx::query_as!(
        ProfileRow,
        r#"SELECT account_id, instrument_id, distance_unit, stop_distance, target_distance,
                  copied_from_size, origin, copied_at, customized, row_version, updated_at
           FROM accounts.account_bracket_profile
           WHERE user_id = $1 AND account_id = ANY($2)
             AND ($3::uuid[] IS NULL OR instrument_id = ANY($3))"#,
        user_id.as_uuid(),
        account_ids,
        instrument_ids,
    )
    .fetch_all(conn)
    .await
    .map_err(other)
}

/// Inserts one account profile; `false` when the account already has one for the instrument.
async fn insert_profile(
    conn: &mut PgConnection,
    user_id: UserId,
    account_id: AccountId,
    p: &AccountBracketProfile,
) -> Result<bool, sqlx::Error> {
    let d = p.profile();
    let inserted = sqlx::query!(
        r#"INSERT INTO accounts.account_bracket_profile
             (user_id, account_id, instrument_id, distance_unit, stop_distance, target_distance,
              copied_from_size, origin, copied_at, customized, row_version, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
           ON CONFLICT (account_id, instrument_id) DO NOTHING"#,
        user_id.as_uuid(),
        account_id.as_uuid(),
        p.instrument_id(),
        d.unit().as_str(),
        d.stop(),
        d.target(),
        p.copied_from().map(|c| c.size_amount.value()),
        p.origin().as_str(),
        p.copied_from().map(|c| c.at),
        p.customized(),
        p.row_version(),
        p.updated_at(),
    )
    .execute(conn)
    .await?;
    Ok(inserted.rows_affected() == 1)
}

/// The initial copies of a new account, in its create transaction (S5 store), each audited as
/// `profiles.account_changed` (`initial_copy`).
pub(crate) async fn insert_initial_copies(
    conn: &mut PgConnection,
    user_id: UserId,
    account_id: AccountId,
    account_name: &AccountName,
    profiles: &[AccountBracketProfile],
) -> Result<(), sqlx::Error> {
    use accounts_application::profiles::{ProfileAction, ProfileEvent};
    for p in profiles {
        if !insert_profile(&mut *conn, user_id, account_id, p).await? {
            return Err(sqlx::Error::Protocol(
                "new account already has a profile".into(),
            ));
        }
        let event = ProfileEvent::AccountChanged {
            at: p.updated_at(),
            account_id,
            account_name: account_name.clone(),
            instrument_id: p.instrument_id(),
            action: ProfileAction::InitialCopy,
            copied_from_size: p.copied_from().map(|c| c.size_amount),
            previous: None,
            new: *p.profile(),
        };
        outbox::insert(&mut *conn, user_id, &event).await?;
    }
    Ok(())
}

#[async_trait]
impl ProfileStore for PgProfileStore {
    async fn defaults(
        &self,
        user_id: UserId,
        instrument_id: Option<Uuid>,
    ) -> Result<Vec<DefaultProfile>, ProfileStoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query!(
            r#"SELECT size_amount, instrument_id, distance_unit, stop_distance, target_distance, updated_at
               FROM accounts.default_bracket_profile
               WHERE user_id = $1 AND ($2::uuid IS NULL OR instrument_id = $2)
               ORDER BY instrument_id, size_amount"#,
            user_id.as_uuid(),
            instrument_id,
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(other)?;
        tx.commit().await.map_err(other)?;
        rows.into_iter()
            .map(|r| {
                Ok(DefaultProfile {
                    instrument_id: r.instrument_id,
                    size_amount: SizeAmount::new(r.size_amount)
                        .map_err(|_| corrupt("size_amount"))?,
                    profile: distance(&r.distance_unit, r.stop_distance, r.target_distance)?,
                    updated_at: r.updated_at,
                })
            })
            .collect()
    }

    async fn put_default(&self, w: DefaultWrite) -> Result<(), ProfileStoreError> {
        let mut tx = self.begin_for(w.user_id).await?;
        let d = &w.default;
        let p = &d.profile;
        let written = match &w.expected {
            None => sqlx::query!(
                r#"INSERT INTO accounts.default_bracket_profile
                     (user_id, size_amount, instrument_id, distance_unit, stop_distance, target_distance, updated_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7)
                   ON CONFLICT (user_id, size_amount, instrument_id) DO NOTHING"#,
                w.user_id.as_uuid(),
                d.size_amount.value(),
                d.instrument_id,
                p.unit().as_str(),
                p.stop(),
                p.target(),
                d.updated_at,
            )
            .execute(&mut *tx)
            .await
            .map_err(other)?,
            Some(e) => sqlx::query!(
                r#"UPDATE accounts.default_bracket_profile
                   SET distance_unit = $4, stop_distance = $5, target_distance = $6, updated_at = $7
                   WHERE user_id = $1 AND size_amount = $2 AND instrument_id = $3
                     AND distance_unit = $8 AND stop_distance = $9 AND target_distance = $10"#,
                w.user_id.as_uuid(),
                d.size_amount.value(),
                d.instrument_id,
                p.unit().as_str(),
                p.stop(),
                p.target(),
                d.updated_at,
                e.unit().as_str(),
                e.stop(),
                e.target(),
            )
            .execute(&mut *tx)
            .await
            .map_err(other)?,
        };
        if written.rows_affected() != 1 {
            return Err(ProfileStoreError::Raced);
        }
        outbox::insert(&mut tx, w.user_id, &w.event)
            .await
            .map_err(other)?;
        tx.commit().await.map_err(other)
    }

    async fn account_profiles(
        &self,
        user_id: UserId,
        account_id: AccountId,
    ) -> Result<Option<(AccountFacts, Vec<AccountBracketProfile>)>, ProfileStoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let Some(facts) = account_facts(&mut tx, user_id, account_id, false).await? else {
            return Ok(None);
        };
        let rows = profiles_of(&mut tx, user_id, &[account_id.as_uuid()], None).await?;
        tx.commit().await.map_err(other)?;
        let profiles = rows.iter().map(to_profile).collect::<Result<_, _>>()?;
        Ok(Some((facts, profiles)))
    }

    async fn write_account_profile(&self, w: AccountProfileWrite) -> Result<(), ProfileStoreError> {
        let mut tx = self.begin_for(w.user_id).await?;
        if account_facts(&mut tx, w.user_id, w.account_id, true)
            .await?
            .is_none()
        {
            return Err(ProfileStoreError::AccountNotFound);
        }
        let p = &w.profile;
        let written = match w.expected_row_version {
            None => insert_profile(&mut tx, w.user_id, w.account_id, p)
                .await
                .map_err(other)?,
            Some(expected) => {
                let d = p.profile();
                sqlx::query!(
                    r#"UPDATE accounts.account_bracket_profile
                       SET distance_unit = $5, stop_distance = $6, target_distance = $7,
                           copied_from_size = $8, origin = $9, copied_at = $10, customized = $11,
                           row_version = $12, updated_at = $13
                       WHERE user_id = $1 AND account_id = $2 AND instrument_id = $3 AND row_version = $4"#,
                    w.user_id.as_uuid(),
                    w.account_id.as_uuid(),
                    p.instrument_id(),
                    expected,
                    d.unit().as_str(),
                    d.stop(),
                    d.target(),
                    p.copied_from().map(|c| c.size_amount.value()),
                    p.origin().as_str(),
                    p.copied_from().map(|c| c.at),
                    p.customized(),
                    p.row_version(),
                    p.updated_at(),
                )
                .execute(&mut *tx)
                .await
                .map_err(other)?
                .rows_affected()
                    == 1
            }
        };
        if !written {
            return Err(ProfileStoreError::Raced);
        }
        outbox::insert(&mut tx, w.user_id, &w.event)
            .await
            .map_err(other)?;
        tx.commit().await.map_err(other)
    }

    async fn live_profiles(
        &self,
        user_id: UserId,
        account_ids: &[AccountId],
        instrument_ids: &[Uuid],
    ) -> Result<Vec<LiveAccountProfiles>, ProfileStoreError> {
        let ids: Vec<Uuid> = account_ids.iter().map(AccountId::as_uuid).collect();
        let mut tx = self.begin_for(user_id).await?;
        let live = sqlx::query_scalar!(
            r#"SELECT id FROM accounts.account
               WHERE user_id = $1 AND id = ANY($2) AND archived_at IS NULL"#,
            user_id.as_uuid(),
            &ids,
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(other)?;
        let rows = profiles_of(&mut tx, user_id, &live, Some(instrument_ids)).await?;
        tx.commit().await.map_err(other)?;
        live.into_iter()
            .map(|id| {
                Ok(LiveAccountProfiles {
                    account_id: AccountId::from_uuid(id),
                    profiles: rows
                        .iter()
                        .filter(|r| r.account_id == id)
                        .map(to_profile)
                        .collect::<Result<_, _>>()?,
                })
            })
            .collect()
    }
}

fn corrupt(what: &str) -> ProfileStoreError {
    ProfileStoreError::Other(format!("profile row: invalid {what}"))
}

/// Database errors carry statement text and constraint names, never bound values.
fn other(e: sqlx::Error) -> ProfileStoreError {
    ProfileStoreError::Other(e.to_string())
}
