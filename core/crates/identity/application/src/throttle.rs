//! Abuse throttling behind swappable ports; the in-memory policies live in `identity-infra`.
//! - Code sends and code checks per IP and per IP prefix (auth policy rev. 2 §4); the durable
//!   per-identifier and per-user limits are counted in the database (see `usecases::limits`).
//! - U11 (decisions 2026-10-23): no global pairing lock; per-IP and per-/24 or /64 prefix limits;
//!   the global failure threshold of ADR-0004 §8 only raises an alert.

use std::net::IpAddr;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrottleDecision {
    Allow,
    Limited { retry_after: Duration },
}

/// In-memory limits of the code endpoints (policy §4): sends per IP and per prefix, the global
/// send alert, and wrong attempts per IP across codes.
#[async_trait]
pub trait CodeThrottle: Send + Sync {
    /// Counts one send from `ip` and decides whether it may happen.
    async fn admit_send(&self, ip: IpAddr, now: DateTime<Utc>) -> ThrottleDecision;
    /// Whether `ip` may still try a code (wrong attempts per IP).
    async fn admit_verify(&self, ip: IpAddr, now: DateTime<Utc>) -> ThrottleDecision;
    /// A wrong code from `ip`.
    async fn record_wrong(&self, ip: IpAddr, now: DateTime<Utc>);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairingDecision {
    Allow,
    /// `pairing.rate_limited`.
    RateLimited { retry_after: Duration },
}

#[async_trait]
pub trait PairingThrottle: Send + Sync {
    /// Counts the attempt and decides whether it may proceed.
    async fn admit(&self, ip: IpAddr, now: DateTime<Utc>) -> PairingDecision;
    /// A rejected code (feeds the global failure alert).
    async fn record_failure(&self, ip: IpAddr, now: DateTime<Utc>);
}

/// Public endpoints that take a one-time token (invitation lookup, sign-up start).
#[async_trait]
pub trait PublicEndpointThrottle: Send + Sync {
    async fn admit(&self, ip: IpAddr, now: DateTime<Utc>) -> ThrottleDecision;
}
