//! Events the accounts module writes to `app.outbox` in the same transaction as the change.
//! - `accounts.*`: the user-visible audit (S5 contract §4, category `config`), in the common
//!   envelope `{actor, details, refs}`: `details` is exactly `AuditDetailsByCode[code]`, `refs`
//!   carries `account_id` + `account_name` for the audit entry's columns.
//! - `account.limits_changed` / `account.binding_changed`: ADR-0006 / ADR-0008 consumers.
//! - `account.changed` / `account.archived`: the SSE notifications (the client refetches).

use chrono::{DateTime, NaiveDate, Utc};
use serde_json::{Map, Value, json};

use accounts_enterprise::{
    AccountId, AccountName, Choice, FirmStance, LifecycleState, MoneyAmount, OptInKind,
    RuleVersionId, VenueAccountId,
};

use crate::ports::{RouteRef, RuleSource};

/// One changed field (`FieldChange`); `None` stands for "absent". `field` is one of
/// `AUDITED_FIELDS`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldChange {
    pub field: &'static str,
    pub old: Option<String>,
    pub new: Option<String>,
}

impl FieldChange {
    fn to_json(&self) -> Value {
        let mut c = Map::new();
        c.insert("field".into(), json!(self.field));
        if let Some(old) = &self.old {
            c.insert("old".into(), json!(old));
        }
        if let Some(new) = &self.new {
            c.insert("new".into(), json!(new));
        }
        Value::Object(c)
    }
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
        /// The binding it released, if any.
        changes: Vec<FieldChange>,
        removed_routes: Vec<RouteRef>,
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
        profit_target: MoneyAmount,
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

/// Every type this module writes to the outbox (the app's `emitted_event_types()`). The last
/// four have no consumer in this wave (allowlisted as `from_now`, Esquadro decision (b)).
pub const EMITTED_EVENT_TYPES: [&str; 10] = [
    "accounts.created",
    "accounts.updated",
    "accounts.archived",
    "accounts.lifecycle_changed",
    "accounts.opt_in_chosen",
    "accounts.evaluation_restarted",
    "account.limits_changed",
    "account.binding_changed",
    "account.changed",
    "account.archived",
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

    /// Outbox payload: the audit envelope for `accounts.*`, `{kind, …}` for the others.
    pub fn payload(&self) -> Value {
        let changes =
            |c: &[FieldChange]| Value::Array(c.iter().map(FieldChange::to_json).collect());
        let details = match self {
            Self::Created { fields, .. } => json!({"changes": changes(fields)}),
            Self::Updated { changes: c, .. } => json!({"changes": changes(c)}),
            Self::Archived {
                changes: c,
                removed_routes,
                ..
            } => {
                let r: Vec<Value> = removed_routes
                    .iter()
                    .map(|r| json!({"source_id": r.source_id, "source_name": r.source_name}))
                    .collect();
                json!({"changes": changes(c), "removed_routes": r, "removed_follower_links": []})
            }
            Self::LifecycleChanged {
                previous,
                new,
                had_open_position,
                ..
            } => json!({
                "previous_state": previous.as_str(),
                "new_state": new.as_str(),
                "had_open_position": had_open_position,
            }),
            Self::OptInChosen {
                kind,
                choice,
                previous_choice,
                rule_version_id,
                firm_stance,
                sources,
                ..
            } => {
                let s: Vec<Value> = sources
                    .iter()
                    .map(|s| json!({"url": s.url, "accessed_at": s.accessed_at.to_string()}))
                    .collect();
                let mut d = json!({
                    "kind": kind.as_str(),
                    "choice": choice.as_str(),
                    "rule_version_id": rule_version_id.as_uuid(),
                    "firm_stance": firm_stance.as_str(),
                    "sources": s,
                });
                if let (Some(p), Some(d)) = (previous_choice, d.as_object_mut()) {
                    d.insert("previous_choice".into(), json!(p.as_str()));
                }
                d
            }
            Self::EvaluationRestarted {
                old_start,
                new_start,
                old_earned_before,
                new_earned_before,
                profit_target,
                ..
            } => {
                // `DetailsEvaluationRestarted` (contract 4.4.0).
                let mut d = Map::new();
                d.insert("start".into(), json!(new_start.to_string()));
                d.insert("profit_target".into(), json!(profit_target.to_wire()));
                if let Some(v) = old_start {
                    d.insert("previous_start".into(), json!(v.to_string()));
                }
                if let Some(v) = new_earned_before {
                    d.insert("earned_before".into(), json!(v.to_wire()));
                }
                if let Some(v) = old_earned_before {
                    d.insert("previous_earned_before".into(), json!(v.to_wire()));
                }
                Value::Object(d)
            }
            _ => return self.notice_payload(),
        };
        let (account_id, account_name) = self.audit_refs().unwrap_or_default();
        json!({
            // Every S5 audit code answers a request of the account's user.
            "actor": "user",
            "details": details,
            "refs": {"account_id": account_id, "account_name": account_name},
        })
    }

    fn audit_refs(&self) -> Option<(String, String)> {
        match self {
            Self::Created {
                account_id,
                account_name,
                ..
            }
            | Self::Updated {
                account_id,
                account_name,
                ..
            }
            | Self::Archived {
                account_id,
                account_name,
                ..
            }
            | Self::LifecycleChanged {
                account_id,
                account_name,
                ..
            }
            | Self::OptInChosen {
                account_id,
                account_name,
                ..
            }
            | Self::EvaluationRestarted {
                account_id,
                account_name,
                ..
            } => Some((
                account_id.as_uuid().to_string(),
                account_name.as_str().to_owned(),
            )),
            _ => None,
        }
    }

    /// `account.*` notices (ADR-0006/0008, SSE): `{kind, account_id, …}`.
    fn notice_payload(&self) -> Value {
        let mut p = Map::new();
        p.insert("kind".into(), json!(self.event_type()));
        match self {
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
            // Audit codes take the envelope (`payload`).
            _ => {}
        }
        Value::Object(p)
    }
}
