//! ═══════════════════════════════════════════════════════════════════
//! 🔮 AuraFS Core Bliss - Quantum-Safe Identity Manager (ENTERPRISE-GRADE)
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Complete lifecycle with crypto, ZK proofs, soul verification, and enterprise features.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use std::collections::HashMap;
use std::sync::Arc;
use std::fmt;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sha3::{Sha3_256, Digest};

use crate::core::{
    Result, AuraFSError, ErrorCode, ErrorPhase, client, internal,
};
use crate::core::crypto::{DilithiumKeypair, sha3_256_digest, gen_random_bytes};
use crate::core::soulproof::{SoulProof, ProofStatus};

/// Quantum-safe BlissID (Soul Identity) - content-addressed from biometric hash
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BlissId(pub String);

impl BlissId {
    /// Generate new BlissID from biometric hash (SHA3-256)
    pub fn new(biometric_hash: &[u8]) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(b"blissid_v1:");
        hasher.update(biometric_hash);
        let digest = hasher.finalize();
        Self(hex::encode(digest))
    }
    
    /// Generate genesis BlissID (system root)
    pub fn genesis() -> Self {
        Self("genesis_0000000000000000000000000000000000000000000000000000000000000000".to_string())
    }
    
    /// Create from hex string with validation
    pub fn from_hex(hex: &str) -> Result<Self> {
        if hex.is_empty() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: None,
                    message: "BlissID hex string is empty".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        // Validate hex format (64 hex chars for SHA3-256)
        if hex.len() != 64 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: None,
                    message: format!("Invalid BlissID hex format: expected 64 hex chars, got {}", hex.len()),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        Ok(Self(hex.to_string()))
    }
    
    /// Validate BlissID format
    pub fn is_valid(&self) -> bool {
        self.0.len() == 64 && self.0.chars().all(|c| c.is_ascii_hexdigit())
    }
    
    /// Get hex representation
    pub fn to_hex(&self) -> &str {
        &self.0
    }
    
    /// Get as bytes (for hashing/signing)
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Display for BlissId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for BlissId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// BlissID Manager trait for identity lifecycle
#[async_trait::async_trait]
pub trait BlissIdManager: Send + Sync {
    /// Register new BlissID with soul proof
    async fn register_blissid(&self, record: BlissIdRecord) -> Result<()>;
    
    /// Verify BlissID with proof
    async fn verify_blissid(&self, blissid: &BlissId, proof: &SoulProof) -> Result<bool>;
    
    /// Deactivate BlissID
    async fn deactivate_blissid(&self, blissid: &BlissId) -> Result<()>;
    
    /// Check if BlissID is active
    async fn is_blissid_active(&self, blissid: &BlissId) -> Result<bool>;
    
    /// Get BlissID record
    async fn get_record(&self, blissid: &BlissId) -> Result<Option<BlissIdRecord>>;
    
    /// List all active BlissIDs
    async fn list_active(&self) -> Result<Vec<BlissId>>;
}

/// BlissID record with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlissIdRecord {
    pub blissid: BlissId,
    pub registered_at: DateTime<Utc>,
    pub proof: SoulProof,
    pub active: bool,
    /// Manager's Dilithium5 signature over registration/deactivation
    pub manager_signature: String,
    pub metadata: HashMap<String, String>,
    /// Last verification timestamp
    pub last_verified_at: Option<DateTime<Utc>>,
    /// Verification count
    pub verification_count: u64,
}

impl BlissIdRecord {
    /// Create new BlissID record with validation
    pub fn new(
        blissid: BlissId,
        proof: SoulProof,
        metadata: HashMap<String, String>,
    ) -> Result<Self> {
        // Validate BlissID
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        Ok(Self {
            blissid,
            registered_at: Utc::now(),
            proof,
            active: true,
            manager_signature: String::new(),
            metadata,
            last_verified_at: None,
            verification_count: 0,
        })
    }
    
    /// Validate record integrity
    pub fn validate(&self) -> Result<()> {
        if !self.blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(self.blissid.clone()),
                    message: "Invalid BlissID in record".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        if self.manager_signature.is_empty() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::InvalidSignature,
                    soul_id: Some(self.blissid.clone()),
                    message: "Missing manager signature".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::InvalidSignature,
            ));
        }
        
        Ok(())
    }
}

/// In-memory BlissID Manager (for testing/development)
pub struct InMemoryBlissIdManager {
    cache: Arc<tokio::sync::RwLock<HashMap<BlissId, BlissIdRecord>>>,
}

impl InMemoryBlissIdManager {
    /// Create new in-memory manager
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        })
    }
}

#[async_trait::async_trait]
impl BlissIdManager for InMemoryBlissIdManager {
    async fn register_blissid(&self, record: BlissIdRecord) -> Result<()> {
        // Validate record
        record.validate()?;
        
        // Check for duplicates
        {
            let cache = self.cache.read().await;
            if cache.contains_key(&record.blissid) {
                return Err(client(
                    AuraFSError::Soul {
                        code: ErrorCode::SoulAlreadyActed,
                        soul_id: Some(record.blissid.clone()),
                        message: "BlissID already registered".to_string(),
                    },
                    ErrorPhase::Identity,
                    ErrorCode::SoulAlreadyActed,
                ));
            }
        }
        
        // Store record
        {
            let mut cache = self.cache.write().await;
            cache.insert(record.blissid.clone(), record);
        }
        
        Ok(())
    }
    
    async fn verify_blissid(&self, blissid: &BlissId, proof: &SoulProof) -> Result<bool> {
        // Validate inputs
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        let cache = self.cache.read().await;
        if let Some(record) = cache.get(blissid) {
            if !record.active {
                return Ok(false);
            }
            
            // Verify proof matches
            if record.proof.commitment != proof.commitment {
                return Ok(false);
            }
            
            // Verify proof cryptographically
            let proof_status = proof.verify().await?;
            Ok(proof_status == ProofStatus::Valid)
        } else {
            Ok(false)
        }
    }
    
    async fn deactivate_blissid(&self, blissid: &BlissId) -> Result<()> {
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        let mut cache = self.cache.write().await;
        if let Some(record) = cache.get_mut(blissid) {
            record.active = false;
            Ok(())
        } else {
            Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "BlissID not found".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ))
        }
    }
    
    async fn is_blissid_active(&self, blissid: &BlissId) -> Result<bool> {
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        let cache = self.cache.read().await;
        Ok(cache.get(blissid).map(|r| r.active).unwrap_or(false))
    }
    
    async fn get_record(&self, blissid: &BlissId) -> Result<Option<BlissIdRecord>> {
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        let cache = self.cache.read().await;
        Ok(cache.get(blissid).cloned())
    }
    
    async fn list_active(&self) -> Result<Vec<BlissId>> {
        let cache = self.cache.read().await;
        Ok(cache.values()
            .filter(|r| r.active)
            .map(|r| r.blissid.clone())
            .collect())
    }
}

/// Production-ready persistent BlissID Manager with crypto verification
pub struct PersistentBlissIdManager {
    /// In-memory cache for fast lookups
    cache: Arc<tokio::sync::RwLock<HashMap<BlissId, BlissIdRecord>>>,
    
    /// Dilithium5 keypair for manager operations
    signing_keypair: Arc<DilithiumKeypair>,
    
    /// Persistence backend (TODO: database)
    _persistence: (),
}

impl PersistentBlissIdManager {
    /// Create production manager with crypto keys
    pub async fn new() -> Result<Arc<Self>> {
        let signing_keypair = Arc::new(DilithiumKeypair::generate()
            .map_err(|e| internal(
                AuraFSError::Crypto {
                    code: ErrorCode::EntropyFailure,
                    message: format!("Failed to generate signing keypair: {}", e),
                },
                ErrorPhase::Crypto,
            ))?);
        
        Ok(Arc::new(Self {
            cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            signing_keypair,
            _persistence: (),
        }))
    }
}

#[async_trait::async_trait]
impl BlissIdManager for PersistentBlissIdManager {
    async fn register_blissid(&self, mut record: BlissIdRecord) -> Result<()> {
        // Validate record structure
        record.validate()?;
        
        // 1. Validate proof structure & crypto
        let proof_status = record.proof.verify().await?;
        if proof_status != ProofStatus::Valid {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::InvalidSoulProof,
                    soul_id: Some(record.blissid.clone()),
                    message: format!("Proof verification failed: {:?}", proof_status),
                },
                ErrorPhase::Identity,
                ErrorCode::InvalidSoulProof,
            ));
        }
        
        // 2. Check for duplicate registration
        {
            let cache = self.cache.read().await;
            if cache.contains_key(&record.blissid) {
                return Err(client(
                    AuraFSError::Soul {
                        code: ErrorCode::SoulAlreadyActed,
                        soul_id: Some(record.blissid.clone()),
                        message: "BlissID already registered".to_string(),
                    },
                    ErrorPhase::Identity,
                    ErrorCode::SoulAlreadyActed,
                ));
            }
        }
        
        // 3. Manager signs the registration for audit trail
        let registration_msg = format!(
            "register:{}:{}", 
            record.blissid, 
            record.registered_at.timestamp()
        );
        let manager_sig = self.signing_keypair.sign(registration_msg.as_bytes())
            .map_err(|e| internal(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidSignature,
                    message: format!("Failed to sign registration: {}", e),
                },
                ErrorPhase::Crypto,
            ))?;
        
        // 4. Store with manager signature
        record.manager_signature = base64::encode(manager_sig);
        {
            let mut cache = self.cache.write().await;
            cache.insert(record.blissid.clone(), record);
        }
        
        // TODO: Persist to database with retry logic
        Ok(())
    }
    
    async fn verify_blissid(&self, blissid: &BlissId, proof: &SoulProof) -> Result<bool> {
        // Validate inputs
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        let cache = self.cache.read().await;
        if let Some(record) = cache.get(blissid) {
            if !record.active {
                return Ok(false);
            }
            
            // Verify provided proof matches stored proof
            if record.proof.commitment != proof.commitment {
                return Ok(false);
            }
            
            // Double-check crypto signature with timeout
            match tokio::time::timeout(
                std::time::Duration::from_secs(5),
                proof.verify()
            ).await {
                Ok(Ok(proof_status)) => {
                    Ok(proof_status == ProofStatus::Valid)
                }
                Ok(Err(e)) => {
                    Err(e)
                }
                Err(_) => {
                    Err(internal(
                        AuraFSError::Crypto {
                            code: ErrorCode::Timeout,
                            message: "Proof verification timeout".to_string(),
                        },
                        ErrorPhase::Crypto,
                    ))
                }
            }
        } else {
            Ok(false)
        }
    }
    
    async fn deactivate_blissid(&self, blissid: &BlissId) -> Result<()> {
        // Validate BlissID
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        let mut cache = self.cache.write().await;
        if let Some(record) = cache.get_mut(blissid) {
            // Sign deactivation for audit
            let deactivation_msg = format!(
                "deactivate:{}:{}", 
                blissid, 
                Utc::now().timestamp()
            );
            let manager_sig = self.signing_keypair.sign(deactivation_msg.as_bytes())
                .map_err(|e| internal(
                    AuraFSError::Crypto {
                        code: ErrorCode::InvalidSignature,
                        message: format!("Failed to sign deactivation: {}", e),
                    },
                    ErrorPhase::Crypto,
                ))?;
            record.manager_signature = base64::encode(manager_sig);
            record.active = false;
            Ok(())
        } else {
            Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "BlissID not found".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ))
        }
    }
    
    async fn is_blissid_active(&self, blissid: &BlissId) -> Result<bool> {
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        let cache = self.cache.read().await;
        Ok(cache.get(blissid).map(|r| r.active).unwrap_or(false))
    }
    
    async fn get_record(&self, blissid: &BlissId) -> Result<Option<BlissIdRecord>> {
        if !blissid.is_valid() {
            return Err(client(
                AuraFSError::Soul {
                    code: ErrorCode::SoulInvalid,
                    soul_id: Some(blissid.clone()),
                    message: "Invalid BlissID format".to_string(),
                },
                ErrorPhase::Identity,
                ErrorCode::SoulInvalid,
            ));
        }
        
        let cache = self.cache.read().await;
        Ok(cache.get(blissid).cloned())
    }
    
    async fn list_active(&self) -> Result<Vec<BlissId>> {
        let cache = self.cache.read().await;
        Ok(cache.values()
            .filter(|r| r.active)
            .map(|r| r.blissid.clone())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blissid_generation() {
        let biometric = b"test_biometric_data";
        let blissid = BlissId::new(biometric);
        assert!(blissid.is_valid());
        assert_eq!(blissid.0.len(), 64);
    }
    
    #[test]
    fn test_blissid_from_hex() {
        let hex = "a".repeat(64);
        let blissid = BlissId::from_hex(&hex).unwrap();
        assert!(blissid.is_valid());
        
        // Invalid hex
        assert!(BlissId::from_hex("invalid").is_err());
    }
}
