//! Shared steps of the account use cases: selection and binding resolution, views, dates.

use std::collections::HashMap;

use chrono::NaiveDate;

use accounts_enterprise::{
    Account, AccountKind, Choice, MaxPositionQty, OptInKind, OptInState, PresetVersionId,
    PropSelection, SizeAmount, VenueAccountId,
};
use shared_kernel::UserId;

use crate::commands::{BindingRef, OptInChoices, SelectionInput};
use crate::deps::AccountsDeps;
use crate::error::{AccountsError, Refusals};
use crate::ports::{
    BindingView, CurrentRule, PortError, PresetFacts, RuleSelection, StoreError, VenueKind,
};
use crate::usecases::opt_in_error;
use crate::views::{AccountView, OptInView, PropView};

pub(crate) fn store_err(e: StoreError) -> AccountsError {
    match e {
        StoreError::NameTaken => AccountsError::NameTaken,
        StoreError::BindingTaken => AccountsError::PlatformAccountTaken,
        StoreError::VersionConflict => AccountsError::VersionConflict,
        StoreError::CreateKeyTaken => AccountsError::IdempotencyKeyReused,
        StoreError::Other(m) => AccountsError::Internal(m),
    }
}

pub(crate) fn port_err(e: PortError) -> AccountsError {
    AccountsError::Internal(e.0)
}

/// `copy` is asked only of followers (Q-S5-12); the follower flow (S7/S19) makes that choice.
/// No operation of S5 adds a follower, so `copy` is never applicable here.
pub(crate) const fn applicable(kind: OptInKind) -> bool {
    matches!(kind, OptInKind::Automation)
}

/// The rule versions of a stored account (`None` for real accounts).
pub(crate) fn rule_selection_of(account: &Account) -> Option<RuleSelection> {
    match &account.selection {
        None => None,
        Some(PropSelection::Manual { .. }) => Some(RuleSelection::Generic),
        Some(PropSelection::Preset {
            preset_version_id,
            firm_platform,
            ..
        }) => Some(RuleSelection::Preset {
            preset_version_id: *preset_version_id,
            firm_platform: firm_platform.clone(),
        }),
    }
}

pub(crate) async fn current_rule(
    deps: &AccountsDeps,
    selection: &RuleSelection,
    kind: OptInKind,
) -> Result<CurrentRule, AccountsError> {
    deps.rules
        .current(selection, kind)
        .await
        .map_err(port_err)?
        .ok_or_else(|| AccountsError::Internal(format!("no {} rule version", kind.as_str())))
}

/// Both kinds' current rules for a selection.
pub(crate) async fn current_rules(
    deps: &AccountsDeps,
    selection: &RuleSelection,
) -> Result<[CurrentRule; 2], AccountsError> {
    Ok([
        current_rule(deps, selection, OptInKind::Automation).await?,
        current_rule(deps, selection, OptInKind::Copy).await?,
    ])
}

/// Today in the user's time zone (evaluation dates, S13b).
pub(crate) async fn today(
    deps: &AccountsDeps,
    user_id: UserId,
) -> Result<NaiveDate, AccountsError> {
    local_date(deps, user_id, deps.clock.now()).await
}

/// The date of `at` in the user's time zone.
pub(crate) async fn local_date(
    deps: &AccountsDeps,
    user_id: UserId,
    at: chrono::DateTime<chrono::Utc>,
) -> Result<NaiveDate, AccountsError> {
    let tz = deps.time_zones.time_zone(user_id).await.map_err(port_err)?;
    Ok(at.with_timezone(&tz).date_naive())
}

/// The prop selection a request resolves to.
pub(crate) struct ResolvedSelection {
    pub selection: Option<PropSelection>,
    pub size: SizeAmount,
    pub rules: Option<RuleSelection>,
    /// The resolved rules carry an evaluation target (only presets, S16).
    pub has_evaluation_target: bool,
    /// The selected preset version (S16: its rules fill the account's rule values).
    pub preset: Option<PresetFacts>,
}

/// Resolves the selection against the catalog, pushing 422s. `None` when the selection cannot
/// be resolved (its refusal is already pushed). `stored_preset`: the preset of the account being
/// updated, to send a newer version of the same preset to `applyPresetVersion`.
pub(crate) async fn resolve_selection(
    deps: &AccountsDeps,
    kind: AccountKind,
    input: &SelectionInput,
    size: Option<SizeAmount>,
    max_position: MaxPositionQty,
    stored_preset: Option<&PresetFacts>,
    refusals: &mut Refusals,
) -> Result<Option<ResolvedSelection>, AccountsError> {
    let manual_size = || {
        size.ok_or_else(|| AccountsError::InvalidRequest {
            pointer: "/size_amount".into(),
        })
    };
    match (kind, input) {
        (AccountKind::Real, _) => Ok(Some(ResolvedSelection {
            selection: None,
            size: manual_size()?,
            rules: None,
            has_evaluation_target: false,
            preset: None,
        })),
        (
            AccountKind::Prop,
            SelectionInput::Manual {
                firm_name,
                plan_name,
                phase,
            },
        ) => Ok(Some(ResolvedSelection {
            selection: Some(PropSelection::Manual {
                firm_name: firm_name.clone(),
                plan_name: plan_name.clone(),
                phase: phase.clone(),
            }),
            size: manual_size()?,
            rules: Some(RuleSelection::Generic),
            has_evaluation_target: false,
            preset: None,
        })),
        (
            AccountKind::Prop,
            SelectionInput::Preset {
                preset_version_id,
                firm_platform,
            },
        ) => {
            let Some(facts) = deps
                .presets
                .version(*preset_version_id)
                .await
                .map_err(port_err)?
            else {
                refusals.push(AccountsError::PresetVersionUnknown);
                return Ok(None);
            };
            match stored_preset {
                // Update: a newer version of the same preset is applied with applyPresetVersion.
                Some(stored)
                    if stored.preset_id == facts.preset_id
                        && stored.preset_version_id != facts.preset_version_id =>
                {
                    refusals.push(AccountsError::UseApplyVersion);
                    return Ok(None);
                }
                Some(stored) if stored.preset_version_id == facts.preset_version_id => {}
                // Create, or another preset: only the current version (S16 AC-3).
                _ if !facts.is_current => {
                    refusals.push(AccountsError::PresetVersionUnknown);
                    return Ok(None);
                }
                _ => {}
            }
            match (facts.firm_platforms.is_empty(), firm_platform) {
                (false, None) => {
                    refusals.push(AccountsError::FirmPlatformRequired);
                    return Ok(None);
                }
                (true, Some(_)) => {
                    refusals.push(AccountsError::FirmPlatformUnknown);
                    return Ok(None);
                }
                (false, Some(p)) if !facts.firm_platforms.contains(p) => {
                    refusals.push(AccountsError::FirmPlatformUnknown);
                    return Ok(None);
                }
                _ => {}
            }
            if facts.max_position_qty.is_some_and(|max| max_position > max) {
                refusals.push(AccountsError::MaxPositionExceedsPreset);
            }
            Ok(Some(ResolvedSelection {
                selection: Some(PropSelection::Preset {
                    preset_version_id: *preset_version_id,
                    firm_id: facts.firm_id,
                    firm_platform: firm_platform.clone(),
                }),
                size: facts.size_amount,
                rules: Some(RuleSelection::Preset {
                    preset_version_id: *preset_version_id,
                    firm_platform: firm_platform.clone(),
                }),
                has_evaluation_target: facts.has_evaluation_target,
                preset: Some(facts),
            }))
        }
        (AccountKind::Prop, SelectionInput::None) => Err(AccountsError::InvalidRequest {
            pointer: "/firm_name".into(),
        }),
    }
}

/// Resolves a binding request through gateway, pushing 409/422s. `Ok(None)` when it cannot be
/// bound (refusal pushed). `except`: the account being updated.
pub(crate) async fn resolve_binding(
    deps: &AccountsDeps,
    user_id: UserId,
    binding: &BindingRef,
    except: Option<accounts_enterprise::AccountId>,
    refusals: &mut Refusals,
) -> Result<Option<VenueAccountId>, AccountsError> {
    let Some(facts) = deps
        .venues
        .resolve(user_id, binding.connection_id, &binding.platform_account)
        .await
        .map_err(port_err)?
    else {
        refusals.push(AccountsError::PlatformAccountUnknown);
        return Ok(None);
    };
    if !deps.production && matches!(facts.kind, VenueKind::Live | VenueKind::Unknown) {
        refusals.push(AccountsError::PlatformAccountKindNotAllowed);
        return Ok(None);
    }
    if deps
        .store
        .binding_taken(user_id, facts.venue_account_id, except)
        .await
        .map_err(store_err)?
    {
        refusals.push(AccountsError::PlatformAccountTaken);
        return Ok(None);
    }
    Ok(Some(facts.venue_account_id))
}

/// Builds the response views of a batch of accounts with one call per port where possible.
pub(crate) async fn views(
    deps: &AccountsDeps,
    user_id: UserId,
    accounts: Vec<Account>,
) -> Result<Vec<AccountView>, AccountsError> {
    let bound: Vec<VenueAccountId> = accounts
        .iter()
        .filter_map(|a| a.binding.map(|b| b.venue_account_id))
        .collect();
    let bindings: HashMap<VenueAccountId, BindingView> = if bound.is_empty() {
        HashMap::new()
    } else {
        deps.venues
            .describe(user_id, &bound)
            .await
            .map_err(port_err)?
            .into_iter()
            .map(|b| (b.venue_account_id, b))
            .collect()
    };
    let mut rules: HashMap<String, [CurrentRule; 2]> = HashMap::new();
    let mut presets: HashMap<PresetVersionId, PresetFacts> = HashMap::new();
    let mut out = Vec::with_capacity(accounts.len());
    for account in accounts {
        let opt_ins = match rule_selection_of(&account) {
            None => None,
            Some(sel) => {
                let key = format!("{sel:?}");
                if !rules.contains_key(&key) {
                    let r = current_rules(deps, &sel).await?;
                    rules.insert(key.clone(), r);
                }
                rules.get(&key).cloned()
            }
        };
        let prop = match &account.selection {
            None => None,
            Some(PropSelection::Manual {
                firm_name,
                plan_name,
                phase,
            }) => Some(PropView {
                firm_name: firm_name.as_str().to_owned(),
                plan_name: plan_name.as_str().to_owned(),
                phase: phase.as_str().to_owned(),
                firm_id: None,
                preset_version_id: None,
                firm_platform: None,
                preset_update_available: None,
            }),
            Some(PropSelection::Preset {
                preset_version_id,
                firm_id,
                firm_platform,
            }) => {
                if !presets.contains_key(preset_version_id) {
                    let f = deps
                        .presets
                        .version(*preset_version_id)
                        .await
                        .map_err(port_err)?
                        .ok_or_else(|| AccountsError::Internal("preset version missing".into()))?;
                    presets.insert(*preset_version_id, f);
                }
                let f = presets
                    .get(preset_version_id)
                    .ok_or_else(|| AccountsError::Internal("preset cache".into()))?;
                Some(PropView {
                    firm_name: f.firm_name.clone(),
                    plan_name: f.plan_name.clone(),
                    phase: f.phase.clone(),
                    firm_id: Some(*firm_id),
                    preset_version_id: Some(*preset_version_id),
                    firm_platform: firm_platform.clone(),
                    preset_update_available: Some(!f.is_current),
                })
            }
        };
        let binding = account
            .binding
            .and_then(|b| bindings.get(&b.venue_account_id).cloned());
        let has_open_position = match account.binding {
            Some(b) => deps
                .venues
                .has_open_position(user_id, b.venue_account_id)
                .await
                .map_err(port_err)?,
            None => false,
        };
        let opt_view = |kind: OptInKind| OptInView {
            kind,
            state: *account.opt_in(kind),
            firm_stance: opt_ins.as_ref().map(|r| r[kind_index(kind)].rule.stance),
            current_rule_version_id: opt_ins.as_ref().map(|r| r[kind_index(kind)].rule.id),
        };
        let automation = opt_view(OptInKind::Automation);
        let copy = opt_view(OptInKind::Copy);
        out.push(AccountView {
            account,
            prop,
            binding,
            automation,
            copy,
            has_open_position,
        });
    }
    Ok(out)
}

pub(crate) async fn view(
    deps: &AccountsDeps,
    user_id: UserId,
    account: Account,
) -> Result<AccountView, AccountsError> {
    views(deps, user_id, vec![account])
        .await?
        .pop()
        .ok_or_else(|| AccountsError::Internal("empty view".into()))
}

pub(crate) const fn kind_index(kind: OptInKind) -> usize {
    match kind {
        OptInKind::Automation => 0,
        OptInKind::Copy => 1,
    }
}

/// The bound platform account as the user named it, for the audit (`None` when unbound). An
/// account gateway no longer describes keeps its opaque id, so the change is still visible.
pub(crate) async fn bound_platform_account(
    deps: &AccountsDeps,
    user_id: UserId,
    account: &Account,
) -> Result<Option<String>, AccountsError> {
    let Some(b) = account.binding else {
        return Ok(None);
    };
    let described = deps
        .venues
        .describe(user_id, &[b.venue_account_id])
        .await
        .map_err(port_err)?;
    Ok(Some(described.first().map_or_else(
        || b.venue_account_id.as_uuid().to_string(),
        |v| v.platform_account.clone(),
    )))
}

/// How one kind's opt-in moves with a request.
pub(crate) enum OptInPlan {
    Keep,
    Set(OptInState, Option<(Choice, CurrentRule)>),
}

/// Opt-in moves of this request (D16): a kind whose rule version this request changes is
/// re-selected (its choice is then required if the new version needs one); otherwise a sent
/// choice is a later change, as `setAccountOptIn`.
pub(crate) fn plan_opt_ins(
    stored: &Account,
    requests: &OptInChoices,
    stored_rules: Option<&[CurrentRule; 2]>,
    new_rules: Option<&[CurrentRule; 2]>,
    now: chrono::DateTime<chrono::Utc>,
    refusals: &mut Refusals,
) -> Vec<OptInPlan> {
    OptInKind::ALL
        .into_iter()
        .map(|kind| {
            let request = requests.get(kind);
            let Some(rules) = new_rules else {
                if stored.kind == AccountKind::Real && request.is_some() {
                    refusals.push(AccountsError::OptInNotApplicable { kind });
                }
                return OptInPlan::Keep;
            };
            let current = &rules[kind_index(kind)];
            let version_changes =
                stored_rules.is_none_or(|s| s[kind_index(kind)].rule.id != current.rule.id);
            if version_changes && kind == OptInKind::Automation {
                return match OptInState::select(&current.rule, applicable(kind), request, now) {
                    Ok(state) => {
                        let choice = match state {
                            OptInState::Chosen(c) => Some((c.choice, current.clone())),
                            _ => None,
                        };
                        OptInPlan::Set(state, choice)
                    }
                    Err(e) => {
                        refusals.push(opt_in_error(kind, e));
                        OptInPlan::Keep
                    }
                };
            }
            // `copy` is never required here; a follower whose copy version changes goes to
            // pending_review through the follower flow (S7/S19), not this form.
            let Some(request) = request else {
                return OptInPlan::Keep;
            };
            match stored
                .opt_in(kind)
                .choose(&current.rule, applicable(kind), request, now)
            {
                Ok(Some(state)) => OptInPlan::Set(state, Some((request.choice, current.clone()))),
                Ok(None) => OptInPlan::Keep,
                Err(e) => {
                    refusals.push(opt_in_error(kind, e));
                    OptInPlan::Keep
                }
            }
        })
        .collect()
}
