//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Inode - Fractal Metadata Engine with Quantum Security
//! 🗄️ Permissioned Shards + Merkle-Patricia Child Maps + Lifecycles
//! 
//! ⚛️  Lattice Physics: Maps Logical Paths to Physical Lattice Geometries
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{ShardId, ShardMetadata, Shard, metadata::LatticeGeometry},
    gov::{BlissId, SoulProof, SoulACL},
    crypto::hash::Blake3Digest,
};
use std::{
    collections::BTreeMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Unique identifier for an inode (Blake3 hash of its genesis state)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InodeId(pub Blake3Digest);

impl InodeId {
    /// Create a new random (unique) inode ID
    pub fn new() -> Self {
        Self(Blake3Digest::random())
    }
    
    /// Create from existing bytes (e.g., recovered from WAL)
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(Blake3Digest(bytes))
    }
}

impl Default for InodeId {
    fn default() -> Self {
        Self(Blake3Digest::default())
    }
}

impl std::fmt::Display for InodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0.0))
    }
}

/// Timestamps for inode lifecycle events (Nano-precision)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InodeTimestamps {
    pub created_ns: u64,
    pub modified_ns: u64,
    pub accessed_ns: u64,
    pub audited_ns: u64,
}

impl InodeTimestamps {
    /// Create timestamps with current time for all events
    pub fn now() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_default().as_nanos() as u64;
        Self {
            created_ns: now,
            modified_ns: now,
            accessed_ns: now,
            audited_ns: now,
        }
    }
    
    /// Update modified time to now
    pub fn touch_modified(&mut self) {
        self.modified_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
    }
    
    /// Update accessed time to now
    pub fn touch_accessed(&mut self) {
        self.accessed_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
    }
    
    /// Update audited time to now
    pub fn touch_audited(&mut self) {
        self.audited_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
    }
}

/// Fractal inode structure representing files/directories.
/// This is the "Atom" of the filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inode {
    /// Unique Inode ID
    pub id: InodeId,
    
    /// Content Shard ID (Pointer to raw data)
    pub shard_id: ShardId,
    
    /// Metadata (Size, Type, Geometry, Tags)
    pub metadata: ShardMetadata,
    
    /// Soul Access Control List (Owner/Permissions)
    pub permissions: SoulACL,
    
    /// Child inodes: Merkle-Patricia Tree (path component → InodeId)
    /// 
    pub children: BTreeMap<String, InodeId>,
    
    /// Extended attributes (xattrs) for custom metadata
    pub xattrs: BTreeMap<String, Vec<u8>>,
    
    /// Lifecycle timestamps
    pub timestamps: InodeTimestamps,
}

impl Inode {
    /// Create new file inode (Default Geometry: FlowerOfLife)
    pub fn new_file(data: Vec<u8>, permissions: SoulACL) -> Self {
        // Default to FlowerOfLife (Superposition) for new files
        Self::new_file_with_geometry(data, permissions, LatticeGeometry::FlowerOfLife)
    }

    /// ✨ Phase II: Create file with specific Lattice Geometry
    pub fn new_file_with_geometry(
        data: Vec<u8>, 
        permissions: SoulACL, 
        geometry: LatticeGeometry
    ) -> Self {
        // Generate Shard ID from content (Content-Addressable)
        let shard_id = ShardId::from_content(&data);
        
        let mut metadata = ShardMetadata::new(
            shard_id.clone(),
            data.len() as u64,
            Some("application/octet-stream".to_string()),
        );
        
        // Apply Physics
        metadata.geometry = geometry;

        Self {
            id: InodeId::new(),
            shard_id,
            metadata,
            permissions,
            children: BTreeMap::new(),
            xattrs: BTreeMap::new(),
            timestamps: InodeTimestamps::now(),
        }
    }
    
    /// Create new directory inode (Default Geometry: Bethe Lattice)
    /// Directories naturally map to the Bethe Lattice (Tree Structure).
    pub fn new_dir(permissions: SoulACL) -> Self {
        let mut metadata = ShardMetadata::default();
        metadata.content_type = Some("directory".to_string());
        
        // ✨ Phase II: Enforce Bethe Lattice for Directory Trees
        metadata.geometry = LatticeGeometry::Bethe;

        Self {
            id: InodeId::new(),
            shard_id: ShardId::default(), // Directories might not have data shards initially
            metadata,
            permissions,
            children: BTreeMap::new(),
            xattrs: BTreeMap::new(),
            timestamps: InodeTimestamps::now(),
        }
    }
    
    /// Check if inode is a directory
    pub fn is_dir(&self) -> bool {
        self.metadata.content_type.as_deref() == Some("directory")
    }
    
    /// Add child inode (Updates Merkle Map)
    pub fn add_child(&mut self, name: String, child_id: InodeId) {
        self.children.insert(name, child_id);
        self.timestamps.touch_modified();
    }
    
    /// Remove child inode
    pub fn remove_child(&mut self, name: &str) {
        self.children.remove(name);
        self.timestamps.touch_modified();
    }
    
    /// Update extended attribute
    pub fn set_xattr(&mut self, key: String, value: Vec<u8>) {
        self.xattrs.insert(key, value);
        self.timestamps.touch_modified();
    }
    
    /// Get extended attribute
    pub fn get_xattr(&self, key: &str) -> Option<&Vec<u8>> {
        self.xattrs.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gov::SoulACL;

    #[test]
    fn test_inode_creation_and_children() {
        let perms = SoulACL::default(); // Use default for test
        
        // Create Directory (Should be Bethe)
        let mut dir_inode = Inode::new_dir(perms.clone());
        assert!(dir_inode.is_dir());
        assert_eq!(dir_inode.metadata.geometry, LatticeGeometry::Bethe);
        
        // Create File (Default)
        let file_inode = Inode::new_file(b"test data".to_vec(), perms.clone());
        assert_eq!(file_inode.metadata.geometry, LatticeGeometry::FlowerOfLife);
        
        // Create File (Kagome - Compute)
        let compute_inode = Inode::new_file_with_geometry(
            b"AI weights".to_vec(), 
            perms.clone(), 
            LatticeGeometry::Kagome
        );
        assert_eq!(compute_inode.metadata.geometry, LatticeGeometry::Kagome);
        
        // Test Hierarchy
        dir_inode.add_child("file.txt".to_string(), file_inode.id.clone());
        assert!(dir_inode.children.contains_key("file.txt"));
        
        dir_inode.remove_child("file.txt");
        assert!(!dir_inode.children.contains_key("file.txt"));
    }
}