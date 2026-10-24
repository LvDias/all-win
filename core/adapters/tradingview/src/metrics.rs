//! Operational counters and the server-side latency histogram by result, never by key,
//! user or token (s1-webhook-ingest §2 steps 1–2; histogram approved by the Architect, C1/C2:
//! labels `adapter` and `result` only, exposed on the internal `/metrics`), and how long the
//! request waited for a database connection (Pulso).

use std::fmt::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Accepted,
    Duplicate,
    NotFound,
    RateLimitedIp,
    RateLimitedFailures,
    RateLimitedKey,
    Rejected,
    Internal,
}

impl Outcome {
    const ALL: [Self; 8] = [
        Self::Accepted,
        Self::Duplicate,
        Self::NotFound,
        Self::RateLimitedIp,
        Self::RateLimitedFailures,
        Self::RateLimitedKey,
        Self::Rejected,
        Self::Internal,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Duplicate => "duplicate",
            Self::NotFound => "not_found",
            Self::RateLimitedIp => "rate_limited_ip",
            Self::RateLimitedFailures => "rate_limited_token_failures",
            Self::RateLimitedKey => "rate_limited_key",
            Self::Rejected => "rejected",
            Self::Internal => "internal",
        }
    }
}

/// Histogram upper bounds in seconds (Prometheus `le`), plus `+Inf` (Pulso: the gate is
/// p95 < 300 ms; TradingView gives up after about 3 s).
const BUCKETS: [f64; 11] = [0.005, 0.01, 0.025, 0.05, 0.1, 0.2, 0.3, 0.5, 1.0, 3.0, 5.0];

/// One latency distribution: cumulative counts are derived at render time.
#[derive(Debug, Default)]
struct Latency {
    /// Per bucket (not cumulative); the last slot is above the largest bound.
    buckets: [AtomicU64; 12],
    sum_micros: AtomicU64,
    count: AtomicU64,
}

impl Latency {
    fn observe(&self, elapsed: Duration) {
        let secs = elapsed.as_secs_f64();
        let slot = BUCKETS
            .iter()
            .position(|le| secs <= *le)
            .unwrap_or(BUCKETS.len());
        self.buckets[slot].fetch_add(1, Ordering::Relaxed);
        let micros = u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX);
        self.sum_micros.fetch_add(micros, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    /// The `_bucket`, `_sum` and `_count` lines of `name` with these labels.
    fn render(&self, name: &str, labels: &str, out: &mut String) {
        let mut cumulative = 0;
        for (n, slot) in self.buckets.iter().enumerate() {
            cumulative += slot.load(Ordering::Relaxed);
            let le = BUCKETS
                .get(n)
                .map_or_else(|| "+Inf".to_owned(), f64::to_string);
            let _ = writeln!(out, "{name}_bucket{{{labels},le=\"{le}\"}} {cumulative}");
        }
        // Seconds with microsecond precision, in integer arithmetic.
        let micros = self.sum_micros.load(Ordering::Relaxed);
        let _ = writeln!(
            out,
            "{name}_sum{{{labels}}} {}.{:06}",
            micros / 1_000_000,
            micros % 1_000_000
        );
        let _ = writeln!(
            out,
            "{name}_count{{{labels}}} {}",
            self.count.load(Ordering::Relaxed)
        );
    }
}

#[derive(Debug, Default)]
pub struct IngressMetrics {
    counts: [AtomicU64; 8],
    latency: [Latency; 8],
    db_acquire: Latency,
}

impl IngressMetrics {
    pub fn record(&self, outcome: Outcome) {
        if let Some(i) = Outcome::ALL.iter().position(|o| *o == outcome) {
            self.counts[i].fetch_add(1, Ordering::Relaxed);
        }
    }

    /// One ingress request, from the moment the route's layer saw it to the response built.
    pub fn observe(&self, outcome: Outcome, elapsed: Duration) {
        if let Some(i) = Outcome::ALL.iter().position(|o| *o == outcome) {
            self.latency[i].observe(elapsed);
        }
    }

    /// The wait for a connection from the application pool at the request's first database
    /// call (source-key lookup), whether or not a connection came: a probe of pool contention.
    pub fn observe_db_acquire(&self, elapsed: Duration) {
        self.db_acquire.observe(elapsed);
    }

    pub fn get(&self, outcome: Outcome) -> u64 {
        Outcome::ALL
            .iter()
            .position(|o| *o == outcome)
            .map_or(0, |i| self.counts[i].load(Ordering::Relaxed))
    }

    /// Prometheus text exposition.
    pub fn render(&self, out: &mut String) {
        out.push_str("# TYPE allwin_ingress_requests_total counter\n");
        for outcome in Outcome::ALL {
            let _ = writeln!(
                out,
                "allwin_ingress_requests_total{{adapter=\"tradingview\",result=\"{}\"}} {}",
                outcome.label(),
                self.get(outcome)
            );
        }
        out.push_str("# TYPE allwin_ingress_request_duration_seconds histogram\n");
        for (outcome, l) in Outcome::ALL.iter().zip(self.latency.iter()) {
            let labels = format!("adapter=\"tradingview\",result=\"{}\"", outcome.label());
            l.render("allwin_ingress_request_duration_seconds", &labels, out);
        }
        out.push_str("# TYPE allwin_ingress_db_acquire_seconds histogram\n");
        self.db_acquire.render(
            "allwin_ingress_db_acquire_seconds",
            "adapter=\"tradingview\"",
            out,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bound_belongs_to_its_own_bucket() {
        let m = IngressMetrics::default();
        m.observe(Outcome::Accepted, Duration::from_millis(300));
        m.observe(Outcome::Accepted, Duration::from_micros(300_001));
        m.observe(Outcome::Accepted, Duration::from_secs(3));
        m.observe(Outcome::Accepted, Duration::from_secs(6));
        let l = &m.latency[0];
        let at = |le: f64| {
            let n = BUCKETS.iter().position(|b| *b == le).unwrap();
            l.buckets[n].load(Ordering::Relaxed)
        };
        assert_eq!((at(0.3), at(0.5), at(3.0)), (1, 1, 1));
        assert_eq!(l.buckets[BUCKETS.len()].load(Ordering::Relaxed), 1);
        assert_eq!(l.count.load(Ordering::Relaxed), 4);
    }

    #[test]
    fn latency_is_bucketed_per_result_with_cumulative_counts() {
        let m = IngressMetrics::default();
        m.observe(Outcome::Accepted, Duration::from_millis(3));
        m.observe(Outcome::Accepted, Duration::from_millis(40));
        m.observe(Outcome::Accepted, Duration::from_secs(9));
        m.observe(Outcome::NotFound, Duration::from_millis(1));
        let mut out = String::new();
        m.render(&mut out);
        let line = |s: &str| out.lines().find(|l| l.starts_with(s)).map(str::to_owned);
        let acc = r#"allwin_ingress_request_duration_seconds_bucket{adapter="tradingview",result="accepted","#;
        assert_eq!(
            line(&format!(r#"{acc}le="0.005"}}"#)).as_deref(),
            Some(&*format!(r#"{acc}le="0.005"}} 1"#))
        );
        assert_eq!(
            line(&format!(r#"{acc}le="0.05"}}"#)).as_deref(),
            Some(&*format!(r#"{acc}le="0.05"}} 2"#))
        );
        assert_eq!(
            line(&format!(r#"{acc}le="+Inf"}}"#)).as_deref(),
            Some(&*format!(r#"{acc}le="+Inf"}} 3"#))
        );
        assert!(out.contains(
            r#"allwin_ingress_request_duration_seconds_count{adapter="tradingview",result="accepted"} 3"#
        ));
        assert!(out.contains(
            r#"allwin_ingress_request_duration_seconds_count{adapter="tradingview",result="not_found"} 1"#
        ));
        assert!(out.contains(
            r#"allwin_ingress_request_duration_seconds_sum{adapter="tradingview",result="accepted"} 9.043000"#
        ));
        // Only the adapter label for the pool wait; the same buckets.
        m.observe_db_acquire(Duration::from_millis(20));
        let mut out = String::new();
        m.render(&mut out);
        assert!(out.contains(
            r#"allwin_ingress_db_acquire_seconds_bucket{adapter="tradingview",le="0.01"} 0"#
        ));
        assert!(out.contains(
            r#"allwin_ingress_db_acquire_seconds_bucket{adapter="tradingview",le="0.025"} 1"#
        ));
        assert!(
            out.contains(r#"allwin_ingress_db_acquire_seconds_count{adapter="tradingview"} 1"#)
        );
        // Only these label names, never a key, user or token.
        for l in out.lines().filter(|l| l.contains("_seconds{")) {
            let labels = l.split_once('{').and_then(|(_, rest)| rest.split_once('}'));
            for pair in labels.unwrap().0.split(',') {
                let name = pair.split_once('=').unwrap().0;
                assert!(["adapter", "result", "le"].contains(&name), "{l}");
            }
        }
    }
}
