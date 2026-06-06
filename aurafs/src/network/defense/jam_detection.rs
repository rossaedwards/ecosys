//! RF jamming detection.

/// [Theorem 3.1: Universality]
/// Detects potential RF jamming based on signal metrics.
pub struct JamDetector {
    threshold: f64,
}

impl JamDetector {
    /// [Theorem 3.1: Universality]
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    /// [Theorem 3.1: Universality]
    pub fn is_jammed(&self, noise_floor_dbm: f64, snr_db: f64) -> bool {
        let jam_score = noise_floor_dbm.abs() / (snr_db.max(1.0));
        jam_score > self.threshold
    }
}