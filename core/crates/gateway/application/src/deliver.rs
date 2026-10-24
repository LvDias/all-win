//! Command delivery loop for one session (ADR-0004 §4–§5; Esquadro B1 acceptance criteria).
//!
//! Order per command: decide (pure) → for a first send, commit `pending → sent` → only then
//! hand the frame to the caller to write. A crash after the commit counts as sent, so the
//! command is retransmitted and never qualifies for the reverse expiry exemption. Commands are
//! taken in creation order (`created_at`, then `msg_id`, a UUIDv7 allocated in routing's list
//! order), so a reverse's flatten always goes before its place (Grosa H1).

use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::command::{
    CommandKind, CommandState, Delivery, DeliveryContext, Prerequisite, RETRANSMISSION_HORIZON,
};
use gateway_enterprise::position::flat_proven;
use gateway_enterprise::report::{ChildRole, ReportFact, ReportStatus};
use gateway_enterprise::sync::{ReasonDetails, ReasonEntry, ReasonScope, SyncReason, SyncState};
use serde_json::{Value, json};

use crate::events::{self, GatewayEvent};
use crate::ports::{
    AccountBindings, CommandTransition, GatewayStore, StoreError, StoredCommand, SyncWrite,
    UnitOfWork,
};
use crate::session::{Session, SessionSettings};

/// The protocol code the plugin uses for the same refusal (ADR-0008 §3.6, Esquadro C-c).
const NOT_SIMULATION: &str = "command.account_not_simulation";

/// Notification registry code for a reverse that closed and did not open (HTTP 5.1.1).
pub const REVERSE_CLOSE_ONLY: &str = "routing.reverse_close_only";

/// A frame the caller must write now, in this order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outgoing {
    pub msg_id: uuid::Uuid,
    pub frame: String,
}

/// One pass: the frames to write, in order, and the commands that could not be processed this
/// time (they are retried at the next pass; one failure never drops the others, Grosa M3).
#[derive(Debug, Default)]
pub struct Pass {
    pub outgoing: Vec<Outgoing>,
    pub failures: Vec<StoreError>,
}

#[derive(Clone)]
pub struct DeliverCommands {
    store: Arc<dyn GatewayStore>,
    bindings: Arc<dyn AccountBindings>,
    settings: SessionSettings,
    /// A sent flatten's normal report timeout, after which a held reverse leg is decided.
    prerequisite_timeout: chrono::Duration,
}

/// Terminal outcome decided by the Core for a command (expiry, horizon, refusal, adapter error).
pub(crate) struct Finish {
    pub to: CommandState,
    pub reason: Option<(SyncReason, ReasonDetails)>,
    pub events: Vec<GatewayEvent>,
}

impl DeliverCommands {
    pub fn new(
        store: Arc<dyn GatewayStore>,
        bindings: Arc<dyn AccountBindings>,
        settings: SessionSettings,
        prerequisite_timeout: chrono::Duration,
    ) -> Self {
        Self {
            store,
            bindings,
            settings,
            prerequisite_timeout,
        }
    }

    /// Reliable outbound messages first (original order), then commands. A store error on the
    /// reads aborts the pass (nothing was decided yet).
    pub async fn handle(&self, session: &Session, now: DateTime<Utc>) -> Result<Pass, StoreError> {
        let mut pass = Pass::default();
        for m in self
            .store
            .unacked_outbound(session.user_id, session.connection_id)
            .await?
        {
            if m.protocol_version <= session.version
                && session.write_due(m.msg_id, now, self.settings.ack_timeout_ms)
            {
                pass.outgoing.push(Outgoing {
                    msg_id: m.msg_id,
                    frame: m.frame,
                });
            }
        }
        for stored in self
            .store
            .open_commands(session.user_id, session.connection_id)
            .await?
        {
            match self.deliver_one(session, stored, now).await {
                Ok(Some(o)) => pass.outgoing.push(o),
                Ok(None) => {}
                Err(e) => pass.failures.push(e),
            }
        }
        Ok(pass)
    }

    async fn deliver_one(
        &self,
        session: &Session,
        stored: StoredCommand,
        now: DateTime<Utc>,
    ) -> Result<Option<Outgoing>, StoreError> {
        let cmd = &stored.command;
        let mapped = session.mapped(stored.account_id);
        let sync = self
            .store
            .account_sync(session.user_id, stored.account_id)
            .await?;
        let out_of_sync = sync.state() == SyncState::OutOfSync;
        let ctx = DeliveryContext {
            negotiated: Some(session.version),
            account_in_welcome: mapped.is_some(),
            snapshot_evaluated: session.snapshot_evaluated(stored.account_id),
            prerequisite: self.prerequisite(session, &stored, now).await?,
            other_sync_reason_open: out_of_sync,
            refused_by_environment: session.refused_by_environment(stored.account_id),
            account_out_of_sync: out_of_sync,
        };
        let decision = cmd.delivery(now, &ctx);
        // Read only when the pass writes an entry (not on every wait or retransmission).
        let refs = match decision {
            Delivery::Wait(_) | Delivery::Done => Value::Null,
            Delivery::Send if cmd.state != CommandState::Pending => Value::Null,
            // A first send passed the session gate, so the welcome maps the account: no accounts
            // read on the send path.
            Delivery::Send if mapped.is_some() => crate::audit::refs(
                stored.account_id,
                mapped.map(|m| m.display_name.as_str()),
                Some(stored.instruction_id),
            ),
            _ => {
                crate::audit::command_refs(
                    &*self.bindings,
                    session,
                    stored.account_id,
                    stored.instruction_id,
                )
                .await?
            }
        };
        match decision {
            Delivery::Send => {
                if cmd.state == CommandState::Pending {
                    let sent = crate::audit::command_sent(
                        cmd.kind,
                        &stored.step,
                        cmd.expires_at,
                        now,
                        refs,
                    );
                    // Commit first (AC-1). Lost race, or expired by the database clock (L1) → skip.
                    if !self
                        .store
                        .mark_sent(session.user_id, cmd.id, now, vec![sent])
                        .await?
                    {
                        return Ok(None);
                    }
                    session.write_due(stored.msg_id, now, self.settings.ack_timeout_ms);
                    return Ok(Some(Outgoing {
                        msg_id: stored.msg_id,
                        frame: stored.frame,
                    }));
                }
                Ok(session
                    .write_due(stored.msg_id, now, self.settings.ack_timeout_ms)
                    .then_some(Outgoing {
                        msg_id: stored.msg_id,
                        frame: stored.frame,
                    }))
            }
            Delivery::Expire {
                flag,
                reverse_close_only,
            } => {
                let mut events = vec![events::audit(
                    "execution.expired",
                    now,
                    json!({ "origin": "all_win", "platform_reason": "command.expired" }),
                    refs.clone(),
                )];
                if reverse_close_only
                    && let Some(n) = self.reverse_close_only(&stored, &events[0], now).await?
                {
                    events.push(n);
                }
                let reason = flag.then(|| {
                    (
                        SyncReason::CommandExpired,
                        ReasonDetails {
                            command_id: Some(cmd.id),
                            expired_by: Some("core".into()),
                            ..ReasonDetails::default()
                        },
                    )
                });
                finish(
                    &*self.store,
                    &stored,
                    Finish {
                        to: CommandState::Expired,
                        reason,
                        events,
                    },
                    &refs,
                    now,
                )
                .await?;
                Ok(None)
            }
            Delivery::ExpireOutOfSync => {
                // Esquadro Q3: no send, no new reason (the account is already flagged).
                let events = vec![events::audit(
                    "execution.expired",
                    now,
                    json!({ "origin": "all_win", "platform_reason": "routing.out_of_sync" }),
                    refs.clone(),
                )];
                finish(
                    &*self.store,
                    &stored,
                    Finish {
                        to: CommandState::Expired,
                        reason: None,
                        events,
                    },
                    &refs,
                    now,
                )
                .await?;
                Ok(None)
            }
            Delivery::Undeliver => {
                let events = vec![events::audit(
                    "execution.undelivered",
                    now,
                    json!({
                        "command_id": cmd.id,
                        "created_at": crate::wire::timestamp(cmd.created_at),
                        "horizon_h": RETRANSMISSION_HORIZON.num_hours(),
                    }),
                    refs.clone(),
                )];
                let reason = Some((
                    SyncReason::CommandUndelivered,
                    ReasonDetails {
                        command_id: Some(cmd.id),
                        ..ReasonDetails::default()
                    },
                ));
                finish(
                    &*self.store,
                    &stored,
                    Finish {
                        to: CommandState::Undelivered,
                        reason,
                        events,
                    },
                    &refs,
                    now,
                )
                .await?;
                Ok(None)
            }
            Delivery::Refuse => {
                let events = vec![events::audit(
                    "execution.rejected",
                    now,
                    json!({ "origin": "all_win", "platform_reason": NOT_SIMULATION }),
                    refs.clone(),
                )];
                let reason = Some((
                    SyncReason::CommandRejected,
                    ReasonDetails {
                        command_id: Some(cmd.id),
                        adapter_reason_code: Some(NOT_SIMULATION.into()),
                        ..ReasonDetails::default()
                    },
                ));
                finish(
                    &*self.store,
                    &stored,
                    Finish {
                        to: CommandState::Rejected,
                        reason,
                        events,
                    },
                    &refs,
                    now,
                )
                .await?;
                Ok(None)
            }
            Delivery::Wait(_) | Delivery::Done => Ok(None),
        }
    }

    async fn prerequisite(
        &self,
        session: &Session,
        stored: &StoredCommand,
        now: DateTime<Utc>,
    ) -> Result<Prerequisite, StoreError> {
        let Some(dep_id) = stored.command.depends_on else {
            return Ok(Prerequisite::None);
        };
        let Some(dep) = self.store.command(session.user_id, dep_id).await? else {
            return Ok(Prerequisite::Failed);
        };
        if !dep.command.state.is_final() {
            // A flatten that never left the Core is still pending, however long the outage
            // was: the normal report timeout only starts at its send (Grosa M1).
            return Ok(match dep.command.sent_at {
                Some(sent) if now - sent >= self.prerequisite_timeout => Prerequisite::TimedOut,
                _ => Prerequisite::NotTerminal,
            });
        }
        if dep.command.state != CommandState::Completed || dep.command.kind != CommandKind::Flatten
        {
            return Ok(Prerequisite::Failed);
        }
        let reports = self.store.command_reports(session.user_id, dep_id).await?;
        let Some(terminal) = flatten_terminal(&reports) else {
            return Ok(Prerequisite::Failed);
        };
        let history = self
            .store
            .position_history(session.user_id, dep.account_id, &dep.instrument)
            .await?;
        Ok(Prerequisite::Succeeded {
            flat_proven: flat_proven(&history, terminal.seq),
        })
    }

    /// `routing.reverse_close_only` for an exempt expired open leg (ADR-0004 §5; Compasso and
    /// Esquadro: one outbox event, type = the registry code, in the audit's transaction).
    /// `None` when the flatten closed nothing (`already_flat`): the params require
    /// `closed_qty ≥ 1`.
    async fn reverse_close_only(
        &self,
        stored: &StoredCommand,
        audit: &GatewayEvent,
        now: DateTime<Utc>,
    ) -> Result<Option<GatewayEvent>, StoreError> {
        let Some(dep_id) = stored.command.depends_on else {
            return Ok(None);
        };
        let reports = self.store.command_reports(stored.user_id, dep_id).await?;
        let closed: i64 = reports
            .iter()
            .filter(|r| r.status != ReportStatus::Duplicate)
            .filter_map(|r| r.fill.as_ref().map(|f| i64::from(f.quantity)))
            .sum();
        if closed < 1 {
            return Ok(None);
        }
        // The closed side: the position just before the flatten's first fill.
        let Some(first_fill) = reports.iter().find(|r| r.fill.is_some()).map(|r| r.seq) else {
            return Ok(None);
        };
        let Some(dep) = self.store.command(stored.user_id, dep_id).await? else {
            return Ok(None);
        };
        let before = self
            .store
            .position_before(stored.user_id, dep.account_id, &dep.instrument, first_fill)
            .await?
            .unwrap_or(0);
        let side = if before < 0 { "short" } else { "long" };
        Ok(Some(events::notification(
            REVERSE_CLOSE_ONLY,
            now,
            stored.account_id,
            json!({ "kind": "instruction", "id": stored.instruction_id }),
            audit.id,
            json!({ "reason_code": "routing.expired", "closed_qty": closed, "closed_side": side }),
        )))
    }
}

fn flatten_terminal(reports: &[ReportFact]) -> Option<&ReportFact> {
    reports.iter().rev().find(|r| {
        matches!(
            r.child_role,
            None | Some(ChildRole::Flatten | ChildRole::Exit)
        ) && matches!(r.status, ReportStatus::Filled | ReportStatus::Cancelled)
    })
}

/// Commits a Core-decided terminal state with its sync reason and events; re-reads the command
/// and the sync state on a conflict (Grosa M3).
pub(crate) async fn finish(
    store: &dyn GatewayStore,
    stored: &StoredCommand,
    outcome: Finish,
    refs: &Value,
    now: DateTime<Utc>,
) -> Result<(), StoreError> {
    let mut attempts = 0;
    loop {
        let Some(current) = store.command(stored.user_id, stored.command.id).await? else {
            return Ok(());
        };
        if current.command.state.is_final() {
            return Ok(());
        }
        let mut sync = store
            .account_sync(stored.user_id, stored.account_id)
            .await?;
        let expected_version = sync.version();
        let mut events = outcome.events.clone();
        let mut writes = Vec::new();
        if let Some((code, details)) = &outcome.reason {
            let entry = ReasonEntry::new(
                *code,
                ReasonScope::Command(stored.command.id),
                now,
                details.clone(),
            );
            if let Some(change) = sync.raise(entry) {
                let codes = sync
                    .wire_codes()
                    .iter()
                    .map(|c| c.as_str().to_owned())
                    .collect();
                events.extend(crate::audit::sync_entries(&change, now, refs));
                events.push(events::account_sync_changed(
                    stored.account_id,
                    &change,
                    codes,
                    json!({ "kind": "command", "ref_id": stored.command.id }),
                    now,
                ));
                writes.push(SyncWrite {
                    account_id: stored.account_id,
                    expected_version,
                    next: sync,
                });
            }
        }
        let work = UnitOfWork {
            commands: vec![CommandTransition {
                id: stored.command.id,
                from: current.command.state,
                to: outcome.to,
                at: now,
            }],
            sync: writes,
            events,
            ..UnitOfWork::default()
        };
        match store.commit(stored.user_id, work).await {
            Err(StoreError::Conflict) if attempts < 3 => attempts += 1,
            other => return other.map(|_| ()),
        }
    }
}
