//! Command delivery loop for one session (ADR-0004 §4–§5; Esquadro B1 acceptance criteria).
//!
//! Order per command: decide (pure) → for a first send, commit `pending → sent` → only then
//! hand the frame to the caller to write. A crash after the commit counts as sent, so the
//! command is retransmitted and never qualifies for the reverse expiry exemption.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::command::{
    CommandKind, CommandState, Delivery, DeliveryContext, Prerequisite, RETRANSMISSION_HORIZON,
};

/// The protocol code the plugin uses for the same refusal (ADR-0008 §3.6, Esquadro C-c).
const NOT_SIMULATION: &str = "command.account_not_simulation";
use gateway_enterprise::position::flat_proven;
use gateway_enterprise::report::{ChildRole, ReportStatus};
use gateway_enterprise::sync::{ReasonDetails, ReasonEntry, ReasonScope, SyncReason, SyncState};
use serde_json::json;

use crate::events::{self, GatewayEvent};
use crate::ports::{
    CommandTransition, GatewayStore, StoreError, StoredCommand, SyncWrite, UnitOfWork,
};
use crate::session::{Session, SessionSettings};

/// A frame the caller must write now, in this order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outgoing {
    pub msg_id: uuid::Uuid,
    pub frame: String,
}

#[derive(Clone)]
pub struct DeliverCommands {
    store: Arc<dyn GatewayStore>,
    settings: SessionSettings,
    /// A flatten's normal report timeout, after which a held reverse leg is decided.
    prerequisite_timeout: chrono::Duration,
}

impl DeliverCommands {
    pub fn new(
        store: Arc<dyn GatewayStore>,
        settings: SessionSettings,
        prerequisite_timeout: chrono::Duration,
    ) -> Self {
        Self {
            store,
            settings,
            prerequisite_timeout,
        }
    }

    /// One pass: reliable outbound messages first (original order), then commands.
    pub async fn handle(
        &self,
        session: &Session,
        now: DateTime<Utc>,
    ) -> Result<Vec<Outgoing>, StoreError> {
        let mut out = Vec::new();
        for m in self
            .store
            .unacked_outbound(session.user_id, session.connection_id)
            .await?
        {
            if m.protocol_version <= session.version
                && session.write_due(m.msg_id, now, self.settings.ack_timeout_ms)
            {
                out.push(Outgoing {
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
            if let Some(o) = self.deliver_one(session, stored, now).await? {
                out.push(o);
            }
        }
        Ok(out)
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
        let ctx = DeliveryContext {
            negotiated: Some(session.version),
            account_in_welcome: mapped.is_some(),
            snapshot_evaluated: session.snapshot_evaluated(stored.account_id),
            prerequisite: self.prerequisite(session, &stored, now).await?,
            other_sync_reason_open: sync.state() == SyncState::OutOfSync,
            refused_by_environment: session.refused_by_environment(stored.account_id),
        };
        match cmd.delivery(now, &ctx) {
            Delivery::Send => {
                if cmd.state == CommandState::Pending {
                    // Commit first (AC-1). Lost race (already sent/expired elsewhere) → skip.
                    let refs = crate::audit::refs(
                        stored.account_id,
                        mapped.map(|m| m.display_name.as_str()),
                        Some(stored.instruction_id),
                    );
                    let sent = crate::audit::command_sent(
                        cmd.kind,
                        &stored.step,
                        cmd.expires_at,
                        now,
                        refs,
                    );
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
                self.finish(
                    &stored,
                    CommandState::Expired,
                    flag.then_some(SyncReason::CommandExpired),
                    reverse_close_only,
                    now,
                )
                .await?;
                Ok(None)
            }
            Delivery::Undeliver => {
                self.finish(
                    &stored,
                    CommandState::Undelivered,
                    Some(SyncReason::CommandUndelivered),
                    false,
                    now,
                )
                .await?;
                Ok(None)
            }
            Delivery::Refuse => {
                self.finish(
                    &stored,
                    CommandState::Rejected,
                    Some(SyncReason::CommandRejected),
                    false,
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
            let started = dep.command.sent_at.unwrap_or(dep.command.created_at);
            return Ok(if now - started >= self.prerequisite_timeout {
                Prerequisite::TimedOut
            } else {
                Prerequisite::NotTerminal
            });
        }
        if dep.command.state != CommandState::Completed || dep.command.kind != CommandKind::Flatten
        {
            return Ok(Prerequisite::Failed);
        }
        let reports = self.store.command_reports(session.user_id, dep_id).await?;
        let terminal = reports.iter().rev().find(|r| {
            matches!(
                r.child_role,
                None | Some(ChildRole::Flatten | ChildRole::Exit)
            ) && matches!(r.status, ReportStatus::Filled | ReportStatus::Cancelled)
        });
        let Some(terminal) = terminal else {
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

    async fn finish(
        &self,
        stored: &StoredCommand,
        to: CommandState,
        reason: Option<SyncReason>,
        _reverse_close_only: bool,
        now: DateTime<Utc>,
    ) -> Result<(), StoreError> {
        let mut attempts = 0;
        loop {
            let mut sync = self
                .store
                .account_sync(stored.user_id, stored.account_id)
                .await?;
            let expected_version = sync.version();
            let refs = crate::audit::refs(stored.account_id, None, Some(stored.instruction_id));
            let mut events: Vec<GatewayEvent> = vec![match to {
                CommandState::Expired => events::audit(
                    "execution.expired",
                    now,
                    json!({ "origin": "all_win", "platform_reason": "command.expired" }),
                    refs.clone(),
                ),
                CommandState::Rejected => events::audit(
                    "execution.rejected",
                    now,
                    json!({ "origin": "all_win", "platform_reason": NOT_SIMULATION }),
                    refs.clone(),
                ),
                _ => events::audit(
                    "execution.undelivered",
                    now,
                    json!({
                        "command_id": stored.command.id,
                        "created_at": crate::wire::timestamp(stored.command.created_at),
                        "horizon_h": RETRANSMISSION_HORIZON.num_hours(),
                    }),
                    refs.clone(),
                ),
            }];
            let mut writes = Vec::new();
            if let Some(code) = reason {
                let details = ReasonDetails {
                    command_id: Some(stored.command.id),
                    expired_by: (code == SyncReason::CommandExpired).then(|| "core".to_owned()),
                    adapter_reason_code: (code == SyncReason::CommandRejected)
                        .then(|| NOT_SIMULATION.to_owned()),
                    ..ReasonDetails::default()
                };
                let entry =
                    ReasonEntry::new(code, ReasonScope::Command(stored.command.id), now, details);
                if let Some(change) = sync.raise(entry) {
                    let codes = sync
                        .wire_codes()
                        .iter()
                        .map(|c| c.as_str().to_owned())
                        .collect();
                    events.extend(crate::audit::sync_entries(&change, now, &refs));
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
                    from: stored.command.state,
                    to,
                    at: now,
                }],
                sync: writes,
                events,
                ..UnitOfWork::default()
            };
            match self.store.commit(stored.user_id, work).await {
                Err(StoreError::Conflict) if attempts < 3 => attempts += 1,
                other => return other.map(|_| ()),
            }
        }
    }
}
