//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Snapshot - Fractal State Freezing & Time Travel
//! 📸 Instant CoW (Copy-on-Write) + Merkle Roots + Soul Retention
//! 
//! ⚛️  Lattice Physics: Collapses the Wavefunction at time `t`,
//!     preserving the Geometry state for future observation.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    storage::{inode::InodeId, directory::Directory},
    gov::BlissId,
    crypto::hash::Blake3Digest,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;
use thiserror::Error;
use tracing::{info, debug};

/// Unique identifier for a snapshot (Hash of the root state)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub Blake3Digest);

impl SnapshotId {
    /// Create new snapshot ID from root hash
    pub fn new(root_hash: Blake3Digest) -> Self {
        // In reality, we might mix timestamp/nonce, but root hash is good for dedup
        Self(root_hash)
    }
}

/// A frozen moment in the Lattice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// The unique ID of this timeline
    pub id: SnapshotId,
    
    /// The Root Inode of the file system at this moment
    /// (Points to the top of the Bethe Lattice)
    pub root_inode: InodeId,
    
    /// The Merkle Root of the directory tree
    pub merkle_root: Blake3Digest,
    
    /// Who triggered the observation (collapse)
    pub creator: BlissId,
    
    /// When the wave function collapsed
    pub timestamp_ns: u64,
    
    /// Tags/Label (e.g., "pre-migration", "daily-backup")
    pub label: String,
}

/// Manages the timelines (Snapshots) of the filesystem
pub struct SnapshotManager {
    /// Active snapshots (Timeline Registry)
    snapshots: Arc<RwLock<HashMap<SnapshotId, Snapshot>>>,
    
    /// Lookup by label (e.g., "latest" -> SnapshotId)
    labels: Arc<RwLock<HashMap<String, SnapshotId>>>,
}

#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("Snapshot not found")]
    NotFound,
    #[error("Snapshot already exists")]
    AlreadyExists,
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl SnapshotManager {
    /// Initialize the Time Machine
    pub fn new() -> Self {
        Self {
            snapshots: Arc::new(RwLock::new(HashMap::new())),
            labels: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new snapshot (Collapse Wavefunction)
    /// This is an O(1) operation because we just store the Root Inode ID.
    /// Since AuraFS Inodes are immutable/CoW, holding the Root ID preserves the entire tree.
    pub async fn create_snapshot(
        &self, 
        root_inode: InodeId, 
        merkle_root: Blake3Digest,
        creator: BlissId,
        label: Option<String>
    ) -> Result<SnapshotId, SnapshotError> {
        let id = SnapshotId::new(merkle_root.clone());
        
        let snapshot = Snapshot {
            id: id.clone(),
            root_inode,
            merkle_root,
            creator,
            timestamp_ns: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
            label: label.clone().unwrap_or_else(|| "unnamed".to_string()),
        };
        
        let mut snaps = self.snapshots.write().await;
        if snaps.contains_key(&id) {
            // Idempotent: If state is identical, we just return existing snapshot
            return Ok(id);
        }
        snaps.insert(id.clone(), snapshot);
        
        if let Some(l) = label {
            let mut labels = self.labels.write().await;
            labels.insert(l, id.clone());
        }
        
        info!("📸 Snapshot created: {} (Root: {:?})", id.0, root_inode);
        Ok(id)
    }
    
    /// Retrieve a snapshot by ID
    pub async fn get_snapshot(&self, id: &SnapshotId) -> Result<Snapshot, SnapshotError> {
        let snaps = self.snapshots.read().await;
        snaps.get(id).cloned().ok_or(SnapshotError::NotFound)
    }
    
    /// Retrieve a snapshot by Label
    pub async fn get_by_label(&self, label: &str) -> Result<Snapshot, SnapshotError> {
        let labels = self.labels.read().await;
        let id = labels.get(label).ok_or(SnapshotError::NotFound)?;
        self.get_snapshot(id).await
    }
    
    /// List all timelines
    pub async fn list_snapshots(&self) -> Vec<Snapshot> {
        let snaps = self.snapshots.read().await;
        let mut list: Vec<Snapshot> = snaps.values().cloned().collect();
        list.sort_by(|a, b| b.timestamp_ns.cmp(&a.timestamp_ns)); // Newest first
        list
    }
    
    /// Delete a snapshot (Prune Timeline)
    /// Note: This doesn't delete data immediately; Garbage Collection does that
    /// by checking if shards are referenced by ANY active snapshot.
    pub async fn delete_snapshot(&self, id: &SnapshotId) -> Result<(), SnapshotError> {
        let mut snaps = self.snapshots.write().await;
        if snaps.remove(id).is_none() {
            return Err(SnapshotError::NotFound);
        }
        
        // Cleanup labels
        let mut labels = self.labels.write().await;
        labels.retain(|_, v| v != id);
        
        info!("🗑️ Snapshot deleted: {}", id.0);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_time_travel() {
        let mgr = SnapshotManager::new();
        let root = InodeId::new();
        let merkle = Blake3Digest::hash_bytes(b"state 1");
        
        // 1. Create Baseline
        let snap1 = mgr.create_snapshot(root.clone(), merkle.clone(), BlissId::genesis(), Some("v1".into()))
            .await.unwrap();
            
        // 2. Evolve State
        let merkle2 = Blake3Digest::hash_bytes(b"state 2");
        let snap2 = mgr.create_snapshot(root.clone(), merkle2, BlissId::genesis(), Some("v2".into()))
            .await.unwrap();
            
        // 3. Verify Timelines
        let list = mgr.list_snapshots().await;
        assert_eq!(list.len(), 2);
        
        // 4. Resolve
        let loaded = mgr.get_by_label("v1").await.unwrap();
        assert_eq!(loaded.id, snap1);
    }
}