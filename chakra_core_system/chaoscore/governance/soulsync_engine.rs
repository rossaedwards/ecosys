//! SoulSync coherence scoring engine for vote weighting
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use chrono::Utc;

use crate::models::SoulCoherence;
use crate::audit_log::AuditLogger;

/// SoulSync engine computes and caches soul coherence scores for governance vote weighting
pub struct SoulSyncEngine {
    coherence_cache: Arc<RwLock<HashMap<String, SoulCoherence>>>,
    audit_logger: Arc<AuditLogger>,
}

impl SoulSyncEngine {
    /// Create new SoulSync engine with given audit logger
    pub fn new(audit_logger: Arc<AuditLogger>) -> Self {
        Self {
            coherence_cache: Arc::new(RwLock::new(HashMap::new())),
            audit_logger,
        }
    }

    /// Compute weighted vote strength for a BlissID asynchronously
    pub async fn compute_vote_weight(&self, bliss_id: &str) -> Result<f64, String> {
        let coherence = self.get_or_compute_coherence(bliss_id).await?;
        Ok(coherence.compute_vote_weight())
    }

    /// Fetch coherence from cache or compute fresh if stale (>1 hour)
    async fn get_or_compute_coherence(&self, bliss_id: &str) -> Result<SoulCoherence, String> {
        {
            let cache = self.coherence_cache.read().unwrap();
            if let Some(coherence) = cache.get(bliss_id) {
                let now = Utc::now().timestamp();
                if now - coherence.last_updated < 3600 {
                    return Ok(coherence.clone());
                }
            }
        }

        // Compute fresh coherence metrics
        let coherence = self.compute_coherence(bliss_id).await?;

        {
            let mut cache = self.coherence_cache.write().unwrap();
            cache.insert(bliss_id.to_string(), coherence.clone());
        }

        self.audit_logger.log_event(
            "coherence_computed",
            &format!("bliss_id={}, coherence={:.3}", bliss_id, coherence.overall_coherence),
        );

        Ok(coherence)
    }

    /// Actual coherence computation logic (replace placeholder with real data sources)
    async fn compute_coherence(&self, bliss_id: &str) -> Result<SoulCoherence, String> {
        // TODO: Integrate with real data inputs:
        // - HRV (heart rate variability) from wearables
        // - Governance participation history
        // - Ethical alignment from behavior analytics

        // Placeholder values for demonstration
        let hrv_coherence = 0.7;
        let participation_rate = 0.5;
        let ethical_alignment = 0.8;

        let overall = (hrv_coherence + participation_rate + ethical_alignment) / 3.0;

        Ok(SoulCoherence {
            bliss_id: bliss_id.to_string(),
            hrv_coherence,
            participation_rate,
            ethical_alignment,
            overall_coherence: overall,
            last_updated: Utc::now().timestamp(),
        })
    }

    /// Allow manual admin/test update of coherence scores
    pub fn update_coherence(
        &self,
        bliss_id: String,
        hrv: f64,
        participation: f64,
        ethical: f64,
    ) -> SoulCoherence {
        let overall = (hrv + participation + ethical) / 3.0;
        let coherence = SoulCoherence {
            bliss_id: bliss_id.clone(),
            hrv_coherence: hrv,
            participation_rate: participation,
            ethical_alignment: ethical,
            overall_coherence: overall,
            last_updated: Utc::now().timestamp(),
        };

        let mut cache = self.coherence_cache.write().unwrap();
        cache.insert(bliss_id, coherence.clone());

        coherence
    }

    /// Retrieve coherence score for a BlissID from cache
    pub fn get_coherence(&self, bliss_id: &str) -> Option<SoulCoherence> {
        let cache = self.coherence_cache.read().unwrap();
        cache.get(bliss_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit_log::AuditLogger;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_vote_weight_computation() {
        let audit_logger = Arc::new(AuditLogger::new());
        let engine = SoulSyncEngine::new(audit_logger);

        let weight = engine.compute_vote_weight("bliss:test").await.unwrap();

        // Base weight 1.0 plus coherence bonus (max 1.5)
        assert!(weight >= 1.0 && weight <= 1.5);
    }

    #[test]
    fn test_manual_coherence_update() {
        let audit_logger = Arc::new(AuditLogger::new());
        let engine = SoulSyncEngine::new(audit_logger);

        let coherence = engine.update_coherence(
            "bliss:manual".to_string(),
            0.9,
            0.8,
            0.95,
        );

        assert_eq!(coherence.overall_coherence, (0.9 + 0.8 + 0.95) / 3.0);
        assert!(coherence.compute_vote_weight() > 1.4);
    }
}