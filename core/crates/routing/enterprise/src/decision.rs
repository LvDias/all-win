//! Per-account routing decision for one instruction (ADR-0003 §4.1 gates, §4.2 actions;
//! S7 AC-2…AC-5, AC-8; ADR-0006 §3.3). Pure: the application layer gathers the facts at the
//! decision and stores the result with its snapshots (`routing.routing_decision`).
//!
//! V1 scope: the TradingView actions (`buy`/`sell` with `open`, `close`, `reverse`); `reduce`,
//! `modify` and `cancel` are refused at ingress for that source (ADR-0003 §6).

use crate::route::{RouteSignalType, SignalTypes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Long,
    Short,
}

impl Side {
    const fn sign(self) -> i64 {
        match self {
            Self::Long => 1,
            Self::Short => -1,
        }
    }
}

/// The instruction's action for one account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// `buy`/`sell` with `position_effect: open` (the V1 TradingView default).
    Open {
        side: Side,
        quantity: u32,
    },
    Close,
    Reverse {
        side: Side,
        quantity: u32,
    },
}

impl Action {
    /// Whether the action can increase exposure; only `close` is an exit here (fail closed).
    pub const fn increases_position(self) -> bool {
        !matches!(self, Self::Close)
    }
}

/// Facts for one account at the decision. A gate whose owning slice is not wired yet is `None`
/// and does not apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionFacts {
    pub account_active: bool,
    /// Automated source and the account's automation opt-in is not accepted.
    pub automation_not_accepted: bool,
    pub signal_types: SignalTypes,
    pub out_of_sync: Option<bool>,
    pub risk_blocked: Option<bool>,
    /// The command could not be delivered before its TTL (ADR-0004).
    pub expired: bool,
    /// ADR-0006 §3.3; `None` until the gate ships with the risk consumer (S13).
    pub limits_pending: Option<bool>,
    /// The account has a stop/target profile for the instrument (S6, Q43); `None` until wired.
    pub has_bracket_profile: Option<bool>,
    pub multiplier: u32,
    pub max_position: u32,
    /// Signed current position in the contract (the Core's expected state).
    pub position: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Routed,
    Capped,
    Skipped,
    Rejected,
    Expired,
}

impl Outcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Routed => "routed",
            Self::Capped => "capped",
            Self::Skipped => "skipped",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }
}

/// One step of the account's plan, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Flatten the whole position and cancel its stops/targets.
    Flatten,
    /// Open/add `quantity` contracts with the profile's stop/target.
    Place { side: Side, quantity: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub outcome: Outcome,
    /// `None` when routed in full.
    pub reason_code: Option<&'static str>,
    /// `quantity × multiplier`, when the action has a quantity.
    pub requested_qty: Option<u32>,
    /// After the cap; 0 when nothing is placed.
    pub executed_qty: Option<u32>,
    pub steps: Vec<Step>,
}

impl Decision {
    fn not_executed(outcome: Outcome, code: &'static str, requested: Option<u32>) -> Self {
        Self {
            outcome,
            reason_code: Some(code),
            requested_qty: requested,
            executed_qty: requested.map(|_| 0),
            steps: Vec::new(),
        }
    }
}

/// Decides what one account does with one instruction of `signal_type`.
pub fn decide(signal_type: RouteSignalType, action: Action, f: &DecisionFacts) -> Decision {
    let requested = match action {
        Action::Open { quantity, .. } | Action::Reverse { quantity, .. } => {
            Some(quantity.saturating_mul(f.multiplier))
        }
        Action::Close => None,
    };
    let skip = |code| Decision::not_executed(Outcome::Skipped, code, requested);
    let increasing = action.increases_position();

    // Gate order: ADR-0003 §4.1.
    if !f.account_active {
        return skip("routing.account_inactive");
    }
    if f.automation_not_accepted {
        return skip("routing.automation_not_accepted");
    }
    if !f.signal_types.contains(signal_type) {
        return skip("routing.signal_type_not_selected");
    }
    if increasing && f.out_of_sync == Some(true) {
        return skip("routing.out_of_sync");
    }
    if increasing && f.risk_blocked == Some(true) {
        return skip("routing.risk_blocked");
    }
    if f.expired {
        return Decision::not_executed(Outcome::Expired, "routing.expired", requested);
    }
    if increasing && f.limits_pending == Some(true) {
        return skip("routing.limits_pending");
    }
    if increasing && f.has_bracket_profile == Some(false) {
        return skip("routing.no_bracket_profile");
    }

    // Actions: ADR-0003 §4.2.
    let max = i64::from(f.max_position);
    match action {
        Action::Close => {
            if f.position == 0 {
                skip("routing.nothing_to_close")
            } else {
                Decision {
                    outcome: Outcome::Routed,
                    reason_code: None,
                    requested_qty: None,
                    executed_qty: None,
                    steps: vec![Step::Flatten],
                }
            }
        }
        Action::Open { side, .. } => {
            let q = i64::from(requested.unwrap_or(0));
            if f.position.signum() == -side.sign() {
                return Decision::not_executed(
                    Outcome::Rejected,
                    "routing.position_conflict",
                    requested,
                );
            }
            let room = (max - f.position.abs()).max(0);
            if room == 0 {
                return skip("routing.max_position_reached");
            }
            place(side, q, room, requested, None)
        }
        Action::Reverse { side, .. } => {
            let q = i64::from(requested.unwrap_or(0));
            if f.position.signum() == side.sign() {
                return Decision::not_executed(
                    Outcome::Rejected,
                    "routing.position_conflict",
                    requested,
                );
            }
            let flatten = (f.position != 0).then_some(Step::Flatten);
            if max == 0 {
                // Nothing can be opened; still flatten what is there (an exit).
                return match flatten {
                    Some(step) => Decision {
                        outcome: Outcome::Capped,
                        reason_code: Some("routing.max_position_reached"),
                        requested_qty: requested,
                        executed_qty: Some(0),
                        steps: vec![step],
                    },
                    None => skip("routing.max_position_reached"),
                };
            }
            place(side, q, max, requested, flatten)
        }
    }
}

fn place(side: Side, q: i64, room: i64, requested: Option<u32>, first: Option<Step>) -> Decision {
    let executed = q.min(room);
    let quantity = u32::try_from(executed).unwrap_or(0);
    let capped = executed < q;
    let mut steps: Vec<Step> = first.into_iter().collect();
    steps.push(Step::Place { side, quantity });
    Decision {
        outcome: if capped {
            Outcome::Capped
        } else {
            Outcome::Routed
        },
        reason_code: capped.then_some("routing.quantity_capped"),
        requested_qty: requested,
        executed_qty: Some(quantity),
        steps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts() -> DecisionFacts {
        DecisionFacts {
            account_active: true,
            automation_not_accepted: false,
            signal_types: SignalTypes::new([RouteSignalType::Trigger]).unwrap(),
            out_of_sync: Some(false),
            risk_blocked: Some(false),
            expired: false,
            limits_pending: Some(false),
            has_bracket_profile: Some(true),
            multiplier: 2,
            max_position: 5,
            position: 0,
        }
    }

    const BUY_2: Action = Action::Open {
        side: Side::Long,
        quantity: 2,
    };
    const T: RouteSignalType = RouteSignalType::Trigger;

    #[test]
    fn quantity_is_multiplied() {
        let d = decide(T, BUY_2, &facts());
        assert_eq!(d.outcome, Outcome::Routed);
        assert_eq!((d.requested_qty, d.executed_qty), (Some(4), Some(4)));
        assert_eq!(
            d.steps,
            [Step::Place {
                side: Side::Long,
                quantity: 4
            }]
        );
    }

    #[test]
    fn excess_over_the_maximum_is_capped_and_a_full_account_gets_nothing() {
        let mut f = facts();
        f.position = 3;
        let d = decide(T, BUY_2, &f);
        assert_eq!(d.outcome, Outcome::Capped);
        assert_eq!(d.reason_code, Some("routing.quantity_capped"));
        assert_eq!((d.requested_qty, d.executed_qty), (Some(4), Some(2)));
        f.position = 5;
        let d = decide(T, BUY_2, &f);
        assert_eq!(d.outcome, Outcome::Skipped);
        assert_eq!(d.reason_code, Some("routing.max_position_reached"));
        assert_eq!(d.executed_qty, Some(0));
        assert!(d.steps.is_empty());
    }

    #[test]
    fn opposite_open_is_a_position_conflict() {
        let mut f = facts();
        f.position = -1;
        let d = decide(T, BUY_2, &f);
        assert_eq!(d.outcome, Outcome::Rejected);
        assert_eq!(d.reason_code, Some("routing.position_conflict"));
    }

    #[test]
    fn unselected_signal_type_is_skipped() {
        let d = decide(RouteSignalType::TradePlan, BUY_2, &facts());
        assert_eq!(d.reason_code, Some("routing.signal_type_not_selected"));
    }

    #[test]
    fn entry_gates_never_block_a_close() {
        let mut f = facts();
        f.position = 3;
        f.out_of_sync = Some(true);
        f.risk_blocked = Some(true);
        f.limits_pending = Some(true);
        f.has_bracket_profile = Some(false);
        let d = decide(T, Action::Close, &f);
        assert_eq!(d.outcome, Outcome::Routed);
        assert_eq!(d.steps, [Step::Flatten]);
        for (gate, code) in [
            (0, "routing.out_of_sync"),
            (1, "routing.risk_blocked"),
            (2, "routing.limits_pending"),
            (3, "routing.no_bracket_profile"),
        ] {
            let mut g = facts();
            match gate {
                0 => g.out_of_sync = Some(true),
                1 => g.risk_blocked = Some(true),
                2 => g.limits_pending = Some(true),
                _ => g.has_bracket_profile = Some(false),
            }
            assert_eq!(decide(T, BUY_2, &g).reason_code, Some(code));
        }
    }

    #[test]
    fn inactive_account_and_declined_automation_block_everything() {
        let mut f = facts();
        f.position = 1;
        f.account_active = false;
        assert_eq!(
            decide(T, Action::Close, &f).reason_code,
            Some("routing.account_inactive")
        );
        f.account_active = true;
        f.automation_not_accepted = true;
        assert_eq!(
            decide(T, Action::Close, &f).reason_code,
            Some("routing.automation_not_accepted")
        );
    }

    #[test]
    fn gate_order_follows_adr_0003() {
        let mut f = facts();
        f.account_active = false;
        f.automation_not_accepted = true;
        f.out_of_sync = Some(true);
        f.expired = true;
        assert_eq!(
            decide(T, BUY_2, &f).reason_code,
            Some("routing.account_inactive")
        );
        f.account_active = true;
        assert_eq!(
            decide(T, BUY_2, &f).reason_code,
            Some("routing.automation_not_accepted")
        );
        f.automation_not_accepted = false;
        assert_eq!(
            decide(T, BUY_2, &f).reason_code,
            Some("routing.out_of_sync")
        );
        f.out_of_sync = Some(false);
        let d = decide(T, BUY_2, &f);
        assert_eq!(
            (d.outcome, d.reason_code),
            (Outcome::Expired, Some("routing.expired"))
        );
    }

    #[test]
    fn unwired_gates_do_not_apply() {
        let mut f = facts();
        f.out_of_sync = None;
        f.risk_blocked = None;
        f.limits_pending = None;
        f.has_bracket_profile = None;
        assert_eq!(decide(T, BUY_2, &f).outcome, Outcome::Routed);
    }

    #[test]
    fn close_on_flat_is_nothing_to_close() {
        let d = decide(T, Action::Close, &facts());
        assert_eq!(d.reason_code, Some("routing.nothing_to_close"));
        assert_eq!(d.requested_qty, None);
    }

    #[test]
    fn reverse_flattens_then_opens_up_to_the_maximum() {
        let mut f = facts();
        f.position = -4;
        let rev = Action::Reverse {
            side: Side::Long,
            quantity: 3,
        };
        let d = decide(T, rev, &f);
        assert_eq!(d.outcome, Outcome::Capped);
        assert_eq!(
            d.steps,
            [
                Step::Flatten,
                Step::Place {
                    side: Side::Long,
                    quantity: 5
                }
            ]
        );
        f.position = 0;
        assert_eq!(
            decide(T, rev, &f).steps,
            [Step::Place {
                side: Side::Long,
                quantity: 5
            }]
        );
        f.position = 2;
        assert_eq!(
            decide(T, rev, &f).reason_code,
            Some("routing.position_conflict")
        );
    }
}
