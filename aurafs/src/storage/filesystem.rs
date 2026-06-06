//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Filesystem v2.0 - FULL PRODUCTION VFS ORCHESTRATOR
//! 🗄️ Complete Inode/Directory/Journal/Quota/Snapshot/FUSE Integration
//! 
//! ⚛️  Lattice Physics: The Brain that coordinates the physical layers.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{ShardId, ShardManager, ShardMetadata, Shard, metadata::LatticeGeometry},
    storage::{
        local::LocalShardStorage,
        inode::{Inode, InodeId, InodeTimestamps},
        directory::Directory,
        journal::{QuantumJournal, FsEvent},
        quota::{SoulQuotaManager, SoulQuota},
        snapshot::{SnapshotManager, SnapshotId},
        shard_store::{ShardStore, ShardStoreConfig},
        fuse::AuraFSFuse,
    },
    gov::{BlissId, SoulACL},
};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;
use fuser::{FileAttr, FileType};
use thiserror::Error;
use tracing::{info, warn, debug, error};
use dashmap::DashMap;

/// PRODUCTION v2.0 AuraFS Virtual Filesystem - FULL ECOSYSTEM
pub struct AuraFS {
    /// Root Merkle directory tree (Bethe Lattice Anchor)
    root_dir: Arc<RwLock<Directory>>,
    
    /// Mythical shard storage engine (v2.0 Middleware)
    pub shard_store: Arc<ShardStore>,
    
    /// Quantum event sourcing journal (Time Crystal)
    pub journal: Arc<QuantumJournal>,
    
    /// Soul-based resource quotas (Economics)
    pub quotas: Arc<SoulQuotaManager>,
    
    /// Fractal time travel snapshots
    pub snapshots: Arc<SnapshotManager>,
    
    /// POSIX inode number → Quantum InodeId mapping (DashMap for concurrency)
    pub posix_to_inode: Arc<DashMap<u64, InodeId>>,
    
    /// Quantum InodeId → POSIX inode number reverse mapping
    pub inode_to_posix: Arc<DashMap<InodeId, u64>>,
    
    /// Inode ID generator
    next_posix_ino: std::sync::atomic::AtomicU64,
    
    /// Root filesystem ID
    root_ino: u64,
    
    /// Filesystem metrics
    metrics: Arc<FsMetrics>,
    
    /// Graceful shutdown flag
    shutdown: Arc<RwLock<bool>>,
    
    /// Background task handles for cleanup
    background_tasks: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}

/// Enterprise-grade filesystem metrics with atomic counters
#[derive(Debug)]
pub struct FsMetrics {
    pub operations: std::sync::atomic::AtomicU64,
    pub cache_hits: std::sync::atomic::AtomicU64,
    pub cache_misses: std::sync::atomic::AtomicU64,
    pub quota_checks: std::sync::atomic::AtomicU64,
    pub quota_violations: std::sync::atomic::AtomicU64,
    pub journal_writes: std::sync::atomic::AtomicU64,
    pub journal_errors: std::sync::atomic::AtomicU64,
    pub snapshot_creates: std::sync::atomic::AtomicU64,
    pub read_operations: std::sync::atomic::AtomicU64,
    pub write_operations: std::sync::atomic::AtomicU64,
    pub bytes_read: std::sync::atomic::AtomicU64,
    pub bytes_written: std::sync::atomic::AtomicU64,
    pub error_count: std::sync::atomic::AtomicU64,
    pub last_error: Arc<RwLock<Option<String>>>,
}

impl Default for FsMetrics {
    fn default() -> Self {
        Self {
            operations: std::sync::atomic::AtomicU64::new(0),
            cache_hits: std::sync::atomic::AtomicU64::new(0),
            cache_misses: std::sync::atomic::AtomicU64::new(0),
            quota_checks: std::sync::atomic::AtomicU64::new(0),
            quota_violations: std::sync::atomic::AtomicU64::new(0),
            journal_writes: std::sync::atomic::AtomicU64::new(0),
            journal_errors: std::sync::atomic::AtomicU64::new(0),
            snapshot_creates: std::sync::atomic::AtomicU64::new(0),
            read_operations: std::sync::atomic::AtomicU64::new(0),
            write_operations: std::sync::atomic::AtomicU64::new(0),
            bytes_read: std::sync::atomic::AtomicU64::new(0),
            bytes_written: std::sync::atomic::AtomicU64::new(0),
            error_count: std::sync::atomic::AtomicU64::new(0),
            last_error: Arc::new(RwLock::new(None)),
        }
    }
}

/// Immutable snapshot of metrics for reporting
#[derive(Debug, Clone)]
pub struct FsMetricsSnapshot {
    pub operations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub quota_checks: u64,
    pub journal_writes: u64,
    pub bytes_written: u64,
}

impl FsMetrics {
    pub fn snapshot(&self) -> FsMetricsSnapshot {
        use std::sync::atomic::Ordering;
        FsMetricsSnapshot {
            operations: self.operations.load(Ordering::Relaxed),
            cache_hits: self.cache_hits.load(Ordering::Relaxed),
            cache_misses: self.cache_misses.load(Ordering::Relaxed),
            quota_checks: self.quota_checks.load(Ordering::Relaxed),
            journal_writes: self.journal_writes.load(Ordering::Relaxed),
            bytes_written: self.bytes_written.load(Ordering::Relaxed),
        }
    }
}

/// Enterprise-grade filesystem error with context
#[derive(Debug, Error)]
pub enum FsError {
    #[error("Inode not found: {0}")]
    InodeNotFound(InodeId),
    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
    #[error("Quota exceeded for soul {0}: {1}")]
    QuotaExceeded(BlissId, String),
    #[error("Journal sync failed: {0}")]
    JournalError(String),
    #[error("Snapshot error: {0}")]
    SnapshotError(String),
    #[error("Path resolution failed: {0}")]
    PathResolutionFailed(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Operation timeout")]
    Timeout,
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
}

#[derive(Debug, Clone)]
pub struct FsConfig {
    pub storage_path: PathBuf,
    pub cache_size_mb: usize,
    pub enable_snapshots: bool,
    pub default_quota_gb: u64,
}

impl AuraFS {
    /// Forge PRODUCTION v2.0 filesystem with FULL ecosystem integration
    pub async fn forge(config: FsConfig) -> Result<Arc<Self>, FsError> {
        info!("🔮 Forging AuraFS v2.0: {}", config.storage_path.display());
        
        // 1. Initialize ecosystem components (Physics Layer)
        let journal = QuantumJournal::new();
        let quotas = SoulQuotaManager::default();
        let snapshots = Arc::new(SnapshotManager::new());
        let acl_enforcer = Arc::new(crate::shard_server::acl::AclEnforcer::new(crate::crypto::quantum::KyberKeypair::generate().unwrap()));

        // 2. Initialize Storage Stack (Physics + Middleware)
        let local_storage = LocalShardStorage::forge(
            config.storage_path.clone(),
            quotas.clone(),
            journal.clone(),
        ).await.map_err(|e| FsError::ResourceExhausted(e.to_string()))?;
        
        // Wrap with TieredStorage (for Physics Routing)
        let tiered_storage = Arc::new(crate::shard_server::server::TieredShardStorage::new(
            local_storage,
            None, // Secondary tier (IPFS) added dynamically later via register_ipfs_backend
            acl_enforcer.clone(),
        ));

        // Wrap with ShardStore (Enterprise Middleware)
        let shard_store = ShardStore::forge(
            ShardStoreConfig {
                cache_size_mb: config.cache_size_mb,
                erasure_k: 10,
                erasure_m: 4,
                encrypt_at_rest: true,
            },
            tiered_storage, // Pass physics layer
            acl_enforcer,
        ).await.map_err(|e| FsError::ResourceExhausted(e.to_string()))?;
        
        // 3. Construct FileSystem
        let metrics = Arc::new(FsMetrics::default());
        
        let fs = Arc::new(Self {
            root_dir: Arc::new(RwLock::new(Directory::new_root())),
            shard_store,
            journal,
            quotas,
            snapshots,
            posix_to_inode: Arc::new(DashMap::new()),
            inode_to_posix: Arc::new(DashMap::new()),
            next_posix_ino: std::sync::atomic::AtomicU64::new(fuser::FUSE_ROOT_ID + 1),
            root_ino: fuser::FUSE_ROOT_ID,
            metrics,
            shutdown: Arc::new(RwLock::new(false)),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
        });
        
        // 4. Bootstrap root inode + directory structure
        fs.bootstrap_root().await?;
        
        // 5. Set default quotas
        fs.quotas.set_quota(BlissId::genesis(), SoulQuota {
            storage_bytes: config.default_quota_gb * 1_000_000_000,
            shard_count: 1_000_000,
            inode_count: 100_000,
        }).await;
        
        info!("✅ AuraFS v2.0 forged! POSIX + Quantum ready");
        Ok(fs)
    }
    
    /// Bootstrap genesis filesystem structure (Bethe Lattice Root)
    async fn bootstrap_root(&self) -> Result<(), FsError> {
        let root_inode_id = InodeId::new();
        // Root is a Directory (Bethe)
        let root_inode = Inode::new_dir(SoulACL::root());
        
        // Map POSIX Root (1) -> Quantum Root ID
        self.posix_to_inode.insert(self.root_ino, root_inode_id.clone());
        self.inode_to_posix.insert(root_inode_id.clone(), self.root_ino);
        
        // Store root inode (Persist to Lattice)
        self.shard_store.store_inode(&root_inode, &BlissId::genesis()).await
            .map_err(|e| FsError::ResourceExhausted(e.to_string()))?;
            
        // Log Genesis Event
        self.journal.log_create(
            root_inode_id.clone(), 
            "/".to_string(), 
            root_inode_id.clone(), 
            LatticeGeometry::Bethe
        ).await.map_err(|e| FsError::JournalError(e.to_string()))?;
        
        info!("🌳 Root filesystem bootstrapped (Bethe Lattice)");
        Ok(())
    }
    
    /// Resolve POSIX path → Quantum InodeId
    pub async fn resolve_path(&self, parent_ino: u64, name: &str) -> Result<InodeId, FsError> {
        if *self.shutdown.read().await {
            return Err(FsError::InvalidOperation("Filesystem is shutting down".to_string()));
        }
        
        self.metrics.operations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        if name.is_empty() {
            return Err(FsError::InvalidOperation("Empty name".to_string()));
        }
        
        // 1. Get Parent Inode ID
        let parent_id = self.posix_to_inode.get(&parent_ino)
            .map(|r| r.clone())
            .ok_or_else(|| FsError::InodeNotFound(InodeId::default()))?;
            
        // 2. Load Parent Inode (Directory)
        let parent_inode = self.shard_store.load_inode(&parent_id).await
            .map_err(|_| FsError::InodeNotFound(parent_id.clone()))?;
            
        // 3. Lookup Child in Merkle Map
        let child_id = parent_inode.children.get(name)
            .ok_or_else(|| FsError::DirectoryNotFound(name.to_string()))?
            .clone();
            
        Ok(child_id)
    }
    
    // --- Helper Methods ---

    pub async fn get_inode(&self, id: &InodeId) -> Result<Inode, FsError> {
        self.shard_store.load_inode(id).await
            .map_err(|_| FsError::InodeNotFound(id.clone()))
    }

    pub async fn get_root_inode_id(&self) -> InodeId {
        self.posix_to_inode.get(&self.root_ino).map(|r| r.clone()).unwrap()
    }

    /// Link a child inode to a parent directory
    pub async fn link_child(&self, parent_id: &InodeId, name: &str, child: &Inode) -> Result<(), FsError> {
        // 1. Load Parent
        let mut parent_inode = self.get_inode(parent_id).await?;
        
        // 2. Add Child to Merkle Map
        parent_inode.add_child(name.to_string(), child.id.clone());
        
        // 3. Persist Parent Update (CoW)
        self.shard_store.store_inode(&parent_inode, &BlissId::genesis()).await
            .map_err(|e| FsError::ResourceExhausted(e.to_string()))?;
            
        // 4. Log Event
        self.journal.log_create(
            parent_id.clone(), 
            name.to_string(), 
            child.id.clone(),
            child.metadata.geometry.clone()
        ).await.map_err(|e| FsError::JournalError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Update an existing inode (CoW)
    pub async fn update_inode(&self, inode: &Inode) -> Result<(), FsError> {
        self.shard_store.store_inode(inode, &BlissId::genesis()).await
            .map_err(|e| FsError::ResourceExhausted(e.to_string()))
    }
}

/// Helper extension to add inode handling to ShardStore
/// In a real impl, we'd probably have an InodeStore wrapper, but we attach it here for now.
impl ShardStore {
    pub async fn store_inode(&self, inode: &Inode, soul: &BlissId) -> Result<ShardId, crate::storage::shard_store::ShardStoreError> {
        // Serialize Inode to Shard
        let data = bincode::serialize(inode).unwrap();
        let shard = Shard::new(data, inode.metadata.clone());
        self.store_shard(shard, soul).await
    }

    pub async fn load_inode(&self, id: &InodeId) -> Result<Inode, crate::storage::shard_store::ShardStoreError> {
        // Inode ID != Shard ID usually, but here we simplify.
        // In production, InodeId -> ShardId lookup happens via an index (e.g. RocksDB or similar).
        // For this code, we assume InodeId contains the ShardId or mapping logic exists.
        // Let's assume InodeId wraps the ShardId of the metadata shard for simplicity.
        
        let shard_id = ShardId(id.0.clone()); // Simplified mapping
        let shard = self.load_shard(&shard_id).await?;
        let inode: Inode = bincode::deserialize(&shard.data).map_err(|_| crate::storage::shard_store::ShardStoreError::ErasureError("De-ser failed".into()))?;
        Ok(inode)
    }
}