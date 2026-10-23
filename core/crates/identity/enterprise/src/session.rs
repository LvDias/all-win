//! Server-side sessions (auth policy §1, Q21 option (a) "trading-day").

use chrono::{DateTime, Duration, Utc};
use shared_kernel::UserId;

use crate::ids::SessionId;
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

/// Password accepted, TOTP not yet verified.
pub const PENDING_2FA_LIFETIME: Duration = Duration::minutes(5);
/// Sensitive actions need a password(+TOTP) proof younger than this.
pub const STEP_UP_WINDOW: Duration = Duration::minutes(15);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfaState {
    NotRequired,
    Pending,
    Verified,
}

impl MfaState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Pending => "pending",
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndReason {
    Logout,
    Expired,
    PasswordReset,
    PasswordChanged,
    TotpChanged,
    AdminTotpReset,
    Rotated,
}

impl EndReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Logout => "logout",
            Self::Expired => "expired",
            Self::PasswordReset => "password_reset",
            Self::PasswordChanged => "password_changed",
            Self::TotpChanged => "totp_changed",
            Self::AdminTotpReset => "admin_totp_reset",
            Self::Rotated => "rotated",
        }
    }

    /// Rotation replaces the session in the same tab, so open tabs are not told it ended.
    pub fn notifies_client(self) -> bool {
        !matches!(self, Self::Rotated)
    }
}

/// What a request may do with a session at a given instant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionStatus {
    /// Full session (`not_required` or `verified`).
    Active,
    /// Pending second factor, still within its 5 minutes.
    Pending2fa,
    PendingExpired,
    Expired,
    Ended,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub token_hash: TokenHash,
    pub persistent: bool,
    pub mfa_state: MfaState,
    /// When the password was accepted at sign-in; the absolute timeout counts from here.
    pub login_at: DateTime<Utc>,
    /// Last password(+TOTP) proof; drives the step-up rule.
    pub authenticated_at: DateTime<Utc>,
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
    pub token_hash: TokenHash,
    pub persistent: bool,
    pub client: ClientInfo,
}

impl Session {
    /// Session after a password proof. With TOTP enabled it is pending for 5 minutes and grants
    /// nothing but the verify endpoints.
    pub fn start(new: NewSession, totp_enabled: bool, now: DateTime<Utc>) -> Self {
        if totp_enabled {
            let expires = now + PENDING_2FA_LIFETIME;
            return Self::build(new, MfaState::Pending, now, now, expires, expires);
        }
        let absolute = now + absolute_timeout(new.persistent);
        let idle = (now + idle_timeout(new.persistent)).min(absolute);
        Self::build(new, MfaState::NotRequired, now, now, idle, absolute)
    }

    /// Full session that replaces `self` after a second factor or a password change (new id,
    /// same login time, so the absolute timeout is never extended by rotation).
    pub fn rotate(&self, new: NewSession, mfa_state: MfaState, now: DateTime<Utc>) -> Self {
        debug_assert_eq!(new.user_id, self.user_id);
        let absolute = self.login_at + absolute_timeout(new.persistent);
        let idle = (now + idle_timeout(new.persistent)).min(absolute);
        Self::build(new, mfa_state, self.login_at, now, idle, absolute)
    }

    fn build(
        new: NewSession,
        mfa_state: MfaState,
        login_at: DateTime<Utc>,
        now: DateTime<Utc>,
        idle_expires_at: DateTime<Utc>,
        absolute_expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: new.id,
            user_id: new.user_id,
            token_hash: new.token_hash,
            persistent: new.persistent,
            mfa_state,
            login_at,
            authenticated_at: now,
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
            return SessionStatus::Ended;
        }
        let expired = now >= self.idle_expires_at || now >= self.absolute_expires_at;
        match (self.mfa_state, expired) {
            (MfaState::Pending, true) => SessionStatus::PendingExpired,
            (MfaState::Pending, false) => SessionStatus::Pending2fa,
            (_, true) => SessionStatus::Expired,
            (_, false) => SessionStatus::Active,
        }
    }

    /// Sliding idle timeout, refreshed only by user-initiated requests (SSE and polling never call
    /// this). Never moves past the absolute timeout.
    pub fn touch(&mut self, now: DateTime<Utc>) {
        if self.mfa_state == MfaState::Pending {
            return;
        }
        self.last_activity_at = now;
        self.idle_expires_at = (now + idle_timeout(self.persistent)).min(self.absolute_expires_at);
    }

    pub fn step_up_valid_until(&self) -> DateTime<Utc> {
        self.authenticated_at + STEP_UP_WINDOW
    }

    pub fn requires_step_up(&self, now: DateTime<Utc>) -> bool {
        now >= self.step_up_valid_until()
    }

    /// A successful step-up also counts as user activity.
    pub fn reauthenticated(&mut self, now: DateTime<Utc>) {
        self.authenticated_at = now;
        self.touch(now);
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
            token_hash: TokenHash::from_bytes([7; 32]),
            persistent,
            client: ClientInfo::default(),
        }
    }

    #[test]
    fn trading_day_timeouts() {
        let s = Session::start(new(false), false, t0());
        assert_eq!(s.idle_expires_at, t0() + Duration::hours(12));
        assert_eq!(s.absolute_expires_at, t0() + Duration::hours(24));
        let p = Session::start(new(true), false, t0());
        assert_eq!(p.idle_expires_at, t0() + Duration::days(7));
        assert_eq!(p.absolute_expires_at, t0() + Duration::days(30));
    }

    #[test]
    fn idle_expiry_and_sliding_refresh() {
        let mut s = Session::start(new(false), false, t0());
        assert_eq!(s.status(t0() + Duration::hours(11)), SessionStatus::Active);
        s.touch(t0() + Duration::hours(11));
        assert_eq!(s.status(t0() + Duration::hours(22)), SessionStatus::Active);
        assert_eq!(
            s.status(t0() + Duration::hours(23) + Duration::minutes(1)),
            SessionStatus::Expired
        );
        let idle = Session::start(new(false), false, t0());
        assert_eq!(
            idle.status(t0() + Duration::hours(12)),
            SessionStatus::Expired
        );
    }

    #[test]
    fn absolute_expiry_is_never_extended() {
        let mut s = Session::start(new(false), false, t0());
        for h in 1..24 {
            s.touch(t0() + Duration::hours(h));
        }
        assert!(s.idle_expires_at <= s.absolute_expires_at);
        assert_eq!(s.status(t0() + Duration::hours(24)), SessionStatus::Expired);
        let rotated = s.rotate(
            NewSession {
                user_id: s.user_id,
                ..new(false)
            },
            MfaState::NotRequired,
            t0() + Duration::hours(20),
        );
        assert_eq!(rotated.absolute_expires_at, t0() + Duration::hours(24));
    }

    #[test]
    fn pending_2fa_lasts_five_minutes_and_is_not_refreshed() {
        let mut s = Session::start(new(true), true, t0());
        assert_eq!(s.mfa_state, MfaState::Pending);
        s.touch(t0() + Duration::minutes(4));
        assert_eq!(
            s.status(t0() + Duration::minutes(4)),
            SessionStatus::Pending2fa
        );
        assert_eq!(
            s.status(t0() + Duration::minutes(5)),
            SessionStatus::PendingExpired
        );
    }

    #[test]
    fn rotation_after_totp_gives_full_timeouts_from_login() {
        let pending = Session::start(new(true), true, t0());
        let full = pending.rotate(
            NewSession {
                user_id: pending.user_id,
                ..new(true)
            },
            MfaState::Verified,
            t0() + Duration::minutes(2),
        );
        assert_ne!(full.id, pending.id);
        assert_eq!(full.status(t0() + Duration::days(6)), SessionStatus::Active);
        assert_eq!(full.absolute_expires_at, t0() + Duration::days(30));
        assert_eq!(full.authenticated_at, t0() + Duration::minutes(2));
    }

    #[test]
    fn step_up_window_is_fifteen_minutes() {
        let mut s = Session::start(new(false), false, t0());
        assert!(!s.requires_step_up(t0() + Duration::minutes(14)));
        assert!(s.requires_step_up(t0() + Duration::minutes(15)));
        s.reauthenticated(t0() + Duration::minutes(30));
        assert!(!s.requires_step_up(t0() + Duration::minutes(44)));
        assert_eq!(s.step_up_valid_until(), t0() + Duration::minutes(45));
    }

    #[test]
    fn ended_is_final() {
        let mut s = Session::start(new(false), false, t0());
        s.end(EndReason::Logout, t0());
        s.end(EndReason::Expired, t0() + Duration::hours(1));
        assert_eq!(s.end_reason, Some(EndReason::Logout));
        assert_eq!(s.status(t0()), SessionStatus::Ended);
        assert!(!EndReason::Rotated.notifies_client());
    }
}
