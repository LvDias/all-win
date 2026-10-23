//! S13b — evaluation progress (Q46): achieved = amount earned before + the Core's net P&L since the
//! evaluation start (manual trades never count); remaining = max(0, target − achieved). Reaching the
//! target is a notice only (Q70): nothing automatic happens.

use rust_decimal::Decimal;

use crate::values::{LimitAmount, truncated_percent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationState {
    InProgress,
    TargetReached,
}

impl EvaluationState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProgress => "in_progress",
            Self::TargetReached => "target_reached",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationProgress {
    pub profit_target: Decimal,
    pub earned_before: Decimal,
    pub achieved: Decimal,
    pub remaining: Decimal,
    pub percent: u32,
    pub state: EvaluationState,
}

impl EvaluationProgress {
    pub fn new(target: LimitAmount, earned_before: Decimal, net_since_start: Decimal) -> Self {
        let target = target.value();
        let achieved = earned_before + net_since_start;
        let state = if achieved >= target {
            EvaluationState::TargetReached
        } else {
            EvaluationState::InProgress
        };
        Self {
            profit_target: target,
            earned_before,
            achieved,
            remaining: (target - achieved).max(Decimal::ZERO),
            percent: truncated_percent(achieved, target),
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    #[test]
    fn progress_adds_the_amount_earned_before() {
        let p =
            EvaluationProgress::new(LimitAmount::parse("3000").unwrap(), d("120.50"), d("1500"));
        assert_eq!(
            (p.achieved, p.remaining, p.percent),
            (d("1620.50"), d("1379.50"), 54)
        );
        assert_eq!(p.state, EvaluationState::InProgress);
    }

    #[test]
    fn reached_shows_zero_remaining_and_losses_show_zero_percent() {
        let t = LimitAmount::parse("3000").unwrap();
        let p = EvaluationProgress::new(t, d("0"), d("3200"));
        assert_eq!(
            (p.remaining, p.state),
            (d("0"), EvaluationState::TargetReached)
        );
        let loss = EvaluationProgress::new(t, d("0"), d("-400"));
        assert_eq!((loss.percent, loss.remaining), (0, d("3400")));
    }
}
