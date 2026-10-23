//! Detached code delivery (auth policy rev. 2 §2.3): the code is sent after the commit and after
//! the HTTP response, so the response time never depends on whether a code was sent. A lost send
//! is not retried; the user asks for a resend.

use std::net::IpAddr;

use identity_enterprise::ids::CodeId;
use identity_enterprise::otp::{DeliveryState, Purpose};
use shared_kernel::UserId;

use crate::changes::{Actor, AuditAction, ChangeSet, CodeTable, Write};
use crate::deps::IdentityDeps;
use crate::error::IdentityError;
use crate::ports::{CodeDelivery, DeliveryError};
use crate::usecases::common::user_audit;

/// One code to send. `suppressed` is a channel clash (the new value belongs to another account):
/// nothing is sent and the state still ends `sent`, exactly like a real send (policy §3).
#[derive(Clone, Debug)]
pub struct DeliveryJob {
    pub table: CodeTable,
    pub code_id: CodeId,
    /// The registered user the code belongs to (`None` for sign-up).
    pub user: Option<UserId>,
    pub purpose: Purpose,
    pub delivery: CodeDelivery,
    pub suppressed: bool,
    pub ip: Option<IpAddr>,
}

/// Records the outcome of a delivery: `delivery_state`, and for a registered user a
/// `security.code_delivery_failed` audit row on failure.
pub struct RecordDelivery {
    deps: IdentityDeps,
}

impl RecordDelivery {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(
        &self,
        job: &DeliveryJob,
        outcome: Result<(), DeliveryError>,
    ) -> Result<(), IdentityError> {
        let now = self.deps.clock.now();
        let state = if outcome.is_ok() || job.suppressed {
            DeliveryState::Sent
        } else {
            DeliveryState::Failed
        };
        // Sign-up codes have no owner yet; their table has no RLS, any scope works.
        let scope = job.user.unwrap_or_else(UserId::new_v7);
        let mut changes = ChangeSet::new(scope).write(Write::SetDeliveryState {
            table: job.table,
            code_id: job.code_id,
            state,
        });
        if let (Err(_), false, Some(user)) = (&outcome, job.suppressed, job.user) {
            changes = changes.audit(user_audit(
                user,
                Actor::System,
                AuditAction::CodeDeliveryFailed {
                    channel: job.delivery.destination.channel(),
                    purpose: job.purpose,
                },
                now,
                job.ip,
            ));
        }
        self.deps.store.commit(changes).await?;
        Ok(())
    }
}
