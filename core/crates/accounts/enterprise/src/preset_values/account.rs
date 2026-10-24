//! An account's rules against its preset version (S16 AC-2, AC-3, AC-6, AC-11; contract §2.4–§2.5):
//! only deviations are stored (`account_rule_value`); the preset value applies otherwise. Pure:
//! the use cases read the rows and the preset, call these, and write the desired rows.
//!
//! `max_position_qty` lives on the account (S5); its row only records that it differs from the
//! preset (`edited`) or resolves a not-confirmed maximum (`chosen_from_conflict`).

use chrono::{DateTime, Utc};

use crate::rule_value::{RuleField, RuleOrigin};

use super::{AccountRuleValue, Chose, PresetRules, Provenance, RuleValue};

/// The preset's rule of one field, with its values as `RuleValue`s.
pub type PresetField = Provenance<RuleValue>;

/// Preset fields in contract order (the four required first).
pub const PRESET_FIELDS: [RuleField; 8] = [
    RuleField::MaxPositionQty,
    RuleField::Drawdown,
    RuleField::DailyLossLimit,
    RuleField::TradingDayReset,
    RuleField::DailyProfitTarget,
    RuleField::ConsistencyRule,
    RuleField::CommissionPerContractSide,
    RuleField::EvaluationProfitTarget,
];

fn required(field: RuleField) -> bool {
    matches!(
        field,
        RuleField::MaxPositionQty
            | RuleField::Drawdown
            | RuleField::DailyLossLimit
            | RuleField::TradingDayReset
    )
}

fn map<T, U>(p: &Provenance<T>, f: impl Fn(&T) -> U) -> Provenance<U> {
    match p {
        Provenance::Verified { value, sources } => Provenance::Verified {
            value: f(value),
            sources: sources.clone(),
        },
        Provenance::NotConfirmed {
            published_values,
            reason,
        } => Provenance::NotConfirmed {
            reason: *reason,
            published_values: published_values
                .iter()
                .map(|pv| super::Published {
                    value: f(&pv.value),
                    sources: pv.sources.clone(),
                })
                .collect(),
        },
    }
}

/// The preset's rule for a value field (`None`: the firm has no such rule). Not for
/// `max_position_qty` (see [`preset_max`]).
pub fn preset_field(rules: &PresetRules, field: RuleField) -> Option<PresetField> {
    match field {
        RuleField::Drawdown => Some(map(&rules.drawdown, |v| RuleValue::Drawdown(v.clone()))),
        RuleField::DailyLossLimit => Some(map(&rules.daily_loss_limit, |v| {
            RuleValue::DailyLossLimit(v.clone())
        })),
        RuleField::TradingDayReset => Some(map(&rules.trading_day_reset, |v| {
            RuleValue::TradingDayReset(v.clone())
        })),
        RuleField::DailyProfitTarget => rules
            .daily_profit_target
            .as_ref()
            .map(|p| map(p, |v| RuleValue::DailyProfitTarget(*v))),
        RuleField::ConsistencyRule => rules
            .consistency_rule
            .as_ref()
            .map(|p| map(p, |v| RuleValue::ConsistencyRule(v.clone()))),
        RuleField::CommissionPerContractSide => rules
            .commission_per_contract_side
            .as_ref()
            .map(|p| map(p, |v| RuleValue::CommissionPerContractSide(*v))),
        RuleField::EvaluationProfitTarget => rules
            .evaluation_profit_target
            .as_ref()
            .map(|p| map(p, |v| RuleValue::EvaluationProfitTarget(*v))),
        RuleField::MaxPositionQty
        | RuleField::ProfitTargetOnReach
        | RuleField::ConsistencyThresholdPct => None,
    }
}

/// The preset's maximum position: `(verified value, cap, published values)`; the cap is the
/// highest published value when not confirmed (S16 §2.10), and none when nothing is published
/// (presets 1.2 partial / not_published: the user's own value, uncapped by the preset).
pub fn preset_max(rules: &PresetRules) -> (Option<u32>, Option<u32>, Vec<u32>) {
    let values: Vec<u32> = rules
        .max_position_qty
        .values()
        .iter()
        .map(|m| m.contracts)
        .collect();
    let cap = values.iter().copied().max();
    (
        rules.max_position_qty.verified().map(|m| m.contracts),
        cap,
        values,
    )
}

fn find(stored: &[AccountRuleValue], field: RuleField) -> Option<&AccountRuleValue> {
    stored.iter().find(|r| r.field == field)
}

/// Origin of a field as `getAccountPreset` shows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldOrigin {
    Preset,
    Edited,
    ChosenFromConflict,
    Unresolved,
}

impl FieldOrigin {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preset => "preset",
            Self::Edited => "edited",
            Self::ChosenFromConflict => "chosen_from_conflict",
            Self::Unresolved => "unresolved",
        }
    }
}

/// The account's value of a field: a rule value, or its maximum position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountValue {
    Rule(RuleValue),
    MaxPosition(u32),
}

/// One `AccountRuleField`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldView {
    pub field: RuleField,
    pub origin: FieldOrigin,
    pub chose: Option<(Chose, DateTime<Utc>)>,
    pub account_value: Option<AccountValue>,
}

fn origin_of_row(row: &AccountRuleValue) -> FieldOrigin {
    match row.origin {
        RuleOrigin::ChosenFromConflict => FieldOrigin::ChosenFromConflict,
        RuleOrigin::Edited | RuleOrigin::SetByUser => FieldOrigin::Edited,
    }
}

/// `getAccountPreset.fields`: every field the preset has, plus fields the account added.
pub fn field_views(
    rules: &PresetRules,
    stored: &[AccountRuleValue],
    max_position_qty: u32,
) -> Vec<FieldView> {
    let mut out = Vec::new();
    for field in PRESET_FIELDS {
        let row = find(stored, field);
        if field == RuleField::MaxPositionQty {
            let (origin, chose) = match row {
                Some(r) => (origin_of_row(r), r.chose),
                None => (FieldOrigin::Preset, None),
            };
            out.push(FieldView {
                field,
                origin,
                chose,
                account_value: Some(AccountValue::MaxPosition(max_position_qty)),
            });
            continue;
        }
        let preset = preset_field(rules, field);
        let view = match (row, &preset) {
            // A tombstone (the user removed the rule, S13) is not tracked.
            (Some(r), _) if r.is_unset => FieldView {
                field,
                origin: FieldOrigin::Unresolved,
                chose: None,
                account_value: None,
            },
            (Some(r), _) => FieldView {
                field,
                origin: origin_of_row(r),
                chose: r
                    .chose
                    .filter(|_| r.origin == RuleOrigin::ChosenFromConflict),
                account_value: r.value.clone().map(AccountValue::Rule),
            },
            (None, Some(Provenance::Verified { value, .. })) => FieldView {
                field,
                origin: FieldOrigin::Preset,
                chose: None,
                account_value: Some(AccountValue::Rule(value.clone())),
            },
            (None, Some(Provenance::NotConfirmed { .. })) => FieldView {
                field,
                origin: FieldOrigin::Unresolved,
                chose: None,
                account_value: None,
            },
            (None, None) => continue,
        };
        out.push(view);
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PresetValueError {
    /// 422 `presets.required_field_unresolved` (`/rule_values/<field>` or `/max_position_qty`).
    #[error("presets.required_field_unresolved")]
    RequiredFieldUnresolved(RuleField),
    /// 422 `accounts.max_position_exceeds_preset` (`/max_position_qty`).
    #[error("accounts.max_position_exceeds_preset")]
    MaxPositionExceedsPreset,
    /// 422 `presets.choice_required` (`/choices/<field>`).
    #[error("presets.choice_required")]
    ChoiceRequired(RuleField),
    /// 422 `presets.value_not_confirmed`.
    #[error("presets.value_not_confirmed")]
    ValueNotConfirmed,
    /// 422 `presets.value_not_confirmed` (`/choices/<field>`): `use_preset` on a field the target
    /// does not confirm; it would erase the user's value and leave no limit.
    #[error("presets.value_not_confirmed")]
    UsePresetNotConfirmed(RuleField),
    /// 422 `presets.value_not_defined`: `restorePresetValue` on a field the preset has no rule for;
    /// a restore never removes a limit silently (PROVISIONAL, Esquadro 2026-10-24).
    #[error("presets.value_not_defined")]
    ValueNotDefined,
}

/// The row a requested value needs against the preset rule (`None`: it equals the verified value).
fn row_for(
    field: RuleField,
    value: RuleValue,
    preset: Option<&PresetField>,
    now: DateTime<Utc>,
) -> Option<AccountRuleValue> {
    let (origin, chose) = match preset {
        Some(Provenance::Verified { value: v, .. }) if *v == value => return None,
        Some(Provenance::NotConfirmed {
            published_values, ..
        }) => {
            let chose = if published_values.iter().any(|p| p.value == value) {
                Chose::PublishedValue
            } else {
                Chose::OtherValue
            };
            (RuleOrigin::ChosenFromConflict, Some((chose, now)))
        }
        _ => (RuleOrigin::Edited, None),
    };
    Some(AccountRuleValue {
        field,
        origin,
        value: Some(value),
        is_unset: false,
        chose,
        saved_at: now,
    })
}

/// The row of the account's maximum position against the preset (`None`: it equals the verified
/// value). `updateAccount` keeps it in step when the maximum changes on a preset account.
pub fn max_position_row(
    max: u32,
    rules: &PresetRules,
    now: DateTime<Utc>,
) -> Result<Option<AccountRuleValue>, PresetValueError> {
    let (verified, cap, published) = preset_max(rules);
    if cap.is_some_and(|cap| max > cap) {
        return Err(PresetValueError::MaxPositionExceedsPreset);
    }
    let (origin, chose) = match verified {
        Some(v) if v == max => return Ok(None),
        Some(_) => (RuleOrigin::Edited, None),
        None => (
            RuleOrigin::ChosenFromConflict,
            Some((
                if published.contains(&max) {
                    Chose::PublishedValue
                } else {
                    Chose::OtherValue
                },
                now,
            )),
        ),
    };
    Ok(Some(AccountRuleValue {
        field: RuleField::MaxPositionQty,
        origin,
        value: None,
        is_unset: false,
        chose,
        saved_at: now,
    }))
}

/// Create, or an update that selects a preset: the rows for the account's values (S16 AC-2, AC-6;
/// the commission is the account's own field and written by the account save). Every required
/// not-confirmed field must be resolved; nothing is ever prefilled from a conflict.
pub fn fill_from_preset(
    rules: &PresetRules,
    requested: &[RuleValue],
    max_position_qty: u32,
    now: DateTime<Utc>,
) -> Result<Vec<AccountRuleValue>, PresetValueError> {
    let mut rows: Vec<AccountRuleValue> = max_position_row(max_position_qty, rules, now)?
        .into_iter()
        .collect();
    for field in PRESET_FIELDS
        .into_iter()
        .filter(|f| *f != RuleField::MaxPositionQty)
    {
        let preset = preset_field(rules, field);
        match requested.iter().find(|v| v.field() == field) {
            Some(v) => rows.extend(row_for(field, v.clone(), preset.as_ref(), now)),
            None if required(field)
                && preset.as_ref().is_some_and(Provenance::is_not_confirmed) =>
            {
                return Err(PresetValueError::RequiredFieldUnresolved(field));
            }
            None => {}
        }
    }
    Ok(rows)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyChoice {
    KeepMine,
    UsePreset,
}

/// One `PresetDiffRow`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffRow {
    pub field: RuleField,
    pub account_value: Option<AccountValue>,
    pub current: Option<PresetField>,
    pub new: Option<PresetField>,
    /// For `max_position_qty`: the current / new maxima.
    pub current_max: Option<Vec<u32>>,
    pub new_max: Option<Vec<u32>>,
    pub edited: bool,
    pub requires_choice: bool,
    pub cap_conflict: bool,
}

fn edited(row: Option<&AccountRuleValue>) -> bool {
    row.is_some_and(|r| {
        matches!(
            r.origin,
            RuleOrigin::Edited | RuleOrigin::ChosenFromConflict | RuleOrigin::SetByUser
        )
    })
}

/// `getAccountPresetDiff`: the fields that differ between the account, its current version and the
/// target. `requires_choice`: the user edited it, or the target does not confirm it.
pub fn diff(
    stored: &[AccountRuleValue],
    max_position_qty: u32,
    current: &PresetRules,
    target: &PresetRules,
) -> Vec<DiffRow> {
    let views = field_views(current, stored, max_position_qty);
    let mut rows = Vec::new();
    for field in PRESET_FIELDS {
        let row = find(stored, field);
        let account_value = views
            .iter()
            .find(|v| v.field == field)
            .and_then(|v| v.account_value.clone());
        if field == RuleField::MaxPositionQty {
            let (_, cur_cap, cur) = preset_max(current);
            let (new_verified, new_cap, new) = preset_max(target);
            let changed = cur != new || cur_cap != new_cap;
            let cap_conflict = new_cap.is_some_and(|cap| max_position_qty > cap);
            if changed || cap_conflict {
                rows.push(DiffRow {
                    field,
                    account_value,
                    current: None,
                    new: None,
                    current_max: Some(cur),
                    new_max: Some(new),
                    edited: edited(row),
                    requires_choice: edited(row) || new_verified.is_none(),
                    cap_conflict,
                });
            }
            continue;
        }
        let cur = preset_field(current, field);
        let new = preset_field(target, field);
        if cur == new {
            continue;
        }
        let is_edited = edited(row);
        let not_confirmed = new.as_ref().is_some_and(Provenance::is_not_confirmed);
        rows.push(DiffRow {
            field,
            account_value,
            current: cur,
            new,
            current_max: None,
            new_max: None,
            edited: is_edited,
            requires_choice: is_edited || not_confirmed,
            cap_conflict: false,
        });
    }
    rows
}

/// The desired row of every field after `applyPresetVersion` (`None` = follows the preset), and
/// the edited fields the user kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyPlan {
    pub rows: Vec<(RuleField, Option<AccountRuleValue>)>,
    pub kept: Vec<RuleField>,
    pub max_position_qty: u32,
}

/// What the user sent with `applyPresetVersion`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ApplyRequest {
    pub choices: Vec<(RuleField, ApplyChoice)>,
    pub values: Vec<RuleValue>,
    pub max_position_qty: Option<u32>,
}

/// `applyPresetVersion` (S16 AC-3, AC-11): every `requires_choice` row needs a choice or a value;
/// nothing is preselected. Fields the diff does not list keep their rows.
pub fn apply(
    stored: &[AccountRuleValue],
    max_position_qty: u32,
    current: &PresetRules,
    target: &PresetRules,
    request: &ApplyRequest,
    now: DateTime<Utc>,
) -> Result<ApplyPlan, PresetValueError> {
    let (choices, values, requested_max) =
        (&request.choices, &request.values, request.max_position_qty);
    let choice = |f: RuleField| choices.iter().find(|(c, _)| *c == f).map(|(_, c)| *c);
    let mut plan = ApplyPlan {
        rows: Vec::new(),
        kept: Vec::new(),
        max_position_qty: requested_max.unwrap_or(max_position_qty),
    };
    for row in diff(stored, max_position_qty, current, target) {
        let field = row.field;
        if field == RuleField::MaxPositionQty {
            let max = match (requested_max, choice(field)) {
                (Some(m), _) => m,
                (None, Some(ApplyChoice::KeepMine)) if row.edited => {
                    plan.kept.push(field);
                    max_position_qty
                }
                (None, Some(ApplyChoice::UsePreset)) => match preset_max(target).0 {
                    Some(v) => v,
                    None => return Err(PresetValueError::RequiredFieldUnresolved(field)),
                },
                // No choice, or `keep_mine` on a maximum the user never edited.
                (None, _) if row.requires_choice || row.cap_conflict => {
                    return Err(if row.cap_conflict {
                        PresetValueError::MaxPositionExceedsPreset
                    } else {
                        PresetValueError::ChoiceRequired(field)
                    });
                }
                (None, _) => max_position_qty,
            };
            plan.max_position_qty = max;
            plan.rows.push((field, max_position_row(max, target, now)?));
            continue;
        }
        let new = row.new.as_ref();
        if let Some(v) = values.iter().find(|v| v.field() == field) {
            plan.rows.push((field, row_for(field, v.clone(), new, now)));
            continue;
        }
        match (choice(field), new) {
            (Some(ApplyChoice::KeepMine), _) if row.edited => {
                plan.kept.push(field);
                let kept = find(stored, field).cloned();
                // A kept value equal to the new verified value no longer deviates.
                let still = kept.filter(|k| match (new, &k.value) {
                    (Some(Provenance::Verified { value, .. }), Some(mine)) => value != mine,
                    _ => true,
                });
                plan.rows.push((field, still));
            }
            (Some(ApplyChoice::UsePreset), Some(Provenance::Verified { .. }) | None) => {
                plan.rows.push((field, None))
            }
            (_, Some(Provenance::NotConfirmed { .. })) if required(field) => {
                return Err(PresetValueError::RequiredFieldUnresolved(field));
            }
            (Some(ApplyChoice::UsePreset), Some(Provenance::NotConfirmed { .. })) => {
                return Err(PresetValueError::UsePresetNotConfirmed(field));
            }
            // No choice, or `keep_mine` on a field the user never edited: nothing resolves it.
            _ if row.requires_choice => return Err(PresetValueError::ChoiceRequired(field)),
            // Not listed as a choice: follows the new preset.
            _ => plan.rows.push((field, None)),
        }
    }
    // A requested maximum the diff does not list (the cap and the published values did not change)
    // still answers to the target's cap (Sonar F1).
    if let Some(max) = requested_max
        && max != max_position_qty
        && !plan
            .rows
            .iter()
            .any(|(f, _)| *f == RuleField::MaxPositionQty)
    {
        let row = max_position_row(max, target, now)?;
        plan.rows.push((RuleField::MaxPositionQty, row));
    }
    Ok(plan)
}

/// The account has an evaluation target after its rows: its own value, or the preset's verified
/// one (an optional not-confirmed target left open is none; a tombstone removes it).
pub fn has_evaluation_target(rules: &PresetRules, stored: &[AccountRuleValue]) -> bool {
    match find(stored, RuleField::EvaluationProfitTarget) {
        Some(r) => !r.is_unset && r.value.is_some(),
        None => rules
            .evaluation_profit_target
            .as_ref()
            .is_some_and(|p| p.verified().is_some()),
    }
}

/// `restorePresetValue`: back to the preset's value (`None`: the field is not edited, a no-op). The
/// preset must confirm the field, and define it at all.
pub fn restore(
    rules: &PresetRules,
    stored: &[AccountRuleValue],
    field: RuleField,
) -> Result<Option<RuleField>, PresetValueError> {
    let Some(_) = find(stored, field) else {
        return Ok(None);
    };
    let not_confirmed = if field == RuleField::MaxPositionQty {
        rules.max_position_qty.is_not_confirmed()
    } else {
        match preset_field(rules, field) {
            Some(p) => p.is_not_confirmed(),
            None => return Err(PresetValueError::ValueNotDefined),
        }
    };
    if not_confirmed {
        return Err(PresetValueError::ValueNotConfirmed);
    }
    Ok(Some(field))
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use serde_json::{Value, json};

    use super::super::RawPresetRules;
    use super::*;
    use crate::preset_values::{DailyLoss, MoneyAmount};

    fn now() -> DateTime<Utc> {
        Utc.timestamp_opt(1_800_000_000, 0).unwrap()
    }

    fn src() -> Value {
        json!([{"url": "https://firm.example/rules", "accessed_at": "2026-10-20"}])
    }

    fn verified(value: Value) -> Value {
        json!({"status": "verified", "value": value, "sources": src()})
    }

    fn conflict(a: Value, b: Value) -> Value {
        json!({"status": "not_confirmed", "published_values": [{"value": a, "sources": src()}, {"value": b, "sources": src()}]})
    }

    fn rules(v: Value) -> PresetRules {
        PresetRules::try_from(serde_json::from_value::<RawPresetRules>(v).unwrap()).unwrap()
    }

    fn base() -> Value {
        json!({
            "max_position_qty": verified(json!({"contracts": 50})),
            "drawdown": verified(json!({"type": "trailing_eod", "amount": "2000.00"})),
            "daily_loss_limit": verified(json!({"kind": "none"})),
            "trading_day_reset": verified(json!({"local_time": "17:00", "time_zone": "America/New_York"})),
        })
    }

    fn dd(amount: &str) -> RuleValue {
        RuleValue::deserialize(
            RuleField::Drawdown,
            json!({"type": "trailing_eod", "amount": amount}),
        )
        .unwrap()
    }

    fn money(s: &str) -> MoneyAmount {
        super::super::money(s, "test").unwrap()
    }

    fn partial() -> Value {
        json!({"status": "not_confirmed", "not_confirmed_reason": "partial",
               "quotes": [{"text": "Drawdown: see the dashboard.", "language": "en",
                           "url": "https://firm.example/rules", "accessed_at": "2026-10-20", "page_title": "Rules"}],
               "research": {"searched_on": "2026-10-20", "sources": src()}})
    }

    fn not_published() -> Value {
        json!({"status": "not_confirmed", "not_confirmed_reason": "not_published",
               "research": {"searched_on": "2026-10-20", "sources": src()}})
    }

    /// Presets 1.2: a partial or missing publication behaves like a conflict (never prefilled, a
    /// required field needs the user's value, chosen as `other_value`), and puts no cap on the maximum.
    #[test]
    fn partial_and_unpublished_fields_need_the_users_value_and_cap_nothing() {
        let mut v = base();
        v["drawdown"] = partial();
        v["max_position_qty"] = not_published();
        let r = rules(v);
        assert_eq!(
            r.drawdown,
            Provenance::NotConfirmed {
                published_values: vec![],
                reason: super::super::NotConfirmedReason::Partial
            }
        );
        assert_eq!(
            fill_from_preset(&r, &[], 40, now()),
            Err(PresetValueError::RequiredFieldUnresolved(
                RuleField::Drawdown
            ))
        );
        let rows = fill_from_preset(&r, &[dd("1800.00")], 9000, now()).unwrap();
        for field in [RuleField::MaxPositionQty, RuleField::Drawdown] {
            let row = rows.iter().find(|x| x.field == field).unwrap();
            assert_eq!(row.origin, RuleOrigin::ChosenFromConflict, "{field:?}");
            assert_eq!(
                row.chose.map(|(c, _)| c),
                Some(Chose::OtherValue),
                "{field:?}"
            );
        }
        assert_eq!(preset_max(&r).1, None, "no published value, no cap");
        let views = field_views(&r, &[], 40);
        let view = views
            .iter()
            .find(|x| x.field == RuleField::Drawdown)
            .unwrap();
        assert_eq!(
            (view.origin, view.account_value.clone()),
            (FieldOrigin::Unresolved, None)
        );
        // Restore needs a confirmed preset value.
        assert_eq!(
            restore(&r, &rows, RuleField::Drawdown),
            Err(PresetValueError::ValueNotConfirmed)
        );
    }

    #[test]
    fn fill_stores_only_deviations() {
        let r = rules(base());
        assert!(
            fill_from_preset(&r, &[dd("2000.00")], 50, now())
                .unwrap()
                .is_empty(),
            "same as preset"
        );
        let rows = fill_from_preset(&r, &[dd("1500.00")], 40, now()).unwrap();
        let fields: Vec<_> = rows.iter().map(|x| (x.field, x.origin)).collect();
        assert_eq!(
            fields,
            [
                (RuleField::MaxPositionQty, RuleOrigin::Edited),
                (RuleField::Drawdown, RuleOrigin::Edited)
            ]
        );
        assert_eq!(
            fill_from_preset(&r, &[], 51, now()),
            Err(PresetValueError::MaxPositionExceedsPreset)
        );
    }

    #[test]
    fn not_confirmed_fields_must_be_resolved_and_never_prefilled() {
        let mut v = base();
        v["drawdown"] = conflict(
            json!({"type": "trailing_eod", "amount": "2000.00"}),
            json!({"type": "static", "amount": "2500.00"}),
        );
        v["max_position_qty"] = conflict(json!({"contracts": 40}), json!({"contracts": 50}));
        let r = rules(v);
        assert_eq!(
            fill_from_preset(&r, &[], 40, now()),
            Err(PresetValueError::RequiredFieldUnresolved(
                RuleField::Drawdown
            ))
        );
        let rows = fill_from_preset(&r, &[dd("2000.00")], 45, now()).unwrap();
        let max = rows
            .iter()
            .find(|x| x.field == RuleField::MaxPositionQty)
            .unwrap();
        assert_eq!(
            (max.origin, max.chose.map(|c| c.0)),
            (RuleOrigin::ChosenFromConflict, Some(Chose::OtherValue))
        );
        let d = rows
            .iter()
            .find(|x| x.field == RuleField::Drawdown)
            .unwrap();
        assert_eq!(d.chose.map(|c| c.0), Some(Chose::PublishedValue));
        // An optional not-confirmed field may stay open.
        let mut v = base();
        v["daily_profit_target"] = conflict(json!({"amount": "500"}), json!({"amount": "600"}));
        let r = rules(v);
        assert!(fill_from_preset(&r, &[], 50, now()).unwrap().is_empty());
        let views = field_views(&r, &[], 50);
        assert_eq!(
            views
                .iter()
                .find(|x| x.field == RuleField::DailyProfitTarget)
                .map(|x| x.origin),
            Some(FieldOrigin::Unresolved)
        );
    }

    #[test]
    fn views_show_origins() {
        let r = rules(base());
        let rows = fill_from_preset(&r, &[dd("1500.00")], 50, now()).unwrap();
        let views = field_views(&r, &rows, 50);
        let origin = |f| views.iter().find(|v| v.field == f).map(|v| v.origin);
        assert_eq!(origin(RuleField::Drawdown), Some(FieldOrigin::Edited));
        assert_eq!(origin(RuleField::MaxPositionQty), Some(FieldOrigin::Preset));
        assert_eq!(origin(RuleField::DailyLossLimit), Some(FieldOrigin::Preset));
        assert_eq!(
            origin(RuleField::DailyProfitTarget),
            None,
            "the firm has none"
        );
    }

    #[test]
    fn diff_and_apply_ask_for_edited_fields_only() {
        let current = rules(base());
        let mut t = base();
        t["drawdown"] = verified(json!({"type": "trailing_eod", "amount": "2500.00"}));
        t["daily_loss_limit"] = verified(json!({"kind": "hard", "amount": "1000.00"}));
        t["max_position_qty"] = verified(json!({"contracts": 30}));
        let target = rules(t);
        let stored = fill_from_preset(&current, &[dd("1500.00")], 50, now()).unwrap();
        let rows = diff(&stored, 50, &current, &target);
        let get = |f| rows.iter().find(|r| r.field == f).unwrap();
        assert!(get(RuleField::Drawdown).requires_choice && get(RuleField::Drawdown).edited);
        assert!(!get(RuleField::DailyLossLimit).requires_choice);
        assert!(get(RuleField::MaxPositionQty).cap_conflict);

        assert_eq!(
            apply(
                &stored,
                50,
                &current,
                &target,
                &ApplyRequest {
                    choices: vec![],
                    values: vec![],
                    max_position_qty: Some(30)
                },
                now()
            ),
            Err(PresetValueError::ChoiceRequired(RuleField::Drawdown))
        );
        assert_eq!(
            apply(
                &stored,
                50,
                &current,
                &target,
                &ApplyRequest {
                    choices: vec![(RuleField::Drawdown, ApplyChoice::KeepMine)],
                    values: vec![],
                    max_position_qty: None,
                },
                now(),
            ),
            Err(PresetValueError::MaxPositionExceedsPreset)
        );
        let plan = apply(
            &stored,
            50,
            &current,
            &target,
            &ApplyRequest {
                choices: vec![(RuleField::Drawdown, ApplyChoice::KeepMine)],
                values: vec![],
                max_position_qty: Some(30),
            },
            now(),
        )
        .unwrap();
        assert_eq!(plan.kept, [RuleField::Drawdown]);
        assert_eq!(plan.max_position_qty, 30);
        let row = |f| {
            plan.rows
                .iter()
                .find(|(x, _)| *x == f)
                .map(|(_, r)| r.clone())
        };
        assert!(row(RuleField::Drawdown).flatten().is_some(), "kept");
        assert_eq!(
            row(RuleField::DailyLossLimit),
            Some(None),
            "follows the new preset"
        );
        assert_eq!(
            row(RuleField::MaxPositionQty),
            Some(None),
            "equals the new verified maximum"
        );

        let plan = apply(
            &stored,
            50,
            &current,
            &target,
            &ApplyRequest {
                choices: vec![(RuleField::Drawdown, ApplyChoice::UsePreset)],
                values: vec![],
                max_position_qty: Some(30),
            },
            now(),
        )
        .unwrap();
        assert_eq!(
            plan.rows
                .iter()
                .find(|(x, _)| *x == RuleField::Drawdown)
                .map(|(_, r)| r.clone()),
            Some(None)
        );
        assert!(plan.kept.is_empty());
    }

    #[test]
    fn apply_resolves_a_not_confirmed_target_with_a_value() {
        let current = rules(base());
        let mut t = base();
        t["daily_loss_limit"] = conflict(
            json!({"kind": "hard", "amount": "1000.00"}),
            json!({"kind": "soft_pause", "amount": "1000.00"}),
        );
        let target = rules(t);
        assert_eq!(
            apply(
                &[],
                50,
                &current,
                &target,
                &ApplyRequest {
                    choices: vec![],
                    values: vec![],
                    max_position_qty: None
                },
                now()
            ),
            Err(PresetValueError::RequiredFieldUnresolved(
                RuleField::DailyLossLimit
            ))
        );
        let pick = RuleValue::DailyLossLimit(DailyLoss::Hard(money("1000.00")));
        let plan = apply(
            &[],
            50,
            &current,
            &target,
            &ApplyRequest {
                choices: vec![],
                values: vec![pick],
                max_position_qty: None,
            },
            now(),
        )
        .unwrap();
        let (_, row) = plan
            .rows
            .iter()
            .find(|(x, _)| *x == RuleField::DailyLossLimit)
            .unwrap();
        assert_eq!(
            row.as_ref().map(|r| (r.origin, r.chose.map(|c| c.0))),
            Some((RuleOrigin::ChosenFromConflict, Some(Chose::PublishedValue)))
        );
    }

    fn unconfirmed_target() -> (PresetRules, PresetRules) {
        let mut c = base();
        c["daily_profit_target"] = verified(json!({"amount": "500.00"}));
        let mut t = base();
        t["daily_profit_target"] = not_published();
        (rules(c), rules(t))
    }

    fn ask_apply(
        stored: &[AccountRuleValue],
        choices: Vec<(RuleField, ApplyChoice)>,
        values: Vec<RuleValue>,
    ) -> Result<ApplyPlan, PresetValueError> {
        let (current, target) = unconfirmed_target();
        apply(
            stored,
            50,
            &current,
            &target,
            &ApplyRequest {
                choices,
                values,
                max_position_qty: None,
            },
            now(),
        )
    }

    /// Lince M1: a field the target does not confirm needs a choice or the user's value even when it
    /// was never edited; `keep_mine` keeps nothing there.
    #[test]
    fn unconfirmed_targets_of_unedited_fields_need_the_users_value() {
        let (current, target) = unconfirmed_target();
        let field = RuleField::DailyProfitTarget;
        let ask = ask_apply;
        let rows = diff(&[], 50, &current, &target);
        let row = rows.iter().find(|r| r.field == field).unwrap();
        assert!(row.requires_choice && !row.edited);
        assert_eq!(
            ask(&[], vec![], vec![]),
            Err(PresetValueError::ChoiceRequired(field))
        );
        assert_eq!(
            ask(&[], vec![(field, ApplyChoice::KeepMine)], vec![]),
            Err(PresetValueError::ChoiceRequired(field))
        );
        let mine = RuleValue::DailyProfitTarget(money("450.00"));
        let plan = ask(&[], vec![], vec![mine.clone()]).unwrap();
        let (_, written) = plan.rows.iter().find(|(f, _)| *f == field).unwrap();
        assert_eq!(written.as_ref().and_then(|r| r.value.clone()), Some(mine));
    }

    /// Lince M1: `use_preset` on a field the target does not confirm would erase the user's value
    /// and leave no limit: 422 `presets.value_not_confirmed`; `keep_mine` keeps the user's.
    #[test]
    fn use_preset_refuses_an_unconfirmed_target() {
        let (current, _) = unconfirmed_target();
        let field = RuleField::DailyProfitTarget;
        let ask = ask_apply;
        let own = RuleValue::DailyProfitTarget(money("400.00"));
        let stored = fill_from_preset(&current, std::slice::from_ref(&own), 50, now()).unwrap();
        assert_eq!(
            ask(&stored, vec![(field, ApplyChoice::UsePreset)], vec![]),
            Err(PresetValueError::UsePresetNotConfirmed(field))
        );
        assert_eq!(
            ask(&stored, vec![], vec![]),
            Err(PresetValueError::ChoiceRequired(field))
        );
        let plan = ask(&stored, vec![(field, ApplyChoice::KeepMine)], vec![]).unwrap();
        assert_eq!(plan.kept, [field]);
        let (_, kept) = plan.rows.iter().find(|(f, _)| *f == field).unwrap();
        assert_eq!(kept.as_ref().and_then(|r| r.value.clone()), Some(own));
    }

    /// Sonar F1: the cap holds when the maximum is not a diff row (same cap in both versions).
    #[test]
    fn apply_refuses_a_requested_maximum_above_the_cap_when_the_cap_is_unchanged() {
        let current = rules(base());
        let mut t = base();
        t["drawdown"] = verified(json!({"type": "trailing_eod", "amount": "2500.00"}));
        let target = rules(t);
        let stored = fill_from_preset(&current, &[], 50, now()).unwrap();
        assert!(
            diff(&stored, 50, &current, &target)
                .iter()
                .all(|r| r.field != RuleField::MaxPositionQty),
            "precondition: the maximum is not a diff row"
        );
        let ask = |max| {
            apply(
                &stored,
                50,
                &current,
                &target,
                &ApplyRequest {
                    choices: vec![],
                    values: vec![],
                    max_position_qty: Some(max),
                },
                now(),
            )
        };
        assert_eq!(ask(9999), Err(PresetValueError::MaxPositionExceedsPreset));
        assert_eq!(ask(51), Err(PresetValueError::MaxPositionExceedsPreset));
        let plan = ask(30).unwrap();
        assert_eq!(plan.max_position_qty, 30);
        let (_, row) = plan
            .rows
            .iter()
            .find(|(f, _)| *f == RuleField::MaxPositionQty)
            .unwrap();
        assert_eq!(row.as_ref().map(|r| r.origin), Some(RuleOrigin::Edited));
        let plan = ask(50).unwrap();
        assert!(
            plan.rows
                .iter()
                .all(|(f, _)| *f != RuleField::MaxPositionQty)
        );
    }

    /// `keep_mine` answers only an edited maximum (HTTP 6.4.0).
    #[test]
    fn keep_mine_does_not_answer_an_unedited_maximum() {
        let current = rules(base());
        let mut t = base();
        t["max_position_qty"] = conflict(json!({"contracts": 40}), json!({"contracts": 60}));
        let target = rules(t);
        let stored = fill_from_preset(&current, &[], 50, now()).unwrap();
        let rows = diff(&stored, 50, &current, &target);
        let row = rows
            .iter()
            .find(|r| r.field == RuleField::MaxPositionQty)
            .unwrap();
        assert!(row.requires_choice && !row.edited);
        assert_eq!(
            apply(
                &stored,
                50,
                &current,
                &target,
                &ApplyRequest {
                    choices: vec![(RuleField::MaxPositionQty, ApplyChoice::KeepMine)],
                    values: vec![],
                    max_position_qty: None,
                },
                now(),
            ),
            Err(PresetValueError::ChoiceRequired(RuleField::MaxPositionQty))
        );
    }

    /// `use_preset` on a required field the target does not confirm: the required refusal first.
    #[test]
    fn use_preset_on_a_required_unconfirmed_field_is_unresolved() {
        let current = rules(base());
        let mut t = base();
        t["drawdown"] = not_published();
        let target = rules(t);
        let stored =
            fill_from_preset(&current, std::slice::from_ref(&dd("1500.00")), 50, now()).unwrap();
        assert_eq!(
            apply(
                &stored,
                50,
                &current,
                &target,
                &ApplyRequest {
                    choices: vec![(RuleField::Drawdown, ApplyChoice::UsePreset)],
                    values: vec![],
                    max_position_qty: None,
                },
                now(),
            ),
            Err(PresetValueError::RequiredFieldUnresolved(
                RuleField::Drawdown
            ))
        );
    }

    #[test]
    fn restore_rules() {
        let r = rules(base());
        let stored = fill_from_preset(&r, &[dd("1500.00")], 50, now()).unwrap();
        assert_eq!(
            restore(&r, &stored, RuleField::Drawdown),
            Ok(Some(RuleField::Drawdown))
        );
        assert_eq!(
            restore(&r, &stored, RuleField::DailyLossLimit),
            Ok(None),
            "not edited: no-op"
        );
        let mut v = base();
        v["drawdown"] = conflict(
            json!({"type": "trailing_eod", "amount": "2000.00"}),
            json!({"type": "static", "amount": "2500.00"}),
        );
        let nc = rules(v);
        let stored = fill_from_preset(&nc, &[dd("2000.00")], 50, now()).unwrap();
        assert_eq!(
            restore(&nc, &stored, RuleField::Drawdown),
            Err(PresetValueError::ValueNotConfirmed)
        );
    }

    /// A restore never removes a limit silently: an edited field the preset has no rule for is
    /// refused (PROVISIONAL, Esquadro 2026-10-24).
    #[test]
    fn restore_refuses_a_field_the_preset_does_not_define() {
        let r = rules(base());
        let own = RuleValue::DailyProfitTarget(money("400.00"));
        let stored = fill_from_preset(&r, &[own], 50, now()).unwrap();
        assert!(find(&stored, RuleField::DailyProfitTarget).is_some());
        assert_eq!(
            restore(&r, &stored, RuleField::DailyProfitTarget),
            Err(PresetValueError::ValueNotDefined)
        );
    }
}
