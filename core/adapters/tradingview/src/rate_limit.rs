//! In-memory token buckets (s1-webhook-ingest §2 steps 1–3). A restart resets them.
//!
//! Memory stays bounded: idle full buckets are pruned (at most once per second, so a
//! flood of new keys cannot make every request pay an O(n) sweep), and beyond
//! [`MAX_TRACKED`] keys every new key shares one overflow bucket instead of growing
//! the map.

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
/// Above this many tracked keys, idle full buckets are pruned.
const PRUNE_THRESHOLD: usize = 10_000;
/// Hard cap on tracked keys; new keys beyond it share the overflow bucket.
pub const MAX_TRACKED: usize = 100_000;
const PRUNE_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone, Copy)]
struct Bucket {
    micro_tokens: u64,
    updated: Instant,
}

struct Buckets<K> {
    map: HashMap<K, Bucket>,
    overflow: Option<Bucket>,
    last_prune: Option<Instant>,
}

pub struct KeyedRateLimiter<K> {
    limit: RateLimit,
    buckets: Mutex<Buckets<K>>,
}

impl<K: Eq + Hash + Clone> KeyedRateLimiter<K> {
    pub fn new(limit: RateLimit) -> Self {
        Self {
            limit,
            buckets: Mutex::new(Buckets {
                map: HashMap::new(),
                overflow: None,
                last_prune: None,
            }),
        }
    }

    /// Takes one token for `key`; `Err(retry_after_secs)` when empty.
    pub fn check(&self, key: &K, now: Instant) -> Result<(), u64> {
        let per_minute = u64::from(self.limit.per_minute.max(1));
        let capacity = per_minute * SCALE;
        // micro-tokens gained per elapsed microsecond = per_minute / 60 s
        let refill =
            |elapsed: Duration| (elapsed.as_micros() as u64).saturating_mul(per_minute) / 60;
        let mut guard = match self.buckets.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let buckets = &mut *guard;
        if buckets.map.len() > PRUNE_THRESHOLD
            && buckets
                .last_prune
                .is_none_or(|t| now.saturating_duration_since(t) >= PRUNE_INTERVAL)
        {
            buckets.last_prune = Some(now);
            buckets.map.retain(|_, b| {
                b.micro_tokens + refill(now.saturating_duration_since(b.updated)) < capacity
            });
        }
        let fresh = Bucket {
            micro_tokens: capacity,
            updated: now,
        };
        let bucket = if buckets.map.contains_key(key) || buckets.map.len() < MAX_TRACKED {
            buckets.map.entry(key.clone()).or_insert(fresh)
        } else {
            buckets.overflow.get_or_insert(fresh)
        };
        let gained = refill(now.saturating_duration_since(bucket.updated));
        bucket.micro_tokens = bucket.micro_tokens.saturating_add(gained).min(capacity);
        bucket.updated = now;
        if bucket.micro_tokens >= SCALE {
            bucket.micro_tokens -= SCALE;
            Ok(())
        } else {
            let missing = SCALE - bucket.micro_tokens;
            // seconds until one full token, rounded up, ≥ 1
            Err((missing * 60).div_ceil(per_minute * SCALE).max(1))
        }
    }

    #[cfg(test)]
    fn tracked(&self) -> usize {
        self.buckets.lock().map(|b| b.map.len()).unwrap_or_default()
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

    #[test]
    fn tracked_keys_are_capped_and_overflow_shares_one_bucket() {
        let l = KeyedRateLimiter::new(RateLimit { per_minute: 2 });
        let t0 = Instant::now();
        for k in 0..MAX_TRACKED as u64 {
            // Two hits leave each bucket non-full, so pruning cannot reclaim them.
            let _ = l.check(&k, t0);
        }
        assert_eq!(l.tracked(), MAX_TRACKED);
        let (a, b, c) = (u64::MAX, u64::MAX - 1, u64::MAX - 2);
        assert!(l.check(&a, t0).is_ok());
        assert!(l.check(&b, t0).is_ok());
        assert!(
            l.check(&c, t0).is_err(),
            "new keys share the overflow bucket"
        );
        assert_eq!(l.tracked(), MAX_TRACKED);
    }
}
