//! Use-case inputs, already shaped by the HTTP schema (400s are the infra's job).

use access::StepUpProof;
use chrono::NaiveDate;
use serde_json::Value;
use uuid::Uuid;

use accounts_enterprise::preset_values::account::ApplyChoice;
use accounts_enterprise::{
    AccountId, AccountKind, AccountName, ChoiceRequest, ConnectionId, FirmPlatform, LifecycleState,
    ManualText, MaxPositionQty, MoneyAmount, Multiplier, OptInKind, PresetVersionId, RuleField,
    SizeAmount,
};

/// The prop selection as sent; `None` for a real account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionInput {
    None,
    Preset {
        preset_version_id: PresetVersionId,
        firm_platform: Option<FirmPlatform>,
    },
    Manual {
        firm_name: ManualText,
        plan_name: ManualText,
        phase: ManualText,
    },
}

/// `PlatformAccountRef`: a platform account reported by one of the caller's connections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingRef {
    pub connection_id: ConnectionId,
    pub platform_account: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OptInChoices {
    pub automation: Option<ChoiceRequest>,
    pub copy: Option<ChoiceRequest>,
}

impl OptInChoices {
    pub fn get(&self, kind: OptInKind) -> Option<ChoiceRequest> {
        match kind {
            OptInKind::Automation => self.automation,
            OptInKind::Copy => self.copy,
        }
    }

    pub fn any(&self) -> bool {
        self.automation.is_some() || self.copy.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct CreateAccount {
    pub idempotency_key: Uuid,
    /// SHA-256 of the canonical body without `step_up`.
    pub request_hash: [u8; 32],
    pub kind: AccountKind,
    pub display_name: AccountName,
    pub selection: SelectionInput,
    /// Required for real and manual prop; absent with a preset (the schema checks it).
    pub size_amount: Option<SizeAmount>,
    pub multiplier: Multiplier,
    pub max_position_qty: MaxPositionQty,
    pub commission_per_contract_side: Option<MoneyAmount>,
    pub evaluation_start: Option<NaiveDate>,
    pub evaluation_earned_before: Option<MoneyAmount>,
    pub binding: Option<BindingRef>,
    pub rule_values: Option<Value>,
    pub profile_size: Option<SizeAmount>,
    pub opt_ins: OptInChoices,
    pub step_up: Option<StepUpProof>,
}

/// `commission_per_contract_side` of an update: absent = unchanged, `null` = remove (4.1.0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommissionInput {
    Unchanged,
    Set(MoneyAmount),
    Remove,
}

/// Fields present in an update body, for the "fits the stored kind" check (400 with pointer).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PropFieldsSent {
    pub preset_version_id: bool,
    pub firm_platform: bool,
    pub firm_name: bool,
    pub plan_name: bool,
    pub phase: bool,
    pub size_amount: bool,
    pub rule_values: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateAccount {
    pub account_id: AccountId,
    pub row_version: i32,
    pub display_name: AccountName,
    pub selection: SelectionInput,
    pub size_amount: Option<SizeAmount>,
    pub multiplier: Multiplier,
    pub max_position_qty: MaxPositionQty,
    pub commission_per_contract_side: CommissionInput,
    pub binding: Option<BindingRef>,
    pub rule_values: Option<Value>,
    pub opt_ins: OptInChoices,
    pub opt_ins_sent: bool,
    pub sent: PropFieldsSent,
    pub step_up: Option<StepUpProof>,
}

#[derive(Debug, Clone)]
pub struct SetOptIn {
    pub account_id: AccountId,
    pub kind: OptInKind,
    pub request: ChoiceRequest,
    pub step_up: Option<StepUpProof>,
}

#[derive(Debug, Clone, Copy)]
pub struct SetLifecycle {
    pub account_id: AccountId,
    pub state: LifecycleState,
    pub row_version: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct RestartEvaluation {
    pub account_id: AccountId,
    pub evaluation_start: NaiveDate,
    pub evaluation_earned_before: Option<MoneyAmount>,
    pub row_version: i32,
}

/// `applyPresetVersion` (S16 AC-3, AC-11).
#[derive(Debug, Clone)]
pub struct ApplyPresetVersion {
    pub account_id: AccountId,
    pub target_preset_version_id: PresetVersionId,
    pub row_version: i32,
    pub firm_platform: Option<FirmPlatform>,
    pub choices: Vec<(RuleField, ApplyChoice)>,
    pub rule_values: Option<Value>,
    pub max_position_qty: Option<MaxPositionQty>,
    pub opt_ins: OptInChoices,
    pub step_up: Option<StepUpProof>,
}

/// `restorePresetValue` (S16-04).
#[derive(Debug, Clone, Copy)]
pub struct RestorePresetValue {
    pub account_id: AccountId,
    pub field: RuleField,
    pub row_version: i32,
}
