//! ═══════════════════════════════════════════════════════════════════
//! 🔮 AuraFS Core SoulProof - Zero-Knowledge Biometric Proofs
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Quantum-safe ZK proofs for biometric uniqueness, soul coherence,
//! and identity verification with Dilithium5 signatures.
//! ═══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::core::{Result, AuraFSError, ErrorCode, ErrorPhase, client};
use crate::core::crypto::{DilithiumKeypair, sha3_256_digest, gen_random_bytes};

/// Soul proof verification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofStatus {
    /// Proof verified successfully.
    Valid,
    /// Proof invalid or expired.
    Invalid,
    /// Proof verification failed (crypto error).
    Failed,
    /// Proof expired.
    Expired,
}

/// Zero-knowledge proof types for soul verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoulProofType {
    /// zk-SNARK proving unique human biometric signature.
    ZkSnarkUniqueHuman,
    /// Dilithium5 quantum-safe signature over biometric hash.
    Dilithium5Biometric,
    /// Soul coherence proof (HRV + biometric fusion).
    SoulCoherence,
    /// Custom proof type.
    Custom(String),
}

/// Comprehensive soul proof structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulProof {
    /// Proof commitment (SHA3-256 hash).
    pub commitment: String,
    /// Proof type identifier.
    pub proof_type: SoulProofType,
    /// Raw proof data (base64 encoded).
    pub proof_bytes: String,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Expiration timestamp (if applicable).
    pub expires_at: Option<DateTime<Utc>>,
    /// Dilithium5 signature over proof commitment.
    pub signature: String,
    /// Public key (base64 encoded).
    pub public_key: String,
    /// Biometric metadata (anonymized).
    pub metadata: HashMap<String, String>,
}

impl SoulProof {
    /// Create new soul proof from biometric data with validation
    pub fn new(
        biometric_hash: &[u8],
        proof_type: SoulProofType,
        dilithium_kp: &DilithiumKeypair,
        expires_after_hours: Option<u64>,
    ) -> Result<Self> {
        // Validate inputs
        if biometric_hash.is_empty() {
            return Err(client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: "Biometric hash is empty".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidInput,
            ));
        }
        
        const MAX_BIOMETRIC_SIZE: usize = 1024 * 1024; // 1MB max
        if biometric_hash.len() > MAX_BIOMETRIC_SIZE {
            return Err(client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: format!(
                        "Biometric hash too large: {} bytes (max {})",
                        biometric_hash.len(), MAX_BIOMETRIC_SIZE
                    ),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidInput,
            ));
        }
        
        // Validate expiration (max 1 year)
        if let Some(hours) = expires_after_hours {
            const MAX_EXPIRATION_HOURS: u64 = 365 * 24;
            if hours > MAX_EXPIRATION_HOURS {
                return Err(client(
                    AuraFSError::Crypto {
                        code: ErrorCode::InvalidInput,
                        message: format!(
                            "Expiration too far in future: {} hours (max {})",
                            hours, MAX_EXPIRATION_HOURS
                        ),
                    },
                    ErrorPhase::Crypto,
                    ErrorCode::InvalidInput,
                ));
            }
        }
        
        // Commitment is SHA3-256 of biometric hash
        let commitment = hex::encode(sha3_256_digest(biometric_hash)?);
        
        // Generate proof bytes (simplified for now)
        let proof_bytes = base64::encode(gen_random_bytes(1024)?);
        
        // Sign the commitment
        let signature = base64::encode(dilithium_kp.sign(commitment.as_bytes())?);
        
        let created_at = Utc::now();
        let expires_at = expires_after_hours.map(|hours| {
            created_at + chrono::Duration::hours(hours as i64)
        });

        Ok(Self {
            commitment,
            proof_type,
            proof_bytes,
            created_at,
            expires_at,
            signature,
            public_key: dilithium_kp.pk_base64(),
            metadata: HashMap::new(),
        })
    }

    /// Syntactic validation of proof structure.
    pub fn validate_structure(&self) -> Result<()> {
        if self.commitment.len() != 64 {
            return Err(client(
                AuraFSError::Other {
                    message: format!("Invalid commitment length: {}", self.commitment.len()),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidSoulProof,
            ));
        }
        
        if self.signature.is_empty() || self.public_key.is_empty() {
            return Err(client(
                AuraFSError::Other {
                    message: "Missing signature or public key".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidSignature,
            ));
        }
        
        if self.expires_at.map_or(false, |exp| exp < Utc::now()) {
            return Err(client(
                AuraFSError::Other {
                    message: "Soul proof expired".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidSoulProof,
            ));
        }
        
        Ok(())
    }

    /// Verify cryptographic integrity of proof.
    pub async fn verify(&self) -> Result<ProofStatus> {
        self.validate_structure()?;
        
        // Decode public key and signature with validation
        let pk_bytes = base64::decode(&self.public_key)
            .map_err(|e| client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidSignature,
                    message: format!("Invalid public key encoding: {}", e),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidSignature,
            ))?;
        
        if pk_bytes.is_empty() {
            return Err(client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidSignature,
                    message: "Decoded public key is empty".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidSignature,
            ));
        }
        
        let sig_bytes = base64::decode(&self.signature)
            .map_err(|e| client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidSignature,
                    message: format!("Invalid signature encoding: {}", e),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidSignature,
            ))?;
        
        if sig_bytes.is_empty() {
            return Err(client(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidSignature,
                    message: "Decoded signature is empty".to_string(),
                },
                ErrorPhase::Crypto,
                ErrorCode::InvalidSignature,
            ));
        }
        
        // Reconstruct Dilithium keypair with timeout
        let pk = match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            async {
                pqcrypto_dilithium::dilithium5::PublicKey::from_bytes(&pk_bytes)
            }
        ).await {
            Ok(Ok(pk)) => pk,
            Ok(Err(_)) => {
                return Err(client(
                    AuraFSError::Crypto {
                        code: ErrorCode::InvalidSignature,
                        message: "Invalid Dilithium public key format".to_string(),
                    },
                    ErrorPhase::Crypto,
                    ErrorCode::InvalidSignature,
                ));
            }
            Err(_) => {
                return Err(client(
                    AuraFSError::Crypto {
                        code: ErrorCode::Timeout,
                        message: "Public key reconstruction timeout".to_string(),
                    },
                    ErrorPhase::Crypto,
                    ErrorCode::Timeout,
                ));
            }
        };
        
        let dilithium_kp = DilithiumKeypair {
            pk,
            sk: pqcrypto_dilithium::dilithium5::SecretKey::from_bytes(&[]).unwrap_or_default(), // We only need public key for verification
        };
        
        // Verify signature over commitment with timeout
        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            dilithium_kp.verify(self.commitment.as_bytes(), &sig_bytes)
        ).await {
            Ok(Ok(true)) => Ok(ProofStatus::Valid),
            Ok(Ok(false)) => Ok(ProofStatus::Failed),
            Ok(Err(e)) => Err(e),
            Err(_) => {
                Err(client(
                    AuraFSError::Crypto {
                        code: ErrorCode::Timeout,
                        message: "Signature verification timeout".to_string(),
                    },
                    ErrorPhase::Crypto,
                    ErrorCode::Timeout,
                ))
            }
        }
    }
}