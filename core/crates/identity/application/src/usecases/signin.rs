//! `logout`.

use identity_enterprise::session::EndReason;

use crate::auth::AuthenticateSession;
use crate::changes::{ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::IdentityDeps;
use crate::error::IdentityError;

/// Idempotent: accepts a full or pending session, or none.
pub struct Logout {
    deps: IdentityDeps,
}

impl Logout {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    pub async fn handle(&self, cookie: Option<&str>) -> Result<(), IdentityError> {
        let Some(session) = AuthenticateSession::new(self.deps.clone())
            .find(cookie)
            .await?
        else {
            return Ok(());
        };
        let now = self.deps.clock.now();
        let changes = ChangeSet::new(session.user_id)
            .write(Write::EndSession {
                session_id: session.id,
                reason: EndReason::Logout,
                at: now,
            })
            .event(IdentityEvent::SessionsEnded {
                user_id: session.user_id,
                sessions: EndedSessions::One(session.id),
                reason: EndReason::Logout,
                at: now,
            });
        self.deps.store.commit(changes).await?;
        Ok(())
    }
}
