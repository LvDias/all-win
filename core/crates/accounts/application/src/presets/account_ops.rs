//! Per-account preset operations (S16 AC-2, AC-3, AC-6, AC-11; contract `modules/presets.yaml`,
//! HTTP 6.0.0): `getAccountPreset`, `getAccountPresetDiff`, `applyPresetVersion`,
//! `restorePresetValue`, and the rule values `createAccount` / `updateAccount` write with a preset.
//!
//! The pure rules live in `accounts_enterprise::preset_values::account`; this layer reads the
//! account, its rows and the preset versions, decides the `limits_version` bump and the events, and
//! hands the changed rows to the account save (Malha's agreement, S16 (2)).

use access::{Caller, RequestOrigin, StepUpAction};
use chrono::{DateTime, NaiveDate, Utc};
use serde_json::Value;

use accounts_enterprise::preset_values::account::{
    self as rules, AccountValue, ApplyRequest, DiffRow, FieldView, PRESET_FIELDS, PresetValueError,
};
use accounts_enterprise::preset_values::{AccountRuleValue, PresetRules, RuleValue};
use accounts_enterprise::{
    Account, AccountId, Choice, FirmPlatform, MaxPositionQty, OptInKind, OptInState,
    PresetVersionId, PropSelection, RuleField, RuleOrigin, RuleVersionId,
};
use shared_kernel::UserId;

use crate::commands::{ApplyPresetVersion, RestorePresetValue};
use crate::deps::AccountsDeps;
use crate::error::{AccountsError, Refusals};
use crate::events::{AccountsEvent, FieldChange};
use crate::ports::{
    AccountWrite, CommissionWrite, PresetFacts, PresetRuleSet, RuleSelection, RuleValueWrite,
};
use crate::usecases::support::{
    OptInPlan, current_rules, kind_index, local_date, plan_opt_ins, port_err, rule_selection_of,
    store_err,
};

/// The only fields a `presets.*` `FieldChange` may name: the preset rule fields and the firm
/// platform (all shown in the account's preset dialog).
pub const PRESET_AUDITED_FIELDS: [&str; 9] = [
    "firm_platform",
    RuleField::MaxPositionQty.as_str(),
    RuleField::Drawdown.as_str(),
    RuleField::DailyLossLimit.as_str(),
    RuleField::TradingDayReset.as_str(),
    RuleField::DailyProfitTarget.as_str(),
    RuleField::ConsistencyRule.as_str(),
    RuleField::CommissionPerContractSide.as_str(),
    RuleField::EvaluationProfitTarget.as_str(),
];

// ---- views ---------------------------------------------------------------------------------

/// `FirmReleaseRef`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseRef {
    pub version_number: i32,
    pub published_at: DateTime<Utc>,
    pub verified_on: NaiveDate,
}

impl ReleaseRef {
    fn of(facts: &PresetFacts) -> Self {
        Self {
            version_number: facts.release_version_number,
            published_at: facts.release_published_at,
            verified_on: facts.release_verified_on,
        }
    }
}

/// `AccountRuleField`: the account's value with its origin, and the preset's rule as published.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountFieldView {
    pub view: FieldView,
    pub preset_value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewerVersion {
    pub preset_version_id: PresetVersionId,
    pub release: ReleaseRef,
    pub suspends: bool,
}

/// `AccountPresetState`.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountPresetView {
    pub account_id: AccountId,
    pub preset_version_id: PresetVersionId,
    pub release: ReleaseRef,
    pub firm_platform: Option<FirmPlatform>,
    pub fields: Vec<AccountFieldView>,
    pub newer_version: Option<NewerVersion>,
    pub row_version: i32,
}

/// `PresetDiffRow` with the preset rules as published.
#[derive(Debug, Clone, PartialEq)]
pub struct DiffRowView {
    pub row: DiffRow,
    pub current_preset_value: Option<Value>,
    pub new_preset_value: Option<Value>,
}

/// `PresetDiff`.
#[derive(Debug, Clone, PartialEq)]
pub struct PresetDiffView {
    pub account_id: AccountId,
    pub from_preset_version_id: PresetVersionId,
    pub to_preset_version_id: PresetVersionId,
    pub rows: Vec<DiffRowView>,
    pub firm_platform_required: bool,
    pub firm_platforms: Vec<FirmPlatform>,
    /// The new rule version per kind (`automation`, `copy`) when it differs from the current one.
    pub opt_in_changes: [Option<RuleVersionId>; 2],
}

// ---- shared steps ---------------------------------------------------------------------------

/// A live preset account of the caller with its preset version (404 otherwise: unknown, foreign,
/// archived, or without a preset).
struct Loaded {
    account: Account,
    facts: PresetFacts,
    firm_platform: Option<FirmPlatform>,
}

async fn load(deps: &AccountsDeps, user: UserId, id: AccountId) -> Result<Loaded, AccountsError> {
    let Some(account) = deps.store.get(user, id).await.map_err(store_err)? else {
        return Err(AccountsError::NotFound);
    };
    let Some(PropSelection::Preset {
        preset_version_id,
        firm_platform,
        ..
    }) = account.selection.clone()
    else {
        return Err(AccountsError::NotFound);
    };
    let facts = deps
        .presets
        .version(preset_version_id)
        .await
        .map_err(port_err)?
        .ok_or_else(|| AccountsError::Internal("preset version missing".into()))?;
    Ok(Loaded {
        account,
        facts,
        firm_platform,
    })
}

/// The preset version's rule set for the firm platform (fail closed when it has none).
pub(crate) fn rule_set<'a>(
    facts: &'a PresetFacts,
    firm_platform: Option<&FirmPlatform>,
) -> Result<&'a PresetRuleSet, AccountsError> {
    facts
        .rule_sets
        .iter()
        .find(|s| s.firm_platform.as_ref() == firm_platform)
        .ok_or_else(|| AccountsError::Internal("preset version without this rule set".into()))
}

fn wire_of(set: &PresetRuleSet, field: RuleField) -> Option<Value> {
    set.wire.get(field.as_str()).cloned()
}

fn value_error(e: PresetValueError) -> AccountsError {
    match e {
        PresetValueError::RequiredFieldUnresolved(field) => {
            AccountsError::RequiredFieldUnresolved { field }
        }
        PresetValueError::MaxPositionExceedsPreset => AccountsError::MaxPositionExceedsPreset,
        PresetValueError::ChoiceRequired(field) => AccountsError::ChoiceRequired { field },
        PresetValueError::ValueNotConfirmed => AccountsError::ValueNotConfirmed,
        PresetValueError::UsePresetNotConfirmed(field) => {
            AccountsError::UsePresetNotConfirmed { field }
        }
        PresetValueError::ValueNotDefined => AccountsError::ValueNotDefined,
    }
}

/// `PresetRuleValues` (schema-checked by the infra) as typed values.
pub(crate) fn parse_rule_values(raw: Option<&Value>) -> Result<Vec<RuleValue>, AccountsError> {
    let Some(raw) = raw else {
        return Ok(Vec::new());
    };
    let Some(object) = raw.as_object() else {
        return Err(AccountsError::InvalidRequest {
            pointer: "/rule_values".into(),
        });
    };
    let mut out = Vec::new();
    for (key, value) in object {
        let field = PRESET_FIELDS
            .into_iter()
            .filter(|f| *f != RuleField::MaxPositionQty)
            .find(|f| f.as_str() == key)
            .ok_or_else(|| AccountsError::InvalidRequest {
                pointer: "/rule_values".into(),
            })?;
        let parsed =
            RuleValue::deserialize(field, value).map_err(|_| AccountsError::InvalidRequest {
                pointer: format!("/rule_values/{}", field.as_str()),
            })?;
        out.push(parsed);
    }
    Ok(out)
}

/// The rows of `desired` that change the stored ones (a row written again unchanged is left
/// alone, so its `saved_at` does not move).
fn changed_writes(
    current: &[AccountRuleValue],
    desired: Vec<(RuleField, Option<AccountRuleValue>)>,
) -> Vec<RuleValueWrite> {
    desired
        .into_iter()
        .filter(
            |(field, row)| match (row, current.iter().find(|c| c.field == *field)) {
                (None, None) => false,
                (Some(new), Some(old)) => !new.same_as(old),
                _ => true,
            },
        )
        .map(|(field, row)| RuleValueWrite { field, row })
        .collect()
}

/// The rows after the writes.
fn after_writes(current: &[AccountRuleValue], writes: &[RuleValueWrite]) -> Vec<AccountRuleValue> {
    let mut rows: Vec<AccountRuleValue> = current
        .iter()
        .filter(|r| !writes.iter().any(|w| w.field == r.field))
        .cloned()
        .collect();
    rows.extend(writes.iter().filter_map(|w| w.row.clone()));
    rows
}

/// Create, or an update that selects a preset: every preset field (but the commission, which the
/// account save writes) gets the row the request needs, and the rows of the previous selection go.
pub(crate) fn fill_writes(
    facts: &PresetFacts,
    selection: &Option<PropSelection>,
    raw_values: Option<&Value>,
    max_position_qty: u32,
    current: &[AccountRuleValue],
    now: DateTime<Utc>,
) -> Result<Vec<RuleValueWrite>, AccountsError> {
    let platform = match selection {
        Some(PropSelection::Preset { firm_platform, .. }) => firm_platform.as_ref(),
        _ => None,
    };
    let set = rule_set(facts, platform)?;
    let values = parse_rule_values(raw_values)?;
    let rows =
        rules::fill_from_preset(&set.rules, &values, max_position_qty, now).map_err(value_error)?;
    let desired = PRESET_FIELDS
        .into_iter()
        .filter(|f| *f != RuleField::CommissionPerContractSide)
        .map(|f| (f, rows.iter().find(|r| r.field == f).cloned()))
        .collect();
    Ok(changed_writes(current, desired))
}

/// `updateAccount` on a preset account that keeps its selection but changes the maximum: its row
/// follows (edited, chosen, or gone when it equals the verified value).
pub(crate) fn max_position_writes(
    facts: &PresetFacts,
    selection: &Option<PropSelection>,
    max_position_qty: u32,
    current: &[AccountRuleValue],
    now: DateTime<Utc>,
) -> Result<Vec<RuleValueWrite>, AccountsError> {
    let platform = match selection {
        Some(PropSelection::Preset { firm_platform, .. }) => firm_platform.as_ref(),
        _ => None,
    };
    let set = rule_set(facts, platform)?;
    let row = rules::max_position_row(max_position_qty, &set.rules, now).map_err(value_error)?;
    Ok(changed_writes(
        current,
        vec![(RuleField::MaxPositionQty, row)],
    ))
}

/// `updateAccount` from a preset to a manual selection: the rows that only meant something against
/// the preset (edited, chosen from a conflict) go; the user's own S13 values stay.
pub(crate) fn leave_preset_writes(current: &[AccountRuleValue]) -> Vec<RuleValueWrite> {
    current
        .iter()
        .filter(|r| {
            r.field != RuleField::CommissionPerContractSide
                && PRESET_FIELDS.contains(&r.field)
                && matches!(
                    r.origin,
                    RuleOrigin::Edited | RuleOrigin::ChosenFromConflict
                )
        })
        .map(|r| RuleValueWrite {
            field: r.field,
            row: None,
        })
        .collect()
}

/// An account value as the audit shows it (`FieldChange`): the contracts as a number, a rule value
/// as compact JSON with sorted keys and string amounts.
pub(crate) fn audit_text(value: &AccountValue) -> String {
    match value {
        AccountValue::MaxPosition(n) => n.to_string(),
        AccountValue::Rule(v) => serde_json::to_value(v.to_wire())
            .map(|j| j.to_string())
            .unwrap_or_default(),
    }
}

/// The rule fields `accounts.created` / `accounts.updated` audit next to the account's own
/// fields (Esquadro, audit (b) A); the maximum and the commission are account fields already.
pub const AUDITED_RULE_FIELDS: [RuleField; 6] = [
    RuleField::Drawdown,
    RuleField::DailyLossLimit,
    RuleField::DailyProfitTarget,
    RuleField::TradingDayReset,
    RuleField::ConsistencyRule,
    RuleField::EvaluationProfitTarget,
];

/// The writes give the account its own evaluation target (then it needs a start, Q46).
pub(crate) fn writes_target(writes: &[RuleValueWrite]) -> bool {
    writes.iter().any(|w| {
        w.field == RuleField::EvaluationProfitTarget
            && w.row
                .as_ref()
                .is_some_and(|r| !r.is_unset && r.value.is_some())
    })
}

/// The preset rules a selection uses (`None`: no preset, or no rule set for its platform).
pub(crate) fn preset_rules<'a>(
    facts: Option<&'a PresetFacts>,
    selection: &Option<PropSelection>,
) -> Option<&'a PresetRules> {
    let Some(PropSelection::Preset { firm_platform, .. }) = selection else {
        return None;
    };
    rule_set(facts?, firm_platform.as_ref())
        .ok()
        .map(|s| &s.rules)
}

/// The account's effective value of a rule field as the audit shows it: its own row, else the
/// preset's verified value; `None` = no limit (a tombstone, or nothing confirmed).
fn effective_text(
    rules: Option<&PresetRules>,
    rows: &[AccountRuleValue],
    field: RuleField,
) -> Option<String> {
    match rows.iter().find(|r| r.field == field) {
        Some(r) if r.is_unset => None,
        Some(r) => r.value.clone().map(|v| audit_text(&AccountValue::Rule(v))),
        None => rules
            .and_then(|r| rules::preset_field(r, field))
            .and_then(|p| p.verified().cloned())
            .map(|v| audit_text(&AccountValue::Rule(v))),
    }
}

/// The audit's rule-field changes of a create/update: only the fields whose rows this write
/// changes (Malha, (b) condition 3), with the effective value before and after.
pub(crate) fn rule_write_changes(
    before: (Option<&PresetRules>, &[AccountRuleValue]),
    after_rules: Option<&PresetRules>,
    writes: &[RuleValueWrite],
) -> Vec<FieldChange> {
    let rows_after = after_writes(before.1, writes);
    AUDITED_RULE_FIELDS
        .into_iter()
        .filter(|f| writes.iter().any(|w| w.field == *f))
        .filter_map(|field| {
            let old = effective_text(before.0, before.1, field);
            let new = effective_text(after_rules, &rows_after, field);
            (old != new).then(|| FieldChange {
                field: field.as_str(),
                old,
                new,
            })
        })
        .collect()
}

/// The audit's changed fields between two states of the account's preset fields.
fn field_changes(before: &[FieldView], after: &[FieldView]) -> Vec<FieldChange> {
    let text = |views: &[FieldView], field: RuleField| {
        views
            .iter()
            .find(|v| v.field == field)
            .and_then(|v| v.account_value.as_ref())
            .map(audit_text)
    };
    PRESET_FIELDS
        .into_iter()
        .filter_map(|field| {
            let (old, new) = (text(before, field), text(after, field));
            (old != new).then(|| FieldChange {
                field: field.as_str(),
                old,
                new,
            })
        })
        .collect()
}

fn commission_of(rows: &[AccountRuleValue]) -> Option<accounts_enterprise::MoneyAmount> {
    rows.iter()
        .find(|r| r.field == RuleField::CommissionPerContractSide && !r.is_unset)
        .and_then(|r| match &r.value {
            Some(RuleValue::CommissionPerContractSide(m)) => Some(*m),
            _ => None,
        })
}

/// Invariant: an evaluation target has a start; gained later it defaults to the registration date
/// in the user's zone (Q46).
async fn default_evaluation_start(
    deps: &AccountsDeps,
    user: UserId,
    rules_after: &PresetRules,
    rows_after: &[AccountRuleValue],
    next: &mut Account,
) -> Result<(), AccountsError> {
    if next.evaluation_start.is_none() && rules::has_evaluation_target(rules_after, rows_after) {
        next.evaluation_start = Some(local_date(deps, user, next.created_at).await?);
    }
    Ok(())
}

/// The firm platform of `target` for an account on `stored` (`Err(required)` when it must be
/// named: the target has platforms and the stored one is not among them).
fn target_platform(
    target: &PresetFacts,
    stored: Option<&FirmPlatform>,
) -> Result<Option<FirmPlatform>, ()> {
    if target.firm_platforms.is_empty() {
        return Ok(None);
    }
    match stored {
        Some(p) if target.firm_platforms.contains(p) => Ok(Some(p.clone())),
        _ => Err(()),
    }
}

/// A newer version of the same preset (`presets.version_not_newer` otherwise).
fn is_newer(current: &PresetFacts, target: &PresetFacts) -> bool {
    target.preset_id == current.preset_id
        && target.release_version_number > current.release_version_number
}

async fn state_view(
    deps: &AccountsDeps,
    account: &Account,
    facts: &PresetFacts,
    firm_platform: Option<FirmPlatform>,
    rows: &[AccountRuleValue],
) -> Result<AccountPresetView, AccountsError> {
    let set = rule_set(facts, firm_platform.as_ref())?;
    let fields = rules::field_views(&set.rules, rows, account.max_position_qty.value())
        .into_iter()
        .map(|view| AccountFieldView {
            preset_value: wire_of(set, view.field),
            view,
        })
        .collect();
    let newer_version = match facts.superseded_by {
        None => None,
        Some(id) => match deps.presets.version(id).await.map_err(port_err)? {
            None => None,
            Some(newer) => Some(NewerVersion {
                preset_version_id: newer.preset_version_id,
                release: ReleaseRef::of(&newer),
                suspends: suspends(deps, account, &newer, firm_platform.as_ref()).await?,
            }),
        },
    };
    Ok(AccountPresetView {
        account_id: account.id,
        preset_version_id: facts.preset_version_id,
        release: ReleaseRef::of(facts),
        firm_platform,
        fields,
        newer_version,
        row_version: account.row_version,
    })
}

/// The newer version changes an opt-in rule version of the account (the release job has then set
/// the opt-in to `pending_review`, Q-S5-3).
async fn suspends(
    deps: &AccountsDeps,
    account: &Account,
    newer: &PresetFacts,
    stored_platform: Option<&FirmPlatform>,
) -> Result<bool, AccountsError> {
    let (Some(current), Ok(platform)) = (
        rule_selection_of(account),
        target_platform(newer, stored_platform),
    ) else {
        return Ok(false);
    };
    let new = RuleSelection::Preset {
        preset_version_id: newer.preset_version_id,
        firm_platform: platform,
    };
    let (cur, new) = (
        current_rules(deps, &current).await?,
        current_rules(deps, &new).await?,
    );
    Ok(OptInKind::ALL
        .into_iter()
        .any(|k| cur[kind_index(k)].rule.id != new[kind_index(k)].rule.id))
}

// ---- use cases ------------------------------------------------------------------------------

/// `getAccountPreset`.
pub struct GetAccountPreset {
    deps: AccountsDeps,
}

impl GetAccountPreset {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        caller: &Caller,
        account_id: AccountId,
    ) -> Result<AccountPresetView, AccountsError> {
        let d = &self.deps;
        let l = load(d, caller.user_id, account_id).await?;
        let rows = d
            .store
            .rule_values(caller.user_id, account_id)
            .await
            .map_err(store_err)?;
        state_view(d, &l.account, &l.facts, l.firm_platform, &rows).await
    }
}

/// `getAccountPresetDiff`.
pub struct GetAccountPresetDiff {
    deps: AccountsDeps,
}

impl GetAccountPresetDiff {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    /// `firm_platform`: the platform to compute the rows for (HTTP 6.2.0 optional query; `None`:
    /// the account's own, when the target offers it). Rules (Compasso, 6.1.0/6.2.0):
    /// - a named platform the target does not offer → 422 `accounts.firm_platform_unknown`;
    /// - rows for the named (or the account's) platform; `firm_platform_required` while that
    ///   platform differs from the account's;
    /// - none to use (the target has platforms and the account none it offers) → `rows = []`,
    ///   `firm_platform_required` with the target's `firm_platforms`.
    pub async fn handle(
        &self,
        caller: &Caller,
        account_id: AccountId,
        target_id: PresetVersionId,
        firm_platform: Option<FirmPlatform>,
    ) -> Result<PresetDiffView, AccountsError> {
        let d = &self.deps;
        let l = load(d, caller.user_id, account_id).await?;
        let not_newer = AccountsError::VersionNotNewer {
            pointer: "/query/target_preset_version_id",
        };
        let target = match d.presets.version(target_id).await.map_err(port_err)? {
            Some(t) if is_newer(&l.facts, &t) => t,
            _ => return Err(not_newer),
        };
        let mut view = PresetDiffView {
            account_id,
            from_preset_version_id: l.facts.preset_version_id,
            to_preset_version_id: target.preset_version_id,
            rows: Vec::new(),
            firm_platform_required: false,
            firm_platforms: target.firm_platforms.clone(),
            opt_in_changes: [None, None],
        };
        // The rows depend on the platform's rules: without one, the user names it first.
        let platform = match firm_platform {
            Some(p) if !target.firm_platforms.contains(&p) => {
                return Err(AccountsError::FirmPlatformUnknownInQuery);
            }
            Some(p) => {
                view.firm_platform_required = l.firm_platform.as_ref() != Some(&p);
                Some(p)
            }
            None => match target_platform(&target, l.firm_platform.as_ref()) {
                Ok(p) => p,
                Err(()) => {
                    view.firm_platform_required = true;
                    return Ok(view);
                }
            },
        };
        let rows = d
            .store
            .rule_values(caller.user_id, account_id)
            .await
            .map_err(store_err)?;
        let current = rule_set(&l.facts, l.firm_platform.as_ref())?;
        let next = rule_set(&target, platform.as_ref())?;
        view.rows = rules::diff(
            &rows,
            l.account.max_position_qty.value(),
            &current.rules,
            &next.rules,
        )
        .into_iter()
        .map(|row| DiffRowView {
            current_preset_value: wire_of(current, row.field),
            new_preset_value: wire_of(next, row.field),
            row,
        })
        .collect();
        if let Some(sel) = rule_selection_of(&l.account) {
            let cur = current_rules(d, &sel).await?;
            let new = current_rules(
                d,
                &RuleSelection::Preset {
                    preset_version_id: target.preset_version_id,
                    firm_platform: platform,
                },
            )
            .await?;
            for kind in OptInKind::ALL {
                let (c, n) = (&cur[kind_index(kind)], &new[kind_index(kind)]);
                if c.rule.id != n.rule.id {
                    view.opt_in_changes[kind_index(kind)] = Some(n.rule.id);
                }
            }
        }
        Ok(view)
    }
}

/// `applyPresetVersion`: one save of the new version, the rows and the opt-in choices. Order
/// after the schema: 404 → step-up → 409 → 422; nothing is saved on any error.
pub struct ApplyPresetVersionUseCase {
    deps: AccountsDeps,
}

impl ApplyPresetVersionUseCase {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        caller: &Caller,
        origin: &RequestOrigin,
        cmd: ApplyPresetVersion,
    ) -> Result<AccountPresetView, AccountsError> {
        let d = &self.deps;
        let user = caller.user_id;
        let values = parse_rule_values(cmd.rule_values.as_ref())?;
        let l = load(d, user, cmd.account_id).await?;
        let stored = &l.account;
        let now = d.clock.now();
        let mut refusals = Refusals::default();

        let target = match d
            .presets
            .version(cmd.target_preset_version_id)
            .await
            .map_err(port_err)?
        {
            Some(t) if is_newer(&l.facts, &t) => Some(t),
            _ => {
                refusals.push(AccountsError::VersionNotNewer {
                    pointer: "/target_preset_version_id",
                });
                None
            }
        };
        if target.as_ref().is_some_and(|t| t.superseded_by.is_some()) {
            refusals.push(AccountsError::VersionSuperseded);
        }
        let platform = target.as_ref().and_then(|t| {
            if t.firm_platforms.is_empty() {
                if cmd.firm_platform.is_some() {
                    refusals.push(AccountsError::FirmPlatformUnknown);
                    return None;
                }
                return Some(None);
            }
            match cmd
                .firm_platform
                .clone()
                .or_else(|| l.firm_platform.clone())
            {
                None => {
                    refusals.push(AccountsError::FirmPlatformRequired);
                    None
                }
                Some(p) if !t.firm_platforms.contains(&p) => {
                    refusals.push(AccountsError::FirmPlatformUnknown);
                    None
                }
                Some(p) => Some(Some(p)),
            }
        });

        let rows = d
            .store
            .rule_values(user, cmd.account_id)
            .await
            .map_err(store_err)?;
        let current_set = rule_set(&l.facts, l.firm_platform.as_ref())?;
        let resolved = match (&target, platform) {
            (Some(t), Some(p)) => {
                let set = rule_set(t, p.as_ref())?;
                // `cap_conflict`: the top-level maximum is the answer; `keep_mine` would break the
                // cap (400, 5.4.0).
                let cap_conflict = rules::diff(
                    &rows,
                    stored.max_position_qty.value(),
                    &current_set.rules,
                    &set.rules,
                )
                .iter()
                .any(|r| r.cap_conflict);
                if cap_conflict
                    && cmd
                        .choices
                        .iter()
                        .any(|(f, _)| *f == RuleField::MaxPositionQty)
                {
                    return Err(AccountsError::InvalidRequest {
                        pointer: "/choices/max_position_qty".into(),
                    });
                }
                Some((t, p, set))
            }
            _ => None,
        };

        let plans = match &resolved {
            Some((t, p, _)) => match rule_selection_of(stored) {
                Some(sel) => {
                    let stored_rules = current_rules(d, &sel).await?;
                    let new_rules = current_rules(
                        d,
                        &RuleSelection::Preset {
                            preset_version_id: t.preset_version_id,
                            firm_platform: p.clone(),
                        },
                    )
                    .await?;
                    plan_opt_ins(
                        stored,
                        &cmd.opt_ins,
                        Some(&stored_rules),
                        Some(&new_rules),
                        now,
                        &mut refusals,
                    )
                }
                None => Vec::new(),
            },
            None => Vec::new(),
        };

        let accepts = plans.iter().any(|p| {
            matches!(p, OptInPlan::Set(OptInState::Chosen(c), _) if c.choice == Choice::Accepted)
        });
        if accepts {
            d.step_up
                .authorize(
                    caller,
                    StepUpAction::AccountOptInAccept,
                    Some(stored.id.as_uuid()),
                    cmd.step_up.clone(),
                    origin,
                )
                .await?;
        }
        if cmd.row_version != stored.row_version {
            refusals.push(AccountsError::VersionConflict);
        }
        let plan = match &resolved {
            Some((_, _, set)) => {
                let request = ApplyRequest {
                    choices: cmd.choices.clone(),
                    values,
                    max_position_qty: cmd.max_position_qty.map(|m| m.value()),
                };
                match rules::apply(
                    &rows,
                    stored.max_position_qty.value(),
                    &current_set.rules,
                    &set.rules,
                    &request,
                    now,
                ) {
                    Ok(plan) => Some(plan),
                    Err(e) => {
                        refusals.push(value_error(e));
                        None
                    }
                }
            }
            None => None,
        };
        refusals.first()?;
        let (Some((target, platform, set)), Some(plan)) = (resolved, plan) else {
            return Err(AccountsError::Internal("unresolved apply".into()));
        };

        let writes = changed_writes(&rows, plan.rows);
        let rows_after = after_writes(&rows, &writes);
        let mut next = stored.clone();
        next.selection = Some(PropSelection::Preset {
            preset_version_id: target.preset_version_id,
            firm_id: target.firm_id,
            firm_platform: platform.clone(),
        });
        next.size_amount = target.size_amount;
        next.max_position_qty = MaxPositionQty::new(i64::from(plan.max_position_qty))
            .map_err(|_| AccountsError::MaxPositionExceedsPreset)?;
        next.commission_per_contract_side = commission_of(&rows_after);
        let mut chosen = Vec::new();
        for (kind, p) in OptInKind::ALL.into_iter().zip(plans) {
            if let OptInPlan::Set(state, choice) = p {
                *next.opt_in_mut(kind) = state;
                if let Some((c, rule)) = choice {
                    chosen.push((kind, c, rule));
                }
            }
        }
        default_evaluation_start(d, user, &set.rules, &rows_after, &mut next).await?;

        let mut changes = field_changes(
            &rules::field_views(&current_set.rules, &rows, stored.max_position_qty.value()),
            &rules::field_views(&set.rules, &rows_after, next.max_position_qty.value()),
        );
        if platform != l.firm_platform {
            changes.insert(
                0,
                FieldChange {
                    field: "firm_platform",
                    old: l.firm_platform.as_ref().map(|p| p.as_str().to_owned()),
                    new: platform.as_ref().map(|p| p.as_str().to_owned()),
                },
            );
        }
        // The preset version always changes, so the derived limits do too.
        next.row_version = stored.row_version + 1;
        next.limits_version = stored.limits_version + 1;
        next.updated_at = now;

        let mut events = vec![AccountsEvent::PresetVersionApplied {
            account_id: next.id,
            account_name: next.display_name.clone(),
            at: now,
            from_preset_version_id: l.facts.preset_version_id,
            to_preset_version_id: target.preset_version_id,
            from_version_number: l.facts.release_version_number,
            to_version_number: target.release_version_number,
            changes,
            kept_fields: plan.kept,
        }];
        for (kind, choice, rule) in chosen {
            events.push(AccountsEvent::OptInChosen {
                account_id: next.id,
                account_name: next.display_name.clone(),
                at: now,
                kind,
                choice,
                previous_choice: stored.opt_in(kind).recorded().map(|c| c.choice),
                rule_version_id: rule.rule.id,
                firm_stance: rule.rule.stance,
                sources: rule.sources,
            });
        }
        events.push(AccountsEvent::LimitsChanged {
            account_id: next.id,
            at: now,
            limits_version: next.limits_version,
        });
        events.push(AccountsEvent::Changed {
            account_id: next.id,
            at: now,
        });
        d.store
            .save(AccountWrite {
                account: next.clone(),
                expected_row_version: stored.row_version,
                expected_limits_version: stored.limits_version,
                commission: CommissionWrite::Unchanged,
                rule_values: writes,
                events,
            })
            .await
            .map_err(store_err)?;
        state_view(d, &next, target, platform, &rows_after).await
    }
}

/// `restorePresetValue`: an edited field back to its preset value. Order: 404 → 409 → 422; a field
/// that is not edited is a no-op (200, not audited).
pub struct RestorePresetValueUseCase {
    deps: AccountsDeps,
}

impl RestorePresetValueUseCase {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        caller: &Caller,
        cmd: RestorePresetValue,
    ) -> Result<AccountPresetView, AccountsError> {
        let d = &self.deps;
        let user = caller.user_id;
        let l = load(d, user, cmd.account_id).await?;
        let stored = &l.account;
        if cmd.row_version != stored.row_version {
            return Err(AccountsError::VersionConflict);
        }
        let rows = d
            .store
            .rule_values(user, cmd.account_id)
            .await
            .map_err(store_err)?;
        let set = rule_set(&l.facts, l.firm_platform.as_ref())?;
        let Some(field) = rules::restore(&set.rules, &rows, cmd.field).map_err(value_error)? else {
            return state_view(d, stored, &l.facts, l.firm_platform, &rows).await;
        };

        let now = d.clock.now();
        let writes = vec![RuleValueWrite { field, row: None }];
        let rows_after = after_writes(&rows, &writes);
        let mut next = stored.clone();
        if field == RuleField::MaxPositionQty {
            let (verified, _, _) = rules::preset_max(&set.rules);
            let max = verified.ok_or(AccountsError::ValueNotConfirmed)?;
            next.max_position_qty = MaxPositionQty::new(i64::from(max))
                .map_err(|_| AccountsError::Internal("preset maximum out of range".into()))?;
        }
        next.commission_per_contract_side = commission_of(&rows_after);
        default_evaluation_start(d, user, &set.rules, &rows_after, &mut next).await?;
        let change = field_changes(
            &rules::field_views(&set.rules, &rows, stored.max_position_qty.value()),
            &rules::field_views(&set.rules, &rows_after, next.max_position_qty.value()),
        )
        .into_iter()
        .next()
        .unwrap_or(FieldChange {
            field: field.as_str(),
            old: None,
            new: None,
        });
        next.row_version = stored.row_version + 1;
        next.limits_version = stored.limits_version + 1;
        next.updated_at = now;
        let events = vec![
            AccountsEvent::PresetValueRestored {
                account_id: next.id,
                account_name: next.display_name.clone(),
                at: now,
                change,
            },
            AccountsEvent::LimitsChanged {
                account_id: next.id,
                at: now,
                limits_version: next.limits_version,
            },
            AccountsEvent::Changed {
                account_id: next.id,
                at: now,
            },
        ];
        d.store
            .save(AccountWrite {
                account: next.clone(),
                expected_row_version: stored.row_version,
                expected_limits_version: stored.limits_version,
                commission: CommissionWrite::Unchanged,
                rule_values: writes,
                events,
            })
            .await
            .map_err(store_err)?;
        state_view(d, &next, &l.facts, l.firm_platform.clone(), &rows_after).await
    }
}
