//! `GatewayStore` on PostgreSQL. Every user-owned statement runs in a transaction with
//! `app.user_id` set, so RLS is the second barrier behind the explicit `user_id` filters.

use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use gateway_application::events::GatewayEvent;
use gateway_application::ports::{
    CommitOutcome, GatewayStore, NewCommand, OutboundMessage, StoreError, StoredCommand,
    UnitOfWork, VenueAccount, VisiblePlatformAccount,
};
use gateway_enterprise::command::{Command, CommandKind, CommandState, PositionEffect};
use gateway_enterprise::ids::{AccountId, CommandId, ConnectionId, VenueAccountId};
use gateway_enterprise::instrument::Instrument;
use gateway_enterprise::protocol::ProtocolVersion;
use gateway_enterprise::report::{
    ChildRole, FillFact, FillOperation, Origin, ReportFact, ReportStatus,
};
use gateway_enterprise::sync::{
    AccountSync, ReasonDetails, ReasonEntry, SyncState, parse_dedupe_key,
};
use rust_decimal::Decimal;
use shared_kernel::UserId;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgGatewayStore {
    /// `allwin_app`: user-scoped work (RLS).
    app: PgPool,
}

fn db(e: sqlx::Error) -> StoreError {
    // 23514 check_violation (e.g. a token-shaped value): deterministic, retrying cannot help.
    if e.as_database_error().and_then(|d| d.code()).as_deref() == Some("23514") {
        return StoreError::Rejected(e.to_string());
    }
    StoreError::Other(e.to_string())
}

fn corrupt(what: &str) -> StoreError {
    StoreError::Other(format!("unexpected stored value: {what}"))
}

/// `YYYY-MM` ↔ the first day of that month (`contract_month date`).
fn month_date(i: &Instrument) -> Result<NaiveDate, StoreError> {
    NaiveDate::parse_from_str(&format!("{}-01", i.contract_month()), "%Y-%m-%d")
        .map_err(|_| corrupt("contract_month"))
}

fn instrument(symbol: &str, month: NaiveDate) -> Result<Instrument, StoreError> {
    Instrument::new(symbol, month.format("%Y-%m").to_string()).map_err(|_| corrupt("instrument"))
}

/// Column lists as literals, so every statement stays a `&'static str` (no SQL is built at runtime).
macro_rules! command_columns {
    () => {
        "id, user_id, instruction_id, account_id, connection_id, step, kind, position_effect, \
         instrument_symbol, contract_month, msg_id, frame, depends_on, expires_at, state, created_at, sent_at"
    };
}

macro_rules! report_columns {
    () => {
        "seq, command_id, origin, child_role, status, reason_code, fill_id, fill_operation, \
         fill_qty, position_after, instrument_symbol, contract_month"
    };
}

impl PgGatewayStore {
    pub fn new(app: PgPool) -> Self {
        Self { app }
    }

    async fn begin_for(&self, user: UserId) -> Result<Transaction<'static, Postgres>, StoreError> {
        let mut tx = self.app.begin().await.map_err(db)?;
        sqlx::query("SELECT set_config('app.user_id', $1, true)")
            .bind(user.as_uuid().to_string())
            .execute(&mut *tx)
            .await
            .map_err(db)?;
        Ok(tx)
    }

    async fn insert_events(
        tx: &mut Transaction<'static, Postgres>,
        user: UserId,
        events: &[GatewayEvent],
    ) -> Result<(), StoreError> {
        for e in events {
            sqlx::query("INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload) VALUES ($1, $2, $3, $4, $5)")
                .bind(e.id)
                .bind(user.as_uuid())
                .bind(e.event_type)
                .bind(e.occurred_at)
                .bind(&e.payload)
                .execute(&mut **tx)
                .await
                .map_err(db)?;
        }
        Ok(())
    }
}

fn parse_kind(s: &str) -> Result<CommandKind, StoreError> {
    Ok(match s {
        "place" => CommandKind::Place,
        "modify" => CommandKind::Modify,
        "cancel" => CommandKind::Cancel,
        "flatten" => CommandKind::Flatten,
        _ => return Err(corrupt("command.kind")),
    })
}

fn kind_str(k: CommandKind) -> &'static str {
    match k {
        CommandKind::Place => "place",
        CommandKind::Modify => "modify",
        CommandKind::Cancel => "cancel",
        CommandKind::Flatten => "flatten",
    }
}

fn effect_str(e: PositionEffect) -> &'static str {
    match e {
        PositionEffect::Open => "open",
        PositionEffect::Reduce => "reduce",
    }
}

fn command_row(r: &PgRow) -> Result<StoredCommand, StoreError> {
    let effect = match r
        .try_get::<Option<String>, _>("position_effect")
        .map_err(db)?
        .as_deref()
    {
        None => None,
        Some("open") => Some(PositionEffect::Open),
        Some("reduce") => Some(PositionEffect::Reduce),
        Some(_) => return Err(corrupt("position_effect")),
    };
    let state: String = r.try_get("state").map_err(db)?;
    let kind: String = r.try_get("kind").map_err(db)?;
    Ok(StoredCommand {
        command: Command {
            id: CommandId::from_uuid(r.try_get("id").map_err(db)?),
            kind: parse_kind(&kind)?,
            position_effect: effect,
            depends_on: r
                .try_get::<Option<Uuid>, _>("depends_on")
                .map_err(db)?
                .map(CommandId::from_uuid),
            expires_at: r.try_get("expires_at").map_err(db)?,
            created_at: r.try_get("created_at").map_err(db)?,
            state: CommandState::parse(&state).ok_or_else(|| corrupt("command.state"))?,
            sent_at: r.try_get("sent_at").map_err(db)?,
        },
        instruction_id: r.try_get("instruction_id").map_err(db)?,
        step: r.try_get("step").map_err(db)?,
        user_id: UserId::from_uuid(r.try_get("user_id").map_err(db)?),
        account_id: AccountId::from_uuid(r.try_get("account_id").map_err(db)?),
        connection_id: ConnectionId::from_uuid(r.try_get("connection_id").map_err(db)?),
        instrument: instrument(
            &r.try_get::<String, _>("instrument_symbol").map_err(db)?,
            r.try_get("contract_month").map_err(db)?,
        )?,
        msg_id: r.try_get("msg_id").map_err(db)?,
        frame: r.try_get("frame").map_err(db)?,
    })
}

fn report_row(r: &PgRow) -> Result<(Option<Instrument>, ReportFact), StoreError> {
    let origin = match r.try_get::<String, _>("origin").map_err(db)?.as_str() {
        "all_win" => Origin::AllWin,
        "local_enforcement" => Origin::LocalEnforcement,
        "external" => Origin::External,
        _ => return Err(corrupt("origin")),
    };
    let child_role = match r
        .try_get::<Option<String>, _>("child_role")
        .map_err(db)?
        .as_deref()
    {
        None => None,
        Some("entry") => Some(ChildRole::Entry),
        Some("stop") => Some(ChildRole::Stop),
        Some("target") => Some(ChildRole::Target),
        Some("exit") => Some(ChildRole::Exit),
        Some("flatten") => Some(ChildRole::Flatten),
        Some(_) => return Err(corrupt("child_role")),
    };
    let status = match r.try_get::<String, _>("status").map_err(db)?.as_str() {
        "received" => ReportStatus::Received,
        "working" => ReportStatus::Working,
        "partially_filled" => ReportStatus::PartiallyFilled,
        "filled" => ReportStatus::Filled,
        "cancelled" => ReportStatus::Cancelled,
        "rejected" => ReportStatus::Rejected,
        "expired" => ReportStatus::Expired,
        "duplicate" => ReportStatus::Duplicate,
        "unknown" => ReportStatus::Unknown,
        _ => return Err(corrupt("status")),
    };
    let fill = match r.try_get::<Option<String>, _>("fill_id").map_err(db)? {
        None => None,
        Some(fill_id) => {
            let operation = match r
                .try_get::<Option<String>, _>("fill_operation")
                .map_err(db)?
                .as_deref()
            {
                Some("add") => FillOperation::Add,
                Some("amend") => FillOperation::Amend,
                Some("remove") => FillOperation::Remove,
                _ => return Err(corrupt("fill_operation")),
            };
            let qty: i32 = r.try_get("fill_qty").map_err(db)?;
            Some(FillFact {
                fill_id,
                operation,
                quantity: u32::try_from(qty).map_err(|_| corrupt("fill_qty"))?,
            })
        }
    };
    let symbol: Option<String> = r.try_get("instrument_symbol").map_err(db)?;
    let month: Option<NaiveDate> = r.try_get("contract_month").map_err(db)?;
    let inst = match (symbol, month) {
        (Some(s), Some(m)) => Some(instrument(&s, m)?),
        _ => None,
    };
    Ok((
        inst,
        ReportFact {
            seq: r.try_get("seq").map_err(db)?,
            command_id: r
                .try_get::<Option<Uuid>, _>("command_id")
                .map_err(db)?
                .map(CommandId::from_uuid),
            origin,
            child_role,
            status,
            reason_code: r.try_get("reason_code").map_err(db)?,
            fill,
            position_after: r
                .try_get::<Option<i32>, _>("position_after")
                .map_err(db)?
                .map(i64::from),
        },
    ))
}

fn venue_row(r: &PgRow) -> Result<VenueAccount, StoreError> {
    Ok(VenueAccount {
        id: VenueAccountId::from_uuid(r.try_get("id").map_err(db)?),
        platform_account: r.try_get("platform_account").map_err(db)?,
        kind: r.try_get("kind").map_err(db)?,
        oco_location: r.try_get("oco_location").map_err(db)?,
        previous_kind: None,
    })
}

fn decimal(s: &str) -> Result<Decimal, StoreError> {
    Decimal::from_str(s).map_err(|_| StoreError::Other("invalid decimal".into()))
}

#[async_trait]
impl GatewayStore for PgGatewayStore {
    async fn upsert_venue_accounts(
        &self,
        user_id: UserId,
        connection: ConnectionId,
        accounts: &[VisiblePlatformAccount],
        seen_at: DateTime<Utc>,
    ) -> Result<Vec<VenueAccount>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let mut out = Vec::with_capacity(accounts.len());
        for a in accounts {
            // The CTE reads the row as it was before this statement: the previous kind.
            let row = sqlx::query(
                "WITH old AS (SELECT kind FROM gateway.venue_account WHERE connection_id = $2 AND platform_account = $3)
                 INSERT INTO gateway.venue_account (id, user_id, connection_id, platform_account, kind, oco_location, first_seen_at, last_seen_at)
                 VALUES ($1, $4, $2, $3, $5, $6, $7, $7)
                 ON CONFLICT (connection_id, platform_account)
                 DO UPDATE SET kind = EXCLUDED.kind, oco_location = EXCLUDED.oco_location, last_seen_at = EXCLUDED.last_seen_at
                 RETURNING id, kind, oco_location, (SELECT kind FROM old) AS previous_kind",
            )
            .bind(Uuid::now_v7())
            .bind(connection.as_uuid())
            .bind(&a.platform_account)
            .bind(user_id.as_uuid())
            .bind(&a.kind)
            .bind(&a.oco_location)
            .bind(seen_at)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
            let kind: String = row.try_get("kind").map_err(db)?;
            let previous: Option<String> = row.try_get("previous_kind").map_err(db)?;
            out.push(VenueAccount {
                id: VenueAccountId::from_uuid(row.try_get("id").map_err(db)?),
                platform_account: a.platform_account.clone(),
                previous_kind: previous.filter(|p| *p != kind),
                kind,
                oco_location: row.try_get("oco_location").map_err(db)?,
            });
        }
        tx.commit().await.map_err(db)?;
        Ok(out)
    }

    async fn insert_commands(
        &self,
        user_id: UserId,
        commands: &[NewCommand],
    ) -> Result<usize, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let mut new = 0;
        for c in commands {
            let s = &c.stored;
            let done = sqlx::query(
                "INSERT INTO gateway.command (id, user_id, instruction_id, account_id, connection_id, step, kind,
                   position_effect, instrument_symbol, contract_month, msg_id, protocol_version, frame, depends_on,
                   expires_at, state, payload, created_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, 'pending', $16, $17)
                 ON CONFLICT DO NOTHING",
            )
            .bind(s.command.id.as_uuid())
            .bind(user_id.as_uuid())
            .bind(s.instruction_id)
            .bind(s.account_id.as_uuid())
            .bind(s.connection_id.as_uuid())
            .bind(&s.step)
            .bind(kind_str(s.command.kind))
            .bind(s.command.position_effect.map(effect_str))
            .bind(s.instrument.symbol())
            .bind(month_date(&s.instrument)?)
            .bind(s.msg_id)
            .bind(gateway_application::wire::COMMAND_ENVELOPE_VERSION.to_string())
            .bind(&s.frame)
            .bind(s.command.depends_on.map(|d| d.as_uuid()))
            .bind(s.command.expires_at)
            .bind(&c.payload)
            .bind(s.command.created_at)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
            new += usize::try_from(done.rows_affected()).unwrap_or(0);
        }
        tx.commit().await.map_err(db)?;
        Ok(new)
    }

    async fn open_commands(
        &self,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<StoredCommand>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(concat!(
            "SELECT ",
            command_columns!(),
            " FROM gateway.command
             WHERE user_id = $1 AND connection_id = $2 AND state IN ('pending','sent')
             ORDER BY created_at, msg_id"
        ))
        .bind(user_id.as_uuid())
        .bind(connection.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.iter().map(command_row).collect()
    }

    async fn command(
        &self,
        user_id: UserId,
        id: CommandId,
    ) -> Result<Option<StoredCommand>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let row = sqlx::query(concat!(
            "SELECT ",
            command_columns!(),
            " FROM gateway.command WHERE user_id = $1 AND id = $2"
        ))
        .bind(user_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.as_ref().map(command_row).transpose()
    }

    async fn in_flight(
        &self,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<StoredCommand>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(concat!("SELECT ", command_columns!(), " FROM gateway.command
             WHERE user_id = $1 AND connection_id = $2 AND state IN ('sent','acked') ORDER BY created_at, msg_id"))
        .bind(user_id.as_uuid())
        .bind(connection.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.iter().map(command_row).collect()
    }

    async fn venue_account(
        &self,
        user_id: UserId,
        id: VenueAccountId,
    ) -> Result<Option<(ConnectionId, VenueAccount)>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let row = sqlx::query(
            "SELECT connection_id, platform_account, kind, oco_location FROM gateway.venue_account WHERE user_id = $1 AND id = $2",
        )
        .bind(user_id.as_uuid())
        .bind(id.as_uuid())
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.map(|r| {
            Ok((
                ConnectionId::from_uuid(r.try_get("connection_id").map_err(db)?),
                VenueAccount {
                    id,
                    platform_account: r.try_get("platform_account").map_err(db)?,
                    kind: r.try_get("kind").map_err(db)?,
                    oco_location: r.try_get("oco_location").map_err(db)?,
                    previous_kind: None,
                },
            ))
        })
        .transpose()
    }

    async fn venue_by_name(
        &self,
        user_id: UserId,
        connection: ConnectionId,
        platform_account: &str,
    ) -> Result<Option<VenueAccount>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let row = sqlx::query(
            "SELECT id, platform_account, kind, oco_location FROM gateway.venue_account
             WHERE user_id = $1 AND connection_id = $2 AND platform_account = $3",
        )
        .bind(user_id.as_uuid())
        .bind(connection.as_uuid())
        .bind(platform_account)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.as_ref().map(venue_row).transpose()
    }

    async fn venues(
        &self,
        user_id: UserId,
        ids: &[VenueAccountId],
    ) -> Result<Vec<(ConnectionId, VenueAccount)>, StoreError> {
        let ids: Vec<Uuid> = ids.iter().map(|i| i.as_uuid()).collect();
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(
            "SELECT id, connection_id, platform_account, kind, oco_location FROM gateway.venue_account
             WHERE user_id = $1 AND id = ANY($2) ORDER BY platform_account, id",
        )
        .bind(user_id.as_uuid())
        .bind(&ids)
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.iter()
            .map(|r| {
                Ok((
                    ConnectionId::from_uuid(r.try_get("connection_id").map_err(db)?),
                    venue_row(r)?,
                ))
            })
            .collect()
    }

    async fn venues_of_connections(
        &self,
        user_id: UserId,
        connections: &[ConnectionId],
    ) -> Result<Vec<(ConnectionId, VenueAccount)>, StoreError> {
        let ids: Vec<Uuid> = connections.iter().map(|c| c.as_uuid()).collect();
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(
            "SELECT id, connection_id, platform_account, kind, oco_location FROM gateway.venue_account
             WHERE user_id = $1 AND connection_id = ANY($2) ORDER BY platform_account, id",
        )
        .bind(user_id.as_uuid())
        .bind(&ids)
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.iter()
            .map(|r| {
                Ok((
                    ConnectionId::from_uuid(r.try_get("connection_id").map_err(db)?),
                    venue_row(r)?,
                ))
            })
            .collect()
    }

    async fn mark_sent(
        &self,
        user_id: UserId,
        id: CommandId,
        at: DateTime<Utc>,
        events: Vec<GatewayEvent>,
    ) -> Result<bool, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let done = sqlx::query(
            "UPDATE gateway.command SET state = 'sent', sent_at = $3
             WHERE user_id = $1 AND id = $2 AND state = 'pending' AND (expires_at IS NULL OR expires_at > $3)",
        )
        .bind(user_id.as_uuid())
        .bind(id.as_uuid())
        .bind(at)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
        if done.rows_affected() == 0 {
            tx.rollback().await.map_err(db)?;
            return Ok(false);
        }
        Self::insert_events(&mut tx, user_id, &events).await?;
        tx.commit().await.map_err(db)?;
        Ok(true)
    }

    async fn ack(
        &self,
        user_id: UserId,
        connection: ConnectionId,
        msg_ids: &[Uuid],
        at: DateTime<Utc>,
    ) -> Result<(), StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        sqlx::query(
            "UPDATE gateway.command SET state = 'acked', acked_at = $4
             WHERE user_id = $1 AND connection_id = $2 AND msg_id = ANY($3) AND state = 'sent'",
        )
        .bind(user_id.as_uuid())
        .bind(connection.as_uuid())
        .bind(msg_ids)
        .bind(at)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
        sqlx::query(
            "UPDATE gateway.outbound_message SET acked_at = $4
             WHERE user_id = $1 AND connection_id = $2 AND msg_id = ANY($3) AND acked_at IS NULL",
        )
        .bind(user_id.as_uuid())
        .bind(connection.as_uuid())
        .bind(msg_ids)
        .bind(at)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)
    }

    async fn enqueue_outbound(&self, m: &OutboundMessage) -> Result<(), StoreError> {
        let mut tx = self.begin_for(m.user_id).await?;
        sqlx::query(
            "INSERT INTO gateway.outbound_message (msg_id, user_id, connection_id, type, payload, protocol_version, frame, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(m.msg_id)
        .bind(m.user_id.as_uuid())
        .bind(m.connection_id.as_uuid())
        .bind(&m.kind)
        .bind(&m.payload)
        .bind(m.protocol_version.to_string())
        .bind(&m.frame)
        .bind(m.created_at)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)
    }

    async fn unacked_outbound(
        &self,
        user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<OutboundMessage>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(
            "SELECT msg_id, type, payload, protocol_version, frame, created_at FROM gateway.outbound_message
             WHERE user_id = $1 AND connection_id = $2 AND acked_at IS NULL ORDER BY created_at, msg_id",
        )
        .bind(user_id.as_uuid())
        .bind(connection.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.iter()
            .map(|r| {
                let v: String = r.try_get("protocol_version").map_err(db)?;
                Ok(OutboundMessage {
                    msg_id: r.try_get("msg_id").map_err(db)?,
                    user_id,
                    connection_id: connection,
                    kind: r.try_get("type").map_err(db)?,
                    payload: r.try_get("payload").map_err(db)?,
                    protocol_version: ProtocolVersion::parse(&v)
                        .map_err(|_| corrupt("protocol_version"))?,
                    frame: r.try_get("frame").map_err(db)?,
                    created_at: r.try_get("created_at").map_err(db)?,
                })
            })
            .collect()
    }

    async fn position_history(
        &self,
        user_id: UserId,
        account: AccountId,
        i: &Instrument,
    ) -> Result<Vec<ReportFact>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(concat!("WITH last_flat AS (
               SELECT max(seq) AS seq FROM gateway.execution_report
               WHERE user_id = $1 AND account_id = $2 AND instrument_symbol = $3 AND contract_month = $4 AND position_after = 0)
             SELECT ", report_columns!(), " FROM gateway.execution_report
             WHERE user_id = $1 AND account_id = $2 AND instrument_symbol = $3 AND contract_month = $4
               AND seq >= coalesce((SELECT seq FROM last_flat), 0)
             ORDER BY seq"))
        .bind(user_id.as_uuid())
        .bind(account.as_uuid())
        .bind(i.symbol())
        .bind(month_date(i)?)
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.iter().map(|r| report_row(r).map(|(_, f)| f)).collect()
    }

    async fn open_positions(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<Vec<(Instrument, i64)>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(
            "SELECT DISTINCT ON (instrument_symbol, contract_month) instrument_symbol, contract_month, position_after
             FROM gateway.execution_report
             WHERE user_id = $1 AND account_id = $2 AND position_after IS NOT NULL
             ORDER BY instrument_symbol, contract_month, seq DESC",
        )
        .bind(user_id.as_uuid())
        .bind(account.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        let mut out = Vec::new();
        for r in rows {
            let q: i32 = r.try_get("position_after").map_err(db)?;
            if q != 0 {
                out.push((
                    instrument(
                        &r.try_get::<String, _>("instrument_symbol").map_err(db)?,
                        r.try_get("contract_month").map_err(db)?,
                    )?,
                    i64::from(q),
                ));
            }
        }
        Ok(out)
    }

    async fn order_reports(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<Vec<(Instrument, ReportFact, Option<CommandKind>)>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        // The last report of each All Win order decides whether it is still live; the command's
        // kind tells outcome reports (modify/cancel) apart from order identities (Grosa M6).
        let rows = sqlx::query(concat!(
            "SELECT r.*, (SELECT c.kind FROM gateway.command c WHERE c.user_id = $1 AND c.id = r.command_id) AS command_kind
             FROM (SELECT DISTINCT ON (command_id, child_role) ",
            report_columns!(),
            " FROM gateway.execution_report
                   WHERE user_id = $1 AND account_id = $2 AND origin = 'all_win' AND command_id IS NOT NULL
                     AND child_role IS NOT NULL AND instrument_symbol IS NOT NULL AND status <> 'duplicate'
                   ORDER BY command_id, child_role, seq DESC) r"
        ))
        .bind(user_id.as_uuid())
        .bind(account.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        let mut out = Vec::new();
        for r in &rows {
            let (i, f) = report_row(r)?;
            let kind = r
                .try_get::<Option<String>, _>("command_kind")
                .map_err(db)?
                .as_deref()
                .map(parse_kind)
                .transpose()?;
            if let Some(i) = i {
                out.push((i, f, kind));
            }
        }
        Ok(out)
    }

    async fn receipt_known(
        &self,
        user_id: UserId,
        connection: ConnectionId,
        msg_id: Uuid,
    ) -> Result<bool, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let known: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM gateway.inbound_receipt WHERE connection_id = $1 AND msg_id = $2)",
        )
        .bind(connection.as_uuid())
        .bind(msg_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(known)
    }

    async fn command_by_msg(
        &self,
        user_id: UserId,
        connection: ConnectionId,
        msg_id: Uuid,
    ) -> Result<Option<StoredCommand>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let row = sqlx::query(concat!(
            "SELECT ",
            command_columns!(),
            " FROM gateway.command WHERE user_id = $1 AND connection_id = $2 AND msg_id = $3"
        ))
        .bind(user_id.as_uuid())
        .bind(connection.as_uuid())
        .bind(msg_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        row.as_ref().map(command_row).transpose()
    }

    async fn protective_reports_since(
        &self,
        user_id: UserId,
        account: AccountId,
        since: DateTime<Utc>,
    ) -> Result<Vec<(CommandId, ChildRole, ReportStatus)>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(concat!(
            "SELECT ",
            report_columns!(),
            " FROM gateway.execution_report
             WHERE user_id = $1 AND account_id = $2 AND received_at >= $3
               AND child_role IN ('stop','target') AND command_id IS NOT NULL
             ORDER BY seq"
        ))
        .bind(user_id.as_uuid())
        .bind(account.as_uuid())
        .bind(since)
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        let mut out = Vec::new();
        for r in &rows {
            let (_, f) = report_row(r)?;
            if let (Some(c), Some(role)) = (f.command_id, f.child_role) {
                out.push((c, role, f.status));
            }
        }
        Ok(out)
    }

    async fn position_before(
        &self,
        user_id: UserId,
        account: AccountId,
        i: &Instrument,
        seq: i64,
    ) -> Result<Option<i64>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let q: Option<i32> = sqlx::query_scalar(
            "SELECT position_after FROM gateway.execution_report
             WHERE user_id = $1 AND account_id = $2 AND instrument_symbol = $3 AND contract_month = $4
               AND seq < $5 AND position_after IS NOT NULL
             ORDER BY seq DESC LIMIT 1",
        )
        .bind(user_id.as_uuid())
        .bind(account.as_uuid())
        .bind(i.symbol())
        .bind(month_date(i)?)
        .bind(seq)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(q.map(i64::from))
    }

    async fn command_reports(
        &self,
        user_id: UserId,
        command: CommandId,
    ) -> Result<Vec<ReportFact>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = sqlx::query(concat!(
            "SELECT ",
            report_columns!(),
            " FROM gateway.execution_report WHERE user_id = $1 AND command_id = $2 ORDER BY seq"
        ))
        .bind(user_id.as_uuid())
        .bind(command.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        rows.iter().map(|r| report_row(r).map(|(_, f)| f)).collect()
    }

    async fn fill_known(
        &self,
        user_id: UserId,
        account: AccountId,
        fill_id: &str,
    ) -> Result<bool, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let known: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM gateway.execution_report
               WHERE user_id = $1 AND account_id = $2 AND fill_id = $3 AND fill_operation = 'add')",
        )
        .bind(user_id.as_uuid())
        .bind(account.as_uuid())
        .bind(fill_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(known)
    }

    async fn account_sync(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<AccountSync, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let head = sqlx::query(
            "SELECT sync_state, flagged_at, cleared_at, version FROM gateway.account_sync WHERE user_id = $1 AND account_id = $2",
        )
        .bind(user_id.as_uuid())
        .bind(account.as_uuid())
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?;
        let Some(head) = head else {
            tx.commit().await.map_err(db)?;
            return Ok(AccountSync::new_in_sync());
        };
        let rows = sqlx::query(
            "SELECT dedupe_key, raised_at, details, resolved_at FROM gateway.account_sync_reason
             WHERE user_id = $1 AND account_id = $2 ORDER BY raised_at, id",
        )
        .bind(user_id.as_uuid())
        .bind(account.as_uuid())
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        let mut reasons = Vec::with_capacity(rows.len());
        for r in rows {
            let key: String = r.try_get("dedupe_key").map_err(db)?;
            let (code, scope) = parse_dedupe_key(&key).ok_or_else(|| corrupt("dedupe_key"))?;
            let details: serde_json::Value = r.try_get("details").map_err(db)?;
            let details: ReasonDetails =
                serde_json::from_value(details).map_err(|_| corrupt("details"))?;
            let mut e = ReasonEntry::new(code, scope, r.try_get("raised_at").map_err(db)?, details);
            e.resolved_at = r.try_get("resolved_at").map_err(db)?;
            reasons.push(e);
        }
        let state = match head
            .try_get::<String, _>("sync_state")
            .map_err(db)?
            .as_str()
        {
            "in_sync" => SyncState::InSync,
            "out_of_sync" => SyncState::OutOfSync,
            _ => return Err(corrupt("sync_state")),
        };
        let version: i32 = head.try_get("version").map_err(db)?;
        Ok(AccountSync::restore(
            state,
            reasons,
            i64::from(version),
            head.try_get("flagged_at").map_err(db)?,
            head.try_get("cleared_at").map_err(db)?,
        ))
    }

    async fn commit(&self, user_id: UserId, work: UnitOfWork) -> Result<CommitOutcome, StoreError> {
        let user = user_id.as_uuid();
        let mut tx = self.begin_for(user_id).await?;

        if let Some((connection, msg_id)) = work.receipt {
            let done = sqlx::query(
                "INSERT INTO gateway.inbound_receipt (connection_id, msg_id, received_at) VALUES ($1, $2, now())
                 ON CONFLICT DO NOTHING",
            )
            .bind(connection.as_uuid())
            .bind(msg_id)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
            if done.rows_affected() == 0 {
                tx.rollback().await.map_err(db)?;
                return Ok(CommitOutcome::Duplicate);
            }
        }

        if let Some(r) = &work.report {
            let fill = r.fact.fill.as_ref();
            let done = sqlx::query(
                "INSERT INTO gateway.execution_report (event_id, user_id, account_id, connection_id, command_id, origin,
                   instrument_symbol, contract_month, child_role, status, platform_order_id, fill_id, fill_qty, fill_price,
                   fill_operation, commission, commission_rate, commission_effective, commission_source, position_after,
                   avg_price_after, reason_code, occurred_at, platform_time, payload)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21,
                   $22, $23, $24, $25)
                 ON CONFLICT DO NOTHING",
            )
            .bind(r.event_id.as_uuid())
            .bind(user)
            .bind(r.account_id.as_uuid())
            .bind(r.connection_id.as_uuid())
            .bind(r.fact.command_id.map(|c| c.as_uuid()))
            .bind(match r.fact.origin {
                Origin::AllWin => "all_win",
                Origin::LocalEnforcement => "local_enforcement",
                Origin::External => "external",
            })
            .bind(r.instrument.as_ref().map(Instrument::symbol))
            .bind(r.instrument.as_ref().map(month_date).transpose()?)
            .bind(r.fact.child_role.map(|c| match c {
                ChildRole::Entry => "entry",
                ChildRole::Stop => "stop",
                ChildRole::Target => "target",
                ChildRole::Exit => "exit",
                ChildRole::Flatten => "flatten",
            }))
            .bind(match r.fact.status {
                ReportStatus::Received => "received",
                ReportStatus::Working => "working",
                ReportStatus::PartiallyFilled => "partially_filled",
                ReportStatus::Filled => "filled",
                ReportStatus::Cancelled => "cancelled",
                ReportStatus::Rejected => "rejected",
                ReportStatus::Expired => "expired",
                ReportStatus::Duplicate => "duplicate",
                ReportStatus::Unknown => "unknown",
            })
            .bind(&r.platform_order_id)
            .bind(fill.map(|f| f.fill_id.clone()))
            .bind(fill.map(|f| i32::try_from(f.quantity).unwrap_or(i32::MAX)))
            .bind(r.fill_price.as_deref().map(decimal).transpose()?)
            .bind(fill.map(|f| match f.operation {
                FillOperation::Add => "add",
                FillOperation::Amend => "amend",
                FillOperation::Remove => "remove",
            }))
            .bind(r.commission.map(|m| m.value()))
            .bind(r.commission_rate.map(|m| m.value()))
            .bind(r.commission_effective.map(|m| m.value()))
            .bind(r.commission_source)
            .bind(r.fact.position_after.map(|p| i32::try_from(p).unwrap_or(i32::MAX)))
            .bind(r.avg_price_after.as_deref().map(decimal).transpose()?)
            .bind(&r.fact.reason_code)
            .bind(r.occurred_at)
            .bind(r.platform_time)
            .bind(&r.payload)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
            if done.rows_affected() == 0 {
                tx.rollback().await.map_err(db)?;
                return Ok(CommitOutcome::Duplicate);
            }
        }

        if let Some(d) = &work.deferred {
            // Behind the receipt; the primary key (connection_id, msg_id) is the second barrier.
            let done = sqlx::query(
                "INSERT INTO gateway.deferred_inbound (connection_id, msg_id, user_id, account_id, type, protocol_version,
                   payload, received_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                 ON CONFLICT DO NOTHING",
            )
            .bind(d.connection_id.as_uuid())
            .bind(d.msg_id)
            .bind(user)
            .bind(d.account_id.map(|a| a.as_uuid()))
            .bind(&d.kind)
            .bind(d.protocol_version.to_string())
            .bind(&d.payload)
            .bind(d.received_at)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
            if done.rows_affected() == 0 {
                tx.rollback().await.map_err(db)?;
                return Ok(CommitOutcome::Duplicate);
            }
        }

        for t in &work.commands {
            let finished = t.to.is_final().then_some(t.at);
            let done = sqlx::query(
                "UPDATE gateway.command SET state = $4, finished_at = coalesce($5, finished_at)
                 WHERE user_id = $1 AND id = $2 AND state = $3",
            )
            .bind(user)
            .bind(t.id.as_uuid())
            .bind(t.from.as_str())
            .bind(t.to.as_str())
            .bind(finished)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
            if done.rows_affected() == 0 {
                tx.rollback().await.map_err(db)?;
                return Err(StoreError::Conflict);
            }
        }

        for w in &work.sync {
            let next = &w.next;
            let state = match next.state() {
                SyncState::InSync => "in_sync",
                SyncState::OutOfSync => "out_of_sync",
            };
            let version = i32::try_from(next.version()).map_err(|_| corrupt("version"))?;
            let expected = i32::try_from(w.expected_version).map_err(|_| corrupt("version"))?;
            // L2: compare-and-set the version first; reason rows follow in the same transaction.
            let done = sqlx::query(
                "UPDATE gateway.account_sync SET sync_state = $4, flagged_at = $5, version = $6
                 WHERE user_id = $1 AND account_id = $2 AND version = $3",
            )
            .bind(user)
            .bind(w.account_id.as_uuid())
            .bind(expected)
            .bind(state)
            .bind(next.flagged_at())
            .bind(version)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
            let mut ok = done.rows_affected() == 1;
            if !ok && expected == 1 {
                // First change of an account: the row did not exist yet (implicit version 1).
                let ins = sqlx::query(
                    "INSERT INTO gateway.account_sync (user_id, account_id, sync_state, flagged_at, version)
                     VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
                )
                .bind(user)
                .bind(w.account_id.as_uuid())
                .bind(state)
                .bind(next.flagged_at())
                .bind(version)
                .execute(&mut *tx)
                .await
                .map_err(db)?;
                ok = ins.rows_affected() == 1;
            }
            if !ok {
                tx.rollback().await.map_err(db)?;
                return Err(StoreError::Conflict);
            }
            // The row set is rewritten from the aggregate (one row per occurrence, active and
            // resolved), under the version just won.
            sqlx::query(
                "DELETE FROM gateway.account_sync_reason WHERE user_id = $1 AND account_id = $2",
            )
            .bind(user)
            .bind(w.account_id.as_uuid())
            .execute(&mut *tx)
            .await
            .map_err(db)?;
            let rows = next
                .active()
                .into_iter()
                .cloned()
                .chain(next.resolved().iter().cloned());
            for e in rows {
                sqlx::query(
                    "INSERT INTO gateway.account_sync_reason (id, user_id, account_id, reason_code, dedupe_key, raised_at, details, resolved_at)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                )
                .bind(Uuid::now_v7())
                .bind(user)
                .bind(w.account_id.as_uuid())
                .bind(e.code.as_str())
                .bind(e.dedupe_key())
                .bind(e.detected_at)
                .bind(serde_json::to_value(&e.details).map_err(|e| StoreError::Other(e.to_string()))?)
                .bind(e.resolved_at)
                .execute(&mut *tx)
                .await
                .map_err(db)?;
            }
        }

        Self::insert_events(&mut tx, user_id, &work.events).await?;
        tx.commit().await.map_err(db)?;
        Ok(CommitOutcome::Committed)
    }
}
