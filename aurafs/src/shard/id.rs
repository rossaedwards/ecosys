//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Multi-Algorithm ShardId - CID-Style Content Addressing
//! 🔑 BLAKE3, SHA256, SHA3-512 with Self-Describing Format
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

// ═══════════════════════════════════════════════════════════════════
// Hash Algorithm (CID Multicodec Compatible)
// ═══════════════════════════════════════════════════════════════════

/// Hash algorithm identifier (CID multicodec compatible)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum HashAlgorithm {
    /// BLAKE3 - Fast, secure, primary algorithm (multicodec: 0x1e)
    Blake3 = 0x1e,
    /// SHA-256 - IPFS/IPLD compatibility (multicodec: 0x12)
    Sha256 = 0x12,
    /// SHA3-512 - Quantum-resistant, high-security (multicodec: 0x14)
    Sha3_512 = 0x14,
}

impl HashAlgorithm {
    pub fn digest_size(&self) -> usize {
        match self {
            HashAlgorithm::Blake3 => 32,
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha3_512 => 64,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::Blake3 => "blake3",
            HashAlgorithm::Sha256 => "sha256",
            HashAlgorithm::Sha3_512 => "sha3-512",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "blake3" => Some(HashAlgorithm::Blake3),
            "sha256" | "sha2-256" => Some(HashAlgorithm::Sha256),
            "sha3-512" | "sha3_512" => Some(HashAlgorithm::Sha3_512),
            _ => None,
        }
    }

    pub fn from_multicodec(code: u8) -> Option<Self> {
        match code {
            0x1e => Some(HashAlgorithm::Blake3),
            0x12 => Some(HashAlgorithm::Sha256),
            0x14 => Some(HashAlgorithm::Sha3_512),
            _ => None,
        }
    }
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        HashAlgorithm::Blake3
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ═══════════════════════════════════════════════════════════════════
// Shard Layer
// ═══════════════════════════════════════════════════════════════════

/// Layer identifier for the Phase II AuraFS architecture
/// 
/// Each layer corresponds to a specific Chakra/Lattice Geometry:
/// - Storage (Root/Bethe): Physical anchors
/// - Object (Soul Star/Flower): Identity/Containers
/// - Data (Third Eye/Diamond): Encrypted payloads
/// - File (Crown/Sierpinski): Fractal addressing
/// - Network (Sacral/Triangular): Flow dynamics
/// - Compute (Solar Plexus/Kagome): Logic & AI Kernels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum ShardLayer {
    /// Physical storage layer (blocks, disks)
    #[default]
    Storage = 0,
    /// Content-addressable object/blob layer
    Object = 1,
    /// Structured data layer (JSON, Protobuf)
    Data = 2,
    /// VFS file/directory layer
    File = 3,
    /// Mesh network replication layer
    Network = 4,
    /// 🟢 Logic/Compute layer (AI, Smart Contracts)
    Compute = 5,
}

impl ShardLayer {
    /// Get the layer name for CID formatting
    pub fn name(&self) -> &'static str {
        match self {
            ShardLayer::Storage => "storage",
            ShardLayer::Object => "object",
            ShardLayer::Data => "data",
            ShardLayer::File => "file",
            ShardLayer::Network => "network",
            ShardLayer::Compute => "compute",
        }
    }

    /// Parse layer from name string
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "storage" => Some(ShardLayer::Storage),
            "object" => Some(ShardLayer::Object),
            "data" => Some(ShardLayer::Data),
            "file" => Some(ShardLayer::File),
            "network" => Some(ShardLayer::Network),
            "compute" | "kernel" => Some(ShardLayer::Compute),
            _ => None,
        }
    }
}

impl fmt::Display for ShardLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ═══════════════════════════════════════════════════════════════════
// Shard Flags
// ═══════════════════════════════════════════════════════════════════

/// Shard flags for encryption, compression, and erasure coding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ShardFlags {
    /// Shard is encrypted (Kyber-1024 KEM)
    pub encrypted: bool,
    /// Shard is compressed (zstd/lz4)
    pub compressed: bool,
    /// Shard has Reed-Solomon erasure coding
    pub erasure_coded: bool,
    /// Shard is a holographic redundancy fragment
    pub holographic: bool,
}

impl ShardFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bits(bits: u8) -> Self {
        Self {
            encrypted: bits & 0x01 != 0,
            compressed: bits & 0x02 != 0,
            erasure_coded: bits & 0x04 != 0,
            holographic: bits & 0x08 != 0,
        }
    }

    pub fn to_bits(&self) -> u8 {
        let mut bits = 0u8;
        if self.encrypted { bits |= 0x01; }
        if self.compressed { bits |= 0x02; }
        if self.erasure_coded { bits |= 0x04; }
        if self.holographic { bits |= 0x08; }
        bits
    }
}

// ═══════════════════════════════════════════════════════════════════
// Aura Prefix (CID Metadata)
// ═══════════════════════════════════════════════════════════════════

/// AuraFS-specific prefix embedded in CID format
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuraPrefix {
    pub version: u8,
    pub layer: ShardLayer,
    pub flags: ShardFlags,
}

impl Default for AuraPrefix {
    fn default() -> Self {
        Self {
            version: 1,
            layer: ShardLayer::default(),
            flags: ShardFlags::default(),
        }
    }
}

impl AuraPrefix {
    pub fn new(layer: ShardLayer) -> Self {
        Self {
            version: 1,
            layer,
            flags: ShardFlags::default(),
        }
    }

    pub fn with_flags(layer: ShardLayer, flags: ShardFlags) -> Self {
        Self {
            version: 1,
            layer,
            flags,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// ShardId Error
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Error)]
pub enum ShardIdError {
    #[error("Invalid CID format: {0}")]
    InvalidCidFormat(String),
    #[error("Unknown hash algorithm: {0}")]
    UnknownAlgorithm(String),
    #[error("Invalid digest length: expected {expected}, actual {actual}")]
    InvalidDigestLength { expected: usize, actual: usize },
    #[error("Base58 decode error: {0}")]
    Base58DecodeError(String),
    #[error("Unknown layer: {0}")]
    UnknownLayer(String),
}

// ═══════════════════════════════════════════════════════════════════
// ShardIdentifier Trait
// ═══════════════════════════════════════════════════════════════════

pub trait ShardIdentifier: Send + Sync {
    fn algorithm(&self) -> HashAlgorithm;
    fn digest(&self) -> &[u8];
    fn aura_prefix(&self) -> &AuraPrefix;

    fn to_cid(&self) -> String {
        let prefix = self.aura_prefix();
        let digest_b58 = bs58::encode(self.digest()).into_string();
        format!(
            "aura:v{}:{}:{}:{}",
            prefix.version,
            prefix.layer.name(),
            self.algorithm().name(),
            digest_b58
        )
    }

    fn short_id(&self) -> String {
        let bytes = self.digest();
        let len = bytes.len().min(8);
        hex::encode(&bytes[..len])
    }
}

// ═══════════════════════════════════════════════════════════════════
// Algorithm-Specific Types
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Blake3ShardId {
    pub prefix: AuraPrefix,
    pub digest: [u8; 32],
}

impl Blake3ShardId {
    pub fn from_content(content: &[u8], layer: ShardLayer) -> Self {
        let hash = blake3::hash(content);
        Self {
            prefix: AuraPrefix::new(layer),
            digest: *hash.as_bytes(),
        }
    }

    pub fn from_bytes(bytes: &[u8], layer: ShardLayer) -> Result<Self, ShardIdError> {
        if bytes.len() != 32 {
            return Err(ShardIdError::InvalidDigestLength {
                expected: 32,
                actual: bytes.len(),
            });
        }
        let mut digest = [0u8; 32];
        digest.copy_from_slice(bytes);
        Ok(Self {
            prefix: AuraPrefix::new(layer),
            digest,
        })
    }
}

impl ShardIdentifier for Blake3ShardId {
    fn algorithm(&self) -> HashAlgorithm { HashAlgorithm::Blake3 }
    fn digest(&self) -> &[u8] { &self.digest }
    fn aura_prefix(&self) -> &AuraPrefix { &self.prefix }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sha256ShardId {
    pub prefix: AuraPrefix,
    pub digest: [u8; 32],
}

impl Sha256ShardId {
    pub fn from_content(content: &[u8], layer: ShardLayer) -> Self {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        let mut digest = [0u8; 32];
        digest.copy_from_slice(&result);
        Self {
            prefix: AuraPrefix::new(layer),
            digest,
        }
    }
}

impl ShardIdentifier for Sha256ShardId {
    fn algorithm(&self) -> HashAlgorithm { HashAlgorithm::Sha256 }
    fn digest(&self) -> &[u8] { &self.digest }
    fn aura_prefix(&self) -> &AuraPrefix { &self.prefix }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sha3_512ShardId {
    pub prefix: AuraPrefix,
    pub digest: [u8; 64],
}

impl Sha3_512ShardId {
    pub fn from_content(content: &[u8], layer: ShardLayer) -> Self {
        use sha3::{Sha3_512, Digest};
        let mut hasher = Sha3_512::new();
        hasher.update(content);
        let result = hasher.finalize();
        let mut digest = [0u8; 64];
        digest.copy_from_slice(&result);
        Self {
            prefix: AuraPrefix::new(layer),
            digest,
        }
    }
}

impl ShardIdentifier for Sha3_512ShardId {
    fn algorithm(&self) -> HashAlgorithm { HashAlgorithm::Sha3_512 }
    fn digest(&self) -> &[u8] { &self.digest }
    fn aura_prefix(&self) -> &AuraPrefix { &self.prefix }
}

// ═══════════════════════════════════════════════════════════════════
// Unified ShardId Enum
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShardId {
    Blake3(Blake3ShardId),
    Sha256(Sha256ShardId),
    Sha3_512(Sha3_512ShardId),
}

impl ShardId {
    pub fn from_content(content: &[u8]) -> Self {
        Self::from_content_with_layer(content, ShardLayer::default())
    }

    pub fn from_content_with_layer(content: &[u8], layer: ShardLayer) -> Self {
        ShardId::Blake3(Blake3ShardId::from_content(content, layer))
    }

    pub fn from_content_with_algorithm(
        content: &[u8],
        algorithm: HashAlgorithm,
        layer: ShardLayer,
    ) -> Self {
        match algorithm {
            HashAlgorithm::Blake3 => ShardId::Blake3(Blake3ShardId::from_content(content, layer)),
            HashAlgorithm::Sha256 => ShardId::Sha256(Sha256ShardId::from_content(content, layer)),
            HashAlgorithm::Sha3_512 => ShardId::Sha3_512(Sha3_512ShardId::from_content(content, layer)),
        }
    }

    pub fn from_cid(cid: &str) -> Result<Self, ShardIdError> {
        let parts: Vec<&str> = cid.split(':').collect();
        if parts.len() != 5 {
            return Err(ShardIdError::InvalidCidFormat(format!("Expected 5 parts, got {}", parts.len())));
        }
        if parts[0] != "aura" {
            return Err(ShardIdError::InvalidCidFormat(format!("Expected 'aura' prefix, got '{}'", parts[0])));
        }

        let version = parts[1].strip_prefix('v').and_then(|v| v.parse::<u8>().ok())
            .ok_or_else(|| ShardIdError::InvalidCidFormat(format!("Invalid version: {}", parts[1])))?;

        let layer = ShardLayer::from_name(parts[2])
            .ok_or_else(|| ShardIdError::UnknownLayer(parts[2].to_string()))?;

        let algorithm = HashAlgorithm::from_name(parts[3])
            .ok_or_else(|| ShardIdError::UnknownAlgorithm(parts[3].to_string()))?;

        let digest_bytes = bs58::decode(parts[4]).into_vec()
            .map_err(|e| ShardIdError::Base58DecodeError(e.to_string()))?;

        let expected_size = algorithm.digest_size();
        if digest_bytes.len() != expected_size {
            return Err(ShardIdError::InvalidDigestLength { expected: expected_size, actual: digest_bytes.len() });
        }

        let prefix = AuraPrefix { version, layer, flags: ShardFlags::default() };

        match algorithm {
            HashAlgorithm::Blake3 => {
                let mut digest = [0u8; 32];
                digest.copy_from_slice(&digest_bytes);
                Ok(ShardId::Blake3(Blake3ShardId { prefix, digest }))
            }
            HashAlgorithm::Sha256 => {
                let mut digest = [0u8; 32];
                digest.copy_from_slice(&digest_bytes);
                Ok(ShardId::Sha256(Sha256ShardId { prefix, digest }))
            }
            HashAlgorithm::Sha3_512 => {
                let mut digest = [0u8; 64];
                digest.copy_from_slice(&digest_bytes);
                Ok(ShardId::Sha3_512(Sha3_512ShardId { prefix, digest }))
            }
        }
    }

    fn inner(&self) -> &dyn ShardIdentifier {
        match self {
            ShardId::Blake3(id) => id,
            ShardId::Sha256(id) => id,
            ShardId::Sha3_512(id) => id,
        }
    }
}

impl ShardIdentifier for ShardId {
    fn algorithm(&self) -> HashAlgorithm { self.inner().algorithm() }
    fn digest(&self) -> &[u8] { self.inner().digest() }
    fn aura_prefix(&self) -> &AuraPrefix { self.inner().aura_prefix() }
}

impl Default for ShardId {
    fn default() -> Self {
        ShardId::Blake3(Blake3ShardId {
            prefix: AuraPrefix::default(),
            digest: [0u8; 32],
        })
    }
}

impl fmt::Display for ShardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_cid())
    }
}

// ═══════════════════════════════════════════════════════════════════
// Conversions & Tests
// ═══════════════════════════════════════════════════════════════════

impl From<Blake3ShardId> for ShardId { fn from(id: Blake3ShardId) -> Self { ShardId::Blake3(id) } }
impl From<Sha256ShardId> for ShardId { fn from(id: Sha256ShardId) -> Self { ShardId::Sha256(id) } }
impl From<Sha3_512ShardId> for ShardId { fn from(id: Sha3_512ShardId) -> Self { ShardId::Sha3_512(id) } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_layer_cid() {
        let content = b"AI model weights";
        let id = Blake3ShardId::from_content(content, ShardLayer::Compute);
        
        let cid = id.to_cid();
        assert!(cid.starts_with("aura:v1:compute:blake3:"));
        
        let parsed = ShardId::from_cid(&cid).unwrap();
        assert_eq!(parsed.aura_prefix().layer, ShardLayer::Compute);
    }
}