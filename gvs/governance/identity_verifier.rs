//! Identity verification with BlissID, ID.me integration, and quantum signatures
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use sha3::{Digest, Sha3_256};
use chrono::Utc;

use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlissID {
    pub id: String,
    pub did: String,          // Decentralized Identifier from ID.me
    pub soul_hash: String,    // Biometric commitment hash
    pub created_at: i64,
    pub active: bool,
    pub soul_proof: SoulProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulProof {
    pub commitment: String,
    pub proof_type: String,   // e.g., "zk-SNARK-unique-human"
    pub verifiable: bool,
}

#[derive(Debug, Clone)]
pub struct IdentityVerifier {
    bliss_registry: Arc<RwLock<HashMap<String, BlissID>>>,
    public_keys: Arc<RwLock<HashMap<String, dilithium5::PublicKey>>>,
}

impl IdentityVerifier {
    pub fn new() -> Self {
        Self {
            bliss_registry: Arc::new(RwLock::new(HashMap::new())),
            public_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register new BlissID along with its Dilithium5 public key
    pub fn register_bliss_id(
        &self,
        bliss_id: BlissID,
        public_key: dilithium5::PublicKey,
    ) -> Result<(), String> {
        let mut registry = self.bliss_registry.write().unwrap();
        let mut keys = self.public_keys.write().unwrap();

        // Enforce one soul per account
        if registry.values().any(|existing| existing.soul_hash == bliss_id.soul_hash) {
            return Err(format!("Soul already registered: {}", bliss_id.id));
        }

        registry.insert(bliss_id.id.clone(), bliss_id.clone());
        keys.insert(bliss_id.id.clone(), public_key);

        log::info!("✅ BlissID registered: {}", bliss_id.id);
        Ok(())
    }

    /// Check if BlissID is active and exists
    pub fn verify_bliss_id(&self, bliss_id: &str) -> bool {
        self.bliss_registry
            .read()
            .unwrap()
            .get(bliss_id)
            .map(|id| id.active)
            .unwrap_or(false)
    }

    /// Verify PQ signature (Dilithium5) for a given message
    pub fn verify_quantum_signature(
        &self,
        bliss_id: &str,
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, String> {
        let keys = self.public_keys.read().unwrap();
        let public_key = keys.get(bliss_id).ok_or_else(|| format!("No public key for BlissID: {}", bliss_id))?;

        let sig = dilithium5::DetachedSignature::from_bytes(signature)
            .map_err(|e| format!("Invalid signature format: {:?}", e))?;

        match dilithium5::verify_detached_signature(&sig, message, public_key) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Verify zero-knowledge soul proof matches registered commitment
    pub fn verify_soul_proof(&self, bliss_id: &str, soul_proof: &SoulProof) -> bool {
        let registry = self.bliss_registry.read().unwrap();

        if let Some(registered) = registry.get(bliss_id) {
            let computed = self.compute_commitment(&registered.soul_hash, bliss_id);
            soul_proof.commitment == computed
                && soul_proof.proof_type == "zk-SNARK-unique-human"
                && soul_proof.verifiable
        } else {
            false
        }
    }

    /// Compute commitment hash combining soul_hash and bliss_id
    fn compute_commitment(&self, soul_hash: &str, bliss_id: &str) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(format!("{}:{}", bliss_id, soul_hash).as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Get BlissID info from registry
    pub fn get_bliss_id(&self, bliss_id: &str) -> Option<BlissID> {
        self.bliss_registry.read().unwrap().get(bliss_id).cloned()
    }

    /// Deactivate a BlissID — useful for security violations or policy breaches
    pub fn deactivate_bliss_id(&self, bliss_id: &str) -> Result<(), String> {
        let mut registry = self.bliss_registry.write().unwrap();

        if let Some(id) = registry.get_mut(bliss_id) {
            id.active = false;
            log::warn!("🚫 BlissID deactivated: {}", bliss_id);
            Ok(())
        } else {
            Err(format!("BlissID not found: {}", bliss_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqcrypto_dilithium::dilithium5;

    #[test]
    fn test_bliss_id_registration() {
        let verifier = IdentityVerifier::new();
        let (pk, _sk) = dilithium5::keypair();

        let bliss_id = BlissID {
            id: "bliss:test-123".to_string(),
            did: "did:example:abc123".to_string(),
            soul_hash: "hash123".to_string(),
            created_at: Utc::now().timestamp(),
            active: true,
            soul_proof: SoulProof {
                commitment: "commit123".to_string(),
                proof_type: "zk-SNARK-unique-human".to_string(),
                verifiable: true,
            },
        };

        assert!(verifier.register_bliss_id(bliss_id.clone(), pk).is_ok());
        assert!(verifier.verify_bliss_id(&bliss_id.id));
    }

    #[test]
    fn test_duplicate_soul_prevention() {
        let verifier = IdentityVerifier::new();
        let (pk1, _) = dilithium5::keypair();
        let (pk2, _) = dilithium5::keypair();

        let bliss_id1 = BlissID {
            id: "bliss:user1".to_string(),
            did: "did:user1".to_string(),
            soul_hash: "same_soul_hash".to_string(),
            created_at: Utc::now().timestamp(),
            active: true,
            soul_proof: SoulProof {
                commitment: "c1".to_string(),
                proof_type: "zk-SNARK-unique-human".to_string(),
                verifiable: true,
            },
        };

        let bliss_id2 = BlissID {
            id: "bliss:user2".to_string(),
            did: "did:user2".to_string(),
            soul_hash: "same_soul_hash".to_string(), // DUPLICATE
            created_at: Utc::now().timestamp(),
            active: true,
            soul_proof: SoulProof {
                commitment: "c2".to_string(),
                proof_type: "zk-SNARK-unique-human".to_string(),
                verifiable: true,
            },
        };

        assert!(verifier.register_bliss_id(bliss_id1, pk1).is_ok());
        assert!(verifier.register_bliss_id(bliss_id2, pk2).is_err());
    }
}