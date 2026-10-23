//! `listConnections`, `createConnection` (step-up), `renameConnection`, `newPairingCode`
//! (step-up; also "move to another machine"), `unpairConnection` (step-up, Q18), plus the
//! status handlers fed by the gateway and by Core startup.

use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use identity_enterprise::codes::{CodeKind, NormalizedCode};
use identity_enterprise::connection::{Connection, ConnectionStatus, LiveState, PairingCode};
use identity_enterprise::ids::{ConnectionId, PairingCodeId};
use identity_enterprise::source::DisplayName;
use shared_kernel::UserId;

use crate::auth::AuthContext;
use crate::changes::{Actor, AuditAction, ChangeSet, IdentityEvent, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{ConnectionAccounts, ConnectionRecord, StoreConflict, StoreError};
use crate::usecases::common::user_audit;
use crate::usecases::step_up::{self, StepUpProof};
use crate::views::{ConnectionView, CreatedConnection, PairingCodeView};
use identity_enterprise::otp::StepUpAction;

/// A collision on a random 40-bit code (unique over the whole history) is retried.
const CODE_ATTEMPTS: usize = 3;

fn view(
    record: ConnectionRecord,
    accounts: Option<ConnectionAccounts>,
    now: DateTime<Utc>,
) -> ConnectionView {
    let code_expires = record.active_code_expires_at.filter(|at| *at > now);
    let accounts = accounts.unwrap_or_default();
    let connection = record.connection;
    ConnectionView {
        status: connection.status,
        status_changed_at: connection.status_changed_at,
        last_seen_at: connection.last_seen_at,
        pairing_code_expires_at: code_expires,
        repairing_pending: connection.is_paired() && code_expires.is_some(),
        visible_accounts: accounts.visible_accounts,
        open_position_accounts: accounts.open_position_accounts,
        connection,
    }
}

async fn accounts_of(
    deps: &IdentityDeps,
    user: UserId,
    ids: &[ConnectionId],
) -> Result<HashMap<ConnectionId, ConnectionAccounts>, IdentityError> {
    deps.presence
        .accounts(user, ids)
        .await
        .map_err(IdentityError::Internal)
}

async fn owned(
    deps: &IdentityDeps,
    ctx: &AuthContext,
    id: ConnectionId,
) -> Result<ConnectionRecord, IdentityError> {
    deps.store
        .connection(ctx.user.id, id)
        .await?
        .ok_or(IdentityError::NotFound)
}

fn map_conflict(err: StoreError) -> IdentityError {
    match err {
        StoreError::Conflict(StoreConflict::NameTaken) => IdentityError::ConnectionNameTaken,
        StoreError::Conflict(StoreConflict::ConnectionUnpaired) => {
            IdentityError::ConnectionUnpaired
        }
        other => other.into(),
    }
}

/// A new code for `connection`: the stored row and the display form shown once.
fn new_code(
    deps: &IdentityDeps,
    connection: &Connection,
    now: DateTime<Utc>,
) -> Result<(PairingCode, PairingCodeView), IdentityError> {
    let random: [u8; 5] = deps.random_bytes();
    let code = NormalizedCode::generate(CodeKind::Pairing, &random)
        .ok_or_else(|| IdentityError::internal("pairing code generation"))?;
    let stored = PairingCode::issue(
        PairingCodeId::new_v7(),
        connection.user_id,
        connection.id,
        deps.hasher.code_hmac(&code),
        now,
    );
    let shown = PairingCodeView {
        pairing_code: code.display(),
        expires_at: stored.expires_at,
    };
    Ok((stored, shown))
}

pub struct ListConnections {
    deps: IdentityDeps,
}

impl ListConnections {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        include_unpaired: bool,
    ) -> Result<Vec<ConnectionView>, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let records = deps
            .store
            .connections(ctx.user.id, include_unpaired)
            .await?;
        let paired: Vec<ConnectionId> = records
            .iter()
            .filter(|r| r.connection.is_paired())
            .map(|r| r.connection.id)
            .collect();
        let mut accounts = accounts_of(deps, ctx.user.id, &paired).await?;
        Ok(records
            .into_iter()
            .map(|r| {
                let a = accounts.remove(&r.connection.id);
                view(r, a, now)
            })
            .collect())
    }
}

pub struct CreateConnection {
    deps: IdentityDeps,
}

impl CreateConnection {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        name: &str,
        proof: Option<&StepUpProof>,
        meta: &RequestMeta,
    ) -> Result<CreatedConnection, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let name = DisplayName::parse(name)
            .map_err(|_| IdentityError::InvalidRequest { pointer: "/name" })?;
        let grant =
            step_up::authorize(deps, ctx, StepUpAction::ConnectionCreate, None, proof, meta)
                .await?;
        let connection = Connection::create(ConnectionId::new_v7(), ctx.user.id, name, now);
        for _ in 0..CODE_ATTEMPTS {
            let (stored, shown) = new_code(deps, &connection, now)?;
            let changes = step_up::with_grant(ctx.user.id, grant.clone())
                .write(Write::InsertConnection(connection.clone()))
                .write(Write::IssuePairingCode(stored))
                .audit(user_audit(
                    ctx.user.id,
                    Actor::User(ctx.user.id),
                    AuditAction::ConnectionCreated {
                        connection_id: connection.id,
                        connection_name: connection.name.as_str().to_owned(),
                    },
                    now,
                    Some(meta.ip),
                ));
            match deps.store.commit(changes).await {
                Ok(()) => {
                    let record = ConnectionRecord {
                        connection: connection.clone(),
                        active_code_expires_at: Some(shown.expires_at),
                    };
                    return Ok(CreatedConnection {
                        connection: view(record, None, now),
                        pairing: shown,
                    });
                }
                Err(StoreError::Conflict(StoreConflict::CodeHmacTaken)) => continue,
                Err(err) => return Err(map_conflict(err)),
            }
        }
        Err(IdentityError::internal("pairing code collisions exhausted"))
    }
}

pub struct RenameConnection {
    deps: IdentityDeps,
}

impl RenameConnection {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        id: ConnectionId,
        name: &str,
        meta: &RequestMeta,
    ) -> Result<ConnectionView, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let name = DisplayName::parse(name)
            .map_err(|_| IdentityError::InvalidRequest { pointer: "/name" })?;
        let mut record = owned(deps, ctx, id).await?;
        record
            .connection
            .ensure_not_unpaired()
            .map_err(|_| IdentityError::ConnectionUnpaired)?;
        let changes = ChangeSet::new(ctx.user.id)
            .write(Write::RenameConnection {
                user_id: ctx.user.id,
                connection_id: id,
                name: name.clone(),
            })
            .audit(user_audit(
                ctx.user.id,
                Actor::User(ctx.user.id),
                AuditAction::ConnectionRenamed {
                    connection_id: id,
                    connection_name: name.as_str().to_owned(),
                    previous_name: record.connection.name.as_str().to_owned(),
                },
                now,
                Some(meta.ip),
            ));
        deps.store.commit(changes).await.map_err(map_conflict)?;
        record.connection.name = name;
        let mut accounts = accounts_of(deps, ctx.user.id, &[id]).await?;
        Ok(view(record, accounts.remove(&id), now))
    }
}

/// Invalidates any active code. For a paired connection the current instance keeps working
/// until the new code is redeemed (re-pairing).
pub struct NewPairingCode {
    deps: IdentityDeps,
}

impl NewPairingCode {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        id: ConnectionId,
        proof: Option<&StepUpProof>,
        meta: &RequestMeta,
    ) -> Result<PairingCodeView, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let record = owned(deps, ctx, id).await?;
        let grant = step_up::authorize(
            deps,
            ctx,
            StepUpAction::PairingCodeNew,
            Some(id.as_uuid()),
            proof,
            meta,
        )
        .await?;
        record
            .connection
            .ensure_not_unpaired()
            .map_err(|_| IdentityError::ConnectionUnpaired)?;
        let superseded = record.active_code_expires_at.is_some_and(|at| at > now);
        for _ in 0..CODE_ATTEMPTS {
            let (stored, shown) = new_code(deps, &record.connection, now)?;
            let mut changes = step_up::with_grant(ctx.user.id, grant.clone())
                .write(Write::IssuePairingCode(stored))
                .audit(user_audit(
                    ctx.user.id,
                    Actor::User(ctx.user.id),
                    AuditAction::PairingCodeIssued {
                        connection_id: id,
                        connection_name: record.connection.name.as_str().to_owned(),
                        code_expires_at: shown.expires_at,
                    },
                    now,
                    Some(meta.ip),
                ));
            if superseded {
                changes = changes.event(IdentityEvent::PairingCodeSuperseded {
                    user_id: ctx.user.id,
                    connection_id: id,
                    at: now,
                });
            }
            match deps.store.commit(changes).await {
                Ok(()) => return Ok(shown),
                Err(StoreError::Conflict(StoreConflict::CodeHmacTaken)) => continue,
                Err(err) => return Err(map_conflict(err)),
            }
        }
        Err(IdentityError::internal("pairing code collisions exhausted"))
    }
}

/// Revokes the device token (WebSocket closed with 4002); mapped accounts go out of sync;
/// nothing is closed. Open positions must be acknowledged.
pub struct UnpairConnection {
    deps: IdentityDeps,
}

impl UnpairConnection {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        id: ConnectionId,
        acknowledge_open_positions: bool,
        proof: Option<&StepUpProof>,
        meta: &RequestMeta,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let mut record = owned(deps, ctx, id).await?;
        if record.connection.unpaired_at.is_some() {
            return Ok(());
        }
        let grant = step_up::authorize(
            deps,
            ctx,
            StepUpAction::ConnectionUnpair,
            Some(id.as_uuid()),
            proof,
            meta,
        )
        .await?;
        let was_paired = record.connection.is_paired();
        if was_paired && !acknowledge_open_positions {
            let open = accounts_of(deps, ctx.user.id, &[id])
                .await?
                .remove(&id)
                .is_some_and(|a| !a.open_position_accounts.is_empty());
            if open {
                return Err(IdentityError::OpenPositionsNotAcknowledged);
            }
        }
        record.connection.unpair(now);
        let changes = step_up::with_grant(ctx.user.id, grant)
            .write(Write::UnpairConnection {
                user_id: ctx.user.id,
                connection_id: id,
                at: now,
            })
            .audit(user_audit(
                ctx.user.id,
                Actor::User(ctx.user.id),
                AuditAction::ConnectionUnpaired {
                    connection_id: id,
                    connection_name: record.connection.name.as_str().to_owned(),
                    open_positions_acknowledged: acknowledge_open_positions,
                },
                now,
                Some(meta.ip),
            ))
            .event(IdentityEvent::ConnectionStatusChanged {
                user_id: ctx.user.id,
                connection_id: id,
                status: ConnectionStatus::Unpaired,
                at: now,
            });
        deps.store.commit(changes).await?;
        Ok(())
    }
}

/// Handler of the gateway's adapter-session events (connected, disconnected, outdated).
pub struct RecordConnectionLiveState {
    deps: IdentityDeps,
}

impl RecordConnectionLiveState {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        user_id: UserId,
        id: ConnectionId,
        state: LiveState,
        at: DateTime<Utc>,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let Some(mut record) = deps.store.connection(user_id, id).await? else {
            return Ok(());
        };
        let connection = &mut record.connection;
        let previous = connection.status;
        if !connection.record_live_state(state, at) {
            return Ok(());
        }
        let changes = ChangeSet::new(user_id)
            .write(Write::UpdateConnectionStatus {
                user_id,
                connection_id: id,
                status: connection.status,
                status_changed_at: connection.status_changed_at,
                last_seen_at: connection.last_seen_at,
            })
            .audit(user_audit(
                user_id,
                Actor::Adapter(id),
                AuditAction::ConnectionStatusChanged {
                    connection_id: id,
                    connection_name: connection.name.as_str().to_owned(),
                    previous_status: previous,
                    status: connection.status,
                },
                at,
                None,
            ))
            .event(IdentityEvent::ConnectionStatusChanged {
                user_id,
                connection_id: id,
                status: connection.status,
                at,
            });
        deps.store.commit(changes).await?;
        Ok(())
    }
}

/// Core startup, before adapters are accepted: no adapter session survives a restart. One
/// atomic change set per user (status, audit, SSE event), so a crash midway leaves the
/// remaining users listed for the next startup.
pub struct DisconnectLiveConnectionsAtStartup {
    deps: IdentityDeps,
}

impl DisconnectLiveConnectionsAtStartup {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Returns how many connections were reset.
    pub async fn handle(&self) -> Result<usize, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let mut by_user: BTreeMap<UserId, Vec<ConnectionId>> = BTreeMap::new();
        for (user, connection) in deps.store.live_connections().await? {
            by_user.entry(user).or_default().push(connection);
        }
        let mut reset = 0;
        for (user_id, connections) in by_user {
            let mut changes = ChangeSet::new(user_id);
            for connection_id in &connections {
                let Some(record) = deps.store.connection(user_id, *connection_id).await? else {
                    continue;
                };
                changes = changes
                    .write(Write::DisconnectLiveConnection {
                        user_id,
                        connection_id: *connection_id,
                        at: now,
                    })
                    .audit(user_audit(
                        user_id,
                        Actor::System,
                        AuditAction::ConnectionStatusChanged {
                            connection_id: *connection_id,
                            connection_name: record.connection.name.as_str().to_owned(),
                            previous_status: record.connection.status,
                            status: ConnectionStatus::Disconnected,
                        },
                        now,
                        None,
                    ))
                    .event(IdentityEvent::ConnectionStatusChanged {
                        user_id,
                        connection_id: *connection_id,
                        status: ConnectionStatus::Disconnected,
                        at: now,
                    });
            }
            deps.store.commit(changes).await?;
            reset += connections.len();
        }
        Ok(reset)
    }
}
