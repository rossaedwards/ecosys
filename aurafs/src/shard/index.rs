//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Index - High-performance Thread-safe Index
//! 🔍 Concurrent Access, Querying, Persistence & Stats
//! ═══════════════════════════════════════════════════════════════════

use crate::crypto::hash::Hash;
use crate::error::{RafsError, Result};
use crate::shard::metadata::{PeerId, ReplicationStatus, ShardId, ShardMetadata, LatticeGeometry, ShardMetadataTrait};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn, error};

/// Thread-safe shard index with concurrent access support
#[derive(Clone)]
pub struct ShardIndex {
    /// Main index: shard_id -> metadata
    shards: Arc<DashMap<ShardId, ShardMetadata>>,

    /// Secondary index: peer_id -> set of shard_ids
    peer_index: Arc<DashMap<PeerId, HashSet<ShardId>>>,

    /// Secondary index: parent_id -> set of child_ids
    hierarchy_index: Arc<DashMap<ShardId, HashSet<ShardId>>>,

    /// Secondary index: tag -> set of shard_ids
    tag_index: Arc<DashMap<String, HashSet<ShardId>>>,

    /// Statistics
    stats: Arc<IndexStats>,
}

/// Index statistics
#[derive(Debug)]
struct IndexStats {
    total_size: AtomicU64,
    shard_count: AtomicU64,
    root_count: AtomicU64,
    leaf_count: AtomicU64,
}

impl IndexStats {
    fn new() -> Self {
        Self {
            total_size: AtomicU64::new(0),
            shard_count: AtomicU64::new(0),
            root_count: AtomicU64::new(0),
            leaf_count: AtomicU64::new(0),
        }
    }

    fn increment_shard(&self, size: u64, is_root: bool, is_leaf: bool) {
        self.total_size.fetch_add(size, Ordering::Relaxed);
        self.shard_count.fetch_add(1, Ordering::Relaxed);
        if is_root {
            self.root_count.fetch_add(1, Ordering::Relaxed);
        }
        if is_leaf {
            self.leaf_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn decrement_shard(&self, size: u64, is_root: bool, is_leaf: bool) {
        self.total_size.fetch_sub(size, Ordering::Relaxed);
        self.shard_count.fetch_sub(1, Ordering::Relaxed);
        if is_root {
            self.root_count.fetch_sub(1, Ordering::Relaxed);
        }
        if is_leaf {
            self.leaf_count.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

/// Query builder for filtering shards
#[derive(Debug, Clone, Default)]
pub struct ShardQuery {
    peer_id: Option<PeerId>,
    parent_id: Option<ShardId>,
    tags: Vec<String>,
    min_size: Option<u64>,
    max_size: Option<u64>,
    recursion_level: Option<u32>,
    needs_audit: Option<i64>,
    replication_status: Option<ReplicationStatus>,
    content_type: Option<String>,
    
    /// 🟢 Phase II: Filter by Sacred Geometry (e.g., find all Kagome/Compute shards)
    geometry: Option<LatticeGeometry>,
}

impl ShardQuery {
    /// Create new empty query
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by peer ID
    pub fn peer(mut self, peer_id: PeerId) -> Self {
        self.peer_id = Some(peer_id);
        self
    }

    /// Filter by parent ID
    pub fn parent(mut self, parent_id: ShardId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Filter by tag
    pub fn tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Filter by minimum size
    pub fn min_size(mut self, size: u64) -> Self {
        self.min_size = Some(size);
        self
    }

    /// Filter by maximum size
    pub fn max_size(mut self, size: u64) -> Self {
        self.max_size = Some(size);
        self
    }

    /// Filter by recursion level
    pub fn level(mut self, level: u32) -> Self {
        self.recursion_level = Some(level);
        self
    }

    /// Filter by audit threshold (seconds)
    pub fn needs_audit(mut self, threshold_secs: i64) -> Self {
        self.needs_audit = Some(threshold_secs);
        self
    }

    /// Filter by replication status
    pub fn replication(mut self, status: ReplicationStatus) -> Self {
        self.replication_status = Some(status);
        self
    }

    /// Filter by content type
    pub fn content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// 🟢 Filter by Lattice Geometry (Phase II)
    pub fn geometry(mut self, geometry: LatticeGeometry) -> Self {
        self.geometry = Some(geometry);
        self
    }

    /// Check if metadata matches query
    fn matches(&self, metadata: &ShardMetadata) -> bool {
        // Check geometry (Phase II)
        if let Some(geo) = self.geometry {
            if metadata.geometry != geo {
                return false;
            }
        }

        // Check peer filter
        if let Some(ref peer_id) = self.peer_id {
            if !metadata.peer_nodes.contains(peer_id) {
                return false;
            }
        }

        // Check parent filter
        if let Some(ref parent_id) = self.parent_id {
            if metadata.parent_shard.as_ref() != Some(parent_id) {
                return false;
            }
        }

        // Check tag filters (must have all tags)
        for tag in &self.tags {
            if !metadata.has_tag(tag) {
                return false;
            }
        }

        // Check size filters
        if let Some(min) = self.min_size {
            if metadata.size_bytes < min {
                return false;
            }
        }
        if let Some(max) = self.max_size {
            if metadata.size_bytes > max {
                return false;
            }
        }

        // Check recursion level
        if let Some(level) = self.recursion_level {
            if metadata.recursion_level != level {
                return false;
            }
        }

        // Check audit threshold
        if let Some(threshold) = self.needs_audit {
            if !metadata.needs_audit(threshold) {
                return false;
            }
        }

        // Check replication status
        if let Some(ref status) = self.replication_status {
            if &metadata.replication_status != status {
                return false;
            }
        }

        // Check content type
        if let Some(ref ct) = self.content_type {
            if metadata.content_type.as_ref() != Some(ct) {
                return false;
            }
        }

        true
    }
}

impl ShardIndex {
    /// Create new empty shard index
    pub fn new() -> Self {
        Self {
            shards: Arc::new(DashMap::new()),
            peer_index: Arc::new(DashMap::new()),
            hierarchy_index: Arc::new(DashMap::new()),
            tag_index: Arc::new(DashMap::new()),
            stats: Arc::new(IndexStats::new()),
        }
    }

    /// Add shard to index with validation
    pub fn add_shard(&self, metadata: ShardMetadata) -> Result<()> {
        // Validate metadata before adding
        if let Err(e) = metadata.validate() {
            return Err(RafsError::InvalidShard(format!(
                "Metadata validation failed for shard {}: {}",
                metadata.shard_id, e
            )));
        }
        
        let shard_id = metadata.shard_id.clone();

        // Check for duplicates
        if self.shards.contains_key(&shard_id) {
            // Update existing entry instead of error
            warn!("Shard {} already exists in index, updating metadata", shard_id);
            return self.update_shard(metadata);
        }

        // Update peer index
        for peer_id in &metadata.peer_nodes {
            self.peer_index
                .entry(peer_id.clone())
                .or_insert_with(HashSet::new)
                .insert(shard_id.clone());
        }

        // Update hierarchy index
        if let Some(ref parent_id) = metadata.parent_shard {
            // Validate parent exists
            if !self.shards.contains_key(parent_id) {
                warn!(
                    "Parent shard {} not found in index for shard {} (may be added later)",
                    parent_id, shard_id
                );
            }
            // Validate parent is not the same as child (circular reference)
            if parent_id == &shard_id {
                return Err(RafsError::InvalidShard(format!(
                    "Circular reference detected: shard {} cannot be its own parent",
                    shard_id
                )));
            }
            self.hierarchy_index
                .entry(parent_id.clone())
                .or_insert_with(HashSet::new)
                .insert(shard_id.clone());
        }

        // Update tag index
        for tag in &metadata.tags {
            self.tag_index
                .entry(tag.clone())
                .or_insert_with(HashSet::new)
                .insert(shard_id.clone());
        }

        // Update statistics
        let is_root = metadata.is_root();
        let is_leaf = metadata.is_leaf();
        self.stats.increment_shard(metadata.size_bytes, is_root, is_leaf);

        // Add to main index
        self.shards.insert(shard_id, metadata);

        Ok(())
    }

    /// Get shard metadata by ID
    pub fn get_shard(&self, shard_id: &ShardId) -> Option<ShardMetadata> {
        self.shards.get(shard_id).map(|entry| entry.value().clone())
    }

    /// Check if shard exists
    pub fn contains(&self, shard_id: &ShardId) -> bool {
        self.shards.contains_key(shard_id)
    }

    /// Remove shard from index
    pub fn remove_shard(&self, shard_id: &ShardId) -> Option<ShardMetadata> {
        let removed = self.shards.remove(shard_id).map(|(_, meta)| meta);

        if let Some(ref metadata) = removed {
            // Update peer index
            for peer_id in &metadata.peer_nodes {
                if let Some(mut entry) = self.peer_index.get_mut(peer_id) {
                    entry.remove(shard_id);
                }
            }

            // Update hierarchy index
            if let Some(ref parent_id) = metadata.parent_shard {
                if let Some(mut entry) = self.hierarchy_index.get_mut(parent_id) {
                    entry.remove(shard_id);
                }
            }

            // Update tag index
            for tag in &metadata.tags {
                if let Some(mut entry) = self.tag_index.get_mut(tag) {
                    entry.remove(shard_id);
                }
            }

            // Update statistics
            let is_root = metadata.is_root();
            let is_leaf = metadata.is_leaf();
            self.stats.decrement_shard(metadata.size_bytes, is_root, is_leaf);
        }

        removed
    }

    /// Update shard metadata
    pub fn update_shard(&self, metadata: ShardMetadata) -> Result<()> {
        // Validate metadata before updating
        if let Err(e) = metadata.validate() {
            return Err(RafsError::InvalidShard(format!(
                "Metadata validation failed for shard {}: {}",
                metadata.shard_id, e
            )));
        }
        
        let shard_id = metadata.shard_id.clone();

        // Remove old entry and re-add to update indexes
        if let Some(_old_metadata) = self.remove_shard(&shard_id) {
            self.add_shard(metadata)?;
            Ok(())
        } else {
            Err(RafsError::ShardNotFound(format!(
                "Shard {} not found in index for update",
                shard_id
            )))
        }
    }

    /// Get all shard IDs
    pub fn shard_ids(&self) -> Vec<ShardId> {
        self.shards.iter().map(|entry| entry.key().clone()).collect()
    }

    /// Get all shards
    pub fn all_shards(&self) -> Vec<ShardMetadata> {
        self.shards.iter().map(|entry| entry.value().clone()).collect()
    }

    /// Query shards with filter
    pub fn query(&self, query: &ShardQuery) -> Vec<ShardMetadata> {
        self.shards
            .iter()
            .filter(|entry| query.matches(entry.value()))
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get root shards (no parent)
    pub fn root_shards(&self) -> Vec<ShardMetadata> {
        self.shards
            .iter()
            .filter(|entry| entry.value().is_root())
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get leaf shards (no children)
    pub fn leaf_shards(&self) -> Vec<ShardMetadata> {
        self.shards
            .iter()
            .filter(|entry| entry.value().is_leaf())
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get children of a shard
    pub fn get_children(&self, parent_id: &ShardId) -> Vec<ShardMetadata> {
        if let Some(entry) = self.hierarchy_index.get(parent_id) {
            entry
                .iter()
                .filter_map(|child_id| self.get_shard(child_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get shards by peer
    pub fn shards_by_peer(&self, peer_id: &PeerId) -> Vec<ShardMetadata> {
        if let Some(entry) = self.peer_index.get(peer_id) {
            entry
                .iter()
                .filter_map(|shard_id| self.get_shard(shard_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get shards by tag
    pub fn shards_by_tag(&self, tag: &str) -> Vec<ShardMetadata> {
        if let Some(entry) = self.tag_index.get(tag) {
            entry
                .iter()
                .filter_map(|shard_id| self.get_shard(shard_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get shards needing audit
    pub fn shards_needing_audit(&self, threshold_secs: i64) -> Vec<ShardMetadata> {
        self.shards
            .iter()
            .filter(|entry| entry.value().needs_audit(threshold_secs))
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Total size of all shards
    pub fn total_size(&self) -> u64 {
        self.stats.total_size.load(Ordering::Relaxed)
    }

    /// Total shard count
    pub fn shard_count(&self) -> u64 {
        self.stats.shard_count.load(Ordering::Relaxed)
    }

    /// Root shard count
    pub fn root_count(&self) -> u64 {
        self.stats.root_count.load(Ordering::Relaxed)
    }

    /// Leaf shard count
    pub fn leaf_count(&self) -> u64 {
        self.stats.leaf_count.load(Ordering::Relaxed)
    }

    /// Get index statistics
    pub fn stats(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        stats.insert("total_size".to_string(), self.total_size());
        stats.insert("shard_count".to_string(), self.shard_count());
        stats.insert("root_count".to_string(), self.root_count());
        stats.insert("leaf_count".to_string(), self.leaf_count());
        stats.insert("peer_count".to_string(), self.peer_index.len() as u64);
        stats.insert("tag_count".to_string(), self.tag_index.len() as u64);
        stats
    }

    /// Clear all shards from index
    pub fn clear(&self) {
        self.shards.clear();
        self.peer_index.clear();
        self.hierarchy_index.clear();
        self.tag_index.clear();

        // Reset stats
        self.stats.total_size.store(0, Ordering::Relaxed);
        self.stats.shard_count.store(0, Ordering::Relaxed);
        self.stats.root_count.store(0, Ordering::Relaxed);
        self.stats.leaf_count.store(0, Ordering::Relaxed);
    }

    /// Save index to file (async) with atomic write
    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        
        let shards: Vec<ShardMetadata> = self.all_shards();
        let json = serde_json::to_string_pretty(&shards)
            .map_err(|e| RafsError::SerializationError(e.to_string()))?;

        // Atomic write: write to temp file, then rename
        let temp_path = path.as_ref().with_extension("tmp");
        let mut file = fs::File::create(&temp_path).await?;
        file.write_all(json.as_bytes()).await?;
        file.flush().await?;
        file.sync_all().await?; // Ensure durability
        
        // Atomic rename
        fs::rename(&temp_path, path).await?;
        
        Ok(())
    }

    /// Load index from file (async) with error recovery
    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let mut file = fs::File::open(path_ref).await
            .map_err(|e| RafsError::IoError(format!(
                "Failed to open index file at {}: {}",
                path_ref.display(), e
            )))?;
        
        let mut json = String::new();
        file.read_to_string(&mut json).await
            .map_err(|e| RafsError::IoError(format!(
                "Failed to read index file at {}: {}",
                path_ref.display(), e
            )))?;

        if json.is_empty() {
            info!("Index file {} is empty, returning empty index", path_ref.display());
            return Ok(Self::new()); // Return empty index if file is empty
        }

        let shards: Vec<ShardMetadata> = serde_json::from_str(&json)
            .map_err(|e| RafsError::SerializationError(format!(
                "Failed to parse index JSON from {}: {}",
                path_ref.display(), e
            )))?;

        let index = Self::new();
        let mut loaded = 0;
        let mut errors = 0;
        let mut error_details = Vec::new();
        
        for (idx, metadata) in shards.into_iter().enumerate() {
            match index.add_shard(metadata.clone()) {
                Ok(_) => loaded += 1,
                Err(e) => {
                    errors += 1;
                    error_details.push(format!("Shard {} at index {}: {}", metadata.shard_id, idx, e));
                    warn!("Failed to add shard {} to index: {}", metadata.shard_id, e);
                    // Continue loading other shards
                }
            }
        }
        
        if errors > 0 {
            warn!(
                "Loaded {} shards from index {} ({} errors encountered). Error details: {:?}",
                loaded, path_ref.display(), errors, error_details
            );
        } else {
            info!("Successfully loaded {} shards from index {}", loaded, path_ref.display());
        }
        
        Ok(index)
    }
}

impl Default for ShardIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash;

    fn create_test_shard(id: u8, parent: Option<ShardId>) -> ShardMetadata {
        // Mock hash function usage for test setup
        let content = vec![id; 100];
        // Note: Actual impl depends on crypto::hash return type (Blake3Digest?)
        // Assuming simple construction or mocking here for "like this" regeneration
        let shard_id = ShardId::from_content(&content); 
        let mut metadata = ShardMetadata::new(shard_id, 100, None);
        metadata.parent_shard = parent;
        metadata
    }

    #[test]
    fn test_add_get_shard() {
        let index = ShardIndex::new();
        let metadata = create_test_shard(1, None);
        let shard_id = metadata.shard_id.clone();

        index.add_shard(metadata.clone()).unwrap();

        let retrieved = index.get_shard(&shard_id).unwrap();
        assert_eq!(retrieved.shard_id, shard_id);
        assert_eq!(index.shard_count(), 1);
    }

    #[test]
    fn test_query_builder() {
        let index = ShardIndex::new();

        let mut shard1 = create_test_shard(1, None);
        shard1.add_peer("peer1".to_string());
        shard1.add_tag("important".to_string());
        shard1.geometry = LatticeGeometry::Kagome; // Logic Core

        let mut shard2 = create_test_shard(2, None);
        shard2.add_peer("peer2".to_string());
        shard2.add_tag("archive".to_string());
        shard2.geometry = LatticeGeometry::Bethe; // Storage Core

        index.add_shard(shard1).unwrap();
        index.add_shard(shard2).unwrap();

        // Query by peer
        let query = ShardQuery::new().peer("peer1".to_string());
        let results = index.query(&query);
        assert_eq!(results.len(), 1);

        // 🟢 Test Phase II Geometry Query
        let geometry_query = ShardQuery::new().geometry(LatticeGeometry::Bethe);
        let geo_results = index.query(&geometry_query);
        assert_eq!(geo_results.len(), 1);
        assert_eq!(geo_results[0].geometry, LatticeGeometry::Bethe);
    }

    #[test]
    fn test_statistics() {
        let index = ShardIndex::new();
        let root1 = create_test_shard(1, None);
        let root2 = create_test_shard(2, None);
        index.add_shard(root1.clone()).unwrap();
        index.add_shard(root2.clone()).unwrap();

        assert_eq!(index.shard_count(), 2);
        assert_eq!(index.root_count(), 2);
        assert_eq!(index.total_size(), 200);
    }

    #[tokio::test]
    async fn test_save_load() {
        let index = ShardIndex::new();
        let shard1 = create_test_shard(1, None);
        let shard2 = create_test_shard(2, None);

        index.add_shard(shard1).unwrap();
        index.add_shard(shard2).unwrap();

        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("index.json");

        index.save_to_file(&file_path).await.unwrap();
        let loaded = ShardIndex::load_from_file(&file_path).await.unwrap();

        assert_eq!(loaded.shard_count(), 2);
    }
}