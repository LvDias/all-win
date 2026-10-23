//! Registered user (invite-only, Q6).

use std::fmt;

use chrono::{DateTime, Utc};
use shared_kernel::UserId;

use crate::email::EmailAddress;
use crate::profile::{Locale, TimeZoneName};

/// argon2id PHC string; parameters live inside so they can be upgraded at next sign-in.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(phc: String) -> Self {
        Self(phc)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PasswordHash(..)")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Closed,
}

impl UserStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub email: EmailAddress,
    pub password_hash: PasswordHash,
    pub locale: Locale,
    pub time_zone: TimeZoneName,
    pub is_platform_admin: bool,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }
}
