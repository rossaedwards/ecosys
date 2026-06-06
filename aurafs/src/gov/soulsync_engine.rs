//! SoulSync Engine - Validates SoulShots for Uniqueness
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use crate::gov::AuditLogger;
use crate::crypto::pqc::dilithium_sig;
use crate::config::AuraConfig;
use crate::gov::sages::{SagesLoop, SagesVote};

/// SoulSync engine now strictly validates the existence of a unique SoulShot
/// rather than calculating a variable "reputation score" for voting.
pub struct SoulSyncEngine {
    // In production, this connects to the SoulShot Enclave/Biometric Node.
    // For Phase II simulation, we track validity in memory.
    valid_souls: Arc<RwLock<HashMap<String, SoulShotRecord>>>, 
    audit_logger: Arc<AuditLogger>,
    sages: Arc<RwLock<SagesLoop>>,
}

#[derive(Debug, Clone)]
struct SoulShotRecord {
    active: bool,
    public_key: dilithium_sig::PublicKey,
    signature: Vec<u8>,
    issued_at: i64,
}

impl SoulSyncEngine {
    pub fn new(audit_logger: Arc<AuditLogger>, sages: Arc<RwLock<SagesLoop>>) -> Self {
        Self {
            valid_souls: Arc::new(RwLock::new(HashMap::new())),
            audit_logger,
            sages,
        }
    }

    /// [Theorem 3.1: Universality]
    /// Register a SoulShot attestation bound to a Dilithium-5 public key.
    pub fn register_soul_shot(
        &self,
        bliss_id: String,
        public_key: dilithium_sig::PublicKey,
        signature: Vec<u8>,
    ) -> Result<(), String> {
        let mut souls = self.valid_souls.write().unwrap();
        let record = SoulShotRecord {
            active: true,
            public_key,
            signature,
            issued_at: Utc::now().timestamp(),
        };
        souls.insert(bliss_id.clone(), record);
        self.audit_logger.log_event(
            "soul_register",
            &format!("Registered SoulShot for {}", bliss_id),
        );
        Ok(())
    }

    /// Verifies that the BlissID is attached to a valid, unique SoulShot.
    /// Returns TRUE if the soul is unique and alive.
    /// This is the "Gatekeeper" for the 1 Soul = 1 Vote system.
    /// [Theorem 3.1: Universality]
    pub async fn verify_soul_shot(&self, bliss_id: &str) -> Result<bool, String> {
        let souls = self.valid_souls.read().unwrap();
        let record = match souls.get(bliss_id) {
            Some(record) => record.clone(),
            None => {
                self.audit_logger.log_event(
                    "soul_rejection",
                    &format!("BlissID {} missing SoulShot registration", bliss_id),
                );
                return Ok(false);
            }
        };

        if !record.active {
            self.audit_logger.log_event(
                "soul_rejection",
                &format!("BlissID {} is revoked", bliss_id),
            );
            return Ok(false);
        }

        let message = Self::soulshot_message(bliss_id);
        let is_valid = dilithium_sig::verify(&message, &record.signature, &record.public_key)
            .map_err(|e| format!("Dilithium-5 verify fail: {:?}", e))?;

        if !is_valid {
            self.audit_logger.log_event(
                "soul_rejection",
                &format!("BlissID {} failed SoulShot signature verification", bliss_id),
            );
        }

        Ok(is_valid)
    }

    /// Validate S.A.G.E.S. quorum for a given proposal.
    pub fn validate_sages_quorum(&self, proposal_id: &str) -> Result<bool, String> {
        let min_quorum = Self::load_min_quorum()?;
        let votes = self.sages.read().unwrap().votes_for_proposal(proposal_id);
        let mut valid_votes = 0u64;

        for vote in votes {
            if Self::verify_sages_vote(&vote)? {
                valid_votes += 1;
            } else {
                self.audit_logger.log_event(
                    "sages_vote_reject",
                    &format!("Invalid S.A.G.E.S. vote for proposal {}", proposal_id),
                );
            }
        }

        let quorum_met = valid_votes >= min_quorum;
        self.audit_logger.log_event(
            "sages_quorum_check",
            &format!(
                "Proposal {} quorum {} (valid_votes={}, required={})",
                proposal_id,
                if quorum_met { "met" } else { "not_met" },
                valid_votes,
                min_quorum
            ),
        );

        Ok(quorum_met)
    }

    /// Manually invalidate a SoulShot (e.g., if a duplicate is detected by the bio-swarm)
    /// [Theorem 3.1: Universality]
    pub fn revoke_soul_shot(&self, bliss_id: String, reason: String) {
        let mut souls = self.valid_souls.write().unwrap();
        if let Some(record) = souls.get_mut(&bliss_id) {
            record.active = false;
        }

        self.audit_logger.log_event(
            "soul_revocation",
            &format!("Revoked {} | Reason: {}", bliss_id, reason),
        );
    }

    /// [Theorem 3.1: Universality]
    /// Generate the canonical SoulShot message for Dilithium-5 binding.
    fn soulshot_message(bliss_id: &str) -> Vec<u8> {
        format!("soulsync:{}", bliss_id).into_bytes()
    }

    fn verify_sages_vote(vote: &SagesVote) -> Result<bool, String> {
        let message = SagesVote::payload(&vote.proposal_id, &vote.sentinel);
        dilithium_sig::verify(&message, &vote.signature, &vote.public_key)
            .map_err(|e| format!("Dilithium-5 verify fail: {:?}", e))
    }

    fn load_min_quorum() -> Result<u64, String> {
        AuraConfig::load()
            .map(|config| config.governance.min_quorum)
            .map_err(|e| format!("Failed to load governance quorum: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gov::AuditLogger;
    use std::sync::Arc;
    use crate::gov::sages::{SagesLoop, SentinelRole};

    #[tokio::test]
    async fn test_soul_verification() {
        let audit_logger = Arc::new(AuditLogger::new());
        let sages = Arc::new(RwLock::new(SagesLoop::new()));
        let engine = SoulSyncEngine::new(audit_logger, sages);

        let (public_key, secret_key) = dilithium_sig::dilithium_keygen();
        let message = b"soulsync:bliss:test_user";
        let signature = dilithium_sig::sign(message, &secret_key).unwrap();
        engine.register_soul_shot("bliss:test_user".to_string(), public_key, signature).unwrap();

        // Verification should pass for registered users
        let is_valid = engine.verify_soul_shot("bliss:test_user").await.unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_revocation() {
        let audit_logger = Arc::new(AuditLogger::new());
        let sages = Arc::new(RwLock::new(SagesLoop::new()));
        let engine = SoulSyncEngine::new(audit_logger, sages);

        let (public_key, secret_key) = dilithium_sig::dilithium_keygen();
        let message = b"soulsync:bliss:bad_actor";
        let signature = dilithium_sig::sign(message, &secret_key).unwrap();
        engine.register_soul_shot("bliss:bad_actor".to_string(), public_key, signature).unwrap();
        engine.revoke_soul_shot("bliss:bad_actor".to_string(), "Sybil detected".to_string());
        
        // In a real impl, verify_soul_shot would now return false for this ID
        // (Mock implementation currently defaults true, but the revocation log is key)
        let entries = engine.audit_logger.get_entries();
        assert!(entries.iter().any(|e| e.description.contains("Revoked bliss:bad_actor")));
    }

    #[test]
    fn test_sages_quorum_validation() {
        let audit_logger = Arc::new(AuditLogger::new());
        let sages = Arc::new(RwLock::new(SagesLoop::new()));
        let engine = SoulSyncEngine::new(audit_logger, sages.clone());

        let proposal_id = "proposal-aurafs-001";
        let roles = vec![
            SentinelRole::Vyrellix,
            SentinelRole::Archivus,
            SentinelRole::Sentry,
            SentinelRole::AuraLord,
            SentinelRole::Aegis,
            SentinelRole::Chrona,
            SentinelRole::Heliox,
            SentinelRole::Nexus,
            SentinelRole::Ordo,
            SentinelRole::Paxia,
            SentinelRole::Quanta,
            SentinelRole::Solaris,
            SentinelRole::Umbra,
        ];

        {
            let mut loop_state = sages.write().unwrap();
            for role in roles {
                let (public_key, private_key) = dilithium_sig::dilithium_keygen();
                let payload = SagesVote::payload(proposal_id, &role);
                let signature = dilithium_sig::sign(&payload, &private_key).unwrap();
                loop_state.record_vote(SagesVote::new(
                    proposal_id.to_string(),
                    role,
                    signature,
                    public_key,
                ));
            }
        }

        let quorum = engine.validate_sages_quorum(proposal_id).unwrap();
        assert!(quorum);
    }
}