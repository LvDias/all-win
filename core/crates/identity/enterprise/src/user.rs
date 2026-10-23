//! Registered user (invite-only, Q6).

use chrono::{DateTime, Utc};
use shared_kernel::UserId;

use crate::email::EmailAddress;
use crate::profile::{Locale, TimeZoneName};

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
