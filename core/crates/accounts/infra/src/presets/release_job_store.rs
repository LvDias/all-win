//! `ReleaseJobStore`: the cursor and the candidate search run on the worker pool (allwin_worker,
//! column grants only, #11c/#11d); each user's change runs on the app pool as that user (RLS),
//! re-reads the accounts under lock, and writes conditional updates whose events are written only
//! when a row changed. A reprocess changes nothing and writes nothing (idempotent).

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use accounts_application::enterprise::{
    AccountId, AccountName, Choice, OptInKind, OptInState, PresetVersionId, PreviousChoice,
    RuleVersionId,
};
use accounts_application::ports::PortError;
use accounts_application::presets::{
    AccountOptIns, FirmRelease, OptInChange, ReleaseJobEvent, ReleaseJobStore, UserOutcome,
    plan_user,
};
use shared_kernel::UserId;

use crate::store::set_user;

#[derive(Clone)]
pub struct PgReleaseJobStore {
    worker: PgPool,
    app: PgPool,
}

impl PgReleaseJobStore {
    /// `worker`: allwin_worker (finds work); `app`: allwin_app (each user's change, RLS).
    pub fn new(worker: PgPool, app: PgPool) -> Self {
        Self { worker, app }
    }
}

fn err(e: sqlx::Error) -> PortError {
    PortError(e.to_string())
}

/// The stored opt-in as far as the job needs it: only `chosen` is ever changed, so any other
/// state reads as `not_required` here.
fn opt_in(
    state: &str,
    choice: Option<&str>,
    rule: Option<Uuid>,
    chosen_at: Option<DateTime<Utc>>,
) -> OptInState {
    match (state, choice.and_then(Choice::parse), rule, chosen_at) {
        ("chosen", Some(choice), Some(rule), Some(chosen_at)) => {
            OptInState::Chosen(PreviousChoice {
                choice,
                rule_version_id: RuleVersionId::from_uuid(rule),
                chosen_at,
            })
        }
        _ => OptInState::NotRequired,
    }
}

async fn read_accounts(
    conn: &mut PgConnection,
    user_id: UserId,
    release: &FirmRelease,
) -> Result<Vec<AccountOptIns>, PortError> {
    let rules: Vec<Uuid> = release
        .previous_rule_ids()
        .iter()
        .map(RuleVersionId::as_uuid)
        .collect();
    let versions: Vec<Uuid> = release
        .superseded_ids()
        .iter()
        .map(PresetVersionId::as_uuid)
        .collect();
    let rows = sqlx::query!(
        r#"SELECT id, display_name, automation_state, automation_choice, automation_rule_version_id,
                  automation_chosen_at, copy_state, copy_choice, copy_rule_version_id, copy_chosen_at,
                  preset_version_id
           FROM accounts.account
           WHERE user_id = $1 AND archived_at IS NULL
             AND (automation_rule_version_id = ANY($2) OR copy_rule_version_id = ANY($2)
                  OR preset_version_id = ANY($3))
           ORDER BY id
           FOR UPDATE"#,
        user_id.as_uuid(),
        &rules,
        &versions,
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(err)?;
    rows.into_iter()
        .map(|r| {
            Ok(AccountOptIns {
                account_id: AccountId::from_uuid(r.id),
                display_name: AccountName::parse(&r.display_name)
                    .map_err(|_| PortError("account row: invalid name".into()))?,
                automation: opt_in(
                    &r.automation_state,
                    r.automation_choice.as_deref(),
                    r.automation_rule_version_id,
                    r.automation_chosen_at,
                ),
                copy: opt_in(
                    &r.copy_state,
                    r.copy_choice.as_deref(),
                    r.copy_rule_version_id,
                    r.copy_chosen_at,
                ),
                preset_version_id: r.preset_version_id.map(PresetVersionId::from_uuid),
            })
        })
        .collect()
}

/// Applies one change if the stored opt-in is still the one planned; `true` when a row changed.
async fn apply_change(
    conn: &mut PgConnection,
    user_id: UserId,
    account_id: AccountId,
    change: &OptInChange,
    now: DateTime<Utc>,
) -> Result<bool, PortError> {
    let (u, a) = (user_id.as_uuid(), account_id.as_uuid());
    let done = match change {
        OptInChange::Suspend {
            kind: OptInKind::Automation,
            reason,
            previous,
            ..
        } => {
            sqlx::query!(
                r#"UPDATE accounts.account
               SET automation_state = 'pending_review', automation_pending_reason = $4,
                   updated_at = $5, row_version = row_version + 1
               WHERE user_id = $1 AND id = $2 AND archived_at IS NULL
                 AND automation_state = 'chosen' AND automation_rule_version_id = $3"#,
                u,
                a,
                previous.rule_version_id.as_uuid(),
                reason.as_str(),
                now,
            )
            .execute(&mut *conn)
            .await
        }
        OptInChange::Suspend {
            kind: OptInKind::Copy,
            reason,
            previous,
            ..
        } => {
            sqlx::query!(
                r#"UPDATE accounts.account
               SET copy_state = 'pending_review', copy_pending_reason = $4,
                   updated_at = $5, row_version = row_version + 1
               WHERE user_id = $1 AND id = $2 AND archived_at IS NULL
                 AND copy_state = 'chosen' AND copy_rule_version_id = $3"#,
                u,
                a,
                previous.rule_version_id.as_uuid(),
                reason.as_str(),
                now,
            )
            .execute(&mut *conn)
            .await
        }
        OptInChange::Repoint {
            kind: OptInKind::Automation,
            previous,
            rule_version_id,
        } => {
            sqlx::query!(
                r#"UPDATE accounts.account
               SET automation_rule_version_id = $4, updated_at = $5, row_version = row_version + 1
               WHERE user_id = $1 AND id = $2 AND archived_at IS NULL
                 AND automation_state = 'chosen' AND automation_rule_version_id = $3"#,
                u,
                a,
                previous.rule_version_id.as_uuid(),
                rule_version_id.as_uuid(),
                now,
            )
            .execute(&mut *conn)
            .await
        }
        OptInChange::Repoint {
            kind: OptInKind::Copy,
            previous,
            rule_version_id,
        } => {
            sqlx::query!(
                r#"UPDATE accounts.account
               SET copy_rule_version_id = $4, updated_at = $5, row_version = row_version + 1
               WHERE user_id = $1 AND id = $2 AND archived_at IS NULL
                 AND copy_state = 'chosen' AND copy_rule_version_id = $3"#,
                u,
                a,
                previous.rule_version_id.as_uuid(),
                rule_version_id.as_uuid(),
                now,
            )
            .execute(&mut *conn)
            .await
        }
    };
    Ok(done.map_err(err)?.rows_affected() == 1)
}

/// Appends an event; with a fixed id a second write is a no-op. `true` when written now.
async fn outbox(
    conn: &mut PgConnection,
    user_id: UserId,
    event: &ReleaseJobEvent,
) -> Result<bool, PortError> {
    let id = event
        .fixed_id(user_id.as_uuid())
        .unwrap_or_else(Uuid::now_v7);
    let written = sqlx::query!(
        r#"INSERT INTO app.outbox (id, user_id, event_type, occurred_at, payload)
           VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING"#,
        id,
        user_id.as_uuid(),
        event.event_type(),
        event.at(),
        event.payload(),
    )
    .execute(&mut *conn)
    .await
    .map_err(err)?;
    Ok(written.rows_affected() == 1)
}

#[async_trait]
impl ReleaseJobStore for PgReleaseJobStore {
    async fn cursor(&self) -> Result<i64, PortError> {
        sqlx::query_scalar!("SELECT last_publish_seq FROM accounts.catalog_release_cursor")
            .fetch_one(&self.worker)
            .await
            .map_err(err)
    }

    async fn advance(&self, from: i64, to: i64) -> Result<bool, PortError> {
        let moved = sqlx::query!(
            r#"UPDATE accounts.catalog_release_cursor SET last_publish_seq = $2, updated_at = now()
               WHERE last_publish_seq = $1 AND $2 > $1"#,
            from,
            to
        )
        .execute(&self.worker)
        .await
        .map_err(err)?;
        Ok(moved.rows_affected() == 1)
    }

    async fn candidates(
        &self,
        rule_ids: &[RuleVersionId],
        preset_versions: &[PresetVersionId],
    ) -> Result<Vec<UserId>, PortError> {
        let rules: Vec<Uuid> = rule_ids.iter().map(RuleVersionId::as_uuid).collect();
        let versions: Vec<Uuid> = preset_versions
            .iter()
            .map(PresetVersionId::as_uuid)
            .collect();
        let users = sqlx::query_scalar!(
            r#"SELECT DISTINCT user_id FROM accounts.account
               WHERE archived_at IS NULL
                 AND ((automation_state = 'chosen' AND automation_rule_version_id = ANY($1))
                   OR (copy_state = 'chosen' AND copy_rule_version_id = ANY($1))
                   OR preset_version_id = ANY($2))
               ORDER BY user_id"#,
            &rules,
            &versions,
        )
        .fetch_all(&self.worker)
        .await
        .map_err(err)?;
        Ok(users.into_iter().map(UserId::from_uuid).collect())
    }

    async fn apply_for_user(
        &self,
        user_id: UserId,
        release: &FirmRelease,
        now: DateTime<Utc>,
    ) -> Result<UserOutcome, PortError> {
        let mut tx = self.app.begin().await.map_err(err)?;
        set_user(&mut tx, user_id)
            .await
            .map_err(|e| PortError(e.to_string()))?;
        let accounts = read_accounts(&mut tx, user_id, release).await?;
        let plan = plan_user(&accounts, release);
        let mut outcome = UserOutcome::default();
        for (account_id, name, change) in &plan.changes {
            if !apply_change(&mut tx, user_id, *account_id, change, now).await? {
                continue;
            }
            let event = match change {
                OptInChange::Suspend {
                    kind,
                    reason,
                    previous,
                    rule_version_id,
                    rule_version_number,
                } => {
                    outcome.suspended += 1;
                    ReleaseJobEvent::OptInSuspended {
                        at: now,
                        account_id: *account_id,
                        account_name: name.clone(),
                        kind: *kind,
                        cause: *reason,
                        previous_rule_version_id: previous.rule_version_id,
                        rule_version_id: *rule_version_id,
                        rule_version_number: *rule_version_number,
                        firm_name: release.firm_name.clone(),
                    }
                }
                OptInChange::Repoint {
                    kind,
                    previous,
                    rule_version_id,
                } => {
                    outcome.repointed += 1;
                    ReleaseJobEvent::OptInRuleUpdated {
                        at: now,
                        account_id: *account_id,
                        account_name: name.clone(),
                        kind: *kind,
                        previous_rule_version_id: previous.rule_version_id,
                        rule_version_id: *rule_version_id,
                    }
                }
            };
            outbox(&mut tx, user_id, &event).await?;
            outbox(
                &mut tx,
                user_id,
                &ReleaseJobEvent::AccountChanged {
                    at: now,
                    account_id: *account_id,
                },
            )
            .await?;
        }
        if !plan.newer_version.is_empty() {
            let event = ReleaseJobEvent::VersionAvailable {
                at: now,
                release_id: release.release_id,
                firm_id: release.firm_id,
                account_ids: plan.newer_version.clone(),
            };
            outcome.notified = outbox(&mut tx, user_id, &event).await?;
        }
        tx.commit().await.map_err(err)?;
        Ok(outcome)
    }
}
