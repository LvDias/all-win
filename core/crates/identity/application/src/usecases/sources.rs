//! `listSources`, `createSource` (step-up), `renameSource`, `rotateSource` (step-up),
//! `revokeSource` (S3 AC-3, Q17). The webhook URL is returned only by create/rotate.

use std::collections::HashMap;

use identity_enterprise::ids::SourceId;
use identity_enterprise::secret::SecretString;
use identity_enterprise::source::{DisplayName, Source, SourceType};
use identity_enterprise::tokens::{KeyId, PrefixedToken, PrefixedTokenKind, SecretBytes};

use crate::auth::AuthContext;
use crate::changes::{Actor, AuditAction, ChangeSet, StoredKey, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{StoreConflict, StoreError};
use crate::usecases::common::user_audit;
use crate::views::{SourceView, SourceWithUrl};

/// A collision on a random 12-character key id is retried with a new one.
const KEY_ID_ATTEMPTS: usize = 3;

fn ingress_base(deps: &IdentityDeps, source_type: SourceType) -> String {
    format!(
        "{}/ingress/v1/{}",
        deps.settings.public_origin,
        source_type.as_str()
    )
}

fn view(
    deps: &IdentityDeps,
    source: Source,
    last_received: &HashMap<SourceId, chrono::DateTime<chrono::Utc>>,
) -> SourceView {
    SourceView {
        is_automated: source.source_type.is_automated(),
        masked_url: format!(
            "{}/awk_{}_…",
            ingress_base(deps, source.source_type),
            source.key_id.as_str()
        ),
        last_received_at: last_received.get(&source.id).copied(),
        source,
    }
}

fn new_source_token(deps: &IdentityDeps) -> (PrefixedToken, StoredKey) {
    let secret = SecretBytes::from_random(deps.random_bytes());
    let secret_hash = deps.hasher.sha256(secret.as_bytes());
    let token = PrefixedToken::new(PrefixedTokenKind::Source, deps.new_key_id(), secret);
    let key = StoredKey {
        key_id: token.key_id().clone(),
        secret_hash,
    };
    (token, key)
}

fn webhook_url(
    deps: &IdentityDeps,
    source_type: SourceType,
    token: &PrefixedToken,
) -> SecretString {
    SecretString::new(format!(
        "{}/{}",
        ingress_base(deps, source_type),
        token.expose().expose()
    ))
}

async fn owned_source(
    deps: &IdentityDeps,
    ctx: &AuthContext,
    source_id: SourceId,
) -> Result<Source, IdentityError> {
    deps.store
        .source(ctx.user.id, source_id)
        .await?
        .ok_or(IdentityError::NotFound)
}

fn map_conflict(err: StoreError) -> IdentityError {
    match err {
        StoreError::Conflict(StoreConflict::NameTaken) => IdentityError::SourceNameTaken,
        StoreError::Conflict(StoreConflict::SourceRevoked) => IdentityError::SourceRevoked,
        other => other.into(),
    }
}

pub struct ListSources {
    deps: IdentityDeps,
}

impl ListSources {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        include_revoked: bool,
    ) -> Result<Vec<SourceView>, IdentityError> {
        let deps = &self.deps;
        let sources = deps.store.sources(ctx.user.id, include_revoked).await?;
        let ids: Vec<SourceId> = sources.iter().map(|s| s.id).collect();
        let last = deps
            .source_activity
            .last_received(ctx.user.id, &ids)
            .await
            .map_err(IdentityError::Internal)?;
        Ok(sources.into_iter().map(|s| view(deps, s, &last)).collect())
    }
}

pub struct CreateSource {
    deps: IdentityDeps,
}

impl CreateSource {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        name: &str,
        source_type: SourceType,
        meta: &RequestMeta,
    ) -> Result<SourceWithUrl, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let name = DisplayName::parse(name)
            .map_err(|_| IdentityError::InvalidRequest { pointer: "/name" })?;
        ctx.require_step_up(now)?;
        let source_id = SourceId::new_v7();
        for _ in 0..KEY_ID_ATTEMPTS {
            let (token, key) = new_source_token(deps);
            let source = Source {
                id: source_id,
                user_id: ctx.user.id,
                source_type,
                name: name.clone(),
                created_at: now,
                revoked_at: None,
                key_id: key.key_id.clone(),
                rotated_at: None,
            };
            let changes = ChangeSet::new(ctx.user.id)
                .write(Write::InsertSource {
                    source: source.clone(),
                    key: key.clone(),
                })
                .audit(user_audit(
                    ctx.user.id,
                    Actor::User(ctx.user.id),
                    AuditAction::SourceCreated {
                        source_id,
                        key_id: key.key_id,
                    },
                    now,
                    Some(meta.ip),
                ));
            match deps.store.commit(changes).await {
                Ok(()) => {
                    deps.source_keys.key_activated(token.key_id());
                    return Ok(SourceWithUrl {
                        webhook_url: webhook_url(deps, source_type, &token),
                        source: view(deps, source, &HashMap::new()),
                    });
                }
                Err(StoreError::Conflict(StoreConflict::KeyIdTaken)) => continue,
                Err(err) => return Err(map_conflict(err)),
            }
        }
        Err(IdentityError::internal(
            "source key id collisions exhausted",
        ))
    }
}

pub struct RenameSource {
    deps: IdentityDeps,
}

impl RenameSource {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        source_id: SourceId,
        name: &str,
        meta: &RequestMeta,
    ) -> Result<SourceView, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let name = DisplayName::parse(name)
            .map_err(|_| IdentityError::InvalidRequest { pointer: "/name" })?;
        let mut source = owned_source(deps, ctx, source_id).await?;
        source
            .ensure_active()
            .map_err(|_| IdentityError::SourceRevoked)?;
        let changes = ChangeSet::new(ctx.user.id)
            .write(Write::RenameSource {
                user_id: ctx.user.id,
                source_id,
                name: name.clone(),
            })
            .audit(user_audit(
                ctx.user.id,
                Actor::User(ctx.user.id),
                AuditAction::SourceRenamed { source_id },
                now,
                Some(meta.ip),
            ));
        deps.store.commit(changes).await.map_err(map_conflict)?;
        source.name = name;
        let last = deps
            .source_activity
            .last_received(ctx.user.id, &[source_id])
            .await
            .map_err(IdentityError::Internal)?;
        Ok(view(deps, source, &last))
    }
}

/// Immediate rotation: the old URL stops working in the same transaction.
pub struct RotateSource {
    deps: IdentityDeps,
}

impl RotateSource {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        source_id: SourceId,
        meta: &RequestMeta,
    ) -> Result<SourceWithUrl, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let mut source = owned_source(deps, ctx, source_id).await?;
        ctx.require_step_up(now)?;
        source
            .ensure_active()
            .map_err(|_| IdentityError::SourceRevoked)?;
        let old_key_id: KeyId = source.key_id.clone();
        for _ in 0..KEY_ID_ATTEMPTS {
            let (token, key) = new_source_token(deps);
            let changes = ChangeSet::new(ctx.user.id)
                .write(Write::RotateSourceKey {
                    user_id: ctx.user.id,
                    source_id,
                    key: key.clone(),
                    at: now,
                })
                .audit(user_audit(
                    ctx.user.id,
                    Actor::User(ctx.user.id),
                    AuditAction::SourceRotated {
                        source_id,
                        old_key_id: old_key_id.clone(),
                        new_key_id: key.key_id.clone(),
                    },
                    now,
                    Some(meta.ip),
                ));
            match deps.store.commit(changes).await {
                Ok(()) => {
                    deps.source_keys.key_activated(&key.key_id);
                    source.key_id = key.key_id;
                    source.rotated_at = Some(now);
                    let last = deps
                        .source_activity
                        .last_received(ctx.user.id, &[source_id])
                        .await
                        .map_err(IdentityError::Internal)?;
                    return Ok(SourceWithUrl {
                        webhook_url: webhook_url(deps, source.source_type, &token),
                        source: view(deps, source, &last),
                    });
                }
                Err(StoreError::Conflict(StoreConflict::KeyIdTaken)) => continue,
                Err(err) => return Err(map_conflict(err)),
            }
        }
        Err(IdentityError::internal(
            "source key id collisions exhausted",
        ))
    }
}

/// No step-up: revoking reduces risk. Idempotent.
pub struct RevokeSource {
    deps: IdentityDeps,
}

impl RevokeSource {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        source_id: SourceId,
        meta: &RequestMeta,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let source = owned_source(deps, ctx, source_id).await?;
        if source.ensure_active().is_err() {
            return Ok(());
        }
        let changes = ChangeSet::new(ctx.user.id)
            .write(Write::RevokeSource {
                user_id: ctx.user.id,
                source_id,
                at: now,
            })
            .audit(user_audit(
                ctx.user.id,
                Actor::User(ctx.user.id),
                AuditAction::SourceRevoked {
                    source_id,
                    key_id: source.key_id,
                },
                now,
                Some(meta.ip),
            ));
        deps.store.commit(changes).await?;
        Ok(())
    }
}
