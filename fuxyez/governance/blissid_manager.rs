//! BlissID lifecycle and registration management
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use log::{info, warn};
use serde::{Serialize, Deserialize};

use crate::identity_verifier::{BlissID, SoulProof};
use crate::audit_log::AuditLogger;

pub struct BlissIDManager {
    registry: Arc<RwLock<HashMap<String, BlissID>>>,
    soul_hash_index: Arc<RwLock<HashMap<String, String>>>,  // soul_hash -> bliss_id
    audit_logger: Arc<AuditLogger>,
}

impl BlissIDManager {
    pub fn new(audit_logger: Arc<AuditLogger>) -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            soul_hash_index: Arc::new(RwLock::new(HashMap::new())),
            audit_logger,
        }
    }

    /// Registers a new BlissID ensuring soul uniqueness
    pub fn register_bliss_id(
        &self,
        did: String,
        soul_hash: String,
        soul_proof: SoulProof,
    ) -> Result<BlissID, String> {
        {
            let index = self.soul_hash_index.read().unwrap();
            if let Some(existing) = index.get(&soul_hash) {
                return Err(format!("Soul already registered with BlissID: {}", existing));
            }
        }

        let bliss_id = format!("bliss:{}", uuid::Uuid::new_v4());
        let now = Utc::now().timestamp();

        let bliss = BlissID {
            id: bliss_id.clone(),
            did,
            soul_hash: soul_hash.clone(),
            created_at: now,
            active: true,
            soul_proof,
        };

        {
            let mut registry = self.registry.write().unwrap();
            registry.insert(bliss_id.clone(), bliss.clone());

            let mut index = self.soul_hash_index.write().unwrap();
            index.insert(soul_hash, bliss_id.clone());
        }

        self.audit_logger.log_event(
            "blissid_registered",
            &format!("id={}, did={}", bliss_id, bliss.did),
        );

        info!("✨ BlissID registered: {}", bliss_id);
        Ok(bliss)
    }

    /// Verifies if BlissID is active
    pub fn verify_bliss_id(&self, bliss_id: &str) -> bool {
        self.registry
            .read()
            .unwrap()
            .get(bliss_id)
            .map(|b| b.active)
            .unwrap_or(false)
    }

    /// Gets a BlissID by its ID
    pub fn get_bliss_id(&self, bliss_id: &str) -> Option<BlissID> {
        self.registry.read().unwrap().get(bliss_id).cloned()
    }

    /// Deactivates a BlissID with a reason
    pub fn deactivate_bliss_id(&self, bliss_id: &str, reason: &str) -> Result<(), String> {
        let mut registry = self.registry.write().unwrap();

        let bliss = registry
            .get_mut(bliss_id)
            .ok_or_else(|| format!("BlissID not found: {}", bliss_id))?;

        bliss.active = false;

        self.audit_logger.log_event(
            "blissid_deactivated",
            &format!("id={}, reason={}", bliss_id, reason),
        );

        warn!("🚫 BlissID deactivated: {} (reason: {})", bliss_id, reason);
        Ok(())
    }

    /// Reactivates a BlissID
    pub fn reactivate_bliss_id(&self, bliss_id: &str) -> Result<(), String> {
        let mut registry = self.registry.write().unwrap();

        let bliss = registry
            .get_mut(bliss_id)
            .ok_or_else(|| format!("BlissID not found: {}", bliss_id))?;

        bliss.active = true;

        self.audit_logger.log_event(
            "blissid_reactivated",
            &format!("id={}", bliss_id),
        );

        info!("✅ BlissID reactivated: {}", bliss_id);
        Ok(())
    }

    /// Checks if a soul hash is already registered
    pub fn soul_exists(&self, soul_hash: &str) -> bool {
        self.soul_hash_index.read().unwrap().contains_key(soul_hash)
    }

    /// Returns total registered BlissIDs count
    pub fn get_total_count(&self) -> usize {
        self.registry.read().unwrap().len()
    }

    /// Returns count of active BlissIDs
    pub fn get_active_count(&self) -> usize {
        self.registry.read().unwrap().values().filter(|b| b.active).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity_verifier::SoulProof;
    use std::sync::Arc;

    #[test]
    fn test_blissid_registration() {
        let audit_logger = Arc::new(AuditLogger::new());
        let manager = BlissIDManager::new(audit_logger);

        let soul_proof = SoulProof {
            commitment: "test_commit".to_string(),
            proof_type: "zk-SNARK-unique-human".to_string(),
            verifiable: true,
        };

        let bliss = manager
            .register_bliss_id(
                "did:test:123".to_string(),
                "soul_hash_123".to_string(),
                soul_proof,
            )
            .unwrap();

        assert!(manager.verify_bliss_id(&bliss.id));
        assert_eq!(manager.get_total_count(), 1);
    }

    #[test]
    fn test_duplicate_soul_prevention() {
        let audit_logger = Arc::new(AuditLogger::new());
        let manager = BlissIDManager::new(audit_logger);

        let soul_proof = SoulProof {
            commitment: "test".to_string(),
            proof_type: "zk-SNARK-unique-human".to_string(),
            verifiable: true,
        };

        // First registration succeeds
        manager
            .register_bliss_id(
                "did:1".to_string(),
                "duplicate_hash".to_string(),
                soul_proof.clone(),
            )
            .unwrap();

        // Duplicate registration fails
        let res = manager.register_bliss_id(
            "did:2".to_string(),
            "duplicate_hash".to_string(),
            soul_proof,
        );

        assert!(res.is_err());
        assert!(res.unwrap_err().contains("already registered"));
    }
}