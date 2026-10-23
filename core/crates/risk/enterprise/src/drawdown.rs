//! Drawdown on the All Win equity (Esquadro, 2026-10-23): equity = account size + Σ net realized
//! since the evaluation start + the last known unrealized. Deposits, withdrawals and earlier P&L do
//! not count. `floor` is the equity at which the drawdown is breached, as it stands now (Q-S13-5).
//! - static: the floor stays at the start equity − amount;
//! - trailing intraday: the high water follows the live equity;
//! - trailing end-of-day: the high water follows each day's closing (realized) equity.
//!
//! A firm's "stops trailing at" is published text only, so it is not applied automatically.

use rust_decimal::Decimal;

use crate::meter::Meter;
use crate::rules::{Drawdown, DrawdownType, RuleKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawdownTracker {
    pub high_water: Decimal,
    pub floor: Decimal,
}

impl DrawdownTracker {
    pub fn start(rule: Drawdown, start_equity: Decimal) -> Self {
        Self {
            high_water: start_equity,
            floor: start_equity - rule.amount.value(),
        }
    }

    /// A live equity observation (trailing intraday only moves the high water up).
    pub fn observe(&mut self, rule: Drawdown, equity: Decimal) {
        if rule.kind == DrawdownType::TrailingIntraday {
            self.raise(rule, equity);
        }
    }

    /// The day's closing equity (trailing end-of-day only moves the high water up).
    pub fn close_day(&mut self, rule: Drawdown, closing_equity: Decimal) {
        if rule.kind == DrawdownType::TrailingEod {
            self.raise(rule, closing_equity);
        }
    }

    fn raise(&mut self, rule: Drawdown, equity: Decimal) {
        if equity > self.high_water {
            self.high_water = equity;
            self.floor = equity - rule.amount.value();
        }
    }

    /// Used = how much of the amount the equity has consumed above the floor.
    pub fn meter(&self, rule: Drawdown, equity: Decimal) -> Meter {
        let amount = rule.amount.value();
        Meter::limit(RuleKind::Drawdown, self.floor + amount - equity, amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meter::MeterState;
    use crate::values::LimitAmount;
    use std::str::FromStr;

    fn d(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    fn rule(kind: DrawdownType) -> Drawdown {
        Drawdown {
            kind,
            amount: LimitAmount::parse("2000").unwrap(),
        }
    }

    #[test]
    fn static_never_trails() {
        let r = rule(DrawdownType::Static);
        let mut t = DrawdownTracker::start(r, d("50000"));
        t.observe(r, d("52000"));
        t.close_day(r, d("52000"));
        assert_eq!(t.floor, d("48000"));
        let m = t.meter(r, d("48500"));
        assert_eq!(
            (m.used, m.percent, m.state),
            (d("1500"), 75, MeterState::Normal)
        );
        assert_eq!(t.meter(r, d("48000")).state, MeterState::Reached);
    }

    #[test]
    fn trailing_intraday_follows_the_live_peak() {
        let r = rule(DrawdownType::TrailingIntraday);
        let mut t = DrawdownTracker::start(r, d("50000"));
        t.observe(r, d("51200.50"));
        t.observe(r, d("50900"));
        assert_eq!((t.high_water, t.floor), (d("51200.50"), d("49200.50")));
        assert_eq!(t.meter(r, d("49600.50")).state, MeterState::Warning);
    }

    #[test]
    fn trailing_eod_moves_only_at_the_close() {
        let r = rule(DrawdownType::TrailingEod);
        let mut t = DrawdownTracker::start(r, d("50000"));
        t.observe(r, d("53000"));
        assert_eq!(t.floor, d("48000"));
        t.close_day(r, d("51000"));
        assert_eq!(t.floor, d("49000"));
        t.close_day(r, d("50500"));
        assert_eq!(t.floor, d("49000"));
    }
}
