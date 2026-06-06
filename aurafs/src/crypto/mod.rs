//! ═══════════════════════════════════════════════════════════════════
//! Cryptography Module - Post-Quantum Security Suite
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! Provides quantum-resistant cryptographic primitives including:
//! - Kyber-1024 Key Encapsulation Mechanism (KEM)
//! - Dilithium-5 Digital Signatures
//! - BLAKE3 Hashing
//! - OPRF (Oblivious Pseudo-Random Functions)
//! - Zero-Knowledge Proofs
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

// ═══════════════════════════════════════════════════════════════════
// SUBMODULES
// ═══════════════════════════════════════════════════════════════════

/// Shard vault and wallet management
pub mod wallet;
/// Post-quantum cryptography (Kyber, Dilithium, Falcon, SPHINCS+)
pub mod pqc;
/// Ledger integration for cryptographic proofs
pub mod ledger;
/// Governance cryptography (voting, consensus signatures)
pub mod gov;
/// Cryptographic primitives (hashing, encoding, RNG)
pub mod primitives;
/// External integrations (Ineffable, Arora, Opulence, SAGES, GVS)
pub mod integrations;

// ═══════════════════════════════════════════════════════════════════
// RE-EXPORTS
// ═══════════════════════════════════════════════════════════════════

// Core primitives
pub use primitives::{
    blake3_hash, sha3_256, sha3_512,
    generate_random_bytes, secure_random_u64,
};

// PQC algorithms
pub use pqc::{
    kyber_keygen, kyber_encapsulate, kyber_decapsulate,
    dilithium_keygen, dilithium_sign, dilithium_verify,
};

// Wallet operations
pub use wallet::{ShardVault, WalletKey, SigningEngine};

// Ledger operations
pub use ledger::{MerkleProof, verify_merkle_proof};

// ═══════════════════════════════════════════════════════════════════
// COMMON TYPES
// ═══════════════════════════════════════════════════════════════════

/// A 32-byte hash
pub type Hash256 = [u8; 32];

/// A 64-byte hash
pub type Hash512 = [u8; 64];

/// Cryptographic signature (variable length for PQC)
pub type Signature = Vec<u8>;

/// Public key (variable length for PQC)
pub type PublicKey = Vec<u8>;

/// Secret key (variable length for PQC)
pub type SecretKey = Vec<u8>;

// ═══════════════════════════════════════════════════════════════════
// CRYPTO RESULT TYPE
// ═══════════════════════════════════════════════════════════════════

/// Crypto error types
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    /// Key generation failed
    #[error("Key generation failed: {0}")]
    KeyGeneration(String),
    /// Signing failed
    #[error("Signing failed: {0}")]
    SigningError(String),
    /// Verification failed
    #[error("Verification failed: {0}")]
    VerificationError(String),
    /// Encryption failed
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    /// Decryption failed
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    /// Invalid key
    #[error("Invalid key: {0}")]
    InvalidKey(String),
    /// Entropy failure
    #[error("Entropy failure: {0}")]
    EntropyError(String),
}

/// Crypto result type
pub type CryptoResult<T> = Result<T, CryptoError>;

// ═══════════════════════════════════════════════════════════════════
// CONVENIENCE FUNCTIONS
// ═══════════════════════════════════════════════════════════════════

use tracing::info;

/// Initialize the crypto subsystem
pub fn init() {
    info!("🔐 Crypto subsystem initialized");
    info!("   - PQC: Kyber-1024, Dilithium-5, Falcon-1024, SPHINCS+");
    info!("   - Hash: BLAKE3, SHA3-256, SHA3-512");
    info!("   - RNG: Hardware-backed CSPRNG");
}

/// Generate a random 32-byte key
pub fn random_key_32() -> Hash256 {
    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key).expect("Failed to generate random key");
    key
}

/// Generate a random 64-byte key
pub fn random_key_64() -> Hash512 {
    let mut key = [0u8; 64];
    getrandom::getrandom(&mut key).expect("Failed to generate random key");
    key
}

/// Secure memory wipe
pub fn secure_wipe(data: &mut [u8]) {
    for byte in data.iter_mut() {
        *byte = 0;
    }
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
}

/// Shard vault initialization (legacy compatibility)
pub async fn shard_vault_empire() {
    info!("💰 SHARD VAULT + Ineffable/Arora/Opulence/SAGES/GVS EMPIRE! 💎");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_key_generation() {
        let key1 = random_key_32();
        let key2 = random_key_32();
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_secure_wipe() {
        let mut data = [1u8, 2, 3, 4, 5];
        secure_wipe(&mut data);
        assert_eq!(data, [0u8; 5]);
    }
}
