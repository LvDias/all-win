//! Access port (ADR-0007, Esquadro 2026-10-23): what other business modules need from identity,
//! without depending on it (ADR-0001 §4). `identity` implements the traits; `app` injects them.
//!
//! - [`SessionAuthenticator`] resolves the session cookie to the authenticated [`Caller`].
//! - [`StepUpGate`] checks the `step_up: { challenge_id, code }` proof of a sensitive action and
//!   consumes the code atomically in identity's own commit, **before** the action runs (option
//!   (a)): two parallel requests with one code cannot both pass, and a code spent by an action that
//!   then fails (409/422, crash) is not reusable.
//!
//! Handler order (contract precedence): 401 → 403 origin → 403 admin → 400 → 404 (including the
//! target's owner) → [`StepUpGate::authorize`] → 409/422. [`AccessError::Unavailable`] is a 500:
//! fail closed, the action never runs without a grant.

#[cfg(feature = "axum")]
pub mod http;

use std::fmt;

use async_trait::async_trait;
use shared_kernel::UserId;
use thiserror::Error;
use uuid::Uuid;

/// The authenticated caller of a request: the minimum other modules need.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Caller {
    pub user_id: UserId,
    pub session_id: Uuid,
    pub device_id: Uuid,
    pub is_platform_admin: bool,
}

/// Whether the request extends the session's idle timeout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activity {
    UserInitiated,
    /// SSE and background polling never extend the idle timeout.
    Background,
}

/// HTTP 2.0.0 `StepUpAction` (auth policy §6.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StepUpAction {
    SourceCreate,
    SourceRotate,
    ConnectionCreate,
    PairingCodeNew,
    ConnectionUnpair,
    AccountOptInAccept,
    AccountCreateOptInAccept,
    AdminInvitationCreate,
    RouteEnable,
    AccountBindingChange,
    /// Q95: "clear out of sync" on an account.
    AccountSyncClear,
    /// Q96: "restart today's count" on an account.
    TradingDayRestart,
}

impl StepUpAction {
    pub const ALL: [Self; 12] = [
        Self::SourceCreate,
        Self::SourceRotate,
        Self::ConnectionCreate,
        Self::PairingCodeNew,
        Self::ConnectionUnpair,
        Self::AccountOptInAccept,
        Self::AccountCreateOptInAccept,
        Self::AdminInvitationCreate,
        Self::RouteEnable,
        Self::AccountBindingChange,
        Self::AccountSyncClear,
        Self::TradingDayRestart,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceCreate => "source_create",
            Self::SourceRotate => "source_rotate",
            Self::ConnectionCreate => "connection_create",
            Self::PairingCodeNew => "pairing_code_new",
            Self::ConnectionUnpair => "connection_unpair",
            Self::AccountOptInAccept => "account_opt_in_accept",
            Self::AccountCreateOptInAccept => "account_create_opt_in_accept",
            Self::AdminInvitationCreate => "admin_invitation_create",
            Self::RouteEnable => "route_enable",
            Self::AccountBindingChange => "account_binding_change",
            Self::AccountSyncClear => "account_sync_clear",
            Self::TradingDayRestart => "trading_day_restart",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.as_str() == raw)
    }

    /// `target_id` is required for these and forbidden for the others (contract `requestStepUp`).
    pub fn requires_target(self) -> bool {
        matches!(
            self,
            Self::SourceRotate
                | Self::PairingCodeNew
                | Self::ConnectionUnpair
                | Self::AccountOptInAccept
                | Self::RouteEnable
                | Self::AccountBindingChange
                | Self::AccountSyncClear
                | Self::TradingDayRestart
        )
    }

    /// Required only while the Core setting `step_up.q35_persistence_actions` is on; the others
    /// are always required (Esquadro, 2026-10-23).
    pub fn is_q35_conditional(self) -> bool {
        matches!(
            self,
            Self::SourceCreate
                | Self::SourceRotate
                | Self::ConnectionCreate
                | Self::PairingCodeNew
                | Self::AdminInvitationCreate
                | Self::RouteEnable
                | Self::AccountBindingChange
                | Self::AccountSyncClear
                | Self::TradingDayRestart
        )
    }

    /// Persistence actions: from a new device they need the e-mail code (M3, policy §6.2).
    pub fn is_persistence(self) -> bool {
        matches!(
            self,
            Self::SourceCreate
                | Self::SourceRotate
                | Self::ConnectionCreate
                | Self::PairingCodeNew
                | Self::AccountOptInAccept
                | Self::AccountCreateOptInAccept
        )
    }
}

/// `step_up: { challenge_id, code }` from the request body. Never printed.
#[derive(Clone)]
pub struct StepUpProof {
    pub challenge_id: String,
    pub code: String,
}

impl fmt::Debug for StepUpProof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StepUpProof(REDACTED)")
    }
}

/// Proof that the step-up of one action on one target passed for one session, in this request.
/// Neither `Clone` nor serializable; only a [`StepUpGate`] implementation creates it.
#[derive(Debug, PartialEq, Eq)]
pub struct StepUpGrant {
    session_id: Uuid,
    action: StepUpAction,
    target_id: Option<Uuid>,
    /// False when the Core setting does not require a step-up for this action.
    proven: bool,
}

impl StepUpGrant {
    /// For [`StepUpGate`] implementations only.
    #[doc(hidden)]
    pub fn issue(
        caller: &Caller,
        action: StepUpAction,
        target_id: Option<Uuid>,
        proven: bool,
    ) -> Self {
        Self {
            session_id: caller.session_id,
            action,
            target_id,
            proven,
        }
    }

    /// The grant covers exactly this caller, action and target.
    pub fn covers(&self, caller: &Caller, action: StepUpAction, target_id: Option<Uuid>) -> bool {
        self.session_id == caller.session_id && self.action == action && self.target_id == target_id
    }

    /// Whether a code was actually consumed (false: not required by the Core setting).
    pub fn proven(&self) -> bool {
        self.proven
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AccessError {
    /// 401 `auth.unauthenticated`.
    #[error("auth.unauthenticated")]
    Unauthenticated,
    /// 403 `auth.step_up_required`: the body has no proof.
    #[error("auth.step_up_required")]
    StepUpRequired,
    /// 403 `auth.code_invalid`: wrong, expired, used, or bound to another session, action or target.
    #[error("auth.code_invalid")]
    CodeInvalid,
    /// 429 `auth.rate_limited` (wrong attempts per IP).
    #[error("auth.rate_limited")]
    RateLimited { retry_after_s: u64 },
    /// 500 `common.internal`; the action must not run.
    #[error("common.internal")]
    Unavailable,
}

impl AccessError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Unauthenticated => "auth.unauthenticated",
            Self::StepUpRequired => "auth.step_up_required",
            Self::CodeInvalid => "auth.code_invalid",
            Self::RateLimited { .. } => "auth.rate_limited",
            Self::Unavailable => "common.internal",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::Unauthenticated => 401,
            Self::StepUpRequired | Self::CodeInvalid => 403,
            Self::RateLimited { .. } => 429,
            Self::Unavailable => 500,
        }
    }
}

#[async_trait]
pub trait SessionAuthenticator: Send + Sync {
    /// Resolves the `__Host-aw_session` cookie value.
    async fn authenticate(
        &self,
        session_cookie: Option<&str>,
        activity: Activity,
    ) -> Result<Caller, AccessError>;
}

/// Request facts the gate audits (never the URL or headers).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestOrigin {
    pub ip: std::net::IpAddr,
}

#[async_trait]
pub trait StepUpGate: Send + Sync {
    /// Checks and consumes the proof for `action` on `target_id`, audits the outcome in the
    /// caller's log, and returns the grant the action needs. Call it after the 400/404 checks
    /// (including the target's owner) and before anything that can answer 409/422.
    async fn authorize(
        &self,
        caller: &Caller,
        action: StepUpAction,
        target_id: Option<Uuid>,
        proof: Option<StepUpProof>,
        origin: &RequestOrigin,
    ) -> Result<StepUpGrant, AccessError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caller() -> Caller {
        Caller {
            user_id: UserId::new_v7(),
            session_id: Uuid::now_v7(),
            device_id: Uuid::now_v7(),
            is_platform_admin: false,
        }
    }

    #[test]
    fn grant_covers_only_its_session_action_and_target() {
        let c = caller();
        let t = Uuid::now_v7();
        let g = StepUpGrant::issue(&c, StepUpAction::ConnectionUnpair, Some(t), true);
        assert!(g.covers(&c, StepUpAction::ConnectionUnpair, Some(t)));
        assert!(!g.covers(&c, StepUpAction::ConnectionUnpair, Some(Uuid::now_v7())));
        assert!(!g.covers(&c, StepUpAction::PairingCodeNew, Some(t)));
        assert!(!g.covers(&caller(), StepUpAction::ConnectionUnpair, Some(t)));
    }

    #[test]
    fn action_table_and_errors() {
        for a in StepUpAction::ALL {
            assert_eq!(StepUpAction::parse(a.as_str()), Some(a));
        }
        assert!(!StepUpAction::AccountOptInAccept.is_q35_conditional());
        assert!(StepUpAction::AccountBindingChange.requires_target());
        for a in [
            StepUpAction::AccountSyncClear,
            StepUpAction::TradingDayRestart,
        ] {
            assert!(a.requires_target() && a.is_q35_conditional() && !a.is_persistence());
        }
        assert_eq!(AccessError::Unavailable.http_status(), 500);
        assert_eq!(AccessError::CodeInvalid.http_status(), 403);
        let p = StepUpProof {
            challenge_id: "secret-token".into(),
            code: "K7Q2-0111".into(),
        };
        assert!(!format!("{p:?}").contains("K7Q2"));
    }
}
