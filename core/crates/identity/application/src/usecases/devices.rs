//! `listDevices`, `forgetDevice` (auth policy rev. 2 §5.2). Forgetting a device revokes it and ends
//! its sessions; its cookie then counts as none, and a pending channel change requested from it
//! is cancelled (Q88 (a)). No step-up: it only reduces access.

use identity_enterprise::ids::DeviceId;
use identity_enterprise::session::EndReason;

use crate::auth::AuthContext;
use crate::changes::{Actor, AuditAction, ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::usecases::channel_change::{CancelScope, cancel_pending_with, commit_with_cancel};
use crate::usecases::common::user_audit;
use crate::views::DeviceView;

pub struct ListDevices {
    deps: IdentityDeps,
}

impl ListDevices {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, ctx: &AuthContext) -> Result<Vec<DeviceView>, IdentityError> {
        let mut devices = self.deps.store.devices(ctx.user.id).await?;
        devices.sort_by_key(|d| std::cmp::Reverse(d.last_seen_at));
        Ok(devices
            .into_iter()
            .map(|d| DeviceView {
                current: d.id == ctx.session.device_id,
                label: d.display_label().to_owned(),
                id: d.id,
                first_seen_at: d.first_seen_at,
                last_seen_at: d.last_seen_at,
            })
            .collect())
    }
}

/// Idempotent; another user's device is 404.
pub struct ForgetDevice {
    deps: IdentityDeps,
}

impl ForgetDevice {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        device_id: DeviceId,
        meta: &RequestMeta,
    ) -> Result<(), IdentityError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let user = ctx.user.id;
        let Some(device) = deps.store.device(user, device_id).await? else {
            return Err(IdentityError::NotFound);
        };
        if device.is_revoked() {
            return Ok(());
        }
        let changes = ChangeSet::new(user)
            .write(Write::RevokeDevice {
                user_id: user,
                device_id,
                at: now,
            })
            .write(Write::EndDeviceSessions {
                user_id: user,
                device_id,
                reason: EndReason::DeviceRevoked,
                at: now,
            })
            .audit(user_audit(
                user,
                Actor::User(user),
                AuditAction::DeviceForgotten {
                    device_id,
                    device_label: device.display_label().to_owned(),
                },
                now,
                Some(meta.ip),
            ))
            .event(IdentityEvent::SessionsEnded {
                user_id: user,
                sessions: EndedSessions::Device(device_id),
                reason: EndReason::DeviceRevoked,
                at: now,
            });
        // Q88 (a): a pending channel change requested from this device is cancelled with it.
        let with_cancel = cancel_pending_with(
            deps,
            ctx,
            CancelScope::RequestedFrom(device_id),
            changes.clone(),
            meta,
        )
        .await?;
        commit_with_cancel(deps, with_cancel, changes).await
    }
}
