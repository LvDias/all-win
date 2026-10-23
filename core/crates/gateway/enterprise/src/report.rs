//! Execution-report facts the domain reasons about (protocol `execution_report`).

use serde::{Deserialize, Serialize};

use crate::ids::CommandId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    AllWin,
    LocalEnforcement,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChildRole {
    Entry,
    Stop,
    Target,
    Exit,
    Flatten,
}

impl ChildRole {
    pub const fn is_protective(self) -> bool {
        matches!(self, Self::Stop | Self::Target)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    Received,
    Working,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
    Duplicate,
    Unknown,
}

impl ReportStatus {
    /// The order is still live on the platform after this report.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Received | Self::Working | Self::PartiallyFilled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FillOperation {
    Add,
    Amend,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FillFact {
    pub fill_id: String,
    pub operation: FillOperation,
    pub quantity: u32,
}

/// One accepted report for a single (account, contract), in arrival order (`seq`). Intake is
/// serialized per connection, so `seq` order is arrival order (Esmeril L3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportFact {
    pub seq: i64,
    pub command_id: Option<CommandId>,
    pub origin: Origin,
    pub child_role: Option<ChildRole>,
    pub status: ReportStatus,
    pub reason_code: Option<String>,
    pub fill: Option<FillFact>,
    /// Signed position after this event, when the adapter reports it.
    pub position_after: Option<i64>,
}

impl ReportFact {
    pub fn reason_is(&self, code: &str) -> bool {
        self.reason_code.as_deref() == Some(code)
    }
}
