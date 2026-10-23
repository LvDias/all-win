//! Profile use-case errors: stable codes (clients translate them) and the field they point at.
//! Precedence (as accounts): 404 → 409 → 422.

use accounts_enterprise::profile::{
    CopySourceError, DistanceRejected, RestoreError, VersionConflict,
};
use thiserror::Error;

use super::ports::ProfileStoreError;
use crate::ports::PortError;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ProfilesError {
    /// 404 `common.not_found`: unknown/archived/other user's account, instrument not enabled, or
    /// a size that is not standard (never 403).
    #[error("common.not_found")]
    NotFound,
    #[error("common.version_conflict")]
    VersionConflict,
    /// 422 `profiles.distance_below_one_tick` / `profiles.distance_too_large`.
    #[error("{0}")]
    Distance(DistanceRejected),
    #[error("profiles.default_missing")]
    DefaultMissing,
    #[error("profiles.profile_size_required")]
    ProfileSizeRequired,
    #[error("profiles.profile_size_invalid")]
    ProfileSizeInvalid,
    #[error("internal: {0}")]
    Internal(String),
}

impl ProfilesError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound => "common.not_found",
            Self::VersionConflict => "common.version_conflict",
            Self::Distance(d) => d.rule.code(),
            Self::DefaultMissing => "profiles.default_missing",
            Self::ProfileSizeRequired => "profiles.profile_size_required",
            Self::ProfileSizeInvalid => "profiles.profile_size_invalid",
            Self::Internal(_) => "common.internal",
        }
    }

    /// The body or query field the error points at (the HTTP layer adds `/` or `/query/`).
    pub fn field(&self) -> Option<&'static str> {
        match self {
            Self::Distance(d) => Some(d.side.field()),
            Self::ProfileSizeRequired | Self::ProfileSizeInvalid => Some("profile_size"),
            _ => None,
        }
    }
}

impl From<PortError> for ProfilesError {
    fn from(e: PortError) -> Self {
        Self::Internal(e.0)
    }
}

impl From<ProfileStoreError> for ProfilesError {
    fn from(e: ProfileStoreError) -> Self {
        match e {
            ProfileStoreError::AccountNotFound => Self::NotFound,
            ProfileStoreError::Raced => Self::VersionConflict,
            ProfileStoreError::Other(m) => Self::Internal(m),
        }
    }
}

impl From<DistanceRejected> for ProfilesError {
    fn from(e: DistanceRejected) -> Self {
        Self::Distance(e)
    }
}

impl From<VersionConflict> for ProfilesError {
    fn from(_: VersionConflict) -> Self {
        Self::VersionConflict
    }
}

impl From<CopySourceError> for ProfilesError {
    fn from(e: CopySourceError) -> Self {
        match e {
            CopySourceError::ProfileSizeRequired => Self::ProfileSizeRequired,
            CopySourceError::ProfileSizeInvalid => Self::ProfileSizeInvalid,
        }
    }
}

impl From<RestoreError> for ProfilesError {
    fn from(e: RestoreError) -> Self {
        match e {
            RestoreError::VersionConflict(v) => v.into(),
            RestoreError::DefaultMissing => Self::DefaultMissing,
            RestoreError::Source(s) => s.into(),
        }
    }
}
