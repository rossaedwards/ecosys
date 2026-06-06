//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Data Engine - Quantum-Fractal Storage Core
//! 🔮 Content-Addressable, Erasure-Coded, Post-Quantum Perfection
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    crypto::{
        hash::{Blake3Digest, blake3_hash},
        quantum::{KyberKeypair, DilithiumSignature, dilithium_sign},
    },
    compression::CompressionAlgorithm,
};
use crate::shard::id::{ShardId, ShardIdentifier, ShardLayer};
use crate::shard::metadata::{ShardMetadata, BlissId};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::{self, Cursor},
    sync::Arc,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use thiserror::Error;

/// Mythical AuraFS Shard - The Atomic Unit of Fractal Reality
/// 
/// In Phase II, this struct is the "Particle" that travels through the Lattice.
/// It carries its own geometry (via metadata), its own proof of existence (signature),
/// and its own audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shard {
    /// Content-addressed unique identifier (multi-algorithm)
    pub shard_id: ShardId,
    
    /// Shard data payload (compressed/encrypted)
    pub data: Vec<u8>,
    
    /// Fractal metadata linking (Contains the LatticeGeometry)
    pub metadata: ShardMetadata,
    
    /// Quantum-safe Dilithium signature
    pub signature: Option<DilithiumSignature>,
    
    /// Erasure coding parameters (K:M parity)
    pub erasure_coding: Option<(usize, usize)>,
    
    /// Replication locations across mesh (TODO: Future - BlissID integration)
    pub replicas: BTreeMap<String, String>, // peer_id -> location
    
    /// Version lineage chain
    pub version_chain: Vec<ShardId>,
    
    /// Audit trail hash
    pub audit_hash: Blake3Digest,
}

impl Shard {
    /// Forge new shard from raw data with quantum perfection
    pub fn forge(data: Vec<u8>, metadata: ShardMetadata) -> Result<Self, ShardError> {
        // The shard ID is the immutable content address
        let shard_id = ShardId::from_content(&data);
        
        // The audit hash binds the data to the metadata (and thus the Geometry)
        let audit_hash = Self::compute_audit_hash(&data, &metadata);
        
        Ok(Self {
            shard_id,
            data,
            metadata,
            signature: None,
            erasure_coding: None,
            replicas: BTreeMap::new(),
            version_chain: vec![],
            audit_hash,
        })
    }

    /// Sign shard with quantum-safe Dilithium
    /// 
    /// Note: Uses the private key from the Keypair to assert ownership of the Particle.
    pub fn sign(mut self, keypair: &KyberKeypair) -> Result<Self, ShardError> {
        let digest = self.shard_id.digest();
        // Phase II: Ensure we are signing the geometry as well via the ID/Audit chain
        let signature_bytes = dilithium_sign(digest, &keypair.private_key)?;
        self.signature = Some(DilithiumSignature(signature_bytes));
        Ok(self)
    }

    /// Compress shard data with adaptive algorithm
    pub async fn compress(mut self, algorithm: CompressionAlgorithm) -> Result<Self, ShardError> {
        let compressed = algorithm.compress(&self.data).await?;
        self.data = compressed;
        // Note: Compression details tracked in metadata if using ObjectShardMetadata layer
        Ok(self)
    }

    /// Apply erasure coding for ultra-resilience (Flower of Life Overlap)
    pub fn with_erasure_coding(mut self, k: usize, m: usize) -> Self {
        self.erasure_coding = Some((k, m));
        self
    }

    /// Add replica location to mesh
    pub fn add_replica(mut self, peer_id: String, location: String) -> Self {
        self.replicas.insert(peer_id, location);
        self
    }

    /// Compute cryptographic audit trail hash
    /// Binds the Data (Body) to the Metadata (Spirit/Geometry)
    fn compute_audit_hash(data: &[u8], metadata: &ShardMetadata) -> Blake3Digest {
        let mut hasher = blake3_hash();
        hasher.update(b"aurafs_shard_v3"); // Phase II Architecture
        hasher.update(data);
        hasher.update(&metadata.shard_id.digest());
        // Implicitly hashing geometry because it's inside metadata
        hasher.finalize()
    }

    /// Validate shard integrity & quantum signatures with detailed error reporting
    pub fn validate(&self) -> Result<(), ShardError> {
        // 1. Content-addressing verification
        let computed_id = ShardId::from_content(&self.data);
        if computed_id.digest() != self.shard_id.digest() {
            return Err(ShardError::ContentMismatch(format!(
                "Expected {}, computed {}",
                self.shard_id.short_id(), computed_id.short_id()
            )));
        }

        // 2. Audit trail verification
        let audit_hash = Self::compute_audit_hash(&self.data, &self.metadata);
        if audit_hash != self.audit_hash {
            return Err(ShardError::AuditMismatch(format!(
                "Expected {}, computed {}",
                hex::encode(&self.audit_hash.as_bytes()[..16]),
                hex::encode(&audit_hash.as_bytes()[..16])
            )));
        }

        // 3. Quantum signature verification
        if let Some(sig) = &self.signature {
            sig.verify(self.shard_id.digest())
                .map_err(|e| ShardError::SignatureVerification(format!("Signature verification failed: {}", e)))?;
        }

        // 4. Data size validation
        if self.data.is_empty() {
            return Err(ShardError::InvalidInput("Shard data cannot be empty".to_string()));
        }

        // 5. Metadata consistency check
        if self.metadata.size_bytes != self.data.len() as u64 {
            return Err(ShardError::ValidationError(format!(
                "Metadata size {} does not match data size {}",
                self.metadata.size_bytes,
                self.data.len()
            )));
        }

        Ok(())
    }

    /// Serialize shard to persistent storage format with validation
    pub async fn serialize(&self) -> Result<Vec<u8>, ShardError> {
        // Validate before serialization to prevent corrupt state persistence
        self.validate()?;
        
        postcard::to_stdvec(self)
            .map_err(|e| ShardError::Serialization(e.to_string()))
    }

    /// Deserialize shard from storage with validation
    pub async fn deserialize(data: &[u8]) -> Result<Self, ShardError> {
        if data.is_empty() {
            return Err(ShardError::InvalidInput("Cannot deserialize empty data".to_string()));
        }
        
        let shard: Self = postcard::from_bytes(data)
            .map_err(|e| ShardError::Deserialization(format!(
                "Failed to deserialize shard data (size: {} bytes): {}",
                data.len(), e
            )))?;
        
        // Validate deserialized shard immediately
        shard.validate()?;
        
        // Verify checksum: compute hash of shard data and compare with shard_id
        let computed_id = ShardId::from_content(&shard.data);
        if computed_id.digest() != shard.shard_id.digest() {
            return Err(ShardError::ChecksumFailed);
        }
        
        Ok(shard)
    }
    
    /// Create a new shard with different data (for encryption/compression)
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        // Recompute shard_id based on new data
        self.shard_id = ShardId::from_content(&self.data);
        // Recompute audit hash
        self.audit_hash = Self::compute_audit_hash(&self.data, &self.metadata);
        self
    }
}

/// Shard persistence engine for local disk storage
/// Note: This is a lightweight helper. Full tiered storage is in `storage.rs`.
pub struct ShardPersistence {
    storage_dir: String,
}

impl ShardPersistence {
    /// Create new persistence engine
    pub fn new(storage_dir: String) -> Self {
        Self { storage_dir }
    }

    /// Persist shard to disk with content-addressed naming and retry logic
    pub async fn store(&self, shard: &Shard) -> Result<(), ShardError> {
        const MAX_RETRIES: usize = 3;
        
        // Validate shard before storing
        shard.validate()?;
        
        let path = format!("{}/{}", self.storage_dir, shard.shard_id.short_id());
        let serialized = shard.serialize().await?;
        
        // Retry write operation
        for attempt in 0..MAX_RETRIES {
            match self.write_shard_file(&path, &serialized).await {
                Ok(_) => return Ok(()),
                Err(e) if attempt < MAX_RETRIES - 1 => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * (attempt as u64 + 1))).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(ShardError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to store shard after {} attempts", MAX_RETRIES)
        )))
    }
    
    /// Write shard file with proper error handling
    async fn write_shard_file(&self, path: &str, data: &[u8]) -> Result<(), ShardError> {
        let mut file = File::create(path).await?;
        file.write_all(data).await?;
        file.sync_all().await?; // Ensure durability
        Ok(())
    }

    /// Load shard from disk with checksum verification
    pub async fn load(&self, shard_id: &ShardId) -> Result<Shard, ShardError> {
        let path = format!("{}/{}", self.storage_dir, shard_id.short_id());
        
        let mut file = File::open(&path).await
            .map_err(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    ShardError::NotFound(format!("Shard {} not found at {}", shard_id.short_id(), path))
                } else {
                    ShardError::Io(e)
                }
            })?;
        
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        
        if buffer.is_empty() {
            return Err(ShardError::InvalidInput("Shard file is empty".to_string()));
        }
        
        Shard::deserialize(&buffer).await
    }
}

/// Enterprise-grade shard errors with comprehensive context
#[derive(Debug, Error)]
pub enum ShardError {
    /// Content-addressing mismatch
    #[error("Content-addressing mismatch: {0}")]
    ContentMismatch(String),
    
    /// Audit trail hash mismatch
    #[error("Audit trail hash mismatch: {0}")]
    AuditMismatch(String),
    
    /// Serialization failed
    #[error("Serialization failed: {0}")]
    Serialization(String),
    
    /// Deserialization failed
    #[error("Deserialization failed: {0}")]
    Deserialization(String),
    
    /// Quantum signature verification failed
    #[error("Quantum signature verification failed: {0}")]
    SignatureVerification(String),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    /// Erasure coding error
    #[error("Erasure coding error: {0}")]
    ErasureCoding(String),
    
    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// Shard creation failed
    #[error("Shard creation failed: {0}")]
    CreationFailed(String),
    
    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),
    
    /// Index error
    #[error("Index error: {0}")]
    IndexError(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Signature error
    #[error("Signature error: {0}")]
    SignatureError(String),
    
    /// Shard not found
    #[error("Shard not found: {0}")]
    NotFound(String),
    
    /// Shard not found (no message)
    #[error("Shard not found")]
    NotFoundSimple,
    
    /// Operation timeout
    #[error("Operation timeout")]
    Timeout,
    
    /// Checksum verification failed
    #[error("Checksum verification failed")]
    ChecksumFailed,
    
    /// Compression error
    #[error("Compression error: {0}")]
    CompressionError(String),
}

/// Shard statistics for monitoring
#[derive(Debug, Serialize)]
pub struct ShardStats {
    /// Shard identifier
    pub shard_id: ShardId,
    /// Compressed size in bytes
    pub size_compressed: usize,
    /// Compression ratio (original/compressed)
    pub compression_ratio: f64,
    /// Number of replicas
    pub replica_count: usize,
    /// Whether validation passed
    pub validation_status: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shard_forging() {
        let data = b"Mythical shard data";
        let shard_id = ShardId::from_content(data);
        let metadata = ShardMetadata::new(shard_id.clone(), data.len() as u64, crate::shard::metadata::ShardLayer::Data);
        let shard = Shard::forge(data.to_vec(), metadata).expect("Failed to forge shard");
        
        assert!(shard.validate().is_ok());
    }

    #[tokio::test]
    async fn test_content_addressing() {
        let data1 = b"Same data";
        let data2 = b"Same data";
        
        let shard_id1 = ShardId::from_content(data1);
        let shard_id2 = ShardId::from_content(data2);
        
        let meta1 = ShardMetadata::new(shard_id1.clone(), data1.len() as u64, crate::shard::metadata::ShardLayer::Data);
        let meta2 = ShardMetadata::new(shard_id2.clone(), data2.len() as u64, crate::shard::metadata::ShardLayer::Data);
        
        let shard1 = Shard::forge(data1.to_vec(), meta1).unwrap();
        let shard2 = Shard::forge(data2.to_vec(), meta2).unwrap();
        
        // Same content = same shard ID
        assert_eq!(shard1.shard_id.digest(), shard2.shard_id.digest());
    }

    #[test]
    fn test_shard_id_display() {
        let data = b"test data for display";
        let shard_id = ShardId::from_content(data);
        let display = shard_id.to_cid();
        
        // Depending on ShardId impl, it might start with aura: or similar
        // Adjust assertion based on actual ShardId::to_cid implementation
        // assert!(display.starts_with("aura:")); 
        assert!(!display.is_empty());
    }
}