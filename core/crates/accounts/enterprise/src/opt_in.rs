//! Per-account opt-ins (decision 2026-10-23, option A; Q24–Q26, Q40, Q42). A prop account chooses,
//! per kind, whether it accepts automated sources (`automation`) or master replication (`copy`),
//! against the rule version whose text the user saw. There is no "saved without a choice" state:
//! a required first choice is atomic with the save that makes it required.

use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::uuid;

use crate::ids::RuleVersionId;

/// Generic rule versions (Q40), seeded with these fixed ids by migration 20261023060900.
pub const GENERIC_AUTOMATION_RULE_VERSION_ID: RuleVersionId =
    RuleVersionId::from_uuid(uuid!("019a1000-0000-7000-8000-000000000001"));
pub const GENERIC_COPY_RULE_VERSION_ID: RuleVersionId =
    RuleVersionId::from_uuid(uuid!("019a1000-0000-7000-8000-000000000002"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptInKind {
    Automation,
    Copy,
}

impl OptInKind {
    pub const ALL: [Self; 2] = [Self::Automation, Self::Copy];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Automation => "automation",
            Self::Copy => "copy",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.as_str() == raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirmStance {
    Allowed,
    Forbidden,
    Conditional,
    Unknown,
    NotConfirmed,
}

impl FirmStance {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Forbidden => "forbidden",
            Self::Conditional => "conditional",
            Self::Unknown => "unknown",
            Self::NotConfirmed => "not_confirmed",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "allowed" => Self::Allowed,
            "forbidden" => Self::Forbidden,
            "conditional" => Self::Conditional,
            "unknown" => Self::Unknown,
            "not_confirmed" => Self::NotConfirmed,
            _ => return None,
        })
    }

    /// Q25: a choice is required unless the firm allows that kind.
    pub const fn requires_choice(self) -> bool {
        !matches!(self, Self::Allowed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Accepted,
    Declined,
}

impl Choice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Declined => "declined",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "accepted" => Some(Self::Accepted),
            "declined" => Some(Self::Declined),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingReason {
    StanceChanged,
    TextChanged,
    /// Reserved for a data-migration attach (Q41); no operation produces it.
    PresetAttached,
}

impl PendingReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StanceChanged => "stance_changed",
            Self::TextChanged => "text_changed",
            Self::PresetAttached => "preset_attached",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "stance_changed" => Some(Self::StanceChanged),
            "text_changed" => Some(Self::TextChanged),
            "preset_attached" => Some(Self::PresetAttached),
            _ => None,
        }
    }
}

/// The rule version in force for an account's selection and one kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleVersionRef {
    pub id: RuleVersionId,
    pub kind: OptInKind,
    pub stance: FirmStance,
}

/// A recorded choice and the rule version it was made against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreviousChoice {
    pub choice: Choice,
    pub rule_version_id: RuleVersionId,
    pub chosen_at: DateTime<Utc>,
}

/// Stored state of one kind (`OptInState` without the derived stance and current version).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptInState {
    /// No choice is asked now: real account, the stance allows the kind, or `copy` of an account
    /// that is not a follower (Q-S5-12).
    NotRequired,
    /// An explicit choice for the current rule version.
    Chosen(PreviousChoice),
    /// Suspended until the user chooses again (Q26/Q42); `previous` is shown, never preselected.
    PendingReview {
        reason: PendingReason,
        previous: Option<PreviousChoice>,
    },
}

/// A choice sent by the client, naming the rule version whose text it displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChoiceRequest {
    pub choice: Choice,
    pub rule_version_id: RuleVersionId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum OptInError {
    /// 422 `accounts.opt_in_required`.
    #[error("accounts.opt_in_required")]
    Required,
    /// 422 `accounts.opt_in_not_applicable`.
    #[error("accounts.opt_in_not_applicable")]
    NotApplicable,
    /// 409 `accounts.rule_version_changed` with the current version.
    #[error("accounts.rule_version_changed")]
    RuleVersionChanged { current: RuleVersionId },
}

impl OptInState {
    /// The kind is accepted by routing: `not_required`, or `chosen` + `accepted`.
    pub fn is_accepted(&self) -> bool {
        match self {
            Self::NotRequired => true,
            Self::Chosen(c) => c.choice == Choice::Accepted,
            Self::PendingReview { .. } => false,
        }
    }

    /// State after a save that (re)selects `current` for this kind: a create, or an update that
    /// changes the rule version. `applicable` is false when the kind is never asked for this
    /// account (`copy` of a non-follower). A required choice must come with the request.
    pub fn select(
        current: &RuleVersionRef,
        applicable: bool,
        request: Option<ChoiceRequest>,
        now: DateTime<Utc>,
    ) -> Result<Self, OptInError> {
        if !applicable || !current.stance.requires_choice() {
            return match request {
                Some(_) => Err(OptInError::NotApplicable),
                None => Ok(Self::NotRequired),
            };
        }
        let Some(r) = request else {
            return Err(OptInError::Required);
        };
        check_version(current, &r)?;
        Ok(Self::Chosen(PreviousChoice {
            choice: r.choice,
            rule_version_id: r.rule_version_id,
            chosen_at: now,
        }))
    }

    /// Records a later choice (`setAccountOptIn`, or `opt_ins` on an update that keeps the rule
    /// version). `Ok(None)`: the same choice for the same version is already in force (no-op).
    pub fn choose(
        &self,
        current: &RuleVersionRef,
        applicable: bool,
        request: ChoiceRequest,
        now: DateTime<Utc>,
    ) -> Result<Option<Self>, OptInError> {
        if self.is_same_choice(&request) {
            return Ok(None);
        }
        if !applicable || !current.stance.requires_choice() {
            return Err(OptInError::NotApplicable);
        }
        check_version(current, &request)?;
        Ok(Some(Self::Chosen(PreviousChoice {
            choice: request.choice,
            rule_version_id: request.rule_version_id,
            chosen_at: now,
        })))
    }

    /// Whether `request` repeats the stored `chosen` state exactly (idempotent retry).
    pub fn is_same_choice(&self, request: &ChoiceRequest) -> bool {
        matches!(self, Self::Chosen(c)
            if c.choice == request.choice && c.rule_version_id == request.rule_version_id)
    }

    /// The choice in force (`chosen`) or the previous one (`pending_review`).
    pub fn recorded(&self) -> Option<&PreviousChoice> {
        match self {
            Self::NotRequired => None,
            Self::Chosen(c) => Some(c),
            Self::PendingReview { previous, .. } => previous.as_ref(),
        }
    }

    pub const fn state_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Chosen(_) => "chosen",
            Self::PendingReview { .. } => "pending_review",
        }
    }
}

fn check_version(current: &RuleVersionRef, r: &ChoiceRequest) -> Result<(), OptInError> {
    if r.rule_version_id == current.id {
        Ok(())
    } else {
        Err(OptInError::RuleVersionChanged {
            current: current.id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn rv(stance: FirmStance) -> RuleVersionRef {
        RuleVersionRef {
            id: GENERIC_AUTOMATION_RULE_VERSION_ID,
            kind: OptInKind::Automation,
            stance,
        }
    }

    fn req(choice: Choice, id: RuleVersionId) -> ChoiceRequest {
        ChoiceRequest {
            choice,
            rule_version_id: id,
        }
    }

    #[test]
    fn unknown_stance_requires_a_choice_on_the_current_version() {
        let now = Utc::now();
        let current = rv(FirmStance::Unknown);
        assert_eq!(
            OptInState::select(&current, true, None, now),
            Err(OptInError::Required)
        );
        let stale = RuleVersionId::from_uuid(Uuid::now_v7());
        assert_eq!(
            OptInState::select(&current, true, Some(req(Choice::Declined, stale)), now),
            Err(OptInError::RuleVersionChanged {
                current: current.id
            })
        );
        let s = OptInState::select(&current, true, Some(req(Choice::Declined, current.id)), now)
            .unwrap();
        assert!(!s.is_accepted());
        assert_eq!(s.state_str(), "chosen");
    }

    #[test]
    fn allowed_or_not_applicable_take_no_choice() {
        let now = Utc::now();
        let allowed = rv(FirmStance::Allowed);
        assert_eq!(
            OptInState::select(&allowed, true, None, now),
            Ok(OptInState::NotRequired)
        );
        assert_eq!(
            OptInState::select(&allowed, true, Some(req(Choice::Accepted, allowed.id)), now),
            Err(OptInError::NotApplicable)
        );
        let unknown = rv(FirmStance::Unknown);
        assert_eq!(
            OptInState::select(&unknown, false, None, now),
            Ok(OptInState::NotRequired)
        );
        assert!(OptInState::NotRequired.is_accepted());
    }

    #[test]
    fn choose_is_idempotent_and_clears_pending_review() {
        let now = Utc::now();
        let current = rv(FirmStance::Forbidden);
        let chosen =
            OptInState::select(&current, true, Some(req(Choice::Accepted, current.id)), now)
                .unwrap();
        assert_eq!(
            chosen.choose(&current, true, req(Choice::Accepted, current.id), now),
            Ok(None)
        );
        let declined = chosen
            .choose(&current, true, req(Choice::Declined, current.id), now)
            .unwrap()
            .unwrap();
        assert!(!declined.is_accepted());

        let pending = OptInState::PendingReview {
            reason: PendingReason::StanceChanged,
            previous: chosen.recorded().copied(),
        };
        assert!(!pending.is_accepted());
        assert_eq!(pending.recorded().map(|c| c.choice), Some(Choice::Accepted));
        let again = pending
            .choose(&current, true, req(Choice::Accepted, current.id), now)
            .unwrap()
            .unwrap();
        assert!(again.is_accepted());
    }

    #[test]
    fn choose_on_not_applicable_kind_is_refused() {
        let now = Utc::now();
        let current = rv(FirmStance::Unknown);
        assert_eq!(
            OptInState::NotRequired.choose(&current, false, req(Choice::Declined, current.id), now),
            Err(OptInError::NotApplicable)
        );
    }

    #[test]
    fn codes_round_trip() {
        for k in OptInKind::ALL {
            assert_eq!(OptInKind::parse(k.as_str()), Some(k));
        }
        for s in [
            "allowed",
            "forbidden",
            "conditional",
            "unknown",
            "not_confirmed",
        ] {
            assert_eq!(FirmStance::parse(s).map(FirmStance::as_str), Some(s));
        }
        assert!(!FirmStance::Allowed.requires_choice());
        assert!(FirmStance::NotConfirmed.requires_choice());
    }
}
