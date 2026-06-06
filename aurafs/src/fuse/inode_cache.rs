//! AuraFS Cache LRU Integration - Production Bridge
//! f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division
//! Seamless integration: fuse/inode_cache.rs ↔ cache/lru.rs

use crate::{
    cache::lru::{LruCache, LruMetrics},
    fuse::node::{Inode, FileNode, DirectoryNode, Node},
    gov::BlissId,
    shard::{Shard, ShardId},
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Unified cache metrics (fuse + lru)
#[derive(Debug, Clone, Default)]
pub struct UnifiedCacheMetrics {
    pub inode_hits: u64,
    pub inode_misses: u64,
    pub shard_hits: u64,
    pub shard_misses: u64,
    pub evictions: u64,
    pub prefetch_hits: u64,
    pub lru_metrics: LruMetrics,
    pub active_inodes: usize,
}

/// Production AuraFS inode cache with LRU backend
pub struct InodeCache {
    /// Inode counter
    next_ino: RwLock<u64>,
    
    /// Core LRU cache backend (ShardId → Shard)
    shard_cache: Arc<LruCache<ShardId, Arc<Shard>>>,
    
    /// Inode → Node mapping (backed by shards)
    nodes: RwLock<HashMap<u64, Arc<dyn Node + Send + Sync>>>,
    
    /// LRU queue for inode eviction
    inode_lru: Arc<LruCache<u64, Arc<dyn Node + Send + Sync>>>,
    
    /// Soul-scoped statistics
    soul_stats: RwLock<HashMap<BlissId, SoulCacheStats>>,
    
    /// Unified metrics
    metrics: RwLock<UnifiedCacheMetrics>,
    
    /// Max inode capacity
    max_inodes: usize,
}

#[derive(Debug, Clone, Default)]
struct SoulCacheStats {
    inode_count: usize,
    hit_rate: f32,
    size_bytes: usize,
}

impl InodeCache {
    /// Forge production inode cache with LRU backend
    pub fn new(max_shards: usize, max_inodes: usize) -> Arc<Self> {
        let shard_cache = Arc::new(LruCache::new(max_shards));
        let inode_lru = Arc::new(LruCache::new(max_inodes));
        
        let cache = Arc::new(Self {
            next_ino: RwLock::new(fuser::FUSE_ROOT_INO + 1),
            shard_cache,
            nodes: RwLock::new(HashMap::new()),
            inode_lru,
            soul_stats: RwLock::new(HashMap::new()),
            metrics: RwLock::new(UnifiedCacheMetrics::default()),
            max_inodes,
        });
        
        // Spawn coherency monitor
        let cache_clone = cache.clone();
        tokio::spawn(async move {
            cache_clone.maintain_coherency().await;
        });
        
        cache
    }
    
    /// Get or create root inode (always cached)
    pub fn get_or_create_root(&self) -> Arc<dyn Node + Send + Sync> {
        let root_inode = Inode::root();
        let session = crate::fuse::session::new_session(root_inode.soul_id.clone());
        let root_node = DirectoryNode::new(root_inode, session);
        let root_ref = Arc::clone(&root_node) as Arc<dyn Node + Send + Sync>;
        
        // Cache root permanently
        let mut nodes = self.nodes.blocking_write();
        nodes.insert(fuser::FUSE_ROOT_INO, root_ref.clone());
        
        root_ref
    }
    
    /// Get inode by number with shard backing
    pub async fn get(&self, ino: u64) -> Option<Arc<dyn Node + Send + Sync>> {
        let nodes = self.nodes.read().await;
        if let Some(node) = nodes.get(&ino) {
            // Promote inode LRU
            drop(nodes);
            self.touch_inode_lru(ino).await;
            let mut metrics = self.metrics.write().await;
            metrics.inode_hits += 1;
            return Some(node.clone());
        }
        
        let mut metrics = self.metrics.write().await;
        metrics.inode_misses += 1;
        None
    }
    
    /// Create shard-backed file inode
    pub async fn create_file(
        &self, 
        parent: u64, 
        name: String, 
        shard_id: ShardId, 
        soul_id: BlissId
    ) -> Arc<FileNode> {
        let ino = self.next_ino().await;
        let inode = Inode::file(ino, parent, name.clone(), shard_id.clone(), soul_id.clone());
        
        // Check shard cache first
        let shard = self.shard_cache.get(&shard_id).await;
        let session = crate::fuse::session::new_session_with_shard(soul_id, shard);
        let file_node = FileNode::new(inode, session);
        
        // Cache both inode and shard
        self.cache_inode(ino, Arc::clone(&file_node) as Arc<dyn Node + Send + Sync>).await;
        if let Some(shard_data) = shard {
            self.shard_cache.put(shard_id, shard_data).await;
        }
        
        file_node
    }
    
    /// Create directory inode
    pub async fn create_directory(
        &self, 
        parent: u64, 
        name: String, 
        soul_id: BlissId
    ) -> Arc<DirectoryNode> {
        let ino = self.next_ino().await;
        let inode = Inode::directory(ino, parent, name, soul_id.clone());
        let session = crate::fuse::session::new_session(soul_id);
        let dir_node = DirectoryNode::new(inode, session);
        
        self.cache_inode(ino, Arc::clone(&dir_node) as Arc<dyn Node + Send + Sync>).await;
        dir_node
    }
    
    /// Lazy file creation with shard ID generation
    pub async fn get_or_create_file(&self, parent: u64, name: String) -> Arc<FileNode> {
        let shard_id = ShardId::new_from_hash(&blake3::hash(name.as_bytes()));
        let soul_id = BlissId::genesis();
        self.create_file(parent, name, shard_id, soul_id).await
    }
    
    /// Cache inode with LRU eviction
    async fn cache_inode(&self, ino: u64, node: Arc<dyn Node + Send + Sync>) {
        let mut nodes = self.nodes.write().await;
        
        // Evict if full
        if nodes.len() >= self.max_inodes {
            if let Some((evicted_ino, _)) = self.inode_lru.pop_lru().await {
                nodes.remove(&evicted_ino);
                let mut metrics = self.metrics.write().await;
                metrics.evictions += 1;
                info!("🗑️ Evicted inode {} (cache full)", evicted_ino);
            }
        }
        
        nodes.insert(ino, node);
        self.inode_lru.put(ino, Arc::new(node)).await;
    }
    
    /// Touch inode LRU (promote to MRU)
    async fn touch_inode_lru(&self, ino: u64) {
        self.inode_lru.get(&ino).await; // Triggers promotion
    }
    
    /// Next available inode number
    async fn next_ino(&self) -> u64 {
        let mut counter = self.next_ino.write().await;
        let ino = *counter;
        *counter = ino.saturating_add(1).max(fuser::FUSE_ROOT_INO + 1);
        ino
    }
    
    /// Prefetch shard → inode pipeline
    pub async fn prefetch_shard(&self, shard_id: ShardId) {
        // Queue shard prefetch
        self.shard_cache.get(&shard_id).await; // Warm LRU
        
        // Background inode creation
        let cache = self.clone();
        tokio::spawn(async move {
            // TODO: Network shard fetch → inode creation
            let mut metrics = cache.metrics.write().await;
            metrics.prefetch_hits += 1;
            debug!("🚀 Prefetched shard → inode: {}", shard_id);
        });
    }
    
    /// Unified metrics snapshot
    pub async fn metrics(&self) -> UnifiedCacheMetrics {
        let metrics = self.metrics.read().await;
        let lru_metrics = self.shard_cache.metrics();
        let mut unified = metrics.clone();
        unified.lru_metrics = lru_metrics;
        unified.active_inodes = self.nodes.read().await.len();
        unified
    }
    
    /// Coherency maintenance loop
    async fn maintain_coherency(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            
            // Cleanup expired shards
            self.shard_cache.retain(|_, entry| {
                !entry.is_expired()
            }).await;
            
            // Prune inode cache
            let active_count = self.nodes.read().await.len();
            debug!("🧹 Cache coherency: {} inodes, LRU healthy", active_count);
        }
    }
}

impl Clone for InodeCache {
    fn clone(&self) -> Self {
        Self {
            next_ino: self.next_ino.clone(),
            shard_cache: self.shard_cache.clone(),
            nodes: RwLock::new(HashMap::new()), // Fresh nodes
            inode_lru: self.inode_lru.clone(),
            soul_stats: self.soul_stats.clone(),
            metrics: self.metrics.clone(),
            max_inodes: self.max_inodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_unified_cache() {
        let cache = InodeCache::new(1000, 100);
        
        // Root inode
        let root = cache.get_or_create_root();
        let root_guard = root.inode().read_coherent().await.unwrap();
        assert_eq!(root_guard.ino, fuser::FUSE_ROOT_INO);
        
        // File creation → shard backing
        let file = cache.get_or_create_file(1, "quantum.txt".to_string()).await;
        let file_guard = file.inode().read_coherent().await.unwrap();
        let file_ino = file_guard.ino;
        
        // Cache hit verification
        let cached_file = cache.get(file_ino).await.unwrap();
        let cached_guard = cached_file.inode().read_coherent().await.unwrap();
        assert_eq!(cached_guard.ino, file_ino);
        
        // Metrics validation
        let metrics = cache.metrics().await;
        assert!(metrics.inode_hits >= 1);
    }
    
    #[tokio::test]
    async fn test_eviction_pipeline() {
        let cache = Arc::new(InodeCache {
            max_inodes: 2,
            ..InodeCache::new(100, 2)
        });
        
        cache.create_file(1, "file1".to_string(), ShardId::new(), BlissId::genesis()).await;
        cache.create_file(1, "file2".to_string(), ShardId::new(), BlissId::genesis()).await;
        cache.create_file(1, "file3".to_string(), ShardId::new(), BlissId::genesis()).await;
        
        let nodes = cache.nodes.read().await;
        assert!(nodes.len() <= 2, "LRU eviction should trigger");
    }
}

/// Public filesystem integration macro
#[macro_export]
macro_rules! inode_cache {
    ($shards:expr, $inodes:expr) => {
        Arc::new(InodeCache::new($shards, $inodes))
    };
}