//! DoS protection for Meshwerk.

use crate::network::defense::rate_limiter::RateLimiter;

/// [Theorem 3.1: Universality]
/// Simple DoS protector using a rate limiter.
pub struct DosProtector {
    limiter: RateLimiter,
}

impl DosProtector {
    /// [Theorem 3.1: Universality]
    pub fn new(max_per_sec: u64) -> Self {
        Self {
            limiter: RateLimiter::new(max_per_sec, max_per_sec),
        }
    }

    /// [Theorem 3.1: Universality]
    pub fn allow(&mut self, peer_id: &str) -> bool {
        self.limiter.allow(peer_id)
    }
}