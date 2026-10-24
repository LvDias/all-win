//! Drawdown on the All Win equity (Esquadro, 2026-10-23): equity = account size + Σ net realized
//! since the evaluation start + the last known unrealized. Deposits, withdrawals and earlier P&L do
//! not count. `floor` is the equity at which the drawdown is breached, as it stands now (Q-S13-5).
//! - static: the floor stays at the start equity − amount;
//! - trailing intraday: the high water follows the live equity;
//! - trailing end-of-day: the high water follows each day's closing (realized) equity.
//!
//!
//! Trailing lock (Q90): with `trailing_lock` `floor_reaches_start`, the floor stops trailing once it
//! reaches `size_amount` + offset and stays there (`locked`, persisted as
//! `drawdown_tracker.stopped_trailing_at`). Without a modelled lock it keeps trailing (stricter than
//! the firm; the UI warns). Removing a lock that already locked makes it trail again from the
//! current high water (`release`).

use rust_decimal::Decimal;

use crate::meter::Meter;
use crate::rules::{Drawdown, DrawdownType, RuleKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawdownTracker {
    pub high_water: Decimal,
    pub floor: Decimal,
    /// The floor reached the lock and no longer trails.
    pub locked: bool,
}

impl DrawdownTracker {
    /// `lock_at` = [`Drawdown::lock_at`] for the account's size.
    pub fn start(rule: Drawdown, start_equity: Decimal, lock_at: Option<Decimal>) -> Self {
        let mut t = Self {
            high_water: start_equity,
            floor: start_equity - rule.amount.value(),
            locked: false,
        };
        t.apply_lock(lock_at);
        t
    }

    /// A live equity observation (trailing intraday only moves the high water up).
    pub fn observe(&mut self, rule: Drawdown, equity: Decimal, lock_at: Option<Decimal>) {
        if rule.kind == DrawdownType::TrailingIntraday {
            self.raise(rule, equity, lock_at);
        }
    }

    /// The day's closing equity (trailing end-of-day only moves the high water up).
    pub fn close_day(&mut self, rule: Drawdown, closing_equity: Decimal, lock_at: Option<Decimal>) {
        if rule.kind == DrawdownType::TrailingEod {
            self.raise(rule, closing_equity, lock_at);
        }
    }

    /// The lock was removed (or will never apply): trail again from the current high water.
    pub fn release(&mut self, rule: Drawdown) {
        if self.locked {
            self.locked = false;
            self.floor = self.high_water - rule.amount.value();
        }
    }

    fn raise(&mut self, rule: Drawdown, equity: Decimal, lock_at: Option<Decimal>) {
        if equity > self.high_water {
            self.high_water = equity;
            if !self.locked {
                self.floor = equity - rule.amount.value();
                self.apply_lock(lock_at);
            }
        }
    }

    fn apply_lock(&mut self, lock_at: Option<Decimal>) {
        if let Some(lock) = lock_at
            && self.floor >= lock
        {
            self.floor = lock;
            self.locked = true;
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
            lock: None,
        }
    }

    #[test]
    fn static_never_trails() {
        let r = rule(DrawdownType::Static);
        let mut t = DrawdownTracker::start(r, d("50000"), None);
        t.observe(r, d("52000"), None);
        t.close_day(r, d("52000"), None);
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
        let mut t = DrawdownTracker::start(r, d("50000"), None);
        t.observe(r, d("51200.50"), None);
        t.observe(r, d("50900"), None);
        assert_eq!((t.high_water, t.floor), (d("51200.50"), d("49200.50")));
        assert_eq!(t.meter(r, d("49600.50")).state, MeterState::Warning);
    }

    #[test]
    fn trailing_eod_moves_only_at_the_close() {
        let r = rule(DrawdownType::TrailingEod);
        let mut t = DrawdownTracker::start(r, d("50000"), None);
        t.observe(r, d("53000"), None);
        assert_eq!(t.floor, d("48000"));
        t.close_day(r, d("51000"), None);
        assert_eq!(t.floor, d("49000"));
        t.close_day(r, d("50500"), None);
        assert_eq!(t.floor, d("49000"));
    }

    #[test]
    fn a_trailing_lock_stops_the_floor_at_size_plus_offset() {
        use crate::rules::TrailingLock;
        use crate::values::NonNegativeMoney;
        let r = Drawdown {
            lock: Some(TrailingLock::FloorReachesStart {
                offset: NonNegativeMoney::parse("100").unwrap(),
            }),
            ..rule(DrawdownType::TrailingIntraday)
        };
        let lock = r.lock_at(d("50000"));
        assert_eq!(lock, Some(d("50100")));
        let mut t = DrawdownTracker::start(r, d("50000"), lock);
        t.observe(r, d("51900"), lock);
        assert_eq!((t.floor, t.locked), (d("49900"), false));
        // The floor would be 50 300: it locks at 50 100 and stays there.
        t.observe(r, d("52300"), lock);
        assert_eq!((t.floor, t.locked), (d("50100"), true));
        t.observe(r, d("55000"), lock);
        assert_eq!((t.high_water, t.floor), (d("55000"), d("50100")));
        assert_eq!(t.meter(r, d("50100")).state, MeterState::Reached);
        // Removing the lock afterwards: it trails again from the current high water.
        t.release(r);
        assert_eq!((t.floor, t.locked), (d("53000"), false));
    }

    #[test]
    fn never_or_no_lock_keeps_trailing_and_static_ignores_a_lock() {
        use crate::rules::TrailingLock;
        let never = Drawdown {
            lock: Some(TrailingLock::Never),
            ..rule(DrawdownType::TrailingEod)
        };
        assert_eq!(never.lock_at(d("50000")), None);
        let mut t = DrawdownTracker::start(never, d("50000"), None);
        t.close_day(never, d("60000"), None);
        assert_eq!((t.floor, t.locked), (d("58000"), false));
        let fixed = Drawdown {
            lock: Some(TrailingLock::FloorReachesStart {
                offset: crate::values::NonNegativeMoney::parse("0").unwrap(),
            }),
            ..rule(DrawdownType::Static)
        };
        assert_eq!(fixed.lock_at(d("50000")), None);
    }
}
