//! Derivation of the limits in force from the account's rule inputs (ADR-0006 §3.7, Q48). The
//! inputs are the user's latest saved value of each kind with that kind's own `saved_at`; the
//! derivation decides, kind by kind, against the limits in force:
//! - stricter (or the first derivation) → in force now;
//! - looser → scheduled from the start of the trading day after that kind's `saved_at`, computed
//!   with the reset IN FORCE. A schedule whose start has already passed is in force now.
//!
//! The trading-day reset step is the same derivation run at the new day's start: every due
//! schedule falls into the limits in force there. Blocks are not touched here (no early unlock).

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::change::{Effect, Pending, RuleChange, classify};
use crate::rules::RiskRules;
use crate::trading_day::ResetSchedule;

/// The kinds a schedule can hold (`risk.scheduled_limit.kind`). The evaluation target is a notice
/// only, so it is always in force at once and never scheduled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LimitKind {
    DailyLoss,
    Drawdown,
    DailyProfitTarget,
    Consistency,
    ProfitTargetOnReach,
    TradingDayReset,
    CommissionPerContractSide,
}

impl LimitKind {
    pub const ALL: [Self; 7] = [
        Self::DailyLoss,
        Self::Drawdown,
        Self::DailyProfitTarget,
        Self::Consistency,
        Self::ProfitTargetOnReach,
        Self::TradingDayReset,
        Self::CommissionPerContractSide,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DailyLoss => "daily_loss",
            Self::Drawdown => "drawdown",
            Self::DailyProfitTarget => "daily_profit_target",
            Self::Consistency => "consistency",
            Self::ProfitTargetOnReach => "profit_target_on_reach",
            Self::TradingDayReset => "trading_day_reset",
            Self::CommissionPerContractSide => "commission_per_contract_side",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.as_str() == s)
    }

    fn change(self, old: &RiskRules, new: &RiskRules) -> RuleChange {
        match self {
            Self::DailyLoss => RuleChange::DailyLoss(old.daily_loss, new.daily_loss),
            Self::Drawdown => RuleChange::Drawdown(old.drawdown, new.drawdown),
            Self::DailyProfitTarget => {
                RuleChange::DailyProfitTarget(old.daily_profit_target, new.daily_profit_target)
            }
            Self::Consistency => {
                RuleChange::Consistency(old.consistency_threshold, new.consistency_threshold)
            }
            Self::ProfitTargetOnReach => RuleChange::OnReach(old.on_reach, new.on_reach),
            Self::TradingDayReset => RuleChange::Reset(old.reset, new.reset),
            Self::CommissionPerContractSide => RuleChange::Commission(
                old.commission_per_contract_side,
                new.commission_per_contract_side,
            ),
        }
    }

    fn copy(self, into: &mut RiskRules, from: &RiskRules) {
        match self {
            Self::DailyLoss => into.daily_loss = from.daily_loss,
            Self::Drawdown => into.drawdown = from.drawdown,
            Self::DailyProfitTarget => into.daily_profit_target = from.daily_profit_target,
            Self::Consistency => into.consistency_threshold = from.consistency_threshold,
            Self::ProfitTargetOnReach => into.on_reach = from.on_reach,
            Self::TradingDayReset => into.reset = from.reset,
            Self::CommissionPerContractSide => {
                into.commission_per_contract_side = from.commission_per_contract_side;
            }
        }
    }
}

/// The account's rule inputs from the accounts query port (one consistent read).
#[derive(Debug, Clone, PartialEq)]
pub struct RuleInputs {
    pub rules: RiskRules,
    /// Each kind's own save time (ADR-0006 §3.7). A kind the user never saved (a preset or default
    /// value) uses `baseline_at`, the time that value became an input.
    pub saved_at: HashMap<LimitKind, DateTime<Utc>>,
    pub baseline_at: DateTime<Utc>,
}

impl RuleInputs {
    fn saved_at(&self, kind: LimitKind) -> DateTime<Utc> {
        self.saved_at
            .get(&kind)
            .copied()
            .unwrap_or(self.baseline_at)
    }
}

/// A looser value waiting for a later trading day (`risk.scheduled_limit`); its value is the
/// input's value of that kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scheduled {
    pub kind: LimitKind,
    pub change: Pending,
    pub effective_from: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Derivation {
    pub in_force: RiskRules,
    pub scheduled: Vec<Scheduled>,
}

/// `in_force` = the limits in force now (`None` at the first derivation: everything applies at
/// once); `reset` = the reset IN FORCE (a reset change never moves today's boundary).
pub fn derive(
    in_force: Option<&RiskRules>,
    inputs: &RuleInputs,
    reset: ResetSchedule,
    now: DateTime<Utc>,
) -> Derivation {
    let Some(current) = in_force else {
        return Derivation {
            in_force: inputs.rules,
            scheduled: Vec::new(),
        };
    };
    let mut next = *current;
    next.evaluation_profit_target = inputs.rules.evaluation_profit_target;
    let mut scheduled = Vec::new();
    for kind in LimitKind::ALL {
        match classify(kind.change(current, &inputs.rules)) {
            Effect::Unchanged => {}
            Effect::Now => kind.copy(&mut next, &inputs.rules),
            Effect::NextTradingDay(change) => {
                let effective_from = reset.next_day_start(inputs.saved_at(kind));
                if effective_from <= now {
                    kind.copy(&mut next, &inputs.rules);
                } else {
                    scheduled.push(Scheduled {
                        kind,
                        change,
                        effective_from,
                    });
                }
            }
        }
    }
    Derivation {
        in_force: next,
        scheduled,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{DailyLoss, Drawdown, DrawdownType, OnReach, ResetTime};
    use crate::values::{LimitAmount, Rate};
    use chrono::NaiveTime;

    fn utc(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }

    fn a(s: &str) -> LimitAmount {
        LimitAmount::parse(s).unwrap()
    }

    fn reset() -> ResetSchedule {
        ResetSchedule::Firm(ResetTime {
            local_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            time_zone: chrono_tz::America::New_York,
        })
    }

    fn rules(loss: &str) -> RiskRules {
        RiskRules {
            daily_loss: Some(DailyLoss::Hard(a(loss))),
            drawdown: Some(Drawdown {
                kind: DrawdownType::TrailingEod,
                amount: a("2000"),
                lock: None,
            }),
            ..RiskRules::default()
        }
    }

    // Tue 2026-11-03 10:00 New York; the day ends at 22:00Z.
    const NOW: &str = "2026-11-03T15:00:00Z";

    fn inputs(r: RiskRules, saved: &[(LimitKind, &str)]) -> RuleInputs {
        RuleInputs {
            rules: r,
            saved_at: saved.iter().map(|(k, t)| (*k, utc(t))).collect(),
            baseline_at: utc("2026-10-01T00:00:00Z"),
        }
    }

    #[test]
    fn the_first_derivation_puts_every_input_in_force() {
        let i = inputs(rules("500"), &[]);
        let d = derive(None, &i, reset(), utc(NOW));
        assert_eq!(d.in_force, i.rules);
        assert!(d.scheduled.is_empty());
    }

    #[test]
    fn stricter_now_looser_tomorrow_and_due_schedules_apply_at_the_reset() {
        let current = rules("500");
        // Daily loss raised (looser) and commission floor added (stricter), saved now.
        let mut wanted = rules("800");
        wanted.commission_per_contract_side = Rate::parse("0.62").ok();
        let i = inputs(
            wanted,
            &[
                (LimitKind::DailyLoss, NOW),
                (LimitKind::CommissionPerContractSide, NOW),
            ],
        );
        let d = derive(Some(&current), &i, reset(), utc(NOW));
        assert_eq!(d.in_force.daily_loss, current.daily_loss);
        assert_eq!(
            d.in_force.commission_per_contract_side,
            Rate::parse("0.62").ok()
        );
        assert_eq!(
            d.scheduled,
            vec![Scheduled {
                kind: LimitKind::DailyLoss,
                change: Pending::LimitRaised,
                effective_from: utc("2026-11-03T22:00:00Z"),
            }]
        );
        // The reset step is the same derivation at the new day's start.
        let at_reset = derive(Some(&d.in_force), &i, reset(), utc("2026-11-03T22:00:00Z"));
        assert_eq!(at_reset.in_force, i.rules);
        assert!(at_reset.scheduled.is_empty());
    }

    #[test]
    fn latest_save_wins_a_stricter_save_replaces_a_pending_looser_one() {
        let current = rules("500");
        // Earlier today 800 was pending; the latest save is 400 (stricter).
        let i = inputs(rules("400"), &[(LimitKind::DailyLoss, NOW)]);
        let d = derive(Some(&current), &i, reset(), utc(NOW));
        assert_eq!(d.in_force.daily_loss, Some(DailyLoss::Hard(a("400"))));
        assert!(d.scheduled.is_empty());
        // Saving the value in force again cancels the pending one.
        let back = inputs(rules("500"), &[(LimitKind::DailyLoss, NOW)]);
        assert!(
            derive(Some(&current), &back, reset(), utc(NOW))
                .scheduled
                .is_empty()
        );
    }

    #[test]
    fn a_looser_save_from_a_past_day_is_already_due() {
        let current = rules("500");
        // Saved yesterday (Mon 12:00 New York); derived only today: in force now.
        let i = inputs(
            rules("800"),
            &[(LimitKind::DailyLoss, "2026-11-02T17:00:00Z")],
        );
        let d = derive(Some(&current), &i, reset(), utc(NOW));
        assert_eq!(d.in_force.daily_loss, Some(DailyLoss::Hard(a("800"))));
    }

    #[test]
    fn reset_and_on_reach_changes_wait_and_the_evaluation_target_never_does() {
        let mut current = rules("500");
        current.on_reach = OnReach::FlattenAndBlock;
        let mut wanted = current;
        wanted.on_reach = OnReach::Notify;
        wanted.reset = Some(ResetTime {
            local_time: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            time_zone: chrono_tz::America::New_York,
        });
        wanted.evaluation_profit_target = Some(a("3000"));
        let i = inputs(
            wanted,
            &[
                (LimitKind::ProfitTargetOnReach, NOW),
                (LimitKind::TradingDayReset, NOW),
            ],
        );
        let d = derive(Some(&current), &i, reset(), utc(NOW));
        assert_eq!(d.in_force.on_reach, OnReach::FlattenAndBlock);
        assert_eq!(d.in_force.reset, None);
        assert_eq!(d.in_force.evaluation_profit_target, Some(a("3000")));
        let kinds: Vec<_> = d.scheduled.iter().map(|s| (s.kind, s.change)).collect();
        assert_eq!(
            kinds,
            vec![
                (
                    LimitKind::ProfitTargetOnReach,
                    Pending::ProfitTargetToNotify
                ),
                (LimitKind::TradingDayReset, Pending::ResetChanged),
            ]
        );
    }

    #[test]
    fn kinds_round_trip() {
        for k in LimitKind::ALL {
            assert_eq!(LimitKind::parse(k.as_str()), Some(k));
        }
    }
}
