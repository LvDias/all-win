//! Use-case errors. Each maps to one stable contract code; clients translate codes.

use std::time::Duration;

use identity_enterprise::password::PasswordRejection;
use thiserror::Error;

use crate::ports::StoreError;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("auth.unauthenticated")]
    Unauthenticated,
    #[error("auth.forbidden")]
    Forbidden,
    #[error("auth.step_up_required")]
    StepUpRequired,
    #[error("auth.invalid_credentials")]
    InvalidCredentials,
    #[error("auth.invalid_code")]
    InvalidCode,
    #[error("auth.pending_expired")]
    PendingExpired,
    #[error("auth.reauth_failed")]
    ReauthFailed,
    #[error("auth.rate_limited")]
    RateLimited { retry_after: Duration },
    #[error("auth.invitation_invalid")]
    InvitationInvalid,
    #[error("auth.reset_token_invalid")]
    ResetTokenInvalid,
    #[error("{}", .rejection.code())]
    PasswordRejected {
        rejection: PasswordRejection,
        pointer: &'static str,
    },
    #[error("auth.totp_already_enabled")]
    TotpAlreadyEnabled,
    #[error("auth.totp_setup_expired")]
    TotpSetupExpired,
    #[error("me.invalid_time_zone")]
    InvalidTimeZone,
    /// Unknown resource or another user's resource (never 403, S3 AC-2).
    #[error("common.not_found")]
    NotFound,
    /// Semantically invalid input that passed the schema; `pointer` locates the field.
    #[error("common.invalid_request")]
    InvalidRequest { pointer: &'static str },
    #[error("invites.email_taken")]
    InvitationEmailTaken,
    #[error("sources.name_taken")]
    SourceNameTaken,
    #[error("sources.revoked")]
    SourceRevoked,
    #[error("connections.name_taken")]
    ConnectionNameTaken,
    #[error("connections.unpaired")]
    ConnectionUnpaired,
    #[error("connections.open_positions_not_acknowledged")]
    OpenPositionsNotAcknowledged,
    #[error("pairing.invalid_code")]
    PairingInvalidCode,
    #[error("pairing.rate_limited")]
    PairingRateLimited { retry_after: Duration },
    #[error("pairing.adapter_version_unsupported")]
    AdapterVersionUnsupported,
    /// Unexpected failure; the detail is for technical logs only, never for the response body.
    #[error("common.internal")]
    Internal(String),
}

impl IdentityError {
    /// Stable contract code (`error.code`).
    pub fn code(&self) -> String {
        self.to_string()
    }

    /// `error.pointer`, when the contract defines one for this error.
    pub fn pointer(&self) -> Option<&'static str> {
        match self {
            Self::PasswordRejected { pointer, .. } | Self::InvalidRequest { pointer } => {
                Some(pointer)
            }
            _ => None,
        }
    }

    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::RateLimited { retry_after } | Self::PairingRateLimited { retry_after } => {
                Some(*retry_after)
            }
            _ => None,
        }
    }

    pub(crate) fn internal(detail: impl Into<String>) -> Self {
        Self::Internal(detail.into())
    }
}

impl From<StoreError> for IdentityError {
    fn from(err: StoreError) -> Self {
        match err {
            StoreError::Conflict(conflict) => {
                Self::internal(format!("unexpected store conflict: {conflict:?}"))
            }
            StoreError::Unavailable(detail) => Self::Internal(detail),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_follow_the_contract() {
        assert_eq!(
            IdentityError::Unauthenticated.code(),
            "auth.unauthenticated"
        );
        assert_eq!(
            IdentityError::Internal("db down at 10.0.0.1".into()).code(),
            "common.internal"
        );
        let e = IdentityError::PasswordRejected {
            rejection: PasswordRejection::Breached,
            pointer: "/password",
        };
        assert_eq!(e.code(), "auth.password_breached");
        assert_eq!(e.pointer(), Some("/password"));
        assert_eq!(
            IdentityError::PairingRateLimited {
                retry_after: Duration::from_secs(1)
            }
            .code(),
            "pairing.rate_limited"
        );
    }
}
