//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! Hybrid Post-Quantum Cryptography
//!
//! Combines CRYSTALS-Dilithium3 (quantum-safe) with RSA (classical)
//! for defense-in-depth against both classical and quantum attacks

use rsa::{RsaPrivateKey, RsaPublicKey, pkcs1v15::SigningKey, pkcs1v15::VerifyingKey};
use rsa::signature::{Signer as RsaSigner, Verifier as RsaVerifier, SignatureEncoding};
use rsa::sha2::Sha256;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

use super::quantum::{QuantumKeyPair, QuantumSigner, QuantumVerifier, QuantumCryptoError};

#[derive(Debug, Error)]
pub enum HybridCryptoError {
    #[error("Quantum crypto error: {0}")]
    QuantumError(#[from] QuantumCryptoError),
    
    #[error("RSA error: {0}")]
    RsaError(String),
    
    #[error("Hybrid verification failed")]
    VerificationFailed,
}

pub type Result<T> = std::result::Result<T, HybridCryptoError>;

/// Hybrid signature (Dilithium3 + RSA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSignature {
    pub quantum_signature: Vec<u8>,
    pub rsa_signature: Vec<u8>,
}

/// Hybrid signer combining quantum-safe and classical cryptography
pub struct HybridSigner {
    quantum_signer: QuantumSigner,
    rsa_private_key: RsaPrivateKey,
}

impl HybridSigner {
    /// Generate new hybrid key pair
    pub fn generate() -> Result<Self> {
        let mut rng = rand::thread_rng();
        
        // Generate quantum-safe keys
        let quantum_keypair = QuantumKeyPair::generate();
        
        // Generate RSA-4096 keys
        let rsa_private = RsaPrivateKey::new(&mut rng, 4096)
            .map_err(|e| HybridCryptoError::RsaError(e.to_string()))?;
        
        info!("Generated hybrid key pair (Dilithium3 + RSA-4096)");
        
        Ok(Self {
            quantum_signer: QuantumSigner::new(quantum_keypair),
            rsa_private_key: rsa_private,
        })
    }
    
    /// Sign message with both quantum and classical schemes
    pub fn sign(&self, message: &[u8]) -> Result<HybridSignature> {
        // Quantum-safe signature
        let quantum_sig = self.quantum_signer.sign(message);
        
        // RSA signature
        let signing_key = SigningKey::<Sha256>::new(self.rsa_private_key.clone());
        let rsa_sig = signing_key.sign(message);
        
        Ok(HybridSignature {
            quantum_signature: quantum_sig,
            rsa_signature: rsa_sig.to_vec(),
        })
    }
    
    pub fn export_public_keys(&self) -> HybridPublicKeys {
        HybridPublicKeys {
            quantum_public_key: self.quantum_signer.keypair.public_key_bytes(),
            rsa_public_key: RsaPublicKey::from(&self.rsa_private_key),
        }
    }
}

/// Hybrid public keys for verification
pub struct HybridPublicKeys {
    pub quantum_public_key: Vec<u8>,
    pub rsa_public_key: RsaPublicKey,
}

impl HybridPublicKeys {
    /// Verify hybrid signature
    pub fn verify(&self, message: &[u8], signature: &HybridSignature) -> Result<()> {
        // Verify quantum signature
        let verifier = QuantumVerifier::from_bytes(&self.quantum_public_key)?;
        verifier.verify(&signature.quantum_signature)?;
        
        // Verify RSA signature
        let verifying_key = VerifyingKey::<Sha256>::new(self.rsa_public_key.clone());
        let rsa_sig = rsa::pkcs1v15::Signature::try_from(signature.rsa_signature.as_slice())
            .map_err(|e| HybridCryptoError::RsaError(e.to_string()))?;
        
        verifying_key.verify(message, &rsa_sig)
            .map_err(|_| HybridCryptoError::VerificationFailed)?;
        
        info!("Hybrid signature verified successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hybrid_sign_verify() {
        let signer = HybridSigner::generate().unwrap();
        let public_keys = signer.export_public_keys();
        
        let message = b"Hybrid quantum + classical signature test";
        let signature = signer.sign(message).unwrap();
        
        public_keys.verify(message, &signature).unwrap();
    }
}