//! Out-of-sync triggers from execution reports and disconnects (ADR-0004 §5–§6; sync code
//! mapping by Esquadro, HTTP 5.3.0 draft). Snapshot triggers live in `snapshot`.

use crate::report::{ChildRole, FillOperation, Origin, ReportFact, ReportStatus};
use crate::sync::SyncReason;

/// What the caller knows about the report's account and order when classifying it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportContext {
    /// The account is a destination of master replication (T3: external activity flags it;
    /// TradingView-only destinations are audited only).
    pub master_replication_destination: bool,
    /// Expected position in that contract before this report (`None` = unknown).
    pub expected_before: Option<i64>,
    /// Quantity already filled on the same order before this report.
    pub filled_before: u32,
    /// For `amend`/`remove`: the original `add` of that `fill_id` is known.
    pub fill_add_known: bool,
    /// For a correction: the authoritative `position_after` equals the expected position after
    /// replaying the corrected history.
    pub correction_consistent: bool,
    /// Protective legs of the account were cancelled after this report's flatten was sent
    /// (the flatten cancels every working order before closing).
    pub protection_cancelled_by_flatten: bool,
}

/// Sync reasons one report raises. Empty means "audit only" (or nothing).
pub fn classify_report(report: &ReportFact, ctx: &ReportContext) -> Vec<SyncReason> {
    let mut reasons = Vec::new();
    if report.status == ReportStatus::Duplicate {
        return reasons;
    }

    if let Some(fill) = &report.fill
        && fill.operation != FillOperation::Add
        && (!ctx.fill_add_known || !ctx.correction_consistent)
    {
        reasons.push(SyncReason::FillCorrected);
    }

    if report.origin == Origin::External {
        if ctx.master_replication_destination {
            reasons.push(SyncReason::ExternalActivity);
        }
        return reasons;
    }

    let protective = report.child_role.is_some_and(ChildRole::is_protective);
    // U19 / Esmeril M1: a flatten (any origin) that failed after cancelling protection while
    // the position stays open. With protection intact it is an ordinary rejection.
    if report.child_role == Some(ChildRole::Flatten)
        && matches!(
            report.status,
            ReportStatus::Rejected | ReportStatus::Unknown
        )
        && ctx.protection_cancelled_by_flatten
        && ctx.expected_before.is_none_or(|q| q != 0)
    {
        reasons.push(SyncReason::FlattenFailedUnprotected);
        return reasons;
    }
    match report.status {
        ReportStatus::Rejected => {
            let reason = if report.reason_is("command.position_changed") {
                SyncReason::PositionChanged
            } else if report.reason_is("order.ambiguous_reference") {
                SyncReason::AmbiguousReference
            } else if report.reason_is("order.protection_failed")
                || report.reason_is("order.protection_not_submitted")
            {
                SyncReason::BracketIncomplete
            } else if report.reason_is("order.cancel_failed") && report.command_id.is_none() {
                // Cancel started by the adapter itself (orphan protection), no command.
                SyncReason::CancelFailed
            } else {
                SyncReason::CommandRejected
            };
            reasons.push(reason);
        }
        ReportStatus::Expired => reasons.push(SyncReason::CommandExpired),
        ReportStatus::Unknown => reasons.push(SyncReason::CommandUnknown),
        ReportStatus::Cancelled => {
            if report.reason_is("command.nothing_to_reduce")
                && ctx.expected_before.is_some_and(|q| q != 0)
            {
                // The position changed under the reduce: same fact as the U13 guard.
                reasons.push(SyncReason::PositionChanged);
            } else if !protective
                && ctx.filled_before > 0
                && !report.reason_is("command.already_flat")
                && report.origin == Origin::AllWin
            {
                // An order that ended partially filled.
                reasons.push(SyncReason::CommandPartiallyFilled);
            }
        }
        _ => {}
    }
    reasons
}

/// How a session ended, for the disconnect rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionEnd {
    /// Socket loss, heartbeat timeout, server shutdown, intake retry (4000/4005).
    Network,
    /// The user pressed "Desconectar" in the plugin (4100).
    ByUser,
}

/// Per-account facts at the moment the session ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountAtDisconnect {
    /// Open position in any contract; `None` (unknown) counts as open (fail closed).
    pub position_open: Option<bool>,
    pub command_in_flight: bool,
    pub protective_working: bool,
    pub oco_local_or_unknown: bool,
}

/// ADR-0004 §6 disconnect rows and §6.1 "while unobservable". Flat with nothing in flight →
/// no reason (status only).
pub fn on_disconnect(end: SessionEnd, account: &AccountAtDisconnect) -> Vec<SyncReason> {
    let mut reasons = Vec::new();
    let exposed = account.position_open.unwrap_or(true) || account.command_in_flight;
    if exposed {
        reasons.push(match end {
            SessionEnd::Network => SyncReason::DisconnectedInTrade,
            SessionEnd::ByUser => SyncReason::DisconnectedByUser,
        });
    }
    if account.protective_working && account.oco_local_or_unknown {
        reasons.push(SyncReason::BracketIntegrityUnknown);
    }
    reasons
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::CommandId;
    use crate::report::FillFact;
    use uuid::Uuid;

    fn ctx() -> ReportContext {
        ReportContext {
            master_replication_destination: false,
            expected_before: Some(0),
            filled_before: 0,
            fill_add_known: true,
            correction_consistent: true,
            protection_cancelled_by_flatten: false,
        }
    }

    fn rep(status: ReportStatus, reason: Option<&str>) -> ReportFact {
        ReportFact {
            seq: 1,
            command_id: Some(CommandId::from_uuid(Uuid::from_u128(1))),
            origin: Origin::AllWin,
            child_role: Some(ChildRole::Entry),
            status,
            reason_code: reason.map(Into::into),
            fill: None,
            position_after: None,
        }
    }

    #[test]
    fn rejections_map_to_their_codes() {
        let cases = [
            (
                "command.account_not_simulation",
                SyncReason::CommandRejected,
            ),
            ("order.platform_rejected", SyncReason::CommandRejected),
            ("command.prerequisite_failed", SyncReason::CommandRejected),
            ("order.cancel_failed", SyncReason::CommandRejected),
            ("command.position_changed", SyncReason::PositionChanged),
            ("order.ambiguous_reference", SyncReason::AmbiguousReference),
            ("order.protection_failed", SyncReason::BracketIncomplete),
            (
                "order.protection_not_submitted",
                SyncReason::BracketIncomplete,
            ),
        ];
        for (code, reason) in cases {
            assert_eq!(
                classify_report(&rep(ReportStatus::Rejected, Some(code)), &ctx()),
                vec![reason],
                "{code}"
            );
        }
        let orphan_cancel_failed = ReportFact {
            command_id: None,
            ..rep(ReportStatus::Rejected, Some("order.cancel_failed"))
        };
        assert_eq!(
            classify_report(&orphan_cancel_failed, &ctx()),
            vec![SyncReason::CancelFailed]
        );
    }

    #[test]
    fn expired_unknown_and_duplicate() {
        assert_eq!(
            classify_report(&rep(ReportStatus::Expired, Some("command.expired")), &ctx()),
            vec![SyncReason::CommandExpired]
        );
        assert_eq!(
            classify_report(&rep(ReportStatus::Unknown, None), &ctx()),
            vec![SyncReason::CommandUnknown]
        );
        assert!(classify_report(&rep(ReportStatus::Duplicate, None), &ctx()).is_empty());
    }

    #[test]
    fn benign_cancellations_raise_nothing() {
        let already_flat = ReportFact {
            child_role: Some(ChildRole::Flatten),
            ..rep(ReportStatus::Cancelled, Some("command.already_flat"))
        };
        assert!(classify_report(&already_flat, &ctx()).is_empty());
        let nothing = rep(ReportStatus::Cancelled, Some("command.nothing_to_reduce"));
        assert!(classify_report(&nothing, &ctx()).is_empty());
        let orphan = ReportFact {
            child_role: Some(ChildRole::Stop),
            ..rep(ReportStatus::Cancelled, Some("order.orphan_cancelled"))
        };
        assert!(
            classify_report(
                &orphan,
                &ReportContext {
                    filled_before: 1,
                    ..ctx()
                }
            )
            .is_empty()
        );
    }

    #[test]
    fn nothing_to_reduce_against_an_open_expected_position_is_position_changed() {
        let nothing = rep(ReportStatus::Cancelled, Some("command.nothing_to_reduce"));
        let c = ReportContext {
            expected_before: Some(-2),
            ..ctx()
        };
        assert_eq!(
            classify_report(&nothing, &c),
            vec![SyncReason::PositionChanged]
        );
    }

    #[test]
    fn partial_then_cancelled_is_partially_filled() {
        let c = ReportContext {
            filled_before: 1,
            ..ctx()
        };
        assert_eq!(
            classify_report(&rep(ReportStatus::Cancelled, None), &c),
            vec![SyncReason::CommandPartiallyFilled]
        );
    }

    #[test]
    fn external_activity_only_on_master_replication_destinations() {
        let ext = ReportFact {
            origin: Origin::External,
            command_id: None,
            ..rep(ReportStatus::Filled, None)
        };
        assert!(classify_report(&ext, &ctx()).is_empty());
        let c = ReportContext {
            master_replication_destination: true,
            ..ctx()
        };
        assert_eq!(
            classify_report(&ext, &c),
            vec![SyncReason::ExternalActivity]
        );
    }

    #[test]
    fn fill_corrections() {
        let amend = ReportFact {
            fill: Some(FillFact {
                fill_id: "f".into(),
                operation: FillOperation::Amend,
                quantity: 1,
            }),
            ..rep(ReportStatus::Filled, None)
        };
        assert!(classify_report(&amend, &ctx()).is_empty());
        assert_eq!(
            classify_report(
                &amend,
                &ReportContext {
                    fill_add_known: false,
                    ..ctx()
                }
            ),
            vec![SyncReason::FillCorrected]
        );
        assert_eq!(
            classify_report(
                &amend,
                &ReportContext {
                    correction_consistent: false,
                    ..ctx()
                }
            ),
            vec![SyncReason::FillCorrected]
        );
    }

    #[test]
    fn disconnect_rule() {
        let flat = AccountAtDisconnect {
            position_open: Some(false),
            command_in_flight: false,
            protective_working: false,
            oco_local_or_unknown: true,
        };
        assert!(on_disconnect(SessionEnd::Network, &flat).is_empty());
        assert!(on_disconnect(SessionEnd::ByUser, &flat).is_empty());
        let open = AccountAtDisconnect {
            position_open: Some(true),
            protective_working: true,
            ..flat
        };
        assert_eq!(
            on_disconnect(SessionEnd::Network, &open),
            vec![
                SyncReason::DisconnectedInTrade,
                SyncReason::BracketIntegrityUnknown
            ]
        );
        assert_eq!(
            on_disconnect(SessionEnd::ByUser, &open),
            vec![
                SyncReason::DisconnectedByUser,
                SyncReason::BracketIntegrityUnknown
            ]
        );
        let in_flight = AccountAtDisconnect {
            command_in_flight: true,
            ..flat
        };
        assert_eq!(
            on_disconnect(SessionEnd::Network, &in_flight),
            vec![SyncReason::DisconnectedInTrade]
        );
        let unknown = AccountAtDisconnect {
            position_open: None,
            ..flat
        };
        assert_eq!(
            on_disconnect(SessionEnd::Network, &unknown),
            vec![SyncReason::DisconnectedInTrade]
        );
        let server_oco = AccountAtDisconnect {
            oco_local_or_unknown: false,
            ..open
        };
        assert_eq!(
            on_disconnect(SessionEnd::Network, &server_oco),
            vec![SyncReason::DisconnectedInTrade]
        );
    }
}

#[cfg(test)]
mod flatten_tests {
    use super::*;
    use crate::ids::CommandId;
    use uuid::Uuid;

    fn flatten_report(status: ReportStatus) -> ReportFact {
        ReportFact {
            seq: 1,
            command_id: Some(CommandId::from_uuid(Uuid::from_u128(1))),
            origin: Origin::AllWin,
            child_role: Some(ChildRole::Flatten),
            status,
            reason_code: Some("order.platform_rejected".into()),
            fill: None,
            position_after: None,
        }
    }

    fn ctx(open: Option<i64>, lost: bool) -> ReportContext {
        ReportContext {
            master_replication_destination: false,
            expected_before: open,
            filled_before: 0,
            fill_add_known: true,
            correction_consistent: true,
            protection_cancelled_by_flatten: lost,
        }
    }

    #[test]
    fn failed_flatten_that_lost_protection_is_critical() {
        for status in [ReportStatus::Rejected, ReportStatus::Unknown] {
            assert_eq!(
                classify_report(&flatten_report(status), &ctx(Some(2), true)),
                vec![SyncReason::FlattenFailedUnprotected]
            );
            // Unknown expected position counts as open (fail closed).
            assert_eq!(
                classify_report(&flatten_report(status), &ctx(None, true)),
                vec![SyncReason::FlattenFailedUnprotected]
            );
        }
    }

    #[test]
    fn failed_flatten_with_protection_intact_or_flat_is_an_ordinary_rejection() {
        assert_eq!(
            classify_report(
                &flatten_report(ReportStatus::Rejected),
                &ctx(Some(2), false)
            ),
            vec![SyncReason::CommandRejected]
        );
        assert_eq!(
            classify_report(&flatten_report(ReportStatus::Rejected), &ctx(Some(0), true)),
            vec![SyncReason::CommandRejected]
        );
        assert_eq!(
            classify_report(&flatten_report(ReportStatus::Unknown), &ctx(Some(2), false)),
            vec![SyncReason::CommandUnknown]
        );
    }
}
