//! `listUsers` and `adminResetTotp` (Q22). `adminCreateResetLink` lives with password reset.

use identity_enterprise::session::EndReason;
use shared_kernel::UserId;

use crate::auth::AuthContext;
use crate::changes::{Actor, AuditAction, ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::UserSummary;
use crate::usecases::common::{admin_audit, user_audit};

pub struct ListUsers {
    deps: IdentityDeps,
}

impl ListUsers {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, ctx: &AuthContext) -> Result<Vec<UserSummary>, IdentityError> {
        ctx.require_admin()?;
        Ok(self.deps.store.users(ctx.user.id).await?)
    }
}

/// Removes TOTP and recovery codes after an out-of-band identity check; ends every session of
/// the affected user. Audited in that user's log and in the admin trail.
pub struct AdminResetTotp {
    deps: IdentityDeps,
}

impl AdminResetTotp {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        ctx: &AuthContext,
        target: UserId,
        meta: &RequestMeta,
    ) -> Result<(), IdentityError> {
        ctx.require_admin()?;
        let deps = &self.deps;
        let now = deps.clock.now();
        let user = deps
            .store
            .user(target)
            .await?
            .ok_or(IdentityError::NotFound)?;
        ctx.require_step_up(now)?;
        let action = AuditAction::TotpResetByAdmin { target: user.id };
        let changes = ChangeSet::new(user.id)
            .write(Write::DeleteTotp { user_id: user.id })
            .write(Write::EndUserSessions {
                user_id: user.id,
                except: None,
                reason: EndReason::AdminTotpReset,
                at: now,
            })
            .audit(user_audit(
                user.id,
                Actor::Admin(ctx.user.id),
                action.clone(),
                now,
                Some(meta.ip),
            ))
            .audit(admin_audit(ctx.user.id, action, now, Some(meta.ip)))
            .event(IdentityEvent::SessionsEnded {
                user_id: user.id,
                sessions: EndedSessions::AllExcept(Vec::new()),
                reason: EndReason::AdminTotpReset,
                at: now,
            });
        deps.store.commit(changes).await?;
        Ok(())
    }
}
