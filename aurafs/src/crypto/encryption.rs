//! Encryption and decryption for RAFS
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! Provides AES-256-GCM encryption for shard content with quantum-safe
//! key derivation and nonce management.

use crate::crypto::hash;
use crate::error::{RafsError, Result};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fmt;

/// AES-256-GCM key size (32 bytes)
pub const KEY_SIZE: usize = 32;

/// AES-256-GCM nonce size (12 bytes)
pub const NONCE_SIZE: usize = 12;

/// Encryption key wrapper
#[derive(Clone)]
pub struct EncryptionKey([u8; KEY_SIZE]);

impl EncryptionKey {
    /// Create key from bytes
    pub fn from_bytes(bytes: [u8; KEY_SIZE]) -> Self {
        Self(bytes)
    }

    /// Create key from slice
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != KEY_SIZE {
            return Err(RafsError::CryptoError(format!(
                "Invalid key size: expected {}, got {}",
                KEY_SIZE,
                slice.len()
            )));
        }
        let mut bytes = [0u8; KEY_SIZE];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Generate random key
    pub fn generate() -> Self {
        let mut key = [0u8; KEY_SIZE];
        OsRng.fill_bytes(&mut key);
        Self(key)
    }

    /// Derive key from password and salt using BLAKE3 KDF
    pub fn derive_from_password(password: &[u8], salt: &[u8]) -> Self {
        let context = format!("rafs-encryption-v1:{}", hex::encode(salt));
        let mut key_material = Vec::new();
        key_material.extend_from_slice(password);
        key_material.extend_from_slice(salt);

        let derived = hash::derive_key(&context, &key_material);
        Self(derived)
    }

    /// Derive key from shared secret (for quantum-safe key exchange)
    pub fn derive_from_shared_secret(shared_secret: &[u8]) -> Self {
        let derived = hash::derive_key("rafs-shared-secret-v1", shared_secret);
        Self(derived)
    }

    /// Get key bytes
    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.0
    }

    /// Convert to AES key
    fn to_aes_key(&self) -> Key<Aes256Gcm> {
        *Key::<Aes256Gcm>::from_slice(&self.0)
    }
}

impl fmt::Debug for EncryptionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EncryptionKey([REDACTED])")
    }
}

/// Encrypted data with nonce and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Encrypted ciphertext
    pub ciphertext: Vec<u8>,

    /// Nonce used for encryption (must be unique per key)
    pub nonce: Vec<u8>,

    /// Algorithm identifier
    pub algorithm: String,

    /// Optional key ID (reference to key, not the key itself)
    pub key_id: Option<String>,
}

impl EncryptedData {
    /// Create new encrypted data
    pub fn new(ciphertext: Vec<u8>, nonce: Vec<u8>) -> Self {
        Self {
            ciphertext,
            nonce,
            algorithm: "AES-256-GCM".to_string(),
            key_id: None,
        }
    }

    /// Set key ID
    pub fn with_key_id(mut self, key_id: String) -> Self {
        self.key_id = Some(key_id);
        self
    }

    /// Get total size (ciphertext + nonce + overhead)
    pub fn size(&self) -> usize {
        self.ciphertext.len() + self.nonce.len()
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| RafsError::SerializationError(e.to_string()))
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes)
            .map_err(|e| RafsError::SerializationError(e.to_string()))
    }
}

/// Cipher for encryption and decryption operations
pub struct Cipher {
    key: EncryptionKey,
    cipher: Aes256Gcm,
}

impl Cipher {
    /// Create cipher from key
    pub fn new(key: EncryptionKey) -> Self {
        let aes_key = key.to_aes_key();
        let cipher = Aes256Gcm::new(&aes_key);
        Self { key, cipher }
    }

    /// Create cipher with generated random key
    pub fn with_random_key() -> Self {
        let key = EncryptionKey::generate();
        Self::new(key)
    }

    /// Create cipher from password
    pub fn from_password(password: &[u8], salt: &[u8]) -> Self {
        let key = EncryptionKey::derive_from_password(password, salt);
        Self::new(key)
    }

    /// Create cipher from shared secret (quantum-safe)
    pub fn from_shared_secret(shared_secret: &[u8]) -> Self {
        let key = EncryptionKey::derive_from_shared_secret(shared_secret);
        Self::new(key)
    }

    /// Get encryption key
    pub fn key(&self) -> &EncryptionKey {
        &self.key
    }

    /// Generate random nonce
    pub fn generate_nonce() -> [u8; NONCE_SIZE] {
        let mut nonce = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce);
        nonce
    }

    /// Encrypt data with random nonce
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData> {
        let nonce_bytes = Self::generate_nonce();
        self.encrypt_with_nonce(plaintext, &nonce_bytes)
    }

    /// Encrypt data with specific nonce
    pub fn encrypt_with_nonce(&self, plaintext: &[u8], nonce_bytes: &[u8; NONCE_SIZE]) -> Result<EncryptedData> {
        let nonce = Nonce::from_slice(nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| RafsError::CryptoError(format!("Encryption failed: {}", e)))?;

        Ok(EncryptedData::new(ciphertext, nonce_bytes.to_vec()))
    }

    /// Decrypt data
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        if encrypted.nonce.len() != NONCE_SIZE {
            return Err(RafsError::CryptoError(format!(
                "Invalid nonce size: expected {}, got {}",
                NONCE_SIZE,
                encrypted.nonce.len()
            )));
        }

        let nonce = Nonce::from_slice(&encrypted.nonce);

        let plaintext = self
            .cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| RafsError::CryptoError(format!("Decryption failed: {}", e)))?;

        Ok(plaintext)
    }

    /// Encrypt data and return bytes
    pub fn encrypt_to_bytes(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let encrypted = self.encrypt(plaintext)?;
        encrypted.to_bytes()
    }

    /// Decrypt data from bytes
    pub fn decrypt_from_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        let encrypted = EncryptedData::from_bytes(bytes)?;
        self.decrypt(&encrypted)
    }
}

impl fmt::Debug for Cipher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cipher")
            .field("key", &"[REDACTED]")
            .finish()
    }
}

/// Key derivation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationParams {
    /// Salt for key derivation
    pub salt: Vec<u8>,

    /// Algorithm used for derivation
    pub algorithm: String,

    /// Optional iterations/cost parameter
    pub iterations: Option<u32>,
}

impl KeyDerivationParams {
    /// Create new key derivation params with random salt
    pub fn new() -> Self {
        let mut salt = vec![0u8; 32];
        OsRng.fill_bytes(&mut salt);

        Self {
            salt,
            algorithm: "BLAKE3-KDF".to_string(),
            iterations: None,
        }
    }

    /// Create with specific salt
    pub fn with_salt(salt: Vec<u8>) -> Self {
        Self {
            salt,
            algorithm: "BLAKE3-KDF".to_string(),
            iterations: None,
        }
    }

    /// Derive key from password
    pub fn derive_key(&self, password: &[u8]) -> EncryptionKey {
        EncryptionKey::derive_from_password(password, &self.salt)
    }
}

impl Default for KeyDerivationParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Encryption context with key management
pub struct EncryptionContext {
    cipher: Cipher,
    key_id: String,
    params: Option<KeyDerivationParams>,
}

impl EncryptionContext {
    /// Create context with generated key
    pub fn new(key_id: String) -> Self {
        Self {
            cipher: Cipher::with_random_key(),
            key_id,
            params: None,
        }
    }

    /// Create context from password
    pub fn from_password(key_id: String, password: &[u8], params: KeyDerivationParams) -> Self {
        let cipher = Cipher::from_password(password, &params.salt);
        Self {
            cipher,
            key_id,
            params: Some(params),
        }
    }

    /// Create context from shared secret
    pub fn from_shared_secret(key_id: String, shared_secret: &[u8]) -> Self {
        let cipher = Cipher::from_shared_secret(shared_secret);
        Self {
            cipher,
            key_id,
            params: None,
        }
    }

    /// Get key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Get key derivation params
    pub fn params(&self) -> Option<&KeyDerivationParams> {
        self.params.as_ref()
    }

    /// Encrypt data with key ID attached
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData> {
        let encrypted = self.cipher.encrypt(plaintext)?;
        Ok(encrypted.with_key_id(self.key_id.clone()))
    }

    /// Decrypt data
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        // Verify key ID matches if present
        if let Some(ref key_id) = encrypted.key_id {
            if key_id != &self.key_id {
                return Err(RafsError::CryptoError(
                    "Key ID mismatch".to_string()
                ));
            }
        }

        self.cipher.decrypt(encrypted)
    }
}

impl fmt::Debug for EncryptionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EncryptionContext")
            .field("key_id", &self.key_id)
            .field("params", &self.params)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let key1 = EncryptionKey::generate();
        let key2 = EncryptionKey::generate();

        // Keys should be different
        assert_ne!(key1.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_key_derivation() {
        let password = b"f0rg3d in l0v3";
        let salt = b"rafs-salt-123456";

        let key1 = EncryptionKey::derive_from_password(password, salt);
        let key2 = EncryptionKey::derive_from_password(password, salt);

        // Same password and salt should produce same key
        assert_eq!(key1.as_bytes(), key2.as_bytes());

        // Different salt should produce different key
        let key3 = EncryptionKey::derive_from_password(password, b"different-salt");
        assert_ne!(key1.as_bytes(), key3.as_bytes());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let cipher = Cipher::with_random_key();
        let plaintext = b"It's recursive...";

        // Encrypt
        let encrypted = cipher.encrypt(plaintext).unwrap();
        assert_ne!(encrypted.ciphertext, plaintext);

        // Decrypt
        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_large_data() {
        let cipher = Cipher::with_random_key();
        let plaintext = vec![42u8; 1024 * 1024]; // 1 MB

        let encrypted = cipher.encrypt(&plaintext).unwrap();
        let decrypted = cipher.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_key_fails() {
        let cipher1 = Cipher::with_random_key();
        let cipher2 = Cipher::with_random_key();

        let plaintext = b"secret data";
        let encrypted = cipher1.encrypt(plaintext).unwrap();

        // Decrypting with wrong key should fail
        let result = cipher2.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_nonce_uniqueness() {
        let nonce1 = Cipher::generate_nonce();
        let nonce2 = Cipher::generate_nonce();

        // Nonces should be different (with high probability)
        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn test_encrypted_data_serialization() {
        let cipher = Cipher::with_random_key();
        let plaintext = b"test serialization";

        let encrypted = cipher.encrypt(plaintext).unwrap();

        // Serialize
        let bytes = encrypted.to_bytes().unwrap();

        // Deserialize
        let deserialized = EncryptedData::from_bytes(&bytes).unwrap();

        // Should be able to decrypt
        let decrypted = cipher.decrypt(&deserialized).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_cipher_from_password() {
        let password = b"user-password-123";
        let salt = b"random-salt-value";

        let cipher1 = Cipher::from_password(password, salt);
        let cipher2 = Cipher::from_password(password, salt);

        let plaintext = b"password-encrypted data";

        // Encrypt with cipher1
        let encrypted = cipher1.encrypt(plaintext).unwrap();

        // Decrypt with cipher2 (same password and salt)
        let decrypted = cipher2.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_cipher_from_shared_secret() {
        let shared_secret = b"quantum-safe-shared-secret-key";

        let cipher1 = Cipher::from_shared_secret(shared_secret);
        let cipher2 = Cipher::from_shared_secret(shared_secret);

        let plaintext = b"quantum-encrypted data";

        let encrypted = cipher1.encrypt(plaintext).unwrap();
        let decrypted = cipher2.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encryption_context() {
        let ctx = EncryptionContext::new("key-001".to_string());
        let plaintext = b"context-encrypted data";

        // Encrypt
        let encrypted = ctx.encrypt(plaintext).unwrap();
        assert_eq!(encrypted.key_id, Some("key-001".to_string()));

        // Decrypt
        let decrypted = ctx.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encryption_context_from_password() {
        let password = b"strong-password";
        let params = KeyDerivationParams::new();

        let ctx = EncryptionContext::from_password("key-002".to_string(), password, params);

        let plaintext = b"password-context data";
        let encrypted = ctx.encrypt(plaintext).unwrap();
        let decrypted = ctx.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_key_id_mismatch() {
        let ctx1 = EncryptionContext::new("key-001".to_string());
        let ctx2 = EncryptionContext::new("key-002".to_string());

        let plaintext = b"data";
        let mut encrypted = ctx1.encrypt(plaintext).unwrap();

        // Manually set different key ID
        encrypted.key_id = Some("key-002".to_string());

        // Should fail due to key mismatch (even though key_id matches)
        let result = ctx2.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_key_derivation_params() {
        let params = KeyDerivationParams::new();
        let password = b"my-password";

        let key = params.derive_key(password);
        assert_eq!(key.as_bytes().len(), KEY_SIZE);
    }

    #[test]
    fn test_encrypt_to_bytes() {
        let cipher = Cipher::with_random_key();
        let plaintext = b"direct bytes encryption";

        let encrypted_bytes = cipher.encrypt_to_bytes(plaintext).unwrap();
        let decrypted = cipher.decrypt_from_bytes(&encrypted_bytes).unwrap();

        assert_eq!(decrypted, plaintext);
    }
}