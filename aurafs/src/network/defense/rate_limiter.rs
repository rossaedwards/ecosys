//! Rate limiter for Meshwerk traffic.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// [Theorem 3.1: Universality]
/// Token bucket rate limiter for per-peer throttling.
pub struct RateLimiter {
    capacity: u64,
    refill_per_sec: u64,
    buckets: HashMap<String, (u64, Instant)>,
}

impl RateLimiter {
    /// [Theorem 3.1: Universality]
    pub fn new(capacity: u64, refill_per_sec: u64) -> Self {
        Self {
            capacity,
            refill_per_sec,
            buckets: HashMap::new(),
        }
    }

    /// [Theorem 3.1: Universality]
    pub fn allow(&mut self, peer_id: &str) -> bool {
        let now = Instant::now();
        let entry = self.buckets.entry(peer_id.to_string()).or_insert((self.capacity, now));
        let elapsed = now.duration_since(entry.1);
        let refill = (elapsed.as_secs_f64() * self.refill_per_sec as f64).floor() as u64;
        if refill > 0 {
            entry.0 = (entry.0 + refill).min(self.capacity);
            entry.1 = now;
        }
        if entry.0 == 0 {
            false
        } else {
            entry.0 -= 1;
            true
        }
    }
}