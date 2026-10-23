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
    /// The account is bound to a platform account (ADR-0008: binding and venue account row).
    pub platform_bound: bool,
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

/// How a `reverse` is treated when an entry gate stops its opening leg. The accepted text
/// skips the whole action; the alternative (flatten, skip only the opening) is with the user
/// (Architect, 2026-10-23). This is the only place that rule lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReverseOnEntryGate {
    SkipWhole,
    FlattenOnly,
}

pub const REVERSE_ON_ENTRY_GATE: ReverseOnEntryGate = ReverseOnEntryGate::SkipWhole;

/// Decides what one account does with one instruction of `signal_type`.
///
/// The audited code is the first failing gate, in this order (Architect, 2026-10-23):
/// account_inactive → platform_unbound → automation_not_accepted → (copy_not_accepted, S19)
/// → signal_type_not_selected → out_of_sync → risk_blocked → limits_pending →
/// no_bracket_profile (these four only for position-increasing actions) → action rules
/// (ADR-0003 §4.2) → expired.
pub fn decide(signal_type: RouteSignalType, action: Action, f: &DecisionFacts) -> Decision {
    decide_with(signal_type, action, f, REVERSE_ON_ENTRY_GATE)
}

fn decide_with(
    signal_type: RouteSignalType,
    action: Action,
    f: &DecisionFacts,
    reverse_rule: ReverseOnEntryGate,
) -> Decision {
    let requested = match action {
        Action::Open { quantity, .. } | Action::Reverse { quantity, .. } => {
            Some(quantity.saturating_mul(f.multiplier))
        }
        Action::Close => None,
    };
    let skip = |code| Decision::not_executed(Outcome::Skipped, code, requested);

    // Gates for every action.
    if !f.account_active {
        return skip("routing.account_inactive");
    }
    if !f.platform_bound {
        return skip("routing.platform_unbound");
    }
    if f.automation_not_accepted {
        return skip("routing.automation_not_accepted");
    }
    if !f.signal_types.contains(signal_type) {
        return skip("routing.signal_type_not_selected");
    }

    // Gates for position-increasing actions only (P1: exits always go through).
    if action.increases_position()
        && let Some(code) = entry_gate(f)
    {
        return match (action, reverse_rule) {
            (Action::Reverse { .. }, ReverseOnEntryGate::FlattenOnly) if f.position != 0 => {
                expire_or(
                    f,
                    Decision {
                        outcome: Outcome::Capped,
                        reason_code: Some(code),
                        requested_qty: requested,
                        executed_qty: Some(0),
                        steps: vec![Step::Flatten],
                    },
                )
            }
            _ => skip(code),
        };
    }

    // Action rules: ADR-0003 §4.2; a command that would run is then subject to its TTL.
    let max = i64::from(f.max_position);
    let planned = match action {
        Action::Close => {
            if f.position == 0 {
                return skip("routing.nothing_to_close");
            }
            Decision {
                outcome: Outcome::Routed,
                reason_code: None,
                requested_qty: None,
                executed_qty: None,
                steps: vec![Step::Flatten],
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
                // Flatten what is there (an exit) and skip the opening.
                let Some(step) = flatten else {
                    return skip("routing.max_position_reached");
                };
                Decision {
                    outcome: Outcome::Capped,
                    reason_code: Some("routing.max_position_reached"),
                    requested_qty: requested,
                    executed_qty: Some(0),
                    steps: vec![step],
                }
            } else {
                place(side, q, max, requested, flatten)
            }
        }
    };
    expire_or(f, planned)
}

/// The first failing entry-only gate. A gate whose slice is not wired yet is `None`.
fn entry_gate(f: &DecisionFacts) -> Option<&'static str> {
    if f.out_of_sync == Some(true) {
        Some("routing.out_of_sync")
    } else if f.risk_blocked == Some(true) {
        Some("routing.risk_blocked")
    } else if f.limits_pending == Some(true) {
        Some("routing.limits_pending")
    } else if f.has_bracket_profile == Some(false) {
        Some("routing.no_bracket_profile")
    } else {
        None
    }
}

/// The last gate: a plan that would send commands is expired when its TTL has passed.
fn expire_or(f: &DecisionFacts, planned: Decision) -> Decision {
    if f.expired && !planned.steps.is_empty() {
        Decision::not_executed(Outcome::Expired, "routing.expired", planned.requested_qty)
    } else {
        planned
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
            platform_bound: true,
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
        f.platform_bound = false;
        f.automation_not_accepted = true;
        f.out_of_sync = Some(true);
        f.risk_blocked = Some(true);
        f.limits_pending = Some(true);
        f.has_bracket_profile = Some(false);
        f.expired = true;
        let order = [
            "routing.account_inactive",
            "routing.platform_unbound",
            "routing.automation_not_accepted",
            "routing.out_of_sync",
            "routing.risk_blocked",
            "routing.limits_pending",
            "routing.no_bracket_profile",
            "routing.expired",
        ];
        for code in order {
            let d = decide(T, BUY_2, &f);
            assert_eq!(d.reason_code, Some(code));
            match code {
                "routing.account_inactive" => f.account_active = true,
                "routing.platform_unbound" => f.platform_bound = true,
                "routing.automation_not_accepted" => f.automation_not_accepted = false,
                "routing.out_of_sync" => f.out_of_sync = Some(false),
                "routing.risk_blocked" => f.risk_blocked = Some(false),
                "routing.limits_pending" => f.limits_pending = Some(false),
                "routing.no_bracket_profile" => f.has_bracket_profile = Some(true),
                _ => assert_eq!(d.outcome, Outcome::Expired),
            }
        }
    }

    #[test]
    fn action_rules_come_before_expired() {
        let mut f = facts();
        f.expired = true;
        assert_eq!(
            decide(T, Action::Close, &f).reason_code,
            Some("routing.nothing_to_close")
        );
        f.position = -1;
        assert_eq!(
            decide(T, BUY_2, &f).reason_code,
            Some("routing.position_conflict")
        );
        let up = Action::Open {
            side: Side::Short,
            quantity: 1,
        };
        f.position = -5;
        assert_eq!(
            decide(T, up, &f).reason_code,
            Some("routing.max_position_reached")
        );
    }

    #[test]
    fn platform_unbound_blocks_exits_too() {
        let mut f = facts();
        f.position = 2;
        f.platform_bound = false;
        assert_eq!(
            decide(T, Action::Close, &f).reason_code,
            Some("routing.platform_unbound")
        );
    }

    #[test]
    fn reverse_stopped_by_an_entry_gate_follows_the_single_rule() {
        let mut f = facts();
        f.position = -3;
        f.risk_blocked = Some(true);
        let rev = Action::Reverse {
            side: Side::Long,
            quantity: 1,
        };
        assert_eq!(REVERSE_ON_ENTRY_GATE, ReverseOnEntryGate::SkipWhole);
        let d = decide(T, rev, &f);
        assert_eq!(
            (d.outcome, d.reason_code),
            (Outcome::Skipped, Some("routing.risk_blocked"))
        );
        assert!(d.steps.is_empty());
        let d = decide_with(T, rev, &f, ReverseOnEntryGate::FlattenOnly);
        assert_eq!(d.outcome, Outcome::Capped);
        assert_eq!(d.steps, [Step::Flatten]);
        assert_eq!(d.reason_code, Some("routing.risk_blocked"));
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
    fn reverse_with_no_room_flattens_and_skips_the_opening() {
        let mut f = facts();
        f.max_position = 0;
        f.position = -2;
        let rev = Action::Reverse {
            side: Side::Long,
            quantity: 1,
        };
        let d = decide(T, rev, &f);
        assert_eq!(d.outcome, Outcome::Capped);
        assert_eq!(d.reason_code, Some("routing.max_position_reached"));
        assert_eq!(d.steps, [Step::Flatten]);
        f.position = 0;
        assert_eq!(decide(T, rev, &f).outcome, Outcome::Skipped);
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
