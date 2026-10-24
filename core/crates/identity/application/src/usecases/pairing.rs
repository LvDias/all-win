//! `pairAdapter` (`POST /adapter/v1/pairings`, ADR-0004 §8): exchanges a pairing code for a
//! device token. The request carries only the code, so limits apply per IP and per prefix
//! (see [`crate::throttle::PairingThrottle`]; no global lock, U11).

use std::net::IpAddr;

use identity_enterprise::codes::{CodeKind, NormalizedCode};
use identity_enterprise::connection::{AdapterInfo, ConnectionStatus, PairedInstance};
use identity_enterprise::tokens::{PrefixedToken, PrefixedTokenKind, SecretBytes};
use uuid::Uuid;

use crate::changes::{
    Actor, AuditAction, ChangeSet, IdentityEvent, StoredKey, TokenRevokedReason, Write,
};
use crate::deps::IdentityDeps;
use crate::error::IdentityError;
use crate::ports::{StoreConflict, StoreError};
use crate::throttle::PairingDecision;
use crate::usecases::common::user_audit;
use crate::views::PairingResult;

const KEY_ID_ATTEMPTS: usize = 3;

pub struct PairAdapterCommand {
    pub pairing_code: String,
    pub instance_id: Uuid,
    pub instance_name: String,
    pub adapter: AdapterInfo,
    pub ip: IpAddr,
}

pub struct PairAdapter {
    deps: IdentityDeps,
}

impl PairAdapter {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cmd: PairAdapterCommand) -> Result<PairingResult, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        match deps.pairing_throttle.admit(cmd.ip, now).await {
            PairingDecision::Allow => {}
            PairingDecision::RateLimited { retry_after } => {
                return Err(IdentityError::PairingRateLimited { retry_after });
            }
        }
        if !deps.adapters.is_supported(&cmd.adapter) {
            return Err(IdentityError::AdapterVersionUnsupported);
        }
        match self.redeem(&cmd).await {
            Err(IdentityError::PairingInvalidCode) => {
                deps.pairing_throttle.record_failure(cmd.ip, now).await;
                Err(IdentityError::PairingInvalidCode)
            }
            other => other,
        }
    }

    async fn redeem(&self, cmd: &PairAdapterCommand) -> Result<PairingResult, IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let code = NormalizedCode::parse(CodeKind::Pairing, &cmd.pairing_code)
            .ok_or(IdentityError::PairingInvalidCode)?;
        let stored = deps
            .store
            .redeemable_pairing_code(&deps.hasher.code_hmac_candidates(&code), now)
            .await?
            .filter(|c| c.is_redeemable(now))
            .ok_or(IdentityError::PairingInvalidCode)?;
        let user_id = stored.user_id;
        let mut record = deps
            .store
            .connection(user_id, stored.connection_id)
            .await?
            .filter(|r| r.connection.unpaired_at.is_none())
            .ok_or(IdentityError::PairingInvalidCode)?;
        let replaced_previous = record.connection.is_paired();
        let instance = PairedInstance {
            instance_id: cmd.instance_id,
            instance_name: cmd.instance_name.clone(),
            adapter: cmd.adapter.clone(),
            paired_at: now,
        };
        record
            .connection
            .pair(instance.clone())
            .map_err(|_| IdentityError::PairingInvalidCode)?;
        let connection_id = record.connection.id;

        for _ in 0..KEY_ID_ATTEMPTS {
            let secret = SecretBytes::from_random(deps.random_bytes());
            let device_key = StoredKey {
                key_id: deps.new_key_id(),
                secret_hash: deps.hasher.sha256(secret.as_bytes()),
            };
            let token =
                PrefixedToken::new(PrefixedTokenKind::Device, device_key.key_id.clone(), secret);
            let changes = ChangeSet::new(user_id)
                .write(Write::UsePairingCode {
                    code_id: stored.id,
                    at: now,
                })
                .write(Write::PairConnection {
                    user_id,
                    connection_id,
                    instance: instance.clone(),
                    device_key: device_key.clone(),
                })
                .audit(user_audit(
                    user_id,
                    Actor::Adapter(connection_id),
                    AuditAction::ConnectionPaired {
                        connection_id,
                        connection_name: record.connection.name.as_str().to_owned(),
                        instance_name: cmd.instance_name.clone(),
                        adapter_version: cmd.adapter.version.clone(),
                        repaired: replaced_previous,
                    },
                    now,
                    Some(cmd.ip),
                ))
                .event(IdentityEvent::ConnectionPaired {
                    user_id,
                    connection_id,
                    instance_name: cmd.instance_name.clone(),
                    at: now,
                })
                .event(IdentityEvent::ConnectionStatusChanged {
                    user_id,
                    connection_id,
                    status: ConnectionStatus::Disconnected,
                    at: now,
                });
            // A re-pair revokes the previous instance's token in the same transaction.
            let changes = if replaced_previous {
                changes.event(IdentityEvent::DeviceTokenRevoked {
                    user_id,
                    connection_id,
                    reason: TokenRevokedReason::Repaired,
                    at: now,
                })
            } else {
                changes
            };
            match deps.store.commit(changes).await {
                Ok(()) => {
                    return Ok(PairingResult {
                        connection_id,
                        device_token: token.expose(),
                    });
                }
                Err(StoreError::Conflict(StoreConflict::KeyIdTaken)) => continue,
                Err(StoreError::Conflict(
                    StoreConflict::PairingCodeGone | StoreConflict::ConnectionUnpaired,
                )) => {
                    return Err(IdentityError::PairingInvalidCode);
                }
                Err(err) => return Err(err.into()),
            }
        }
        Err(IdentityError::internal(
            "device key id collisions exhausted",
        ))
    }
}
