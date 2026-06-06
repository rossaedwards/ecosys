//! ═══════════════════════════════════════════════════════════════════
//! 🔐 AuraFS Core Crypto - Quantum-Safe Cryptography & Primitives
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Implements Dilithium5 signatures, SHA3 hashing, post-quantum keygen,
//! quantum random number generation, and secure RNG abstractions.
//! ═══════════════════════════════════════════════════════════════════

use sha3::{Shake256, Sha3_256, Digest};
use pqcrypto_dilithium::dilithium5::*;
use rand_core::{CryptoRng, RngCore};
use thiserror::Error;
use base64::{encode as base64_encode, decode as base64_decode};

use crate::core::{Result, AuraFSError, ErrorPhase, ErrorCode, internal};

/// Cryptographic errors for AuraFS.
#[derive(Debug, Error)]
pub enum CryptoError {
    /// Dilithium5 operation failure.
    #[error("Dilithium5 crypto failure: {0}")]
    DilithiumFailure(String),

    /// Hashing failure.
    #[error("Hash computation failure")]
    HashError,

    /// Base64 decode error.
    #[error("Base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),

    /// Randomness source error.
    #[error("Randomness generation failed")]
    RandomnessFailure,

    /// Generic crypto error.
    #[error("General crypto error: {0}")]
    General(String),
}

/// Keypair for Dilithium5 post-quantum signatures.
#[derive(Debug, Clone)]
pub struct DilithiumKeypair {
    pub pk: PublicKey,
    pub sk: SecretKey,
}

impl DilithiumKeypair {
    /// Generate a new Dilithium5 keypair with validation
    pub fn generate() -> Result<Self> {
        // Generate keypair with retry on entropy failure
        const MAX_RETRIES: usize = 3;
        let mut last_error = None;
        
        for attempt in 0..MAX_RETRIES {
            match keypair() {
                Ok((pk, sk)) => {
                    // Validate keypair sizes
                    if pk.as_bytes().is_empty() || sk.as_bytes().is_empty() {
                        last_error = Some("Generated keypair has empty keys".to_string());
                        if attempt < MAX_RETRIES - 1 {
                            continue;
                        }
                        break;
                    }
                    
                    return Ok(Self { pk, sk });
                }
                Err(e) if attempt < MAX_RETRIES - 1 => {
                    last_error = Some(format!("Dilithium5 keypair gen failed: {:?}", e));
                    // Small delay before retry
                    std::thread::sleep(std::time::Duration::from_millis(10 * (attempt as u64 + 1)));
                    continue;
                }
                Err(e) => {
                    last_error = Some(format!("Dilithium5 keypair gen failed: {:?}", e));
                    break;
                }
            }
        }
        
        Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::EntropyFailure,
                message: format!(
                    "Dilithium5 keypair generation failed after {} attempts: {}",
                    MAX_RETRIES,
                    last_error.unwrap_or_else(|| "Unknown error".to_string())
                ),
            },
            ErrorPhase::Crypto,
        ))
    }

    /// Sign a message with validation
    pub fn sign(&self, msg: &[u8]) -> Result<Vec<u8>> {
        // Validate inputs
        if msg.is_empty() {
            return Err(internal(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: "Cannot sign empty message".to_string(),
                },
                ErrorPhase::Crypto,
            ));
        }
        
        const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB max
        if msg.len() > MAX_MESSAGE_SIZE {
            return Err(internal(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: format!(
                        "Message too large: {} bytes (max {})",
                        msg.len(), MAX_MESSAGE_SIZE
                    ),
                },
                ErrorPhase::Crypto,
            ));
        }
        
        // Validate secret key
        if self.sk.as_bytes().is_empty() {
            return Err(internal(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidSignature,
                    message: "Secret key is empty".to_string(),
                },
                ErrorPhase::Crypto,
            ));
        }
        
        Ok(sign(msg, &self.sk))
    }

    /// Verify a signature with validation
    pub fn verify(&self, msg: &[u8], signature: &[u8]) -> Result<bool> {
        // Validate inputs
        if msg.is_empty() {
            return Err(internal(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidInput,
                    message: "Cannot verify signature for empty message".to_string(),
                },
                ErrorPhase::Crypto,
            ));
        }
        
        if signature.is_empty() {
            return Err(internal(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidSignature,
                    message: "Signature is empty".to_string(),
                },
                ErrorPhase::Crypto,
            ));
        }
        
        // Validate public key
        if self.pk.as_bytes().is_empty() {
            return Err(internal(
                AuraFSError::Crypto {
                    code: ErrorCode::InvalidSignature,
                    message: "Public key is empty".to_string(),
                },
                ErrorPhase::Crypto,
            ));
        }
        
        match verify(msg, signature, &self.pk) {
            Ok(()) => Ok(true),
            Err(_) => {
                Err(internal(
                    AuraFSError::Crypto {
                        code: ErrorCode::InvalidSignature,
                        message: "Dilithium5 signature verification failed".to_string(),
                    },
                    ErrorPhase::Crypto,
                ))
            }
        }
    }

    /// Serialize public key as base64.
    pub fn pk_base64(&self) -> String {
        base64_encode(self.pk.as_bytes())
    }

    /// Serialize secret key as base64.
    pub fn sk_base64(&self) -> String {
        base64_encode(self.sk.as_bytes())
    }
}

/// SHA3-256 hash function wrapper with validation
pub fn sha3_256_digest(data: &[u8]) -> Result<Vec<u8>> {
    // Validate input
    if data.is_empty() {
        return Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::InvalidInput,
                message: "Cannot hash empty data".to_string(),
            },
            ErrorPhase::Crypto,
        ));
    }
    
    const MAX_HASH_SIZE: usize = 100 * 1024 * 1024; // 100MB max
    if data.len() > MAX_HASH_SIZE {
        return Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::InvalidInput,
                message: format!(
                    "Data too large for hashing: {} bytes (max {})",
                    data.len(), MAX_HASH_SIZE
                ),
            },
            ErrorPhase::Crypto,
        ));
    }
    
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    Ok(hasher.finalize().to_vec())
}

/// SHAKE256 extendable-output function for entropy or randomness with validation
pub fn shake256_xof(data: &[u8], output_len: usize) -> Result<Vec<u8>> {
    // Validate inputs
    if data.is_empty() {
        return Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::InvalidInput,
                message: "Cannot generate XOF from empty input".to_string(),
            },
            ErrorPhase::Crypto,
        ));
    }
    
    if output_len == 0 {
        return Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::InvalidInput,
                message: "Cannot generate zero-length XOF output".to_string(),
            },
            ErrorPhase::Crypto,
        ));
    }
    
    const MAX_XOF_SIZE: usize = 10 * 1024 * 1024; // 10MB max
    if output_len > MAX_XOF_SIZE {
        return Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::InvalidInput,
                message: format!(
                    "XOF output length too large: {} bytes (max {})",
                    output_len, MAX_XOF_SIZE
                ),
            },
            ErrorPhase::Crypto,
        ));
    }
    
    let mut shake = Shake256::default();
    shake.update(data);
    let mut out = vec![0u8; output_len];
    shake.finalize_xof().read(&mut out);
    
    // Validate output (should not be all zeros)
    if out.iter().all(|&b| b == 0) {
        return Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::HashError,
                message: "XOF output is all zeros".to_string(),
            },
            ErrorPhase::Crypto,
        ));
    }
    
    Ok(out)
}

/// Quantum-safe random number generator wrapper.
pub struct QuantumRng<R: RngCore + CryptoRng> {
    inner: R,
}

impl<R: RngCore + CryptoRng> QuantumRng<R> {
    pub fn new(rng: R) -> Self {
        Self { inner: rng }
    }

    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest)
    }

    pub fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), CryptoError> {
        self.inner.try_fill_bytes(dest).map_err(|_| CryptoError::RandomnessFailure)
    }
}

/// Generate cryptographically secure random bytes with validation
pub fn gen_random_bytes(len: usize) -> Result<Vec<u8>> {
    // Validate length
    if len == 0 {
        return Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::InvalidInput,
                message: "Cannot generate zero-length random bytes".to_string(),
            },
            ErrorPhase::Crypto,
        ));
    }
    
    const MAX_RANDOM_SIZE: usize = 10 * 1024 * 1024; // 10MB max
    if len > MAX_RANDOM_SIZE {
        return Err(internal(
            AuraFSError::Crypto {
                code: ErrorCode::InvalidInput,
                message: format!(
                    "Requested random size too large: {} bytes (max {})",
                    len, MAX_RANDOM_SIZE
                ),
            },
            ErrorPhase::Crypto,
        ));
    }
    
    let mut buf = vec![0u8; len];
    
    // Retry on entropy failure
    const MAX_RETRIES: usize = 3;
    for attempt in 0..MAX_RETRIES {
        match getrandom::getrandom(&mut buf) {
            Ok(_) => {
                // Validate randomness (check not all zeros)
                if buf.iter().all(|&b| b == 0) {
                    if attempt < MAX_RETRIES - 1 {
                        continue;
                    }
                    return Err(internal(
                        AuraFSError::Crypto {
                            code: ErrorCode::EntropyFailure,
                            message: "Generated random bytes are all zeros".to_string(),
                        },
                        ErrorPhase::Crypto,
                    ));
                }
                
                return Ok(buf);
            }
            Err(e) if attempt < MAX_RETRIES - 1 => {
                // Small delay before retry
                std::thread::sleep(std::time::Duration::from_millis(10 * (attempt as u64 + 1)));
                continue;
            }
            Err(e) => {
                return Err(internal(
                    AuraFSError::Crypto {
                        code: ErrorCode::EntropyFailure,
                        message: format!(
                            "Failed to get secure random bytes after {} attempts: {}",
                            MAX_RETRIES, e
                        ),
                    },
                    ErrorPhase::Crypto,
                ));
            }
        }
    }
    
    unreachable!()
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dilithium_keypair_generate_and_sign_verify() {
        let kp = DilithiumKeypair::generate().expect("Keypair generation failed");
        let message = b"test message for signing";

        let sig = kp.sign(message).expect("Failed to sign");
        let verified = kp.verify(message, &sig).expect("Verification failed");
        assert!(verified);

        // Verification fails for bad message
        let bad_message = b"tampered message";
        assert!(kp.verify(bad_message, &sig).is_err());
    }

    #[test]
    fn test_sha3_256_consistency() {
        let data = b"some useful data";
        let h1 = sha3_256_digest(data);
        let h2 = sha3_256_digest(data);
        assert_eq!(h1, h2);

        // Different data => Different hash
        let h3 = sha3_256_digest(b"some other data");
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_shake256_xof_length() {
        let data = b"entropy input";
        let out1 = shake256_xof(data, 64);
        let out2 = shake256_xof(data, 64);
        assert_eq!(out1, out2);
        assert_eq!(out1.len(), 64);
    }

    #[test]
    fn test_gen_random_bytes() {
        let b1 = gen_random_bytes(32);
        let b2 = gen_random_bytes(32);
        assert_eq!(b1.len(), 32);
        assert_eq!(b2.len(), 32);
        assert_ne!(b1, b2);
    }
}