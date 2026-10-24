//! `accounts.account_rule_value`, read and written in the caller's user-scoped transaction. One
//! implementation for every writer (S16 fill/apply/restore, S13 putAccountRiskRules; Malha):
//! - it only writes rows (upsert, tombstone, delete) and returns the fields that really changed;
//! - a value written again unchanged keeps its row and its `saved_at` (Q48: effective_from must not
//!   move by itself);
//! - it never bumps `limits_version`, writes events or touches `row_version`: the caller does that
//!   once per transaction (ADR-0006), and fills `evaluation_start` when the change gives the account
//!   an evaluation target.

use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::PgConnection;

use accounts_application::enterprise::preset_values::{AccountRuleValue, Chose, RuleValue};
use accounts_application::enterprise::{AccountId, RuleField, RuleOrigin};
pub use accounts_application::ports::RuleValueWrite;
use shared_kernel::UserId;

const FIELDS: [RuleField; 10] = [
    RuleField::MaxPositionQty,
    RuleField::Drawdown,
    RuleField::DailyLossLimit,
    RuleField::DailyProfitTarget,
    RuleField::TradingDayReset,
    RuleField::ConsistencyRule,
    RuleField::ProfitTargetOnReach,
    RuleField::ConsistencyThresholdPct,
    RuleField::CommissionPerContractSide,
    RuleField::EvaluationProfitTarget,
];

fn field_of(raw: &str) -> Option<RuleField> {
    FIELDS.into_iter().find(|f| f.as_str() == raw)
}

fn origin_of(raw: &str) -> Option<RuleOrigin> {
    [
        RuleOrigin::Edited,
        RuleOrigin::ChosenFromConflict,
        RuleOrigin::SetByUser,
    ]
    .into_iter()
    .find(|o| o.as_str() == raw)
}

fn corrupt(what: &str) -> sqlx::Error {
    sqlx::Error::Protocol(format!("account_rule_value row: invalid {what}"))
}

struct Row {
    field: String,
    origin: String,
    value: Option<Value>,
    chose: Option<String>,
    chosen_at: Option<DateTime<Utc>>,
    is_unset: bool,
    saved_at: DateTime<Utc>,
}

fn to_value(r: Row) -> Result<AccountRuleValue, sqlx::Error> {
    let field = field_of(&r.field).ok_or_else(|| corrupt("field"))?;
    let value = match &r.value {
        Some(v) => Some(RuleValue::deserialize(field, v).map_err(|e| corrupt(&e.0))?),
        None => None,
    };
    let chose = match (r.chose.as_deref(), r.chosen_at) {
        (Some("published_value"), Some(at)) => Some((Chose::PublishedValue, at)),
        (Some("other_value"), Some(at)) => Some((Chose::OtherValue, at)),
        (None, None) => None,
        _ => return Err(corrupt("chose")),
    };
    Ok(AccountRuleValue {
        field,
        origin: origin_of(&r.origin).ok_or_else(|| corrupt("origin"))?,
        value,
        is_unset: r.is_unset,
        chose,
        saved_at: r.saved_at,
    })
}

/// The account's rule values (RLS: the caller's transaction has `app.user_id` set).
pub async fn read_rule_values(
    conn: &mut PgConnection,
    user_id: UserId,
    account_id: AccountId,
) -> Result<Vec<AccountRuleValue>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Row,
        r#"SELECT field, origin, value, chose, chosen_at, is_unset, saved_at
           FROM accounts.account_rule_value WHERE user_id = $1 AND account_id = $2 ORDER BY field"#,
        user_id.as_uuid(),
        account_id.as_uuid(),
    )
    .fetch_all(&mut *conn)
    .await?;
    rows.into_iter().map(to_value).collect()
}

/// Writes the desired rows; returns the fields whose stored meaning changed.
pub async fn write_rule_values(
    conn: &mut PgConnection,
    user_id: UserId,
    account_id: AccountId,
    writes: &[RuleValueWrite],
    now: DateTime<Utc>,
) -> Result<Vec<RuleField>, sqlx::Error> {
    let current = read_rule_values(&mut *conn, user_id, account_id).await?;
    let mut changed = Vec::new();
    for w in writes {
        let existing = current.iter().find(|c| c.field == w.field);
        match (&w.row, existing) {
            (None, None) => {}
            (Some(new), Some(old)) if new.same_as(old) => {}
            (None, Some(_)) => {
                sqlx::query!(
                    "DELETE FROM accounts.account_rule_value WHERE user_id = $1 AND account_id = $2 AND field = $3",
                    user_id.as_uuid(),
                    account_id.as_uuid(),
                    w.field.as_str(),
                )
                .execute(&mut *conn)
                .await?;
                changed.push(w.field);
            }
            (Some(new), _) => {
                if new.field != w.field {
                    return Err(sqlx::Error::Protocol(
                        "rule value written under another field".into(),
                    ));
                }
                let value = match &new.value {
                    Some(v) => Some(
                        serde_json::to_value(v.to_wire())
                            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?,
                    ),
                    None => None,
                };
                sqlx::query!(
                    r#"INSERT INTO accounts.account_rule_value
                         (user_id, account_id, field, origin, value, chose, chosen_at, is_unset, saved_at, updated_at)
                       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
                       ON CONFLICT (account_id, field) DO UPDATE SET
                         origin = EXCLUDED.origin, value = EXCLUDED.value, chose = EXCLUDED.chose,
                         chosen_at = EXCLUDED.chosen_at, is_unset = EXCLUDED.is_unset,
                         saved_at = EXCLUDED.saved_at, updated_at = EXCLUDED.updated_at"#,
                    user_id.as_uuid(),
                    account_id.as_uuid(),
                    w.field.as_str(),
                    new.origin.as_str(),
                    value,
                    new.chose.map(|(c, _)| c.as_str()),
                    new.chose.map(|(_, at)| at),
                    new.is_unset,
                    now,
                )
                .execute(&mut *conn)
                .await?;
                changed.push(w.field);
            }
        }
    }
    Ok(changed)
}
