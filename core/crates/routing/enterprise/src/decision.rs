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

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Long => "long",
            Self::Short => "short",
        }
    }

    /// `long` / `short` / `flat` of a signed position (audit `position_side`).
    pub const fn of_position(position: i64) -> &'static str {
        match position.signum() {
            1 => "long",
            -1 => "short",
            _ => "flat",
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

    /// The V1 action of a stored instruction (instruction v1: `action`, `position_effect`,
    /// `side`, `quantity`). `None` for what this engine does not route (`reduce`, `modify`,
    /// `cancel`) or for an instruction that breaks the schema's conditionals.
    pub fn from_instruction(
        action: &str,
        position_effect: Option<&str>,
        side: Option<&str>,
        quantity: Option<i32>,
    ) -> Option<Self> {
        let quantity = quantity
            .and_then(|q| u32::try_from(q).ok())
            .filter(|q| *q > 0);
        match (action, position_effect, side) {
            ("buy", None | Some("open"), None) => Some(Self::Open {
                side: Side::Long,
                quantity: quantity?,
            }),
            ("sell", None | Some("open"), None) => Some(Self::Open {
                side: Side::Short,
                quantity: quantity?,
            }),
            ("close", None, None) => Some(Self::Close),
            ("reverse", None, Some(side)) => Some(Self::Reverse {
                side: match side {
                    "long" => Side::Long,
                    "short" => Side::Short,
                    _ => return None,
                },
                quantity: quantity?,
            }),
            _ => None,
        }
    }
}

/// Facts for one account at the decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionFacts {
    pub account_active: bool,
    /// The account is bound to a platform account (ADR-0008: binding and venue account row).
    pub platform_bound: bool,
    /// Automated source and the account's automation opt-in is not accepted.
    pub automation_not_accepted: bool,
    pub signal_types: SignalTypes,
    /// Entry gates 6–9 always come from a real answer: there is no "not wired" value, and a
    /// port that cannot answer stops the whole decision (fail closed; Architect, 2026-10-23).
    pub out_of_sync: bool,
    pub risk_blocked: bool,
    /// The command could not be delivered before its TTL (ADR-0004).
    pub expired: bool,
    /// ADR-0006 §3.3.
    pub limits_pending: bool,
    /// The account has a stop/target profile for the instrument (S6, Q43).
    pub has_bracket_profile: bool,
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
    /// Not sent by a protective gate (HTTP 5.1.1 `AuditOutcome`: out of sync, risk block,
    /// position conflict).
    Refused,
    Expired,
}

impl Outcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Routed => "routed",
            Self::Capped => "capped",
            Self::Skipped => "skipped",
            Self::Refused => "refused",
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
    /// Q75 (ADR-0003 r4): a flatten sent while the expected position is flat, because the
    /// account is out of sync and its flat view is not trusted for exits (audit
    /// `expected_flat`; the gateway notifies `routing.out_of_sync_flatten` when it ends).
    pub expected_flat: bool,
}

impl Decision {
    /// Notification of a `reverse` that ran only its close leg (HTTP 5.1.0, registry
    /// `routing.reverse_close_only`, params `reason_code` = the gate's code, including
    /// `routing.expired` for the routing TTL gate, and `reverse_closed_qty`). One per occurrence.
    pub const REVERSE_CLOSE_ONLY: &'static str = "routing.reverse_close_only";

    /// The notification this decision raises on its own, if any.
    pub fn notification_code(&self) -> Option<&'static str> {
        self.reverse_closed_qty.map(|_| Self::REVERSE_CLOSE_ONLY)
    }

    fn not_executed(outcome: Outcome, code: &'static str, requested: Option<u32>) -> Self {
        Self {
            outcome,
            reason_code: Some(code),
            requested_qty: requested,
            executed_qty: requested.map(|_| 0),
            reverse_closed_qty: None,
            expected_flat: false,
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
    let skip = |code| Decision::not_executed(not_sent(code), code, requested);
    let conflict = || skip("routing.position_conflict");

    if let Some(code) = common_gate(signal_type, f) {
        return skip(code);
    }

    let max = i64::from(f.max_position);
    match action {
        // An exit: no entry gate and no expiry (P1; ADR-0004 T1).
        Action::Close => {
            if f.position == 0 {
                // Q75: a flat view is trusted for exits only in sync (this reads gate 6).
                if !f.out_of_sync {
                    return skip("routing.nothing_to_close");
                }
                return Decision {
                    outcome: Outcome::Routed,
                    reason_code: None,
                    requested_qty: None,
                    executed_qty: None,
                    reverse_closed_qty: None,
                    steps: vec![Step::Flatten],
                    expected_flat: true,
                };
            }
            Decision {
                outcome: Outcome::Routed,
                reason_code: None,
                requested_qty: None,
                executed_qty: None,
                reverse_closed_qty: None,
                expected_flat: false,
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

/// Step 1 of a `reverse` decided in two legs (ADR-0003 §4.1 r4; Architect, 2026-10-23): an
/// exit never waits for the entry gates, so against an opposite position the close leg
/// (flatten + cancel protections) is decided at once from gates 1–5 and the position alone.
/// `None` when there is no such leg (flat, already on the reverse side, or a gate for every
/// action fails): the whole reverse then waits for the entry gates. Never reads gates 6–9.
pub fn reverse_close_leg(
    signal_type: RouteSignalType,
    action: Action,
    f: &DecisionFacts,
) -> Option<Decision> {
    let Action::Reverse { side, quantity } = action else {
        return None;
    };
    if common_gate(signal_type, f).is_some() || f.position.signum() != -side.sign() {
        return None;
    }
    Some(Decision {
        outcome: Outcome::Routed,
        reason_code: None,
        requested_qty: Some(quantity.saturating_mul(f.multiplier)),
        executed_qty: None,
        reverse_closed_qty: None,
        expected_flat: false,
        steps: vec![Step::Flatten],
    })
}

/// Step 2: the opening of a `reverse` whose close leg was decided in step 1, with the position
/// read then (`f.position`). The same rules as [`decide`], without the flatten that already
/// left: it opens (after the flatten) or closes only with the barring code.
pub fn reverse_open_leg(
    signal_type: RouteSignalType,
    action: Action,
    f: &DecisionFacts,
) -> Decision {
    let mut d = decide(signal_type, action, f);
    d.steps.retain(|s| *s != Step::Flatten);
    d
}

/// An entry whose TTL passed before gates 6–9 answered (Architect, N1): only what does not
/// depend on those gates is named — gates 1–5, `position_conflict` — and otherwise it is
/// `routing.expired` (a reverse against an opposite position: close only). It never names a
/// gate that did not answer and never places. Exits never get here (they do not read 6–9).
pub fn decide_expired_unanswered(
    signal_type: RouteSignalType,
    action: Action,
    f: &DecisionFacts,
) -> Decision {
    let requested = match action {
        Action::Open { quantity, .. } | Action::Reverse { quantity, .. } => {
            Some(quantity.saturating_mul(f.multiplier))
        }
        Action::Close => None,
    };
    let not = |code| Decision::not_executed(not_sent(code), code, requested);
    if let Some(code) = common_gate(signal_type, f) {
        return not(code);
    }
    match action {
        Action::Close => decide(signal_type, action, f),
        Action::Open { side, .. } if f.position.signum() == -side.sign() => {
            not("routing.position_conflict")
        }
        Action::Reverse { side, .. } if f.position.signum() == side.sign() => {
            not("routing.position_conflict")
        }
        Action::Open { .. } => not("routing.expired"),
        Action::Reverse { .. } => close_only(f, "routing.expired", requested),
    }
}

/// Gates for every action (1–5), in ADR-0003 §4.1 order.
fn common_gate(signal_type: RouteSignalType, f: &DecisionFacts) -> Option<&'static str> {
    if !f.account_active {
        Some("routing.account_inactive")
    } else if !f.platform_bound {
        Some("routing.platform_unbound")
    } else if f.automation_not_accepted {
        Some("routing.automation_not_accepted")
    } else if !f.signal_types.contains(signal_type) {
        Some("routing.signal_type_not_selected")
    } else {
        None
    }
}

/// The first failing entry-only gate before the action rules.
fn entry_gate(f: &DecisionFacts) -> Option<&'static str> {
    if f.out_of_sync {
        Some("routing.out_of_sync")
    } else if f.risk_blocked {
        Some("routing.risk_blocked")
    } else if f.limits_pending {
        Some("routing.limits_pending")
    } else if !f.has_bracket_profile {
        Some("routing.no_bracket_profile")
    } else {
        None
    }
}

/// The outcome of an instruction not sent to the account (HTTP 5.1.1 `AuditOutcome`): a
/// protective gate refuses, an expired entry expires, every configuration rule skips.
fn not_sent(code: &'static str) -> Outcome {
    match code {
        "routing.out_of_sync" | "routing.risk_blocked" | "routing.position_conflict" => {
            Outcome::Refused
        }
        "routing.expired" => Outcome::Expired,
        _ => Outcome::Skipped,
    }
}

/// Q74 (b), the only place of the rule: a barred `reverse` runs only its close leg when the
/// position is opposite (decision `capped`, audit `partial`), and is skipped when flat.
fn close_only(f: &DecisionFacts, code: &'static str, requested: Option<u32>) -> Decision {
    if f.position == 0 {
        // Q75: out of sync, the flat view is not trusted: the closing leg is sent as a flatten
        // and the opening is never created (closed quantity unknown until the flatten ends).
        if code == "routing.out_of_sync" {
            return Decision {
                outcome: Outcome::Capped,
                reason_code: Some(code),
                requested_qty: requested,
                executed_qty: Some(0),
                reverse_closed_qty: None,
                steps: vec![Step::Flatten],
                expected_flat: true,
            };
        }
        return Decision::not_executed(not_sent(code), code, requested);
    }
    Decision {
        outcome: Outcome::Capped,
        reason_code: Some(code),
        requested_qty: requested,
        executed_qty: Some(0),
        reverse_closed_qty: Some(u32::try_from(f.position.unsigned_abs()).unwrap_or(u32::MAX)),
        expected_flat: false,
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
        expected_flat: false,
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
            out_of_sync: false,
            risk_blocked: false,
            expired: false,
            limits_pending: false,
            has_bracket_profile: true,
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
    fn instructions_map_to_v1_actions() {
        let f = Action::from_instruction;
        assert_eq!(f("buy", None, None, Some(2)), Some(BUY_2));
        assert_eq!(
            f("sell", Some("open"), None, Some(1)),
            Some(Action::Open {
                side: Side::Short,
                quantity: 1
            })
        );
        assert_eq!(f("close", None, None, Some(3)), Some(Action::Close));
        assert_eq!(
            f("reverse", None, Some("short"), Some(1)),
            Some(Action::Reverse {
                side: Side::Short,
                quantity: 1
            })
        );
        for (a, e, s, q) in [
            ("buy", Some("reduce"), None, Some(1)),
            ("buy", None, None, None),
            ("buy", None, None, Some(0)),
            ("reverse", None, None, Some(1)),
            ("reverse", None, Some("flat"), Some(1)),
            ("modify", None, None, None),
            ("cancel", None, None, None),
        ] {
            assert_eq!(f(a, e, s, q), None, "{a} {e:?} {s:?} {q:?}");
        }
        assert_eq!(
            [
                Side::of_position(3),
                Side::of_position(-1),
                Side::of_position(0)
            ],
            ["long", "short", "flat"]
        );
    }

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
        assert_eq!(d.outcome, Outcome::Refused);
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
        f.out_of_sync = true;
        f.risk_blocked = true;
        f.limits_pending = true;
        f.has_bracket_profile = false;
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
                0 => g.out_of_sync = true,
                1 => g.risk_blocked = true,
                2 => g.limits_pending = true,
                _ => g.has_bracket_profile = false,
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
        f.out_of_sync = true;
        f.risk_blocked = true;
        f.limits_pending = true;
        f.has_bracket_profile = false;
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
            assert_eq!(d.outcome, not_sent(code), "{code}");
            assert_eq!(
                d.outcome,
                match code {
                    "routing.out_of_sync" | "routing.risk_blocked" => Outcome::Refused,
                    "routing.expired" => Outcome::Expired,
                    _ => Outcome::Skipped,
                },
                "{code}"
            );
            match code {
                "routing.account_inactive" => f.account_active = true,
                "routing.platform_unbound" => f.platform_bound = true,
                "routing.automation_not_accepted" => f.automation_not_accepted = false,
                "routing.out_of_sync" => f.out_of_sync = false,
                "routing.risk_blocked" => f.risk_blocked = false,
                "routing.limits_pending" => f.limits_pending = false,
                "routing.no_bracket_profile" => f.has_bracket_profile = true,
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
            ("routing.out_of_sync", |f| f.out_of_sync = true),
            ("routing.risk_blocked", |f| f.risk_blocked = true),
            ("routing.limits_pending", |f| f.limits_pending = true),
            ("routing.no_bracket_profile", |f| {
                f.has_bracket_profile = false
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
            assert_eq!(
                d.notification_code(),
                Some("routing.reverse_close_only"),
                "{code}"
            );
            // Flat: nothing to close, not sent with the code — except out of sync (Q75), where
            // the flat view is not trusted and the closing leg goes as a flatten.
            f.position = 0;
            let d = decide(T, rev, &f);
            assert_eq!(d.reason_code, Some(code), "{code}");
            if code == "routing.out_of_sync" {
                assert_eq!(d.outcome, Outcome::Capped);
                assert_eq!(d.steps, [Step::Flatten]);
                assert_eq!(d.executed_qty, Some(0));
                assert!(d.expected_flat);
            } else {
                assert_eq!(d.outcome, not_sent(code), "{code}");
                assert!(d.steps.is_empty(), "{code}");
                assert!(!d.expected_flat);
            }
            assert_eq!(d.reverse_closed_qty, None, "{code}");
            assert_eq!(d.notification_code(), None, "{code}");
            // Already on the reverse side: a conflict, never a flatten.
            f.position = 2;
            let d = decide(T, rev, &f);
            assert_eq!(
                (d.outcome, d.reason_code),
                (Outcome::Refused, Some("routing.position_conflict")),
                "{code}"
            );
            assert!(d.steps.is_empty(), "{code}");
        }
    }

    #[test]
    fn a_reverse_against_an_opposite_position_closes_first_without_entry_gates() {
        let rev = Action::Reverse {
            side: Side::Long,
            quantity: 1,
        };
        let mut f = facts();
        // Entry gates are not read for the close leg.
        f.out_of_sync = true;
        f.risk_blocked = true;
        f.position = -3;
        let close = reverse_close_leg(T, rev, &f).unwrap();
        assert_eq!(close.steps, [Step::Flatten]);
        assert_eq!(close.outcome, Outcome::Routed);
        // No close leg: flat, already long, a gate for every action, or not a reverse.
        f.position = 0;
        assert_eq!(reverse_close_leg(T, rev, &f), None);
        f.position = 2;
        assert_eq!(reverse_close_leg(T, rev, &f), None);
        f.position = -3;
        f.account_active = false;
        assert_eq!(reverse_close_leg(T, rev, &f), None);
        f.account_active = true;
        assert_eq!(reverse_close_leg(T, Action::Close, &f), None);
        assert_eq!(reverse_close_leg(T, BUY_2, &f), None);

        // Step 2 with the answers: barred → close only, nothing more to send.
        let open = reverse_open_leg(T, rev, &f);
        assert_eq!(
            (open.outcome, open.reason_code),
            (Outcome::Capped, Some("routing.out_of_sync"))
        );
        assert!(open.steps.is_empty());
        assert_eq!(open.reverse_closed_qty, Some(3));
        // Clear → only the opening is left to send.
        f.out_of_sync = false;
        f.risk_blocked = false;
        let open = reverse_open_leg(T, rev, &f);
        assert_eq!(open.outcome, Outcome::Routed);
        assert_eq!(
            open.steps,
            [Step::Place {
                side: Side::Long,
                quantity: 2
            }]
        );
        // TTL passed before the answers → expired, close only.
        f.expired = true;
        let open = reverse_open_leg(T, rev, &f);
        assert_eq!(
            (open.outcome, open.reason_code),
            (Outcome::Capped, Some("routing.expired"))
        );
        assert!(open.steps.is_empty());
    }

    /// With the TTL passed, no rule ever sends an entry, whatever the entry gates say: the use
    /// case may then decide without their answers (never execute late).
    #[test]
    fn an_expired_entry_never_places() {
        let mut f = facts();
        f.expired = true;
        for position in [-2, 0, 2] {
            f.position = position;
            for action in [
                BUY_2,
                Action::Reverse {
                    side: Side::Long,
                    quantity: 1,
                },
                Action::Reverse {
                    side: Side::Short,
                    quantity: 1,
                },
            ] {
                let d = decide(T, action, &f);
                assert!(
                    d.steps.iter().all(|s| *s == Step::Flatten),
                    "{action:?} at {position}: {d:?}"
                );
            }
        }
    }

    #[test]
    fn an_unanswered_expired_entry_never_names_a_gate() {
        let mut f = facts();
        f.expired = true;
        // Whatever the (unknown) entry gates hold, they are not read.
        f.out_of_sync = true;
        f.position = -1;
        let d = decide_expired_unanswered(T, BUY_2, &f);
        assert_eq!(
            (d.outcome, d.reason_code),
            (Outcome::Refused, Some("routing.position_conflict"))
        );
        f.position = 0;
        let d = decide_expired_unanswered(T, BUY_2, &f);
        assert_eq!(
            (d.outcome, d.reason_code),
            (Outcome::Expired, Some("routing.expired"))
        );
        let rev = Action::Reverse {
            side: Side::Long,
            quantity: 1,
        };
        f.position = -2;
        let d = decide_expired_unanswered(T, rev, &f);
        assert_eq!(
            (d.outcome, d.reason_code, d.reverse_closed_qty),
            (Outcome::Capped, Some("routing.expired"), Some(2))
        );
        assert_eq!(d.steps, [Step::Flatten]);
        f.position = 1;
        assert_eq!(
            decide_expired_unanswered(T, rev, &f).reason_code,
            Some("routing.position_conflict")
        );
        // Gates 1–5 are known and still come first.
        f.account_active = false;
        assert_eq!(
            decide_expired_unanswered(T, BUY_2, &f).reason_code,
            Some("routing.account_inactive")
        );
    }

    /// Q75 (ADR-0003 r4 §4.2): a close against a flat expected position reads gate 6 — in
    /// sync it is nothing to close, out of sync it is sent as a flatten anyway.
    #[test]
    fn a_flat_close_trusts_the_flat_view_only_in_sync() {
        let mut f = facts();
        let d = decide(T, Action::Close, &f);
        assert_eq!(d.reason_code, Some("routing.nothing_to_close"));
        assert!(d.steps.is_empty() && !d.expected_flat);
        f.out_of_sync = true;
        let d = decide(T, Action::Close, &f);
        assert_eq!((d.outcome, d.reason_code), (Outcome::Routed, None));
        assert_eq!(d.steps, [Step::Flatten]);
        assert!(d.expected_flat);
        // A non-flat close never depends on the sync state.
        f.position = 2;
        assert!(!decide(T, Action::Close, &f).expected_flat);
    }

    #[test]
    fn exits_never_expire() {
        let mut f = facts();
        f.expired = true;
        f.position = 2;
        f.out_of_sync = true;
        let d = decide(T, Action::Close, &f);
        assert_eq!((d.outcome, d.steps), (Outcome::Routed, vec![Step::Flatten]));
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
