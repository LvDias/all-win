//! Per-IP budget of failed device-token authentications on the public `/adapter/v1` upgrade
//! (Crivo g1): the same rule as the ingress token path (`ingress.ip_token_failures_per_min`),
//! failures only. Over budget, the token is not even checked and the refusal is identical to a
//! wrong token (close 4001, no detail). In memory: a restart resets it.
//!
//! IPv4 by address, IPv6 by /64. Memory stays bounded: beyond [`MAX_TRACKED`] keys every new
//! key shares one overflow counter.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use axum::extract::ConnectInfo;
use axum::http::{Extensions, HeaderMap};

const WINDOW: Duration = Duration::from_secs(60);
pub const MAX_TRACKED: usize = 100_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum IpKey {
    V4(u32),
    /// The /64 prefix.
    V6(u64),
    Overflow,
}

impl From<IpAddr> for IpKey {
    fn from(ip: IpAddr) -> Self {
        match ip {
            IpAddr::V4(v4) => Self::V4(u32::from(v4)),
            IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
                Some(v4) => Self::V4(u32::from(v4)),
                None => Self::V6((u128::from(v6) >> 64) as u64),
            },
        }
    }
}

/// Failures per IP in fixed one-minute windows.
pub struct FailureBudget {
    per_minute: u32,
    counts: Mutex<HashMap<IpKey, (Instant, u32)>>,
}

impl FailureBudget {
    pub fn new(per_minute: u32) -> Self {
        Self {
            per_minute: per_minute.max(1),
            counts: Mutex::new(HashMap::new()),
        }
    }

    fn key(map: &HashMap<IpKey, (Instant, u32)>, ip: IpAddr) -> IpKey {
        let key = IpKey::from(ip);
        if map.len() >= MAX_TRACKED && !map.contains_key(&key) {
            IpKey::Overflow
        } else {
            key
        }
    }

    /// Whether `ip` has spent its failures for the current window.
    pub fn exhausted(&self, ip: IpAddr, now: Instant) -> bool {
        let map = self
            .counts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let key = Self::key(&map, ip);
        map.get(&key).is_some_and(|(start, n)| {
            now.saturating_duration_since(*start) < WINDOW && *n >= self.per_minute
        })
    }

    /// Charges one failed authentication to `ip`.
    pub fn record(&self, ip: IpAddr, now: Instant) {
        let mut map = self
            .counts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if map.len() >= MAX_TRACKED {
            map.retain(|_, (start, _)| now.saturating_duration_since(*start) < WINDOW);
        }
        let key = Self::key(&map, ip);
        let entry = map.entry(key).or_insert((now, 0));
        if now.saturating_duration_since(entry.0) >= WINDOW {
            *entry = (now, 0);
        }
        entry.1 = entry.1.saturating_add(1);
    }
}

/// Client address: the last `X-Forwarded-For` hop when the proxy is trusted, otherwise the TCP
/// peer (same rule as the ingress).
pub fn client_ip(
    headers: &HeaderMap,
    extensions: &Extensions,
    trust_forwarded_for: bool,
) -> Option<IpAddr> {
    if trust_forwarded_for
        && let Some(ip) = headers
            .get_all("x-forwarded-for")
            .iter()
            .filter_map(|v| v.to_str().ok())
            .flat_map(|v| v.split(','))
            .next_back()
            .and_then(|v| v.trim().parse().ok())
    {
        return Some(ip);
    }
    extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|c| c.0.ip())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn failures_are_counted_per_ip_and_window() {
        let b = FailureBudget::new(2);
        let t = Instant::now();
        let a = ip("203.0.113.7");
        assert!(!b.exhausted(a, t));
        b.record(a, t);
        b.record(a, t);
        assert!(b.exhausted(a, t));
        assert!(!b.exhausted(ip("203.0.113.8"), t), "another address");
        assert!(!b.exhausted(a, t + WINDOW), "a new window");
    }

    #[test]
    fn ipv6_shares_the_64_prefix() {
        let b = FailureBudget::new(1);
        let t = Instant::now();
        b.record(ip("2001:db8:1:2::1"), t);
        assert!(b.exhausted(ip("2001:db8:1:2::ffff"), t));
        assert!(!b.exhausted(ip("2001:db8:1:3::1"), t));
    }

    #[test]
    fn forwarded_for_only_when_trusted() {
        let mut h = HeaderMap::new();
        h.insert(
            "x-forwarded-for",
            "198.51.100.1, 203.0.113.9".parse().unwrap(),
        );
        let mut e = Extensions::new();
        e.insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 9))));
        assert_eq!(client_ip(&h, &e, true), Some(ip("203.0.113.9")));
        assert_eq!(client_ip(&h, &e, false), Some(ip("127.0.0.1")));
    }
}
