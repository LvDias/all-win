//! Negative cache of unknown `key_id`s (s1-webhook-ingest §2 step 2): a canonical token
//! whose `key_id` does not exist is answered without a database lookup for a while.
//! Only "not found" is cached; revoked or inactive keys always hit the database.

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct UnknownKeyCache {
    ttl: Duration,
    capacity: usize,
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    expires: HashMap<String, Instant>,
    /// Insertion order, for bounded eviction (oldest first).
    order: VecDeque<String>,
}

impl UnknownKeyCache {
    pub fn new(ttl: Duration, capacity: usize) -> Self {
        Self {
            ttl,
            capacity: capacity.max(1),
            inner: Mutex::new(Inner::default()),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    pub fn is_unknown(&self, key_id: &str, now: Instant) -> bool {
        let mut inner = self.lock();
        match inner.expires.get(key_id) {
            Some(expires) if *expires > now => true,
            Some(_) => {
                inner.expires.remove(key_id);
                false
            }
            None => false,
        }
    }

    pub fn remember_unknown(&self, key_id: &str, now: Instant) {
        if self.ttl.is_zero() {
            return;
        }
        let mut inner = self.lock();
        if inner
            .expires
            .insert(key_id.to_owned(), now + self.ttl)
            .is_none()
        {
            inner.order.push_back(key_id.to_owned());
        }
        while inner.expires.len() > self.capacity {
            match inner.order.pop_front() {
                Some(oldest) => {
                    inner.expires.remove(&oldest);
                }
                None => break,
            }
        }
        // `order` may hold ids already removed (expired or invalidated); keep it bounded.
        if inner.order.len() > self.capacity.saturating_mul(2) {
            let Inner { expires, order } = &mut *inner;
            order.retain(|k| expires.contains_key(k));
        }
    }

    /// Called when a key is created, so a fresh key is never answered as unknown.
    pub fn invalidate(&self, key_id: &str) {
        self.lock().expires.remove(key_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remembers_until_ttl_and_can_be_invalidated() {
        let c = UnknownKeyCache::new(Duration::from_secs(60), 10);
        let t0 = Instant::now();
        assert!(!c.is_unknown("AAAAAAAAAAAA", t0));
        c.remember_unknown("AAAAAAAAAAAA", t0);
        assert!(c.is_unknown("AAAAAAAAAAAA", t0 + Duration::from_secs(59)));
        assert!(!c.is_unknown("AAAAAAAAAAAA", t0 + Duration::from_secs(60)));
        c.remember_unknown("BBBBBBBBBBBB", t0);
        c.invalidate("BBBBBBBBBBBB");
        assert!(!c.is_unknown("BBBBBBBBBBBB", t0));
    }

    #[test]
    fn bounded_by_capacity_oldest_first() {
        let c = UnknownKeyCache::new(Duration::from_secs(60), 2);
        let t0 = Instant::now();
        for k in ["k1", "k2", "k3"] {
            c.remember_unknown(k, t0);
        }
        assert!(!c.is_unknown("k1", t0));
        assert!(c.is_unknown("k2", t0) && c.is_unknown("k3", t0));
    }
}
