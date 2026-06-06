//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Metadata - Bio-Resonant Layered Architecture
//! 🔑 Storage, Object, Data, File, Network, & Compute Metadata
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::shard::id::{ShardId, ShardLayer, ShardIdentifier};
use crate::compression::CompressionAlgorithm;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// ═══════════════════════════════════════════════════════════════════
// 🟢 THE LATTICE ATLAS (Phase II Physics)
// ═══════════════════════════════════════════════════════════════════

/// The topological geometry required for the shard's bio-resonance.
/// This dictates how the node mesh routes, stores, and processes the data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LatticeGeometry {
    /// 🌸 SOUL STAR (The Dream)
    /// The Fundamental Grid. Used for high-level Objects, Identities, and Pointers.
    /// Physics: Balanced Logic (17) + Stability. The "Superset".
    FlowerOfLife,

    /// 🧠 SOLAR PLEXUS (The Brain)
    /// Kagome Lattice. Used for Compute, AI Kernels, and Smart Contracts.
    /// Physics: Max Frustration (Score 25) = Computation.
    Kagome,

    /// 🌲 ROOT CHAKRA (The Anchor)
    /// Bethe Lattice (Tree). Used for Cold Storage and Archival.
    /// Physics: Max Stability (0.178) + Flat Band (0.438).
    Bethe,

    /// 🌊 SACRAL CHAKRA (The Flow)
    /// Triangular Lattice. Used for Network Routing and Gossip.
    /// Physics: Min Path Length (2.56) = Max Velocity.
    Triangular,

    /// 💎 THIRD EYE (The Shield)
    /// Diamond Cubic. Used for Data Encapsulation and Encryption.
    /// Physics: High Bandgap + Density.
    Diamond,

    /// 👑 CROWN CHAKRA (The Memory)
    /// Sierpinski Fractal. Used for Files and Indexing.
    /// Physics: Max Localization (0.109). Infinite storage in finite space.
    Sierpinski,
}

impl Default for LatticeGeometry {
    fn default() -> Self {
        Self::FlowerOfLife // All things return to the Source
    }
}

// ═══════════════════════════════════════════════════════════════════
// BlissId Stub (TODO: Future - Not Yet Implemented)
// ═══════════════════════════════════════════════════════════════════

/// BlissId - Soul-based identity system
pub type BlissId = String;

// ═══════════════════════════════════════════════════════════════════
// Replication Status
// ═══════════════════════════════════════════════════════════════════

/// Replication health state in AuraFS mesh
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplicationStatus {
    /// All replicas are healthy and synchronized
    Healthy,
    /// Some replicas are missing or out of sync
    Degraded,
    /// Replication status is unknown or not yet checked
    Unknown,
    /// Shard is being replicated to new peers
    Replicating,
    /// Shard has been marked for deletion
    PendingDelete,
}

impl Default for ReplicationStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Peer node identifier
pub type PeerId = String;

/// Inode identifier for file layer
pub type InodeId = u64;

// ═══════════════════════════════════════════════════════════════════
// Encryption Scheme (for Object Layer)
// ═══════════════════════════════════════════════════════════════════

/// Encryption scheme for shard data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionScheme {
    /// No encryption
    None,
    /// AES-256-GCM symmetric encryption
    Aes256Gcm,
    /// ChaCha20-Poly1305 symmetric encryption
    ChaCha20Poly1305,
    /// Kyber-1024 KEM hybrid encryption (Phase II Quantum Safe)
    Kyber1024,
}

impl Default for EncryptionScheme {
    fn default() -> Self {
        Self::None
    }
}

// ═══════════════════════════════════════════════════════════════════
// ShardMetadata Trait
// ═══════════════════════════════════════════════════════════════════

/// Core metadata trait - all layers implement this
pub trait ShardMetadataTrait: Send + Sync {
    fn shard_id(&self) -> &ShardId;
    fn size_bytes(&self) -> u64;
    fn created_ns(&self) -> u64;
    fn content_type(&self) -> Option<&str>;
    fn layer(&self) -> ShardLayer;
    fn owner(&self) -> Option<&BlissId>;
    fn tags(&self) -> &[String];
    
    /// Get the Bio-Resonant Geometry
    fn geometry(&self) -> LatticeGeometry;
}

// ═══════════════════════════════════════════════════════════════════
// Core Shard Metadata (Base Struct)
// ═══════════════════════════════════════════════════════════════════

/// Core metadata struct - composition base for all layer-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreShardMetadata {
    /// Unique shard identifier
    pub shard_id: ShardId,

    /// Size of shard in bytes
    pub size_bytes: u64,

    /// Timestamp shard was created (unix nanos)
    pub created_ns: u64,

    /// Content type (e.g., 'model_slice', 'audio', 'video', 'json')
    pub content_type: Option<String>,

    /// Owner identity (TODO: Future - BlissID integration)
    pub owner: Option<BlissId>,

    /// Custom tags for flexible querying/labeling
    pub tags: Vec<String>,

    /// Which layer this metadata belongs to
    pub layer: ShardLayer,

    /// 🟢 The Geometry Field (The Bio-Resonant Shape)
    /// Automatically assigned based on layer, but mutable by CoherenceMonitor
    #[serde(default)] 
    pub geometry: LatticeGeometry,
}

impl CoreShardMetadata {
    /// Create new core metadata with automatic geometry assignment
    pub fn new(shard_id: ShardId, size_bytes: u64, layer: ShardLayer) -> Self {
        let now_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        // 🔮 Auto-Resonance: Map Layer to Physics Geometry
        let default_geometry = match layer {
            ShardLayer::Storage => LatticeGeometry::Bethe,       // Root/Stability
            ShardLayer::Network => LatticeGeometry::Triangular,  // Sacral/Flow
            ShardLayer::Data => LatticeGeometry::Diamond,        // Third Eye/Density
            ShardLayer::File => LatticeGeometry::Sierpinski,     // Crown/Fractal
            ShardLayer::Object => LatticeGeometry::FlowerOfLife, // Soul Star/Source
            // Fallback for Compute if ShardLayer::Compute exists in ID module
            _ => LatticeGeometry::Kagome, // Assume Compute/Logic for others
        };

        Self {
            shard_id,
            size_bytes,
            created_ns: now_ns,
            content_type: None,
            owner: None,
            tags: Vec::new(),
            layer,
            geometry: default_geometry,
        }
    }

    /// Create with content type
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Set owner
    pub fn with_owner(mut self, owner: BlissId) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        let tag = tag.into();
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
        self
    }
}

impl ShardMetadataTrait for CoreShardMetadata {
    fn shard_id(&self) -> &ShardId { &self.shard_id }
    fn size_bytes(&self) -> u64 { self.size_bytes }
    fn created_ns(&self) -> u64 { self.created_ns }
    fn content_type(&self) -> Option<&str> { self.content_type.as_deref() }
    fn layer(&self) -> ShardLayer { self.layer }
    fn owner(&self) -> Option<&BlissId> { self.owner.as_ref() }
    fn tags(&self) -> &[String] { &self.tags }
    fn geometry(&self) -> LatticeGeometry { self.geometry }
}

// ═══════════════════════════════════════════════════════════════════
// Storage Layer Metadata (Root Chakra / Bethe)
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageShardMetadata {
    pub core: CoreShardMetadata,
    pub disk_path: Option<PathBuf>,
    pub replicas: u8,
    pub erasure_k: u8,
    pub erasure_n: u8,
    pub health_score: f32,
    pub last_check_ns: u64,
}

impl StorageShardMetadata {
    pub fn new(shard_id: ShardId, size_bytes: u64) -> Self {
        Self {
            core: CoreShardMetadata::new(shard_id, size_bytes, ShardLayer::Storage),
            disk_path: None,
            replicas: 3,
            erasure_k: 8,
            erasure_n: 12,
            health_score: 1.0,
            last_check_ns: 0,
        }
    }

    pub fn with_disk_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.disk_path = Some(path.into());
        self
    }

    pub fn with_erasure(mut self, k: u8, n: u8) -> Self {
        self.erasure_k = k;
        self.erasure_n = n;
        self
    }
}

impl ShardMetadataTrait for StorageShardMetadata {
    fn shard_id(&self) -> &ShardId { &self.core.shard_id }
    fn size_bytes(&self) -> u64 { self.core.size_bytes }
    fn created_ns(&self) -> u64 { self.core.created_ns }
    fn content_type(&self) -> Option<&str> { self.core.content_type.as_deref() }
    fn layer(&self) -> ShardLayer { ShardLayer::Storage }
    fn owner(&self) -> Option<&BlissId> { self.core.owner.as_ref() }
    fn tags(&self) -> &[String] { &self.core.tags }
    fn geometry(&self) -> LatticeGeometry { self.core.geometry }
}

// ═══════════════════════════════════════════════════════════════════
// Object Layer Metadata (Soul Star / Flower of Life)
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectShardMetadata {
    pub core: CoreShardMetadata,
    pub compression: CompressionAlgorithm,
    pub encryption: EncryptionScheme,
    pub parent_shard: Option<ShardId>,
    pub child_shards: Vec<ShardId>,
    pub uncompressed_size: u64,
}

impl ObjectShardMetadata {
    pub fn new(shard_id: ShardId, size_bytes: u64) -> Self {
        Self {
            core: CoreShardMetadata::new(shard_id, size_bytes, ShardLayer::Object),
            compression: CompressionAlgorithm::None,
            encryption: EncryptionScheme::None,
            parent_shard: None,
            child_shards: Vec::new(),
            uncompressed_size: size_bytes,
        }
    }

    pub fn with_compression(mut self, algo: CompressionAlgorithm, uncompressed_size: u64) -> Self {
        self.compression = algo;
        self.uncompressed_size = uncompressed_size;
        self
    }

    pub fn with_encryption(mut self, scheme: EncryptionScheme) -> Self {
        self.encryption = scheme;
        self
    }

    pub fn with_parent(mut self, parent: ShardId) -> Self {
        self.parent_shard = Some(parent);
        self
    }

    pub fn add_child(&mut self, child: ShardId) {
        self.child_shards.push(child);
    }
}

impl ShardMetadataTrait for ObjectShardMetadata {
    fn shard_id(&self) -> &ShardId { &self.core.shard_id }
    fn size_bytes(&self) -> u64 { self.core.size_bytes }
    fn created_ns(&self) -> u64 { self.core.created_ns }
    fn content_type(&self) -> Option<&str> { self.core.content_type.as_deref() }
    fn layer(&self) -> ShardLayer { ShardLayer::Object }
    fn owner(&self) -> Option<&BlissId> { self.core.owner.as_ref() }
    fn tags(&self) -> &[String] { &self.core.tags }
    fn geometry(&self) -> LatticeGeometry { self.core.geometry }
}

// ═══════════════════════════════════════════════════════════════════
// Compute Layer Metadata (Solar Plexus / Kagome)
// ═══════════════════════════════════════════════════════════════════

/// Compute layer metadata - AI Models & Smart Contracts
/// 
/// 🧠 The "Brain" of AuraFS. Stores PyTorch tensors and WASM contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeShardMetadata {
    pub core: CoreShardMetadata,
    
    /// Precision (e.g., "f16", "f32", "int8")
    pub precision: String,
    
    /// Estimated FLOPs required to process this shard
    pub flops_cost: u64,
    
    /// Frustration Score (Complexity of the logic)
    pub frustration_score: f64,
}

impl ComputeShardMetadata {
    pub fn new(shard_id: ShardId, size_bytes: u64) -> Self {
        // Note: Assuming ShardLayer::Compute exists, or falling back
        // If ShardLayer::Compute is not yet in id.rs, we use Object but force Kagome
        let mut core = CoreShardMetadata::new(shard_id, size_bytes, ShardLayer::Object);
        core.geometry = LatticeGeometry::Kagome; 

        Self {
            core,
            precision: "f32".to_string(),
            flops_cost: 0,
            frustration_score: 0.0,
        }
    }
}

impl ShardMetadataTrait for ComputeShardMetadata {
    fn shard_id(&self) -> &ShardId { &self.core.shard_id }
    fn size_bytes(&self) -> u64 { self.core.size_bytes }
    fn created_ns(&self) -> u64 { self.core.created_ns }
    fn content_type(&self) -> Option<&str> { self.core.content_type.as_deref() }
    fn layer(&self) -> ShardLayer { self.core.layer }
    fn owner(&self) -> Option<&BlissId> { self.core.owner.as_ref() }
    fn tags(&self) -> &[String] { &self.core.tags }
    fn geometry(&self) -> LatticeGeometry { self.core.geometry }
}

// ═══════════════════════════════════════════════════════════════════
// Data Layer Metadata (Third Eye / Diamond)
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataShardMetadata {
    pub core: CoreShardMetadata,
    pub format: String,
    pub schema_version: u32,
    pub schema_hash: Option<String>,
    pub record_count: Option<u64>,
}

impl DataShardMetadata {
    pub fn new(shard_id: ShardId, size_bytes: u64, format: impl Into<String>) -> Self {
        Self {
            core: CoreShardMetadata::new(shard_id, size_bytes, ShardLayer::Data),
            format: format.into(),
            schema_version: 1,
            schema_hash: None,
            record_count: None,
        }
    }

    pub fn with_schema(mut self, version: u32, hash: Option<String>) -> Self {
        self.schema_version = version;
        self.schema_hash = hash;
        self
    }
}

impl ShardMetadataTrait for DataShardMetadata {
    fn shard_id(&self) -> &ShardId { &self.core.shard_id }
    fn size_bytes(&self) -> u64 { self.core.size_bytes }
    fn created_ns(&self) -> u64 { self.core.created_ns }
    fn content_type(&self) -> Option<&str> { self.core.content_type.as_deref() }
    fn layer(&self) -> ShardLayer { ShardLayer::Data }
    fn owner(&self) -> Option<&BlissId> { self.core.owner.as_ref() }
    fn tags(&self) -> &[String] { &self.core.tags }
    fn geometry(&self) -> LatticeGeometry { self.core.geometry }
}

// ═══════════════════════════════════════════════════════════════════
// File Layer Metadata (Crown / Sierpinski)
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileShardMetadata {
    pub core: CoreShardMetadata,
    pub inode_id: InodeId,
    pub permissions: u16,
    pub mtime_ns: u64,
    pub atime_ns: u64,
    pub filename: String,
    pub parent_inode: Option<InodeId>,
}

impl FileShardMetadata {
    pub fn new(shard_id: ShardId, size_bytes: u64, inode_id: InodeId, filename: impl Into<String>) -> Self {
        Self {
            core: CoreShardMetadata::new(shard_id, size_bytes, ShardLayer::File),
            inode_id,
            permissions: 0o644,
            mtime_ns: 0, // In production use SystemTime::now()
            atime_ns: 0,
            filename: filename.into(),
            parent_inode: None,
        }
    }

    pub fn with_permissions(mut self, perms: u16) -> Self {
        self.permissions = perms;
        self
    }

    pub fn with_parent(mut self, parent: InodeId) -> Self {
        self.parent_inode = Some(parent);
        self
    }
}

impl ShardMetadataTrait for FileShardMetadata {
    fn shard_id(&self) -> &ShardId { &self.core.shard_id }
    fn size_bytes(&self) -> u64 { self.core.size_bytes }
    fn created_ns(&self) -> u64 { self.core.created_ns }
    fn content_type(&self) -> Option<&str> { self.core.content_type.as_deref() }
    fn layer(&self) -> ShardLayer { ShardLayer::File }
    fn owner(&self) -> Option<&BlissId> { self.core.owner.as_ref() }
    fn tags(&self) -> &[String] { &self.core.tags }
    fn geometry(&self) -> LatticeGeometry { self.core.geometry }
}

// ═══════════════════════════════════════════════════════════════════
// Network Layer Metadata (Sacral / Triangular)
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkShardMetadata {
    pub core: CoreShardMetadata,
    pub peer_locations: HashSet<PeerId>,
    pub replication_status: ReplicationStatus,
    pub last_sync_ns: u64,
    pub mesh_hops: u8,
    pub replication_priority: u8,
}

impl NetworkShardMetadata {
    pub fn new(shard_id: ShardId, size_bytes: u64) -> Self {
        Self {
            core: CoreShardMetadata::new(shard_id, size_bytes, ShardLayer::Network),
            peer_locations: HashSet::new(),
            replication_status: ReplicationStatus::Unknown,
            last_sync_ns: 0,
            mesh_hops: 0,
            replication_priority: 5,
        }
    }

    pub fn add_peer(&mut self, peer: PeerId) {
        self.peer_locations.insert(peer);
    }

    pub fn remove_peer(&mut self, peer: &str) {
        self.peer_locations.remove(peer);
    }

    pub fn update_replication_status(&mut self, desired_replicas: usize) {
        let count = self.peer_locations.len();
        self.replication_status = if count >= desired_replicas {
            ReplicationStatus::Healthy
        } else if count > 0 {
            ReplicationStatus::Degraded
        } else {
            ReplicationStatus::Unknown
        };
    }
}

impl ShardMetadataTrait for NetworkShardMetadata {
    fn shard_id(&self) -> &ShardId { &self.core.shard_id }
    fn size_bytes(&self) -> u64 { self.core.size_bytes }
    fn created_ns(&self) -> u64 { self.core.created_ns }
    fn content_type(&self) -> Option<&str> { self.core.content_type.as_deref() }
    fn layer(&self) -> ShardLayer { ShardLayer::Network }
    fn owner(&self) -> Option<&BlissId> { self.core.owner.as_ref() }
    fn tags(&self) -> &[String] { &self.core.tags }
    fn geometry(&self) -> LatticeGeometry { self.core.geometry }
}

// ═══════════════════════════════════════════════════════════════════
// Legacy ShardMetadata (Backward Compatibility)
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardMetadata {
    pub shard_id: ShardId,
    pub size_bytes: u64,
    pub content_type: Option<String>,
    pub parent_shard: Option<ShardId>,
    pub child_shards: Vec<ShardId>,
    pub peer_nodes: HashSet<String>,
    pub replication_status: ReplicationStatus,
    pub tags: Vec<String>,
    pub recursion_level: u32,
    pub last_audit_ns: u64,
    pub created_ns: u64,
    pub owner: Option<BlissId>,
    pub audit_hashes: BTreeMap<String, Vec<u8>>,
    pub layer_idx: Option<u32>,
    
    // Legacy support for new geometry field
    #[serde(default)]
    pub geometry: LatticeGeometry,
}

impl ShardMetadata {
    pub fn new(shard_id: ShardId, size_bytes: u64, content_type: Option<String>) -> Self {
        let now_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        Self {
            shard_id,
            size_bytes,
            content_type,
            parent_shard: None,
            child_shards: Vec::new(),
            peer_nodes: HashSet::new(),
            replication_status: ReplicationStatus::Unknown,
            tags: Vec::new(),
            recursion_level: 0,
            last_audit_ns: 0,
            created_ns: now_ns,
            owner: None,
            audit_hashes: BTreeMap::new(),
            layer_idx: None,
            geometry: LatticeGeometry::FlowerOfLife, // Legacy defaults to Source
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.size_bytes == 0 && !self.child_shards.is_empty() {
            return Err("Non-empty shard cannot have zero size".to_string());
        }
        if let Some(ref parent) = self.parent_shard {
            if parent == &self.shard_id {
                return Err("Shard cannot be its own parent".to_string());
            }
        }
        if self.recursion_level > 1000 {
            return Err("Recursion level too deep".to_string());
        }
        Ok(())
    }

    pub fn add_peer(&mut self, peer_id: String) {
        self.peer_nodes.insert(peer_id);
    }
    pub fn remove_peer(&mut self, peer_id: &str) {
        self.peer_nodes.remove(peer_id);
    }
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) { self.tags.push(tag); }
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
    pub fn touch_audit(&mut self) {
        self.last_audit_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
    }
    pub fn needs_audit(&self, threshold_secs: i64) -> bool {
        let now_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        if self.last_audit_ns == 0 { return true; }
        let last_audit_secs = (self.last_audit_ns / 1_000_000_000) as i64;
        (now_ns - last_audit_secs) >= threshold_secs
    }
    pub fn is_root(&self) -> bool { self.parent_shard.is_none() }
    pub fn is_leaf(&self) -> bool { self.child_shards.is_empty() }
    pub fn update_replication_status(&mut self, desired_replication: usize) {
        let count = self.peer_nodes.len();
        self.replication_status = if count >= desired_replication {
            ReplicationStatus::Healthy
        } else {
            ReplicationStatus::Degraded
        };
    }
}

impl Default for ShardMetadata {
    fn default() -> Self {
        Self::new(ShardId::default(), 0, None)
    }
}

impl PartialEq for ShardMetadata {
    fn eq(&self, other: &Self) -> bool { self.shard_id == other.shard_id }
}

impl Eq for ShardMetadata {}

impl ShardMetadataTrait for ShardMetadata {
    fn shard_id(&self) -> &ShardId { &self.shard_id }
    fn size_bytes(&self) -> u64 { self.size_bytes }
    fn created_ns(&self) -> u64 { self.created_ns }
    fn content_type(&self) -> Option<&str> { self.content_type.as_deref() }
    fn layer(&self) -> ShardLayer { ShardLayer::Object }
    fn owner(&self) -> Option<&BlissId> { self.owner.as_ref() }
    fn tags(&self) -> &[String] { &self.tags }
    fn geometry(&self) -> LatticeGeometry { self.geometry }
}

// ═══════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_metadata() {
        let shard_id = ShardId::from_content(b"test");
        let core = CoreShardMetadata::new(shard_id.clone(), 100, ShardLayer::Storage)
            .with_content_type("application/json")
            .with_tag("important");

        assert_eq!(core.shard_id(), &shard_id);
        assert_eq!(core.size_bytes(), 100);
        assert_eq!(core.layer(), ShardLayer::Storage);
        // Verify automatic bio-resonance assignment
        assert_eq!(core.geometry, LatticeGeometry::Bethe); 
        assert_eq!(core.tags(), &["important".to_string()]);
    }

    #[test]
    fn test_storage_metadata() {
        let shard_id = ShardId::from_content(b"storage test");
        let meta = StorageShardMetadata::new(shard_id, 1024)
            .with_erasure(8, 12);

        assert_eq!(meta.erasure_k, 8);
        assert_eq!(meta.erasure_n, 12);
        assert_eq!(meta.layer(), ShardLayer::Storage);
        assert_eq!(meta.core.geometry, LatticeGeometry::Bethe); // Root Chakra
    }

    #[test]
    fn test_network_metadata() {
        let shard_id = ShardId::from_content(b"network test");
        let mut meta = NetworkShardMetadata::new(shard_id, 1024);

        meta.add_peer("peer1".to_string());
        meta.add_peer("peer2".to_string());
        meta.update_replication_status(3);

        assert_eq!(meta.peer_locations.len(), 2);
        assert_eq!(meta.replication_status, ReplicationStatus::Degraded);
        assert_eq!(meta.core.geometry, LatticeGeometry::Triangular); // Sacral Chakra
    }
    
    #[test]
    fn test_compute_metadata() {
        let shard_id = ShardId::from_content(b"compute test");
        let meta = ComputeShardMetadata::new(shard_id, 2048);
        
        assert_eq!(meta.core.geometry, LatticeGeometry::Kagome); // Solar Plexus
        assert_eq!(meta.precision, "f32");
    }

    #[test]
    fn test_legacy_metadata_compatibility() {
        let shard_id = ShardId::from_content(b"legacy test");
        let mut meta = ShardMetadata::new(shard_id, 100, Some("text/plain".to_string()));

        meta.add_peer("peer1".to_string());
        meta.add_tag("legacy".to_string());

        assert!(meta.validate().is_ok());
        assert!(meta.is_root());
        assert!(meta.is_leaf());
        assert_eq!(meta.geometry, LatticeGeometry::FlowerOfLife); // Legacy defaults to Dream
    }
}