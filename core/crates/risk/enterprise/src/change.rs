//! Q48 / ADR-0006 §3.7: a saved value that tightens a rule applies now; one that loosens it applies
//! from the next trading day. "Looser" follows the contract: removing a rule, `flatten_and_block` →
//! `notify`, `daily_loss` `hard` → `soft_pause` / `none`, a higher amount, a lower commission
//! floor. A change that is not comparable (a drawdown type change, or a daily loss that is stricter
//! in one part and looser in another) counts as looser (fail closed). A reset change is always
//! scheduled.

use crate::rules::{DailyLoss, Drawdown, OnReach, ResetTime};
use crate::values::{LimitAmount, Rate, ThresholdPct};

/// What is pending until the next trading day (`pending_changes[].change`, plus the reset and
/// commission changes the contract shows apart).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pending {
    LimitRaised,
    RuleRemoved,
    ProfitTargetToNotify,
    DailyLossSoftened,
    DailyLossChanged,
    DrawdownTypeChanged,
    ResetChanged,
    CommissionFloorLowered,
    CommissionFloorRemoved,
}

impl Pending {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LimitRaised => "limit_raised",
            Self::RuleRemoved => "rule_removed",
            Self::ProfitTargetToNotify => "profit_target_to_notify",
            Self::DailyLossSoftened => "daily_loss_softened",
            Self::DailyLossChanged => "daily_loss_changed",
            Self::DrawdownTypeChanged => "drawdown_type_changed",
            Self::ResetChanged => "reset_changed",
            Self::CommissionFloorLowered => "commission_floor_lowered",
            Self::CommissionFloorRemoved => "commission_floor_removed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Effect {
    Unchanged,
    Now,
    NextTradingDay(Pending),
}

/// One kind's value in force and the latest saved one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleChange {
    DailyLoss(Option<DailyLoss>, Option<DailyLoss>),
    Drawdown(Option<Drawdown>, Option<Drawdown>),
    DailyProfitTarget(Option<LimitAmount>, Option<LimitAmount>),
    OnReach(OnReach, OnReach),
    Consistency(Option<ThresholdPct>, Option<ThresholdPct>),
    Reset(Option<ResetTime>, Option<ResetTime>),
    Commission(Option<Rate>, Option<Rate>),
    /// Notice only (Q70): never a limit, so always effective at once.
    EvaluationTarget(Option<LimitAmount>, Option<LimitAmount>),
}

pub fn classify(change: RuleChange) -> Effect {
    use Effect::{NextTradingDay as Next, Now, Unchanged};
    match change {
        RuleChange::DailyLoss(old, new) => match (old, new) {
            (o, n) if o == n => Unchanged,
            (None, Some(_)) => Now,
            (Some(_), None) => Next(Pending::RuleRemoved),
            (Some(o), Some(n)) => daily_loss(o, n),
            (None, None) => Unchanged,
        },
        RuleChange::Drawdown(old, new) => match (old, new) {
            (o, n) if o == n => Unchanged,
            (None, Some(_)) => Now,
            (Some(_), None) => Next(Pending::RuleRemoved),
            (Some(o), Some(n)) if o.kind != n.kind => Next(Pending::DrawdownTypeChanged),
            (Some(o), Some(n)) => amount(o.amount, n.amount),
            (None, None) => Unchanged,
        },
        RuleChange::DailyProfitTarget(old, new) => optional_amount(old, new),
        RuleChange::Consistency(old, new) => match (old, new) {
            (o, n) if o == n => Unchanged,
            (None, Some(_)) => Now,
            (Some(_), None) => Next(Pending::RuleRemoved),
            (Some(o), Some(n)) if n < o => Now,
            (Some(_), Some(_)) => Next(Pending::LimitRaised),
            (None, None) => Unchanged,
        },
        RuleChange::OnReach(old, new) => match (old, new) {
            (o, n) if o == n => Unchanged,
            (_, OnReach::FlattenAndBlock) => Now,
            (_, OnReach::Notify) => Next(Pending::ProfitTargetToNotify),
        },
        RuleChange::Reset(old, new) if old == new => Unchanged,
        RuleChange::Reset(..) => Next(Pending::ResetChanged),
        RuleChange::Commission(old, new) => match (old, new) {
            (o, n) if o == n => Unchanged,
            (None, Some(_)) => Now,
            (Some(_), None) => Next(Pending::CommissionFloorRemoved),
            (Some(o), Some(n)) if n > o => Now,
            (Some(_), Some(_)) => Next(Pending::CommissionFloorLowered),
            (None, None) => Unchanged,
        },
        RuleChange::EvaluationTarget(old, new) if old == new => Unchanged,
        RuleChange::EvaluationTarget(..) => Now,
    }
}

fn amount(old: LimitAmount, new: LimitAmount) -> Effect {
    if new < old {
        Effect::Now
    } else if new > old {
        Effect::NextTradingDay(Pending::LimitRaised)
    } else {
        Effect::Unchanged
    }
}

fn optional_amount(old: Option<LimitAmount>, new: Option<LimitAmount>) -> Effect {
    match (old, new) {
        (o, n) if o == n => Effect::Unchanged,
        (None, Some(_)) => Effect::Now,
        (Some(_), None) => Effect::NextTradingDay(Pending::RuleRemoved),
        (Some(o), Some(n)) => amount(o, n),
        (None, None) => Effect::Unchanged,
    }
}

/// Kind and amount compared separately; `none` has no amount (no limit).
fn daily_loss(old: DailyLoss, new: DailyLoss) -> Effect {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let kind = new.rank().cmp(&old.rank());
    let amt = match (old.amount(), new.amount()) {
        (Some(o), Some(n)) => o.cmp(&n), // Greater = the new amount is lower = stricter
        (None, Some(_)) => Greater,
        (Some(_), None) => Less,
        (None, None) => Equal,
    };
    match (kind, amt) {
        (Equal, Equal) => Effect::Unchanged,
        (Greater | Equal, Greater | Equal) => Effect::Now,
        (Less, Less | Equal) => Effect::NextTradingDay(Pending::DailyLossSoftened),
        (Equal, Less) => Effect::NextTradingDay(Pending::LimitRaised),
        (Greater, Less) | (Less, Greater) => Effect::NextTradingDay(Pending::DailyLossChanged),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::DrawdownType;
    use Effect::{NextTradingDay as Next, Now, Unchanged};

    fn a(s: &str) -> LimitAmount {
        LimitAmount::parse(s).unwrap()
    }

    fn dd(kind: DrawdownType, s: &str) -> Option<Drawdown> {
        Some(Drawdown { kind, amount: a(s) })
    }

    #[test]
    fn daily_loss_matrix() {
        use DailyLoss::{Hard, None as NoLimit, SoftPause};
        let c = |o, n| classify(RuleChange::DailyLoss(o, n));
        assert_eq!(c(Some(Hard(a("500"))), Some(Hard(a("500")))), Unchanged);
        assert_eq!(c(Some(Hard(a("500"))), Some(Hard(a("400")))), Now);
        assert_eq!(
            c(Some(Hard(a("500"))), Some(Hard(a("600")))),
            Next(Pending::LimitRaised)
        );
        assert_eq!(c(None, Some(SoftPause(a("500")))), Now);
        assert_eq!(c(Some(Hard(a("500"))), None), Next(Pending::RuleRemoved));
        assert_eq!(c(Some(SoftPause(a("500"))), Some(Hard(a("500")))), Now);
        assert_eq!(c(Some(SoftPause(a("500"))), Some(Hard(a("400")))), Now);
        assert_eq!(c(Some(NoLimit), Some(Hard(a("900")))), Now);
        assert_eq!(
            c(Some(Hard(a("500"))), Some(SoftPause(a("500")))),
            Next(Pending::DailyLossSoftened)
        );
        assert_eq!(
            c(Some(Hard(a("500"))), Some(NoLimit)),
            Next(Pending::DailyLossSoftened)
        );
        // Mixed (contract example): soft_pause → hard with a higher amount waits.
        assert_eq!(
            c(Some(SoftPause(a("500"))), Some(Hard(a("600")))),
            Next(Pending::DailyLossChanged)
        );
        assert_eq!(
            c(Some(Hard(a("500"))), Some(SoftPause(a("400")))),
            Next(Pending::DailyLossChanged)
        );
    }

    #[test]
    fn drawdown_type_change_is_not_comparable_and_waits() {
        use DrawdownType::{Static, TrailingEod};
        let c = |o, n| classify(RuleChange::Drawdown(o, n));
        assert_eq!(c(dd(Static, "2000"), dd(Static, "1500")), Now);
        assert_eq!(
            c(dd(Static, "2000"), dd(Static, "2500")),
            Next(Pending::LimitRaised)
        );
        // ADR-0006 §3.7 example: static 2000 → trailing_eod 2500, and a smaller amount too.
        assert_eq!(
            c(dd(Static, "2000"), dd(TrailingEod, "2500")),
            Next(Pending::DrawdownTypeChanged)
        );
        assert_eq!(
            c(dd(Static, "2000"), dd(TrailingEod, "1000")),
            Next(Pending::DrawdownTypeChanged)
        );
        assert_eq!(c(None, dd(Static, "2000")), Now);
        assert_eq!(c(dd(Static, "2000"), None), Next(Pending::RuleRemoved));
    }

    #[test]
    fn target_reach_consistency_reset_commission() {
        assert_eq!(
            classify(RuleChange::DailyProfitTarget(
                Some(a("800")),
                Some(a("600"))
            )),
            Now
        );
        assert_eq!(
            classify(RuleChange::OnReach(
                OnReach::FlattenAndBlock,
                OnReach::Notify
            )),
            Next(Pending::ProfitTargetToNotify)
        );
        assert_eq!(
            classify(RuleChange::OnReach(
                OnReach::Notify,
                OnReach::FlattenAndBlock
            )),
            Now
        );
        let t = |s| ThresholdPct::parse(s).ok();
        assert_eq!(classify(RuleChange::Consistency(t("40"), t("30"))), Now);
        assert_eq!(
            classify(RuleChange::Consistency(t("30"), t("40"))),
            Next(Pending::LimitRaised)
        );
        let reset = |h| {
            Some(ResetTime {
                local_time: chrono::NaiveTime::from_hms_opt(h, 0, 0).unwrap(),
                time_zone: chrono_tz::America::New_York,
            })
        };
        // Even an earlier reset waits: moving it mid-day would end today's block early.
        assert_eq!(
            classify(RuleChange::Reset(reset(17), reset(16))),
            Next(Pending::ResetChanged)
        );
        assert_eq!(classify(RuleChange::Reset(reset(17), reset(17))), Unchanged);
        let r = |s| Rate::parse(s).ok();
        assert_eq!(classify(RuleChange::Commission(r("0.50"), r("0.62"))), Now);
        assert_eq!(classify(RuleChange::Commission(None, r("0.62"))), Now);
        assert_eq!(
            classify(RuleChange::Commission(r("0.62"), r("0.50"))),
            Next(Pending::CommissionFloorLowered)
        );
        assert_eq!(
            classify(RuleChange::Commission(r("0.62"), None)),
            Next(Pending::CommissionFloorRemoved)
        );
        assert_eq!(
            classify(RuleChange::EvaluationTarget(
                Some(a("3000")),
                Some(a("4000"))
            )),
            Now
        );
    }
}
