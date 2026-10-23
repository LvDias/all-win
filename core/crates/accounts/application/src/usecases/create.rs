//! `createAccount` (S5 AC-1…AC-3, AC-5): idempotent by `Idempotency-Key`; first opt-in choices,
//! rule values, initial profiles and audit in one save. Order after the schema: idempotent replay
//! → step-up → 409 → 422; nothing is saved on any error.

use access::{Caller, RequestOrigin, StepUpAction};
use chrono::Duration;

use accounts_enterprise::{
    Account, AccountId, AccountKind, Binding, Choice, LifecycleState, OptInKind, OptInState,
};

use crate::commands::CreateAccount;
use crate::deps::AccountsDeps;
use crate::error::{AccountsError, Refusals};
use crate::events::AccountsEvent;
use crate::ports::{CreateKeyRecord, InitialProfileError, NewAccount, StoreError};
use crate::usecases::audit;
use crate::usecases::support::{
    applicable, current_rules, kind_index, resolve_binding, resolve_selection, store_err, today,
    view,
};
use crate::views::AccountView;

/// A key replays its create for 24 h; after that it is refused, never reused (§2.2).
const REPLAY_WINDOW_HOURS: i64 = 24;

#[derive(Debug)]
pub enum CreateOutcome {
    /// 201.
    Created(AccountView),
    /// 200: the account the first request created, as it is now (with `archived_at` if archived).
    Replayed(AccountView),
}

pub struct CreateAccountUseCase {
    deps: AccountsDeps,
}

impl CreateAccountUseCase {
    pub fn new(deps: AccountsDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        caller: &Caller,
        origin: &RequestOrigin,
        cmd: CreateAccount,
    ) -> Result<CreateOutcome, AccountsError> {
        let d = &self.deps;
        let user = caller.user_id;
        if let Some(record) = d
            .store
            .find_by_create_key(user, cmd.idempotency_key)
            .await
            .map_err(store_err)?
        {
            return self.replay(caller, record, &cmd).await;
        }

        let accepts = [cmd.opt_ins.automation, cmd.opt_ins.copy]
            .iter()
            .flatten()
            .any(|r| r.choice == Choice::Accepted);
        if accepts {
            // Consumed in identity's own commit before the action (ADR-0007); a later refusal
            // leaves it spent.
            d.step_up
                .authorize(
                    caller,
                    StepUpAction::AccountCreateOptInAccept,
                    None,
                    cmd.step_up.clone(),
                    origin,
                )
                .await?;
        }

        let now = d.clock.now();
        let mut refusals = Refusals::default();
        if cmd.kind == AccountKind::Real && cmd.opt_ins.any() {
            // The schema refuses it (400); kept as a guard for other callers.
            return Err(AccountsError::InvalidRequest {
                pointer: "/opt_ins".into(),
            });
        }
        if d.store
            .name_taken(user, cmd.display_name.as_str(), None)
            .await
            .map_err(store_err)?
        {
            refusals.push(AccountsError::NameTaken);
        }
        let resolved = resolve_selection(
            d,
            cmd.kind,
            &cmd.selection,
            cmd.size_amount,
            cmd.max_position_qty,
            None,
            &mut refusals,
        )
        .await?;

        let mut automation = OptInState::NotRequired;
        let mut copy = OptInState::NotRequired;
        let mut chosen_rules = Vec::new();
        if let Some(rules_sel) = resolved.as_ref().and_then(|r| r.rules.clone()) {
            let rules = current_rules(d, &rules_sel).await?;
            for kind in OptInKind::ALL {
                let current = &rules[kind_index(kind)];
                match OptInState::select(
                    &current.rule,
                    applicable(kind),
                    cmd.opt_ins.get(kind),
                    now,
                ) {
                    Ok(state) => {
                        if let OptInState::Chosen(c) = state {
                            chosen_rules.push((kind, c.choice, current.clone()));
                        }
                        match kind {
                            OptInKind::Automation => automation = state,
                            OptInKind::Copy => copy = state,
                        }
                    }
                    Err(e) => refusals.push(crate::usecases::opt_in_error(kind, e)),
                }
            }
        }

        let venue = match &cmd.binding {
            Some(b) => resolve_binding(d, user, b, None, &mut refusals).await?,
            None => None,
        };

        let profiles = match &resolved {
            Some(r) => match d
                .profiles
                .plan(user, cmd.kind, &r.size, cmd.profile_size.as_ref(), now)
                .await
            {
                Ok(p) => p,
                Err(InitialProfileError::ProfileSizeRequired) => {
                    refusals.push(AccountsError::ProfileSizeRequired);
                    Vec::new()
                }
                Err(InitialProfileError::ProfileSizeInvalid) => {
                    refusals.push(AccountsError::ProfileSizeInvalid);
                    Vec::new()
                }
                Err(InitialProfileError::Unavailable(m)) => return Err(AccountsError::Internal(m)),
            },
            None => Vec::new(),
        };

        refusals.first()?;
        let Some(resolved) = resolved else {
            return Err(AccountsError::Internal("unresolved selection".into()));
        };
        if cmd.rule_values.is_some() {
            // Rule values come with a preset (S16); presets have no catalog before S16, so this
            // is unreachable in S5 and must never be silently dropped.
            return Err(AccountsError::Internal("rule_values need S16".into()));
        }

        let evaluation_start = match (cmd.kind, cmd.evaluation_start) {
            (_, Some(date)) => Some(date),
            // Default: the registration date in the user's time zone (S13b, data model §5.2).
            (AccountKind::Prop, None) => Some(today(d, user).await?),
            (AccountKind::Real, None) => None,
        };
        let account = Account {
            id: AccountId::new_v7(),
            user_id: user,
            kind: cmd.kind,
            display_name: cmd.display_name.clone(),
            selection: resolved.selection,
            size_amount: resolved.size,
            multiplier: cmd.multiplier,
            max_position_qty: cmd.max_position_qty,
            lifecycle: LifecycleState::Active,
            binding: venue.map(|venue_account_id| Binding {
                venue_account_id,
                bound_at: now,
            }),
            automation,
            copy,
            commission_per_contract_side: cmd.commission_per_contract_side,
            evaluation_start,
            evaluation_earned_before: cmd.evaluation_earned_before,
            row_version: 1,
            limits_version: 1,
            created_at: now,
            updated_at: now,
            archived_at: None,
        };

        let binding_audit = audit::binding_value(
            cmd.binding.as_ref().map(|b| b.connection_id.as_uuid()),
            cmd.binding.as_ref().map(|b| b.platform_account.as_str()),
        );
        let mut events = vec![AccountsEvent::Created {
            account_id: account.id,
            account_name: account.display_name.clone(),
            at: now,
            fields: audit::created(&account, binding_audit),
        }];
        for (kind, choice, rule) in chosen_rules {
            events.push(AccountsEvent::OptInChosen {
                account_id: account.id,
                account_name: account.display_name.clone(),
                at: now,
                kind,
                choice,
                previous_choice: None,
                rule_version_id: rule.rule.id,
                firm_stance: rule.rule.stance,
                sources: rule.sources,
            });
        }
        events.push(AccountsEvent::LimitsChanged {
            account_id: account.id,
            at: now,
            limits_version: account.limits_version,
        });
        if let Some(b) = account.binding {
            events.push(AccountsEvent::BindingChanged {
                account_id: account.id,
                at: now,
                venue_account_id: Some(b.venue_account_id),
                limits_version: account.limits_version,
            });
        }
        events.push(AccountsEvent::Changed {
            account_id: account.id,
            at: now,
        });

        let new = NewAccount {
            account: account.clone(),
            idempotency_key: cmd.idempotency_key,
            request_hash: cmd.request_hash,
            initial_profiles: profiles,
            events,
        };
        match d.store.create(new).await {
            Ok(()) => Ok(CreateOutcome::Created(view(d, user, account).await?)),
            Err(StoreError::CreateKeyTaken) => {
                // A concurrent request with the same key committed first.
                let record = d
                    .store
                    .find_by_create_key(user, cmd.idempotency_key)
                    .await
                    .map_err(store_err)?
                    .ok_or(AccountsError::IdempotencyKeyReused)?;
                self.replay(caller, record, &cmd).await
            }
            Err(e) => Err(store_err(e)),
        }
    }

    async fn replay(
        &self,
        caller: &Caller,
        record: CreateKeyRecord,
        cmd: &CreateAccount,
    ) -> Result<CreateOutcome, AccountsError> {
        let d = &self.deps;
        let age = d.clock.now() - record.account.created_at;
        if record.request_hash != cmd.request_hash || age >= Duration::hours(REPLAY_WINDOW_HOURS) {
            return Err(AccountsError::IdempotencyKeyReused);
        }
        Ok(CreateOutcome::Replayed(
            view(d, caller.user_id, record.account).await?,
        ))
    }
}
