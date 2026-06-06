//! Kyber-1024 KEM helpers for AuraFS PQC.
//! 
//! Enforces Theorem 3.1 universality for topological protection.

use crate::error::{RafsError, Result};
use pqcrypto_kyber::kyber1024;
use pqcrypto_traits::kem::{Ciphertext as _, PublicKey as _, SecretKey as _, SharedSecret as _};

pub type PublicKey = kyber1024::PublicKey;
pub type SecretKey = kyber1024::SecretKey;
pub type Ciphertext = kyber1024::Ciphertext;
pub type SharedSecret = kyber1024::SharedSecret;

/// [Theorem 3.1: Universality]
/// Generate a Kyber-1024 keypair for inter-node KEM handshakes.
pub fn kyber_keygen() -> (PublicKey, SecretKey) {
    kyber1024::keypair()
}

/// [Theorem 3.1: Universality]
/// Encapsulate a shared secret to a Kyber-1024 public key.
pub fn kyber_encapsulate(public_key: &PublicKey) -> Result<(SharedSecret, Ciphertext)> {
    Ok(kyber1024::encapsulate(public_key))
}

/// [Theorem 3.1: Universality]
/// Decapsulate a Kyber-1024 shared secret.
pub fn kyber_decapsulate(ciphertext: &Ciphertext, secret_key: &SecretKey) -> Result<SharedSecret> {
    Ok(kyber1024::decapsulate(ciphertext, secret_key))
}

/// [Theorem 3.1: Universality]
/// Deserialize a Kyber-1024 public key.
pub fn kyber_public_key_from_bytes(bytes: &[u8]) -> Result<PublicKey> {
    PublicKey::from_bytes(bytes)
        .map_err(|_| RafsError::CryptoError("Invalid Kyber-1024 public key".to_string()))
}

/// [Theorem 3.1: Universality]
/// Deserialize a Kyber-1024 secret key.
pub fn kyber_secret_key_from_bytes(bytes: &[u8]) -> Result<SecretKey> {
    SecretKey::from_bytes(bytes)
        .map_err(|_| RafsError::CryptoError("Invalid Kyber-1024 secret key".to_string()))
}