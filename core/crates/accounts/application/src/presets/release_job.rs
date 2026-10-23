//! The accounts job over published firm releases (S16 §2.11, Esquadro (B)): every 30 s and at
//! start, it reads the releases after its cursor, in `publish_seq` order. For each one:
//! - the worker FINDS the users with a candidate account (a chosen opt-in on a replaced rule text,
//!   or the release's previous preset version);
//! - each user's change runs in that user's transaction as `allwin_app`, which re-reads the
//!   accounts and recomputes the effect (the worker's read is only a candidate);
//! - the cursor advances only after every user of the release is done; a failure stops the run
//!   and the next run retries (every write is idempotent).
//!
//! Effects per account: a material text change (`stance_changed`, `text_changed`) moves the
//! chosen opt-in to `pending_review` (audit `accounts.opt_in_suspended`); a `not_material` one
//! re-points it to the new text (audit `accounts.opt_in_rule_updated`), and until that audit code
//! is in the contract such a release waits (fail closed: the routing gate keeps blocking). Accounts
//! on a superseded preset version get `presets.version_available` (never applied silently).

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use accounts_enterprise::{
    AccountId, AccountName, FirmId, OptInKind, OptInState, PendingReason, PresetVersionId,
    PreviousChoice, RuleVersionId,
};
use shared_kernel::{Clock, UserId};

use crate::ports::PortError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplaceCause {
    StanceChanged,
    TextChanged,
    NotMaterial,
}

impl ReplaceCause {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "stance_changed" => Some(Self::StanceChanged),
            "text_changed" => Some(Self::TextChanged),
            "not_material" => Some(Self::NotMaterial),
            _ => None,
        }
    }
}

/// An opt-in rule text the release replaced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplacedText {
    pub kind: OptInKind,
    pub previous: RuleVersionId,
    pub new: RuleVersionId,
    pub new_version_number: i32,
    pub cause: ReplaceCause,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmRelease {
    pub publish_seq: i64,
    pub release_id: Uuid,
    pub firm_id: FirmId,
    pub firm_name: String,
    pub replaced: Vec<ReplacedText>,
    /// `(previous preset version, its version in this release)`.
    pub superseded: Vec<(PresetVersionId, PresetVersionId)>,
}

impl FirmRelease {
    pub fn previous_rule_ids(&self) -> Vec<RuleVersionId> {
        self.replaced.iter().map(|t| t.previous).collect()
    }

    pub fn superseded_ids(&self) -> Vec<PresetVersionId> {
        self.superseded.iter().map(|(old, _)| *old).collect()
    }
}

/// catalog: releases after a `publish_seq`, ascending (adapter in `crates/app`).
#[async_trait]
pub trait ReleaseFeed: Send + Sync {
    async fn published_after(&self, after: i64, limit: i64) -> Result<Vec<FirmRelease>, PortError>;
}

/// An account as the user's transaction re-reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountOptIns {
    pub account_id: AccountId,
    pub display_name: AccountName,
    pub automation: OptInState,
    pub copy: OptInState,
    pub preset_version_id: Option<PresetVersionId>,
}

impl AccountOptIns {
    fn state(&self, kind: OptInKind) -> &OptInState {
        match kind {
            OptInKind::Automation => &self.automation,
            OptInKind::Copy => &self.copy,
        }
    }
}

/// One automatic change to an account's opt-in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptInChange {
    /// `chosen` → `pending_review` (Q26/Q42); the previous choice is kept and shown.
    Suspend {
        kind: OptInKind,
        reason: PendingReason,
        previous: PreviousChoice,
        rule_version_id: RuleVersionId,
        rule_version_number: i32,
    },
    /// Same choice and `chosen_at`, new rule text (`not_material`).
    Repoint {
        kind: OptInKind,
        previous: PreviousChoice,
        rule_version_id: RuleVersionId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserPlan {
    pub changes: Vec<(AccountId, AccountName, OptInChange)>,
    /// Accounts on a superseded preset version (one `presets.version_available` per user and firm).
    pub newer_version: Vec<AccountId>,
}

/// The effect of `release` on one user's accounts, from their current state.
pub fn plan_user(accounts: &[AccountOptIns], release: &FirmRelease) -> UserPlan {
    let mut plan = UserPlan::default();
    let superseded = release.superseded_ids();
    for a in accounts {
        for t in &release.replaced {
            let OptInState::Chosen(previous) = a.state(t.kind) else {
                continue;
            };
            if previous.rule_version_id != t.previous {
                continue;
            }
            let change = match t.cause {
                ReplaceCause::StanceChanged | ReplaceCause::TextChanged => OptInChange::Suspend {
                    kind: t.kind,
                    reason: if t.cause == ReplaceCause::StanceChanged {
                        PendingReason::StanceChanged
                    } else {
                        PendingReason::TextChanged
                    },
                    previous: *previous,
                    rule_version_id: t.new,
                    rule_version_number: t.new_version_number,
                },
                ReplaceCause::NotMaterial => OptInChange::Repoint {
                    kind: t.kind,
                    previous: *previous,
                    rule_version_id: t.new,
                },
            };
            plan.changes
                .push((a.account_id, a.display_name.clone(), change));
        }
        if a.preset_version_id
            .is_some_and(|pv| superseded.contains(&pv))
        {
            plan.newer_version.push(a.account_id);
        }
    }
    plan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UserOutcome {
    pub suspended: usize,
    pub repointed: usize,
    /// The `presets.version_available` event was written now (not already there).
    pub notified: bool,
}

/// The job's own rows: the cursor (worker) and each user's transaction (app, RLS).
#[async_trait]
pub trait ReleaseJobStore: Send + Sync {
    async fn cursor(&self) -> Result<i64, PortError>;
    /// Compare-and-set; `false` when another run moved it.
    async fn advance(&self, from: i64, to: i64) -> Result<bool, PortError>;
    /// Users with a live account that may be affected (worker; candidates only).
    async fn candidates(
        &self,
        rule_ids: &[RuleVersionId],
        preset_versions: &[PresetVersionId],
    ) -> Result<Vec<UserId>, PortError>;
    /// In the user's transaction: re-reads the accounts, plans with [`plan_user`], writes the
    /// conditional updates and their events.
    async fn apply_for_user(
        &self,
        user_id: UserId,
        release: &FirmRelease,
        now: DateTime<Utc>,
    ) -> Result<UserOutcome, PortError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RunReport {
    pub releases_done: usize,
    pub users: usize,
    pub suspended: usize,
    pub notified: usize,
    /// A release waits: it re-points opt-ins and `accounts.opt_in_rule_updated` is not in the
    /// contract yet (Esquadro); the cursor stays before it.
    pub waiting_at: Option<i64>,
}

/// Whether the `not_material` path may run (its audit code exists in the contract).
pub const REPOINT_ENABLED: bool = false;

pub struct ProcessPublishedReleases {
    feed: Arc<dyn ReleaseFeed>,
    store: Arc<dyn ReleaseJobStore>,
    clock: Arc<dyn Clock>,
}

/// Releases read per run.
const BATCH: i64 = 20;

impl ProcessPublishedReleases {
    pub fn new(
        feed: Arc<dyn ReleaseFeed>,
        store: Arc<dyn ReleaseJobStore>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self { feed, store, clock }
    }

    pub async fn handle(&self) -> Result<RunReport, PortError> {
        let mut report = RunReport::default();
        let mut cursor = self.store.cursor().await?;
        for release in self.feed.published_after(cursor, BATCH).await? {
            if !REPOINT_ENABLED
                && release
                    .replaced
                    .iter()
                    .any(|t| t.cause == ReplaceCause::NotMaterial)
            {
                report.waiting_at = Some(release.publish_seq);
                break;
            }
            let users = self
                .store
                .candidates(&release.previous_rule_ids(), &release.superseded_ids())
                .await?;
            let now = self.clock.now();
            for user in &users {
                let out = self.store.apply_for_user(*user, &release, now).await?;
                report.suspended += out.suspended;
                report.notified += usize::from(out.notified);
            }
            report.users += users.len();
            if !self.store.advance(cursor, release.publish_seq).await? {
                // Another run advanced it: stop; the next run starts from the stored cursor.
                break;
            }
            cursor = release.publish_seq;
            report.releases_done += 1;
        }
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use accounts_enterprise::Choice;

    fn t0() -> DateTime<Utc> {
        Utc.timestamp_opt(1_800_000_000, 0).unwrap()
    }

    fn rv(n: u128) -> RuleVersionId {
        RuleVersionId::from_uuid(Uuid::from_u128(n))
    }

    fn chosen(id: RuleVersionId, choice: Choice) -> OptInState {
        OptInState::Chosen(PreviousChoice {
            choice,
            rule_version_id: id,
            chosen_at: t0(),
        })
    }

    fn account(automation: OptInState, copy: OptInState, pv: Option<u128>) -> AccountOptIns {
        AccountOptIns {
            account_id: AccountId::new_v7(),
            display_name: AccountName::parse("Conta").unwrap(),
            automation,
            copy,
            preset_version_id: pv.map(|n| PresetVersionId::from_uuid(Uuid::from_u128(n))),
        }
    }

    fn release(cause: ReplaceCause) -> FirmRelease {
        FirmRelease {
            publish_seq: 7,
            release_id: Uuid::nil(),
            firm_id: FirmId::from_uuid(Uuid::nil()),
            firm_name: "Firm".into(),
            replaced: vec![ReplacedText {
                kind: OptInKind::Automation,
                previous: rv(1),
                new: rv(2),
                new_version_number: 2,
                cause,
            }],
            superseded: vec![(
                PresetVersionId::from_uuid(Uuid::from_u128(10)),
                PresetVersionId::from_uuid(Uuid::from_u128(11)),
            )],
        }
    }

    #[test]
    fn material_changes_suspend_only_matching_chosen_opt_ins() {
        let hit = account(
            chosen(rv(1), Choice::Accepted),
            OptInState::NotRequired,
            None,
        );
        let declined = account(
            chosen(rv(1), Choice::Declined),
            OptInState::NotRequired,
            None,
        );
        let other_text = account(
            chosen(rv(9), Choice::Accepted),
            OptInState::NotRequired,
            None,
        );
        let pending = account(
            OptInState::PendingReview {
                reason: PendingReason::TextChanged,
                previous: None,
            },
            OptInState::NotRequired,
            None,
        );
        // The copy slot is not the replaced kind even with the same id.
        let copy_only = account(
            OptInState::NotRequired,
            chosen(rv(1), Choice::Accepted),
            None,
        );
        let plan = plan_user(
            &[
                hit.clone(),
                declined.clone(),
                other_text,
                pending,
                copy_only,
            ],
            &release(ReplaceCause::TextChanged),
        );
        let ids: Vec<AccountId> = plan.changes.iter().map(|(id, _, _)| *id).collect();
        assert_eq!(ids, [hit.account_id, declined.account_id]);
        assert!(matches!(
            plan.changes[0].2,
            OptInChange::Suspend {
                reason: PendingReason::TextChanged,
                rule_version_number: 2,
                ..
            }
        ));
    }

    #[test]
    fn not_material_repoints_and_superseded_presets_are_announced() {
        let a = account(
            chosen(rv(1), Choice::Accepted),
            OptInState::NotRequired,
            Some(10),
        );
        let b = account(OptInState::NotRequired, OptInState::NotRequired, Some(11));
        let plan = plan_user(&[a.clone(), b], &release(ReplaceCause::NotMaterial));
        assert!(
            matches!(plan.changes[0].2, OptInChange::Repoint { rule_version_id, .. } if rule_version_id == rv(2))
        );
        assert_eq!(plan.newer_version, [a.account_id]);
    }
}
