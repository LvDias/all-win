//! `risk` module — enterprise layer (V1-S13 / S13b). Pure rules: the account's risk rule values,
//! whether a change tightens (now) or loosens (next trading day, ADR-0006 §3.7), the trading day,
//! the meters (used / limit / truncated percent / state), the trailing drawdown and the evaluation
//! progress. Every figure is net and exact (`Decimal`); nothing here names a platform.

pub mod change;
pub mod drawdown;
pub mod evaluation;
pub mod meter;
pub mod rules;
pub mod trading_day;
pub mod values;

pub use change::{Effect, Pending, RuleChange, classify};
pub use drawdown::DrawdownTracker;
pub use evaluation::{EvaluationProgress, EvaluationState};
pub use meter::{Meter, MeterState, consistency_share, live_net};
pub use rules::{DailyLoss, Drawdown, DrawdownType, OnReach, ResetTime, RiskRules, RuleKind};
pub use trading_day::{ResetSchedule, TradingDay};
pub use values::{LimitAmount, Rate, ThresholdPct, ValueError, truncated_percent};
