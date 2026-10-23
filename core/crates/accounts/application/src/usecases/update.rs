//! `updateAccount`: full replacement of the editable fields with optimistic concurrency. Order
//! after the schema: 404 (+ the body must fit the stored kind, 400) → step-up → 409 → 422.

use access::{Caller, RequestOrigin, StepUpAction};

use accounts_enterprise::{
    Account, AccountKind, Binding, Choice, OptInKind, OptInState, PropSelection, RuleOrigin,
};

use crate::commands::{CommissionInput, UpdateAccount};
use crate::deps::AccountsDeps;
use crate::error::{AccountsError, Refusals};
use crate::events::AccountsEvent;
use crate::ports::{AccountWrite, CommissionWrite, CurrentRule, RuleSelection};
use crate::usecases::audit;
use crate::usecases::opt_in_error;
use crate::usecases::support::{
    applicable, bound_platform_account, current_rules, kind_index, local_date, port_err,
    resolve_binding, resolve_selection, rule_selection_of, store_err, view,
};
use crate::views::AccountView;

pub struct UpdateAccountUseCase {
    deps: AccountsDeps,
}

/// How one kind's opt-in moves with this request.
enum OptInPlan {
    Keep,
    Set(OptInState, Option<(Choice, CurrentRule)>),
}

impl UpdateAccountUseCase {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        caller: &Caller,
        origin: &RequestOrigin,
        cmd: UpdateAccount,
    ) -> Result<AccountView, AccountsError> {
        let d = &self.deps;
        let user = caller.user_id;
        let Some(stored) = d.store.get(user, cmd.account_id).await.map_err(store_err)? else {
            return Err(AccountsError::NotFound);
        };
        fits_kind(stored.kind, &cmd)?;

        // Reads needed to decide the step-up (no state changes yet).
        let stored_preset = match &stored.selection {
            Some(PropSelection::Preset {
                preset_version_id, ..
            }) => d
                .presets
                .version(*preset_version_id)
                .await
                .map_err(port_err)?,
            _ => None,
        };
        let mut refusals = Refusals::default();
        let resolved = resolve_selection(
            d,
            stored.kind,
            &cmd.selection,
            cmd.size_amount,
            cmd.max_position_qty,
            stored_preset.as_ref(),
            &mut refusals,
        )
        .await?;
        let stored_rules = match rule_selection_of(&stored) {
            Some(sel) => Some(current_rules(d, &sel).await?),
            None => None,
        };
        let new_rules = match resolved.as_ref().and_then(|r| r.rules.clone()) {
            Some(sel) => Some((current_rules(d, &sel).await?, sel)),
            None => None,
        };
        let selection_changed = resolved
            .as_ref()
            .is_some_and(|r| r.selection != stored.selection);

        let mut binding_refusals = Refusals::default();
        let new_venue = match &cmd.binding {
            Some(b) => resolve_binding(d, user, b, Some(stored.id), &mut binding_refusals).await?,
            None => None,
        };
        let stored_venue = stored.binding.map(|b| b.venue_account_id);
        // An unresolvable binding request is a change too (it would replace the stored one).
        let binding_changes = match (&cmd.binding, stored_venue) {
            (None, None) => false,
            (None, Some(_)) | (Some(_), None) => true,
            (Some(_), Some(old)) => new_venue != Some(old),
        };

        let plans = self.plan_opt_ins(
            &stored,
            &cmd,
            stored_rules.as_ref(),
            new_rules.as_ref(),
            &mut refusals,
        );

        // Step-up: an accepted choice that changes the stored state, or a binding change of an
        // account that receives automation (Q35). One proof covers the request.
        let accepts = plans.iter().any(|p| {
            matches!(p, OptInPlan::Set(OptInState::Chosen(c), _) if c.choice == Choice::Accepted)
        });
        let action = if accepts {
            Some(StepUpAction::AccountOptInAccept)
        } else if binding_changes
            && (stored.has_accepted_automation()
                || d.routes
                    .has_enabled_automated_route(user, stored.id)
                    .await
                    .map_err(port_err)?)
        {
            Some(StepUpAction::AccountBindingChange)
        } else {
            None
        };
        if let Some(action) = action {
            d.step_up
                .authorize(
                    caller,
                    action,
                    Some(stored.id.as_uuid()),
                    cmd.step_up.clone(),
                    origin,
                )
                .await?;
        }

        // 409 / 422.
        if cmd.row_version != stored.row_version {
            refusals.push(AccountsError::VersionConflict);
        }
        if cmd.display_name.as_str().to_lowercase() != stored.display_name.as_str().to_lowercase()
            && d.store
                .name_taken(user, cmd.display_name.as_str(), Some(stored.id))
                .await
                .map_err(store_err)?
        {
            refusals.push(AccountsError::NameTaken);
        }
        if binding_changes {
            if let Some(old) = stored_venue
                && d.venues
                    .has_open_position(user, old)
                    .await
                    .map_err(port_err)?
            {
                refusals.push(AccountsError::BindingOpenPosition);
            }
            if cmd.binding.is_some() {
                for e in binding_refusals.into_inner() {
                    refusals.push(e);
                }
            }
        }
        if cmd.rule_values.is_some() && !selection_changed {
            refusals.push(AccountsError::RuleValuesWithoutSelectionChange);
        }
        refusals.first()?;
        let Some(resolved) = resolved else {
            return Err(AccountsError::Internal("unresolved selection".into()));
        };
        if cmd.rule_values.is_some() {
            // Only with a changed preset selection (S16); unreachable before S16's catalog.
            return Err(AccountsError::Internal("rule_values need S16".into()));
        }

        let now = d.clock.now();
        let mut next = stored.clone();
        next.display_name = cmd.display_name.clone();
        if resolved.has_evaluation_target && next.evaluation_start.is_none() {
            // Invariant: an evaluation target has a start. Gained later, it defaults to the
            // account's registration date in the user's zone (Q46), never the change date.
            next.evaluation_start = Some(local_date(d, caller.user_id, stored.created_at).await?);
        }
        next.selection = resolved.selection;
        next.size_amount = resolved.size;
        next.multiplier = cmd.multiplier;
        next.max_position_qty = cmd.max_position_qty;
        next.commission_per_contract_side = match cmd.commission_per_contract_side {
            CommissionInput::Unchanged => stored.commission_per_contract_side,
            CommissionInput::Set(m) => Some(m),
            CommissionInput::Remove => None,
        };
        if binding_changes {
            next.binding = new_venue.map(|venue_account_id| Binding {
                venue_account_id,
                bound_at: now,
            });
        }
        let mut chosen = Vec::new();
        for (kind, plan) in OptInKind::ALL.into_iter().zip(plans) {
            if let OptInPlan::Set(state, choice) = plan {
                *next.opt_in_mut(kind) = state;
                if let Some((c, rule)) = choice {
                    chosen.push((kind, c, rule));
                }
            }
        }

        let before_binding = bound_platform_account(d, caller.user_id, &stored).await?;
        let after_binding = if binding_changes {
            cmd.binding.as_ref().map(|b| b.platform_account.clone())
        } else {
            before_binding.clone()
        };
        let changes = audit::changes(&stored, before_binding, &next, after_binding);
        let opt_in_changed = next.automation != stored.automation || next.copy != stored.copy;
        if changes.is_empty() && !opt_in_changed {
            // A request that changes nothing is not audited.
            return view(d, user, stored).await;
        }

        let limits = stored.limits_inputs_differ(&next);
        next.row_version = stored.row_version + 1;
        if limits {
            next.limits_version = stored.limits_version + 1;
        }
        next.updated_at = now;

        let mut events = Vec::new();
        if !changes.is_empty() {
            events.push(AccountsEvent::Updated {
                account_id: next.id,
                account_name: next.display_name.clone(),
                at: now,
                changes,
            });
        }
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
        if limits {
            events.push(AccountsEvent::LimitsChanged {
                account_id: next.id,
                at: now,
                limits_version: next.limits_version,
            });
        }
        if binding_changes {
            events.push(AccountsEvent::BindingChanged {
                account_id: next.id,
                at: now,
                venue_account_id: next.binding.map(|b| b.venue_account_id),
                limits_version: next.limits_version,
            });
        }
        events.push(AccountsEvent::Changed {
            account_id: next.id,
            at: now,
        });

        let origin_of_rule = match next.selection {
            Some(PropSelection::Preset { .. }) => RuleOrigin::Edited,
            _ => RuleOrigin::SetByUser,
        };
        let commission = match (
            stored.commission_per_contract_side,
            next.commission_per_contract_side,
        ) {
            (a, b) if a == b => CommissionWrite::Unchanged,
            (_, Some(_)) => CommissionWrite::Set(origin_of_rule),
            (Some(_), None) => CommissionWrite::Unset(origin_of_rule),
            (None, None) => CommissionWrite::Unchanged,
        };
        d.store
            .save(AccountWrite {
                account: next.clone(),
                expected_row_version: stored.row_version,
                commission,
                events,
            })
            .await
            .map_err(store_err)?;
        view(d, user, next).await
    }

    /// Opt-in moves of this request (D16): a kind whose rule version this request changes is
    /// re-selected (its choice is then required if the new version needs one); otherwise a sent
    /// choice is a later change, as `setAccountOptIn`.
    fn plan_opt_ins(
        &self,
        stored: &Account,
        cmd: &UpdateAccount,
        stored_rules: Option<&[CurrentRule; 2]>,
        new_rules: Option<&([CurrentRule; 2], RuleSelection)>,
        refusals: &mut Refusals,
    ) -> Vec<OptInPlan> {
        let now = self.deps.clock.now();
        OptInKind::ALL
            .into_iter()
            .map(|kind| {
                let request = cmd.opt_ins.get(kind);
                let Some((rules, _)) = new_rules else {
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
                    Ok(Some(state)) => {
                        OptInPlan::Set(state, Some((request.choice, current.clone())))
                    }
                    Ok(None) => OptInPlan::Keep,
                    Err(e) => {
                        refusals.push(opt_in_error(kind, e));
                        OptInPlan::Keep
                    }
                }
            })
            .collect()
    }
}

/// `AccountUpdateRequest` keyed on the stored kind: a body that does not fit gets 400 with the
/// pointer of the first offending field.
fn fits_kind(kind: AccountKind, cmd: &UpdateAccount) -> Result<(), AccountsError> {
    let s = &cmd.sent;
    let first = match kind {
        AccountKind::Real => [
            (s.preset_version_id, "/preset_version_id"),
            (s.firm_platform, "/firm_platform"),
            (s.firm_name, "/firm_name"),
            (s.plan_name, "/plan_name"),
            (s.phase, "/phase"),
            (s.rule_values, "/rule_values"),
        ]
        .into_iter()
        .find(|(sent, _)| *sent)
        .map(|(_, p)| p),
        AccountKind::Prop => {
            if !s.preset_version_id && !s.firm_name {
                Some("/firm_name")
            } else {
                None
            }
        }
    };
    match first {
        Some(pointer) => Err(AccountsError::InvalidRequest {
            pointer: pointer.into(),
        }),
        None if kind == AccountKind::Real && !s.size_amount => Err(AccountsError::InvalidRequest {
            pointer: "/size_amount".into(),
        }),
        None => Ok(()),
    }
}
