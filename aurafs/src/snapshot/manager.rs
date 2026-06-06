//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS Snapshot Manager
//!
//! Point-in-time snapshots with copy-on-write, delta storage,
//! and consensus-driven commit for distributed version control.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tracing::{debug, error, info, warn};

#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("Snapshot not found: {0}")]
    SnapshotNotFound(String),
    
    #[error("Snapshot already exists: {0}")]
    SnapshotExists(String),
    
    #[error("Invalid snapshot data: {0}")]
    InvalidData(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, SnapshotError>;

/// Snapshot ID type
pub type SnapshotId = String;

/// Snapshot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub timestamp: u64,
    pub description: String,
    pub data_hash: String,
    pub parent: Option<SnapshotId>,
    pub delta_path: Option<String>,
}

/// Snapshot Manager
pub struct SnapshotManager {
    snapshots: Arc<RwLock<HashMap<SnapshotId, Snapshot>>>,
    storage_dir: String,
    deltas_dir: String,
}

impl SnapshotManager {
    /// Create new snapshot manager
    pub fn new(storage_dir: &str) -> Result<Self> {
        fs::create_dir_all(storage_dir)?;
        
        let deltas_dir = PathBuf::from(storage_dir).join("deltas");
        fs::create_dir_all(&deltas_dir)?;
        
        let manager = Self {
            snapshots: Arc::new(RwLock::new(HashMap::new())),
            storage_dir: storage_dir.to_string(),
            deltas_dir: deltas_dir.to_string_lossy().to_string(),
        };
        
        manager.load()?;
        info!("Initialized snapshot manager at {}", storage_dir);
        
        Ok(manager)
    }
    
    /// Compute hash of data path
    fn hash_data_path<P: AsRef<Path>>(path: P) -> Result<String> {
        let mut hasher = Sha256::new();
        
        if path.as_ref().is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let file_path = entry.path();
                
                if file_path.is_file() {
                    let data = fs::read(&file_path)?;
                    hasher.update(&data);
                }
            }
        } else if path.as_ref().is_file() {
            let data = fs::read(path)?;
            hasher.update(&data);
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }
    
    /// Compute delta between snapshots
    fn compute_delta(&self, parent_id: &str, child_id: &str) -> Result<String> {
        let delta_file = PathBuf::from(&self.deltas_dir)
            .join(format!("{}_to_{}.delta", parent_id, child_id));
        
        // Simplified delta - in production use binary diff algorithm
        let delta_content = format!("Delta from {} to {}", parent_id, child_id);
        fs::write(&delta_file, delta_content)?;
        
        Ok(delta_file.to_string_lossy().to_string())
    }
    
    /// Consensus commit placeholder
    fn consensus_commit(&self, snapshot: &Snapshot) -> bool {
        // TODO: Integrate with RAFT/PBFT consensus
        info!("Consensus commit for snapshot: {}", snapshot.id);
        true
    }
    
    /// Create new snapshot
    pub fn create_snapshot<P: AsRef<Path>>(
        &self,
        data_path: P,
        description: &str,
        parent_id: Option<SnapshotId>,
    ) -> Result<SnapshotId> {
        let data_hash = Self::hash_data_path(&data_path)?;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let snapshot_id = format!("{:x}", Sha256::digest(
            format!("{}-{}", timestamp, description).as_bytes()
        ))[..16].to_string();
        
        let delta_path = if let Some(ref parent) = parent_id {
            Some(self.compute_delta(parent, &snapshot_id)?)
        } else {
            None
        };
        
        let snapshot = Snapshot {
            id: snapshot_id.clone(),
            timestamp,
            description: description.to_string(),
            data_hash,
            parent: parent_id,
            delta_path,
        };
        
        if !self.consensus_commit(&snapshot) {
            return Err(SnapshotError::InvalidData("Consensus failed".to_string()));
        }
        
        let mut snapshots = self.snapshots.write().unwrap();
        snapshots.insert(snapshot_id.clone(), snapshot);
        
        self.save()?;
        info!("Created snapshot: {}", snapshot_id);
        
        Ok(snapshot_id)
    }
    
    /// Get snapshot by ID
    pub fn get_snapshot(&self, id: &str) -> Result<Snapshot> {
        let snapshots = self.snapshots.read().unwrap();
        snapshots.get(id)
            .cloned()
            .ok_or_else(|| SnapshotError::SnapshotNotFound(id.to_string()))
    }
    
    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<Snapshot> {
        let snapshots = self.snapshots.read().unwrap();
        let mut list: Vec<_> = snapshots.values().cloned().collect();
        list.sort_by_key(|s| s.timestamp);
        list.reverse();
        list
    }
    
    /// Rollback to snapshot
    pub fn rollback(&self, snapshot_id: &str) -> Result<()> {
        let snapshots = self.snapshots.read().unwrap();
        
        if !snapshots.contains_key(snapshot_id) {
            return Err(SnapshotError::SnapshotNotFound(snapshot_id.to_string()));
        }
        
        // TODO: Implement actual rollback logic
        info!("Rollback to snapshot: {}", snapshot_id);
        Ok(())
    }
    
    /// Save snapshots to disk
    fn save(&self) -> Result<()> {
        let snapshots = self.snapshots.read().unwrap();
        let path = PathBuf::from(&self.storage_dir).join("snapshots.json");
        
        let data = serde_json::to_string_pretty(&*snapshots)
            .map_err(|e| SnapshotError::SerializationError(e.to_string()))?;
        
        fs::write(path, data)?;
        Ok(())
    }
    
    /// Load snapshots from disk
    fn load(&self) -> Result<()> {
        let path = PathBuf::from(&self.storage_dir).join("snapshots.json");
        
        if !path.exists() {
            return Ok(());
        }
        
        let data = fs::read_to_string(path)?;
        let loaded: HashMap<SnapshotId, Snapshot> = serde_json::from_str(&data)
            .map_err(|e| SnapshotError::SerializationError(e.to_string()))?;
        
        let mut snapshots = self.snapshots.write().unwrap();
        *snapshots = loaded;
        
        info!("Loaded {} snapshots", snapshots.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_snapshot_creation() {
        let manager = SnapshotManager::new("/tmp/aurafs_snapshots").unwrap();
        let snap_id = manager.create_snapshot("/tmp/test_data", "Test snapshot", None).unwrap();
        
        let snapshot = manager.get_snapshot(&snap_id).unwrap();
        assert_eq!(snapshot.description, "Test snapshot");
    }
}