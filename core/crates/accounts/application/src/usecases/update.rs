//! `updateAccount`: full replacement of the editable fields with optimistic concurrency. Order
//! after the schema: 404 (+ the body must fit the stored kind, 400) → step-up → 409 → 422.

use access::{Caller, RequestOrigin, StepUpAction};

use accounts_enterprise::{
    AccountKind, Binding, Choice, OptInKind, OptInState, PropSelection, RuleOrigin,
};

use crate::commands::{CommissionInput, UpdateAccount};
use crate::deps::AccountsDeps;
use crate::error::{AccountsError, Refusals};
use crate::events::AccountsEvent;
use crate::ports::{AccountWrite, CommissionWrite};
use crate::presets::account_ops::{
    fill_writes, leave_preset_writes, max_position_writes, preset_rules, rule_write_changes,
    writes_target,
};
use crate::usecases::audit;
use crate::usecases::support::{
    OptInPlan, bound_platform_account, current_rules, local_date, plan_opt_ins, port_err,
    resolve_binding, resolve_selection, rule_selection_of, store_err, view,
};
use crate::views::AccountView;

pub struct UpdateAccountUseCase {
    deps: AccountsDeps,
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

        let plans = plan_opt_ins(
            &stored,
            &cmd.opt_ins,
            stored_rules.as_ref(),
            new_rules.as_ref().map(|(rules, _)| rules),
            d.clock.now(),
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
        let now = d.clock.now();
        // S16: the rows follow the selection (only deviations from the preset are stored).
        let stored_is_preset = matches!(stored.selection, Some(PropSelection::Preset { .. }));
        let rows = match resolved.as_ref() {
            Some(r) if r.preset.is_some() || stored_is_preset => d
                .store
                .rule_values(user, stored.id)
                .await
                .map_err(store_err)?,
            _ => Vec::new(),
        };
        let rule_writes = match resolved.as_ref() {
            Some(r) => {
                let max = cmd.max_position_qty.value();
                let writes = match (&r.preset, selection_changed) {
                    (Some(facts), true) => fill_writes(
                        facts,
                        &r.selection,
                        cmd.rule_values.as_ref(),
                        max,
                        &rows,
                        now,
                    ),
                    (Some(facts), false) if cmd.max_position_qty != stored.max_position_qty => {
                        max_position_writes(facts, &r.selection, max, &rows, now)
                    }
                    (None, true) if stored_is_preset => Ok(leave_preset_writes(&rows)),
                    // The schema refuses them without a preset (400); kept as a guard.
                    (None, _) if cmd.rule_values.is_some() => Err(AccountsError::InvalidRequest {
                        pointer: "/rule_values".into(),
                    }),
                    _ => Ok(Vec::new()),
                };
                match writes {
                    Ok(w) => w,
                    Err(e) => {
                        refusals.push(e);
                        Vec::new()
                    }
                }
            }
            None => Vec::new(),
        };
        refusals.first()?;
        let Some(resolved) = resolved else {
            return Err(AccountsError::Internal("unresolved selection".into()));
        };

        let mut next = stored.clone();
        next.display_name = cmd.display_name.clone();
        if (resolved.has_evaluation_target || writes_target(&rule_writes))
            && next.evaluation_start.is_none()
        {
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
        let mut changes = audit::changes(&stored, before_binding, &next, after_binding);
        // The rule fields whose rows this update changes (S16 audit (b) A).
        changes.extend(rule_write_changes(
            (
                preset_rules(stored_preset.as_ref(), &stored.selection),
                &rows,
            ),
            preset_rules(resolved.preset.as_ref(), &next.selection),
            &rule_writes,
        ));
        let opt_in_changed = next.automation != stored.automation || next.copy != stored.copy;
        if changes.is_empty() && !opt_in_changed && rule_writes.is_empty() {
            // A request that changes nothing is not audited.
            return view(d, user, stored).await;
        }

        // Rule values feed the derived limits too (ADR-0006): any changed row bumps them.
        let limits = stored.limits_inputs_differ(&next) || !rule_writes.is_empty();
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
                expected_limits_version: stored.limits_version,
                commission,
                rule_values: rule_writes,
                events,
            })
            .await
            .map_err(store_err)?;
        view(d, user, next).await
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
