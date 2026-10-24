//! `position_snapshot` handling (ADR-0004 §3 step 3, §6, §6.1). The reconnect snapshot opens
//! delivery for its accounts (Esquadro B1 criterion 2).

use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::ids::{AccountId, CommandId};
use gateway_enterprise::report::{ChildRole, Origin, ReportStatus};
use gateway_enterprise::snapshot::{
    ExpectedAccount, ExpectedOrders, ObservedAccount, ObservedOrder, ObservedOrderState, OrderRef,
    SnapshotTrigger, UnobservedFills, evaluate,
};
use gateway_enterprise::sync::{ReasonDetails, ReasonEntry, ReasonScope, SyncReason};
use serde_json::json;
use uuid::Uuid;

use crate::events;
use crate::ports::{
    AccountBindings, CommitOutcome, GatewayStore, PortError, StoreError, SyncWrite, UnitOfWork,
};
use crate::session::Session;
use crate::wire::PositionSnapshot;

#[derive(Debug, thiserror::Error)]
pub enum SnapshotError {
    #[error("invalid snapshot: {0}")]
    Invalid(&'static str),
    #[error(transparent)]
    Port(#[from] PortError),
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Clone)]
pub struct HandlePositionSnapshot {
    store: Arc<dyn GatewayStore>,
    bindings: Arc<dyn AccountBindings>,
    sessions: crate::session::Sessions,
}

impl HandlePositionSnapshot {
    pub fn new(
        store: Arc<dyn GatewayStore>,
        bindings: Arc<dyn AccountBindings>,
        sessions: crate::session::Sessions,
    ) -> Self {
        Self {
            store,
            bindings,
            sessions,
        }
    }

    /// One transaction for the whole snapshot (receipt + every account's sync change). After
    /// commit, the accounts of a reconnect snapshot are open for delivery.
    pub async fn handle(
        &self,
        session: &Session,
        msg_id: Uuid,
        sent_at: DateTime<Utc>,
        snapshot: &PositionSnapshot,
        now: DateTime<Utc>,
    ) -> Result<CommitOutcome, SnapshotError> {
        let trigger = parse_trigger(&snapshot.trigger)?;
        let user = session.user_id;
        let mut attempts = 0;
        loop {
            let mut work = UnitOfWork {
                receipt: Some((session.connection_id, msg_id)),
                ..UnitOfWork::default()
            };
            let mut evaluated = Vec::new();
            let mut restored = Vec::new();
            for entry in &snapshot.accounts {
                let account_id = AccountId::from_uuid(entry.account_id);
                let Some(mapped) = session.mapped(account_id) else {
                    continue;
                };
                let observed = observed(entry)?;
                let bound =
                    self.bindings.binding_of(user, account_id).await? == Some(mapped.venue.id);
                if !bound {
                    // ADR-0008 §3.2 step 4: audit a non-flat position on an unbound venue account.
                    for (instrument, &qty) in observed.positions.iter().filter(|(_, q)| **q != 0) {
                        work.events.push(events::audit(
                            "execution.unbound_activity",
                            now,
                            json!({
                                "connection_id": session.connection_id,
                                "platform_account": entry.platform_account,
                                "kind": "snapshot",
                                "instrument_symbol": instrument.symbol(),
                                "contract_month": instrument.contract_month(),
                                "position": qty,
                            }),
                            // About no All Win account (contract): no account refs (D3).
                            json!({}),
                        ));
                    }
                    evaluated.push(account_id);
                    continue;
                }
                let expected = self.expected(user, account_id).await?;
                let mut sync = self.store.account_sync(user, account_id).await?;
                // Fills of protective legs received while the account was unobservable (since
                // the active integrity reason was raised): a double fill flags, any fill keeps the
                // integrity reason until the user clears (Esquadro Q2).
                let unobserved = match sync
                    .active()
                    .iter()
                    .find(|r| r.code == SyncReason::BracketIntegrityUnknown)
                    .map(|r| r.detected_at)
                {
                    Some(since) => UnobservedFills(
                        self.store
                            .protective_reports_since(user, account_id, since)
                            .await?
                            .into_iter()
                            .filter(|(_, _, st)| *st == ReportStatus::Filled)
                            .map(|(c, r, _)| (c, r))
                            .collect(),
                    ),
                    None => UnobservedFills::default(),
                };
                let findings = evaluate(
                    trigger,
                    &expected,
                    &observed,
                    mapped.venue.oco_local_or_unknown(),
                    &unobserved,
                );

                let expected_version = sync.version();
                let refs = crate::audit::refs(account_id, Some(&mapped.display_name), None);
                let mut changed = false;
                let mut entries = Vec::new();
                for code in &findings.raise {
                    if *code == SyncReason::SnapshotMismatch
                        && !findings.position_differences.is_empty()
                    {
                        // One reason per contract that differs.
                        for (instrument, expected_qty, reported_qty) in
                            &findings.position_differences
                        {
                            let details = ReasonDetails {
                                snapshot_trigger: Some(trigger_name(trigger).to_owned()),
                                expected_position: Some(*expected_qty),
                                reported_position: Some(*reported_qty),
                                ..ReasonDetails::default()
                            };
                            entries.push(ReasonEntry::new(
                                *code,
                                ReasonScope::Instrument(instrument.clone()),
                                now,
                                details,
                            ));
                        }
                        continue;
                    }
                    let details = ReasonDetails {
                        snapshot_trigger: (*code == SyncReason::SnapshotMismatch)
                            .then(|| trigger_name(trigger).to_owned()),
                        ..ReasonDetails::default()
                    };
                    entries.push(ReasonEntry::new(*code, ReasonScope::Account, now, details));
                }
                for entry in entries {
                    if let Some(change) = sync.raise(entry) {
                        work.events
                            .extend(crate::audit::sync_entries(&change, now, &refs));
                        work.events
                            .push(sync_event(account_id, &change, &sync, msg_id, now));
                        changed = true;
                    }
                }
                for code in &findings.resolve {
                    if let Some(change) = sync.resolve(*code, &ReasonScope::Account, now) {
                        work.events
                            .extend(crate::audit::sync_entries(&change, now, &refs));
                        work.events
                            .push(sync_event(account_id, &change, &sync, msg_id, now));
                        changed = true;
                    }
                }
                // ADR-0004 §6 row 3: flat disconnect, and the reconnect snapshot shows flat with no
                // All Win working orders → back to normal, recorded (`sync.restored`).
                if trigger == SnapshotTrigger::Reconnect
                    && self.sessions.has_flat_disconnect(account_id)
                    && findings.raise.is_empty()
                    && sync.state() == gateway_enterprise::sync::SyncState::InSync
                    && observed.positions.values().all(|q| *q == 0)
                    && !observed
                        .working_orders
                        .iter()
                        .any(|o| o.origin == Origin::AllWin)
                {
                    restored.push(account_id);
                    work.events.extend(crate::audit::restored(
                        sync.version(),
                        session.connection_id,
                        now,
                        refs.clone(),
                    ));
                }
                if changed {
                    work.sync.push(SyncWrite {
                        account_id,
                        expected_version,
                        next: sync,
                    });
                }
                evaluated.push(account_id);
            }
            match self.store.commit(user, work).await {
                Ok(outcome) => {
                    // The marker is consumed only once the entry is committed (Grosa L6), and only
                    // a reconnect snapshot of THIS session opens the command gate: not a
                    // duplicate, not an old one retransmitted from the outbox (Grosa M9).
                    if outcome == CommitOutcome::Committed {
                        for a in restored {
                            self.sessions.take_flat_disconnect(a);
                        }
                        if trigger == SnapshotTrigger::Reconnect
                            && session.is_current_snapshot(sent_at)
                        {
                            for a in evaluated {
                                session.mark_snapshot_evaluated(a);
                            }
                        }
                    }
                    return Ok(outcome);
                }
                Err(StoreError::Conflict) if attempts < 3 => attempts += 1,
                Err(e) => return Err(e.into()),
            }
        }
    }

    async fn expected(
        &self,
        user: shared_kernel::UserId,
        account: AccountId,
    ) -> Result<ExpectedAccount, StoreError> {
        let positions: BTreeMap<_, _> = self
            .store
            .open_positions(user, account)
            .await?
            .into_iter()
            .collect();
        let mut orders = ExpectedOrders::default();
        for (instrument, report, kind) in self.store.order_reports(user, account).await? {
            orders.apply(&instrument, &report, kind);
        }
        Ok(ExpectedAccount { positions, orders })
    }
}

fn sync_event(
    account: AccountId,
    change: &gateway_enterprise::sync::SyncChange,
    sync: &gateway_enterprise::sync::AccountSync,
    msg_id: Uuid,
    now: DateTime<Utc>,
) -> events::GatewayEvent {
    let codes = sync
        .wire_codes()
        .iter()
        .map(|c| c.as_str().to_owned())
        .collect();
    events::account_sync_changed(
        account,
        change,
        codes,
        json!({ "kind": "position_snapshot", "ref_id": msg_id }),
        now,
    )
}

fn parse_trigger(t: &str) -> Result<SnapshotTrigger, SnapshotError> {
    Ok(match t {
        "reconnect" => SnapshotTrigger::Reconnect,
        "periodic" => SnapshotTrigger::Periodic,
        "change" => SnapshotTrigger::Change,
        "requested" => SnapshotTrigger::Requested,
        "account_reset" => SnapshotTrigger::AccountReset,
        _ => return Err(SnapshotError::Invalid("trigger")),
    })
}

fn trigger_name(t: SnapshotTrigger) -> &'static str {
    // The HTTP detail only distinguishes reconnect from the rest (periodic).
    if t == SnapshotTrigger::Reconnect {
        "reconnect"
    } else {
        "periodic"
    }
}

fn observed(a: &crate::wire::SnapshotAccount) -> Result<ObservedAccount, SnapshotError> {
    let positions = a
        .positions
        .iter()
        .map(|p| (p.instrument.clone(), p.position.quantity))
        .collect();
    let working_orders = a
        .working_orders
        .iter()
        .map(|o| {
            let origin = match o.origin.as_str() {
                "all_win" => Origin::AllWin,
                "local_enforcement" => Origin::LocalEnforcement,
                "external" => Origin::External,
                _ => return Err(SnapshotError::Invalid("origin")),
            };
            let role = match o.child_role.as_deref() {
                None => None,
                Some("entry") => Some(ChildRole::Entry),
                Some("stop") => Some(ChildRole::Stop),
                Some("target") => Some(ChildRole::Target),
                Some("exit") => Some(ChildRole::Exit),
                Some("flatten") => Some(ChildRole::Flatten),
                Some(_) => return Err(SnapshotError::Invalid("child_role")),
            };
            let reference = match (o.command_id, role) {
                (Some(c), Some(r)) => Some(OrderRef {
                    command_id: CommandId::from_uuid(c),
                    child_role: r,
                }),
                _ => None,
            };
            let state = match o.state.as_deref() {
                None => None,
                Some("accepted") => Some(ObservedOrderState::Accepted),
                Some("working") => Some(ObservedOrderState::Working),
                Some("trigger_pending") => Some(ObservedOrderState::TriggerPending),
                Some("unknown") => Some(ObservedOrderState::Unknown),
                Some(_) => return Err(SnapshotError::Invalid("state")),
            };
            Ok(ObservedOrder {
                instrument: o.instrument.clone(),
                origin,
                reference,
                state,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ObservedAccount {
        connected: a.connected,
        positions,
        working_orders,
    })
}
