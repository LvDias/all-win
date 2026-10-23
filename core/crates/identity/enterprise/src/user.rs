//! Registered user (invite-only, Q6). There is no password: the confirmed channels (the e-mail,
//! later WhatsApp) are the only way to sign in (auth policy rev. 2 §1).

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;

use crate::channel::{Channel, Destination, PhoneNumber};
use crate::email::EmailAddress;
use crate::profile::{Locale, TimeZoneName};

/// A further channel change is refused for this long after the last one (policy §7).
pub const CHANNEL_CHANGE_FREEZE: Duration = Duration::hours(24);

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

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "active" => Some(Self::Active),
            "closed" => Some(Self::Closed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub email: EmailAddress,
    pub email_verified_at: DateTime<Utc>,
    /// Confirmed WhatsApp number (S3b); `None` until added by a channel change.
    pub whatsapp: Option<PhoneNumber>,
    pub locale: Locale,
    pub time_zone: TimeZoneName,
    pub is_platform_admin: bool,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub channel_changed_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }

    /// The confirmed value of a channel, if the user has one.
    pub fn destination(&self, channel: Channel) -> Option<Destination> {
        match channel {
            Channel::Email => Some(Destination::Email(self.email.clone())),
            Channel::Whatsapp => self.whatsapp.clone().map(Destination::Whatsapp),
        }
    }

    /// Whether `destination` is still a current confirmed value of the user (codes sent to a
    /// value that was replaced meanwhile are dead; policy §2.2).
    pub fn has_destination(&self, destination: &Destination) -> bool {
        self.destination(destination.channel())
            .is_some_and(|d| d.normalized() == destination.normalized())
    }

    /// When a new channel change is accepted; in the past when allowed now.
    pub fn channel_change_allowed_at(&self) -> DateTime<Utc> {
        self.channel_changed_at
            .map_or(self.created_at, |at| at + CHANNEL_CHANGE_FREEZE)
    }
}
