//! QA [Backend] — S7 routing decision engine, black-box table tests over the public
//! `routing_enterprise::decision` API.
//!
//! Spec: ADR-0003 r3 §4.1 (gate order, P1, Q74 b, exits never expire) and §4.2 (action rules);
//! slice plan S7 AC-2, AC-3, AC-4, AC-8; HTTP 5.1.x (`reverse_closed_qty`,
//! `routing.reverse_close_only`). Q75 b / Q76 a cases are `#[ignore]`d until ADR-0003 r4.

#![cfg(test)]

use routing_enterprise::decision::{Action, Decision, DecisionFacts, Outcome, Side, Step, decide};
use routing_enterprise::{RouteSignalType, SignalTypes};

const SIGNAL: RouteSignalType = RouteSignalType::Trigger;

/// (side, position, quantity, multiplier, max, outcome, code, executed)
type OpenRow = (Side, i64, u32, u32, u32, Outcome, Option<&'static str>, u32);
/// (side, position, quantity, max, outcome, code, steps)
type ReverseRow = (
    Side,
    i64,
    u32,
    u32,
    Outcome,
    Option<&'static str>,
    Vec<Step>,
);

/// An account on which every gate passes: flat, multiplier 1, maximum 5.
fn clear() -> DecisionFacts {
    DecisionFacts {
        account_active: true,
        platform_bound: true,
        automation_not_accepted: false,
        signal_types: SignalTypes::new([SIGNAL]).unwrap(),
        out_of_sync: Some(false),
        risk_blocked: Some(false),
        expired: false,
        limits_pending: Some(false),
        has_bracket_profile: Some(true),
        multiplier: 1,
        max_position: 5,
        position: 0,
    }
}

/// A failing gate of ADR-0003 §4.1, in table order, with the fact change that trips it.
#[derive(Clone, Copy)]
struct Gate {
    code: &'static str,
    /// Gates 6–10 bar only position-increasing actions.
    entry_only: bool,
    trip: fn(&mut DecisionFacts),
}

/// ADR-0003 §4.1 gates in evaluation order. Gate 4 (`copy_not_accepted`) belongs to master
/// sources (S19) and is not modelled by the V1 engine; gate 10 (`expired`) is evaluated after
/// the action rules.
const GATES: [Gate; 9] = [
    Gate {
        code: "routing.account_inactive",
        entry_only: false,
        trip: |f| f.account_active = false,
    },
    Gate {
        code: "routing.platform_unbound",
        entry_only: false,
        trip: |f| f.platform_bound = false,
    },
    Gate {
        code: "routing.automation_not_accepted",
        entry_only: false,
        trip: |f| f.automation_not_accepted = true,
    },
    Gate {
        code: "routing.signal_type_not_selected",
        entry_only: false,
        trip: |f| f.signal_types = SignalTypes::new([RouteSignalType::TradePlan]).unwrap(),
    },
    Gate {
        code: "routing.out_of_sync",
        entry_only: true,
        trip: |f| f.out_of_sync = Some(true),
    },
    Gate {
        code: "routing.risk_blocked",
        entry_only: true,
        trip: |f| f.risk_blocked = Some(true),
    },
    Gate {
        code: "routing.limits_pending",
        entry_only: true,
        trip: |f| f.limits_pending = Some(true),
    },
    Gate {
        code: "routing.no_bracket_profile",
        entry_only: true,
        trip: |f| f.has_bracket_profile = Some(false),
    },
    Gate {
        code: "routing.expired",
        entry_only: true,
        trip: |f| f.expired = true,
    },
];

/// Entry gates 6–9 (Q74 b: a barred reverse closes only).
fn entry_bars() -> impl Iterator<Item = Gate> {
    GATES[4..8].iter().copied()
}

fn with(gates: &[Gate], position: i64) -> DecisionFacts {
    let mut f = clear();
    f.position = position;
    for g in gates {
        (g.trip)(&mut f);
    }
    f
}

fn open(side: Side, quantity: u32) -> Action {
    Action::Open { side, quantity }
}

fn reverse(side: Side, quantity: u32) -> Action {
    Action::Reverse { side, quantity }
}

/// Nothing is sent to the platform and nothing is notified by the decision itself.
fn assert_not_executed(d: &Decision, outcome: Outcome, code: &str, ctx: &str) {
    assert_eq!(d.outcome, outcome, "{ctx}");
    assert_eq!(d.reason_code, Some(code), "{ctx}");
    assert!(d.steps.is_empty(), "{ctx}: steps {:?}", d.steps);
    assert_eq!(d.reverse_closed_qty, None, "{ctx}");
    assert_eq!(d.notification_code(), None, "{ctx}");
    if d.requested_qty.is_some() {
        assert_eq!(d.executed_qty, Some(0), "{ctx}");
    }
}

fn assert_close_only(d: &Decision, code: &str, closed: u32, ctx: &str) {
    assert_eq!(d.outcome, Outcome::Capped, "{ctx}");
    assert_eq!(d.reason_code, Some(code), "{ctx}");
    assert_eq!(d.steps, vec![Step::Flatten], "{ctx}");
    assert_eq!(d.executed_qty, Some(0), "{ctx}");
    assert_eq!(d.reverse_closed_qty, Some(closed), "{ctx}");
    assert_eq!(
        d.notification_code(),
        Some("routing.reverse_close_only"),
        "{ctx}"
    );
}

// ---------------------------------------------------------------- §4.1 gate order

#[test]
fn baseline_open_is_routed_in_full() {
    let d = decide(SIGNAL, open(Side::Long, 2), &clear());
    assert_eq!(d.outcome, Outcome::Routed);
    assert_eq!(d.reason_code, None);
    assert_eq!(d.requested_qty, Some(2));
    assert_eq!(d.executed_qty, Some(2));
    assert_eq!(
        d.steps,
        vec![Step::Place {
            side: Side::Long,
            quantity: 2
        }]
    );
    assert_eq!(d.notification_code(), None);
}

#[test]
fn each_gate_alone_bars_an_entry_with_its_code() {
    for g in GATES {
        let d = decide(SIGNAL, open(Side::Long, 1), &with(&[g], 0));
        let outcome = if g.code == "routing.expired" {
            Outcome::Expired
        } else {
            Outcome::Skipped
        };
        assert_not_executed(&d, outcome, g.code, g.code);
    }
}

#[test]
fn first_failing_gate_wins_for_every_pair() {
    for (i, first) in GATES.iter().enumerate() {
        for later in &GATES[i + 1..] {
            let ctx = format!("{} + {}", first.code, later.code);
            for side in [Side::Long, Side::Short] {
                let d = decide(SIGNAL, open(side, 1), &with(&[*first, *later], 0));
                assert_eq!(d.reason_code, Some(first.code), "open {side:?}: {ctx}");
                assert!(d.steps.is_empty(), "open {side:?}: {ctx}");
            }
        }
    }
}

#[test]
fn all_gates_failing_reports_account_inactive() {
    let d = decide(SIGNAL, open(Side::Short, 1), &with(&GATES, 0));
    assert_not_executed(
        &d,
        Outcome::Skipped,
        "routing.account_inactive",
        "all gates",
    );
}

#[test]
fn unwired_entry_gates_do_not_apply() {
    let mut f = clear();
    f.out_of_sync = None;
    f.risk_blocked = None;
    f.limits_pending = None;
    f.has_bracket_profile = None;
    let d = decide(SIGNAL, open(Side::Long, 1), &f);
    assert_eq!(d.outcome, Outcome::Routed);
}

#[test]
fn signal_type_filter_per_route_ac2() {
    let mut f = clear();
    f.signal_types =
        SignalTypes::new([RouteSignalType::TradePlan, RouteSignalType::NqTrigger]).unwrap();
    for t in [RouteSignalType::TradePlan, RouteSignalType::NqTrigger] {
        assert_eq!(
            decide(t, open(Side::Long, 1), &f).outcome,
            Outcome::Routed,
            "{t:?}"
        );
    }
    let d = decide(RouteSignalType::Trigger, open(Side::Long, 1), &f);
    assert_not_executed(
        &d,
        Outcome::Skipped,
        "routing.signal_type_not_selected",
        "trigger",
    );
}

// ---------------------------------------------------------------- P1: exits

#[test]
fn close_is_blocked_only_by_gates_for_all_actions() {
    for g in GATES {
        for position in [3, -3] {
            let d = decide(SIGNAL, Action::Close, &with(&[g], position));
            let ctx = format!("close P={position} under {}", g.code);
            if g.entry_only {
                assert_eq!(d.outcome, Outcome::Routed, "{ctx}");
                assert_eq!(d.reason_code, None, "{ctx}");
                assert_eq!(d.steps, vec![Step::Flatten], "{ctx}");
            } else {
                assert_not_executed(&d, Outcome::Skipped, g.code, &ctx);
            }
        }
    }
}

#[test]
fn close_under_every_entry_gate_and_expired_still_flattens() {
    let entry: Vec<Gate> = GATES.iter().copied().filter(|g| g.entry_only).collect();
    let d = decide(SIGNAL, Action::Close, &with(&entry, -2));
    assert_eq!(d.outcome, Outcome::Routed);
    assert_eq!(d.steps, vec![Step::Flatten]);
}

#[test]
fn close_carries_no_quantity_and_ignores_the_cap() {
    let mut f = with(&[], 4);
    f.max_position = 0;
    f.multiplier = 3;
    let d = decide(SIGNAL, Action::Close, &f);
    assert_eq!(d.outcome, Outcome::Routed);
    assert_eq!(d.requested_qty, None);
    assert_eq!(d.executed_qty, None);
    assert_eq!(d.steps, vec![Step::Flatten]);
}

#[test]
fn close_when_flat_is_nothing_to_close() {
    let d = decide(SIGNAL, Action::Close, &clear());
    assert_not_executed(
        &d,
        Outcome::Skipped,
        "routing.nothing_to_close",
        "close flat",
    );
}

// ---------------------------------------------------------------- §4.2 open, AC-3, AC-4

#[test]
fn executed_quantity_is_quantity_times_multiplier_ac3() {
    let mut f = clear();
    f.multiplier = 3;
    f.max_position = 100;
    let d = decide(SIGNAL, open(Side::Short, 4), &f);
    assert_eq!(d.requested_qty, Some(12));
    assert_eq!(d.executed_qty, Some(12));
    assert_eq!(
        d.steps,
        vec![Step::Place {
            side: Side::Short,
            quantity: 12
        }]
    );
}

#[test]
fn open_cap_table_ac4() {
    // (side, position, quantity, multiplier, max, outcome, code, executed)
    let rows: [OpenRow; 10] = [
        (Side::Long, 0, 5, 1, 5, Outcome::Routed, None, 5),
        (
            Side::Long,
            0,
            6,
            1,
            5,
            Outcome::Capped,
            Some("routing.quantity_capped"),
            5,
        ),
        (
            Side::Long,
            0,
            2,
            3,
            5,
            Outcome::Capped,
            Some("routing.quantity_capped"),
            5,
        ),
        (
            Side::Long,
            3,
            4,
            1,
            5,
            Outcome::Capped,
            Some("routing.quantity_capped"),
            2,
        ),
        (Side::Short, -3, 2, 1, 5, Outcome::Routed, None, 2),
        (
            Side::Short,
            -3,
            4,
            1,
            5,
            Outcome::Capped,
            Some("routing.quantity_capped"),
            2,
        ),
        (
            Side::Long,
            5,
            1,
            1,
            5,
            Outcome::Skipped,
            Some("routing.max_position_reached"),
            0,
        ),
        (
            Side::Short,
            -5,
            1,
            1,
            5,
            Outcome::Skipped,
            Some("routing.max_position_reached"),
            0,
        ),
        // Already above the maximum (e.g. the maximum was lowered): no room, never negative.
        (
            Side::Long,
            7,
            1,
            1,
            5,
            Outcome::Skipped,
            Some("routing.max_position_reached"),
            0,
        ),
        (
            Side::Long,
            0,
            1,
            1,
            0,
            Outcome::Skipped,
            Some("routing.max_position_reached"),
            0,
        ),
    ];
    for (side, position, q, mult, max, outcome, code, executed) in rows {
        let ctx = format!("{side:?} P={position} Q={q}x{mult} M={max}");
        let mut f = with(&[], position);
        f.multiplier = mult;
        f.max_position = max;
        let d = decide(SIGNAL, open(side, q), &f);
        assert_eq!(d.outcome, outcome, "{ctx}");
        assert_eq!(d.reason_code, code, "{ctx}");
        assert_eq!(d.requested_qty, Some(q * mult), "{ctx}");
        assert_eq!(d.executed_qty, Some(executed), "{ctx}");
        if executed == 0 {
            assert!(d.steps.is_empty(), "{ctx}");
        } else {
            assert_eq!(
                d.steps,
                vec![Step::Place {
                    side,
                    quantity: executed
                }],
                "{ctx}"
            );
        }
    }
}

#[test]
fn open_opposite_to_position_is_rejected() {
    for (side, position) in [(Side::Long, -1), (Side::Short, 2)] {
        let d = decide(SIGNAL, open(side, 1), &with(&[], position));
        assert_not_executed(
            &d,
            Outcome::Rejected,
            "routing.position_conflict",
            "open opposite",
        );
    }
}

#[test]
fn entry_gates_precede_the_action_rules_for_open() {
    for g in entry_bars() {
        let d = decide(SIGNAL, open(Side::Long, 1), &with(&[g], -2));
        assert_eq!(
            d.reason_code,
            Some(g.code),
            "{} before position_conflict",
            g.code
        );
    }
}

#[test]
fn ttl_is_checked_after_the_action_rules() {
    let expired = GATES[8];
    // Opposite position: position_conflict wins over expired.
    let d = decide(SIGNAL, open(Side::Long, 1), &with(&[expired], -1));
    assert_not_executed(
        &d,
        Outcome::Rejected,
        "routing.position_conflict",
        "conflict+expired",
    );
    // No room: max_position_reached wins over expired.
    let d = decide(SIGNAL, open(Side::Long, 1), &with(&[expired], 5));
    assert_not_executed(
        &d,
        Outcome::Skipped,
        "routing.max_position_reached",
        "full+expired",
    );
    // Room: expired, nothing placed even when the quantity would be capped.
    let d = decide(SIGNAL, open(Side::Long, 9), &with(&[expired], 1));
    assert_not_executed(&d, Outcome::Expired, "routing.expired", "capped+expired");
}

// ---------------------------------------------------------------- §4.2 reverse

#[test]
fn reverse_unbarred_table() {
    // (side, position, quantity, max, outcome, code, steps)
    let rows: [ReverseRow; 6] = [
        (
            Side::Long,
            -3,
            2,
            5,
            Outcome::Routed,
            None,
            vec![
                Step::Flatten,
                Step::Place {
                    side: Side::Long,
                    quantity: 2,
                },
            ],
        ),
        // The open is min(Q, M), not M − |P|: the close leg flattened first.
        (
            Side::Short,
            4,
            5,
            5,
            Outcome::Routed,
            None,
            vec![
                Step::Flatten,
                Step::Place {
                    side: Side::Short,
                    quantity: 5,
                },
            ],
        ),
        (
            Side::Long,
            -3,
            7,
            5,
            Outcome::Capped,
            Some("routing.quantity_capped"),
            vec![
                Step::Flatten,
                Step::Place {
                    side: Side::Long,
                    quantity: 5,
                },
            ],
        ),
        (
            Side::Long,
            0,
            2,
            5,
            Outcome::Routed,
            None,
            vec![Step::Place {
                side: Side::Long,
                quantity: 2,
            }],
        ),
        (
            Side::Short,
            0,
            9,
            5,
            Outcome::Capped,
            Some("routing.quantity_capped"),
            vec![Step::Place {
                side: Side::Short,
                quantity: 5,
            }],
        ),
        (
            Side::Long,
            2,
            1,
            5,
            Outcome::Rejected,
            Some("routing.position_conflict"),
            vec![],
        ),
    ];
    for (side, position, q, max, outcome, code, steps) in rows {
        let ctx = format!("reverse {side:?} P={position} Q={q} M={max}");
        let mut f = with(&[], position);
        f.max_position = max;
        let d = decide(SIGNAL, reverse(side, q), &f);
        assert_eq!(d.outcome, outcome, "{ctx}");
        assert_eq!(d.reason_code, code, "{ctx}");
        assert_eq!(d.steps, steps, "{ctx}");
        assert_eq!(d.reverse_closed_qty, None, "{ctx}");
        assert_eq!(d.notification_code(), None, "{ctx}");
    }
}

#[test]
fn reverse_with_max_zero_closes_only() {
    let mut f = with(&[], -3);
    f.max_position = 0;
    let d = decide(SIGNAL, reverse(Side::Long, 1), &f);
    assert_close_only(&d, "routing.max_position_reached", 3, "M=0 opposite");

    f.position = 0;
    let d = decide(SIGNAL, reverse(Side::Long, 1), &f);
    assert_not_executed(
        &d,
        Outcome::Skipped,
        "routing.max_position_reached",
        "M=0 flat",
    );
}

/// ADR-0003 §4.1 Q74 b: the three positions for each of the four entry gates, both sides.
#[test]
fn q74_barred_reverse_matrix() {
    for g in entry_bars() {
        for side in [Side::Long, Side::Short] {
            let opposite = if side == Side::Long { -3 } else { 3 };
            let ctx = format!("{} reverse {side:?}", g.code);

            let d = decide(SIGNAL, reverse(side, 2), &with(&[g], opposite));
            assert_close_only(&d, g.code, 3, &format!("{ctx} P opposite"));
            assert_eq!(d.requested_qty, Some(2), "{ctx}");

            let d = decide(SIGNAL, reverse(side, 2), &with(&[g], 0));
            assert_not_executed(&d, Outcome::Skipped, g.code, &format!("{ctx} P flat"));

            let d = decide(SIGNAL, reverse(side, 2), &with(&[g], -opposite));
            assert_not_executed(
                &d,
                Outcome::Rejected,
                "routing.position_conflict",
                &format!("{ctx} P already {side:?}"),
            );
        }
    }
}

#[test]
fn q74_limits_pending_close_only_is_notified() {
    let limits = GATES[6];
    let d = decide(SIGNAL, reverse(Side::Short, 1), &with(&[limits], 2));
    assert_eq!(d.notification_code(), Some("routing.reverse_close_only"));
}

#[test]
fn q74_first_entry_bar_names_the_skipped_open() {
    for (i, first) in GATES[4..8].iter().enumerate() {
        for later in &GATES[4 + i + 1..] {
            let d = decide(SIGNAL, reverse(Side::Long, 1), &with(&[*first, *later], -2));
            assert_close_only(
                &d,
                first.code,
                2,
                &format!("{} + {}", first.code, later.code),
            );
        }
    }
}

#[test]
fn q74_bars_for_all_actions_still_block_the_close_leg() {
    for g in &GATES[..4] {
        let d = decide(SIGNAL, reverse(Side::Long, 1), &with(&[*g], -2));
        assert_not_executed(&d, Outcome::Skipped, g.code, g.code);
    }
}

#[test]
fn q74_closed_qty_is_abs_position_at_the_decision() {
    let oos = GATES[4];
    for (side, position) in [(Side::Long, -1), (Side::Long, -40), (Side::Short, 17)] {
        let mut f = with(&[oos], position);
        f.max_position = 5;
        f.multiplier = 4;
        let d = decide(SIGNAL, reverse(side, 3), &f);
        let closed = u32::try_from(position.unsigned_abs()).unwrap();
        assert_close_only(&d, "routing.out_of_sync", closed, &format!("P={position}"));
        assert_eq!(d.requested_qty, Some(12));
    }
}

/// HTTP 5.1.0: the open leg expires at routing on an account without a bar.
#[test]
fn reverse_expired_at_routing_matrix() {
    let expired = GATES[8];
    let d = decide(SIGNAL, reverse(Side::Long, 2), &with(&[expired], -3));
    assert_close_only(&d, "routing.expired", 3, "expired P opposite");

    let d = decide(SIGNAL, reverse(Side::Long, 2), &with(&[expired], 0));
    assert_not_executed(&d, Outcome::Expired, "routing.expired", "expired P flat");

    let d = decide(SIGNAL, reverse(Side::Long, 2), &with(&[expired], 3));
    assert_not_executed(
        &d,
        Outcome::Rejected,
        "routing.position_conflict",
        "expired P already X",
    );
}

#[test]
fn reverse_entry_bar_precedes_expired() {
    let expired = GATES[8];
    for g in entry_bars() {
        let d = decide(SIGNAL, reverse(Side::Short, 1), &with(&[g, expired], 2));
        assert_close_only(&d, g.code, 2, &format!("{} + expired", g.code));
        let d = decide(SIGNAL, reverse(Side::Short, 1), &with(&[g, expired], 0));
        assert_not_executed(
            &d,
            Outcome::Skipped,
            g.code,
            &format!("{} + expired flat", g.code),
        );
    }
}

#[test]
fn reverse_never_opens_on_a_barred_account() {
    let bars: Vec<Gate> = GATES[4..].to_vec();
    for g in &bars {
        for position in [-4, 0, 4] {
            for side in [Side::Long, Side::Short] {
                let d = decide(SIGNAL, reverse(side, 1), &with(&[*g], position));
                assert!(
                    !d.steps.iter().any(|s| matches!(s, Step::Place { .. })),
                    "{} P={position} {side:?}: {:?}",
                    g.code,
                    d.steps
                );
            }
        }
    }
}

// ---------------------------------------------------------------- Q75 b / Q76 a (ADR-0003 r4)

/// Q75 b: a close on an out-of-sync account that the Core believes flat is still sent as a
/// flatten; the plugin checks the real position and reports `already_flat`.
#[test]
#[ignore = "pending ADR-0003 r4 (Q75 b)"]
fn q75_close_on_out_of_sync_flat_account_is_sent_as_flatten() {
    let d = decide(SIGNAL, Action::Close, &with(&[GATES[4]], 0));
    assert_eq!(d.steps, vec![Step::Flatten]);
}

/// Q75 b also covers the closing leg of a barred reverse (Q74).
#[test]
#[ignore = "pending ADR-0003 r4 (Q75 b)"]
fn q75_barred_reverse_on_out_of_sync_flat_account_sends_the_close_leg() {
    let d = decide(SIGNAL, reverse(Side::Long, 1), &with(&[GATES[4]], 0));
    assert_eq!(d.steps, vec![Step::Flatten]);
    assert_eq!(d.reason_code, Some("routing.out_of_sync"));
}
