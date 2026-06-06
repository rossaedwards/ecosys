//! Quantum-safe cryptography for RAFS
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! Implements post-quantum cryptography using Kyber (key encapsulation) and
//! Dilithium (digital signatures) for quantum-resistant security.

use crate::error::{RafsError, Result};
use pqcrypto_dilithium::dilithium5;
use pqcrypto_kyber::kyber1024;
use pqcrypto_traits::kem::{Ciphertext as KemCiphertext, PublicKey as KemPublicKey, SecretKey as KemSecretKey, SharedSecret};
use pqcrypto_traits::sign::{PublicKey as SignPublicKey, SecretKey as SignSecretKey, SignedMessage, DetachedSignature};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Kyber key pair for key encapsulation
#[derive(Clone)]
pub struct KyberKeyPair {
    pub public_key: kyber1024::PublicKey,
    pub secret_key: kyber1024::SecretKey,
}

impl KyberKeyPair {
    /// Generate new Kyber key pair
    pub fn generate() -> Self {
        let (public_key, secret_key) = kyber1024::keypair();
        Self {
            public_key,
            secret_key,
        }
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.as_bytes().to_vec()
    }

    /// Get secret key bytes
    pub fn secret_key_bytes(&self) -> Vec<u8> {
        self.secret_key.as_bytes().to_vec()
    }

    /// Create from public key bytes
    pub fn from_public_key_bytes(bytes: &[u8]) -> Result<kyber1024::PublicKey> {
        kyber1024::PublicKey::from_bytes(bytes)
            .map_err(|_| RafsError::CryptoError("Invalid Kyber public key".to_string()))
    }

    /// Create from secret key bytes
    pub fn from_secret_key_bytes(bytes: &[u8]) -> Result<kyber1024::SecretKey> {
        kyber1024::SecretKey::from_bytes(bytes)
            .map_err(|_| RafsError::CryptoError("Invalid Kyber secret key".to_string()))
    }

    /// Encapsulate shared secret (sender side)
    pub fn encapsulate(public_key: &kyber1024::PublicKey) -> (kyber1024::SharedSecret, kyber1024::Ciphertext) {
        kyber1024::encapsulate(public_key)
    }

    /// Decapsulate shared secret (receiver side)
    pub fn decapsulate(&self, ciphertext: &kyber1024::Ciphertext) -> kyber1024::SharedSecret {
        kyber1024::decapsulate(ciphertext, &self.secret_key)
    }
}

impl fmt::Debug for KyberKeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KyberKeyPair")
            .field("public_key_len", &self.public_key.as_bytes().len())
            .field("secret_key_len", &self.secret_key.as_bytes().len())
            .finish()
    }
}

/// Dilithium key pair for digital signatures
#[derive(Clone)]
pub struct DilithiumKeyPair {
    pub public_key: dilithium5::PublicKey,
    pub secret_key: dilithium5::SecretKey,
}

impl DilithiumKeyPair {
    /// Generate new Dilithium key pair
    pub fn generate() -> Self {
        let (public_key, secret_key) = dilithium5::keypair();
        Self {
            public_key,
            secret_key,
        }
    }

    /// Get public key bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.as_bytes().to_vec()
    }

    /// Get secret key bytes
    pub fn secret_key_bytes(&self) -> Vec<u8> {
        self.secret_key.as_bytes().to_vec()
    }

    /// Create from public key bytes
    pub fn from_public_key_bytes(bytes: &[u8]) -> Result<dilithium5::PublicKey> {
        dilithium5::PublicKey::from_bytes(bytes)
            .map_err(|_| RafsError::CryptoError("Invalid Dilithium public key".to_string()))
    }

    /// Create from secret key bytes
    pub fn from_secret_key_bytes(bytes: &[u8]) -> Result<dilithium5::SecretKey> {
        dilithium5::SecretKey::from_bytes(bytes)
            .map_err(|_| RafsError::CryptoError("Invalid Dilithium secret key".to_string()))
    }

    /// Sign message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        dilithium5::sign(message, &self.secret_key).as_bytes().to_vec()
    }

    /// Sign message (detached signature)
    pub fn sign_detached(&self, message: &[u8]) -> Vec<u8> {
        dilithium5::detached_sign(message, &self.secret_key).as_bytes().to_vec()
    }

    /// Verify signed message
    pub fn verify(signed_message: &[u8], public_key: &dilithium5::PublicKey) -> Result<Vec<u8>> {
        let signed_msg = dilithium5::SignedMessage::from_bytes(signed_message)
            .map_err(|_| RafsError::CryptoError("Invalid signed message".to_string()))?;

        dilithium5::open(&signed_msg, public_key)
            .map(|msg| msg.to_vec())
            .map_err(|_| RafsError::CryptoError("Signature verification failed".to_string()))
    }

    /// Verify detached signature
    pub fn verify_detached(
        message: &[u8],
        signature: &[u8],
        public_key: &dilithium5::PublicKey,
    ) -> Result<bool> {
        let sig = dilithium5::DetachedSignature::from_bytes(signature)
            .map_err(|_| RafsError::CryptoError("Invalid signature".to_string()))?;

        dilithium5::verify_detached_signature(&sig, message, public_key)
            .map(|_| true)
            .map_err(|_| RafsError::CryptoError("Signature verification failed".to_string()))
    }
}

impl fmt::Debug for DilithiumKeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DilithiumKeyPair")
            .field("public_key_len", &self.public_key.as_bytes().len())
            .field("secret_key_len", &self.secret_key.as_bytes().len())
            .finish()
    }
}

/// Complete quantum-safe key pair (KEM + Signature)
#[derive(Debug, Clone)]
pub struct QuantumKeyPair {
    pub kyber: KyberKeyPair,
    pub dilithium: DilithiumKeyPair,
}

impl QuantumKeyPair {
    /// Generate new quantum-safe key pair
    pub fn generate() -> Self {
        Self {
            kyber: KyberKeyPair::generate(),
            dilithium: DilithiumKeyPair::generate(),
        }
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> QuantumKeyPairBytes {
        QuantumKeyPairBytes {
            kyber_public: self.kyber.public_key_bytes(),
            kyber_secret: self.kyber.secret_key_bytes(),
            dilithium_public: self.dilithium.public_key_bytes(),
            dilithium_secret: self.dilithium.secret_key_bytes(),
        }
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &QuantumKeyPairBytes) -> Result<Self> {
        let kyber_public = KyberKeyPair::from_public_key_bytes(&bytes.kyber_public)?;
        let kyber_secret = KyberKeyPair::from_secret_key_bytes(&bytes.kyber_secret)?;
        let dilithium_public = DilithiumKeyPair::from_public_key_bytes(&bytes.dilithium_public)?;
        let dilithium_secret = DilithiumKeyPair::from_secret_key_bytes(&bytes.dilithium_secret)?;

        Ok(Self {
            kyber: KyberKeyPair {
                public_key: kyber_public,
                secret_key: kyber_secret,
            },
            dilithium: DilithiumKeyPair {
                public_key: dilithium_public,
                secret_key: dilithium_secret,
            },
        })
    }
}

/// Serializable quantum key pair bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumKeyPairBytes {
    pub kyber_public: Vec<u8>,
    pub kyber_secret: Vec<u8>,
    pub dilithium_public: Vec<u8>,
    pub dilithium_secret: Vec<u8>,
}

/// Public keys for verification and encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumPublicKeys {
    pub kyber_public: Vec<u8>,
    pub dilithium_public: Vec<u8>,
}

impl QuantumPublicKeys {
    /// Create from key pair
    pub fn from_keypair(keypair: &QuantumKeyPair) -> Self {
        Self {
            kyber_public: keypair.kyber.public_key_bytes(),
            dilithium_public: keypair.dilithium.public_key_bytes(),
        }
    }

    /// Get Kyber public key
    pub fn kyber_public_key(&self) -> Result<kyber1024::PublicKey> {
        KyberKeyPair::from_public_key_bytes(&self.kyber_public)
    }

    /// Get Dilithium public key
    pub fn dilithium_public_key(&self) -> Result<dilithium5::PublicKey> {
        DilithiumKeyPair::from_public_key_bytes(&self.dilithium_public)
    }
}

/// Encapsulated shared secret with ciphertext
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncapsulatedSecret {
    pub ciphertext: Vec<u8>,
    pub shared_secret: Vec<u8>,
}

/// Quantum-safe crypto operations
pub struct QuantumCrypto;

impl QuantumCrypto {
    /// Generate new quantum-safe key pair
    pub fn generate_keypair() -> QuantumKeyPair {
        QuantumKeyPair::generate()
    }

    /// Encapsulate shared secret for key exchange
    pub fn encapsulate(public_keys: &QuantumPublicKeys) -> Result<EncapsulatedSecret> {
        let kyber_public = public_keys.kyber_public_key()?;
        let (shared_secret, ciphertext) = KyberKeyPair::encapsulate(&kyber_public);

        Ok(EncapsulatedSecret {
            ciphertext: ciphertext.as_bytes().to_vec(),
            shared_secret: shared_secret.as_bytes().to_vec(),
        })
    }

    /// Decapsulate shared secret
    pub fn decapsulate(keypair: &QuantumKeyPair, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let ct = kyber1024::Ciphertext::from_bytes(ciphertext)
            .map_err(|_| RafsError::CryptoError("Invalid ciphertext".to_string()))?;

        let shared_secret = keypair.kyber.decapsulate(&ct);
        Ok(shared_secret.as_bytes().to_vec())
    }

    /// Sign data
    pub fn sign(keypair: &QuantumKeyPair, data: &[u8]) -> Vec<u8> {
        keypair.dilithium.sign(data)
    }

    /// Sign data (detached signature)
    pub fn sign_detached(keypair: &QuantumKeyPair, data: &[u8]) -> Vec<u8> {
        keypair.dilithium.sign_detached(data)
    }

    /// Verify signed data
    pub fn verify(signed_data: &[u8], public_keys: &QuantumPublicKeys) -> Result<Vec<u8>> {
        let dilithium_public = public_keys.dilithium_public_key()?;
        DilithiumKeyPair::verify(signed_data, &dilithium_public)
    }

    /// Verify detached signature
    pub fn verify_detached(
        data: &[u8],
        signature: &[u8],
        public_keys: &QuantumPublicKeys,
    ) -> Result<bool> {
        let dilithium_public = public_keys.dilithium_public_key()?;
        DilithiumKeyPair::verify_detached(data, signature, &dilithium_public)
    }
}

/// Shard signature for authenticity verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardSignature {
    /// Detached signature
    pub signature: Vec<u8>,

    /// Signer's public keys
    pub signer_public_keys: QuantumPublicKeys,

    /// Timestamp of signature
    pub timestamp: i64,
}

impl ShardSignature {
    /// Create new shard signature
    pub fn new(data: &[u8], keypair: &QuantumKeyPair) -> Self {
        let signature = QuantumCrypto::sign_detached(keypair, data);
        let signer_public_keys = QuantumPublicKeys::from_keypair(keypair);
        let timestamp = chrono::Utc::now().timestamp();

        Self {
            signature,
            signer_public_keys,
            timestamp,
        }
    }

    /// Verify signature
    pub fn verify(&self, data: &[u8]) -> Result<bool> {
        QuantumCrypto::verify_detached(data, &self.signature, &self.signer_public_keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_keypair_generation() {
        let keypair = KyberKeyPair::generate();
        
        assert!(keypair.public_key_bytes().len() > 0);
        assert!(keypair.secret_key_bytes().len() > 0);
    }

    #[test]
    fn test_kyber_key_encapsulation() {
        let keypair = KyberKeyPair::generate();
        
        // Encapsulate
        let (shared_secret1, ciphertext) = KyberKeyPair::encapsulate(&keypair.public_key);
        
        // Decapsulate
        let shared_secret2 = keypair.decapsulate(&ciphertext);
        
        // Shared secrets should match
        assert_eq!(shared_secret1.as_bytes(), shared_secret2.as_bytes());
    }

    #[test]
    fn test_dilithium_keypair_generation() {
        let keypair = DilithiumKeyPair::generate();
        
        assert!(keypair.public_key_bytes().len() > 0);
        assert!(keypair.secret_key_bytes().len() > 0);
    }

    #[test]
    fn test_dilithium_sign_verify() {
        let keypair = DilithiumKeyPair::generate();
        let message = b"f0rg3d in l0v3 - It's recursive...";

        // Sign
        let signed_message = keypair.sign(message);

        // Verify
        let verified_message = DilithiumKeyPair::verify(&signed_message, &keypair.public_key).unwrap();
        assert_eq!(verified_message, message);
    }

    #[test]
    fn test_dilithium_sign_verify_detached() {
        let keypair = DilithiumKeyPair::generate();
        let message = b"Aurphyx quantum-safe signature";

        // Sign detached
        let signature = keypair.sign_detached(message);

        // Verify detached
        let valid = DilithiumKeyPair::verify_detached(message, &signature, &keypair.public_key).unwrap();
        assert!(valid);

        // Verify with wrong message should fail
        let result = DilithiumKeyPair::verify_detached(b"wrong message", &signature, &keypair.public_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_quantum_keypair_generation() {
        let keypair = QuantumKeyPair::generate();
        
        assert!(keypair.kyber.public_key_bytes().len() > 0);
        assert!(keypair.dilithium.public_key_bytes().len() > 0);
    }

    #[test]
    fn test_quantum_keypair_serialization() {
        let keypair = QuantumKeyPair::generate();
        
        // Serialize
        let bytes = keypair.to_bytes();
        
        // Deserialize
        let restored = QuantumKeyPair::from_bytes(&bytes).unwrap();
        
        // Keys should match
        assert_eq!(
            keypair.kyber.public_key_bytes(),
            restored.kyber.public_key_bytes()
        );
        assert_eq!(
            keypair.dilithium.public_key_bytes(),
            restored.dilithium.public_key_bytes()
        );
    }

    #[test]
    fn test_quantum_crypto_encapsulate_decapsulate() {
        let keypair = QuantumCrypto::generate_keypair();
        let public_keys = QuantumPublicKeys::from_keypair(&keypair);

        // Encapsulate
        let encapsulated = QuantumCrypto::encapsulate(&public_keys).unwrap();

        // Decapsulate
        let decapsulated = QuantumCrypto::decapsulate(&keypair, &encapsulated.ciphertext).unwrap();

        // Shared secrets should match
        assert_eq!(encapsulated.shared_secret, decapsulated);
    }

    #[test]
    fn test_quantum_crypto_sign_verify() {
        let keypair = QuantumCrypto::generate_keypair();
        let public_keys = QuantumPublicKeys::from_keypair(&keypair);
        let data = b"RAFS quantum-safe data";

        // Sign
        let signed = QuantumCrypto::sign(&keypair, data);

        // Verify
        let verified = QuantumCrypto::verify(&signed, &public_keys).unwrap();
        assert_eq!(verified, data);
    }

    #[test]
    fn test_quantum_crypto_sign_verify_detached() {
        let keypair = QuantumCrypto::generate_keypair();
        let public_keys = QuantumPublicKeys::from_keypair(&keypair);
        let data = b"RAFS shard data for signing";

        // Sign detached
        let signature = QuantumCrypto::sign_detached(&keypair, data);

        // Verify detached
        let valid = QuantumCrypto::verify_detached(data, &signature, &public_keys).unwrap();
        assert!(valid);
    }

    #[test]
    fn test_shard_signature() {
        let keypair = QuantumCrypto::generate_keypair();
        let data = b"Shard content to be signed";

        // Create signature
        let shard_sig = ShardSignature::new(data, &keypair);

        // Verify signature
        let valid = shard_sig.verify(data).unwrap();
        assert!(valid);

        // Verify with wrong data should fail
        let result = shard_sig.verify(b"wrong data");
        assert!(result.is_err());
    }

    #[test]
    fn test_public_keys_extraction() {
        let keypair = QuantumCrypto::generate_keypair();
        let public_keys = QuantumPublicKeys::from_keypair(&keypair);

        // Extract and compare
        let kyber_public = public_keys.kyber_public_key().unwrap();
        let dilithium_public = public_keys.dilithium_public_key().unwrap();

        assert_eq!(kyber_public.as_bytes(), keypair.kyber.public_key.as_bytes());
        assert_eq!(dilithium_public.as_bytes(), keypair.dilithium.public_key.as_bytes());
    }
}

// Type aliases and compatibility types for existing codebase
/// Combined quantum keypair (Kyber + Dilithium) for compatibility
#[derive(Debug, Clone)]
pub struct KyberKeypair {
    pub kyber: KyberKeyPair,
    pub dilithium: DilithiumKeyPair,
}

impl KyberKeypair {
    /// Generate new combined quantum keypair
    pub fn generate() -> Self {
        Self {
            kyber: KyberKeyPair::generate(),
            dilithium: DilithiumKeyPair::generate(),
        }
    }
    
    /// Access private key (Dilithium secret key) for signing
    pub fn private_key(&self) -> &dilithium5::SecretKey {
        &self.dilithium.secret_key
    }
    
    /// Access public key (Dilithium public key) for verification
    pub fn public_key(&self) -> &dilithium5::PublicKey {
        &self.dilithium.public_key
    }
}

impl From<QuantumKeyPair> for KyberKeypair {
    fn from(qkp: QuantumKeyPair) -> Self {
        Self {
            kyber: qkp.kyber,
            dilithium: qkp.dilithium,
        }
    }
}

/// [Theorem 3.1: Universality]
/// Legacy Kyber768 keypair interface backed by Kyber-1024.
pub struct Kyber768Keypair {
    inner: KyberKeyPair,
}

impl Kyber768Keypair {
    pub fn generate<T>(_rng: &T) -> Result<Self> {
        Ok(Self {
            inner: KyberKeyPair::generate(),
        })
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.inner.public_key_bytes()
    }

    pub fn encapsulate(&self, peer_public_key: &[u8]) -> Vec<u8> {
        if let Ok(pk) = KyberKeyPair::from_public_key_bytes(peer_public_key) {
            let (_shared, ct) = KyberKeyPair::encapsulate(&pk);
            ct.as_bytes().to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn decapsulate(&self, ciphertext: &[u8]) -> Vec<u8> {
        if let Ok(ct) = kyber1024::Ciphertext::from_bytes(ciphertext) {
            self.inner.decapsulate(&ct).as_bytes().to_vec()
        } else {
            Vec::new()
        }
    }
}

/// [Theorem 3.1: Universality]
/// Legacy Dilithium2 keypair interface backed by Dilithium-5.
pub struct Dilithium2Keypair {
    inner: DilithiumKeyPair,
}

impl Dilithium2Keypair {
    pub fn generate<T>(_rng: &T) -> Result<Self> {
        Ok(Self {
            inner: DilithiumKeyPair::generate(),
        })
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.inner.public_key_bytes()
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.inner.sign_detached(message)
    }
}

/// [Theorem 3.1: Universality]
/// Legacy Dilithium2 signature verifier backed by Dilithium-5.
pub struct Dilithium2Signature;

impl Dilithium2Signature {
    pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<()> {
        let pk = DilithiumKeyPair::from_public_key_bytes(public_key)?;
        match DilithiumKeyPair::verify_detached(message, signature, &pk)? {
            true => Ok(()),
            false => Err(RafsError::CryptoError("Signature verification failed".to_string())),
        }
    }
}

/// Dilithium signature wrapper for shard signing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DilithiumSignature(pub Vec<u8>);

impl DilithiumSignature {
    /// Create new signature from bytes
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    
    /// Get signature bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Helper function to sign data with Dilithium (for compatibility)
pub fn dilithium_sign(data: &[u8], secret_key: &dilithium5::SecretKey) -> Result<Vec<u8>> {
    let signature = dilithium5::detached_sign(data, secret_key);
    Ok(signature.as_bytes().to_vec())
}