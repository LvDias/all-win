//! The account's own rule values (`accounts.account_rule_value`, data model §5.7). Only deviations
//! from the preset are stored; without a preset every value the user sets is `set_by_user`.
//! Every write is a limits input (ADR-0006 §3.1): it bumps `limits_version`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleField {
    MaxPositionQty,
    Drawdown,
    DailyLossLimit,
    DailyProfitTarget,
    TradingDayReset,
    ConsistencyRule,
    ProfitTargetOnReach,
    ConsistencyThresholdPct,
    /// U20 (a): the commission floor per contract and side, `{"amount": "0.37"}`.
    CommissionPerContractSide,
    /// Q46: the evaluation profit target, `{"amount": "3000.00"}`.
    EvaluationProfitTarget,
}

impl RuleField {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaxPositionQty => "max_position_qty",
            Self::Drawdown => "drawdown",
            Self::DailyLossLimit => "daily_loss_limit",
            Self::DailyProfitTarget => "daily_profit_target",
            Self::TradingDayReset => "trading_day_reset",
            Self::ConsistencyRule => "consistency_rule",
            Self::ProfitTargetOnReach => "profit_target_on_reach",
            Self::ConsistencyThresholdPct => "consistency_threshold_pct",
            Self::CommissionPerContractSide => "commission_per_contract_side",
            Self::EvaluationProfitTarget => "evaluation_profit_target",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleOrigin {
    /// Differs from the preset value (S16 AC-2).
    Edited,
    /// Resolves a not-confirmed preset field (S16 AC-6).
    ChosenFromConflict,
    /// The account has no preset (real, or manual-mode prop).
    SetByUser,
}

impl RuleOrigin {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Edited => "edited",
            Self::ChosenFromConflict => "chosen_from_conflict",
            Self::SetByUser => "set_by_user",
        }
    }
}
