//! Field snapshots for `accounts.created` / `accounts.updated` (changed fields with old and new
//! values, money as strings; S5 contract §4).

use serde_json::{Value, json};

use accounts_enterprise::{Account, PropSelection};

use crate::events::{FieldChange, opt_date, opt_money};

/// `binding` as the user named it: `{connection_id, platform_account}`, or `null`.
pub(crate) fn binding_value(
    connection_id: Option<uuid::Uuid>,
    platform_account: Option<&str>,
) -> Value {
    match (connection_id, platform_account) {
        (Some(c), Some(p)) => json!({"connection_id": c, "platform_account": p}),
        _ => Value::Null,
    }
}

/// Audited fields of an account, in a stable order.
pub(crate) fn fields(account: &Account, binding: Value) -> Vec<(&'static str, Value)> {
    let text = |s: &str| json!(s);
    let (firm_name, plan_name, phase, preset, platform) = match &account.selection {
        None => (
            Value::Null,
            Value::Null,
            Value::Null,
            Value::Null,
            Value::Null,
        ),
        Some(PropSelection::Manual {
            firm_name,
            plan_name,
            phase,
        }) => (
            text(firm_name.as_str()),
            text(plan_name.as_str()),
            text(phase.as_str()),
            Value::Null,
            Value::Null,
        ),
        Some(PropSelection::Preset {
            preset_version_id,
            firm_platform,
            ..
        }) => (
            Value::Null,
            Value::Null,
            Value::Null,
            json!(preset_version_id.as_uuid()),
            firm_platform
                .as_ref()
                .map_or(Value::Null, |p| text(p.as_str())),
        ),
    };
    vec![
        ("kind", text(account.kind.as_str())),
        ("display_name", text(account.display_name.as_str())),
        ("preset_version_id", preset),
        ("firm_platform", platform),
        ("firm_name", firm_name),
        ("plan_name", plan_name),
        ("phase", phase),
        ("size_amount", json!(account.size_amount.to_wire())),
        ("multiplier", json!(account.multiplier.value())),
        ("max_position_qty", json!(account.max_position_qty.value())),
        (
            "commission_per_contract_side",
            opt_money(account.commission_per_contract_side),
        ),
        ("evaluation_start", opt_date(account.evaluation_start)),
        (
            "evaluation_earned_before",
            opt_money(account.evaluation_earned_before),
        ),
        ("binding", binding),
    ]
}

pub(crate) fn created(account: &Account, binding: Value) -> Vec<FieldChange> {
    fields(account, binding)
        .into_iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(field, new)| FieldChange {
            field,
            old: Value::Null,
            new,
        })
        .collect()
}

pub(crate) fn changes(
    before: &Account,
    before_binding: Value,
    after: &Account,
    after_binding: Value,
) -> Vec<FieldChange> {
    fields(before, before_binding)
        .into_iter()
        .zip(fields(after, after_binding))
        .filter(|((_, old), (_, new))| old != new)
        .map(|((field, old), (_, new))| FieldChange { field, old, new })
        .collect()
}
