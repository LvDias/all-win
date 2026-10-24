//! Use-case errors: stable codes (clients translate them), the JSON pointer where the contract
//! defines one, and the check precedence of the S5 contract (§3): 404 → step-up → 409 → 422.

use access::AccessError;
use accounts_enterprise::{OptInKind, RuleField, RuleVersionId};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum AccountsError {
    /// 404 `common.not_found`: unknown, archived, or another user's account (never 403).
    #[error("common.not_found")]
    NotFound,
    /// 400 `common.invalid_request` for a rule the schema layer cannot express.
    #[error("common.invalid_request")]
    InvalidRequest { pointer: String },
    #[error("{0}")]
    Access(AccessError),
    #[error("accounts.idempotency_key_reused")]
    IdempotencyKeyReused,
    #[error("common.version_conflict")]
    VersionConflict,
    #[error("accounts.name_taken")]
    NameTaken,
    #[error("accounts.platform_account_taken")]
    PlatformAccountTaken,
    #[error("accounts.binding_open_position")]
    BindingOpenPosition,
    #[error("accounts.rule_version_changed")]
    RuleVersionChanged { current: RuleVersionId },
    #[error("accounts.delete_open_position")]
    DeleteOpenPosition,
    #[error("accounts.lifecycle_transition_invalid")]
    LifecycleTransitionInvalid,
    #[error("accounts.opt_in_required")]
    OptInRequired { kind: OptInKind },
    #[error("accounts.opt_in_not_applicable")]
    OptInNotApplicable { kind: OptInKind },
    #[error("accounts.max_position_exceeds_preset")]
    MaxPositionExceedsPreset,
    #[error("accounts.preset_version_unknown")]
    PresetVersionUnknown,
    #[error("accounts.firm_platform_required")]
    FirmPlatformRequired,
    #[error("accounts.firm_platform_unknown")]
    FirmPlatformUnknown,
    /// 422 `accounts.firm_platform_unknown` for the `firm_platform` query of
    /// `getAccountPresetDiff` (HTTP 6.2.0, `/query/firm_platform`).
    #[error("accounts.firm_platform_unknown")]
    FirmPlatformUnknownInQuery,
    #[error("accounts.platform_account_unknown")]
    PlatformAccountUnknown,
    #[error("accounts.platform_account_kind_not_allowed")]
    PlatformAccountKindNotAllowed,
    #[error("accounts.profile_size_required")]
    ProfileSizeRequired,
    #[error("accounts.profile_size_invalid")]
    ProfileSizeInvalid,
    #[error("presets.use_apply_version")]
    UseApplyVersion,
    #[error("accounts.rule_values_without_selection_change")]
    RuleValuesWithoutSelectionChange,
    /// `field`: the evaluation field sent on create (`/evaluation_start`,
    /// `/evaluation_earned_before`); `None` on `restartEvaluation`.
    #[error("accounts.evaluation_not_applicable")]
    EvaluationNotApplicable { field: Option<&'static str> },
    #[error("accounts.evaluation_start_invalid")]
    EvaluationStartInvalid,
    /// 409 `presets.version_superseded`: a newer version than the target was published.
    #[error("presets.version_superseded")]
    VersionSuperseded,
    /// 422 `presets.version_not_newer`; `pointer`: `/target_preset_version_id` (apply) or
    /// `/query/target_preset_version_id` (diff).
    #[error("presets.version_not_newer")]
    VersionNotNewer { pointer: &'static str },
    /// 422 `presets.choice_required` (`/choices/<field>`).
    #[error("presets.choice_required")]
    ChoiceRequired { field: RuleField },
    /// 422 `presets.required_field_unresolved` (`/rule_values/<field>`).
    #[error("presets.required_field_unresolved")]
    RequiredFieldUnresolved { field: RuleField },
    /// 422 `presets.value_not_confirmed`.
    #[error("presets.value_not_confirmed")]
    ValueNotConfirmed,
    /// 500 `common.internal`; the detail is logged, never returned.
    #[error("common.internal")]
    Internal(String),
}

impl From<AccessError> for AccountsError {
    fn from(e: AccessError) -> Self {
        Self::Access(e)
    }
}

impl AccountsError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound => "common.not_found",
            Self::InvalidRequest { .. } => "common.invalid_request",
            Self::Access(e) => e.code(),
            Self::IdempotencyKeyReused => "accounts.idempotency_key_reused",
            Self::VersionConflict => "common.version_conflict",
            Self::NameTaken => "accounts.name_taken",
            Self::PlatformAccountTaken => "accounts.platform_account_taken",
            Self::BindingOpenPosition => "accounts.binding_open_position",
            Self::RuleVersionChanged { .. } => "accounts.rule_version_changed",
            Self::DeleteOpenPosition => "accounts.delete_open_position",
            Self::LifecycleTransitionInvalid => "accounts.lifecycle_transition_invalid",
            Self::OptInRequired { .. } => "accounts.opt_in_required",
            Self::OptInNotApplicable { .. } => "accounts.opt_in_not_applicable",
            Self::MaxPositionExceedsPreset => "accounts.max_position_exceeds_preset",
            Self::PresetVersionUnknown => "accounts.preset_version_unknown",
            Self::FirmPlatformRequired => "accounts.firm_platform_required",
            Self::FirmPlatformUnknown | Self::FirmPlatformUnknownInQuery => {
                "accounts.firm_platform_unknown"
            }
            Self::PlatformAccountUnknown => "accounts.platform_account_unknown",
            Self::PlatformAccountKindNotAllowed => "accounts.platform_account_kind_not_allowed",
            Self::ProfileSizeRequired => "accounts.profile_size_required",
            Self::ProfileSizeInvalid => "accounts.profile_size_invalid",
            Self::UseApplyVersion => "presets.use_apply_version",
            Self::RuleValuesWithoutSelectionChange => {
                "accounts.rule_values_without_selection_change"
            }
            Self::EvaluationNotApplicable { .. } => "accounts.evaluation_not_applicable",
            Self::EvaluationStartInvalid => "accounts.evaluation_start_invalid",
            Self::VersionSuperseded => "presets.version_superseded",
            Self::VersionNotNewer { .. } => "presets.version_not_newer",
            Self::ChoiceRequired { .. } => "presets.choice_required",
            Self::RequiredFieldUnresolved { .. } => "presets.required_field_unresolved",
            Self::ValueNotConfirmed => "presets.value_not_confirmed",
            Self::Internal(_) => "common.internal",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::InvalidRequest { .. } => 400,
            Self::Access(e) => e.http_status(),
            Self::IdempotencyKeyReused
            | Self::VersionConflict
            | Self::NameTaken
            | Self::PlatformAccountTaken
            | Self::BindingOpenPosition
            | Self::RuleVersionChanged { .. }
            | Self::DeleteOpenPosition
            | Self::LifecycleTransitionInvalid
            | Self::VersionSuperseded => 409,
            Self::Internal(_) => 500,
            _ => 422,
        }
    }

    /// JSON pointer of the offending field (contract §3).
    pub fn pointer(&self) -> Option<String> {
        let p = match self {
            Self::InvalidRequest { pointer } => return Some(pointer.clone()),
            Self::NameTaken => "/display_name",
            Self::PlatformAccountTaken
            | Self::BindingOpenPosition
            | Self::PlatformAccountUnknown
            | Self::PlatformAccountKindNotAllowed => "/binding",
            Self::OptInRequired { kind } | Self::OptInNotApplicable { kind } => {
                return Some(format!("/opt_ins/{}", kind.as_str()));
            }
            Self::MaxPositionExceedsPreset => "/max_position_qty",
            Self::PresetVersionUnknown | Self::UseApplyVersion => "/preset_version_id",
            Self::FirmPlatformRequired | Self::FirmPlatformUnknown => "/firm_platform",
            Self::ProfileSizeRequired | Self::ProfileSizeInvalid => "/profile_size",
            Self::RuleValuesWithoutSelectionChange => "/rule_values",
            Self::EvaluationStartInvalid => "/evaluation_start",
            Self::EvaluationNotApplicable { field: Some(f) } => f,
            Self::VersionNotNewer { pointer } => pointer,
            Self::FirmPlatformUnknownInQuery => "/query/firm_platform",
            Self::ChoiceRequired { field } => return Some(format!("/choices/{}", field.as_str())),
            Self::RequiredFieldUnresolved { field } => {
                return Some(format!("/rule_values/{}", field.as_str()));
            }
            _ => return None,
        };
        Some(p.to_owned())
    }

    /// Order among the 409/422 answers of one write: 409 before 422, then the order of the
    /// contract's error table (s5-accounts.md §3). The first one is answered.
    fn rank(&self) -> (u16, u8) {
        let index = match self {
            Self::IdempotencyKeyReused => 0,
            Self::VersionConflict => 1,
            Self::NameTaken => 2,
            Self::PlatformAccountTaken => 3,
            Self::BindingOpenPosition => 4,
            Self::RuleVersionChanged { .. } => 5,
            Self::OptInRequired { .. } => 10,
            Self::OptInNotApplicable { .. } => 11,
            Self::MaxPositionExceedsPreset => 12,
            Self::PresetVersionUnknown => 13,
            Self::FirmPlatformRequired => 14,
            Self::FirmPlatformUnknown | Self::FirmPlatformUnknownInQuery => 15,
            Self::PlatformAccountUnknown => 16,
            Self::PlatformAccountKindNotAllowed => 17,
            Self::ProfileSizeRequired => 18,
            Self::ProfileSizeInvalid => 19,
            Self::UseApplyVersion => 20,
            Self::RuleValuesWithoutSelectionChange => 21,
            Self::EvaluationNotApplicable { .. } => 22,
            // S16 apply: the version, then the choices (presets.yaml order).
            Self::VersionSuperseded => 6,
            Self::VersionNotNewer { .. } => 23,
            Self::ChoiceRequired { .. } => 24,
            Self::RequiredFieldUnresolved { .. } => 25,
            Self::ValueNotConfirmed => 26,
            _ => 30,
        };
        (self.http_status(), index)
    }
}

/// Collects the rule refusals of one write and answers the one the contract ranks first.
#[derive(Debug, Default)]
pub struct Refusals(Vec<AccountsError>);

impl Refusals {
    pub fn push(&mut self, e: AccountsError) {
        self.0.push(e);
    }

    pub fn into_inner(self) -> Vec<AccountsError> {
        self.0
    }

    pub fn first(self) -> Result<(), AccountsError> {
        match self.0.into_iter().min_by_key(AccountsError::rank) {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conflicts_win_over_rejections_then_table_order() {
        let mut r = Refusals::default();
        r.push(AccountsError::PlatformAccountUnknown);
        r.push(AccountsError::OptInRequired {
            kind: OptInKind::Automation,
        });
        r.push(AccountsError::PlatformAccountTaken);
        r.push(AccountsError::NameTaken);
        assert_eq!(r.first(), Err(AccountsError::NameTaken));

        let mut r = Refusals::default();
        r.push(AccountsError::PlatformAccountUnknown);
        r.push(AccountsError::OptInRequired {
            kind: OptInKind::Automation,
        });
        let e = r.first().unwrap_err();
        assert_eq!(e.pointer().as_deref(), Some("/opt_ins/automation"));
        assert_eq!(e.http_status(), 422);
        assert!(Refusals::default().first().is_ok());
    }
}
