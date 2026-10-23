//! Use-case errors. Each maps to one stable contract code; clients translate codes.

use std::time::Duration;

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
    /// One answer for every code failure (wrong, expired, used, exhausted, foreign, forged).
    #[error("auth.code_invalid")]
    CodeInvalid,
    #[error("auth.channel_unavailable")]
    ChannelUnavailable,
    #[error("auth.step_up_channel_not_allowed")]
    StepUpChannelNotAllowed,
    #[error("auth.rate_limited")]
    RateLimited { retry_after: Duration },
    #[error("auth.invitation_invalid")]
    InvitationInvalid,
    #[error("me.invalid_time_zone")]
    InvalidTimeZone,
    #[error("me.invalid_channel_value")]
    InvalidChannelValue,
    #[error("me.channel_unchanged")]
    ChannelUnchanged,
    #[error("me.channel_change_too_soon")]
    ChannelChangeTooSoon,
    #[error("me.channel_change_pending")]
    ChannelChangePending,
    #[error("me.channel_change_closed")]
    ChannelChangeClosed,
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
            Self::InvalidRequest { pointer } => Some(pointer),
            Self::InvalidChannelValue => Some("/new_value"),
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
            // A step-up code consumed with the action was used meanwhile.
            StoreError::Conflict(crate::ports::StoreConflict::CodeGone) => Self::CodeInvalid,
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
        assert_eq!(
            IdentityError::PairingRateLimited {
                retry_after: Duration::from_secs(1)
            }
            .code(),
            "pairing.rate_limited"
        );
    }
}
