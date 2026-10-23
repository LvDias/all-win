//! Abuse throttling behind swappable ports. The policies live in `identity-infra`:
//! - Login: the accepted password policy (auth policy §2). Paused: the login method is being
//!   redefined (passwordless), so U10 is not implemented.
//! - U11 (decisions 2026-10-23) — no global pairing lock: per-IP and per-/24 or /64 prefix
//!   limits; the global failure threshold of ADR-0004 §8 only raises an alert.

use std::net::IpAddr;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared_kernel::UserId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrottleDecision {
    Allow,
    Limited { retry_after: Duration },
}

/// One password/second-factor attempt. Login, the TOTP and recovery-code verify endpoints,
/// step-up and the credential-changing endpoints share this throttle (auth policy §2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoginAttempt {
    /// Normalized e-mail as submitted (or the session user's e-mail).
    pub email_normalized: String,
    pub ip: IpAddr,
    /// Resolved account, if the e-mail belongs to one.
    pub user: Option<UserId>,
    /// The account completed a sign-in from this IP in the last 30 days.
    pub ip_is_usual: bool,
}

#[async_trait]
pub trait LoginThrottle: Send + Sync {
    async fn check(&self, attempt: &LoginAttempt, now: DateTime<Utc>) -> ThrottleDecision;
    async fn record_failure(&self, attempt: &LoginAttempt, now: DateTime<Utc>);
    async fn record_success(&self, attempt: &LoginAttempt, now: DateTime<Utc>);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairingDecision {
    Allow,
    /// `pairing.rate_limited`.
    RateLimited {
        retry_after: Duration,
    },
}

#[async_trait]
pub trait PairingThrottle: Send + Sync {
    /// Counts the attempt and decides whether it may proceed.
    async fn admit(&self, ip: IpAddr, now: DateTime<Utc>) -> PairingDecision;
    /// A rejected code (feeds the global failure alert).
    async fn record_failure(&self, ip: IpAddr, now: DateTime<Utc>);
}

/// Public endpoints that take a one-time token or trigger an e-mail (invitation lookup/accept,
/// password reset request/confirm).
#[async_trait]
pub trait PublicEndpointThrottle: Send + Sync {
    async fn admit(&self, ip: IpAddr, now: DateTime<Utc>) -> ThrottleDecision;
}
