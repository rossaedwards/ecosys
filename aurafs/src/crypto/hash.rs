//! Cryptographic hashing for RAFS
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! Provides quantum-resistant hashing using BLAKE3 for content addressing,
//! shard identification, and integrity verification.
//!! Features:
//! - Hashing data blocks and streams
//! - Keyed hashing for HMAC-like operations
//! - Key derivation using BLAKE3 KDF
//! - Content ID (CID) generation for shards
//! - Hash verification
//!! Uses the `blake3` crate for hashing and `serde` for serialization.
//!
//! # Examples
//! ```rust
//! use rafs::crypto::hash::{hash, Hasher, Hash};
//!! let data = b"Hello, RAFS!";
//! let hash = hash(data);
use blake3;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Hash output size (32 bytes for BLAKE3)
pub const HASH_SIZE: usize = 32;

/// Cryptographic hash wrapper
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash([u8; HASH_SIZE]);

/// BLAKE3 digest wrapper (for compatibility with existing code)
/// Can be constructed from [u8; 32] or Vec<u8>
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Blake3Digest(pub Vec<u8>);

impl From<[u8; 32]> for Blake3Digest {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes.to_vec())
    }
}

impl From<Vec<u8>> for Blake3Digest {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Blake3Digest {
    /// Create from bytes array
    pub fn from_bytes(bytes: [u8; HASH_SIZE]) -> Self {
        Self(bytes.to_vec())
    }
    
    /// Create from Vec<u8>
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    
    /// Create from slice
    pub fn from_slice(slice: &[u8]) -> Result<Self, crate::error::RafsError> {
        Ok(Self(slice.to_vec()))
    }
    
    /// Hash bytes and return Blake3Digest
    pub fn hash_bytes(data: &[u8]) -> Self {
        let hash = hash(data);
        Self(hash.as_bytes().to_vec())
    }
    
    /// Get as bytes slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Convert to Hash
    pub fn to_hash(&self) -> Result<Hash, crate::error::RafsError> {
        Hash::from_slice(&self.0)
    }
}

impl From<Hash> for Blake3Digest {
    fn from(hash: Hash) -> Self {
        Self(hash.as_bytes().to_vec())
    }
}

impl From<Blake3Digest> for Hash {
    fn from(digest: Blake3Digest) -> Self {
        Hash::from_slice(&digest.0).unwrap_or_else(|_| {
            // Fallback: pad or truncate to HASH_SIZE
            let mut bytes = [0u8; HASH_SIZE];
            let len = digest.0.len().min(HASH_SIZE);
            bytes[..len].copy_from_slice(&digest.0[..len]);
            Hash(bytes)
        })
    }
}

impl Hash {
    /// Create hash from byte array
    pub fn from_bytes(bytes: [u8; HASH_SIZE]) -> Self {
        Self(bytes)
    }

    /// Create hash from slice (panics if wrong size)
    pub fn from_slice(slice: &[u8]) -> Result<Self, crate::error::RafsError> {
        if slice.len() != HASH_SIZE {
            return Err(crate::error::RafsError::CryptoError(
                format!("Invalid hash size: expected {}, got {}", HASH_SIZE, slice.len())
            ));
        }
        let mut bytes = [0u8; HASH_SIZE];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Get hash as byte slice
    pub fn as_bytes(&self) -> &[u8; HASH_SIZE] {
        &self.0
    }

    /// Get hash as hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Create hash from hex string
    pub fn from_hex(hex: &str) -> Result<Self, crate::error::RafsError> {
        let bytes = hex::decode(hex)
            .map_err(|e| crate::error::RafsError::CryptoError(format!("Invalid hex: {}", e)))?;
        Self::from_slice(&bytes)
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({})", self.to_hex())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Content-addressable hasher using BLAKE3
pub struct Hasher {
    inner: blake3::Hasher,
}

impl Hasher {
    /// Create new hasher
    pub fn new() -> Self {
        Self {
            inner: blake3::Hasher::new(),
        }
    }

    /// Update hasher with data
    pub fn update(&mut self, data: &[u8]) -> &mut Self {
        self.inner.update(data);
        self
    }

    /// Finalize and return hash
    pub fn finalize(&self) -> Hash {
        let hash = self.inner.finalize();
        Hash(hash.into())
    }

    /// Reset hasher for reuse
    pub fn reset(&mut self) {
        self.inner = blake3::Hasher::new();
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Hash a single block of data
pub fn hash(data: &[u8]) -> Hash {
    let hash = blake3::hash(data);
    Hash(hash.into())
}

/// Hash multiple blocks of data
pub fn hash_chunks(chunks: &[&[u8]]) -> Hash {
    let mut hasher = Hasher::new();
    for chunk in chunks {
        hasher.update(chunk);
    }
    hasher.finalize()
}

/// Keyed hash for HMAC-like operations
pub fn keyed_hash(key: &[u8; 32], data: &[u8]) -> Hash {
    let hash = blake3::keyed_hash(key, data);
    Hash(hash.into())
}

/// Derive key from password using BLAKE3 KDF
pub fn derive_key(context: &str, key_material: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key(context);
    hasher.update(key_material);
    let hash = hasher.finalize();
    hash.into()
}

/// Content ID (CID) generator for shards
pub fn generate_shard_cid(data: &[u8]) -> String {
    let hash = hash(data);
    format!("rafs:{}", hash.to_hex())
}

/// Verify data matches expected hash
pub fn verify_hash(data: &[u8], expected: &Hash) -> bool {
    let computed = hash(data);
    computed == *expected
}

/// Hash bytes and return Blake3Digest (helper function)
pub fn blake3_hash_bytes(data: &[u8]) -> Blake3Digest {
    hash(data)
}

impl Blake3Digest {
    /// Hash bytes and return Blake3Digest
    pub fn hash_bytes(data: &[u8]) -> Self {
        hash(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_creation() {
        let data = b"f0rg3d in l0v3";
        let hash = hash(data);
        assert_eq!(hash.as_bytes().len(), HASH_SIZE);
    }

    #[test]
    fn test_hash_deterministic() {
        let data = b"It's recursive...";
        let hash1 = hash(data);
        let hash2 = hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_hex_roundtrip() {
        let data = b"Aurphyx";
        let hash1 = hash(data);
        let hex = hash1.to_hex();
        let hash2 = Hash::from_hex(&hex).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hasher_incremental() {
        let mut hasher = Hasher::new();
        hasher.update(b"Hello, ");
        hasher.update(b"World!");
        let hash1 = hasher.finalize();

        let hash2 = hash(b"Hello, World!");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_verify_hash() {
        let data = b"test data";
        let hash = hash(data);
        assert!(verify_hash(data, &hash));
        assert!(!verify_hash(b"wrong data", &hash));
    }

    #[test]
    fn test_generate_shard_cid() {
        let data = b"shard content";
        let cid = generate_shard_cid(data);
        assert!(cid.starts_with("rafs:"));
        assert_eq!(cid.len(), 5 + (HASH_SIZE * 2)); // "rafs:" + hex
    }

    #[test]
    fn test_keyed_hash() {
        let key = [0u8; 32];
        let data = b"message";
        let hash1 = keyed_hash(&key, data);
        let hash2 = keyed_hash(&key, data);
        assert_eq!(hash1, hash2);

        // Different key produces different hash
        let key2 = [1u8; 32];
        let hash3 = keyed_hash(&key2, data);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_derive_key() {
        let context = "rafs-encryption";
        let material = b"user-password-123";
        let key1 = derive_key(context, material);
        let key2 = derive_key(context, material);
        assert_eq!(key1, key2);

        // Different context produces different key
        let key3 = derive_key("different-context", material);
        assert_ne!(key1, key3);
    }
}
