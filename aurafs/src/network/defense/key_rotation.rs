//! Key rotation scheduler for mesh peers.

use std::time::{Duration, Instant};

/// [Theorem 3.1: Universality]
/// Tracks rotation intervals for key material.
pub struct KeyRotation {
    interval: Duration,
    last_rotated: Instant,
}

impl KeyRotation {
    /// [Theorem 3.1: Universality]
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_rotated: Instant::now(),
        }
    }

    /// [Theorem 3.1: Universality]
    pub fn should_rotate(&self) -> bool {
        self.last_rotated.elapsed() >= self.interval
    }

    /// [Theorem 3.1: Universality]
    pub fn mark_rotated(&mut self) {
        self.last_rotated = Instant::now();
    }
}