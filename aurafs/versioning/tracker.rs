//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS Versioning Tracker
//!
//! Shard version history with automatic versioning, conflict resolution,
//! and distributed version control for fractal shards.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tracing::{debug, info, warn};

use crate::core::shard::ShardId;

#[derive(Debug, Error)]
pub enum VersionError {
    #[error("Version not found: {0}")]
    VersionNotFound(u64),
    
    #[error("Shard not tracked: {0}")]
    ShardNotTracked(String),
    
    #[error("Conflict detected")]
    ConflictDetected,
}

pub type Result<T> = std::result::Result<T, VersionError>;

/// Version metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub version_number: u64,
    pub shard_id: ShardId,
    pub timestamp: u64,
    pub author: String,
    pub message: String,
    pub data_hash: String,
    pub parent_version: Option<u64>,
}

/// Shard version history
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShardHistory {
    shard_id: ShardId,
    versions: Vec<Version>,
    current_version: u64,
}

/// Versioning Tracker
pub struct VersionTracker {
    histories: Arc<RwLock<HashMap<ShardId, ShardHistory>>>,
}

impl VersionTracker {
    /// Create new version tracker
    pub fn new() -> Self {
        info!("Initialized version tracker");
        
        Self {
            histories: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Track new shard
    pub fn track_shard(&self, shard_id: ShardId) {
        let mut histories = self.histories.write().unwrap();
        
        if !histories.contains_key(&shard_id) {
            histories.insert(shard_id.clone(), ShardHistory {
                shard_id: shard_id.clone(),
                versions: Vec::new(),
                current_version: 0,
            });
            
            info!("Now tracking shard: {:?}", shard_id);
        }
    }
    
    /// Create new version
    pub fn create_version(
        &self,
        shard_id: &ShardId,
        author: &str,
        message: &str,
        data_hash: &str,
    ) -> Result<u64> {
        let mut histories = self.histories.write().unwrap();
        
        let history = histories.get_mut(shard_id)
            .ok_or_else(|| VersionError::ShardNotTracked(format!("{:?}", shard_id)))?;
        
        let version_number = history.versions.len() as u64 + 1;
        let parent_version = if version_number > 1 {
            Some(version_number - 1)
        } else {
            None
        };
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let version = Version {
            version_number,
            shard_id: shard_id.clone(),
            timestamp,
            author: author.to_string(),
            message: message.to_string(),
            data_hash: data_hash.to_string(),
            parent_version,
        };
        
        history.versions.push(version);
        history.current_version = version_number;
        
        info!("Created version {} for shard {:?}", version_number, shard_id);
        
        Ok(version_number)
    }
    
    /// Get version
    pub fn get_version(&self, shard_id: &ShardId, version_number: u64) -> Result<Version> {
        let histories = self.histories.read().unwrap();
        
        let history = histories.get(shard_id)
            .ok_or_else(|| VersionError::ShardNotTracked(format!("{:?}", shard_id)))?;
        
        history.versions.iter()
            .find(|v| v.version_number == version_number)
            .cloned()
            .ok_or(VersionError::VersionNotFound(version_number))
    }
    
    /// Get version history
    pub fn get_history(&self, shard_id: &ShardId) -> Result<Vec<Version>> {
        let histories = self.histories.read().unwrap();
        
        let history = histories.get(shard_id)
            .ok_or_else(|| VersionError::ShardNotTracked(format!("{:?}", shard_id)))?;
        
        Ok(history.versions.clone())
    }
    
    /// Get current version
    pub fn get_current_version(&self, shard_id: &ShardId) -> Result<u64> {
        let histories = self.histories.read().unwrap();
        
        let history = histories.get(shard_id)
            .ok_or_else(|| VersionError::ShardNotTracked(format!("{:?}", shard_id)))?;
        
        Ok(history.current_version)
    }
    
    /// Checkout specific version
    pub fn checkout(&self, shard_id: &ShardId, version_number: u64) -> Result<Version> {
        let version = self.get_version(shard_id, version_number)?;
        
        info!("Checked out version {} for shard {:?}", version_number, shard_id);
        
        Ok(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_tracking() {
        let tracker = VersionTracker::new();
        let shard_id = ShardId::new(b"test-shard-data").unwrap();
        
        tracker.track_shard(shard_id.clone());
        let v1 = tracker.create_version(&shard_id, "user", "Initial", "hash1").unwrap();
        let v2 = tracker.create_version(&shard_id, "user", "Update", "hash2").unwrap();
        
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        
        let history = tracker.get_history(&shard_id).unwrap();
        assert_eq!(history.len(), 2);
    }
}