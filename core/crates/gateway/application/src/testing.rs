//! In-memory fakes for the use-case tests.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use gateway_enterprise::command::{CommandKind, CommandState};
use gateway_enterprise::ids::{AccountId, CommandId, ConnectionId, VenueAccountId};
use gateway_enterprise::instrument::Instrument;
use gateway_enterprise::position::ExpectedPosition;
use gateway_enterprise::report::{ChildRole, FillOperation, ReportFact, ReportStatus};
use gateway_enterprise::sync::AccountSync;
use shared_kernel::{Money, UserId};
use uuid::Uuid;

use crate::events::GatewayEvent;
use crate::ports::*;

#[derive(Default)]
pub struct FakeStore {
    pub inner: Mutex<StoreState>,
    /// Fail every commit (port/store failure injection).
    pub fail_commits: Mutex<bool>,
}

#[derive(Default)]
pub struct StoreState {
    pub venues: Vec<(ConnectionId, VenueAccount)>,
    pub commands: BTreeMap<CommandId, StoredCommand>,
    pub command_order: Vec<CommandId>,
    pub acked: HashSet<Uuid>,
    pub outbound: Vec<OutboundMessage>,
    pub receipts: HashSet<(ConnectionId, Uuid)>,
    pub reports: Vec<(AccountId, Option<Instrument>, ReportFact)>,
    /// Arrival time of each report (`occurred_at` here), parallel to `reports`.
    pub report_times: Vec<DateTime<Utc>>,
    pub event_ids: HashSet<Uuid>,
    pub fills: HashSet<(AccountId, String)>,
    pub sync: HashMap<AccountId, AccountSync>,
    pub events: Vec<GatewayEvent>,
    pub deferred: Vec<DeferredInbound>,
    pub seq: i64,
}

impl FakeStore {
    pub fn state(&self) -> std::sync::MutexGuard<'_, StoreState> {
        self.inner.lock().unwrap()
    }

    pub fn event_types(&self) -> Vec<&'static str> {
        self.state().events.iter().map(|e| e.event_type).collect()
    }

    pub fn add_venue(&self, connection: ConnectionId, venue: VenueAccount) {
        self.state().venues.push((connection, venue));
    }
}

#[async_trait]
impl GatewayStore for FakeStore {
    async fn upsert_venue_accounts(
        &self,
        _user_id: UserId,
        connection: ConnectionId,
        accounts: &[VisiblePlatformAccount],
        _seen_at: DateTime<Utc>,
    ) -> Result<Vec<VenueAccount>, StoreError> {
        let mut s = self.state();
        let mut out = Vec::new();
        for a in accounts {
            let existing = s
                .venues
                .iter_mut()
                .find(|(c, v)| *c == connection && v.platform_account == a.platform_account);
            let row = match existing {
                Some((_, v)) => {
                    v.previous_kind = (v.kind != a.kind).then(|| v.kind.clone());
                    v.kind = a.kind.clone();
                    v.oco_location = a.oco_location.clone();
                    v.clone()
                }
                None => {
                    let v = VenueAccount {
                        id: VenueAccountId::from_uuid(Uuid::now_v7()),
                        platform_account: a.platform_account.clone(),
                        kind: a.kind.clone(),
                        oco_location: a.oco_location.clone(),
                        previous_kind: None,
                    };
                    s.venues.push((connection, v.clone()));
                    v
                }
            };
            out.push(row);
        }
        Ok(out)
    }

    async fn insert_commands(
        &self,
        _user_id: UserId,
        commands: &[NewCommand],
    ) -> Result<usize, StoreError> {
        let mut s = self.state();
        let mut new = 0;
        for c in commands {
            let id = c.stored.command.id;
            if let std::collections::btree_map::Entry::Vacant(e) = s.commands.entry(id) {
                e.insert(c.stored.clone());
                s.command_order.push(id);
                new += 1;
            }
        }
        Ok(new)
    }

    async fn open_commands(
        &self,
        _user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<StoredCommand>, StoreError> {
        let s = self.state();
        // Same order as the PG store: created_at, then msg_id (Grosa H1).
        let mut v: Vec<StoredCommand> = s
            .commands
            .values()
            .filter(|c| c.connection_id == connection && c.command.state.awaits_ack())
            .cloned()
            .collect();
        v.sort_by_key(|c| (c.command.created_at, c.msg_id));
        Ok(v)
    }

    async fn command(
        &self,
        _user_id: UserId,
        id: CommandId,
    ) -> Result<Option<StoredCommand>, StoreError> {
        Ok(self.state().commands.get(&id).cloned())
    }

    async fn in_flight(
        &self,
        _user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<StoredCommand>, StoreError> {
        Ok(self
            .state()
            .commands
            .values()
            .filter(|c| {
                c.connection_id == connection
                    && matches!(c.command.state, CommandState::Sent | CommandState::Acked)
            })
            .cloned()
            .collect())
    }

    async fn venue_account(
        &self,
        _user_id: UserId,
        id: VenueAccountId,
    ) -> Result<Option<(ConnectionId, VenueAccount)>, StoreError> {
        Ok(self
            .state()
            .venues
            .iter()
            .find(|(_, v)| v.id == id)
            .cloned())
    }

    async fn mark_sent(
        &self,
        _user_id: UserId,
        id: CommandId,
        at: DateTime<Utc>,
        events: Vec<GatewayEvent>,
    ) -> Result<bool, StoreError> {
        let mut s = self.state();
        let Some(c) = s.commands.get_mut(&id) else {
            return Ok(false);
        };
        if c.command.state != CommandState::Pending {
            return Ok(false);
        }
        c.command.state = CommandState::Sent;
        c.command.sent_at = Some(at);
        s.events.extend(events);
        Ok(true)
    }

    async fn ack(
        &self,
        _user_id: UserId,
        _connection: ConnectionId,
        msg_ids: &[Uuid],
        _at: DateTime<Utc>,
    ) -> Result<(), StoreError> {
        let mut s = self.state();
        for id in msg_ids {
            s.acked.insert(*id);
            if let Some(c) = s.commands.values_mut().find(|c| c.msg_id == *id)
                && c.command.state == CommandState::Sent
            {
                c.command.state = CommandState::Acked;
            }
        }
        Ok(())
    }

    async fn enqueue_outbound(&self, message: &OutboundMessage) -> Result<(), StoreError> {
        self.state().outbound.push(message.clone());
        Ok(())
    }

    async fn unacked_outbound(
        &self,
        _user_id: UserId,
        connection: ConnectionId,
    ) -> Result<Vec<OutboundMessage>, StoreError> {
        let s = self.state();
        Ok(s.outbound
            .iter()
            .filter(|m| m.connection_id == connection && !s.acked.contains(&m.msg_id))
            .cloned()
            .collect())
    }

    async fn position_history(
        &self,
        _user_id: UserId,
        account: AccountId,
        instrument: &Instrument,
    ) -> Result<Vec<ReportFact>, StoreError> {
        let s = self.state();
        let all: Vec<ReportFact> = s
            .reports
            .iter()
            .filter(|(a, i, _)| *a == account && i.as_ref() == Some(instrument))
            .map(|(_, _, f)| f.clone())
            .collect();
        let start = all
            .iter()
            .rposition(|f| f.position_after == Some(0))
            .unwrap_or(0);
        Ok(all[start..].to_vec())
    }

    async fn open_positions(
        &self,
        user_id: UserId,
        account: AccountId,
    ) -> Result<Vec<(Instrument, i64)>, StoreError> {
        let instruments: Vec<Instrument> = {
            let s = self.state();
            let mut v: Vec<Instrument> = s
                .reports
                .iter()
                .filter(|(a, _, _)| *a == account)
                .filter_map(|(_, i, _)| i.clone())
                .collect();
            v.sort();
            v.dedup();
            v
        };
        let mut out = Vec::new();
        for i in instruments {
            let h = self.position_history(user_id, account, &i).await?;
            if let Some(q) = ExpectedPosition::flat()
                .replay(&h)
                .quantity()
                .filter(|q| *q != 0)
            {
                out.push((i, q));
            }
        }
        Ok(out)
    }

    async fn order_reports(
        &self,
        _user_id: UserId,
        account: AccountId,
    ) -> Result<Vec<(Instrument, ReportFact, Option<CommandKind>)>, StoreError> {
        let s = self.state();
        Ok(s.reports
            .iter()
            .filter(|(a, _, _)| *a == account)
            .filter_map(|(_, i, f)| {
                let kind = f
                    .command_id
                    .and_then(|c| s.commands.get(&c))
                    .map(|c| c.command.kind);
                i.clone().map(|i| (i, f.clone(), kind))
            })
            .collect())
    }

    async fn receipt_known(
        &self,
        _user_id: UserId,
        connection: ConnectionId,
        msg_id: Uuid,
    ) -> Result<bool, StoreError> {
        Ok(self.state().receipts.contains(&(connection, msg_id)))
    }

    async fn command_by_msg(
        &self,
        _user_id: UserId,
        connection: ConnectionId,
        msg_id: Uuid,
    ) -> Result<Option<StoredCommand>, StoreError> {
        Ok(self
            .state()
            .commands
            .values()
            .find(|c| c.connection_id == connection && c.msg_id == msg_id)
            .cloned())
    }

    async fn protective_reports_since(
        &self,
        _user_id: UserId,
        account: AccountId,
        since: DateTime<Utc>,
    ) -> Result<Vec<(CommandId, ChildRole, ReportStatus)>, StoreError> {
        let s = self.state();
        Ok(s.reports
            .iter()
            .zip(s.report_times.iter())
            .filter(|((a, _, f), at)| {
                *a == account && **at >= since && f.child_role.is_some_and(ChildRole::is_protective)
            })
            .filter_map(|((_, _, f), _)| Some((f.command_id?, f.child_role?, f.status)))
            .collect())
    }

    async fn position_before(
        &self,
        _user_id: UserId,
        account: AccountId,
        instrument: &Instrument,
        seq: i64,
    ) -> Result<Option<i64>, StoreError> {
        Ok(self
            .state()
            .reports
            .iter()
            .filter(|(a, i, f)| *a == account && i.as_ref() == Some(instrument) && f.seq < seq)
            .filter_map(|(_, _, f)| f.position_after)
            .next_back())
    }

    async fn command_reports(
        &self,
        _user_id: UserId,
        command: CommandId,
    ) -> Result<Vec<ReportFact>, StoreError> {
        Ok(self
            .state()
            .reports
            .iter()
            .filter(|(_, _, f)| f.command_id == Some(command))
            .map(|(_, _, f)| f.clone())
            .collect())
    }

    async fn fill_known(
        &self,
        _user_id: UserId,
        account: AccountId,
        fill_id: &str,
    ) -> Result<bool, StoreError> {
        Ok(self.state().fills.contains(&(account, fill_id.to_owned())))
    }

    async fn account_sync(
        &self,
        _user_id: UserId,
        account: AccountId,
    ) -> Result<AccountSync, StoreError> {
        Ok(self
            .state()
            .sync
            .get(&account)
            .cloned()
            .unwrap_or_else(AccountSync::new_in_sync))
    }

    async fn commit(
        &self,
        _user_id: UserId,
        work: UnitOfWork,
    ) -> Result<CommitOutcome, StoreError> {
        if *self.fail_commits.lock().unwrap() {
            return Err(StoreError::Other("injected".into()));
        }
        let mut s = self.state();
        if let Some(r) = work.receipt
            && s.receipts.contains(&r)
        {
            return Ok(CommitOutcome::Duplicate);
        }
        if let Some(report) = &work.report {
            if s.event_ids.contains(&report.event_id.as_uuid()) {
                return Ok(CommitOutcome::Duplicate);
            }
            if let Some(f) = &report.fact.fill
                && f.operation == FillOperation::Add
                && s.fills.contains(&(report.account_id, f.fill_id.clone()))
            {
                return Ok(CommitOutcome::Duplicate);
            }
        }
        for w in &work.sync {
            let current = s.sync.get(&w.account_id).map_or(1, AccountSync::version);
            if current != w.expected_version {
                return Err(StoreError::Conflict);
            }
        }
        for t in &work.commands {
            if s.commands.get(&t.id).map(|c| c.command.state) != Some(t.from) {
                return Err(StoreError::Conflict);
            }
        }
        if let Some(r) = work.receipt {
            s.receipts.insert(r);
        }
        if let Some(report) = work.report {
            s.seq += 1;
            let mut fact = report.fact.clone();
            fact.seq = s.seq;
            s.event_ids.insert(report.event_id.as_uuid());
            if let Some(f) = &fact.fill
                && f.operation == FillOperation::Add
            {
                s.fills.insert((report.account_id, f.fill_id.clone()));
            }
            s.reports
                .push((report.account_id, report.instrument.clone(), fact));
            s.report_times.push(report.occurred_at);
        }
        for t in work.commands {
            if let Some(c) = s.commands.get_mut(&t.id) {
                c.command.state = t.to;
            }
        }
        for w in work.sync {
            s.sync.insert(w.account_id, w.next);
        }
        if let Some(d) = work.deferred {
            s.deferred.push(d);
        }
        s.events.extend(work.events);
        Ok(CommitOutcome::Committed)
    }
}

#[derive(Default)]
pub struct FakeBindings {
    pub by_venue: Mutex<HashMap<VenueAccountId, BoundAccount>>,
}

impl FakeBindings {
    pub fn bind(&self, venue: VenueAccountId, account: AccountId, name: &str) {
        self.by_venue.lock().unwrap().insert(
            venue,
            BoundAccount {
                account_id: account,
                display_name: name.into(),
            },
        );
    }

    pub fn unbind_all(&self) {
        self.by_venue.lock().unwrap().clear();
    }
}

#[async_trait]
impl AccountBindings for FakeBindings {
    async fn accounts_for_venues(
        &self,
        _user_id: UserId,
        venues: &[VenueAccountId],
    ) -> Result<HashMap<VenueAccountId, BoundAccount>, PortError> {
        let m = self.by_venue.lock().unwrap();
        Ok(venues
            .iter()
            .filter_map(|v| m.get(v).map(|b| (*v, b.clone())))
            .collect())
    }

    async fn binding_of(
        &self,
        _user_id: UserId,
        account: AccountId,
    ) -> Result<Option<VenueAccountId>, PortError> {
        Ok(self
            .by_venue
            .lock()
            .unwrap()
            .iter()
            .find(|(_, b)| b.account_id == account)
            .map(|(v, _)| *v))
    }
}

#[derive(Default)]
pub struct FakeRisk {
    pub limits: Mutex<HashMap<AccountId, DerivedLimits>>,
    pub rate: Mutex<Option<Money>>,
    pub fail: Mutex<bool>,
}

#[async_trait]
impl RiskLimits for FakeRisk {
    async fn current_limits(
        &self,
        _user_id: UserId,
        accounts: &[AccountId],
    ) -> Result<HashMap<AccountId, DerivedLimits>, PortError> {
        let m = self.limits.lock().unwrap();
        Ok(accounts
            .iter()
            .filter_map(|a| m.get(a).map(|l| (*a, l.clone())))
            .collect())
    }

    async fn commission_rate(
        &self,
        _user_id: UserId,
        _account: AccountId,
    ) -> Result<Option<Money>, PortError> {
        if *self.fail.lock().unwrap() {
            return Err(PortError("risk down".into()));
        }
        Ok(*self.rate.lock().unwrap())
    }
}
