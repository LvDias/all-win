//! Field snapshots for `accounts.created` / `accounts.updated` / `accounts.archived` (changed
//! fields with old and new values as strings; S5 contract §4, `FieldChange`).

use accounts_enterprise::{Account, PropSelection};

use crate::events::FieldChange;

/// The only fields a `FieldChange` may name (Esquadro, `safe_to_display`): the account form's
/// fields, then the preset rule fields (S16 audit (b) A, the names of `PRESET_AUDITED_FIELDS`).
/// A new field is added here only after deciding it is safe to show in the audit log.
pub const AUDITED_FIELDS: [&str; 20] = [
    "kind",
    "display_name",
    "preset_version_id",
    "firm_platform",
    "firm_name",
    "plan_name",
    "phase",
    "size_amount",
    "multiplier",
    "max_position_qty",
    "commission_per_contract_side",
    "evaluation_start",
    "evaluation_earned_before",
    "binding",
    "drawdown",
    "daily_loss_limit",
    "daily_profit_target",
    "trading_day_reset",
    "consistency_rule",
    "evaluation_profit_target",
];

/// The account's own audited fields, in `fields` order.
const ACCOUNT_FIELDS: [&str; 14] = [
    "kind",
    "display_name",
    "preset_version_id",
    "firm_platform",
    "firm_name",
    "plan_name",
    "phase",
    "size_amount",
    "multiplier",
    "max_position_qty",
    "commission_per_contract_side",
    "evaluation_start",
    "evaluation_earned_before",
    "binding",
];

/// Audited fields of an account, in `ACCOUNT_FIELDS` order. `binding`: the platform account
/// name as the user knows it (`None` when unbound).
pub(crate) fn fields(account: &Account, binding: Option<String>) -> [Option<String>; 14] {
    let (firm_name, plan_name, phase, preset, platform) = match &account.selection {
        None => (None, None, None, None, None),
        Some(PropSelection::Manual {
            firm_name,
            plan_name,
            phase,
        }) => (
            Some(firm_name.as_str().to_owned()),
            Some(plan_name.as_str().to_owned()),
            Some(phase.as_str().to_owned()),
            None,
            None,
        ),
        Some(PropSelection::Preset {
            preset_version_id,
            firm_platform,
            ..
        }) => (
            None,
            None,
            None,
            Some(preset_version_id.as_uuid().to_string()),
            firm_platform.as_ref().map(|p| p.as_str().to_owned()),
        ),
    };
    [
        Some(account.kind.as_str().to_owned()),
        Some(account.display_name.as_str().to_owned()),
        preset,
        platform,
        firm_name,
        plan_name,
        phase,
        Some(account.size_amount.to_wire()),
        Some(account.multiplier.value().to_string()),
        Some(account.max_position_qty.value().to_string()),
        account.commission_per_contract_side.map(|m| m.to_wire()),
        account.evaluation_start.map(|d| d.to_string()),
        account.evaluation_earned_before.map(|m| m.to_wire()),
        binding,
    ]
}

pub(crate) fn created(account: &Account, binding: Option<String>) -> Vec<FieldChange> {
    ACCOUNT_FIELDS
        .into_iter()
        .zip(fields(account, binding))
        .filter_map(|(field, new)| {
            new.map(|new| FieldChange {
                field,
                old: None,
                new: Some(new),
            })
        })
        .collect()
}

pub(crate) fn changes(
    before: &Account,
    before_binding: Option<String>,
    after: &Account,
    after_binding: Option<String>,
) -> Vec<FieldChange> {
    ACCOUNT_FIELDS
        .into_iter()
        .zip(
            fields(before, before_binding)
                .into_iter()
                .zip(fields(after, after_binding)),
        )
        .filter(|(_, (old, new))| old != new)
        .map(|(field, (old, new))| FieldChange { field, old, new })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presets::account_ops::AUDITED_RULE_FIELDS;

    #[test]
    fn audited_fields_are_the_account_fields_then_the_rule_field_names() {
        let rules: Vec<&str> = AUDITED_RULE_FIELDS.iter().map(|f| f.as_str()).collect();
        assert_eq!(&AUDITED_FIELDS[..14], &ACCOUNT_FIELDS[..]);
        assert_eq!(&AUDITED_FIELDS[14..], &rules[..]);
    }
}
