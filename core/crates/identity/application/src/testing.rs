//! In-memory fakes of every port, for use-case tests. `FakeStore` applies a `ChangeSet` all or
//! nothing and refuses writes outside the change set's scope (stand-in for RLS).

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use identity_enterprise::codes::NormalizedCode;
use identity_enterprise::connection::{AdapterInfo, ConnectionStatus, PairingCode};
use identity_enterprise::ids::{ConnectionId, InvitationId, SessionId, SourceId};
use identity_enterprise::invitation::Invitation;
use identity_enterprise::profile::Locale;
use identity_enterprise::secret::SecretString;
use identity_enterprise::session::Session;
use identity_enterprise::source::Source;
use identity_enterprise::tokens::{CodeHmac, TokenHash};
use identity_enterprise::user::User;
use shared_kernel::{Clock, UserId};

use crate::changes::{ChangeSet, StoredKey, Write};
use crate::deps::{IdentityDeps, IdentitySettings};
use crate::ports::*;
use crate::throttle::*;

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

#[derive(Clone, Debug)]
pub struct DeviceRow {
    pub connection_id: ConnectionId,
    pub key: StoredKey,
    pub revoked: bool,
}

#[derive(Clone, Default)]
pub struct State {
    pub users: HashMap<UserId, User>,
    pub sessions: HashMap<SessionId, Session>,
    pub invitations: Vec<(Invitation, TokenHash)>,
    pub sources: HashMap<SourceId, (Source, Vec<KeyRow>)>,
    pub connections: HashMap<ConnectionId, identity_enterprise::connection::Connection>,
    pub codes: Vec<PairingCode>,
    pub devices: Vec<DeviceRow>,
}

#[derive(Default)]
pub struct FakeStore {
    pub state: Mutex<State>,
    pub committed: Mutex<Vec<ChangeSet>>,
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

    fn apply(st: &mut State, scope: UserId, w: Write) -> Result<(), StoreError> {
        match w {
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
            Write::InsertSession(s) => {
                rls(scope, s.user_id)?;
                st.sessions.insert(s.id, s);
            }
            Write::UpdateSessionActivity {
                session_id,
                last_activity_at,
                idle_expires_at,
                authenticated_at,
            } => {
                let s = st.sessions.get_mut(&session_id).unwrap();
                rls(scope, s.user_id)?;
                s.last_activity_at = last_activity_at;
                s.idle_expires_at = idle_expires_at;
                s.authenticated_at = authenticated_at;
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
                if st.devices.iter().any(|d| d.key.key_id == device_key.key_id) {
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
                    .devices
                    .iter_mut()
                    .filter(|d| d.connection_id == connection_id)
                {
                    d.revoked = true;
                }
                st.devices.push(DeviceRow {
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
                    .devices
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
    async fn user_for_login(&self, email: &str) -> Result<Option<User>, StoreError> {
        Ok(self
            .state
            .lock()
            .unwrap()
            .users
            .values()
            .find(|u| u.email.normalized() == email)
            .cloned())
    }
    async fn user(&self, id: UserId) -> Result<Option<User>, StoreError> {
        Ok(self.state.lock().unwrap().users.get(&id).cloned())
    }
    async fn users(&self, _admin: UserId) -> Result<Vec<UserSummary>, StoreError> {
        let st = self.state.lock().unwrap();
        Ok(st
            .users
            .values()
            .map(|u| UserSummary {
                user: u.clone(),
                last_sign_in_at: None,
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
    async fn signed_in_from(
        &self,
        user: UserId,
        ip: IpAddr,
        since: DateTime<Utc>,
    ) -> Result<bool, StoreError> {
        Ok(self.state.lock().unwrap().sessions.values().any(|s| {
            s.user_id == user
                && s.client.ip == Some(ip)
                && s.login_at >= since
                && s.mfa_state != identity_enterprise::session::MfaState::Pending
        }))
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

/// Not cryptographic: reversible stand-ins with the right shapes.
pub struct FakeHasher;

impl SecretHasher for FakeHasher {
    fn sha256(&self, bytes: &[u8]) -> TokenHash {
        let mut out = [0u8; 32];
        for (i, b) in bytes.iter().enumerate() {
            out[i % 32] ^= b.rotate_left(3);
        }
        TokenHash::from_bytes(out)
    }
    fn code_hmac(&self, code: &NormalizedCode) -> CodeHmac {
        let mut bytes = [0u8; 32];
        bytes[..code.as_bytes().len()].copy_from_slice(code.as_bytes());
        CodeHmac {
            bytes,
            pepper_version: 1,
        }
    }
    fn code_hmac_candidates(&self, code: &NormalizedCode) -> Vec<CodeHmac> {
        vec![self.code_hmac(code)]
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

/// Every fake, plus the `IdentityDeps` that wires them.
pub struct World {
    pub deps: IdentityDeps,
    pub store: Arc<FakeStore>,
    pub clock: Arc<TestClock>,
    pub mailer: Arc<FakeMailer>,
    pub presence: Arc<FakePresence>,
    pub pairing_throttle: Arc<FakePairingThrottle>,
    pub activated_keys: Arc<RecordingKeyListener>,
}

impl World {
    pub fn new() -> Self {
        let store = Arc::new(FakeStore::default());
        let clock = Arc::new(TestClock(Mutex::new(t0())));
        let mailer = Arc::new(FakeMailer::default());
        let presence = Arc::new(FakePresence::default());
        let pairing_throttle = Arc::new(FakePairingThrottle::default());
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
            settings: IdentitySettings {
                public_origin: ORIGIN.into(),
            },
        };
        Self {
            deps,
            store,
            clock,
            mailer,
            presence,
            pairing_throttle,
            activated_keys,
        }
    }
}
