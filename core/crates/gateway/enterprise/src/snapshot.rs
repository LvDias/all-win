//! Comparison of a `position_snapshot` account entry with the Core's expected state
//! (ADR-0004 §6, §6.1). Snapshots are not stored; findings become sync reasons and audit rows.

use std::collections::{BTreeMap, BTreeSet};

use crate::command::CommandKind;
use crate::ids::CommandId;
use crate::instrument::Instrument;
use crate::report::{ChildRole, Origin, ReportFact, ReportStatus};
use crate::sync::SyncReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotTrigger {
    Reconnect,
    Periodic,
    Change,
    Requested,
    AccountReset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservedOrderState {
    Accepted,
    Working,
    TriggerPending,
    Unknown,
}

/// An All Win order on the platform, identified by `(command_id, child_role)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderRef {
    pub command_id: CommandId,
    pub child_role: ChildRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedOrder {
    pub instrument: Instrument,
    pub origin: Origin,
    pub reference: Option<OrderRef>,
    pub state: Option<ObservedOrderState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedAccount {
    pub connected: bool,
    pub positions: BTreeMap<Instrument, i64>,
    pub working_orders: Vec<ObservedOrder>,
}

/// Expected All Win working orders, derived from reports in `seq` order: the last report of
/// each `(command_id, child_role)` decides whether it is live.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExpectedOrders {
    live: BTreeMap<OrderRef, Instrument>,
}

impl ExpectedOrders {
    /// `kind` is the kind of the report's command. Outcome reports of `modify`/`cancel` commands
    /// are never order identities (reason-codes §3), and a failed cancel (`rejected` +
    /// `order.cancel_failed`) leaves the order live (Grosa M6).
    pub fn apply(
        &mut self,
        instrument: &Instrument,
        report: &ReportFact,
        kind: Option<CommandKind>,
    ) {
        let (Some(command_id), Some(child_role)) = (report.command_id, report.child_role) else {
            return;
        };
        if report.origin != Origin::AllWin
            || report.status == ReportStatus::Duplicate
            || matches!(kind, Some(CommandKind::Modify | CommandKind::Cancel))
            || (report.status == ReportStatus::Rejected && report.reason_is("order.cancel_failed"))
        {
            return;
        }
        let key = OrderRef {
            command_id,
            child_role,
        };
        if report.status.is_live() {
            self.live.insert(key, instrument.clone());
        } else {
            self.live.remove(&key);
        }
    }

    pub fn has_protective(&self) -> bool {
        self.live.keys().any(|k| k.child_role.is_protective())
    }

    fn protective_for(&self, instrument: &Instrument) -> BTreeSet<OrderRef> {
        self.live
            .iter()
            .filter(|(k, i)| k.child_role.is_protective() && *i == instrument)
            .map(|(k, _)| *k)
            .collect()
    }
}

/// Protective fills received since the account became unobservable (the active
/// `bracket_integrity_unknown` reason's `detected_at`), as `(bracket owner, leg)`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UnobservedFills(pub Vec<(CommandId, ChildRole)>);

impl UnobservedFills {
    /// Both the stop and the target of one bracket filled (§6.1, Q27).
    pub fn double_fill(&self) -> bool {
        self.0.iter().any(|(c, r)| {
            *r == ChildRole::Stop
                && self
                    .0
                    .iter()
                    .any(|(c2, r2)| c2 == c && *r2 == ChildRole::Target)
        })
    }
}

/// Expected state of one account.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExpectedAccount {
    /// Known positions; an instrument absent here is expected flat.
    pub positions: BTreeMap<Instrument, i64>,
    pub orders: ExpectedOrders,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SnapshotFindings {
    pub raise: BTreeSet<SyncReason>,
    pub resolve: BTreeSet<SyncReason>,
    /// (instrument, expected, observed) — for the audit/alert detail.
    pub position_differences: Vec<(Instrument, i64, i64)>,
    pub missing_orders: Vec<OrderRef>,
    pub unexpected_orders: Vec<OrderRef>,
}

/// `oco_local_or_unknown`: the venue account's `oco_location` is `local` or `unknown` (§6.1).
pub fn evaluate(
    trigger: SnapshotTrigger,
    expected: &ExpectedAccount,
    observed: &ObservedAccount,
    oco_local_or_unknown: bool,
    unobserved: &UnobservedFills,
) -> SnapshotFindings {
    let mut f = SnapshotFindings::default();
    if trigger == SnapshotTrigger::AccountReset {
        // A discontinuity, never read as fills (S11 AC-6).
        f.raise.insert(SyncReason::SimulationReset);
        return f;
    }

    if !observed.connected {
        // The broker connection is lost: the adapter cannot see positions or orders, so nothing
        // is compared (Agulha); only the integrity of local-OCO brackets becomes unknown (§6.1).
        let protective_working = expected.orders.has_protective()
            || observed.working_orders.iter().any(|o| {
                o.origin == Origin::AllWin
                    && o.reference.is_some_and(|r| r.child_role.is_protective())
            });
        if protective_working && oco_local_or_unknown {
            f.raise.insert(SyncReason::BracketIntegrityUnknown);
        }
        return f;
    }

    let instruments: BTreeSet<&Instrument> = expected
        .positions
        .keys()
        .chain(observed.positions.keys())
        .collect();
    for i in instruments {
        let e = expected.positions.get(i).copied().unwrap_or(0);
        let o = observed.positions.get(i).copied().unwrap_or(0);
        if e != o {
            f.position_differences.push((i.clone(), e, o));
        }
    }

    let observed_refs: BTreeSet<OrderRef> = observed
        .working_orders
        .iter()
        .filter(|o| o.origin == Origin::AllWin)
        .filter_map(|o| o.reference)
        .collect();
    let expected_refs: BTreeSet<OrderRef> = expected.orders.live.keys().copied().collect();
    f.missing_orders = expected_refs.difference(&observed_refs).copied().collect();
    f.unexpected_orders = observed_refs.difference(&expected_refs).copied().collect();

    if !f.position_differences.is_empty()
        || !f.missing_orders.is_empty()
        || !f.unexpected_orders.is_empty()
    {
        f.raise.insert(SyncReason::SnapshotMismatch);
    }

    for (instrument, &quantity) in &observed.positions {
        let protective_live: Vec<&ObservedOrder> = observed
            .working_orders
            .iter()
            .filter(|o| {
                o.origin == Origin::AllWin
                    && &o.instrument == instrument
                    && o.reference.is_some_and(|r| r.child_role.is_protective())
            })
            .collect();
        let expected_protective = expected.orders.protective_for(instrument);
        if quantity != 0 {
            let any_unknown = protective_live
                .iter()
                .any(|o| o.state == Some(ObservedOrderState::Unknown));
            let missing = expected_protective
                .iter()
                .any(|r| !protective_live.iter().any(|o| o.reference == Some(*r)));
            if any_unknown || missing {
                f.raise.insert(SyncReason::BracketIncomplete);
            }
        } else if !protective_live.is_empty() {
            // Flat with a leg still working: the adapter cancels it (orphan protection).
            f.raise.insert(SyncReason::BracketOrphanLeg);
        }
    }
    // A flat instrument absent from `positions` with legs still working is an orphan too.
    for o in &observed.working_orders {
        let protective =
            o.origin == Origin::AllWin && o.reference.is_some_and(|r| r.child_role.is_protective());
        if protective && !observed.positions.contains_key(&o.instrument) {
            f.raise.insert(SyncReason::BracketOrphanLeg);
        }
    }

    if trigger == SnapshotTrigger::Reconnect && unobserved.double_fill() {
        // TODO(S11): the orphan_fill flatten of §6.1 Q27; S2 only flags and alerts (Esquadro Q2).
        f.raise.insert(SyncReason::BracketDoubleFill);
    }
    // A protective leg that filled while the account was unobservable never resolves the
    // integrity reason: it stays until the user clears (Esquadro Q2).
    if f.raise.is_empty() && unobserved.0.is_empty() {
        // Observable again and everything matches.
        f.resolve.insert(SyncReason::BracketIntegrityUnknown);
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn mnq() -> Instrument {
        Instrument::new("MNQ", "2026-12").unwrap()
    }

    fn r(n: u128, role: ChildRole) -> OrderRef {
        OrderRef {
            command_id: CommandId::from_uuid(Uuid::from_u128(n)),
            child_role: role,
        }
    }

    fn report(n: u128, role: ChildRole, status: ReportStatus) -> ReportFact {
        ReportFact {
            seq: 1,
            command_id: Some(CommandId::from_uuid(Uuid::from_u128(n))),
            origin: Origin::AllWin,
            child_role: Some(role),
            status,
            reason_code: None,
            fill: None,
            position_after: None,
        }
    }

    fn observed_order(reference: OrderRef, state: ObservedOrderState) -> ObservedOrder {
        ObservedOrder {
            instrument: mnq(),
            origin: Origin::AllWin,
            reference: Some(reference),
            state: Some(state),
        }
    }

    fn bracketed() -> ExpectedAccount {
        let mut orders = ExpectedOrders::default();
        orders.apply(
            &mnq(),
            &report(1, ChildRole::Stop, ReportStatus::Working),
            Some(CommandKind::Place),
        );
        orders.apply(
            &mnq(),
            &report(1, ChildRole::Target, ReportStatus::Working),
            Some(CommandKind::Place),
        );
        ExpectedAccount {
            positions: BTreeMap::from([(mnq(), 1)]),
            orders,
        }
    }

    fn matching() -> ObservedAccount {
        ObservedAccount {
            connected: true,
            positions: BTreeMap::from([(mnq(), 1)]),
            working_orders: vec![
                observed_order(r(1, ChildRole::Stop), ObservedOrderState::Working),
                observed_order(r(1, ChildRole::Target), ObservedOrderState::Working),
            ],
        }
    }

    #[test]
    fn match_raises_nothing_and_resolves_integrity_unknown() {
        let f = evaluate(
            SnapshotTrigger::Reconnect,
            &bracketed(),
            &matching(),
            true,
            &UnobservedFills::default(),
        );
        assert!(f.raise.is_empty());
        assert!(f.resolve.contains(&SyncReason::BracketIntegrityUnknown));
    }

    #[test]
    fn position_difference_is_a_mismatch() {
        let mut o = matching();
        o.positions.insert(mnq(), 2);
        let f = evaluate(
            SnapshotTrigger::Periodic,
            &bracketed(),
            &o,
            false,
            &UnobservedFills::default(),
        );
        assert!(f.raise.contains(&SyncReason::SnapshotMismatch));
        assert_eq!(f.position_differences, vec![(mnq(), 1, 2)]);
    }

    #[test]
    fn missing_or_unknown_leg_on_open_position_is_bracket_incomplete() {
        let mut o = matching();
        o.working_orders.pop();
        let f = evaluate(
            SnapshotTrigger::Reconnect,
            &bracketed(),
            &o,
            true,
            &UnobservedFills::default(),
        );
        assert!(f.raise.contains(&SyncReason::BracketIncomplete));
        assert_eq!(f.missing_orders, vec![r(1, ChildRole::Target)]);

        let mut o = matching();
        o.working_orders[0].state = Some(ObservedOrderState::Unknown);
        let f = evaluate(
            SnapshotTrigger::Reconnect,
            &bracketed(),
            &o,
            true,
            &UnobservedFills::default(),
        );
        assert!(f.raise.contains(&SyncReason::BracketIncomplete));
    }

    #[test]
    fn flat_with_a_working_leg_is_an_orphan() {
        let mut expected = bracketed();
        expected.orders.apply(
            &mnq(),
            &report(1, ChildRole::Stop, ReportStatus::Filled),
            Some(CommandKind::Place),
        );
        expected.positions.insert(mnq(), 0);
        let o = ObservedAccount {
            connected: true,
            positions: BTreeMap::new(),
            working_orders: vec![observed_order(
                r(1, ChildRole::Target),
                ObservedOrderState::Working,
            )],
        };
        let f = evaluate(
            SnapshotTrigger::Reconnect,
            &expected,
            &o,
            true,
            &UnobservedFills::default(),
        );
        assert!(f.raise.contains(&SyncReason::BracketOrphanLeg));
    }

    #[test]
    fn broker_disconnected_with_legs_on_local_oco_is_integrity_unknown() {
        let mut o = matching();
        o.connected = false;
        let f = evaluate(
            SnapshotTrigger::Periodic,
            &bracketed(),
            &o,
            true,
            &UnobservedFills::default(),
        );
        assert!(f.raise.contains(&SyncReason::BracketIntegrityUnknown));
        let f = evaluate(
            SnapshotTrigger::Periodic,
            &bracketed(),
            &o,
            false,
            &UnobservedFills::default(),
        );
        assert!(!f.raise.contains(&SyncReason::BracketIntegrityUnknown));
        assert!(f.resolve.is_empty());
    }

    #[test]
    fn outcome_reports_and_failed_cancels_do_not_change_orders() {
        let mut orders = bracketed().orders;
        // A cancel command's outcome under the leg's role is not an identity.
        orders.apply(
            &mnq(),
            &report(7, ChildRole::Stop, ReportStatus::Cancelled),
            Some(CommandKind::Cancel),
        );
        orders.apply(
            &mnq(),
            &report(8, ChildRole::Target, ReportStatus::Working),
            Some(CommandKind::Modify),
        );
        // A failed cancel of the real leg keeps it live.
        let mut failed = report(1, ChildRole::Stop, ReportStatus::Rejected);
        failed.reason_code = Some("order.cancel_failed".into());
        orders.apply(&mnq(), &failed, Some(CommandKind::Place));
        assert_eq!(orders, bracketed().orders);
    }

    #[test]
    fn double_fill_is_flagged_and_a_filled_leg_never_resolves_integrity() {
        let c = CommandId::from_uuid(Uuid::from_u128(1));
        let o = matching();
        let one = UnobservedFills(vec![(c, ChildRole::Stop)]);
        let f = evaluate(SnapshotTrigger::Reconnect, &bracketed(), &o, true, &one);
        assert!(!f.raise.contains(&SyncReason::BracketDoubleFill));
        assert!(!f.resolve.contains(&SyncReason::BracketIntegrityUnknown));
        let both = UnobservedFills(vec![(c, ChildRole::Stop), (c, ChildRole::Target)]);
        let f = evaluate(SnapshotTrigger::Reconnect, &bracketed(), &o, true, &both);
        assert!(f.raise.contains(&SyncReason::BracketDoubleFill));
        // Legs of different brackets are not a double fill.
        let other = CommandId::from_uuid(Uuid::from_u128(2));
        assert!(
            !UnobservedFills(vec![(c, ChildRole::Stop), (other, ChildRole::Target)]).double_fill()
        );
    }

    #[test]
    fn disconnected_broker_is_not_compared() {
        // The adapter cannot see positions: an empty snapshot is not a mismatch.
        let o = ObservedAccount {
            connected: false,
            positions: BTreeMap::new(),
            working_orders: vec![],
        };
        let f = evaluate(
            SnapshotTrigger::Periodic,
            &bracketed(),
            &o,
            true,
            &UnobservedFills::default(),
        );
        assert_eq!(
            f.raise,
            BTreeSet::from([SyncReason::BracketIntegrityUnknown])
        );
        assert!(
            f.position_differences.is_empty()
                && f.missing_orders.is_empty()
                && f.resolve.is_empty()
        );
    }

    #[test]
    fn account_reset_is_only_a_reset() {
        let o = ObservedAccount {
            connected: true,
            positions: BTreeMap::new(),
            working_orders: vec![],
        };
        let f = evaluate(
            SnapshotTrigger::AccountReset,
            &bracketed(),
            &o,
            true,
            &UnobservedFills::default(),
        );
        assert_eq!(f.raise, BTreeSet::from([SyncReason::SimulationReset]));
        assert!(f.position_differences.is_empty());
    }

    #[test]
    fn external_orders_are_not_compared() {
        let mut o = matching();
        o.working_orders.push(ObservedOrder {
            instrument: mnq(),
            origin: Origin::External,
            reference: None,
            state: None,
        });
        let f = evaluate(
            SnapshotTrigger::Periodic,
            &bracketed(),
            &o,
            true,
            &UnobservedFills::default(),
        );
        assert!(f.raise.is_empty());
    }
}
