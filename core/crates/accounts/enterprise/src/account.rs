//! The trading account aggregate (wave1-data-model §5.2).

use chrono::{DateTime, NaiveDate, Utc};
use shared_kernel::UserId;

use crate::ids::{AccountId, FirmId, PresetVersionId, VenueAccountId};
use crate::lifecycle::LifecycleState;
use crate::opt_in::{Choice, OptInKind, OptInState};
use crate::values::{
    AccountName, FirmPlatform, ManualText, MaxPositionQty, MoneyAmount, Multiplier, SizeAmount,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountKind {
    Prop,
    Real,
}

impl AccountKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prop => "prop",
            Self::Real => "real",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "prop" => Some(Self::Prop),
            "real" => Some(Self::Real),
            _ => None,
        }
    }
}

/// How a prop account names its firm, plan and phase (S5 contract §2.4): a preset version (S16)
/// or the manual fields, never both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropSelection {
    Preset {
        preset_version_id: PresetVersionId,
        firm_id: FirmId,
        firm_platform: Option<FirmPlatform>,
    },
    Manual {
        firm_name: ManualText,
        plan_name: ManualText,
        phase: ManualText,
    },
}

/// The bound platform account (ADR-0008): gateway's opaque id; accounts never reads platform data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Binding {
    pub venue_account_id: VenueAccountId,
    pub bound_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub user_id: UserId,
    pub kind: AccountKind,
    pub display_name: AccountName,
    /// `Some` exactly for prop accounts.
    pub selection: Option<PropSelection>,
    pub size_amount: SizeAmount,
    pub multiplier: Multiplier,
    pub max_position_qty: MaxPositionQty,
    pub lifecycle: LifecycleState,
    pub binding: Option<Binding>,
    pub automation: OptInState,
    pub copy: OptInState,
    /// The account's commission floor (rule value `commission_per_contract_side`); `None` = none.
    pub commission_per_contract_side: Option<MoneyAmount>,
    pub evaluation_start: Option<NaiveDate>,
    pub evaluation_earned_before: Option<MoneyAmount>,
    pub row_version: i32,
    pub limits_version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
}

impl Account {
    pub fn opt_in(&self, kind: OptInKind) -> &OptInState {
        match kind {
            OptInKind::Automation => &self.automation,
            OptInKind::Copy => &self.copy,
        }
    }

    pub fn opt_in_mut(&mut self, kind: OptInKind) -> &mut OptInState {
        match kind {
            OptInKind::Automation => &mut self.automation,
            OptInKind::Copy => &mut self.copy,
        }
    }

    /// The account receives automation through its own choice (automation `chosen` +
    /// `accepted`). An enabled route from an automated source (S7) is the other case; routing
    /// answers that part.
    pub fn has_accepted_automation(&self) -> bool {
        matches!(self.automation, OptInState::Chosen(c) if c.choice == Choice::Accepted)
    }

    /// The inputs of the derived risk limits differ (ADR-0006 §3.1): selection, size, maximum
    /// position, commission floor, evaluation inputs and binding.
    pub fn limits_inputs_differ(&self, other: &Self) -> bool {
        self.selection != other.selection
            || self.size_amount != other.size_amount
            || self.max_position_qty != other.max_position_qty
            || self.commission_per_contract_side != other.commission_per_contract_side
            || self.evaluation_start != other.evaluation_start
            || self.evaluation_earned_before != other.evaluation_earned_before
            || self.binding.map(|b| b.venue_account_id) != other.binding.map(|b| b.venue_account_id)
    }
}
