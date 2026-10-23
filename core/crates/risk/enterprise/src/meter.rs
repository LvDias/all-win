//! Meters (`RiskMeter`): used vs limit, truncated percent and state. Exactly three levels plus the
//! target (F§7.1, D18): `normal` < 80 %, `warning` ≥ 80 %, `reached` at the limit, and
//! `target_reached` for the daily profit target. Consistency never goes beyond `warning` (AC-6).

use rust_decimal::{Decimal, RoundingStrategy};

use crate::rules::RuleKind;
use crate::values::{Rate, truncated_percent};

/// The warning level, fixed at 80 % (D18).
const WARNING_PCT: u32 = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeterState {
    Normal,
    Warning,
    TargetReached,
    Reached,
}

impl MeterState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Warning => "warning",
            Self::TargetReached => "target_reached",
            Self::Reached => "reached",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meter {
    pub kind: RuleKind,
    pub state: MeterState,
    pub limit: Decimal,
    pub used: Decimal,
    pub percent: u32,
}

impl Meter {
    /// Daily loss or drawdown: `used` is the loss (or drawdown) consumed, ≥ 0.
    pub fn limit(kind: RuleKind, used: Decimal, limit: Decimal) -> Self {
        let used = used.max(Decimal::ZERO);
        let percent = truncated_percent(used, limit);
        let state = if used >= limit {
            MeterState::Reached
        } else {
            level(percent)
        };
        Self {
            kind,
            state,
            limit,
            used,
            percent,
        }
    }

    /// Daily profit target: `used` is the profit made today (may be negative; shown as is).
    pub fn target(used: Decimal, limit: Decimal) -> Self {
        let percent = truncated_percent(used, limit);
        let state = if used >= limit {
            MeterState::TargetReached
        } else {
            level(percent)
        };
        Self {
            kind: RuleKind::DailyProfitTarget,
            state,
            limit,
            used,
            percent,
        }
    }

    /// Consistency: `share` is the largest day's share of the total profit, as a percent; tracked
    /// and warned, never reached.
    pub fn consistency(share: Decimal, threshold: Decimal) -> Self {
        let percent = truncated_percent(share, threshold);
        Self {
            kind: RuleKind::Consistency,
            state: level(percent),
            limit: threshold,
            used: share,
            percent,
        }
    }
}

const fn level(percent: u32) -> MeterState {
    if percent >= WARNING_PCT {
        MeterState::Warning
    } else {
        MeterState::Normal
    }
}

/// Today's live net P&L: realized net + last known unrealized − the expected closing commission
/// (floor × open quantity; fail closed: a limit triggers a few dollars earlier, never later).
pub fn live_net(
    realized_net: Decimal,
    unrealized: Option<Decimal>,
    open_qty: u32,
    floor: Option<Rate>,
) -> Decimal {
    let closing = floor.map_or(Decimal::ZERO, |r| r.value() * Decimal::from(open_qty));
    realized_net + unrealized.unwrap_or(Decimal::ZERO) - closing
}

/// The largest day's share of the total net profit, in percent truncated to one decimal; `None`
/// while the total is not positive (nothing to compare).
pub fn consistency_share(days_net: &[Decimal]) -> Option<Decimal> {
    let total: Decimal = days_net.iter().sum();
    let best = days_net.iter().copied().max()?;
    if total <= Decimal::ZERO || best <= Decimal::ZERO {
        return None;
    }
    Some((best * Decimal::ONE_HUNDRED / total).round_dp_with_strategy(1, RoundingStrategy::ToZero))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    #[test]
    fn limit_levels() {
        let m = |u| Meter::limit(RuleKind::DailyLoss, d(u), d("500"));
        assert_eq!(
            (m("399.99").percent, m("399.99").state),
            (79, MeterState::Normal)
        );
        assert_eq!(
            (m("400").percent, m("400").state),
            (80, MeterState::Warning)
        );
        assert_eq!(m("499.99").state, MeterState::Warning);
        assert_eq!(m("500").state, MeterState::Reached);
        assert_eq!((m("-20").used, m("-20").percent), (d("0"), 0));
    }

    #[test]
    fn target_and_consistency_levels() {
        assert_eq!(
            Meter::target(d("800"), d("800")).state,
            MeterState::TargetReached
        );
        assert_eq!(Meter::target(d("-50"), d("800")).state, MeterState::Normal);
        let c = Meter::consistency(d("75"), d("30"));
        assert_eq!((c.percent, c.state), (250, MeterState::Warning));
    }

    #[test]
    fn live_net_deducts_the_expected_closing_commission() {
        let floor = Rate::parse("0.62").ok();
        assert_eq!(
            live_net(d("-100"), Some(d("-50.25")), 3, floor),
            d("-152.11")
        );
        assert_eq!(live_net(d("-100"), None, 0, floor), d("-100"));
        assert_eq!(live_net(d("10"), Some(d("5")), 2, None), d("15"));
    }

    #[test]
    fn consistency_share_of_the_best_day() {
        assert_eq!(
            consistency_share(&[d("300"), d("200"), d("-100")]),
            Some(d("75.0"))
        );
        assert_eq!(consistency_share(&[d("100"), d("200")]), Some(d("66.6")));
        assert_eq!(consistency_share(&[d("-100"), d("50")]), None);
        assert_eq!(consistency_share(&[]), None);
    }

    #[test]
    fn worst_state_order() {
        assert!(MeterState::Reached > MeterState::TargetReached);
        assert!(MeterState::TargetReached > MeterState::Warning);
    }
}
