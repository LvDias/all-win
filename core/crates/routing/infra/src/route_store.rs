//! Routes (`routing.subscription`, wave1-data-model §5.5): one user-scoped transaction per
//! call, with the `routing.subscription_changed` audit event appended to the outbox in the
//! same transaction. Follower subscriptions (`master_account`, S19) are not S7 routes and are
//! never read or written here.

use async_trait::async_trait;
use routing_application::enterprise::{AccountId, RouteId, RouteSignalType, SignalTypes, SourceId};
use routing_application::routes::{
    CreateOutcome, PortError, RouteChange, RouteFilter, RouteRecord, RouteStore, RouteUpdate,
    UpdateOutcome,
};
use shared_kernel::UserId;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgRouteStore {
    pool: PgPool,
}

impl PgRouteStore {
    /// `pool` connects as `allwin_app` (RLS enforced).
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn begin_for(&self, user: UserId) -> Result<Transaction<'static, Postgres>, PortError> {
        let mut tx = self.pool.begin().await.map_err(port_error)?;
        sqlx::query!(
            "SELECT set_config('app.user_id', $1, true)",
            user.as_uuid().to_string()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(port_error)?;
        Ok(tx)
    }
}

struct Row {
    id: Uuid,
    user_id: Uuid,
    source_id: Uuid,
    account_id: Uuid,
    signal_types: Vec<String>,
    is_enabled: bool,
    row_version: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<Row> for RouteRecord {
    type Error = PortError;

    fn try_from(r: Row) -> Result<Self, PortError> {
        let types = r
            .signal_types
            .iter()
            .map(|t| RouteSignalType::parse(t))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| PortError("route with a non-S7 signal type".into()))?;
        Ok(Self {
            id: RouteId::from_uuid(r.id),
            user_id: UserId::from_uuid(r.user_id),
            source_id: SourceId::from_uuid(r.source_id),
            account_id: AccountId::from_uuid(r.account_id),
            signal_types: SignalTypes::new(types).map_err(|e| PortError(e.to_string()))?,
            is_enabled: r.is_enabled,
            row_version: r.row_version,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
    }
}

fn type_names(types: &SignalTypes) -> Vec<String> {
    types.iter().map(|t| t.as_str().to_owned()).collect()
}

#[async_trait]
impl RouteStore for PgRouteStore {
    async fn list(&self, user: UserId, f: RouteFilter) -> Result<Vec<RouteRecord>, PortError> {
        let mut tx = self.begin_for(user).await?;
        let rows = sqlx::query_as!(
            Row,
            r#"SELECT id, user_id, source_id, account_id, signal_types, is_enabled, row_version,
                      created_at, updated_at
               FROM routing.subscription
               WHERE user_id = $1
                 AND ($2::uuid IS NULL OR source_id = $2)
                 AND ($3::uuid IS NULL OR account_id = $3)
                 AND NOT ('master_account' = ANY (signal_types))
               ORDER BY id"#,
            user.as_uuid(),
            f.source_id.map(|s| s.as_uuid()),
            f.account_id.map(|a| a.as_uuid()),
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(port_error)?;
        tx.commit().await.map_err(port_error)?;
        rows.into_iter().map(RouteRecord::try_from).collect()
    }

    async fn get(&self, user: UserId, id: RouteId) -> Result<Option<RouteRecord>, PortError> {
        let mut tx = self.begin_for(user).await?;
        let row = sqlx::query_as!(
            Row,
            r#"SELECT id, user_id, source_id, account_id, signal_types, is_enabled, row_version,
                      created_at, updated_at
               FROM routing.subscription
               WHERE user_id = $1 AND id = $2 AND NOT ('master_account' = ANY (signal_types))"#,
            user.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(port_error)?;
        tx.commit().await.map_err(port_error)?;
        row.map(RouteRecord::try_from).transpose()
    }

    async fn create(
        &self,
        route: &RouteRecord,
        change: &RouteChange,
    ) -> Result<CreateOutcome, PortError> {
        let mut tx = self.begin_for(route.user_id).await?;
        // The natural key decides a race: the loser takes the DO NOTHING path.
        let inserted = sqlx::query_scalar!(
            r#"INSERT INTO routing.subscription
                 (id, user_id, source_id, account_id, signal_types, is_enabled, row_version,
                  created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $7)
               ON CONFLICT (user_id, source_id, account_id) DO NOTHING
               RETURNING id"#,
            route.id.as_uuid(),
            route.user_id.as_uuid(),
            route.source_id.as_uuid(),
            route.account_id.as_uuid(),
            &type_names(&route.signal_types),
            route.is_enabled,
            route.created_at,
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(port_error)?;
        if inserted.is_none() {
            return Ok(CreateOutcome::Exists);
        }
        append_change(&mut tx, route.user_id, change).await?;
        tx.commit().await.map_err(port_error)?;
        Ok(CreateOutcome::Created)
    }

    async fn update(
        &self,
        u: &RouteUpdate,
        change: Option<&RouteChange>,
    ) -> Result<UpdateOutcome, PortError> {
        let mut tx = self.begin_for(u.user_id).await?;
        let row = sqlx::query_as!(
            Row,
            r#"UPDATE routing.subscription
               SET signal_types = $4, is_enabled = $5, row_version = row_version + 1, updated_at = $6
               WHERE user_id = $1 AND id = $2 AND row_version = $3
                 AND NOT ('master_account' = ANY (signal_types))
               RETURNING id, user_id, source_id, account_id, signal_types, is_enabled, row_version,
                         created_at, updated_at"#,
            u.user_id.as_uuid(),
            u.id.as_uuid(),
            u.expected_version,
            &type_names(&u.signal_types),
            u.is_enabled,
            u.at,
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(port_error)?;
        let Some(row) = row else {
            let exists = sqlx::query_scalar!(
                r#"SELECT EXISTS (SELECT 1 FROM routing.subscription
                                  WHERE user_id = $1 AND id = $2
                                    AND NOT ('master_account' = ANY (signal_types))) AS "exists!""#,
                u.user_id.as_uuid(),
                u.id.as_uuid(),
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(port_error)?;
            return Ok(if exists {
                UpdateOutcome::VersionConflict
            } else {
                UpdateOutcome::NotFound
            });
        };
        if let Some(change) = change {
            append_change(&mut tx, u.user_id, change).await?;
        }
        tx.commit().await.map_err(port_error)?;
        Ok(UpdateOutcome::Updated(RouteRecord::try_from(row)?))
    }

    async fn delete(
        &self,
        user: UserId,
        id: RouteId,
        change: &RouteChange,
    ) -> Result<bool, PortError> {
        let mut tx = self.begin_for(user).await?;
        let deleted = sqlx::query!(
            r#"DELETE FROM routing.subscription
               WHERE user_id = $1 AND id = $2 AND NOT ('master_account' = ANY (signal_types))"#,
            user.as_uuid(),
            id.as_uuid(),
        )
        .execute(&mut *tx)
        .await
        .map_err(port_error)?
        .rows_affected();
        if deleted == 0 {
            return Ok(false);
        }
        append_change(&mut tx, user, change).await?;
        tx.commit().await.map_err(port_error)?;
        Ok(true)
    }
}

async fn append_change(
    tx: &mut Transaction<'static, Postgres>,
    user: UserId,
    change: &RouteChange,
) -> Result<(), PortError> {
    sqlx::query!(
        r#"INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload)
           VALUES ($1, $2, $3, $4, $5)"#,
        Uuid::now_v7(),
        user.as_uuid(),
        RouteChange::EVENT_TYPE,
        change.at,
        change.payload(),
    )
    .execute(&mut **tx)
    .await
    .map_err(port_error)?;
    Ok(())
}

/// Database errors carry statement text and constraint names, never bound values.
fn port_error(e: sqlx::Error) -> PortError {
    PortError(e.to_string())
}
