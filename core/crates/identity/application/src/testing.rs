//! In-memory fakes of every port, for use-case tests. `FakeStore` applies a `ChangeSet` all or
//! nothing and refuses writes outside the change set's scope (stand-in for RLS). Conditional
//! writes follow the store's SQL predicates.

use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use identity_enterprise::channel::Destination;
use identity_enterprise::channel_change::{ChangeStatus, ChannelChange};
use identity_enterprise::codes::NormalizedCode;
use identity_enterprise::connection::{AdapterInfo, ConnectionStatus, PairingCode};
use identity_enterprise::device::KnownDevice;
use identity_enterprise::ids::{
    ChannelChangeId, CodeId, ConnectionId, DeviceId, InvitationId, SessionId, SourceId,
};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::otp::{
    CLAIMS_LEN, ChallengeClaims, CodeState, DeviceClass, InvalidReason, MAX_WRONG_ATTEMPTS,
    OneTimeCode, Purpose, SignupCode,
};
use identity_enterprise::profile::Locale;
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::Session;
use identity_enterprise::source::Source;
use identity_enterprise::tokens::{CodeHmac, TokenHash};
use identity_enterprise::user::User;
use shared_kernel::{Clock, UserId};

use crate::changes::{ChangeSet, CodeTable, StoredKey, Write};
use crate::deps::{IdentityDeps, IdentitySettings};
use crate::ports::*;
use crate::throttle::*;
use crate::usecases::delivery::DeliveryJob;

pub const ORIGIN: &str = "https://app.test";

pub fn t0() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap()
}

pub struct TestClock(Mutex<DateTime<Utc>>);

impl TestClock {
    pub fn advance(&self, by: Duration) {
        *self.0.lock().unwrap() += by;
    }
}

impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct KeyRow {
    pub key: StoredKey,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

/// A device token of a paired connection.
#[derive(Clone, Debug)]
pub struct TokenRow {
    pub connection_id: ConnectionId,
    pub key: StoredKey,
    pub revoked: bool,
}

#[derive(Clone, Default)]
pub struct State {
    pub users: HashMap<UserId, User>,
    pub sessions: HashMap<SessionId, Session>,
    pub known_devices: Vec<KnownDevice>,
    pub invitations: Vec<(Invitation, TokenHash)>,
    pub one_time: Vec<OneTimeCode>,
    pub signup: Vec<SignupCode>,
    pub changes: Vec<ChannelChange>,
    /// (identifier bytes, class, window seconds, window start) → sends.
    pub counters: HashMap<([u8; 32], DeviceClass, i32, DateTime<Utc>), i32>,
    pub sources: HashMap<SourceId, (Source, Vec<KeyRow>)>,
    pub connections: HashMap<ConnectionId, identity_enterprise::connection::Connection>,
    pub codes: Vec<PairingCode>,
    pub device_tokens: Vec<TokenRow>,
}

#[derive(Default)]
pub struct FakeStore {
    pub state: Mutex<State>,
    pub committed: Mutex<Vec<ChangeSet>>,
    /// Number of `login_code_placeholder` calls (unknown identifiers).
    pub placeholders: Mutex<u32>,
}

fn conflict(c: StoreConflict) -> StoreError {
    StoreError::Conflict(c)
}

fn rls(scope: UserId, owner: UserId) -> Result<(), StoreError> {
    if scope == owner {
        Ok(())
    } else {
        Err(StoreError::Unavailable(
            "row-level security: write outside scope".into(),
        ))
    }
}

fn window_start(now: DateTime<Utc>, seconds: i32) -> DateTime<Utc> {
    let s = i64::from(seconds);
    let t = now.timestamp();
    DateTime::from_timestamp(t - t.rem_euclid(s), 0).unwrap()
}

impl FakeStore {
    pub fn user_by_email(&self, email: &str) -> Option<User> {
        let email = email.to_lowercase();
        let st = self.state.lock().unwrap();
        st.users
            .values()
            .find(|u| u.email.normalized() == email)
            .cloned()
    }

    pub fn insert_user(&self, user: User) {
        self.state.lock().unwrap().users.insert(user.id, user);
    }

    pub fn committed(&self) -> Vec<ChangeSet> {
        self.committed.lock().unwrap().clone()
    }

    pub fn one_time_codes(&self) -> Vec<OneTimeCode> {
        self.state.lock().unwrap().one_time.clone()
    }

    fn current_key(rows: &[KeyRow]) -> Option<&KeyRow> {
        rows.iter().max_by_key(|r| r.created_at)
    }

    fn source_view(source: &Source, rows: &[KeyRow]) -> Source {
        let mut s = source.clone();
        if let Some(cur) = Self::current_key(rows) {
            s.key_id = cur.key.key_id.clone();
            s.rotated_at = (rows.len() > 1).then_some(cur.created_at);
        }
        s
    }

    fn code_state(
        st: &mut State,
        table: CodeTable,
        id: CodeId,
    ) -> Option<(&mut CodeState, Option<UserId>)> {
        match table {
            CodeTable::OneTime => st
                .one_time
                .iter_mut()
                .find(|c| c.id == id)
                .map(|c| (&mut c.state, Some(c.user_id))),
            CodeTable::Signup => st
                .signup
                .iter_mut()
                .find(|c| c.id == id)
                .map(|c| (&mut c.state, None)),
        }
    }

    fn invalidate_change_codes(st: &mut State, change: ChannelChangeId, at: DateTime<Utc>) {
        for c in st.one_time.iter_mut().filter(|c| {
            c.channel_change_id == Some(change)
                && c.state.consumed_at.is_none()
                && c.state.invalidated_at.is_none()
        }) {
            c.state.invalidated_at = Some(at);
            c.state.invalidated_reason = Some(InvalidReason::ChangeClosed);
        }
    }

    fn invalidate_invitation_codes(st: &mut State, invitation: InvitationId, at: DateTime<Utc>) {
        for c in st.signup.iter_mut().filter(|c| {
            c.invitation_id == invitation
                && c.state.consumed_at.is_none()
                && c.state.invalidated_at.is_none()
        }) {
            c.state.invalidated_at = Some(at);
            c.state.invalidated_reason = Some(InvalidReason::InvitationClosed);
        }
    }

    fn change_mut(
        st: &mut State,
        scope: UserId,
        id: ChannelChangeId,
    ) -> Result<&mut ChannelChange, StoreError> {
        let c = st
            .changes
            .iter_mut()
            .find(|c| c.id == id)
            .ok_or(conflict(StoreConflict::ChannelChangeGone))?;
        rls(scope, c.user_id)?;
        Ok(c)
    }

    #[allow(clippy::too_many_lines)]
    fn apply(st: &mut State, scope: UserId, w: Write) -> Result<(), StoreError> {
        match w {
            Write::InsertUser(u) => {
                rls(scope, u.id)?;
                if st
                    .users
                    .values()
                    .any(|o| o.email.normalized() == u.email.normalized())
                {
                    return Err(conflict(StoreConflict::EmailTaken));
                }
                st.users.insert(u.id, u);
            }
            Write::UpdateProfile {
                user_id,
                locale,
                time_zone,
            } => {
                rls(scope, user_id)?;
                let u = st.users.get_mut(&user_id).unwrap();
                if let Some(l) = locale {
                    u.locale = l;
                }
                if let Some(tz) = time_zone {
                    u.time_zone = tz;
                }
            }
            Write::SetChannelValue { user_id, value, at } => {
                rls(scope, user_id)?;
                let taken = st.users.values().any(|o| {
                    o.id != user_id
                        && o.destination(value.channel())
                            .is_some_and(|d| d.normalized() == value.normalized())
                });
                if taken {
                    return Err(conflict(match value {
                        Destination::Email(_) => StoreConflict::EmailTaken,
                        Destination::Whatsapp(_) => StoreConflict::WhatsappTaken,
                    }));
                }
                let u = st.users.get_mut(&user_id).unwrap();
                match value {
                    Destination::Email(e) => {
                        u.email = e;
                        u.email_verified_at = at;
                    }
                    Destination::Whatsapp(p) => u.whatsapp = Some(p),
                }
                u.channel_changed_at = Some(at);
            }
            Write::InsertDevice(d) => {
                rls(scope, d.user_id)?;
                st.known_devices.push(d);
            }
            Write::TouchDevice {
                user_id,
                device_id,
                ip,
                at,
            } => {
                rls(scope, user_id)?;
                if let Some(d) = st.known_devices.iter_mut().find(|d| d.id == device_id) {
                    d.last_seen_at = at;
                    d.last_ip = ip;
                }
            }
            Write::RevokeDevice {
                user_id,
                device_id,
                at,
            } => {
                rls(scope, user_id)?;
                if let Some(d) = st
                    .known_devices
                    .iter_mut()
                    .find(|d| d.id == device_id && d.user_id == user_id)
                {
                    d.revoked_at.get_or_insert(at);
                }
            }
            Write::RevokeAllDevices { user_id, at } => {
                rls(scope, user_id)?;
                for d in st.known_devices.iter_mut().filter(|d| d.user_id == user_id) {
                    d.revoked_at.get_or_insert(at);
                }
            }
            Write::InsertSession(s) => {
                rls(scope, s.user_id)?;
                st.sessions.insert(s.id, s);
            }
            Write::UpdateSessionActivity {
                session_id,
                last_activity_at,
                idle_expires_at,
            } => {
                let s = st.sessions.get_mut(&session_id).unwrap();
                rls(scope, s.user_id)?;
                s.last_activity_at = last_activity_at;
                s.idle_expires_at = idle_expires_at;
            }
            Write::EndSession {
                session_id,
                reason,
                at,
            } => {
                if let Some(s) = st.sessions.get_mut(&session_id) {
                    rls(scope, s.user_id)?;
                    s.end(reason, at);
                }
            }
            Write::EndUserSessions {
                user_id,
                except,
                reason,
                at,
            } => {
                rls(scope, user_id)?;
                for s in st
                    .sessions
                    .values_mut()
                    .filter(|s| s.user_id == user_id && Some(s.id) != except)
                {
                    s.end(reason, at);
                }
            }
            Write::EndDeviceSessions {
                user_id,
                device_id,
                reason,
                at,
            } => {
                rls(scope, user_id)?;
                for s in st
                    .sessions
                    .values_mut()
                    .filter(|s| s.user_id == user_id && s.device_id == device_id)
                {
                    s.end(reason, at);
                }
            }
            Write::InsertInvitation {
                invitation,
                token_hash,
            } => {
                if st
                    .users
                    .values()
                    .any(|u| u.email.normalized() == invitation.email)
                    || st
                        .invitations
                        .iter()
                        .any(|(i, _)| i.email == invitation.email && i.is_open())
                {
                    return Err(conflict(StoreConflict::EmailTaken));
                }
                st.invitations.push((invitation, token_hash));
            }
            Write::SupersedeExpiredInvitation { invitation_id, at } => {
                if let Some((i, _)) = st
                    .invitations
                    .iter_mut()
                    .find(|(i, _)| i.id == invitation_id)
                {
                    i.superseded_at = Some(at);
                }
                Self::invalidate_invitation_codes(st, invitation_id, at);
            }
            Write::AcceptInvitation {
                invitation_id,
                user_id,
                at,
            } => {
                let (i, _) = st
                    .invitations
                    .iter_mut()
                    .find(|(i, _)| i.id == invitation_id)
                    .unwrap();
                if !i.is_open() || at >= i.expires_at {
                    return Err(conflict(StoreConflict::InvitationGone));
                }
                i.accepted_at = Some(at);
                i.accepted_user_id = Some(user_id);
            }
            Write::RevokeInvitation { invitation_id, at } => {
                let (i, _) = st
                    .invitations
                    .iter_mut()
                    .find(|(i, _)| i.id == invitation_id)
                    .unwrap();
                if i.is_open() {
                    i.revoked_at = Some(at);
                }
                Self::invalidate_invitation_codes(st, invitation_id, at);
            }
            Write::InsertOneTimeCode(c) => {
                rls(scope, c.user_id)?;
                st.one_time.push(c);
            }
            Write::InsertSignupCode(c) => st.signup.push(c),
            Write::ResendCode {
                table,
                code_id,
                code_hmac,
                sent_at,
                expires_at,
            } => {
                let (state, owner) = Self::code_state(st, table, code_id)
                    .ok_or(conflict(StoreConflict::CodeGone))?;
                if let Some(owner) = owner {
                    rls(scope, owner)?;
                }
                if !state.is_open(sent_at) {
                    return Err(conflict(StoreConflict::CodeGone));
                }
                state.code_hmac = code_hmac;
                state.sent_at = sent_at;
                state.expires_at = expires_at;
                state.delivery_state = identity_enterprise::otp::DeliveryState::Pending;
            }
            Write::FailCode { table, code_id } => {
                if let Some((state, owner)) = Self::code_state(st, table, code_id) {
                    if let Some(owner) = owner {
                        rls(scope, owner)?;
                    }
                    if state.consumed_at.is_none() && state.invalidated_at.is_none() {
                        state.failed_attempts += 1;
                        if state.failed_attempts >= MAX_WRONG_ATTEMPTS {
                            state.invalidated_at = Some(state.sent_at);
                            state.invalidated_reason = Some(InvalidReason::Attempts);
                        }
                    }
                }
            }
            Write::ConsumeCode {
                table,
                code_id,
                code_hmac,
                at,
            } => {
                let (state, owner) = Self::code_state(st, table, code_id)
                    .ok_or(conflict(StoreConflict::CodeGone))?;
                if let Some(owner) = owner {
                    rls(scope, owner)?;
                }
                if !state.is_open(at) || state.code_hmac != code_hmac {
                    return Err(conflict(StoreConflict::CodeGone));
                }
                state.consumed_at = Some(at);
            }
            Write::SetDeliveryState {
                table,
                code_id,
                state,
            } => {
                if let Some((s, owner)) = Self::code_state(st, table, code_id) {
                    if let Some(owner) = owner {
                        rls(scope, owner)?;
                    }
                    s.delivery_state = state;
                }
            }
            Write::InvalidateUserCodes {
                user_id,
                reason,
                at,
            } => {
                rls(scope, user_id)?;
                for c in st.one_time.iter_mut().filter(|c| {
                    c.user_id == user_id
                        && c.state.consumed_at.is_none()
                        && c.state.invalidated_at.is_none()
                }) {
                    c.state.invalidated_at = Some(at);
                    c.state.invalidated_reason = Some(reason);
                }
            }
            Write::InsertChannelChange(c) => {
                rls(scope, c.user_id)?;
                if st
                    .changes
                    .iter()
                    .any(|o| o.user_id == c.user_id && o.status.is_open())
                {
                    return Err(conflict(StoreConflict::ChannelChangeOpen));
                }
                st.changes.push(c);
            }
            Write::ExpireStaleChannelChange { user_id, at } => {
                rls(scope, user_id)?;
                let stale: Vec<ChannelChangeId> = st
                    .changes
                    .iter_mut()
                    .filter(|c| {
                        c.user_id == user_id
                            && c.status == ChangeStatus::AwaitingCodes
                            && c.expires_at <= at
                    })
                    .map(|c| {
                        c.status = ChangeStatus::Expired;
                        c.id
                    })
                    .collect();
                for id in stale {
                    Self::invalidate_change_codes(st, id, at);
                }
            }
            Write::AssertChannelChangeAwaiting { change_id, at } => {
                let c = Self::change_mut(st, scope, change_id)?;
                if c.status != ChangeStatus::AwaitingCodes || c.expires_at <= at {
                    return Err(conflict(StoreConflict::ChannelChangeGone));
                }
            }
            Write::CompleteChannelChange { change_id, at } => {
                let c = Self::change_mut(st, scope, change_id)?;
                if c.status != ChangeStatus::AwaitingCodes || c.expires_at <= at {
                    return Err(conflict(StoreConflict::ChannelChangeGone));
                }
                c.status = ChangeStatus::Completed;
                c.completed_at = Some(at);
            }
            Write::HoldChannelChange {
                change_id,
                hold_until,
                at,
            } => {
                let c = Self::change_mut(st, scope, change_id)?;
                if c.status != ChangeStatus::AwaitingCodes || c.expires_at <= at {
                    return Err(conflict(StoreConflict::ChannelChangeGone));
                }
                c.status = ChangeStatus::OnHold;
                c.hold_until = Some(hold_until);
            }
            Write::FinishHeldChannelChange { change_id, at } => {
                let c = Self::change_mut(st, scope, change_id)?;
                if c.status != ChangeStatus::OnHold {
                    return Err(conflict(StoreConflict::ChannelChangeGone));
                }
                c.status = ChangeStatus::Completed;
                c.completed_at = Some(at);
            }
            Write::CancelChannelChange {
                change_id,
                by_session,
                at,
            } => {
                let c = Self::change_mut(st, scope, change_id)?;
                let open = c.status == ChangeStatus::OnHold
                    || (c.status == ChangeStatus::AwaitingCodes && c.expires_at > at);
                if !open {
                    return Err(conflict(StoreConflict::ChannelChangeGone));
                }
                c.status = ChangeStatus::Cancelled;
                c.cancelled_at = Some(at);
                c.cancelled_by_session_id = by_session;
                Self::invalidate_change_codes(st, change_id, at);
            }
            Write::InsertSource { source, key } => {
                rls(scope, source.user_id)?;
                Self::check_key(st, &key)?;
                Self::check_source_name(st, source.user_id, source.name.as_str(), None)?;
                let row = KeyRow {
                    key,
                    created_at: source.created_at,
                    revoked_at: None,
                };
                st.sources.insert(source.id, (source, vec![row]));
            }
            Write::RenameSource {
                user_id,
                source_id,
                name,
            } => {
                rls(scope, user_id)?;
                Self::check_source_name(st, user_id, name.as_str(), Some(source_id))?;
                let (s, _) = st
                    .sources
                    .get_mut(&source_id)
                    .filter(|(s, _)| s.user_id == user_id)
                    .unwrap();
                if s.revoked_at.is_some() {
                    return Err(conflict(StoreConflict::SourceRevoked));
                }
                s.name = name;
            }
            Write::RotateSourceKey {
                user_id,
                source_id,
                key,
                at,
            } => {
                rls(scope, user_id)?;
                Self::check_key(st, &key)?;
                let (s, rows) = st
                    .sources
                    .get_mut(&source_id)
                    .filter(|(s, _)| s.user_id == user_id)
                    .unwrap();
                if s.revoked_at.is_some() {
                    return Err(conflict(StoreConflict::SourceRevoked));
                }
                for r in rows.iter_mut() {
                    r.revoked_at.get_or_insert(at);
                }
                rows.push(KeyRow {
                    key,
                    created_at: at,
                    revoked_at: None,
                });
            }
            Write::RevokeSource {
                user_id,
                source_id,
                at,
            } => {
                rls(scope, user_id)?;
                let (s, rows) = st
                    .sources
                    .get_mut(&source_id)
                    .filter(|(s, _)| s.user_id == user_id)
                    .unwrap();
                s.revoked_at.get_or_insert(at);
                for r in rows.iter_mut() {
                    r.revoked_at.get_or_insert(at);
                }
            }
            Write::InsertConnection(c) => {
                rls(scope, c.user_id)?;
                Self::check_connection_name(st, c.user_id, c.name.as_str(), None)?;
                st.connections.insert(c.id, c);
            }
            Write::RenameConnection {
                user_id,
                connection_id,
                name,
            } => {
                rls(scope, user_id)?;
                Self::check_connection_name(st, user_id, name.as_str(), Some(connection_id))?;
                let c = st
                    .connections
                    .get_mut(&connection_id)
                    .filter(|c| c.user_id == user_id)
                    .unwrap();
                if c.unpaired_at.is_some() {
                    return Err(conflict(StoreConflict::ConnectionUnpaired));
                }
                c.name = name;
            }
            Write::IssuePairingCode(code) => {
                rls(scope, code.user_id)?;
                if st
                    .codes
                    .iter()
                    .any(|c| c.code_hmac.bytes == code.code_hmac.bytes)
                {
                    return Err(conflict(StoreConflict::CodeHmacTaken));
                }
                if st
                    .connections
                    .get(&code.connection_id)
                    .is_none_or(|c| c.unpaired_at.is_some())
                {
                    return Err(conflict(StoreConflict::ConnectionUnpaired));
                }
                for c in st
                    .codes
                    .iter_mut()
                    .filter(|c| c.connection_id == code.connection_id && c.used_at.is_none())
                {
                    c.invalidated_at.get_or_insert(code.created_at);
                }
                st.codes.push(code);
            }
            Write::UsePairingCode { code_id, at } => {
                let c = st.codes.iter_mut().find(|c| c.id == code_id).unwrap();
                rls(scope, c.user_id)?;
                if !c.is_redeemable(at) {
                    return Err(conflict(StoreConflict::PairingCodeGone));
                }
                c.used_at = Some(at);
            }
            Write::PairConnection {
                user_id,
                connection_id,
                instance,
                device_key,
            } => {
                rls(scope, user_id)?;
                if st
                    .device_tokens
                    .iter()
                    .any(|d| d.key.key_id == device_key.key_id)
                {
                    return Err(conflict(StoreConflict::KeyIdTaken));
                }
                let c = st
                    .connections
                    .get_mut(&connection_id)
                    .filter(|c| c.user_id == user_id)
                    .unwrap();
                c.pair(instance)
                    .map_err(|_| conflict(StoreConflict::ConnectionUnpaired))?;
                for d in st
                    .device_tokens
                    .iter_mut()
                    .filter(|d| d.connection_id == connection_id)
                {
                    d.revoked = true;
                }
                st.device_tokens.push(TokenRow {
                    connection_id,
                    key: device_key,
                    revoked: false,
                });
            }
            Write::UpdateConnectionStatus {
                user_id,
                connection_id,
                status,
                status_changed_at,
                last_seen_at,
            } => {
                rls(scope, user_id)?;
                let c = st.connections.get_mut(&connection_id).unwrap();
                c.status = status;
                c.status_changed_at = status_changed_at;
                c.last_seen_at = last_seen_at;
            }
            Write::DisconnectLiveConnection {
                user_id,
                connection_id,
                at,
            } => {
                rls(scope, user_id)?;
                let c = st.connections.get_mut(&connection_id).unwrap();
                if matches!(
                    c.status,
                    ConnectionStatus::Connected | ConnectionStatus::Outdated
                ) {
                    c.status = ConnectionStatus::Disconnected;
                    c.status_changed_at = at;
                }
            }
            Write::UnpairConnection {
                user_id,
                connection_id,
                at,
            } => {
                rls(scope, user_id)?;
                let c = st
                    .connections
                    .get_mut(&connection_id)
                    .filter(|c| c.user_id == user_id)
                    .unwrap();
                c.unpair(at);
                for d in st
                    .device_tokens
                    .iter_mut()
                    .filter(|d| d.connection_id == connection_id)
                {
                    d.revoked = true;
                }
                for p in st
                    .codes
                    .iter_mut()
                    .filter(|p| p.connection_id == connection_id && p.used_at.is_none())
                {
                    p.invalidated_at.get_or_insert(at);
                }
            }
        }
        Ok(())
    }

    fn check_key(st: &State, key: &StoredKey) -> Result<(), StoreError> {
        let taken = st
            .sources
            .values()
            .flat_map(|(_, rows)| rows)
            .any(|r| r.key.key_id == key.key_id);
        if taken {
            Err(conflict(StoreConflict::KeyIdTaken))
        } else {
            Ok(())
        }
    }

    fn check_source_name(
        st: &State,
        user: UserId,
        name: &str,
        except: Option<SourceId>,
    ) -> Result<(), StoreError> {
        let taken = st.sources.values().any(|(s, _)| {
            s.user_id == user
                && s.revoked_at.is_none()
                && Some(s.id) != except
                && s.name.as_str().to_lowercase() == name.to_lowercase()
        });
        if taken {
            Err(conflict(StoreConflict::NameTaken))
        } else {
            Ok(())
        }
    }

    fn check_connection_name(
        st: &State,
        user: UserId,
        name: &str,
        except: Option<ConnectionId>,
    ) -> Result<(), StoreError> {
        let taken = st.connections.values().any(|c| {
            c.user_id == user
                && c.unpaired_at.is_none()
                && Some(c.id) != except
                && c.name.as_str().to_lowercase() == name.to_lowercase()
        });
        if taken {
            Err(conflict(StoreConflict::NameTaken))
        } else {
            Ok(())
        }
    }

    fn record(st: &State, c: &identity_enterprise::connection::Connection) -> ConnectionRecord {
        let active = st
            .codes
            .iter()
            .filter(|p| {
                p.connection_id == c.id && p.used_at.is_none() && p.invalidated_at.is_none()
            })
            .map(|p| p.expires_at)
            .max();
        ConnectionRecord {
            connection: c.clone(),
            active_code_expires_at: active,
        }
    }
}

#[async_trait]
impl IdentityStore for FakeStore {
    async fn login_target(&self, email: &str) -> Result<Option<LoginTarget>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .users
            .values()
            .find(|u| u.email.normalized() == email)
            .map(|u| LoginTarget {
                user_id: u.id,
                status: u.status,
                locale: u.locale,
                destinations: [
                    identity_enterprise::channel::Channel::Email,
                    identity_enterprise::channel::Channel::Whatsapp,
                ]
                .into_iter()
                .filter_map(|c| u.destination(c))
                .collect(),
            }))
    }
    async fn user(&self, id: UserId) -> Result<Option<User>, StoreError> {
        Ok(self.state.lock().unwrap().users.get(&id).cloned())
    }
    async fn users(&self, admin: UserId) -> Result<Vec<UserSummary>, StoreError> {
        let st = self.state.lock().unwrap();
        if !st.users.get(&admin).is_some_and(|u| u.is_platform_admin) {
            return Ok(Vec::new());
        }
        Ok(st
            .users
            .values()
            .map(|u| UserSummary {
                id: u.id,
                email: u.email.as_entered().to_owned(),
                status: u.status,
                is_platform_admin: u.is_platform_admin,
                whatsapp_verified: u.whatsapp.is_some(),
                created_at: u.created_at,
                last_sign_in_at: st
                    .sessions
                    .values()
                    .filter(|s| s.user_id == u.id)
                    .map(|s| s.login_at)
                    .max(),
            })
            .collect())
    }
    async fn email_registered(&self, _admin: UserId, email: &str) -> Result<bool, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .users
            .values()
            .any(|u| u.email.normalized() == email))
    }
    async fn session_by_token(&self, hash: &TokenHash) -> Result<Option<Session>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .sessions
            .values()
            .find(|s| s.token_hash == *hash)
            .cloned())
    }
    async fn session(&self, user: UserId, id: SessionId) -> Result<Option<Session>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .sessions
            .get(&id)
            .filter(|s| s.user_id == user)
            .cloned())
    }
    async fn device_by_key(
        &self,
        user: UserId,
        hash: &TokenHash,
    ) -> Result<Option<KnownDevice>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .known_devices
            .iter()
            .filter(|d| d.user_id == user && d.key_hash == *hash)
            .max_by_key(|d| d.revoked_at.is_none())
            .cloned())
    }
    async fn device_key_in_use(&self, hash: &TokenHash) -> Result<bool, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .known_devices
            .iter()
            .any(|d| d.key_hash == *hash && d.revoked_at.is_none()))
    }
    async fn device(&self, user: UserId, id: DeviceId) -> Result<Option<KnownDevice>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .known_devices
            .iter()
            .find(|d| d.user_id == user && d.id == id)
            .cloned())
    }
    async fn devices(&self, user: UserId) -> Result<Vec<KnownDevice>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .known_devices
            .iter()
            .filter(|d| d.user_id == user && d.revoked_at.is_none())
            .cloned()
            .collect())
    }
    async fn invitation(&self, id: InvitationId) -> Result<Option<Invitation>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .invitations
            .iter()
            .find(|(i, _)| i.id == id)
            .map(|(i, _)| i.clone()))
    }
    async fn invitation_by_token(
        &self,
        hash: &TokenHash,
    ) -> Result<Option<Invitation>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .invitations
            .iter()
            .find(|(_, h)| h == hash)
            .map(|(i, _)| i.clone()))
    }
    async fn open_invitation_for(&self, email: &str) -> Result<Option<Invitation>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .invitations
            .iter()
            .find(|(i, _)| i.email == email && i.is_open())
            .map(|(i, _)| i.clone()))
    }
    async fn invitations(&self) -> Result<Vec<Invitation>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .invitations
            .iter()
            .map(|(i, _)| i.clone())
            .collect())
    }
    async fn signup_code(&self, id: CodeId) -> Result<Option<SignupCode>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .signup
            .iter()
            .find(|c| c.id == id)
            .cloned())
    }
    async fn one_time_code_owner(
        &self,
        id: CodeId,
    ) -> Result<Option<(UserId, Purpose)>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .one_time
            .iter()
            .find(|c| c.id == id)
            .map(|c| (c.user_id, c.purpose)))
    }
    async fn one_time_code(
        &self,
        user: UserId,
        id: CodeId,
    ) -> Result<Option<OneTimeCode>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .one_time
            .iter()
            .find(|c| c.id == id && c.user_id == user)
            .cloned())
    }
    async fn change_codes(
        &self,
        user: UserId,
        change: ChannelChangeId,
    ) -> Result<Vec<OneTimeCode>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .one_time
            .iter()
            .filter(|c| c.user_id == user && c.channel_change_id == Some(change))
            .cloned()
            .collect())
    }
    async fn signed_in_sends_since(
        &self,
        user: UserId,
        since: DateTime<Utc>,
    ) -> Result<i64, StoreError> {
        let n = self
            .state
            .lock()
            .unwrap()
            .one_time
            .iter()
            .filter(|c| {
                c.user_id == user
                    && matches!(c.purpose, Purpose::StepUp | Purpose::ChannelChange)
                    && c.state.created_at >= since
            })
            .count();
        Ok(i64::try_from(n).unwrap())
    }
    async fn channel_change(
        &self,
        user: UserId,
        id: ChannelChangeId,
    ) -> Result<Option<ChannelChange>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .changes
            .iter()
            .find(|c| c.id == id && c.user_id == user)
            .cloned())
    }
    async fn open_channel_change(&self, user: UserId) -> Result<Option<ChannelChange>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .changes
            .iter()
            .find(|c| c.user_id == user && c.status.is_open())
            .cloned())
    }
    async fn latest_channel_change(
        &self,
        user: UserId,
    ) -> Result<Option<ChannelChange>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .changes
            .iter()
            .filter(|c| c.user_id == user)
            .max_by_key(|c| c.created_at)
            .cloned())
    }
    async fn due_channel_holds(
        &self,
        now: DateTime<Utc>,
    ) -> Result<Vec<(UserId, ChannelChangeId)>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .changes
            .iter()
            .filter(|c| c.hold_is_due(now))
            .map(|c| (c.user_id, c.id))
            .collect())
    }
    async fn count_send(
        &self,
        identifier: &CodeHmac,
        class: DeviceClass,
        windows: &[i32],
        now: DateTime<Utc>,
    ) -> Result<Vec<WindowCount>, StoreError> {
        let mut st = self.state.lock().unwrap();
        Ok(windows
            .iter()
            .map(|w| {
                let start = window_start(now, *w);
                let n = st
                    .counters
                    .entry((identifier.bytes, class, *w, start))
                    .or_insert(0);
                *n += 1;
                WindowCount {
                    window_seconds: *w,
                    window_start: start,
                    sends: *n,
                }
            })
            .collect())
    }
    async fn send_counts(
        &self,
        identifier: &CodeHmac,
        class: DeviceClass,
        windows: &[i32],
        now: DateTime<Utc>,
    ) -> Result<Vec<WindowCount>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(windows
            .iter()
            .map(|w| {
                let start = window_start(now, *w);
                WindowCount {
                    window_seconds: *w,
                    window_start: start,
                    sends: st
                        .counters
                        .get(&(identifier.bytes, class, *w, start))
                        .copied()
                        .unwrap_or(0),
                }
            })
            .collect())
    }
    async fn login_code_placeholder(&self) -> Result<(), StoreError> {
        *self.placeholders.lock().unwrap() += 1;
        Ok(())
    }
    async fn sources(
        &self,
        user: UserId,
        include_revoked: bool,
    ) -> Result<Vec<Source>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .sources
            .values()
            .filter(|(s, _)| s.user_id == user && (include_revoked || s.revoked_at.is_none()))
            .map(|(s, rows)| Self::source_view(s, rows))
            .collect())
    }
    async fn source(&self, user: UserId, id: SourceId) -> Result<Option<Source>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .sources
            .get(&id)
            .filter(|(s, _)| s.user_id == user)
            .map(|(s, rows)| Self::source_view(s, rows)))
    }
    async fn connections(
        &self,
        user: UserId,
        include_unpaired: bool,
    ) -> Result<Vec<ConnectionRecord>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .connections
            .values()
            .filter(|c| c.user_id == user && (include_unpaired || c.unpaired_at.is_none()))
            .map(|c| Self::record(&st, c))
            .collect())
    }
    async fn connection(
        &self,
        user: UserId,
        id: ConnectionId,
    ) -> Result<Option<ConnectionRecord>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .connections
            .get(&id)
            .filter(|c| c.user_id == user)
            .map(|c| Self::record(&st, c)))
    }
    async fn redeemable_pairing_code(
        &self,
        candidates: &[CodeHmac],
        _now: DateTime<Utc>,
    ) -> Result<Option<PairingCode>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .codes
            .iter()
            .find(|c| candidates.contains(&c.code_hmac))
            .cloned())
    }
    async fn live_connections(&self) -> Result<Vec<(UserId, ConnectionId)>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .connections
            .values()
            .filter(|c| {
                matches!(
                    c.status,
                    ConnectionStatus::Connected | ConnectionStatus::Outdated
                )
            })
            .map(|c| (c.user_id, c.id))
            .collect())
    }
    async fn commit(&self, changes: ChangeSet) -> Result<(), StoreError> {
        let mut guard = self.state.lock().unwrap();
        let mut next = guard.clone();
        for w in changes.writes.clone() {
            Self::apply(&mut next, changes.scope, w)?;
        }
        *guard = next;
        self.committed.lock().unwrap().push(changes);
        Ok(())
    }
}

/// Not cryptographic: deterministic stand-ins with the right shapes. Sealed challenges are hex of
/// the claims plus an 8-byte keyed checksum, so a forged or altered token fails to open.
pub struct FakeHasher;

fn mix(seed: u64, data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut h = seed;
    for (i, b) in data.iter().enumerate() {
        h = (h ^ u64::from(*b)).wrapping_mul(0x100000001b3);
        out[i % 32] ^= (h >> 24) as u8;
    }
    for (i, o) in out.iter_mut().enumerate() {
        h = h.wrapping_mul(0x100000001b3).wrapping_add(i as u64);
        *o ^= (h >> 32) as u8;
    }
    out
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
        .collect()
}

impl SecretHasher for FakeHasher {
    fn sha256(&self, bytes: &[u8]) -> TokenHash {
        TokenHash::from_bytes(mix(1, bytes))
    }
    fn code_hmac(&self, code: &NormalizedCode) -> CodeHmac {
        CodeHmac {
            bytes: mix(2, code.as_bytes()),
            pepper_version: 1,
        }
    }
    fn code_hmac_candidates(&self, code: &NormalizedCode) -> Vec<CodeHmac> {
        vec![self.code_hmac(code)]
    }
    fn otp_hmac(&self, id: CodeId, code: &NormalizedCode) -> CodeHmac {
        let mut data = id.as_uuid().as_bytes().to_vec();
        data.extend_from_slice(code.as_bytes());
        CodeHmac {
            bytes: mix(3, &data),
            pepper_version: 1,
        }
    }
    fn otp_hmac_with(&self, id: CodeId, code: &NormalizedCode, v: i16) -> Option<CodeHmac> {
        (v == 1).then(|| self.otp_hmac(id, code))
    }
    fn identifier_hmac(&self, normalized: &str) -> CodeHmac {
        CodeHmac {
            bytes: mix(4, normalized.as_bytes()),
            pepper_version: 1,
        }
    }
    fn seal_challenge(&self, claims: &ChallengeClaims) -> SecretString {
        let bytes = claims.to_bytes();
        let mac = mix(5, &bytes);
        SecretString::new(format!("{}{}", hex(&bytes), hex(&mac[..8])))
    }
    fn open_challenge(&self, token: &str) -> Option<ChallengeClaims> {
        let raw = unhex(token)?;
        if raw.len() != CLAIMS_LEN + 8 {
            return None;
        }
        let (bytes, mac) = raw.split_at(CLAIMS_LEN);
        (mix(5, bytes)[..8] == *mac).then(|| ChallengeClaims::from_bytes(bytes))?
    }
    fn mac_eq(&self, a: &CodeHmac, b: &CodeHmac) -> bool {
        a == b
    }
}

/// Counter bytes: every fill differs, so tokens and codes are unique.
#[derive(Default)]
pub struct FakeRandom(Mutex<u64>);

impl RandomSource for FakeRandom {
    fn fill(&self, buf: &mut [u8]) {
        let mut n = self.0.lock().unwrap();
        for chunk in buf.chunks_mut(8) {
            *n = n
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let bytes = n.to_be_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
    }
}

#[derive(Default)]
pub struct FakeMailer {
    pub fail: Mutex<bool>,
    pub sent: Mutex<Vec<(String, String)>>,
}

#[async_trait]
impl Mailer for FakeMailer {
    async fn send_invitation(
        &self,
        to: &str,
        _l: Locale,
        url: &SecretString,
    ) -> Result<(), MailError> {
        if *self.fail.lock().unwrap() {
            return Err(MailError);
        }
        self.sent
            .lock()
            .unwrap()
            .push((to.to_owned(), url.expose().to_owned()));
        Ok(())
    }
}

#[derive(Default)]
pub struct FakePresence(pub Mutex<HashMap<ConnectionId, ConnectionAccounts>>);

#[async_trait]
impl ConnectionPresence for FakePresence {
    async fn accounts(
        &self,
        _u: UserId,
        ids: &[ConnectionId],
    ) -> Result<HashMap<ConnectionId, ConnectionAccounts>, String> {
        let map = self.0.lock().unwrap();
        Ok(ids
            .iter()
            .filter_map(|id| map.get(id).map(|a| (*id, a.clone())))
            .collect())
    }
}

pub struct NoActivity;

#[async_trait]
impl SourceActivity for NoActivity {
    async fn last_received(
        &self,
        _u: UserId,
        _s: &[SourceId],
    ) -> Result<HashMap<SourceId, DateTime<Utc>>, String> {
        Ok(HashMap::new())
    }
}

#[derive(Default)]
pub struct RecordingKeyListener(pub Mutex<Vec<String>>);

impl SourceKeyListener for RecordingKeyListener {
    fn key_activated(&self, key_id: &identity_enterprise::tokens::KeyId) {
        self.0.lock().unwrap().push(key_id.as_str().to_owned());
    }
}

pub struct FakeAdapters;

impl AdapterCompatibility for FakeAdapters {
    fn is_supported(&self, a: &AdapterInfo) -> bool {
        a.version != "0.0.1"
    }
}

#[derive(Default)]
pub struct FakePairingThrottle {
    pub decision: Mutex<Option<PairingDecision>>,
    pub failures: Mutex<u32>,
}

#[async_trait]
impl PairingThrottle for FakePairingThrottle {
    async fn admit(&self, _ip: IpAddr, _n: DateTime<Utc>) -> PairingDecision {
        self.decision
            .lock()
            .unwrap()
            .unwrap_or(PairingDecision::Allow)
    }
    async fn record_failure(&self, _ip: IpAddr, _n: DateTime<Utc>) {
        *self.failures.lock().unwrap() += 1;
    }
}

pub struct AllowPublic;

#[async_trait]
impl PublicEndpointThrottle for AllowPublic {
    async fn admit(&self, _ip: IpAddr, _n: DateTime<Utc>) -> ThrottleDecision {
        ThrottleDecision::Allow
    }
}

/// Code throttle: allows unless told otherwise; counts wrong attempts.
#[derive(Default)]
pub struct FakeCodeThrottle {
    pub send: Mutex<Option<ThrottleDecision>>,
    pub sends: Mutex<u32>,
    pub wrong: Mutex<u32>,
}

#[async_trait]
impl CodeThrottle for FakeCodeThrottle {
    async fn admit_send(&self, _ip: IpAddr, _n: DateTime<Utc>) -> ThrottleDecision {
        *self.sends.lock().unwrap() += 1;
        self.send.lock().unwrap().unwrap_or(ThrottleDecision::Allow)
    }
    async fn admit_verify(&self, _ip: IpAddr, _n: DateTime<Utc>) -> ThrottleDecision {
        ThrottleDecision::Allow
    }
    async fn record_wrong(&self, _ip: IpAddr, _n: DateTime<Utc>) {
        *self.wrong.lock().unwrap() += 1;
    }
}

/// Captures every dispatched delivery (the plaintext code is only here, as in memory).
#[derive(Default)]
pub struct FakeDelivery(pub Mutex<Vec<DeliveryJob>>);

impl DeliveryQueue for FakeDelivery {
    fn dispatch(&self, job: DeliveryJob) {
        self.0.lock().unwrap().push(job);
    }
}

impl FakeDelivery {
    pub fn jobs(&self) -> Vec<DeliveryJob> {
        self.0.lock().unwrap().clone()
    }

    /// The plaintext of the last code sent to `destination`.
    pub fn last_code_to(&self, destination: &str) -> Option<String> {
        self.jobs()
            .into_iter()
            .rev()
            .find(|j| j.delivery.destination.normalized() == destination && !j.suppressed)
            .map(|j| j.delivery.code.expose().to_owned())
    }
}

#[derive(Default)]
pub struct FakeAlerts(pub Mutex<Vec<SecurityAlert>>);

impl SecurityAlerts for FakeAlerts {
    fn raise(&self, alert: SecurityAlert) {
        self.0.lock().unwrap().push(alert);
    }
}

#[derive(Default)]
pub struct FakeAccounts(pub Mutex<HashSet<(UserId, uuid::Uuid)>>);

#[async_trait]
impl AccountTargets for FakeAccounts {
    async fn owned_by(&self, user: UserId, account: uuid::Uuid) -> Result<bool, String> {
        Ok(self.0.lock().unwrap().contains(&(user, account)))
    }
}

/// Every fake, plus the `IdentityDeps` that wires them.
pub struct World {
    pub deps: IdentityDeps,
    pub store: Arc<FakeStore>,
    pub clock: Arc<TestClock>,
    pub mailer: Arc<FakeMailer>,
    pub presence: Arc<FakePresence>,
    pub pairing_throttle: Arc<FakePairingThrottle>,
    pub code_throttle: Arc<FakeCodeThrottle>,
    pub delivery: Arc<FakeDelivery>,
    pub alerts: Arc<FakeAlerts>,
    pub accounts: Arc<FakeAccounts>,
    pub activated_keys: Arc<RecordingKeyListener>,
}

impl World {
    pub fn new() -> Self {
        Self::with_settings(true)
    }

    pub fn with_settings(q35: bool) -> Self {
        let store = Arc::new(FakeStore::default());
        let clock = Arc::new(TestClock(Mutex::new(t0())));
        let mailer = Arc::new(FakeMailer::default());
        let presence = Arc::new(FakePresence::default());
        let pairing_throttle = Arc::new(FakePairingThrottle::default());
        let code_throttle = Arc::new(FakeCodeThrottle::default());
        let delivery = Arc::new(FakeDelivery::default());
        let alerts = Arc::new(FakeAlerts::default());
        let accounts = Arc::new(FakeAccounts::default());
        let activated_keys = Arc::new(RecordingKeyListener::default());
        let deps = IdentityDeps {
            store: store.clone(),
            clock: clock.clone(),
            hasher: Arc::new(FakeHasher),
            random: Arc::new(FakeRandom::default()),
            mailer: mailer.clone(),
            presence: presence.clone(),
            source_activity: Arc::new(NoActivity),
            source_keys: activated_keys.clone(),
            adapters: Arc::new(FakeAdapters),
            pairing_throttle: pairing_throttle.clone(),
            public_throttle: Arc::new(AllowPublic),
            code_throttle: code_throttle.clone(),
            delivery: delivery.clone(),
            alerts: alerts.clone(),
            account_targets: accounts.clone(),
            settings: IdentitySettings {
                public_origin: ORIGIN.into(),
                step_up_q35_persistence_actions: q35,
                whatsapp_enabled: false,
            },
        };
        Self {
            deps,
            store,
            clock,
            mailer,
            presence,
            pairing_throttle,
            code_throttle,
            delivery,
            alerts,
            accounts,
            activated_keys,
        }
    }
}
