//! namespace/shard.rs - Quantum-Aware Namespace Shard Lifecycle
//! f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division
//! Manages shards linked to namespace entries, quantum sealing and replication state

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use crate::namespace::{NamespaceEntry, SharedNamespace, NamespaceResult, NamespaceError};
use crate::core::shard::{ShardId};
use tracing::{debug, info, warn};

/// Metadata about the shard linked to a NamespaceEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceShard {
    pub shard_id: ShardId,
    pub sealed: bool,                  // Quantum sealed flag
    pub quantum_entropy: u8,           // 0-255 quantum entropy measure
    pub replica_nodes: Vec<String>,    // Peers hosting replicas of this shard
    pub last_modified: DateTime<Utc>,
}

impl NamespaceShard {
    /// Create new shard metadata with quantum seal flag off
    pub fn new(shard_id: ShardId) -> Self {
        NamespaceShard {
            shard_id,
            sealed: false,
            quantum_entropy: 0,
            replica_nodes: Vec::new(),
            last_modified: Utc::now(),
        }
    }

    /// Mark shard as quantum sealed with entropy
    pub fn seal(&mut self, entropy: u8) {
        self.sealed = true;
        self.quantum_entropy = entropy;
        self.last_modified = Utc::now();
    }

    /// Add a replica node to the shard
    pub fn add_replica(&mut self, node_id: String) {
        if !self.replica_nodes.contains(&node_id) {
            self.replica_nodes.push(node_id);
            self.last_modified = Utc::now();
        }
    }

    /// Remove a replica node
    pub fn remove_replica(&mut self, node_id: &str) {
        self.replica_nodes.retain(|n| n != node_id);
        self.last_modified = Utc::now();
    }
}

/// Shard-aware namespace manager extension trait
#[async_trait::async_trait]
pub trait NamespaceShardManager {
    /// Attach a new shard to a namespace path
    async fn attach_shard(&self, path: &str, shard_id: ShardId) -> NamespaceResult<()>;

    /// Seal a shard at the given namespace path with quantum entropy
    async fn seal_shard(&self, path: &str, entropy: u8) -> NamespaceResult<()>;

    /// Add replica node hosting the shard linked to the path
    async fn add_replica_node(&self, path: &str, node_id: String) -> NamespaceResult<()>;

    /// Query shard metadata for a namespace path
    async fn get_shard_metadata(&self, path: &str) -> NamespaceResult<Option<NamespaceShard>>;
}

/// Concrete shard namespace manager implementation
pub struct ShardNamespaceManager {
    namespace: SharedNamespace,
}

impl ShardNamespaceManager {
    /// Forge new shard namespace manager wrapper
    pub fn new(namespace: SharedNamespace) -> Self {
        Self { namespace }
    }
}

#[async_trait::async_trait]
impl NamespaceShardManager for ShardNamespaceManager {
    async fn attach_shard(&self, path: &str, shard_id: ShardId) -> NamespaceResult<()> {
        let mut ns = self.namespace.write().await;
        let entry = ns.get_entry_mut(path)
            .ok_or_else(|| NamespaceError::PathNotFound(path.to_string()))?;
        entry.shard_id = Some(shard_id.clone());

        // Create default shard metadata attached to entry's metadata map
        let shard_meta = NamespaceShard::new(shard_id);
        entry.metadata.insert("shard_meta".to_string(), serde_json::to_value(&shard_meta)?);

        ns.save().await?;
        info!("Attached shard {} to path {}", shard_meta.shard_id.to_hex(), path);
        Ok(())
    }

    async fn seal_shard(&self, path: &str, entropy: u8) -> NamespaceResult<()> {
        let mut ns = self.namespace.write().await;
        let entry = ns.get_entry_mut(path)
            .ok_or_else(|| NamespaceError::PathNotFound(path.to_string()))?;
        let shard_id = entry.shard_id.clone()
            .ok_or_else(|| NamespaceError::NotAShard(path.to_string()))?;

        let shard_meta_val = entry.metadata.get_mut("shard_meta")
            .ok_or_else(|| NamespaceError::NotAShard(path.to_string()))?;
        let mut shard_meta: NamespaceShard = serde_json::from_value(shard_meta_val.clone())?;

        shard_meta.seal(entropy);
        *shard_meta_val = serde_json::to_value(&shard_meta)?;

        ns.save().await?;
        info!("Quantum sealed shard {} at path {} with entropy {}", shard_id.to_hex(), path, entropy);
        Ok(())
    }

    async fn add_replica_node(&self, path: &str, node_id: String) -> NamespaceResult<()> {
        let mut ns = self.namespace.write().await;
        let entry = ns.get_entry_mut(path)
            .ok_or_else(|| NamespaceError::PathNotFound(path.to_string()))?;

        let shard_meta_val = entry.metadata.get_mut("shard_meta")
            .ok_or_else(|| NamespaceError::NotAShard(path.to_string()))?;
        let mut shard_meta: NamespaceShard = serde_json::from_value(shard_meta_val.clone())?;

        shard_meta.add_replica(node_id);
        *shard_meta_val = serde_json::to_value(&shard_meta)?;

        ns.save().await?;
        info!("Added replica node to shard at path {}", path);
        Ok(())
    }

    async fn get_shard_metadata(&self, path: &str) -> NamespaceResult<Option<NamespaceShard>> {
        let ns = self.namespace.read().await;
        let entry = ns.get_entry(path).ok_or_else(|| NamespaceError::PathNotFound(path.to_string()))?;

        if let Some(meta_val) = entry.metadata.get("shard_meta") {
            let shard_meta: NamespaceShard = serde_json::from_value(meta_val.clone())?;
            return Ok(Some(shard_meta));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namespace::manager::NamespaceManager;
    use tempfile::tempdir;
    use crate::core::shard::ShardId;

    #[tokio::test]
    async fn test_attach_seal_add_replica() {
        let dir = tempdir().unwrap();
        let ns_mgr = Arc::new(RwLock::new(NamespaceManager::new(dir.path().to_str().unwrap()).unwrap()));
        let shard_mgr = ShardNamespaceManager::new(ns_mgr.clone());

        let test_path = "/test/file.txt";
        let shard_id = ShardId::new_random();

        ns_mgr.write().await.create_file(test_path, "alice".to_string(), shard_id.clone()).unwrap();

        shard_mgr.attach_shard(test_path, shard_id.clone()).await.unwrap();

        let shard_meta = shard_mgr.get_shard_metadata(test_path).await.unwrap();
        assert!(shard_meta.is_some());
        assert_eq!(shard_meta.as_ref().unwrap().shard_id, shard_id);

        shard_mgr.seal_shard(test_path, 128).await.unwrap();
        let sealed_meta = shard_mgr.get_shard_metadata(test_path).await.unwrap().unwrap();
        assert!(sealed_meta.sealed);
        assert_eq!(sealed_meta.quantum_entropy, 128);

        shard_mgr.add_replica_node(test_path, "node01".to_string()).await.unwrap();
        let updated_meta = shard_mgr.get_shard_metadata(test_path).await.unwrap().unwrap();
        assert_eq!(updated_meta.replica_nodes.len(), 1);
        assert_eq!(updated_meta.replica_nodes[0], "node01".to_string());
    }
}