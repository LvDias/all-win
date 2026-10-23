//! In-memory token buckets (s1-webhook-ingest §2 steps 1 and 3). A restart resets them.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// `per_minute` requests per minute with a burst of the same size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    pub per_minute: u32,
}

/// Tokens are counted in millionths, so refill needs no floating point.
const SCALE: u64 = 1_000_000;

struct Bucket {
    micro_tokens: u64,
    updated: Instant,
}

pub struct KeyedRateLimiter<K> {
    limit: RateLimit,
    buckets: Mutex<HashMap<K, Bucket>>,
}

/// Above this many tracked keys, idle full buckets are dropped.
const PRUNE_THRESHOLD: usize = 10_000;

impl<K: Eq + Hash + Clone> KeyedRateLimiter<K> {
    pub fn new(limit: RateLimit) -> Self {
        Self {
            limit,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Takes one token for `key`; `Err(retry_after_secs)` when empty.
    pub fn check(&self, key: &K, now: Instant) -> Result<(), u64> {
        let per_minute = u64::from(self.limit.per_minute.max(1));
        let capacity = per_minute * SCALE;
        // micro-tokens gained per elapsed microsecond = per_minute / 60s
        let refill =
            |elapsed: Duration| (elapsed.as_micros() as u64).saturating_mul(per_minute) / 60;
        let mut buckets = match self.buckets.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if buckets.len() > PRUNE_THRESHOLD {
            buckets.retain(|_, b| {
                b.micro_tokens + refill(now.saturating_duration_since(b.updated)) < capacity
            });
        }
        let bucket = buckets.entry(key.clone()).or_insert(Bucket {
            micro_tokens: capacity,
            updated: now,
        });
        let gained = refill(now.saturating_duration_since(bucket.updated));
        bucket.micro_tokens = bucket.micro_tokens.saturating_add(gained).min(capacity);
        bucket.updated = now;
        if bucket.micro_tokens >= SCALE {
            bucket.micro_tokens -= SCALE;
            Ok(())
        } else {
            let missing = SCALE - bucket.micro_tokens;
            // seconds until one full token: missing / (per_minute * SCALE / 60s), rounded up, ≥ 1
            let secs = (missing * 60).div_ceil(per_minute * SCALE);
            Err(secs.max(1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burst_then_refill() {
        let l = KeyedRateLimiter::new(RateLimit { per_minute: 60 });
        let t0 = Instant::now();
        for _ in 0..60 {
            assert!(l.check(&"k", t0).is_ok());
        }
        assert_eq!(l.check(&"k", t0), Err(1));
        assert!(l.check(&"other", t0).is_ok());
        assert!(l.check(&"k", t0 + Duration::from_secs(1)).is_ok());
        assert!(l.check(&"k", t0 + Duration::from_secs(1)).is_err());
    }

    #[test]
    fn retry_after_reflects_slow_refill() {
        let l = KeyedRateLimiter::new(RateLimit { per_minute: 1 });
        let t0 = Instant::now();
        assert!(l.check(&1, t0).is_ok());
        assert_eq!(l.check(&1, t0), Err(60));
    }
}
