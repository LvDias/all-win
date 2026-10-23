//! End of a session (ADR-0004 §6 disconnect rows, §6.1 "while unobservable"; Esquadro X1).

use std::sync::Arc;

use chrono::{DateTime, Utc};
use gateway_enterprise::snapshot::ExpectedOrders;
use gateway_enterprise::sync::{ReasonDetails, ReasonEntry, ReasonScope};
use gateway_enterprise::triggers::{AccountAtDisconnect, SessionEnd, on_disconnect};
use serde_json::json;

use crate::events::{self, DisconnectReason, SessionStatus};
use crate::ports::{GatewayStore, StoreError, SyncWrite, UnitOfWork};
use crate::session::{Session, Sessions};

#[derive(Clone)]
pub struct HandleDisconnect {
    store: Arc<dyn GatewayStore>,
    sessions: Sessions,
}

impl HandleDisconnect {
    pub fn new(store: Arc<dyn GatewayStore>, sessions: Sessions) -> Self {
        Self { store, sessions }
    }

    /// `superseded`: closed with 4004 because a new session of the same connection took over;
    /// nothing is recorded for it (the connection stays connected).
    pub async fn handle(
        &self,
        session: &Session,
        end: SessionEnd,
        superseded: bool,
        now: DateTime<Utc>,
    ) -> Result<(), StoreError> {
        if superseded || !self.sessions.remove(session) {
            return Ok(());
        }
        let user = session.user_id;
        let in_flight = self.store.in_flight(user, session.connection_id).await?;
        let mut attempts = 0;
        loop {
            let mut work = UnitOfWork::default();
            for mapped in session.accounts() {
                let account = mapped.account_id;
                let open = self.store.open_positions(user, account).await?;
                let mut orders = ExpectedOrders::default();
                for (instrument, report) in self.store.order_reports(user, account).await? {
                    orders.apply(&instrument, &report);
                }
                let facts = AccountAtDisconnect {
                    position_open: Some(!open.is_empty()),
                    command_in_flight: in_flight.iter().any(|c| c.account_id == account),
                    protective_working: orders.has_protective(),
                    oco_local_or_unknown: mapped.venue.oco_local_or_unknown(),
                };
                let reasons = on_disconnect(end, &facts);
                if reasons.is_empty() {
                    continue;
                }
                let mut sync = self.store.account_sync(user, account).await?;
                let expected_version = sync.version();
                let refs = crate::audit::refs(account, Some(&mapped.display_name), None);
                let mut changed = false;
                for code in reasons {
                    let details = ReasonDetails {
                        expected_position: open.first().map(|(_, q)| *q),
                        ..ReasonDetails::default()
                    };
                    if let Some(change) =
                        sync.raise(ReasonEntry::new(code, ReasonScope::Account, now, details))
                    {
                        work.events
                            .extend(crate::audit::sync_entries(&change, now, &refs));
                        let codes = sync
                            .wire_codes()
                            .iter()
                            .map(|c| c.as_str().to_owned())
                            .collect();
                        work.events.push(events::account_sync_changed(
                            account,
                            &change,
                            codes,
                            json!({ "kind": "session", "ref_id": session.id }),
                            now,
                        ));
                        changed = true;
                    }
                }
                if changed {
                    work.sync.push(SyncWrite {
                        account_id: account,
                        expected_version,
                        next: sync,
                    });
                }
            }
            work.events.push(events::connection_session_changed(
                session.connection_id,
                session.id,
                SessionStatus::Disconnected,
                Some(match end {
                    SessionEnd::ByUser => DisconnectReason::User,
                    SessionEnd::Network => DisconnectReason::Network,
                }),
                Some(session.version.to_string()),
                &session.adapter_version,
                session.instance_id,
                now,
            ));
            match self.store.commit(user, work).await {
                Err(StoreError::Conflict) if attempts < 3 => attempts += 1,
                other => return other.map(|_| ()),
            }
        }
    }
}
