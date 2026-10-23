//! Abuse throttling behind swappable ports. The policies live in `identity-infra`:
//! - U11 (decisions 2026-10-23) — no global pairing lock: per-IP and per-/24 or /64 prefix
//!   limits; the global failure threshold of ADR-0004 §8 only raises an alert.

use std::net::IpAddr;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrottleDecision {
    Allow,
    Limited { retry_after: Duration },
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

/// Public endpoints that take a one-time token (invitation lookup).
#[async_trait]
pub trait PublicEndpointThrottle: Send + Sync {
    async fn admit(&self, ip: IpAddr, now: DateTime<Utc>) -> ThrottleDecision;
}
