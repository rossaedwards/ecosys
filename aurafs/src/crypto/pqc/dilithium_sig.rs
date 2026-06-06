//! Dilithium-5 signature helpers for AuraFS PQC.
//! 
//! Enforces Theorem 3.1 universality for topological protection.

use crate::error::{RafsError, Result};
use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::{DetachedSignature, PublicKey as _, SecretKey as _};

pub type PublicKey = dilithium5::PublicKey;
pub type PrivateKey = dilithium5::SecretKey;

/// [Theorem 3.1: Universality]
/// Generate a Dilithium-5 keypair for AuraFS signatures.
pub fn dilithium_keygen() -> (PublicKey, PrivateKey) {
    dilithium5::keypair()
}

/// [Theorem 3.1: Universality]
/// Sign a message using Dilithium-5 and return detached signature bytes.
pub fn dilithium_sign(message: &[u8], secret_key: &PrivateKey) -> Result<Vec<u8>> {
    let signature = dilithium5::detached_sign(message, secret_key);
    Ok(signature.as_bytes().to_vec())
}

/// [Theorem 3.1: Universality]
/// Verify a Dilithium-5 detached signature.
pub fn dilithium_verify(message: &[u8], signature: &[u8], public_key: &PublicKey) -> Result<bool> {
    let sig = DetachedSignature::from_bytes(signature)
        .map_err(|e| RafsError::CryptoError(format!("Invalid Dilithium signature: {:?}", e)))?;
    Ok(dilithium5::verify_detached_signature(&sig, message, public_key).is_ok())
}

/// [Theorem 3.1: Universality]
/// Convenience wrapper for signing in subsystems.
pub fn sign(message: &[u8], secret_key: &PrivateKey) -> Result<Vec<u8>> {
    dilithium_sign(message, secret_key)
}

/// [Theorem 3.1: Universality]
/// Convenience wrapper for verification in subsystems.
pub fn verify(message: &[u8], signature: &[u8], public_key: &PublicKey) -> Result<bool> {
    dilithium_verify(message, signature, public_key)
}