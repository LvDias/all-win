//! `createInvitation`, `revokeInvitation`, `listInvitations` (admin) and the public
//! `lookupInvitation` (S3 AC-1, Q6/Q19).

use identity_enterprise::email::EmailAddress;
use identity_enterprise::ids::InvitationId;
use identity_enterprise::invitation::{Invitation, InvitationStatus};
use identity_enterprise::profile::Locale;

use crate::auth::AuthContext;
use crate::changes::{AuditAction, ChangeSet, Write};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::ports::{StoreConflict, StoreError};
use crate::usecases::common::{admin_audit, check_public_throttle, fragment_link};
use crate::views::{CreatedInvitation, InvitationLookup, InvitationView};

pub struct CreateInvitationCommand {
    pub email: String,
    pub locale: Locale,
    pub send_email: bool,
    pub meta: RequestMeta,
}

pub struct CreateInvitation {
    deps: IdentityDeps,
}

impl CreateInvitation {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Returns the link once. The invitation is saved before the e-mail is attempted, so a link
    /// that was e-mailed always exists; `email_sent = false` tells the admin to deliver it.
    pub async fn handle(
        &self,
        ctx: &AuthContext,
        cmd: CreateInvitationCommand,
    ) -> Result<CreatedInvitation, IdentityError> {
        ctx.require_admin()?;
        let deps = &self.deps;
        let now = deps.clock.now();
        let email = EmailAddress::parse(&cmd.email)
            .map_err(|_| IdentityError::InvalidRequest { pointer: "/email" })?;
        if deps
            .store
            .email_registered(ctx.user.id, email.normalized())
            .await?
        {
            return Err(IdentityError::InvitationEmailTaken);
        }
        let mut changes = ChangeSet::new(ctx.user.id);
        if let Some(open) = deps.store.open_invitation_for(email.normalized()).await? {
            if open.status(now) == InvitationStatus::Pending {
                return Err(IdentityError::InvitationEmailTaken);
            }
            changes = changes.write(Write::SupersedeExpiredInvitation {
                invitation_id: open.id,
                at: now,
            });
        }
        let token = deps.issue_token();
        let invitation = Invitation::issue(
            InvitationId::new_v7(),
            email.normalized().to_owned(),
            cmd.locale,
            ctx.user.id,
            now,
        );
        let changes = changes
            .write(Write::InsertInvitation {
                invitation: invitation.clone(),
                token_hash: token.hash,
            })
            .audit(admin_audit(
                ctx.user.id,
                AuditAction::InvitationIssued {
                    invitation_id: invitation.id,
                },
                now,
                Some(cmd.meta.ip),
            ));
        match deps.store.commit(changes).await {
            Ok(()) => {}
            Err(StoreError::Conflict(StoreConflict::EmailTaken)) => {
                return Err(IdentityError::InvitationEmailTaken);
            }
            Err(err) => return Err(err.into()),
        }
        let invite_url = fragment_link(deps, cmd.locale.as_str(), "invite", &token);
        let email_sent = cmd.send_email
            && deps
                .mailer
                .send_invitation(email.as_entered(), cmd.locale, &invite_url)
                .await
                .is_ok();
        Ok(CreatedInvitation {
            invitation: InvitationView {
                status: invitation.status(now),
                invitation,
            },
            invite_url,
            email_sent,
        })
    }
}

pub struct RevokeInvitation {
    deps: IdentityDeps,
}

impl RevokeInvitation {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Idempotent; revoking an accepted or already closed invitation changes nothing.
    pub async fn handle(
        &self,
        ctx: &AuthContext,
        invitation_id: InvitationId,
        meta: &RequestMeta,
    ) -> Result<(), IdentityError> {
        ctx.require_admin()?;
        let deps = &self.deps;
        let now = deps.clock.now();
        let invitation = deps
            .store
            .invitation(invitation_id)
            .await?
            .ok_or(IdentityError::NotFound)?;
        if !invitation.is_open() {
            return Ok(());
        }
        let changes = ChangeSet::new(ctx.user.id)
            .write(Write::RevokeInvitation {
                invitation_id,
                at: now,
            })
            .audit(admin_audit(
                ctx.user.id,
                AuditAction::InvitationRevoked { invitation_id },
                now,
                Some(meta.ip),
            ));
        deps.store.commit(changes).await?;
        Ok(())
    }
}

pub struct ListInvitations {
    deps: IdentityDeps,
}

impl ListInvitations {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, ctx: &AuthContext) -> Result<Vec<InvitationView>, IdentityError> {
        ctx.require_admin()?;
        let now = self.deps.clock.now();
        Ok(self
            .deps
            .store
            .invitations()
            .await?
            .into_iter()
            .map(|invitation| InvitationView {
                status: invitation.status(now),
                invitation,
            })
            .collect())
    }
}

/// Resolves a presented invitation token to a pending invitation; every other case is the
/// single `auth.invitation_invalid`.
pub(crate) async fn pending_invitation(deps: &IdentityDeps, token: &str) -> Result<Invitation, IdentityError> {
    let now = deps.clock.now();
    let Some(hash) = deps.hash_presented_token(token) else {
        return Err(IdentityError::InvitationInvalid);
    };
    match deps.store.invitation_by_token(&hash).await? {
        Some(invitation) if invitation.status(now) == InvitationStatus::Pending => Ok(invitation),
        _ => Err(IdentityError::InvitationInvalid),
    }
}

pub struct LookupInvitation {
    deps: IdentityDeps,
}

impl LookupInvitation {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        token: &str,
        meta: &RequestMeta,
    ) -> Result<InvitationLookup, IdentityError> {
        check_public_throttle(&self.deps, meta.ip, self.deps.clock.now()).await?;
        let invitation = pending_invitation(&self.deps, token).await?;
        Ok(InvitationLookup {
            email: invitation.email,
            locale: invitation.locale,
            expires_at: invitation.expires_at,
        })
    }
}
