//! Operational counters by result, never by key, user or token
//! (s1-webhook-ingest §2 steps 1–2).

use std::fmt::Write;
use std::sync::atomic::{AtomicU64, Ordering};

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

#[derive(Debug, Default)]
pub struct IngressMetrics {
    counts: [AtomicU64; 8],
}

impl IngressMetrics {
    pub fn record(&self, outcome: Outcome) {
        if let Some(i) = Outcome::ALL.iter().position(|o| *o == outcome) {
            self.counts[i].fetch_add(1, Ordering::Relaxed);
        }
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
    }
}
