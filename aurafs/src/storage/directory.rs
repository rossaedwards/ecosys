//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Directory - Merkle-Patricia Production Tree Engine
//! 🗄️ Hierarchical Namespaces + Soul Ownership + Fractal Resolution
//! 
//! ⚛️  Lattice Physics: Directories map to the "Bethe Lattice" (Tree).
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    storage::inode::{InodeId, Inode},
    gov::{BlissId, SoulACL},
    shard::{ShardId, ShardMetadata, metadata::LatticeGeometry},
    crypto::hash::Blake3Digest,
};
use std::{
    collections::BTreeMap,
    sync::Arc,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Merkle-Patricia directory tree node (fractal namespace).
/// Represents a node in the Bethe Lattice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    /// Directory name/path component
    pub name: String,
    
    /// Soul owner with full governance
    pub owner: BlissId,
    
    /// Child inodes: Merkle-Patricia Trie (name → InodeId)
    pub inodes: BTreeMap<String, InodeId>,
    
    /// Parent directory reference (for tree walking)
    pub parent: Option<InodeId>,
    
    /// Directory metadata shard (Includes Geometry)
    pub metadata: ShardMetadata,
    
    /// Directory ACL
    pub permissions: SoulACL,
}

impl Directory {
    /// Create root directory (Genesis Namespace)
    pub fn new_root() -> Self {
        let mut metadata = ShardMetadata::default();
        metadata.content_type = Some("directory".to_string());
        
        // ✨ Phase II: Root is the anchor of the Bethe Lattice
        metadata.geometry = LatticeGeometry::Bethe;

        Self {
            name: "/".to_string(),
            owner: BlissId::genesis(),
            inodes: BTreeMap::new(),
            parent: None,
            metadata,
            permissions: SoulACL::root(),
        }
    }
    
    /// Add child inode to directory (Extending the Tree)
    pub fn add_inode(&mut self, name: String, inode_id: InodeId) -> bool {
        // In a Bethe lattice, adding a node increases local connectivity
        if self.inodes.insert(name.clone(), inode_id).is_none() {
            true // New entry
        } else {
            false // Overwritten
        }
    }
    
    /// Remove child inode from directory
    pub fn remove_inode(&mut self, name: &str) -> Option<InodeId> {
        self.inodes.remove(name)
    }
    
    /// Resolve child inode by name (Merkle lookup)
    pub fn get_inode(&self, name: &str) -> Option<&InodeId> {
        self.inodes.get(name)
    }
    
    /// Check if directory is empty
    pub fn is_empty(&self) -> bool {
        self.inodes.is_empty()
    }
    
    /// List all child names
    pub fn list_children(&self) -> Vec<String> {
        self.inodes.keys().cloned().collect()
    }
    
    /// Compute Merkle root hash of directory tree
    /// ✨ Phase II: Includes Lattice Geometry in the hash
    pub fn merkle_root(&self) -> Blake3Digest {
        let mut hasher = blake3::Hasher::new();
        
        // Mix in Lattice Type
        hasher.update(format!("{:?}", self.metadata.geometry).as_bytes());
        
        for (name, inode_id) in &self.inodes {
            hasher.update(name.as_bytes());
            hasher.update(&inode_id.0.0); // Assuming InodeId wraps Blake3Digest(pub [u8; 32]) or similar
        }
        
        Blake3Digest::from_hash(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_operations() {
        let mut root = Directory::new_root();
        let file_id = InodeId::new();
        
        // Verify Phase II Lattice Type
        assert_eq!(root.metadata.geometry, LatticeGeometry::Bethe);
        
        assert!(root.add_inode("file.txt".to_string(), file_id.clone()));
        assert_eq!(root.get_inode("file.txt"), Some(&file_id));
        assert_eq!(root.list_children(), vec!["file.txt".to_string()]);
        
        // Verify Merkle Root changes
        let root_hash_1 = root.merkle_root();
        
        assert_eq!(root.remove_inode("file.txt"), Some(file_id));
        assert!(root.is_empty());
        
        let root_hash_2 = root.merkle_root();
        assert_ne!(root_hash_1, root_hash_2);
    }
}