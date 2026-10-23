//! identity's implementation of the access port (ADR-0007) used by other modules: session
//! authentication and the step-up gate. For actions of other modules the code is consumed in
//! identity's own commit before the action runs (option (a)).

use access::{
    AccessError, Activity as PortActivity, Caller, RequestOrigin, SessionAuthenticator,
    StepUpAction, StepUpGate, StepUpGrant, StepUpProof,
};
use async_trait::async_trait;
use identity_enterprise::ids::SessionId;
use identity_enterprise::session::SessionStatus;
use uuid::Uuid;

use crate::auth::{Activity, AuthContext, AuthenticateSession};
use crate::deps::{IdentityDeps, RequestMeta};
use crate::error::IdentityError;
use crate::usecases::step_up;

pub struct IdentityAccess {
    deps: IdentityDeps,
}

impl IdentityAccess {
    pub fn new(deps: IdentityDeps) -> Self {
        Self { deps }
    }
}

pub(crate) fn caller_of(ctx: &AuthContext) -> Caller {
    Caller {
        user_id: ctx.user.id,
        session_id: ctx.session.id.as_uuid(),
        device_id: ctx.session.device_id.as_uuid(),
        is_platform_admin: ctx.user.is_platform_admin,
    }
}

fn port_error(e: IdentityError) -> AccessError {
    match e {
        IdentityError::Unauthenticated => AccessError::Unauthenticated,
        IdentityError::StepUpRequired => AccessError::StepUpRequired,
        IdentityError::CodeInvalid => AccessError::CodeInvalid,
        IdentityError::RateLimited { retry_after } => AccessError::RateLimited {
            retry_after_s: retry_after.as_secs().max(1),
        },
        _ => AccessError::Unavailable,
    }
}

#[async_trait]
impl SessionAuthenticator for IdentityAccess {
    async fn authenticate(
        &self,
        session_cookie: Option<&str>,
        activity: PortActivity,
    ) -> Result<Caller, AccessError> {
        let activity = match activity {
            PortActivity::UserInitiated => Activity::UserInitiated,
            PortActivity::Background => Activity::Background,
        };
        AuthenticateSession::new(self.deps.clone())
            .handle(session_cookie, activity)
            .await
            .map(|ctx| caller_of(&ctx))
            .map_err(port_error)
    }
}

#[async_trait]
impl StepUpGate for IdentityAccess {
    async fn authorize(
        &self,
        caller: &Caller,
        action: StepUpAction,
        target_id: Option<Uuid>,
        proof: Option<StepUpProof>,
        origin: &RequestOrigin,
    ) -> Result<StepUpGrant, AccessError> {
        let deps = &self.deps;
        let now = deps.clock.now();
        let session = deps
            .store
            .session(caller.user_id, SessionId::from_uuid(caller.session_id))
            .await
            .map_err(|_| AccessError::Unavailable)?;
        let user = deps
            .store
            .user(caller.user_id)
            .await
            .map_err(|_| AccessError::Unavailable)?;
        let (Some(session), Some(user)) = (session, user) else {
            return Err(AccessError::Unauthenticated);
        };
        if session.status(now) != SessionStatus::Active || !user.is_active() {
            return Err(AccessError::Unauthenticated);
        }
        let ctx = AuthContext { session, user };
        let meta = RequestMeta {
            ip: origin.ip,
            user_agent: None,
        };
        let grant = step_up::authorize(deps, &ctx, action, target_id, proof.as_ref(), &meta)
            .await
            .map_err(port_error)?;
        let proven = grant.is_some();
        if let Some(changes) = grant {
            deps.store
                .commit(changes)
                .await
                .map_err(|e| port_error(e.into()))?;
        }
        Ok(StepUpGrant::issue(caller, action, target_id, proven))
    }
}
