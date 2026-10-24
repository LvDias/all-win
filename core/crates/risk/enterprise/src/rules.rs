//! The account's risk rules as the risk module tracks them (the S16 value types, `RiskRules`).
//! An absent rule is not set; a drawdown of type `other` is set but untracked.

use chrono::NaiveTime;
use chrono_tz::Tz;

use rust_decimal::Decimal;

use crate::values::{LimitAmount, NonNegativeMoney, Rate, ThresholdPct};

/// The four rules with a meter (`RiskRuleKind`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleKind {
    DailyLoss,
    Drawdown,
    DailyProfitTarget,
    Consistency,
}

impl RuleKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DailyLoss => "daily_loss",
            Self::Drawdown => "drawdown",
            Self::DailyProfitTarget => "daily_profit_target",
            Self::Consistency => "consistency",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawdownType {
    TrailingIntraday,
    TrailingEod,
    Static,
    /// Published but not computable (`description` only): listed in `untracked_rules`.
    Other,
}

impl DrawdownType {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "trailing_intraday" => Self::TrailingIntraday,
            "trailing_eod" => Self::TrailingEod,
            "static" => Self::Static,
            "other" => Self::Other,
            _ => return None,
        })
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrailingIntraday => "trailing_intraday",
            Self::TrailingEod => "trailing_eod",
            Self::Static => "static",
            Self::Other => "other",
        }
    }

    pub const fn is_tracked(self) -> bool {
        !matches!(self, Self::Other)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Drawdown {
    pub kind: DrawdownType,
    pub amount: LimitAmount,
    /// Trailing types only (Q90). `None` = not modelled: the floor keeps trailing (with a warning).
    pub lock: Option<TrailingLock>,
}

/// When a trailing floor stops trailing (`DrawdownValue.trailing_lock`, Q90).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrailingLock {
    /// The floor locks once it reaches `size_amount` + `offset` (the account's own size, Q73).
    FloorReachesStart { offset: NonNegativeMoney },
    /// The firm states it never stops trailing.
    Never,
}

impl Drawdown {
    /// The equity at which the floor locks, if it ever does.
    pub fn lock_at(&self, size_amount: Decimal) -> Option<Decimal> {
        match (self.kind, self.lock) {
            (
                DrawdownType::TrailingIntraday | DrawdownType::TrailingEod,
                Some(TrailingLock::FloorReachesStart { offset }),
            ) => Some(size_amount + offset.value()),
            _ => None,
        }
    }
}

/// `DailyLossValue`. `None` = the firm states it has no daily loss limit (a value, not "unset").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DailyLoss {
    Hard(LimitAmount),
    SoftPause(LimitAmount),
    None,
}

impl DailyLoss {
    pub const fn amount(&self) -> Option<LimitAmount> {
        match self {
            Self::Hard(a) | Self::SoftPause(a) => Some(*a),
            Self::None => None,
        }
    }

    pub const fn kind_str(&self) -> &'static str {
        match self {
            Self::Hard(_) => "hard",
            Self::SoftPause(_) => "soft_pause",
            Self::None => "none",
        }
    }

    /// Strictness of the kind: `hard` > `soft_pause` > `none`.
    pub(crate) const fn rank(&self) -> u8 {
        match self {
            Self::Hard(_) => 2,
            Self::SoftPause(_) => 1,
            Self::None => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OnReach {
    #[default]
    Notify,
    FlattenAndBlock,
}

impl OnReach {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notify => "notify",
            Self::FlattenAndBlock => "flatten_and_block",
        }
    }
}

/// A firm's trading-day reset (prop accounts' copy, Q-S13-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResetTime {
    pub local_time: NaiveTime,
    pub time_zone: Tz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RiskRules {
    pub drawdown: Option<Drawdown>,
    pub daily_loss: Option<DailyLoss>,
    pub daily_profit_target: Option<LimitAmount>,
    pub on_reach: OnReach,
    pub consistency_threshold: Option<ThresholdPct>,
    /// `None` for real accounts (CME session close).
    pub reset: Option<ResetTime>,
    pub evaluation_profit_target: Option<LimitAmount>,
    pub commission_per_contract_side: Option<Rate>,
}

impl RiskRules {
    /// Rules not set, in meter order (UI "sem limite definido").
    pub fn unset(&self) -> Vec<RuleKind> {
        let mut out = Vec::new();
        if self.daily_loss.is_none() {
            out.push(RuleKind::DailyLoss);
        }
        if self.drawdown.is_none() {
            out.push(RuleKind::Drawdown);
        }
        if self.daily_profit_target.is_none() {
            out.push(RuleKind::DailyProfitTarget);
        }
        if self.consistency_threshold.is_none() {
            out.push(RuleKind::Consistency);
        }
        out
    }

    /// Rules set but not tracked (a drawdown of type `other`).
    pub fn untracked(&self) -> Vec<RuleKind> {
        match self.drawdown {
            Some(d) if !d.kind.is_tracked() => vec![RuleKind::Drawdown],
            _ => Vec::new(),
        }
    }
}
