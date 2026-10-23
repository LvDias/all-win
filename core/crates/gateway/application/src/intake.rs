//! Execution-report intake (ADR-0004 §4–§7). One transaction per report: msg_id receipt,
//! report row (event_id and (account, fill_id) dedupe), command state, sync CAS, E1/E2.
//!
//! Esquadro P1: if anything fails (store or a port), nothing is written and the frame is not
//! acked; the caller closes the session with 4000 "intake_retry" and the adapter retransmits
//! in order. Intake runs one report at a time per connection, so `seq` is arrival order.

use std::str::FromStr;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::command::CommandState;
use gateway_enterprise::commission::effective_commission;
use gateway_enterprise::ids::{AccountId, CommandId, EventId};
use gateway_enterprise::position::ExpectedPosition;
use gateway_enterprise::report::{
    ChildRole, FillFact, FillOperation, Origin, ReportFact, ReportStatus,
};
use gateway_enterprise::sync::{ReasonDetails, ReasonEntry, ReasonScope, SyncReason};
use gateway_enterprise::triggers::{ReportContext, classify_report};
use rust_decimal::Decimal;
use serde_json::{Value, json};
use shared_kernel::Money;
use uuid::Uuid;

use crate::events;
use crate::ports::{
    AccountBindings, CommandTransition, CommitOutcome, GatewayStore, PortError, ReportRow,
    RiskLimits, StoreError, SyncWrite, UnitOfWork,
};
use crate::session::Session;
use crate::wire::{ExecutionReport, timestamp};

#[derive(Debug, thiserror::Error)]
pub enum IntakeError {
    /// A schema-valid report this Core cannot interpret (e.g. an account outside the session's
    /// mapping): answered with `error` and not acked.
    #[error("invalid report: {0}")]
    Invalid(&'static str),
    #[error(transparent)]
    Port(#[from] PortError),
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntakeOutcome {
    Stored,
    /// Already stored (msg_id, event_id or fill): ack again, do nothing else.
    Duplicate,
}

#[derive(Clone)]
pub struct HandleExecutionReport {
    store: Arc<dyn GatewayStore>,
    bindings: Arc<dyn AccountBindings>,
    risk: Arc<dyn RiskLimits>,
}

impl HandleExecutionReport {
    pub fn new(
        store: Arc<dyn GatewayStore>,
        bindings: Arc<dyn AccountBindings>,
        risk: Arc<dyn RiskLimits>,
    ) -> Self {
        Self {
            store,
            bindings,
            risk,
        }
    }

    pub async fn handle(
        &self,
        session: &Session,
        msg_id: Uuid,
        report: &ExecutionReport,
        payload: &Value,
        now: DateTime<Utc>,
    ) -> Result<IntakeOutcome, IntakeError> {
        let account_id = AccountId::from_uuid(report.account_id);
        let mapped = session
            .mapped(account_id)
            .ok_or(IntakeError::Invalid("account not in welcome"))?;
        if mapped.venue.platform_account != report.platform_account {
            return Err(IntakeError::Invalid(
                "platform_account does not match the mapping",
            ));
        }
        let fact = to_fact(report)?;
        let user = session.user_id;

        // Commission is fixed once, with the rate in force now (rev. 15.1, Esquadro L2).
        let mut commission = None;
        let mut commission_rate = None;
        let mut effective = None;
        if let Some(fill) = &report.fill {
            commission = fill.commission.as_deref().map(money).transpose()?;
            commission_rate = self.risk.commission_rate(user, account_id).await?;
            let e = effective_commission(commission, commission_rate, fill.quantity)
                .map_err(|_| IntakeError::Invalid("commission"))?;
            effective = Some(e);
        }

        // ADR-0008 §3.2 step 4: activity on a venue account that is no longer bound to it.
        let bound_now = self.bindings.binding_of(user, account_id).await? == Some(mapped.venue.id);

        let stored_instruction = match fact.command_id {
            Some(c) => self.store.command(user, c).await?.map(|s| s.instruction_id),
            None => None,
        };

        let mut attempts = 0;
        loop {
            let history = self
                .store
                .position_history(user, account_id, &report.instrument)
                .await?;
            let expected = ExpectedPosition::flat().replay(&history);
            let filled_before = match fact.command_id {
                Some(c) => self
                    .store
                    .command_reports(user, c)
                    .await?
                    .iter()
                    .filter(|r| r.child_role == fact.child_role)
                    .filter_map(|r| r.fill.as_ref().map(|f| f.quantity))
                    .sum(),
                None => 0,
            };
            let fill_add_known = match &fact.fill {
                Some(f) if f.operation != FillOperation::Add => {
                    self.store.fill_known(user, account_id, &f.fill_id).await?
                }
                _ => true,
            };
            let ctx = ReportContext {
                // S19 (master replication) is V1.1: every destination is TradingView-only (T3).
                master_replication_destination: false,
                expected_before: expected.quantity(),
                filled_before,
                fill_add_known,
                correction_consistent: fact.position_after.is_none()
                    || fact.position_after == expected.quantity(),
            };
            let reasons = if bound_now {
                classify_report(&fact, &ctx)
            } else {
                Vec::new()
            };

            let mut sync = self.store.account_sync(user, account_id).await?;
            let expected_version = sync.version();
            let mut work = UnitOfWork {
                receipt: Some((session.connection_id, msg_id)),
                ..UnitOfWork::default()
            };

            let mut changes = Vec::new();
            for code in reasons {
                let entry = ReasonEntry::new(
                    code,
                    scope_for(code, &fact, report),
                    now,
                    details_for(code, &fact, &ctx),
                );
                if let Some(c) = sync.raise(entry) {
                    changes.push(c);
                }
            }
            let refs = crate::audit::refs(account_id, Some(&mapped.display_name), None);
            for c in &changes {
                work.events
                    .extend(crate::audit::sync_entries(c, now, &refs));
            }
            if let Some(last) = changes.last() {
                let mut merged = last.clone();
                merged.added = changes.iter().flat_map(|c| c.added.clone()).collect();
                let codes = sync
                    .wire_codes()
                    .iter()
                    .map(|c| c.as_str().to_owned())
                    .collect();
                work.events.push(events::account_sync_changed(
                    account_id,
                    &merged,
                    codes,
                    json!({ "kind": "execution_report", "ref_id": report.event_id }),
                    now,
                ));
                work.sync.push(SyncWrite {
                    account_id,
                    expected_version,
                    next: sync,
                });
            }

            if let Some(transition) = self.command_transition(user, &fact, now).await? {
                work.commands.push(transition);
            }

            let e1 = e1_payload(
                report,
                &fact,
                commission,
                commission_rate,
                effective.map(|e| (e.effective, e.source.as_str())),
                session,
            );
            work.events.push(events::execution_reported(now, e1));
            if let Some(code) = crate::audit::execution_code(&fact) {
                let details = crate::audit::execution_details(
                    report,
                    &fact,
                    effective.map(|e| e.effective.to_string()),
                );
                let exec_refs = match fact.command_id.and(stored_instruction) {
                    Some(i) => crate::audit::refs(account_id, Some(&mapped.display_name), Some(i)),
                    None => refs.clone(),
                };
                work.events
                    .push(events::audit(code, now, details, exec_refs));
            }
            if !bound_now && let Some(fill) = &report.fill {
                work.events.push(events::audit(
                    "execution.unbound_activity",
                    now,
                    {
                        let mut d = json!({
                            "connection_id": session.connection_id,
                            "platform_account": report.platform_account,
                            "kind": "fill",
                            "instrument_symbol": report.instrument.symbol(),
                            "fill_qty": fill.quantity,
                        });
                        if let Some(side) = &report.side {
                            d["side"] = json!(side);
                        }
                        d
                    },
                    json!({ "account_id": account_id }),
                ));
            }
            work.report = Some(ReportRow {
                event_id: EventId::from_uuid(report.event_id),
                account_id,
                connection_id: session.connection_id,
                instrument: Some(report.instrument.clone()),
                fact: fact.clone(),
                fill_price: report.fill.as_ref().map(|f| f.price.clone()),
                commission,
                commission_rate,
                commission_effective: effective.map(|e| e.effective),
                commission_source: effective.map(|e| e.source.as_str()),
                platform_order_id: report.platform_order_id.clone(),
                avg_price_after: report
                    .position_after
                    .as_ref()
                    .and_then(|p| p.average_price.clone()),
                occurred_at: report.occurred_at,
                platform_time: report.platform_time,
                payload: payload.clone(),
            });

            match self.store.commit(user, work).await {
                Ok(CommitOutcome::Committed) => return Ok(IntakeOutcome::Stored),
                Ok(CommitOutcome::Duplicate) => return Ok(IntakeOutcome::Duplicate),
                Err(StoreError::Conflict) if attempts < 3 => attempts += 1,
                Err(e) => return Err(e.into()),
            }
        }
    }

    /// The command follows reports about its own order (not its protective legs).
    async fn command_transition(
        &self,
        user: shared_kernel::UserId,
        fact: &ReportFact,
        now: DateTime<Utc>,
    ) -> Result<Option<CommandTransition>, StoreError> {
        let Some(id) = fact.command_id else {
            return Ok(None);
        };
        if fact.child_role.is_some_and(ChildRole::is_protective) || fact.origin != Origin::AllWin {
            return Ok(None);
        }
        let to = match fact.status {
            ReportStatus::Filled | ReportStatus::Cancelled => CommandState::Completed,
            ReportStatus::Rejected => CommandState::Rejected,
            ReportStatus::Expired => CommandState::Expired,
            ReportStatus::Unknown => CommandState::Unknown,
            ReportStatus::Received
            | ReportStatus::Working
            | ReportStatus::PartiallyFilled
            | ReportStatus::Duplicate => {
                return Ok(None);
            }
        };
        let Some(stored) = self.store.command(user, id).await? else {
            return Ok(None);
        };
        let mut cmd = stored.command.clone();
        match cmd.finish(to) {
            Ok(true) => Ok(Some(CommandTransition {
                id,
                from: stored.command.state,
                to,
                at: now,
            })),
            _ => Ok(None),
        }
    }
}

fn money(s: &str) -> Result<Money, IntakeError> {
    Decimal::from_str(s)
        .map(Money::new)
        .map_err(|_| IntakeError::Invalid("money"))
}

fn to_fact(r: &ExecutionReport) -> Result<ReportFact, IntakeError> {
    let origin = match r.origin.as_str() {
        "all_win" => Origin::AllWin,
        "local_enforcement" => Origin::LocalEnforcement,
        "external" => Origin::External,
        _ => return Err(IntakeError::Invalid("origin")),
    };
    let child_role = r
        .child_role
        .as_deref()
        .map(|c| match c {
            "entry" => Ok(ChildRole::Entry),
            "stop" => Ok(ChildRole::Stop),
            "target" => Ok(ChildRole::Target),
            "exit" => Ok(ChildRole::Exit),
            "flatten" => Ok(ChildRole::Flatten),
            _ => Err(IntakeError::Invalid("child_role")),
        })
        .transpose()?;
    let status = match r.status.as_str() {
        "received" => ReportStatus::Received,
        "working" => ReportStatus::Working,
        "partially_filled" => ReportStatus::PartiallyFilled,
        "filled" => ReportStatus::Filled,
        "cancelled" => ReportStatus::Cancelled,
        "rejected" => ReportStatus::Rejected,
        "expired" => ReportStatus::Expired,
        "duplicate" => ReportStatus::Duplicate,
        "unknown" => ReportStatus::Unknown,
        _ => return Err(IntakeError::Invalid("status")),
    };
    let fill = r
        .fill
        .as_ref()
        .map(|f| {
            let operation = match f.operation.as_deref().unwrap_or("add") {
                "add" => FillOperation::Add,
                "amend" => FillOperation::Amend,
                "remove" => FillOperation::Remove,
                _ => return Err(IntakeError::Invalid("fill.operation")),
            };
            Ok(FillFact {
                fill_id: f.fill_id.clone(),
                operation,
                quantity: f.quantity,
            })
        })
        .transpose()?;
    Ok(ReportFact {
        seq: 0,
        command_id: r.command_id.map(CommandId::from_uuid),
        origin,
        child_role,
        status,
        reason_code: r.reason_code.clone(),
        fill,
        position_after: r.position_after.as_ref().map(|p| p.quantity),
    })
}

fn scope_for(code: SyncReason, fact: &ReportFact, r: &ExecutionReport) -> ReasonScope {
    match (code, fact.command_id) {
        (
            SyncReason::ExternalActivity
            | SyncReason::FillCorrected
            | SyncReason::BracketIncomplete,
            _,
        )
        | (_, None) => ReasonScope::Instrument(r.instrument.clone()),
        (_, Some(c)) => ReasonScope::Command(c),
    }
}

fn details_for(code: SyncReason, fact: &ReportFact, ctx: &ReportContext) -> ReasonDetails {
    let adapter_code = matches!(
        code,
        SyncReason::CommandRejected | SyncReason::CommandExpired | SyncReason::CancelFailed
    )
    .then(|| fact.reason_code.clone())
    .flatten();
    ReasonDetails {
        command_id: fact.command_id,
        adapter_reason_code: adapter_code,
        expired_by: (code == SyncReason::CommandExpired).then(|| "adapter".to_owned()),
        expected_position: matches!(
            code,
            SyncReason::PositionChanged | SyncReason::FillCorrected
        )
        .then_some(ctx.expected_before)
        .flatten(),
        reported_position: fact
            .position_after
            .filter(|_| code == SyncReason::FillCorrected),
        filled_qty: (code == SyncReason::CommandPartiallyFilled).then_some(ctx.filled_before),
        ..ReasonDetails::default()
    }
}

fn e1_payload(
    r: &ExecutionReport,
    fact: &ReportFact,
    commission: Option<Money>,
    rate: Option<Money>,
    effective: Option<(Money, &'static str)>,
    session: &Session,
) -> Value {
    json!({
        "event_id": r.event_id,
        "account_id": r.account_id,
        "connection_id": session.connection_id,
        "command_id": r.command_id,
        "origin": r.origin,
        "child_role": r.child_role,
        "status": r.status,
        "reason_code": r.reason_code,
        "instrument": r.instrument,
        "side": r.side,
        "position_after": fact.position_after,
        "occurred_at": timestamp(r.occurred_at),
        "platform_time": r.platform_time.map(timestamp),
        "fill": r.fill.as_ref().map(|f| json!({
            "fill_id": f.fill_id,
            "operation": f.operation.as_deref().unwrap_or("add"),
            "quantity": f.quantity,
            "price": f.price,
            "reported_commission": commission.map(|c| c.to_string()),
            "commission_rate": rate.map(|c| c.to_string()),
            "commission_effective": effective.map(|(e, _)| e.to_string()),
            "commission_source": effective.map(|(_, s)| s),
        })),
    })
}
