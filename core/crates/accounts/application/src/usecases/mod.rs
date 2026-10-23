//! Account use cases (S5). Every one is scoped by the caller's `user_id`: another user's account
//! is indistinguishable from a missing one (404).

pub(crate) mod audit;
pub mod create;
pub(crate) mod support;
pub mod update;

use access::{Caller, RequestOrigin, StepUpAction};

use accounts_enterprise::{AccountId, Choice, OptInError, OptInKind};

use crate::commands::{RestartEvaluation, SetLifecycle, SetOptIn};
use crate::deps::AccountsDeps;
use crate::error::{AccountsError, Refusals};
use crate::events::AccountsEvent;
use crate::ports::{AccountWrite, CommissionWrite};
use crate::views::{AccountView, ArchiveImpact, OptInView};
use support::{
    applicable, current_rule, port_err, rule_selection_of, store_err, today, view, views,
};

pub(crate) fn opt_in_error(kind: OptInKind, e: OptInError) -> AccountsError {
    match e {
        OptInError::Required => AccountsError::OptInRequired { kind },
        OptInError::NotApplicable => AccountsError::OptInNotApplicable { kind },
        OptInError::RuleVersionChanged { current } => AccountsError::RuleVersionChanged { current },
    }
}

/// `listAccounts` / `getAccount` / `getAccountArchiveImpact`.
pub struct ReadAccounts {
    deps: AccountsDeps,
}

impl ReadAccounts {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn list(&self, caller: &Caller) -> Result<Vec<AccountView>, AccountsError> {
        let accounts = self
            .deps
            .store
            .list(caller.user_id)
            .await
            .map_err(store_err)?;
        views(&self.deps, caller.user_id, accounts).await
    }

    pub async fn get(&self, caller: &Caller, id: AccountId) -> Result<AccountView, AccountsError> {
        let account = self.live(caller, id).await?;
        view(&self.deps, caller.user_id, account).await
    }

    /// Query port for identity's step-up targets (`AccountTargets::owned_by`): a live account
    /// of this user. Unknown, archived and other users' accounts are all `false`.
    pub async fn exists(
        &self,
        user_id: shared_kernel::UserId,
        id: AccountId,
    ) -> Result<bool, AccountsError> {
        Ok(self
            .deps
            .store
            .get(user_id, id)
            .await
            .map_err(store_err)?
            .is_some())
    }

    pub async fn archive_impact(
        &self,
        caller: &Caller,
        id: AccountId,
    ) -> Result<ArchiveImpact, AccountsError> {
        let account = self.live(caller, id).await?;
        let d = &self.deps;
        let has_open_position = match account.binding {
            Some(b) => d
                .venues
                .has_open_position(caller.user_id, b.venue_account_id)
                .await
                .map_err(port_err)?,
            None => false,
        };
        let routes = d
            .routes
            .routes_of(caller.user_id, id)
            .await
            .map_err(port_err)?;
        Ok(ArchiveImpact {
            has_open_position,
            routes,
        })
    }

    async fn live(
        &self,
        caller: &Caller,
        id: AccountId,
    ) -> Result<accounts_enterprise::Account, AccountsError> {
        self.deps
            .store
            .get(caller.user_id, id)
            .await
            .map_err(store_err)?
            .ok_or(AccountsError::NotFound)
    }
}

/// `setAccountLifecycle` (S5 AC-4).
pub struct SetAccountLifecycle {
    deps: AccountsDeps,
}

impl SetAccountLifecycle {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        caller: &Caller,
        cmd: SetLifecycle,
    ) -> Result<AccountView, AccountsError> {
        let d = &self.deps;
        let user = caller.user_id;
        let stored = d
            .store
            .get(user, cmd.account_id)
            .await
            .map_err(store_err)?
            .ok_or(AccountsError::NotFound)?;
        if cmd.row_version != stored.row_version {
            return Err(AccountsError::VersionConflict);
        }
        let Some(new_state) = stored
            .lifecycle
            .transition(cmd.state)
            .map_err(|_| AccountsError::LifecycleTransitionInvalid)?
        else {
            return view(d, user, stored).await;
        };
        let had_open_position = match stored.binding {
            Some(b) => d
                .venues
                .has_open_position(user, b.venue_account_id)
                .await
                .map_err(port_err)?,
            None => false,
        };
        let now = d.clock.now();
        let mut next = stored.clone();
        next.lifecycle = new_state;
        next.row_version += 1;
        next.updated_at = now;
        let events = vec![
            AccountsEvent::LifecycleChanged {
                account_id: next.id,
                account_name: next.display_name.clone(),
                at: now,
                previous: stored.lifecycle,
                new: new_state,
                had_open_position,
            },
            AccountsEvent::Changed {
                account_id: next.id,
                at: now,
            },
        ];
        save(d, &stored, next.clone(), events).await?;
        view(d, user, next).await
    }
}

/// `setAccountOptIn`: later changes and resolving `pending_review`. Idempotent for the same
/// choice and version (no step-up, not audited).
pub struct SetAccountOptIn {
    deps: AccountsDeps,
}

impl SetAccountOptIn {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        caller: &Caller,
        origin: &RequestOrigin,
        cmd: SetOptIn,
    ) -> Result<OptInView, AccountsError> {
        let d = &self.deps;
        let user = caller.user_id;
        let stored = d
            .store
            .get(user, cmd.account_id)
            .await
            .map_err(store_err)?
            .ok_or(AccountsError::NotFound)?;
        let kind = cmd.kind;
        let state = *stored.opt_in(kind);
        let rule = match rule_selection_of(&stored) {
            Some(sel) => Some(current_rule(d, &sel, kind).await?),
            None => None,
        };
        let current_view = |state| OptInView {
            kind,
            state,
            firm_stance: rule.as_ref().map(|r| r.rule.stance),
            current_rule_version_id: rule.as_ref().map(|r| r.rule.id),
        };
        if state.is_same_choice(&cmd.request) {
            return Ok(current_view(state));
        }
        if cmd.request.choice == Choice::Accepted {
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
        let Some(rule) = rule.clone() else {
            return Err(AccountsError::OptInNotApplicable { kind });
        };
        let now = d.clock.now();
        let mut refusals = Refusals::default();
        let new_state = match state.choose(&rule.rule, applicable(kind), cmd.request, now) {
            Ok(Some(s)) => s,
            Ok(None) => return Ok(current_view(state)),
            Err(e) => {
                refusals.push(opt_in_error(kind, e));
                refusals.first()?;
                return Err(AccountsError::Internal("refusal expected".into()));
            }
        };
        let mut next = stored.clone();
        *next.opt_in_mut(kind) = new_state;
        next.row_version += 1;
        next.updated_at = now;
        let events = vec![
            AccountsEvent::OptInChosen {
                account_id: next.id,
                account_name: next.display_name.clone(),
                at: now,
                kind,
                choice: cmd.request.choice,
                previous_choice: state.recorded().map(|c| c.choice),
                rule_version_id: rule.rule.id,
                firm_stance: rule.rule.stance,
                sources: rule.sources.clone(),
            },
            AccountsEvent::Changed {
                account_id: next.id,
                at: now,
            },
        ];
        save(d, &stored, next, events).await?;
        Ok(current_view(new_state))
    }
}

/// `archiveAccount` (S5 AC-6, Q39): refused while a position is open; otherwise the account
/// leaves every list, releases its binding and frees its name, in one transaction.
pub struct ArchiveAccount {
    deps: AccountsDeps,
}

impl ArchiveAccount {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, caller: &Caller, id: AccountId) -> Result<(), AccountsError> {
        let d = &self.deps;
        let user = caller.user_id;
        let stored = d
            .store
            .get(user, id)
            .await
            .map_err(store_err)?
            .ok_or(AccountsError::NotFound)?;
        if let Some(b) = stored.binding
            && d.venues
                .has_open_position(user, b.venue_account_id)
                .await
                .map_err(port_err)?
        {
            return Err(AccountsError::DeleteOpenPosition);
        }
        // Routing removes the subscriptions on `account.archived` (S7); they are listed here
        // as they are now, so the audit names what was removed.
        let removed_routes = d.routes.routes_of(user, id).await.map_err(port_err)?;
        let now = d.clock.now();
        let mut next = stored.clone();
        next.archived_at = Some(now);
        next.binding = None;
        next.row_version += 1;
        next.updated_at = now;
        let mut events = vec![AccountsEvent::Archived {
            account_id: next.id,
            account_name: next.display_name.clone(),
            at: now,
            removed_routes,
            binding_released: stored.binding.is_some(),
        }];
        if stored.binding.is_some() {
            next.limits_version += 1;
            events.push(AccountsEvent::LimitsChanged {
                account_id: next.id,
                at: now,
                limits_version: next.limits_version,
            });
            events.push(AccountsEvent::BindingChanged {
                account_id: next.id,
                at: now,
                venue_account_id: None,
                limits_version: next.limits_version,
            });
        }
        events.push(AccountsEvent::ArchivedNotice {
            account_id: next.id,
            at: now,
        });
        save(d, &stored, next, events).await
    }
}

/// `restartEvaluation` (S13b; Q69 a): a new evaluation start, and optionally the profit
/// already earned before it. A limits input (ADR-0006).
pub struct RestartAccountEvaluation {
    deps: AccountsDeps,
}

impl RestartAccountEvaluation {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        caller: &Caller,
        cmd: RestartEvaluation,
    ) -> Result<AccountView, AccountsError> {
        let d = &self.deps;
        let user = caller.user_id;
        let stored = d
            .store
            .get(user, cmd.account_id)
            .await
            .map_err(store_err)?
            .ok_or(AccountsError::NotFound)?;
        if cmd.row_version != stored.row_version {
            return Err(AccountsError::VersionConflict);
        }
        if !d
            .store
            .has_evaluation_target(user, stored.id)
            .await
            .map_err(store_err)?
        {
            return Err(AccountsError::EvaluationNotApplicable);
        }
        if cmd.evaluation_start > today(d, user).await? {
            return Err(AccountsError::EvaluationStartInvalid);
        }
        let now = d.clock.now();
        let mut next = stored.clone();
        next.evaluation_start = Some(cmd.evaluation_start);
        next.evaluation_earned_before = cmd.evaluation_earned_before;
        next.row_version += 1;
        next.limits_version += 1;
        next.updated_at = now;
        let events = vec![
            AccountsEvent::EvaluationRestarted {
                account_id: next.id,
                account_name: next.display_name.clone(),
                at: now,
                old_start: stored.evaluation_start,
                new_start: cmd.evaluation_start,
                old_earned_before: stored.evaluation_earned_before,
                new_earned_before: cmd.evaluation_earned_before,
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
        save(d, &stored, next.clone(), events).await?;
        view(d, user, next).await
    }
}

async fn save(
    d: &AccountsDeps,
    stored: &accounts_enterprise::Account,
    next: accounts_enterprise::Account,
    events: Vec<AccountsEvent>,
) -> Result<(), AccountsError> {
    d.store
        .save(AccountWrite {
            account: next,
            expected_row_version: stored.row_version,
            commission: CommissionWrite::Unchanged,
            events,
        })
        .await
        .map_err(store_err)
}
