//! What the read operations return: the stored account plus the derived parts of `Account`
//! (opt-in stance and current rule version, preset display, binding, open position).

use accounts_enterprise::{
    Account, FirmId, FirmPlatform, FirmStance, OptInKind, OptInState, PresetVersionId,
    RuleVersionId,
};

use crate::ports::{BindingView, RouteRef};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptInView {
    pub kind: OptInKind,
    pub state: OptInState,
    /// `None` only for real accounts.
    pub firm_stance: Option<FirmStance>,
    pub current_rule_version_id: Option<RuleVersionId>,
}

/// Firm, plan and phase as shown: from the preset version, or the manual text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropView {
    pub firm_name: String,
    pub plan_name: String,
    pub phase: String,
    pub firm_id: Option<FirmId>,
    pub preset_version_id: Option<PresetVersionId>,
    pub firm_platform: Option<FirmPlatform>,
    pub preset_update_available: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountView {
    pub account: Account,
    pub prop: Option<PropView>,
    pub binding: Option<BindingView>,
    pub automation: OptInView,
    pub copy: OptInView,
    pub has_open_position: bool,
}

impl AccountView {
    pub fn opt_in(&self, kind: OptInKind) -> &OptInView {
        match kind {
            OptInKind::Automation => &self.automation,
            OptInKind::Copy => &self.copy,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveImpact {
    pub has_open_position: bool,
    pub routes: Vec<RouteRef>,
}
