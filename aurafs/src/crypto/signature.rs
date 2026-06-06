// src/crypto/signature.rs

use anyhow::{Result, anyhow};
use pqcrypto_dilithium::verify; // Dilithium post-quantum signature crate (example)

// Verifies a quantum-safe signature on the given message bytes.
// Returns Ok(true) if valid, Err on cryptographic errors.
pub fn verify_signature(message: &[u8], signature: &[u8]) -> Result<bool> {
    verify(message, signature)
        .map(|_| true)
        .map_err(|e| anyhow!("Signature verification failed: {:?}", e))
}
// Signs a message using a quantum-safe signature scheme.
// Returns the signature bytes or an error.
pub fn quantum_sign(message: &[u8], secret_key: &[u8]) -> Result<Vec<u8>> {
    use pqcrypto_dilithium::sign;

    sign(message, secret_key)
        .map_err(|e| anyhow!("Signature generation failed: {:?}", e))
}
// Verifies a quantum-safe signature on the given message bytes.
// Returns Ok(true) if valid, Err on cryptographic errors.
pub fn verify_signature(message: &[u8], signature: &[u8]) -> Result<bool> {
    verify(message, signature)
        .map(|_| true)
        .map_err(|e| anyhow!("Signature verification failed: {:?}", e))
}
// Signs a message using a quantum-safe signature scheme.
// Returns the signature bytes or an error.