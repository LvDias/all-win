//! Expected position and position identity per (account, contract), derived from the order of
//! execution reports (ADR-0004 §5, U13). Never derived from routing order.

use crate::ids::CommandId;
use crate::report::{FillOperation, Origin, ReportFact, ReportStatus};

/// Identity of the current position, with the adapter's rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionIdentity {
    Flat,
    /// The All Win `place` (open) whose fill took the position from flat, or flipped it.
    AllWin(CommandId),
    External,
    /// The Core cannot vouch for it (fail closed): no `reduce` is sent.
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedPosition {
    /// `None` = unknown (no baseline yet).
    quantity: Option<i64>,
    identity: PositionIdentity,
    opening_fill_id: Option<String>,
}

impl ExpectedPosition {
    /// No baseline: neither quantity nor identity is known.
    pub fn unknown() -> Self {
        Self {
            quantity: None,
            identity: PositionIdentity::Unknown,
            opening_fill_id: None,
        }
    }

    pub fn flat() -> Self {
        Self {
            quantity: Some(0),
            identity: PositionIdentity::Flat,
            opening_fill_id: None,
        }
    }

    /// Baseline from a platform snapshot: the quantity is known, a non-flat identity is not.
    pub fn from_snapshot(quantity: i64) -> Self {
        if quantity == 0 {
            Self::flat()
        } else {
            Self {
                quantity: Some(quantity),
                identity: PositionIdentity::Unknown,
                opening_fill_id: None,
            }
        }
    }

    /// Rebuilds from reports already ordered by `seq`, starting flat or from a baseline.
    pub fn replay<'a>(mut self, reports: impl IntoIterator<Item = &'a ReportFact>) -> Self {
        for r in reports {
            self.apply(r);
        }
        self
    }

    pub fn quantity(&self) -> Option<i64> {
        self.quantity
    }

    pub fn identity(&self) -> &PositionIdentity {
        &self.identity
    }

    pub fn is_open(&self) -> Option<bool> {
        self.quantity.map(|q| q != 0)
    }

    /// `refs.position_ref` for a `reduce`; `None` → routing skips with `routing.position_unknown`.
    pub fn position_ref(&self) -> Option<CommandId> {
        match self.identity {
            PositionIdentity::AllWin(id) => Some(id),
            _ => None,
        }
    }

    /// A discontinuity (account reset, journal loss): keep nothing.
    pub fn invalidate(&mut self) {
        *self = Self::unknown();
    }

    /// Applies one report. Only reports carrying `position_after` move the position; a
    /// `duplicate` repeats a known state and is ignored.
    pub fn apply(&mut self, report: &ReportFact) {
        if report.status == ReportStatus::Duplicate {
            return;
        }
        let Some(after) = report.position_after else {
            return;
        };
        let previous = self.quantity;
        self.quantity = Some(after);
        if after == 0 {
            self.identity = PositionIdentity::Flat;
            self.opening_fill_id = None;
            return;
        }
        let correction = report
            .fill
            .as_ref()
            .is_some_and(|f| f.operation != FillOperation::Add);
        let touches_opening = report.fill.as_ref().is_some_and(|f| {
            f.operation != FillOperation::Add && self.opening_fill_id.as_deref() == Some(&f.fill_id)
        });
        if touches_opening {
            self.identity = PositionIdentity::Unknown;
            return;
        }
        let crossed = match previous {
            Some(p) => p == 0 || p.signum() != after.signum(),
            // No baseline: we cannot tell whether this fill opened the position.
            None => {
                self.identity = PositionIdentity::Unknown;
                return;
            }
        };
        if !crossed {
            return;
        }
        if correction {
            self.identity = PositionIdentity::Unknown;
            self.opening_fill_id = None;
            return;
        }
        self.identity = match (report.origin, report.command_id) {
            (Origin::AllWin, Some(id)) => PositionIdentity::AllWin(id),
            _ => PositionIdentity::External,
        };
        self.opening_fill_id = report.fill.as_ref().map(|f| f.fill_id.clone());
    }
}

/// Proof that a flatten left the account flat (ADR-0004 §5 r4): no later report for that
/// account and contract, and either `cancelled` + `command.already_flat` (the adapter checked
/// a zero position) or the latest report carrying `position_after` shows 0.
///
/// `reports` are that (account, contract)'s reports ordered by `seq`; `terminal_seq` is the
/// flatten's terminal report.
pub fn flat_proven(reports: &[ReportFact], terminal_seq: i64) -> bool {
    if reports.iter().any(|r| r.seq > terminal_seq) {
        return false;
    }
    let Some(terminal) = reports.iter().find(|r| r.seq == terminal_seq) else {
        return false;
    };
    if terminal.status == ReportStatus::Cancelled && terminal.reason_is("command.already_flat") {
        return true;
    }
    terminal.status == ReportStatus::Filled
        && reports
            .iter()
            .rev()
            .find_map(|r| r.position_after)
            .is_some_and(|q| q == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::{ChildRole, FillFact};
    use uuid::Uuid;

    fn cid(n: u128) -> CommandId {
        CommandId::from_uuid(Uuid::from_u128(n))
    }

    fn fill(
        seq: i64,
        origin: Origin,
        cmd: Option<u128>,
        fill_id: &str,
        op: FillOperation,
        after: i64,
    ) -> ReportFact {
        ReportFact {
            seq,
            command_id: cmd.map(cid),
            origin,
            child_role: Some(ChildRole::Entry),
            status: ReportStatus::Filled,
            reason_code: None,
            fill: Some(FillFact {
                fill_id: fill_id.into(),
                operation: op,
                quantity: 1,
            }),
            position_after: Some(after),
        }
    }

    use FillOperation::{Add, Amend, Remove};
    use Origin::{AllWin, External};

    #[test]
    fn first_fill_from_flat_sets_identity_and_adds_keep_it() {
        let p = ExpectedPosition::flat().replay(&[
            fill(1, AllWin, Some(1), "f1", Add, 1),
            fill(2, AllWin, Some(2), "f2", Add, 2),
            fill(3, External, None, "f3", Add, 3),
        ]);
        assert_eq!(p.position_ref(), Some(cid(1)));
        assert_eq!(p.quantity(), Some(3));
    }

    #[test]
    fn two_entries_in_flight_first_to_fill_wins() {
        let p = ExpectedPosition::flat().replay(&[
            fill(1, AllWin, Some(7), "a", Add, -1),
            fill(2, AllWin, Some(6), "b", Add, -2),
        ]);
        assert_eq!(p.position_ref(), Some(cid(7)));
    }

    #[test]
    fn flat_then_reopen_by_hand_is_external() {
        let p = ExpectedPosition::flat().replay(&[
            fill(1, AllWin, Some(1), "f1", Add, 1),
            fill(2, AllWin, Some(1), "f2", Add, 0),
            fill(3, External, None, "f3", Add, 1),
        ]);
        assert_eq!(p.identity(), &PositionIdentity::External);
        assert_eq!(p.position_ref(), None);
    }

    #[test]
    fn flip_sets_a_new_identity() {
        let p = ExpectedPosition::flat().replay(&[
            fill(1, External, None, "f1", Add, 2),
            fill(2, AllWin, Some(9), "f2", Add, -1),
        ]);
        assert_eq!(p.position_ref(), Some(cid(9)));
    }

    #[test]
    fn correction_of_the_opening_fill_is_unknown() {
        for op in [Amend, Remove] {
            let p = ExpectedPosition::flat().replay(&[
                fill(1, AllWin, Some(1), "f1", Add, 2),
                fill(2, AllWin, Some(1), "f1", op, 1),
            ]);
            assert_eq!(p.identity(), &PositionIdentity::Unknown, "{op:?}");
            assert_eq!(p.quantity(), Some(1));
        }
    }

    #[test]
    fn correction_of_another_fill_keeps_identity() {
        let p = ExpectedPosition::flat().replay(&[
            fill(1, AllWin, Some(1), "f1", Add, 1),
            fill(2, AllWin, Some(2), "f2", Add, 2),
            fill(3, AllWin, Some(2), "f2", Remove, 1),
        ]);
        assert_eq!(p.position_ref(), Some(cid(1)));
    }

    #[test]
    fn no_baseline_or_snapshot_baseline_is_unknown() {
        let p = ExpectedPosition::unknown().replay(&[fill(1, AllWin, Some(1), "f1", Add, 1)]);
        assert_eq!(p.identity(), &PositionIdentity::Unknown);
        let p =
            ExpectedPosition::from_snapshot(2).replay(&[fill(1, AllWin, Some(1), "f1", Add, 3)]);
        assert_eq!(p.identity(), &PositionIdentity::Unknown);
        let mut p = ExpectedPosition::flat().replay(&[fill(1, AllWin, Some(1), "f1", Add, 1)]);
        p.invalidate();
        assert_eq!((p.quantity(), p.position_ref()), (None, None));
    }

    #[test]
    fn duplicates_and_reports_without_position_are_ignored() {
        let mut dup = fill(2, External, None, "x", Add, 5);
        dup.status = ReportStatus::Duplicate;
        let working = ReportFact {
            position_after: None,
            fill: None,
            status: ReportStatus::Working,
            ..fill(3, AllWin, Some(4), "", Add, 0)
        };
        let p = ExpectedPosition::flat().replay(&[
            fill(1, AllWin, Some(1), "f1", Add, 1),
            dup,
            working,
        ]);
        assert_eq!((p.quantity(), p.position_ref()), (Some(1), Some(cid(1))));
    }

    #[test]
    fn flat_proof_requires_no_later_report() {
        let flatten_filled = ReportFact {
            child_role: Some(ChildRole::Flatten),
            ..fill(2, AllWin, Some(5), "f2", Add, 0)
        };
        let reports = vec![fill(1, AllWin, Some(1), "f1", Add, 1), flatten_filled];
        assert!(flat_proven(&reports, 2));
        let mut later = reports.clone();
        later.push(fill(3, External, None, "f3", Add, 1));
        assert!(!flat_proven(&later, 2));
    }

    #[test]
    fn already_flat_is_its_own_proof() {
        let already = ReportFact {
            seq: 4,
            command_id: Some(cid(5)),
            origin: AllWin,
            child_role: Some(ChildRole::Flatten),
            status: ReportStatus::Cancelled,
            reason_code: Some("command.already_flat".into()),
            fill: None,
            position_after: None,
        };
        assert!(flat_proven(
            &[fill(1, AllWin, Some(1), "f1", Add, 1), already.clone()],
            4
        ));
        let other_cancel = ReportFact {
            reason_code: Some("order.orphan_cancelled".into()),
            ..already
        };
        assert!(!flat_proven(&[other_cancel], 4));
    }

    #[test]
    fn filled_flatten_with_non_zero_position_is_not_proof() {
        let r = ReportFact {
            child_role: Some(ChildRole::Flatten),
            ..fill(2, AllWin, Some(5), "f2", Add, 1)
        };
        assert!(!flat_proven(&[r], 2));
    }
}
