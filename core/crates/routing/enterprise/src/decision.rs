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
    /// Q74: contracts flattened by the close leg of a `reverse` whose open leg was skipped
    /// with `reason_code` (audit `reverse_closed_qty`; notification `routing.reverse_close_only`).
    pub reverse_closed_qty: Option<u32>,
    pub steps: Vec<Step>,
}

impl Decision {
    fn not_executed(outcome: Outcome, code: &'static str, requested: Option<u32>) -> Self {
        Self {
            outcome,
            reason_code: Some(code),
            requested_qty: requested,
            executed_qty: requested.map(|_| 0),
            reverse_closed_qty: None,
            steps: Vec::new(),
        }
    }
}

/// Decides what one account does with one instruction of `signal_type` (HTTP 5.0.0; ADR-0003
/// §4.1/§4.2 with Q74 b).
///
/// The audited code is the first failing gate, in this order: account_inactive →
/// platform_unbound → automation_not_accepted → (copy_not_accepted, S19) →
/// signal_type_not_selected → out_of_sync → risk_blocked → limits_pending → no_bracket_profile
/// (these four only for position-increasing actions) → action rules → expired (entries only:
/// an exit never expires; it waits and executes late, audited as late).
///
/// A `reverse` checks the position first: already on the reverse side → `position_conflict`
/// (never a flatten). When its open leg is barred (an entry gate, no room, or expired), an
/// opposite position runs only the close leg (`reverse_close_only`) and a flat one is skipped
/// with the code. A reverse never opens on a barred account.
pub fn decide(signal_type: RouteSignalType, action: Action, f: &DecisionFacts) -> Decision {
    let requested = match action {
        Action::Open { quantity, .. } | Action::Reverse { quantity, .. } => {
            Some(quantity.saturating_mul(f.multiplier))
        }
        Action::Close => None,
    };
    let skip = |code| Decision::not_executed(Outcome::Skipped, code, requested);
    let conflict =
        || Decision::not_executed(Outcome::Rejected, "routing.position_conflict", requested);

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

    let max = i64::from(f.max_position);
    match action {
        // An exit: no entry gate and no expiry (P1; ADR-0004 T1).
        Action::Close => {
            if f.position == 0 {
                return skip("routing.nothing_to_close");
            }
            Decision {
                outcome: Outcome::Routed,
                reason_code: None,
                requested_qty: None,
                executed_qty: None,
                reverse_closed_qty: None,
                steps: vec![Step::Flatten],
            }
        }
        Action::Open { side, .. } => {
            if let Some(code) = entry_gate(f) {
                return skip(code);
            }
            if f.position.signum() == -side.sign() {
                return conflict();
            }
            let room = (max - f.position.abs()).max(0);
            if room == 0 {
                return skip("routing.max_position_reached");
            }
            if f.expired {
                return Decision::not_executed(Outcome::Expired, "routing.expired", requested);
            }
            place(
                side,
                i64::from(requested.unwrap_or(0)),
                room,
                requested,
                None,
            )
        }
        Action::Reverse { side, .. } => {
            // §4.2 position check first.
            if f.position.signum() == side.sign() {
                return conflict();
            }
            let barred = entry_gate(f)
                .or((max == 0).then_some("routing.max_position_reached"))
                .or(f.expired.then_some("routing.expired"));
            match barred {
                Some(code) => close_only(f, code, requested),
                None => {
                    let flatten = (f.position != 0).then_some(Step::Flatten);
                    place(
                        side,
                        i64::from(requested.unwrap_or(0)),
                        max,
                        requested,
                        flatten,
                    )
                }
            }
        }
    }
}

/// The first failing entry-only gate before the action rules. Unwired gates (`None`) do not
/// apply.
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

/// Q74 (b), the only place of the rule: a barred `reverse` runs only its close leg when the
/// position is opposite (decision `capped`, audit `partial`), and is skipped when flat.
fn close_only(f: &DecisionFacts, code: &'static str, requested: Option<u32>) -> Decision {
    if f.position == 0 {
        let outcome = if code == "routing.expired" {
            Outcome::Expired
        } else {
            Outcome::Skipped
        };
        return Decision::not_executed(outcome, code, requested);
    }
    Decision {
        outcome: Outcome::Capped,
        reason_code: Some(code),
        requested_qty: requested,
        executed_qty: Some(0),
        reverse_closed_qty: Some(u32::try_from(f.position.unsigned_abs()).unwrap_or(u32::MAX)),
        steps: vec![Step::Flatten],
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
        reverse_closed_qty: None,
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
    fn barred_reverse_closes_only_or_skips_when_flat() {
        let rev = Action::Reverse {
            side: Side::Long,
            quantity: 1,
        };
        type Bar = fn(&mut DecisionFacts);
        let bars: [(&str, Bar); 6] = [
            ("routing.out_of_sync", |f| f.out_of_sync = Some(true)),
            ("routing.risk_blocked", |f| f.risk_blocked = Some(true)),
            ("routing.limits_pending", |f| f.limits_pending = Some(true)),
            ("routing.no_bracket_profile", |f| {
                f.has_bracket_profile = Some(false)
            }),
            ("routing.max_position_reached", |f| f.max_position = 0),
            ("routing.expired", |f| f.expired = true),
        ];
        for (code, bar) in bars {
            let mut f = facts();
            bar(&mut f);
            // Opposite: the close leg runs, the open is skipped with the gate's code.
            f.position = -3;
            let d = decide(T, rev, &f);
            assert_eq!(
                (d.outcome, d.reason_code),
                (Outcome::Capped, Some(code)),
                "{code}"
            );
            assert_eq!(d.steps, [Step::Flatten], "{code}");
            assert_eq!(d.executed_qty, Some(0), "{code}");
            assert_eq!(d.reverse_closed_qty, Some(3), "{code}");
            // Flat: nothing to close, skipped with the code.
            f.position = 0;
            let d = decide(T, rev, &f);
            assert_eq!(d.reason_code, Some(code), "{code}");
            assert!(d.steps.is_empty(), "{code}");
            assert_eq!(d.reverse_closed_qty, None, "{code}");
            // Already on the reverse side: a conflict, never a flatten.
            f.position = 2;
            let d = decide(T, rev, &f);
            assert_eq!(
                (d.outcome, d.reason_code),
                (Outcome::Rejected, Some("routing.position_conflict")),
                "{code}"
            );
            assert!(d.steps.is_empty(), "{code}");
        }
    }

    #[test]
    fn exits_never_expire() {
        let mut f = facts();
        f.expired = true;
        f.position = 2;
        f.out_of_sync = Some(true);
        let d = decide(T, Action::Close, &f);
        assert_eq!((d.outcome, d.steps), (Outcome::Routed, vec![Step::Flatten]));
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
