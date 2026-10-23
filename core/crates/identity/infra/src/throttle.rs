//! In-memory throttles (single Core instance, data model §4.1; a restart resets them).
//!
//! - [`PolicyLoginThrottle`]: the accepted password login policy (auth policy §2); paused while
//!   the login method is redefined.
//! - [`PairingThrottlePolicy`] / [`PrefixPairingThrottle`]: U11 — attempts per IP and per /24
//!   (IPv4) or /64 (IPv6) prefix; the global failure threshold raises an alert and never
//!   blocks pairing.
//! - [`PerIpPublicThrottle`]: public token/e-mail endpoints, per IP.

use std::collections::{HashMap, VecDeque};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use identity_application::throttle::{
    LoginAttempt, LoginThrottle, PairingDecision, PairingThrottle, PublicEndpointThrottle,
    ThrottleDecision,
};
use shared_kernel::UserId;

/// Sliding-window counter per key.
struct Window<K> {
    hits: HashMap<K, VecDeque<DateTime<Utc>>>,
}

impl<K> Default for Window<K> {
    fn default() -> Self {
        Self {
            hits: HashMap::new(),
        }
    }
}

impl<K: std::hash::Hash + Eq + Clone> Window<K> {
    fn prune(&mut self, key: &K, now: DateTime<Utc>, span: chrono::Duration) -> usize {
        let Some(q) = self.hits.get_mut(key) else {
            return 0;
        };
        while q.front().is_some_and(|t| *t <= now - span) {
            q.pop_front();
        }
        let n = q.len();
        if n == 0 {
            self.hits.remove(key);
        }
        n
    }

    /// Seconds until the oldest hit leaves the window (at least 1).
    fn retry_after(&self, key: &K, now: DateTime<Utc>, span: chrono::Duration) -> Duration {
        let oldest = self
            .hits
            .get(key)
            .and_then(|q| q.front().copied())
            .unwrap_or(now);
        secs((oldest + span - now).num_seconds())
    }

    fn push(&mut self, key: K, now: DateTime<Utc>) {
        self.hits.entry(key).or_default().push_back(now);
    }
}

fn secs(s: i64) -> Duration {
    Duration::from_secs(u64::try_from(s.max(1)).unwrap_or(1))
}

#[derive(Clone, Copy, Debug)]
pub struct LoginPolicy {
    pub per_ip_per_minute: usize,
    /// Consecutive failures per (account, IP) before the progressive delay starts.
    pub delay_after_failures: u32,
    pub max_delay: chrono::Duration,
    pub account_failures_per_hour: usize,
    pub account_lock: chrono::Duration,
}

impl Default for LoginPolicy {
    /// Auth policy §2: 10/min per IP; 5 consecutive failures per (account, IP) then 1 s, 2 s, 4 s…
    /// up to 60 s; 20 failures per hour per account lock it for unknown IPs for 15 minutes.
    fn default() -> Self {
        Self {
            per_ip_per_minute: 10,
            delay_after_failures: 5,
            max_delay: chrono::Duration::seconds(60),
            account_failures_per_hour: 20,
            account_lock: chrono::Duration::minutes(15),
        }
    }
}

#[derive(Default)]
struct LoginState {
    per_ip: Window<IpAddr>,
    /// (account, IP) → (consecutive failures, last failure).
    consecutive: HashMap<(UserId, IpAddr), (u32, DateTime<Utc>)>,
    account_failures: Window<UserId>,
    locked_until: HashMap<UserId, DateTime<Utc>>,
}

pub struct PolicyLoginThrottle {
    policy: LoginPolicy,
    state: Mutex<LoginState>,
}

impl PolicyLoginThrottle {
    pub fn new(policy: LoginPolicy) -> Self {
        Self {
            policy,
            state: Mutex::new(LoginState::default()),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, LoginState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn delay(&self, failures: u32) -> chrono::Duration {
        let over = failures.saturating_sub(self.policy.delay_after_failures);
        if failures < self.policy.delay_after_failures {
            return chrono::Duration::zero();
        }
        let secs = 1i64.checked_shl(over).unwrap_or(i64::MAX);
        chrono::Duration::seconds(secs).min(self.policy.max_delay)
    }
}

#[async_trait]
impl LoginThrottle for PolicyLoginThrottle {
    async fn check(&self, a: &LoginAttempt, now: DateTime<Utc>) -> ThrottleDecision {
        let minute = chrono::Duration::minutes(1);
        let mut st = self.lock();
        if st.per_ip.prune(&a.ip, now, minute) >= self.policy.per_ip_per_minute {
            return ThrottleDecision::Limited {
                retry_after: st.per_ip.retry_after(&a.ip, now, minute),
            };
        }
        st.per_ip.push(a.ip, now);
        let Some(user) = a.user else {
            return ThrottleDecision::Allow;
        };
        if !a.ip_is_usual
            && let Some(until) = st.locked_until.get(&user).copied()
        {
            if now < until {
                return ThrottleDecision::Limited {
                    retry_after: secs((until - now).num_seconds()),
                };
            }
            st.locked_until.remove(&user);
        }
        if let Some((failures, last)) = st.consecutive.get(&(user, a.ip)).copied() {
            let wait_until = last + self.delay(failures);
            if now < wait_until {
                return ThrottleDecision::Limited {
                    retry_after: secs((wait_until - now).num_seconds()),
                };
            }
        }
        ThrottleDecision::Allow
    }

    async fn record_failure(&self, a: &LoginAttempt, now: DateTime<Utc>) {
        let Some(user) = a.user else { return };
        let hour = chrono::Duration::hours(1);
        let mut st = self.lock();
        let entry = st.consecutive.entry((user, a.ip)).or_insert((0, now));
        *entry = (entry.0.saturating_add(1), now);
        st.account_failures.push(user, now);
        if st.account_failures.prune(&user, now, hour) >= self.policy.account_failures_per_hour {
            st.locked_until.insert(user, now + self.policy.account_lock);
        }
    }

    async fn record_success(&self, a: &LoginAttempt, _now: DateTime<Utc>) {
        if let Some(user) = a.user {
            self.lock().consecutive.remove(&(user, a.ip));
        }
    }
}

/// U11 (decisions 2026-10-23); ADR-0004 §8 without its global lock.
#[derive(Clone, Copy, Debug)]
pub struct PairingThrottlePolicy {
    pub per_ip_per_minute: usize,
    pub per_prefix_per_minute: usize,
    /// Failed attempts across all clients within [`Self::global_window`] that raise the alert.
    pub global_failures_alert: usize,
    pub global_window: chrono::Duration,
}

impl Default for PairingThrottlePolicy {
    /// 10/min per IP, 30/min per /24 or /64, alert at 100 failures in 10 minutes.
    fn default() -> Self {
        Self {
            per_ip_per_minute: 10,
            per_prefix_per_minute: 30,
            global_failures_alert: 100,
            global_window: chrono::Duration::minutes(10),
        }
    }
}

/// Receives the operational alert when failed pairings cross the global threshold.
pub trait PairingAlertSink: Send + Sync {
    fn global_failures_exceeded(&self, failures: usize, window: chrono::Duration);
}

/// Writes the alert to the technical log (no IPs or codes, only the count).
pub struct LogPairingAlerts;

impl PairingAlertSink for LogPairingAlerts {
    fn global_failures_exceeded(&self, failures: usize, window: chrono::Duration) {
        tracing::warn!(
            target: "security_alert",
            failures,
            window_seconds = window.num_seconds(),
            "pairing failures crossed the global alert threshold"
        );
    }
}

/// The /24 (IPv4) or /64 (IPv6) network an address belongs to; IPv4-mapped IPv6 counts as IPv4.
fn network_prefix(ip: IpAddr) -> IpAddr {
    match ip.to_canonical() {
        IpAddr::V4(v4) => IpAddr::V4(std::net::Ipv4Addr::from(u32::from(v4) & 0xFFFF_FF00)),
        IpAddr::V6(v6) => IpAddr::V6(std::net::Ipv6Addr::from(
            u128::from(v6) & !((1u128 << 64) - 1),
        )),
    }
}

#[derive(Default)]
struct PairingState {
    per_ip: Window<IpAddr>,
    per_prefix: Window<IpAddr>,
    failures: Window<()>,
    /// Set while the global count stays at or above the threshold, so the alert fires once
    /// per episode.
    alerting: bool,
}

pub struct PrefixPairingThrottle {
    policy: PairingThrottlePolicy,
    alerts: Arc<dyn PairingAlertSink>,
    state: Mutex<PairingState>,
}

impl PrefixPairingThrottle {
    pub fn new(policy: PairingThrottlePolicy, alerts: Arc<dyn PairingAlertSink>) -> Self {
        Self {
            policy,
            alerts,
            state: Mutex::new(PairingState::default()),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, PairingState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

#[async_trait]
impl PairingThrottle for PrefixPairingThrottle {
    async fn admit(&self, ip: IpAddr, now: DateTime<Utc>) -> PairingDecision {
        let minute = chrono::Duration::minutes(1);
        let ip = ip.to_canonical();
        let prefix = network_prefix(ip);
        let mut st = self.lock();
        let ip_full = st.per_ip.prune(&ip, now, minute) >= self.policy.per_ip_per_minute;
        let prefix_full =
            st.per_prefix.prune(&prefix, now, minute) >= self.policy.per_prefix_per_minute;
        if ip_full || prefix_full {
            let mut retry_after = Duration::ZERO;
            if ip_full {
                retry_after = retry_after.max(st.per_ip.retry_after(&ip, now, minute));
            }
            if prefix_full {
                retry_after = retry_after.max(st.per_prefix.retry_after(&prefix, now, minute));
            }
            return PairingDecision::RateLimited { retry_after };
        }
        st.per_ip.push(ip, now);
        st.per_prefix.push(prefix, now);
        PairingDecision::Allow
    }

    async fn record_failure(&self, _ip: IpAddr, now: DateTime<Utc>) {
        let window = self.policy.global_window;
        let crossed = {
            let mut st = self.lock();
            st.failures.push((), now);
            let failures = st.failures.prune(&(), now, window);
            let over = failures >= self.policy.global_failures_alert;
            let crossed = over && !st.alerting;
            st.alerting = over;
            crossed.then_some(failures)
        };
        if let Some(failures) = crossed {
            self.alerts.global_failures_exceeded(failures, window);
        }
    }
}

pub struct PerIpPublicThrottle {
    per_minute: usize,
    attempts: Mutex<Window<IpAddr>>,
}

impl PerIpPublicThrottle {
    pub fn new(per_minute: usize) -> Self {
        Self {
            per_minute,
            attempts: Mutex::new(Window::default()),
        }
    }
}

#[async_trait]
impl PublicEndpointThrottle for PerIpPublicThrottle {
    async fn admit(&self, ip: IpAddr, now: DateTime<Utc>) -> ThrottleDecision {
        let minute = chrono::Duration::minutes(1);
        let mut w = self
            .attempts
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if w.prune(&ip, now, minute) >= self.per_minute {
            return ThrottleDecision::Limited {
                retry_after: w.retry_after(&ip, now, minute),
            };
        }
        w.push(ip, now);
        ThrottleDecision::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::net::Ipv4Addr;

    fn t0() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 23, 9, 0, 0).unwrap()
    }

    fn attempt(user: Option<UserId>, ip: u8, usual: bool) -> LoginAttempt {
        LoginAttempt {
            email_normalized: "a@b.c".into(),
            ip: IpAddr::V4(Ipv4Addr::new(198, 51, 100, ip)),
            user,
            ip_is_usual: usual,
        }
    }

    fn limited(d: ThrottleDecision) -> bool {
        matches!(d, ThrottleDecision::Limited { .. })
    }

    #[tokio::test]
    async fn ten_attempts_per_minute_per_ip() {
        let t = PolicyLoginThrottle::new(LoginPolicy::default());
        let a = attempt(None, 1, false);
        for _ in 0..10 {
            assert!(!limited(t.check(&a, t0()).await));
        }
        assert!(
            matches!(t.check(&a, t0()).await, ThrottleDecision::Limited { retry_after } if retry_after == Duration::from_secs(60))
        );
        assert!(
            !limited(t.check(&attempt(None, 2, false), t0()).await),
            "other IP unaffected"
        );
        assert!(!limited(
            t.check(&a, t0() + chrono::Duration::seconds(61)).await
        ));
    }

    #[tokio::test]
    async fn progressive_delay_after_five_consecutive_failures() {
        let t = PolicyLoginThrottle::new(LoginPolicy {
            per_ip_per_minute: 1000,
            ..LoginPolicy::default()
        });
        let u = UserId::new_v7();
        let a = attempt(Some(u), 1, true);
        let mut now = t0();
        for _ in 0..4 {
            assert!(!limited(t.check(&a, now).await));
            t.record_failure(&a, now).await;
        }
        assert!(!limited(t.check(&a, now).await), "4 failures: no delay yet");
        t.record_failure(&a, now).await; // 5th
        assert!(
            matches!(t.check(&a, now).await, ThrottleDecision::Limited { retry_after } if retry_after == Duration::from_secs(1))
        );
        now += chrono::Duration::seconds(1);
        assert!(!limited(t.check(&a, now).await));
        t.record_failure(&a, now).await; // 6th → 2 s
        assert!(
            matches!(t.check(&a, now).await, ThrottleDecision::Limited { retry_after } if retry_after == Duration::from_secs(2))
        );
        for _ in 0..10 {
            t.record_failure(&a, now).await;
        }
        assert!(
            matches!(t.check(&a, now).await, ThrottleDecision::Limited { retry_after } if retry_after == Duration::from_secs(60)),
            "capped"
        );
        t.record_success(&a, now).await;
        assert!(
            !limited(t.check(&a, now).await),
            "success resets the (account, IP) streak"
        );
    }

    #[tokio::test]
    async fn account_soft_lock_spares_usual_ips() {
        let t = PolicyLoginThrottle::new(LoginPolicy {
            per_ip_per_minute: 1000,
            delay_after_failures: 1000,
            ..LoginPolicy::default()
        });
        let u = UserId::new_v7();
        for i in 0..20u8 {
            t.record_failure(&attempt(Some(u), i, false), t0()).await;
        }
        assert!(
            limited(t.check(&attempt(Some(u), 99, false), t0()).await),
            "unknown IP locked"
        );
        assert!(
            !limited(t.check(&attempt(Some(u), 99, true), t0()).await),
            "usual IP still allowed"
        );
        assert!(!limited(
            t.check(
                &attempt(Some(u), 99, false),
                t0() + chrono::Duration::minutes(15)
            )
            .await
        ));
    }

    #[derive(Default)]
    struct RecordingAlerts(Mutex<Vec<usize>>);

    impl PairingAlertSink for RecordingAlerts {
        fn global_failures_exceeded(&self, failures: usize, _window: chrono::Duration) {
            self.0.lock().unwrap().push(failures);
        }
    }

    fn pairing() -> (PrefixPairingThrottle, Arc<RecordingAlerts>) {
        let alerts = Arc::new(RecordingAlerts::default());
        (
            PrefixPairingThrottle::new(PairingThrottlePolicy::default(), alerts.clone()),
            alerts,
        )
    }

    fn v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(a, b, c, d))
    }

    #[test]
    fn prefixes_are_slash_24_and_slash_64() {
        assert_eq!(network_prefix(v4(192, 0, 2, 77)), v4(192, 0, 2, 0));
        let v6: IpAddr = "2001:db8:1:2:aaaa:bbbb:cccc:dddd".parse().unwrap();
        assert_eq!(
            network_prefix(v6),
            "2001:db8:1:2::".parse::<IpAddr>().unwrap()
        );
        let mapped: IpAddr = "::ffff:192.0.2.77".parse().unwrap();
        assert_eq!(network_prefix(mapped), v4(192, 0, 2, 0));
    }

    #[tokio::test]
    async fn pairing_limits_ten_per_minute_per_ip() {
        let (t, _) = pairing();
        for _ in 0..10 {
            assert_eq!(
                t.admit(v4(192, 0, 2, 1), t0()).await,
                PairingDecision::Allow
            );
        }
        assert!(matches!(
            t.admit(v4(192, 0, 2, 1), t0()).await,
            PairingDecision::RateLimited { retry_after } if retry_after == Duration::from_secs(60)
        ));
        assert_eq!(
            t.admit(v4(192, 0, 2, 2), t0()).await,
            PairingDecision::Allow,
            "other IP in the prefix still has room"
        );
        let mapped: IpAddr = "::ffff:192.0.2.1".parse().unwrap();
        assert!(
            matches!(
                t.admit(mapped, t0()).await,
                PairingDecision::RateLimited { .. }
            ),
            "IPv4-mapped IPv6 is the same client"
        );
        assert_eq!(
            t.admit(v4(192, 0, 2, 1), t0() + chrono::Duration::seconds(61))
                .await,
            PairingDecision::Allow
        );
    }

    #[tokio::test]
    async fn pairing_limits_thirty_per_minute_per_prefix() {
        let (t, _) = pairing();
        for n in 1..=30u8 {
            assert_eq!(
                t.admit(v4(192, 0, 2, n), t0()).await,
                PairingDecision::Allow
            );
        }
        assert!(
            matches!(
                t.admit(v4(192, 0, 2, 99), t0()).await,
                PairingDecision::RateLimited { .. }
            ),
            "fresh IP in a full /24"
        );
        assert_eq!(
            t.admit(v4(192, 0, 3, 1), t0()).await,
            PairingDecision::Allow,
            "neighbouring /24 unaffected"
        );

        let v6 = |host: u16| -> IpAddr { format!("2001:db8:0:7::{host:x}").parse().unwrap() };
        for h in 1..=30u16 {
            assert_eq!(t.admit(v6(h), t0()).await, PairingDecision::Allow);
        }
        assert!(matches!(
            t.admit(v6(999), t0()).await,
            PairingDecision::RateLimited { .. }
        ));
        let other: IpAddr = "2001:db8:0:8::1".parse().unwrap();
        assert_eq!(t.admit(other, t0()).await, PairingDecision::Allow);
    }

    #[tokio::test]
    async fn global_failures_alert_once_and_never_lock() {
        let (t, alerts) = pairing();
        for i in 0..150u32 {
            let ip = v4(10, (i / 250) as u8, (i % 250) as u8, 1);
            t.record_failure(ip, t0()).await;
        }
        assert_eq!(
            *alerts.0.lock().unwrap(),
            vec![100],
            "one alert per episode"
        );
        assert_eq!(
            t.admit(v4(203, 0, 113, 5), t0()).await,
            PairingDecision::Allow,
            "the alert does not block pairing"
        );

        // After the window drains the alert re-arms.
        let later = t0() + chrono::Duration::minutes(11);
        for _ in 0..100 {
            t.record_failure(v4(10, 9, 9, 9), later).await;
        }
        assert_eq!(*alerts.0.lock().unwrap(), vec![100, 100]);
    }
}
