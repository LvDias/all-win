//! Server-side sessions (auth policy rev. 2 §5.1, Q21/Q29 option (a) "trading-day"). A session
//! is started by a confirmed one-time code and bound to the browser's device (`known_device`).

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;

use crate::channel::Channel;
use crate::ids::{DeviceId, SessionId};
use crate::tokens::TokenHash;

/// Q21 (a): idle 12 h / absolute 24 h, or idle 7 d / absolute 30 d with "keep me signed in".
pub fn idle_timeout(persistent: bool) -> Duration {
    if persistent {
        Duration::days(7)
    } else {
        Duration::hours(12)
    }
}

pub fn absolute_timeout(persistent: bool) -> Duration {
    if persistent {
        Duration::days(30)
    } else {
        Duration::hours(24)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndReason {
    Logout,
    Expired,
    /// Replaced by a new session id in the same tab (channel change).
    Rotated,
    /// Another session completed a channel change, or a channel-change hold applied.
    ChannelChanged,
    /// "Forget this device".
    DeviceRevoked,
    /// "Sair de todos os dispositivos".
    SignedOutEverywhere,
}

impl EndReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Logout => "logout",
            Self::Expired => "expired",
            Self::Rotated => "rotated",
            Self::ChannelChanged => "channel_changed",
            Self::DeviceRevoked => "device_revoked",
            Self::SignedOutEverywhere => "signed_out_everywhere",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "logout" => Self::Logout,
            "expired" => Self::Expired,
            "rotated" => Self::Rotated,
            "channel_changed" => Self::ChannelChanged,
            "device_revoked" => Self::DeviceRevoked,
            "signed_out_everywhere" => Self::SignedOutEverywhere,
            _ => return None,
        })
    }

    /// Rotation replaces the session in the same tab, so open tabs are not told it ended.
    pub fn notifies_client(self) -> bool {
        !matches!(self, Self::Rotated)
    }
}

/// What a request may do with a session at a given instant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionStatus {
    Active,
    Expired,
    Ended,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub device_id: DeviceId,
    pub token_hash: TokenHash,
    pub persistent: bool,
    /// Channel of the code that signed in.
    pub login_channel: Channel,
    /// Code sign-in; the absolute timeout counts from here and survives rotation.
    pub login_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub idle_expires_at: DateTime<Utc>,
    pub absolute_expires_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub end_reason: Option<EndReason>,
    pub client: ClientInfo,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClientInfo {
    pub ip: Option<std::net::IpAddr>,
    pub user_agent: Option<String>,
}

pub struct NewSession {
    pub id: SessionId,
    pub user_id: UserId,
    pub device_id: DeviceId,
    pub token_hash: TokenHash,
    pub persistent: bool,
    pub login_channel: Channel,
    pub client: ClientInfo,
}

impl Session {
    /// Session after a confirmed sign-in or sign-up code.
    pub fn start(new: NewSession, now: DateTime<Utc>) -> Self {
        let absolute = now + absolute_timeout(new.persistent);
        let idle = (now + idle_timeout(new.persistent)).min(absolute);
        Self::build(new, now, now, idle, absolute)
    }

    /// Session that replaces `self` after a channel change (new id and cookie, same device, same
    /// login time, so the absolute timeout is never extended by rotation).
    pub fn rotate(&self, id: SessionId, token_hash: TokenHash, now: DateTime<Utc>) -> Self {
        let new = NewSession {
            id,
            user_id: self.user_id,
            device_id: self.device_id,
            token_hash,
            persistent: self.persistent,
            login_channel: self.login_channel,
            client: self.client.clone(),
        };
        let absolute = self.login_at + absolute_timeout(new.persistent);
        let idle = (now + idle_timeout(new.persistent)).min(absolute);
        Self::build(new, self.login_at, now, idle, absolute)
    }

    fn build(
        new: NewSession,
        login_at: DateTime<Utc>,
        now: DateTime<Utc>,
        idle_expires_at: DateTime<Utc>,
        absolute_expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: new.id,
            user_id: new.user_id,
            device_id: new.device_id,
            token_hash: new.token_hash,
            persistent: new.persistent,
            login_channel: new.login_channel,
            login_at,
            last_activity_at: now,
            idle_expires_at,
            absolute_expires_at,
            ended_at: None,
            end_reason: None,
            client: new.client,
        }
    }

    pub fn status(&self, now: DateTime<Utc>) -> SessionStatus {
        if self.ended_at.is_some() {
            SessionStatus::Ended
        } else if now >= self.idle_expires_at || now >= self.absolute_expires_at {
            SessionStatus::Expired
        } else {
            SessionStatus::Active
        }
    }

    /// Sliding idle timeout, refreshed only by user-initiated requests (SSE and polling never call
    /// this). Never moves past the absolute timeout.
    pub fn touch(&mut self, now: DateTime<Utc>) {
        self.last_activity_at = now;
        self.idle_expires_at = (now + idle_timeout(self.persistent)).min(self.absolute_expires_at);
    }

    pub fn end(&mut self, reason: EndReason, now: DateTime<Utc>) {
        if self.ended_at.is_none() {
            self.ended_at = Some(now);
            self.end_reason = Some(reason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn t0() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 23, 8, 0, 0).unwrap()
    }

    fn new(persistent: bool) -> NewSession {
        NewSession {
            id: SessionId::new_v7(),
            user_id: UserId::new_v7(),
            device_id: DeviceId::new_v7(),
            token_hash: TokenHash::from_bytes([7; 32]),
            persistent,
            login_channel: Channel::Email,
            client: ClientInfo::default(),
        }
    }

    #[test]
    fn trading_day_timeouts() {
        let s = Session::start(new(false), t0());
        assert_eq!(s.idle_expires_at, t0() + Duration::hours(12));
        assert_eq!(s.absolute_expires_at, t0() + Duration::hours(24));
        let p = Session::start(new(true), t0());
        assert_eq!(p.idle_expires_at, t0() + Duration::days(7));
        assert_eq!(p.absolute_expires_at, t0() + Duration::days(30));
    }

    #[test]
    fn idle_expiry_and_sliding_refresh() {
        let mut s = Session::start(new(false), t0());
        assert_eq!(s.status(t0() + Duration::hours(11)), SessionStatus::Active);
        s.touch(t0() + Duration::hours(11));
        assert_eq!(s.status(t0() + Duration::hours(22)), SessionStatus::Active);
        assert_eq!(
            s.status(t0() + Duration::hours(23) + Duration::minutes(1)),
            SessionStatus::Expired
        );
        let idle = Session::start(new(false), t0());
        assert_eq!(
            idle.status(t0() + Duration::hours(12)),
            SessionStatus::Expired
        );
    }

    #[test]
    fn absolute_expiry_is_never_extended_by_touch_or_rotation() {
        let mut s = Session::start(new(false), t0());
        for h in 1..24 {
            s.touch(t0() + Duration::hours(h));
        }
        assert!(s.idle_expires_at <= s.absolute_expires_at);
        assert_eq!(s.status(t0() + Duration::hours(24)), SessionStatus::Expired);
        let rotated = s.rotate(
            SessionId::new_v7(),
            TokenHash::from_bytes([8; 32]),
            t0() + Duration::hours(20),
        );
        assert_ne!(rotated.id, s.id);
        assert_eq!(rotated.device_id, s.device_id);
        assert_eq!(rotated.login_at, t0());
        assert_eq!(rotated.absolute_expires_at, t0() + Duration::hours(24));
    }

    #[test]
    fn ended_is_final() {
        let mut s = Session::start(new(false), t0());
        s.end(EndReason::Logout, t0());
        s.end(EndReason::Expired, t0() + Duration::hours(1));
        assert_eq!(s.end_reason, Some(EndReason::Logout));
        assert_eq!(s.status(t0()), SessionStatus::Ended);
        assert!(!EndReason::Rotated.notifies_client());
        for r in [
            EndReason::Logout,
            EndReason::Expired,
            EndReason::Rotated,
            EndReason::ChannelChanged,
            EndReason::DeviceRevoked,
            EndReason::SignedOutEverywhere,
        ] {
            assert_eq!(EndReason::parse(r.as_str()), Some(r));
        }
    }
}
