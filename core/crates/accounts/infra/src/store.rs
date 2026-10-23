//! `accounts.account` / `accounts.account_rule_value` in PostgreSQL. Every statement runs in a
//! transaction scoped by `app.user_id` (RLS, fail-closed) and also filters `user_id` itself.

use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use shared_kernel::UserId;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
use uuid::Uuid;

use accounts_application::enterprise::{
    Account, AccountId, AccountKind, AccountName, Binding, Choice, FirmId, FirmPlatform,
    LifecycleState, ManualText, MaxPositionQty, MoneyAmount, Multiplier, OptInState, PendingReason,
    PresetVersionId, PreviousChoice, PropSelection, RuleField, RuleOrigin, RuleVersionId,
    SizeAmount, VenueAccountId,
};
use accounts_application::ports::{
    AccountStore, AccountWrite, CommissionWrite, CreateKeyRecord, NewAccount, StoreError,
};

use crate::outbox;

#[derive(Clone)]
pub struct PgAccountStore {
    pool: PgPool,
}

impl PgAccountStore {
    /// `pool` connects as `allwin_app` (RLS enforced).
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn begin_for(
        &self,
        user_id: UserId,
    ) -> Result<Transaction<'static, Postgres>, StoreError> {
        let mut tx = self.pool.begin().await.map_err(other)?;
        set_user(&mut tx, user_id).await?;
        Ok(tx)
    }
}

/// Sets the RLS context of the current transaction.
pub async fn set_user(conn: &mut PgConnection, user_id: UserId) -> Result<(), StoreError> {
    sqlx::query!(
        "SELECT set_config('app.user_id', $1, true)",
        user_id.as_uuid().to_string()
    )
    .fetch_one(conn)
    .await
    .map_err(other)?;
    Ok(())
}

/// One row of `accounts.account` with its commission floor.
struct AccountRow {
    id: Uuid,
    user_id: Uuid,
    kind: String,
    display_name: String,
    firm_id: Option<Uuid>,
    firm_name: Option<String>,
    plan_name: Option<String>,
    phase: Option<String>,
    preset_version_id: Option<Uuid>,
    firm_platform: Option<String>,
    size_amount: Decimal,
    multiplier: i32,
    max_position_qty: i32,
    lifecycle_state: String,
    venue_account_id: Option<Uuid>,
    bound_at: Option<DateTime<Utc>>,
    automation_state: String,
    automation_choice: Option<String>,
    automation_rule_version_id: Option<Uuid>,
    automation_pending_reason: Option<String>,
    automation_chosen_at: Option<DateTime<Utc>>,
    copy_state: String,
    copy_choice: Option<String>,
    copy_rule_version_id: Option<Uuid>,
    copy_pending_reason: Option<String>,
    copy_chosen_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    archived_at: Option<DateTime<Utc>>,
    row_version: i32,
    limits_version: i32,
    evaluation_start: Option<NaiveDate>,
    evaluation_earned_before: Option<Decimal>,
    commission: Option<String>,
    create_request_hash: Option<Vec<u8>>,
}

/// `SELECT … FROM accounts.account a <tail>` as `AccountRow` (one column list for every read).
macro_rules! select_account {
    ($tail:literal, $($arg:expr),* $(,)?) => {
        sqlx::query_as!(
            AccountRow,
            r#"SELECT a.id, a.user_id, a.kind, a.display_name, a.firm_id, a.firm_name, a.plan_name,
                      a.phase, a.preset_version_id, a.firm_platform, a.size_amount, a.multiplier,
                      a.max_position_qty, a.lifecycle_state, a.venue_account_id, a.bound_at,
                      a.automation_state, a.automation_choice, a.automation_rule_version_id,
                      a.automation_pending_reason, a.automation_chosen_at,
                      a.copy_state, a.copy_choice, a.copy_rule_version_id, a.copy_pending_reason,
                      a.copy_chosen_at, a.created_at, a.updated_at, a.archived_at, a.row_version,
                      a.limits_version, a.evaluation_start, a.evaluation_earned_before,
                      (SELECT r.value->>'amount' FROM accounts.account_rule_value r
                        WHERE r.user_id = a.user_id AND r.account_id = a.id
                          AND r.field = 'commission_per_contract_side' AND NOT r.is_unset) AS commission,
                      a.create_request_hash
                 FROM accounts.account a "# + $tail,
            $($arg),*
        )
    };
}

#[async_trait]
impl AccountStore for PgAccountStore {
    async fn list(&self, user_id: UserId) -> Result<Vec<Account>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let rows = select_account!(
            "WHERE a.user_id = $1 AND a.archived_at IS NULL ORDER BY lower(a.display_name), a.id",
            user_id.as_uuid()
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(other)?;
        tx.commit().await.map_err(other)?;
        rows.into_iter().map(to_account).collect()
    }

    async fn get(&self, user_id: UserId, id: AccountId) -> Result<Option<Account>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let row = select_account!(
            "WHERE a.user_id = $1 AND a.id = $2 AND a.archived_at IS NULL",
            user_id.as_uuid(),
            id.as_uuid()
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(other)?;
        tx.commit().await.map_err(other)?;
        row.map(to_account).transpose()
    }

    async fn find_by_create_key(
        &self,
        user_id: UserId,
        key: Uuid,
    ) -> Result<Option<CreateKeyRecord>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let row = select_account!(
            "WHERE a.user_id = $1 AND a.create_idempotency_key = $2",
            user_id.as_uuid(),
            key
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(other)?;
        tx.commit().await.map_err(other)?;
        let Some(row) = row else { return Ok(None) };
        let request_hash: [u8; 32] = row
            .create_request_hash
            .clone()
            .and_then(|h| h.try_into().ok())
            .ok_or_else(|| StoreError::Other("create_request_hash".into()))?;
        Ok(Some(CreateKeyRecord {
            account: to_account(row)?,
            request_hash,
        }))
    }

    async fn name_taken(
        &self,
        user_id: UserId,
        name: &str,
        except: Option<AccountId>,
    ) -> Result<bool, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let taken = sqlx::query_scalar!(
            r#"SELECT EXISTS (
                 SELECT 1 FROM accounts.account
                  WHERE user_id = $1 AND archived_at IS NULL AND lower(display_name) = lower($2)
                    AND id IS DISTINCT FROM $3) AS "taken!""#,
            user_id.as_uuid(),
            name,
            except.map(|e| e.as_uuid()),
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(other)?;
        tx.commit().await.map_err(other)?;
        Ok(taken)
    }

    async fn binding_taken(
        &self,
        user_id: UserId,
        venue_account_id: VenueAccountId,
        except: Option<AccountId>,
    ) -> Result<bool, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let taken = sqlx::query_scalar!(
            r#"SELECT EXISTS (
                 SELECT 1 FROM accounts.account
                  WHERE user_id = $1 AND venue_account_id = $2 AND id IS DISTINCT FROM $3) AS "taken!""#,
            user_id.as_uuid(),
            venue_account_id.as_uuid(),
            except.map(|e| e.as_uuid()),
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(other)?;
        tx.commit().await.map_err(other)?;
        Ok(taken)
    }

    async fn evaluation_target(
        &self,
        user_id: UserId,
        id: AccountId,
    ) -> Result<Option<MoneyAmount>, StoreError> {
        let mut tx = self.begin_for(user_id).await?;
        let amount = sqlx::query_scalar!(
            r#"SELECT value->>'amount' AS "amount!"
                 FROM accounts.account_rule_value
                WHERE user_id = $1 AND account_id = $2 AND field = 'evaluation_profit_target'
                  AND NOT is_unset"#,
            user_id.as_uuid(),
            id.as_uuid(),
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(other)?;
        tx.commit().await.map_err(other)?;
        amount
            .map(|s| {
                Decimal::from_str(&s)
                    .ok()
                    .and_then(|d| MoneyAmount::new(d).ok())
                    .ok_or_else(|| corrupt("evaluation_profit_target"))
            })
            .transpose()
    }

    async fn create(&self, new: NewAccount) -> Result<(), StoreError> {
        let a = &new.account;
        let mut tx = self.begin_for(a.user_id).await?;
        let c = Columns::of(a);
        sqlx::query!(
            r#"INSERT INTO accounts.account (
                 id, user_id, display_name, kind, firm_id, firm_name, plan_name, phase,
                 preset_version_id, firm_platform, size_amount, multiplier, max_position_qty,
                 lifecycle_state, venue_account_id, bound_at,
                 automation_state, automation_choice, automation_rule_version_id,
                 automation_pending_reason, automation_chosen_at,
                 copy_state, copy_choice, copy_rule_version_id, copy_pending_reason, copy_chosen_at,
                 created_at, updated_at, row_version, limits_version, limits_changed_at,
                 evaluation_start, evaluation_earned_before, create_idempotency_key, create_request_hash)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
                       $17, $18, $19, $20, $21, $22, $23, $24, $25, $26,
                       $27, $27, 1, $28, $27, $29, $30, $31, $32)"#,
            a.id.as_uuid(),
            a.user_id.as_uuid(),
            a.display_name.as_str(),
            a.kind.as_str(),
            c.firm_id,
            c.firm_name,
            c.plan_name,
            c.phase,
            c.preset_version_id,
            c.firm_platform,
            a.size_amount.value(),
            c.multiplier,
            c.max_position_qty,
            a.lifecycle.as_str(),
            c.venue_account_id,
            c.bound_at,
            c.automation.state,
            c.automation.choice,
            c.automation.rule_version_id,
            c.automation.pending_reason,
            c.automation.chosen_at,
            c.copy.state,
            c.copy.choice,
            c.copy.rule_version_id,
            c.copy.pending_reason,
            c.copy.chosen_at,
            a.created_at,
            a.limits_version,
            a.evaluation_start,
            a.evaluation_earned_before.map(|m| m.value()),
            new.idempotency_key,
            &new.request_hash[..],
        )
        .execute(&mut *tx)
        .await
        .map_err(unique)?;
        if a.commission_per_contract_side.is_some() {
            write_commission(&mut tx, a, origin_of(a), a.created_at).await?;
        }
        crate::profiles::insert_initial_copies(&mut tx, a.user_id, a.id, &new.initial_profiles)
            .await
            .map_err(other)?;
        for e in &new.events {
            outbox::insert(&mut tx, a.user_id, e).await.map_err(other)?;
        }
        tx.commit().await.map_err(unique)
    }

    async fn save(&self, write: AccountWrite) -> Result<(), StoreError> {
        let a = &write.account;
        let mut tx = self.begin_for(a.user_id).await?;
        let c = Columns::of(a);
        let updated = sqlx::query!(
            r#"UPDATE accounts.account SET
                 display_name = $4, firm_id = $5, firm_name = $6, plan_name = $7, phase = $8,
                 preset_version_id = $9, firm_platform = $10, size_amount = $11, multiplier = $12,
                 max_position_qty = $13, lifecycle_state = $14, venue_account_id = $15, bound_at = $16,
                 automation_state = $17, automation_choice = $18, automation_rule_version_id = $19,
                 automation_pending_reason = $20, automation_chosen_at = $21,
                 copy_state = $22, copy_choice = $23, copy_rule_version_id = $24,
                 copy_pending_reason = $25, copy_chosen_at = $26,
                 updated_at = $27, archived_at = $28, row_version = row_version + 1,
                 limits_changed_at = CASE WHEN limits_version <> $29 THEN $27 ELSE limits_changed_at END,
                 limits_version = $29, evaluation_start = $30, evaluation_earned_before = $31
               WHERE user_id = $1 AND id = $2 AND row_version = $3 AND archived_at IS NULL"#,
            a.user_id.as_uuid(),
            a.id.as_uuid(),
            write.expected_row_version,
            a.display_name.as_str(),
            c.firm_id,
            c.firm_name,
            c.plan_name,
            c.phase,
            c.preset_version_id,
            c.firm_platform,
            a.size_amount.value(),
            c.multiplier,
            c.max_position_qty,
            a.lifecycle.as_str(),
            c.venue_account_id,
            c.bound_at,
            c.automation.state,
            c.automation.choice,
            c.automation.rule_version_id,
            c.automation.pending_reason,
            c.automation.chosen_at,
            c.copy.state,
            c.copy.choice,
            c.copy.rule_version_id,
            c.copy.pending_reason,
            c.copy.chosen_at,
            a.updated_at,
            a.archived_at,
            a.limits_version,
            a.evaluation_start,
            a.evaluation_earned_before.map(|m| m.value()),
        )
        .execute(&mut *tx)
        .await
        .map_err(unique)?;
        if updated.rows_affected() != 1 {
            return Err(StoreError::VersionConflict);
        }
        match write.commission {
            CommissionWrite::Unchanged => {}
            CommissionWrite::Set(origin) | CommissionWrite::Unset(origin) => {
                write_commission(&mut tx, a, origin, a.updated_at).await?;
            }
        }
        for e in &write.events {
            outbox::insert(&mut tx, a.user_id, e).await.map_err(other)?;
        }
        tx.commit().await.map_err(unique)
    }
}

/// Upserts the commission floor rule value: the value, or a tombstone when it is removed.
async fn write_commission(
    conn: &mut PgConnection,
    a: &Account,
    origin: RuleOrigin,
    at: DateTime<Utc>,
) -> Result<(), StoreError> {
    let value = a
        .commission_per_contract_side
        .map(|m| serde_json::json!({ "amount": m.to_wire() }));
    sqlx::query!(
        r#"INSERT INTO accounts.account_rule_value
             (user_id, account_id, field, origin, value, is_unset, saved_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
           ON CONFLICT (account_id, field) DO UPDATE
             SET origin = EXCLUDED.origin, value = EXCLUDED.value, is_unset = EXCLUDED.is_unset,
                 chose = NULL, chosen_at = NULL, saved_at = EXCLUDED.saved_at,
                 updated_at = EXCLUDED.updated_at"#,
        a.user_id.as_uuid(),
        a.id.as_uuid(),
        RuleField::CommissionPerContractSide.as_str(),
        origin.as_str(),
        value,
        value.is_none(),
        at,
    )
    .execute(conn)
    .await
    .map_err(other)?;
    Ok(())
}

fn origin_of(a: &Account) -> RuleOrigin {
    match a.selection {
        Some(PropSelection::Preset { .. }) => RuleOrigin::Edited,
        _ => RuleOrigin::SetByUser,
    }
}

struct OptInColumns {
    state: &'static str,
    choice: Option<&'static str>,
    rule_version_id: Option<Uuid>,
    pending_reason: Option<&'static str>,
    chosen_at: Option<DateTime<Utc>>,
}

impl OptInColumns {
    fn of(s: &OptInState) -> Self {
        let recorded = s.recorded();
        Self {
            state: s.state_str(),
            choice: recorded.map(|c| c.choice.as_str()),
            rule_version_id: recorded.map(|c| c.rule_version_id.as_uuid()),
            pending_reason: match s {
                OptInState::PendingReview { reason, .. } => Some(reason.as_str()),
                _ => None,
            },
            chosen_at: recorded.map(|c| c.chosen_at),
        }
    }
}

/// Column values of an account that are not plain fields.
struct Columns {
    firm_id: Option<Uuid>,
    firm_name: Option<String>,
    plan_name: Option<String>,
    phase: Option<String>,
    preset_version_id: Option<Uuid>,
    firm_platform: Option<String>,
    multiplier: i32,
    max_position_qty: i32,
    venue_account_id: Option<Uuid>,
    bound_at: Option<DateTime<Utc>>,
    automation: OptInColumns,
    copy: OptInColumns,
}

impl Columns {
    fn of(a: &Account) -> Self {
        let (firm_id, firm_name, plan_name, phase, preset_version_id, firm_platform) =
            match &a.selection {
                None => (None, None, None, None, None, None),
                Some(PropSelection::Manual {
                    firm_name,
                    plan_name,
                    phase,
                }) => (
                    None,
                    Some(firm_name.as_str().to_owned()),
                    Some(plan_name.as_str().to_owned()),
                    Some(phase.as_str().to_owned()),
                    None,
                    None,
                ),
                Some(PropSelection::Preset {
                    preset_version_id,
                    firm_id,
                    firm_platform,
                }) => (
                    Some(firm_id.as_uuid()),
                    None,
                    None,
                    None,
                    Some(preset_version_id.as_uuid()),
                    firm_platform.as_ref().map(|p| p.as_str().to_owned()),
                ),
            };
        Self {
            firm_id,
            firm_name,
            plan_name,
            phase,
            preset_version_id,
            firm_platform,
            // Bounded by the value objects (≤ 10 000).
            multiplier: i32::try_from(a.multiplier.value()).unwrap_or(i32::MAX),
            max_position_qty: i32::try_from(a.max_position_qty.value()).unwrap_or(i32::MAX),
            venue_account_id: a.binding.map(|b| b.venue_account_id.as_uuid()),
            bound_at: a.binding.map(|b| b.bound_at),
            automation: OptInColumns::of(&a.automation),
            copy: OptInColumns::of(&a.copy),
        }
    }
}

fn corrupt(what: &str) -> StoreError {
    StoreError::Other(format!("account row: invalid {what}"))
}

fn opt_in_of(
    state: &str,
    choice: Option<&str>,
    rule_version_id: Option<Uuid>,
    pending_reason: Option<&str>,
    chosen_at: Option<DateTime<Utc>>,
) -> Result<OptInState, StoreError> {
    let recorded = match (choice, rule_version_id, chosen_at) {
        (Some(c), Some(rv), Some(at)) => Some(PreviousChoice {
            choice: Choice::parse(c).ok_or_else(|| corrupt("choice"))?,
            rule_version_id: RuleVersionId::from_uuid(rv),
            chosen_at: at,
        }),
        (None, _, None) => None,
        _ => return Err(corrupt("opt-in choice")),
    };
    match state {
        "not_required" => Ok(OptInState::NotRequired),
        "chosen" => recorded
            .map(OptInState::Chosen)
            .ok_or_else(|| corrupt("chosen opt-in")),
        "pending_review" => Ok(OptInState::PendingReview {
            reason: pending_reason
                .and_then(PendingReason::parse)
                .ok_or_else(|| corrupt("pending reason"))?,
            previous: recorded,
        }),
        _ => Err(corrupt("opt-in state")),
    }
}

fn to_account(r: AccountRow) -> Result<Account, StoreError> {
    let kind = AccountKind::parse(&r.kind).ok_or_else(|| corrupt("kind"))?;
    let text = |v: Option<String>, what: &str| {
        v.as_deref()
            .and_then(|s| ManualText::parse(s).ok())
            .ok_or_else(|| corrupt(what))
    };
    let selection = match (kind, r.preset_version_id) {
        (AccountKind::Real, _) => None,
        (AccountKind::Prop, Some(pv)) => Some(PropSelection::Preset {
            preset_version_id: PresetVersionId::from_uuid(pv),
            firm_id: FirmId::from_uuid(r.firm_id.ok_or_else(|| corrupt("firm_id"))?),
            firm_platform: r
                .firm_platform
                .as_deref()
                .map(FirmPlatform::parse)
                .transpose()
                .map_err(|_| corrupt("firm_platform"))?,
        }),
        (AccountKind::Prop, None) => Some(PropSelection::Manual {
            firm_name: text(r.firm_name, "firm_name")?,
            plan_name: text(r.plan_name, "plan_name")?,
            phase: text(r.phase, "phase")?,
        }),
    };
    let money = |d: Option<Decimal>, what: &str| {
        d.map(MoneyAmount::new)
            .transpose()
            .map_err(|_| corrupt(what))
    };
    let commission = r
        .commission
        .as_deref()
        .map(|s| {
            Decimal::from_str(s)
                .ok()
                .and_then(|d| MoneyAmount::new(d).ok())
                .ok_or_else(|| corrupt("commission"))
        })
        .transpose()?;
    Ok(Account {
        id: AccountId::from_uuid(r.id),
        user_id: UserId::from_uuid(r.user_id),
        kind,
        display_name: AccountName::parse(&r.display_name).map_err(|_| corrupt("display_name"))?,
        selection,
        size_amount: SizeAmount::new(r.size_amount).map_err(|_| corrupt("size_amount"))?,
        multiplier: Multiplier::new(i64::from(r.multiplier)).map_err(|_| corrupt("multiplier"))?,
        max_position_qty: MaxPositionQty::new(i64::from(r.max_position_qty))
            .map_err(|_| corrupt("max_position_qty"))?,
        lifecycle: LifecycleState::parse(&r.lifecycle_state).ok_or_else(|| corrupt("lifecycle"))?,
        binding: match (r.venue_account_id, r.bound_at) {
            (Some(v), Some(at)) => Some(Binding {
                venue_account_id: VenueAccountId::from_uuid(v),
                bound_at: at,
            }),
            _ => None,
        },
        automation: opt_in_of(
            &r.automation_state,
            r.automation_choice.as_deref(),
            r.automation_rule_version_id,
            r.automation_pending_reason.as_deref(),
            r.automation_chosen_at,
        )?,
        copy: opt_in_of(
            &r.copy_state,
            r.copy_choice.as_deref(),
            r.copy_rule_version_id,
            r.copy_pending_reason.as_deref(),
            r.copy_chosen_at,
        )?,
        commission_per_contract_side: commission,
        evaluation_start: r.evaluation_start,
        evaluation_earned_before: money(r.evaluation_earned_before, "evaluation_earned_before")?,
        row_version: r.row_version,
        limits_version: r.limits_version,
        created_at: r.created_at,
        updated_at: r.updated_at,
        archived_at: r.archived_at,
    })
}

/// Database errors carry statement text and constraint names, never bound values; they are
/// logged as internal errors and never returned to clients.
fn other(e: sqlx::Error) -> StoreError {
    StoreError::Other(e.to_string())
}

/// Maps the unique constraints a concurrent write can hit after the pre-checks.
fn unique(e: sqlx::Error) -> StoreError {
    if let sqlx::Error::Database(db) = &e {
        match db.constraint() {
            Some("account_user_name_key") => return StoreError::NameTaken,
            Some("account_venue_account_key") => return StoreError::BindingTaken,
            Some("account_create_key") => return StoreError::CreateKeyTaken,
            _ => {}
        }
    }
    other(e)
}
