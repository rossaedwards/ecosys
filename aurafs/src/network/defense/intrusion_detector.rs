//! Intrusion detection for Meshwerk.

use std::collections::HashMap;

/// [Theorem 3.1: Universality]
/// Simple anomaly detector based on event counts.
pub struct IntrusionDetector {
    threshold: u64,
    counts: HashMap<String, u64>,
}

impl IntrusionDetector {
    /// [Theorem 3.1: Universality]
    pub fn new(threshold: u64) -> Self {
        Self {
            threshold,
            counts: HashMap::new(),
        }
    }

    /// [Theorem 3.1: Universality]
    pub fn record_event(&mut self, peer_id: &str) -> bool {
        let entry = self.counts.entry(peer_id.to_string()).or_insert(0);
        *entry += 1;
        *entry >= self.threshold
    }
}