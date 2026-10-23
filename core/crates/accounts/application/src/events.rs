//! Events the accounts module writes to `app.outbox` in the same transaction as the change.
//! - `accounts.*`: the user-visible audit (S5 contract §4, category `config`); money as strings.
//! - `account.limits_changed` / `account.binding_changed`: ADR-0006 / ADR-0008 consumers.
//! - `account.changed` / `account.archived`: the SSE notifications (the client refetches).

use chrono::{DateTime, NaiveDate, Utc};
use serde_json::{Map, Value, json};

use accounts_enterprise::{
    AccountId, AccountName, Choice, FirmStance, LifecycleState, MoneyAmount, OptInKind,
    RuleVersionId, VenueAccountId,
};

use crate::ports::{RouteRef, RuleSource};

/// One changed field of `accounts.updated`; `Value::Null` stands for "absent".
#[derive(Debug, Clone, PartialEq)]
pub struct FieldChange {
    pub field: &'static str,
    pub old: Value,
    pub new: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccountsEvent {
    Created {
        account_id: AccountId,
        account_name: AccountName,
        at: DateTime<Utc>,
        /// Every stored field, as `FieldChange::new` values.
        fields: Vec<FieldChange>,
    },
    Updated {
        account_id: AccountId,
        account_name: AccountName,
        at: DateTime<Utc>,
        changes: Vec<FieldChange>,
    },
    Archived {
        account_id: AccountId,
        account_name: AccountName,
        at: DateTime<Utc>,
        removed_routes: Vec<RouteRef>,
        binding_released: bool,
    },
    LifecycleChanged {
        account_id: AccountId,
        account_name: AccountName,
        at: DateTime<Utc>,
        previous: LifecycleState,
        new: LifecycleState,
        had_open_position: bool,
    },
    OptInChosen {
        account_id: AccountId,
        account_name: AccountName,
        at: DateTime<Utc>,
        kind: OptInKind,
        choice: Choice,
        previous_choice: Option<Choice>,
        rule_version_id: RuleVersionId,
        firm_stance: FirmStance,
        sources: Vec<RuleSource>,
    },
    EvaluationRestarted {
        account_id: AccountId,
        account_name: AccountName,
        at: DateTime<Utc>,
        old_start: Option<NaiveDate>,
        new_start: NaiveDate,
        old_earned_before: Option<MoneyAmount>,
        new_earned_before: Option<MoneyAmount>,
    },
    LimitsChanged {
        account_id: AccountId,
        at: DateTime<Utc>,
        limits_version: i32,
    },
    BindingChanged {
        account_id: AccountId,
        at: DateTime<Utc>,
        venue_account_id: Option<VenueAccountId>,
        limits_version: i32,
    },
    /// SSE `account.changed`.
    Changed {
        account_id: AccountId,
        at: DateTime<Utc>,
    },
    /// SSE `account.archived`.
    ArchivedNotice {
        account_id: AccountId,
        at: DateTime<Utc>,
    },
}

/// Audited event types (the audit consumer registers these).
pub const AUDIT_EVENT_TYPES: [&str; 6] = [
    "accounts.created",
    "accounts.updated",
    "accounts.archived",
    "accounts.lifecycle_changed",
    "accounts.opt_in_chosen",
    "accounts.evaluation_restarted",
];

impl AccountsEvent {
    pub const fn event_type(&self) -> &'static str {
        match self {
            Self::Created { .. } => "accounts.created",
            Self::Updated { .. } => "accounts.updated",
            Self::Archived { .. } => "accounts.archived",
            Self::LifecycleChanged { .. } => "accounts.lifecycle_changed",
            Self::OptInChosen { .. } => "accounts.opt_in_chosen",
            Self::EvaluationRestarted { .. } => "accounts.evaluation_restarted",
            Self::LimitsChanged { .. } => "account.limits_changed",
            Self::BindingChanged { .. } => "account.binding_changed",
            Self::Changed { .. } => "account.changed",
            Self::ArchivedNotice { .. } => "account.archived",
        }
    }

    pub const fn at(&self) -> DateTime<Utc> {
        match self {
            Self::Created { at, .. }
            | Self::Updated { at, .. }
            | Self::Archived { at, .. }
            | Self::LifecycleChanged { at, .. }
            | Self::OptInChosen { at, .. }
            | Self::EvaluationRestarted { at, .. }
            | Self::LimitsChanged { at, .. }
            | Self::BindingChanged { at, .. }
            | Self::Changed { at, .. }
            | Self::ArchivedNotice { at, .. } => *at,
        }
    }

    /// Outbox payload. Audit events carry `account_id` and the `account_name` snapshot.
    pub fn payload(&self) -> Value {
        let mut p = Map::new();
        p.insert("kind".into(), json!(self.event_type()));
        match self {
            Self::Created {
                account_id,
                account_name,
                fields,
                ..
            } => {
                audit_head(&mut p, account_id, account_name);
                let f: Map<String, Value> = fields
                    .iter()
                    .map(|c| (c.field.to_owned(), c.new.clone()))
                    .collect();
                p.insert("fields".into(), Value::Object(f));
            }
            Self::Updated {
                account_id,
                account_name,
                changes,
                ..
            } => {
                audit_head(&mut p, account_id, account_name);
                let c: Vec<Value> = changes
                    .iter()
                    .map(|c| json!({"field": c.field, "old": c.old, "new": c.new}))
                    .collect();
                p.insert("changes".into(), Value::Array(c));
            }
            Self::Archived {
                account_id,
                account_name,
                removed_routes,
                binding_released,
                ..
            } => {
                audit_head(&mut p, account_id, account_name);
                let r: Vec<Value> = removed_routes
                    .iter()
                    .map(|r| json!({"source_id": r.source_id, "source_name": r.source_name}))
                    .collect();
                p.insert("removed_routes".into(), Value::Array(r));
                p.insert("removed_follower_links".into(), json!([]));
                p.insert("binding_released".into(), json!(binding_released));
            }
            Self::LifecycleChanged {
                account_id,
                account_name,
                previous,
                new,
                had_open_position,
                ..
            } => {
                audit_head(&mut p, account_id, account_name);
                p.insert("previous_state".into(), json!(previous.as_str()));
                p.insert("new_state".into(), json!(new.as_str()));
                p.insert("had_open_position".into(), json!(had_open_position));
            }
            Self::OptInChosen {
                account_id,
                account_name,
                kind,
                choice,
                previous_choice,
                rule_version_id,
                firm_stance,
                sources,
                ..
            } => {
                audit_head(&mut p, account_id, account_name);
                p.insert("opt_in_kind".into(), json!(kind.as_str()));
                p.insert("choice".into(), json!(choice.as_str()));
                p.insert(
                    "previous_choice".into(),
                    previous_choice.map_or(Value::Null, |c| json!(c.as_str())),
                );
                p.insert("rule_version_id".into(), json!(rule_version_id.as_uuid()));
                p.insert("firm_stance".into(), json!(firm_stance.as_str()));
                let s: Vec<Value> = sources
                    .iter()
                    .map(|s| json!({"url": s.url, "accessed_at": s.accessed_at.to_string()}))
                    .collect();
                p.insert("sources".into(), Value::Array(s));
            }
            Self::EvaluationRestarted {
                account_id,
                account_name,
                old_start,
                new_start,
                old_earned_before,
                new_earned_before,
                ..
            } => {
                audit_head(&mut p, account_id, account_name);
                p.insert("old_start".into(), opt_date(*old_start));
                p.insert("new_start".into(), json!(new_start.to_string()));
                p.insert("old_earned_before".into(), opt_money(*old_earned_before));
                p.insert("new_earned_before".into(), opt_money(*new_earned_before));
            }
            Self::LimitsChanged {
                account_id,
                limits_version,
                ..
            } => {
                p.insert("account_id".into(), json!(account_id.as_uuid()));
                p.insert("limits_version".into(), json!(limits_version));
            }
            Self::BindingChanged {
                account_id,
                venue_account_id,
                limits_version,
                ..
            } => {
                p.insert("account_id".into(), json!(account_id.as_uuid()));
                p.insert(
                    "venue_account_id".into(),
                    venue_account_id.map_or(Value::Null, |v| json!(v.as_uuid())),
                );
                p.insert("limits_version".into(), json!(limits_version));
            }
            Self::Changed { account_id, .. } | Self::ArchivedNotice { account_id, .. } => {
                p.insert("account_id".into(), json!(account_id.as_uuid()));
            }
        }
        Value::Object(p)
    }
}

fn audit_head(p: &mut Map<String, Value>, id: &AccountId, name: &AccountName) {
    p.insert("account_id".into(), json!(id.as_uuid()));
    p.insert("account_name".into(), json!(name.as_str()));
}

pub fn opt_money(m: Option<MoneyAmount>) -> Value {
    m.map_or(Value::Null, |m| json!(m.to_wire()))
}

pub fn opt_date(d: Option<NaiveDate>) -> Value {
    d.map_or(Value::Null, |d| json!(d.to_string()))
}
