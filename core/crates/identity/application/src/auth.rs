//! Session authentication and the authorization checks every use case relies on.

use chrono::{DateTime, Duration, Utc};
use identity_enterprise::session::{EndReason, Session, SessionStatus};
use identity_enterprise::user::User;

use crate::changes::{ChangeSet, EndedSessions, IdentityEvent, Write};
use crate::deps::IdentityDeps;
use crate::error::IdentityError;

/// Activity refreshes are written at most once per this interval per session.
const ACTIVITY_WRITE_INTERVAL: Duration = Duration::seconds(60);

/// Authenticated caller of a session operation.
#[derive(Clone, Debug)]
pub struct AuthContext {
    pub session: Session,
    pub user: User,
}

impl AuthContext {
    /// Admin operations: checked before anything else that could reveal state (`auth.forbidden`).
    pub fn require_admin(&self) -> Result<(), IdentityError> {
        if self.user.is_platform_admin {
            Ok(())
        } else {
            Err(IdentityError::Forbidden)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activity {
    /// Extends the idle timeout.
    UserInitiated,
    /// SSE stream and background polling: never extends the idle timeout.
    Background,
}

pub struct AuthenticateSession {
    deps: IdentityDeps,
}

impl AuthenticateSession {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }

    /// Resolves the session cookie value. Expired sessions are ended (`session.ended`, reason
    /// `expired`) on first sight.
    pub async fn handle(
        &self,
        cookie: Option<&str>,
        activity: Activity,
    ) -> Result<AuthContext, IdentityError> {
        let now = self.deps.clock.now();
        let Some(mut session) = self.find(cookie).await? else {
            return Err(IdentityError::Unauthenticated);
        };
        match session.status(now) {
            SessionStatus::Active => {}
            SessionStatus::Expired => {
                self.end_expired(&session, now).await?;
                return Err(IdentityError::Unauthenticated);
            }
            SessionStatus::Ended => return Err(IdentityError::Unauthenticated),
        }
        let user = match self.deps.store.user(session.user_id).await? {
            Some(user) if user.is_active() => user,
            SessionStatus::Ended => return Err(IdentityError::Unauthenticated),
        };
        if activity == Activity::UserInitiated
            && now - session.last_activity_at >= ACTIVITY_WRITE_INTERVAL
        {
            session.touch(now);
            let changes = ChangeSet::new(session.user_id).write(activity_write(&session));
            self.deps.store.commit(changes).await?;
        }
        Ok(AuthContext { session, user })
    }

    /// Any non-ended session regardless of state (logout).
    pub(crate) async fn find(
        &self,
        cookie: Option<&str>,
    ) -> Result<Option<Session>, IdentityError> {
        let Some(hash) = cookie.and_then(|c| self.deps.hash_presented_token(c)) else {
            return Ok(None);
        };
        let session = self.deps.store.session_by_token(&hash).await?;
        Ok(session.filter(|s| s.ended_at.is_none()))
    }

    async fn end_expired(
        &self,
        session: &Session,
        now: DateTime<Utc>,
    ) -> Result<(), IdentityError> {
        let changes = ChangeSet::new(session.user_id)
            .write(Write::EndSession {
                session_id: session.id,
                reason: EndReason::Expired,
                at: now,
            })
            .event(IdentityEvent::SessionsEnded {
                user_id: session.user_id,
                sessions: EndedSessions::One(session.id),
                reason: EndReason::Expired,
                at: now,
            });
        self.deps.store.commit(changes).await?;
        Ok(())
    }
}

pub(crate) fn activity_write(session: &Session) -> Write {
    Write::UpdateSessionActivity {
        session_id: session.id,
        last_activity_at: session.last_activity_at,
        idle_expires_at: session.idle_expires_at,
    }
}
