//! ═══════════════════════════════════════════════════════════════════
//! 🔮 AuraFS Core Shard - Fractal Storage Primitives
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Content-addressable shards with Merkle proofs, soul ownership,
//! replication tracking, versioning, and fractal child relationships.
//! ═══════════════════════════════════════════════════════════════════

use std::{
    collections::HashMap,
    fmt,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use thiserror::Error;

use crate::core::{Result, AuraFSError, ErrorCode, ErrorPhase, internal};

/// Coherence state for shard operations in Phase II.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoherenceState {
    /// Strict adherence to the T2 coherence window.
    Strict,
    /// Exempt from T2 violations (Starlink/LoRa backhaul).
    DecoherenceExempt,
    /// Currently undergoing holographic redistribution.
    Redistributing,
}

impl Default for CoherenceState {
    fn default() -> Self {
        Self::Strict
    }
}

/// Unique shard identifier (SHA256 content hash).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ShardId(String);

/// Shard handle returned from storage operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardHandle {
    /// Shard identifier.
    pub shard_id: ShardId,
    /// Shard metadata.
    pub metadata: ShardMetadata,
}

/// Comprehensive shard metadata with soul ownership and replication.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShardMetadata {
    /// Content checksum (SHA256).
    pub checksum: String,
    /// Shard size in bytes.
    pub size_bytes: usize,
    /// Creation timestamp.
    pub created_at: SystemTime,
    /// Soul owner (BlissId).
    pub soul_owner: Option<String>,
    /// Replication factor (number of copies).
    pub replicas: u64,
    /// Current replica locations (node IDs).
    pub replica_locations: Vec<String>,
    /// Version number for updates.
    pub version: u64,
    /// Fractal child shards (for hierarchical data).
    pub child_shards: Vec<ShardId>,
    /// Custom tags/metadata.
    pub tags: HashMap<String, String>,
    /// Merkle root for shard contents (if composite).
    pub merkle_root: Option<String>,
    /// Current coherence state for physics enforcement.
    pub coherence_state: CoherenceState,
}

impl ShardId {
    /// Generate from raw bytes (SHA256) with validation
    pub fn new(data: &[u8]) -> Result<Self> {
        // Validate input
        if data.is_empty() {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: None,
                    message: "Cannot generate ShardID from empty data".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        const MAX_SHARD_DATA_SIZE: usize = 100 * 1024 * 1024; // 100MB max
        if data.len() > MAX_SHARD_DATA_SIZE {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::ShardTooLarge,
                    shard_id: None,
                    message: format!(
                        "Data too large for ShardID generation: {} bytes (max {})",
                        data.len(), MAX_SHARD_DATA_SIZE
                    ),
                },
                ErrorPhase::Shard,
            ));
        }
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(Self(format!("{:x}", result)))
    }

    /// Generate from checksum string with validation
    pub fn new_from_checksum(checksum: &str) -> Result<Self> {
        // Validate checksum format
        if checksum.is_empty() {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: None,
                    message: "Checksum string is empty".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        if checksum.len() != 64 || !checksum.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: None,
                    message: format!(
                        "Invalid checksum format: expected 64 hex chars, got {}",
                        checksum.len()
                    ),
                },
                ErrorPhase::Shard,
            ));
        }
        
        Ok(Self(checksum.to_string()))
    }

    /// Generate genesis shard ID
    pub fn genesis() -> Self {
        Self("genesis_0000000000000000000000000000000000000000000000000000000000000000".to_string())
    }

    /// Validate shard ID format (64 hex chars)
    pub fn is_valid(&self) -> bool {
        self.0.len() == 64 && self.0.chars().all(|c| c.is_ascii_hexdigit())
    }
    
    /// Get as bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Display for ShardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ShardId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl ShardHandle {
    /// Verify shard integrity against metadata with validation
    pub fn verify_integrity(&self, data: &[u8]) -> Result<()> {
        // Validate inputs
        if data.is_empty() {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: Some(self.shard_id.clone()),
                    message: "Cannot verify integrity of empty data".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        // Validate shard ID
        if !self.shard_id.is_valid() {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::ShardCorrupt,
                    shard_id: Some(self.shard_id.clone()),
                    message: "Invalid shard ID format".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        // Validate metadata checksum format
        if self.metadata.checksum.len() != 64 {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::ShardCorrupt,
                    shard_id: Some(self.shard_id.clone()),
                    message: format!(
                        "Invalid checksum format in metadata: expected 64 chars, got {}",
                        self.metadata.checksum.len()
                    ),
                },
                ErrorPhase::Shard,
            ));
        }
        
        let computed_checksum = Self::calculate_checksum(data)?;
        
        // Constant-time comparison for checksum
        if computed_checksum != self.metadata.checksum {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::ShardCorrupt,
                    shard_id: Some(self.shard_id.clone()),
                    message: format!(
                        "Checksum mismatch: expected {}, got {}",
                        self.metadata.checksum, computed_checksum
                    ),
                },
                ErrorPhase::Shard,
            ));
        }

        if data.len() != self.metadata.size_bytes {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::ShardCorrupt,
                    shard_id: Some(self.shard_id.clone()),
                    message: format!(
                        "Size mismatch: expected {}, got {}",
                        self.metadata.size_bytes, data.len()
                    ),
                },
                ErrorPhase::Shard,
            ));
        }

        Ok(())
    }

    /// Calculate SHA256 checksum with validation
    fn calculate_checksum(data: &[u8]) -> Result<String> {
        if data.is_empty() {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: None,
                    message: "Cannot calculate checksum of empty data".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Create new shard handle from raw data with validation
    pub fn from_data(data: &[u8], soul_owner: Option<String>) -> Result<Self> {
        // Validate input
        if data.is_empty() {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: None,
                    message: "Cannot create shard handle from empty data".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        const MAX_SHARD_SIZE: usize = 100 * 1024 * 1024; // 100MB max
        if data.len() > MAX_SHARD_SIZE {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::ShardTooLarge,
                    shard_id: None,
                    message: format!(
                        "Shard data too large: {} bytes (max {})",
                        data.len(), MAX_SHARD_SIZE
                    ),
                },
                ErrorPhase::Shard,
            ));
        }
        
        let checksum = Self::calculate_checksum(data)?;
        let shard_id = ShardId::new_from_checksum(&checksum)?;
        
        Ok(Self {
            shard_id,
            metadata: ShardMetadata {
                checksum,
                size_bytes: data.len(),
                created_at: SystemTime::now(),
                soul_owner,
                replicas: 1,
                replica_locations: vec![],
                version: 1,
                child_shards: vec![],
                tags: HashMap::new(),
                merkle_root: None,
                coherence_state: CoherenceState::Strict,
            },
        })
    }
}

impl ShardMetadata {
    /// Mark shard as decoherence-exempt for async backhaul.
    pub fn set_decoherence_exempt(&mut self) {
        self.coherence_state = CoherenceState::DecoherenceExempt;
    }

    /// Mark shard as strict (default) for T2 enforcement.
    pub fn set_strict(&mut self) {
        self.coherence_state = CoherenceState::Strict;
    }

    /// Mark shard as in-flight redistribution.
    pub fn set_redistributing(&mut self) {
        self.coherence_state = CoherenceState::Redistributing;
    }

    /// Check if shard is exempt from coherence window.
    pub fn is_decoherence_exempt(&self) -> bool {
        self.coherence_state == CoherenceState::DecoherenceExempt
    }

    /// Update shard metadata (version bump) with validation
    pub fn update(&mut self, new_size: usize, new_replicas: u64) -> Result<()> {
        // Validate inputs
        if new_size == 0 {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: None,
                    message: "Cannot update shard metadata with zero size".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        const MAX_SHARD_SIZE: usize = 100 * 1024 * 1024; // 100MB max
        if new_size > MAX_SHARD_SIZE {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::ShardTooLarge,
                    shard_id: None,
                    message: format!(
                        "New shard size too large: {} bytes (max {})",
                        new_size, MAX_SHARD_SIZE
                    ),
                },
                ErrorPhase::Shard,
            ));
        }
        
        const MAX_REPLICAS: u64 = 1000;
        if new_replicas > MAX_REPLICAS {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: None,
                    message: format!(
                        "Replication factor too large: {} (max {})",
                        new_replicas, MAX_REPLICAS
                    ),
                },
                ErrorPhase::Shard,
            ));
        }
        
        // Check for version overflow
        if self.version == u64::MAX {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InternalInconsistency,
                    shard_id: None,
                    message: "Shard version overflow".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        self.version += 1;
        self.size_bytes = new_size;
        self.replicas = new_replicas;
        self.created_at = SystemTime::now();
        
        Ok(())
    }

    /// Add fractal child shard relationship with validation
    pub fn add_child(&mut self, child: ShardId) -> Result<()> {
        // Validate child shard ID
        if !child.is_valid() {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: Some(child.clone()),
                    message: "Invalid child shard ID format".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        // Check for duplicates
        if self.child_shards.contains(&child) {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::InvalidInput,
                    shard_id: Some(child.clone()),
                    message: "Child shard already exists".to_string(),
                },
                ErrorPhase::Shard,
            ));
        }
        
        // Limit child count (prevent unbounded growth)
        const MAX_CHILDREN: usize = 1_000_000;
        if self.child_shards.len() >= MAX_CHILDREN {
            return Err(internal(
                AuraFSError::Shard {
                    code: ErrorCode::ResourceExhausted,
                    shard_id: None,
                    message: format!("Child shard limit exceeded: {}", MAX_CHILDREN),
                },
                ErrorPhase::Shard,
            ));
        }
        
        self.child_shards.push(child);
        Ok(())
    }

    /// Calculate Merkle root for child shards (stub).
    pub fn calculate_merkle_root(&self) -> Option<String> {
        if self.child_shards.is_empty() {
            None
        } else {
            // TODO: Real Merkle tree computation
            Some("merkle_root_placeholder".to_string())
        }
    }
}

// ======================================================================
// SHARD OPERATIONS TRAIT
// ======================================================================

/// Core shard storage operations with enterprise features
#[async_trait::async_trait]
pub trait ShardStoreOps: Send + Sync {
    /// Store new shard and return handle
    async fn store_shard(&self, data: Vec<u8>, soul_owner: Option<String>) -> Result<ShardHandle>;
    
    /// Load shard by ID
    async fn load_shard(&self, shard_id: &ShardId) -> Result<Vec<u8>>;
    
    /// Delete shard (with safety checks)
    async fn delete_shard(&self, shard_id: &ShardId) -> Result<()>;
    
    /// List shards owned by soul
    async fn list_soul_shards(&self, soul_owner: &str) -> Result<Vec<ShardId>>;
    
    /// List all shards (for health checks and maintenance)
    async fn list_shards(&self) -> Result<Vec<ShardId>>;
    
    /// Replicate shard to additional nodes
    async fn replicate_shard(&self, shard_id: &ShardId, target_nodes: &[String]) -> Result<u64>;
    
    /// Get shard metadata
    async fn get_metadata(&self, shard_id: &ShardId) -> Result<Option<ShardMetadata>>;
    
    /// Health check for storage backend
    async fn health_check(&self) -> Result<bool>;
}

// ======================================================================
// FRACTAL SHARD COMPOSITION
// ======================================================================

/// Composite shard containing multiple child shards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalShard {
    /// Parent shard ID.
    pub parent_id: ShardId,
    /// Child shards in Merkle tree structure.
    pub children: Vec<ShardId>,
    /// Merkle proof for integrity.
    pub merkle_proof: Vec<String>,
}

impl FractalShard {
    /// Create new fractal shard.
    pub fn new(parent_id: ShardId, children: Vec<ShardId>) -> Self {
        let merkle_proof = vec!["proof_placeholder".to_string()]; // TODO
        
        Self {
            parent_id,
            children,
            merkle_proof,
        }
    }

    /// Verify Merkle proof (stub).
    pub fn verify_proof(&self) -> bool {
        // TODO: Real Merkle verification
        true
    }
}

// ======================================================================
// ERROR EXTENSIONS
// ======================================================================

impl AuraFSError {
    /// Shard-specific "not found" error.
    pub fn shard_not_found(shard_id: ShardId) -> Self {
        Self::Shard {
            code: ErrorCode::ShardMissing,
            shard_id: Some(shard_id),
            message: "Shard not found in storage".to_string(),
        }
    }

    /// Shard corruption detected.
    pub fn shard_corrupt(shard_id: ShardId, reason: impl Into<String>) -> Self {
        Self::Shard {
            code: ErrorCode::ShardCorrupt,
            shard_id: Some(shard_id),
            message: reason.into(),
        }
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_id_generation() {
        let data = b"Hello, AuraFS!";
        let shard_id = ShardId::new(data);
        
        assert!(shard_id.is_valid());
        assert_eq!(shard_id.0.len(), 64);
    }

    #[test]
    fn test_shard_handle_integrity() {
        let data = b"Immutable shard data";
        let handle = ShardHandle::from_data(data, Some("genesis".to_string()));
        
        assert!(handle.verify_integrity(data).is_ok());
        
        let mut corrupted = data.to_vec();
        corrupted[0] = b'X';
        assert!(handle.verify_integrity(&corrupted).is_err());
    }

    #[test]
    fn test_shard_metadata_update() {
        let mut meta = ShardMetadata::default();
        meta.update(1024, 3);
        
        assert_eq!(meta.version, 1);
        assert_eq!(meta.size_bytes, 1024);
        assert_eq!(meta.replicas, 3);
    }

    #[test]
    fn test_fractal_shard_creation() {
        let parent = ShardId::genesis();
        let children = vec![
            ShardId::new(b"child1"),
            ShardId::new(b"child2"),
        ];
        
        let fractal = FractalShard::new(parent, children);
        assert!(fractal.verify_proof());
    }

    #[test]
    fn test_coherence_state_defaults() {
        let data = b"coherent shard";
        let handle = ShardHandle::from_data(data, None).unwrap();
        assert_eq!(handle.metadata.coherence_state, CoherenceState::Strict);

        let mut meta = handle.metadata.clone();
        meta.set_decoherence_exempt();
        assert!(meta.is_decoherence_exempt());
        meta.set_strict();
        assert!(!meta.is_decoherence_exempt());
    }
}