//! SoulSync binding for AuraFS governance.
//!
//! Ensures Aura-Shards bind to SoulSync identity via Dilithium-5.

use crate::crypto::pqc::dilithium_sig;
use crate::error::{RafsError, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// [Theorem 3.1: Universality]
/// Binding record linking a SoulSync identity to a Dilithium-5 signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulBinding {
    pub bliss_id: String,
    pub soul_hash: String,
    pub signature: Vec<u8>,
    pub issued_at: i64,
}

/// [Theorem 3.1: Universality]
/// Create a new SoulBinding for a BlissID + soul hash.
pub fn bind_soul(
    bliss_id: &str,
    soul_hash: &str,
    signing_key: &dilithium_sig::PrivateKey,
) -> Result<SoulBinding> {
    let issued_at = Utc::now().timestamp();
    let message = soul_binding_message(bliss_id, soul_hash);
    let signature = dilithium_sig::sign(&message, signing_key)?;

    Ok(SoulBinding {
        bliss_id: bliss_id.to_string(),
        soul_hash: soul_hash.to_string(),
        signature,
        issued_at,
    })
}

/// [Theorem 3.1: Universality]
/// Verify a SoulBinding against a Dilithium-5 public key.
pub fn verify_binding(
    binding: &SoulBinding,
    public_key: &dilithium_sig::PublicKey,
) -> Result<bool> {
    let message = soul_binding_message(&binding.bliss_id, &binding.soul_hash);
    dilithium_sig::verify(&message, &binding.signature, public_key)
        .map_err(|e| RafsError::CryptoError(format!("SoulBinding verify failed: {:?}", e)))
}

/// [Theorem 3.1: Universality]
/// Canonical SoulBinding message for signing/verification.
fn soul_binding_message(bliss_id: &str, soul_hash: &str) -> Vec<u8> {
    format!("soulsync:{}:{}", bliss_id, soul_hash).into_bytes()
}