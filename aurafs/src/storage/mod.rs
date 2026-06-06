//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Storage HAL - The Lattice Atlas Physical Layer
//! 🧠 Context-Aware Tiering: Routes Data Based on Lattice Geometry
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{Shard, ShardId, ShardMetadata, metadata::LatticeGeometry},
    shard_server::acl::AclEnforcer,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use thiserror::Error;
use tracing::{info, warn, debug};

// -----------------------------------------------------------------------------
// Module Exports
// -----------------------------------------------------------------------------

pub use self::{
    filesystem::{AuraFS, FsConfig, FsMetrics},
    inode::{Inode, InodeId, InodeTimestamps},
    directory::Directory,
    journal::{QuantumJournal, JournalError},
    quota::{SoulQuotaManager, SoulQuota, QuotaError},
    snapshot::{SnapshotManager, Snapshot, SnapshotId, SnapshotError},
    local::LocalShardStorage,
    shard_store::{ShardStore, ShardStoreConfig, ShardStoreError},
    fuse::AuraFSFuse,
};

pub mod filesystem;
pub mod inode;
pub mod directory;
pub mod journal;
pub mod quota;
pub mod snapshot;
pub mod local;
pub mod shard_store;
pub mod fuse;

// -----------------------------------------------------------------------------
// The Lattice Physics HAL (Hardware Abstraction Layer)
// -----------------------------------------------------------------------------

/// The fundamental interface for all AuraFS storage backends.
/// Whether it's NVMe, IPFS, or RAM, it must implement this physics.
#[async_trait]
pub trait ShardStorage: Send + Sync + Debug {
    /// Persist a shard.
    /// Implementation should respect the shard's LatticeGeometry if possible.
    async fn store(&self, shard: &Shard) -> Result<(), StorageError>;

    /// Retrieve a shard by ID.
    async fn load(&self, shard_id: &ShardId) -> Result<Shard, StorageError>;

    /// Delete a shard.
    async fn delete(&self, shard_id: &ShardId) -> Result<(), StorageError>;

    /// List stored shards (optional prefix filtering).
    async fn list(&self, prefix: Option<&str>) -> Result<Vec<ShardId>, StorageError>;

    /// Report backend health and physics metrics.
    async fn health(&self) -> Result<StorageHealth, StorageError>;
}

/// Tiered Storage Orchestrator (The "Lattice Router")
/// Routes data to Hot (Primary) or Cold (Secondary) storage based on Geometry.
#[derive(Debug, Clone)]
pub struct TieredShardStorage {
    /// Hot Storage (NVMe/SSD) - Optimized for Kagome (Compute) & Triangular (Network)
    pub primary: Arc<dyn ShardStorage>,
    
    /// Deep Storage (IPFS/S3) - Optimized for Bethe (Storage) & FlowerOfLife (Archival)
    pub secondary: Option<Arc<dyn ShardStorage>>,
    
    /// Security Enforcement
    pub acl_enforcer: Arc<AclEnforcer>,
}

impl TieredShardStorage {
    /// Create a new tiered storage engine
    pub fn new(
        primary: Arc<dyn ShardStorage>,
        secondary: Option<Arc<dyn ShardStorage>>,
        acl_enforcer: Arc<AclEnforcer>,
    ) -> Self {
        Self {
            primary,
            secondary,
            acl_enforcer,
        }
    }

    /// Determines the optimal storage tier based on Lattice Physics
    fn determine_tier(&self, geometry: &LatticeGeometry) -> StorageTier {
        match geometry {
            // High Frequency / Low Latency -> Primary
            LatticeGeometry::Kagome => StorageTier::Primary,      // Compute Heavy
            LatticeGeometry::Triangular => StorageTier::Primary,  // Network Heavy
            LatticeGeometry::Sierpinski => StorageTier::Primary,  // Index/Hot
            
            // Low Frequency / High Persistence -> Secondary
            LatticeGeometry::Bethe => StorageTier::Secondary,     // Deep Storage
            LatticeGeometry::FlowerOfLife => StorageTier::Secondary, // Archival
        }
    }
}

#[async_trait]
impl ShardStorage for TieredShardStorage {
    async fn store(&self, shard: &Shard) -> Result<(), StorageError> {
        let tier = self.determine_tier(&shard.metadata.geometry);
        
        match tier {
            StorageTier::Primary => {
                debug!("🔥 Storing shard {} in PRIMARY tier ({:?})", shard.shard_id, shard.metadata.geometry);
                self.primary.store(shard).await
            }
            StorageTier::Secondary => {
                if let Some(secondary) = &self.secondary {
                    debug!("🧊 Storing shard {} in SECONDARY tier ({:?})", shard.shard_id, shard.metadata.geometry);
                    // Write-Through strategy: Store in Secondary, but keep hot reference in Primary if needed?
                    // For pure Bethe storage, we skip Primary to save NVMe space.
                    secondary.store(shard).await
                } else {
                    warn!("⚠️  Shard {} requested Secondary tier but none available. Fallback to Primary.", shard.shard_id);
                    self.primary.store(shard).await
                }
            }
        }
    }

    async fn load(&self, shard_id: &ShardId) -> Result<Shard, StorageError> {
        
        // 1. Try Primary (Fastest)
        match self.primary.load(shard_id).await {
            Ok(shard) => return Ok(shard),
            Err(StorageError::NotFound) => {
                // 2. Fallback to Secondary (Deep Search)
                if let Some(secondary) = &self.secondary {
                    debug!("🔍 Shard {} miss in Primary, checking Secondary...", shard_id);
                    let shard = secondary.load(shard_id).await?;
                    
                    // 3. Promote to Primary (Cache on Read)
                    // If we accessed it, it's now "hot" (Triangular/Active).
                    // We temporarily cache it in NVMe.
                    let _ = self.primary.store(&shard).await;
                    
                    return Ok(shard);
                } else {
                    return Err(StorageError::NotFound);
                }
            }
            Err(e) => return Err(e),
        }
    }

    async fn delete(&self, shard_id: &ShardId) -> Result<(), StorageError> {
        // Delete from both to ensure consistency
        let p_res = self.primary.delete(shard_id).await;
        let s_res = if let Some(secondary) = &self.secondary {
            secondary.delete(shard_id).await
        } else {
            Ok(())
        };

        // Return error only if both failed, otherwise successful removal
        if p_res.is_err() && s_res.is_err() {
            return p_res;
        }
        Ok(())
    }

    async fn list(&self, prefix: Option<&str>) -> Result<Vec<ShardId>, StorageError> {
        // Merge lists from both tiers
        let mut ids = self.primary.list(prefix).await?;
        if let Some(secondary) = &self.secondary {
            let sec_ids = secondary.list(prefix).await?;
            for id in sec_ids {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
        Ok(ids)
    }

    async fn health(&self) -> Result<StorageHealth, StorageError> {
        let p_health = self.primary.health().await?;
        let s_health = if let Some(sec) = &self.secondary {
            Some(sec.health().await?)
        } else {
            None
        };

        // Aggregated Health
        Ok(StorageHealth {
            backend: StorageBackend::Tiered,
            available_bytes: p_health.available_bytes + s_health.as_ref().map(|h| h.available_bytes).unwrap_or(0),
            used_bytes: p_health.used_bytes + s_health.as_ref().map(|h| h.used_bytes).unwrap_or(0),
            shard_count: p_health.shard_count + s_health.as_ref().map(|h| h.shard_count).unwrap_or(0),
            latency_ms: p_health.latency_ms, // Primary latency dominates perception
            healthy: p_health.healthy && s_health.map(|h| h.healthy).unwrap_or(true),
        })
    }
}

// -----------------------------------------------------------------------------
// Support Types
// -----------------------------------------------------------------------------

/// Helper enum for internal routing
enum StorageTier {
    Primary,
    Secondary,
}

/// Identifies the physical backend type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageBackend {
    /// Local Filesystem / NVMe
    Local { path: String },
    /// IPFS Cluster / Swarm
    IPFS { node_id: String },
    /// In-Memory (for testing/caching)
    Memory,
    /// Composite Tiered Backend
    Tiered,
    /// Unknown/Custom
    Unknown(String),
}

/// Metrics and Status for a storage backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealth {
    pub backend: StorageBackend,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub shard_count: u64,
    pub latency_ms: f32,
    pub healthy: bool,
}

/// Standard Storage Errors (Mapped to main Error via From)
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Shard not found")]
    NotFound,
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization Error: {0}")]
    SerializationError(String),
    #[error("Backend Error: {0}")]
    BackendError(String),
    #[error("Access Denied")]
    AccessDenied,
}

// -----------------------------------------------------------------------------
// Existing Error Handling (Preserved for backward compat)
// -----------------------------------------------------------------------------

/// Unified storage result type with comprehensive error handling
pub type Result<T> = std::result::Result<T, Error>;

/// Enterprise-grade storage error enum with proper error context
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Filesystem error: {0}")]
    Filesystem(#[from] filesystem::FsError),
    #[error("Local storage error: {0}")]
    Local(#[from] local::LocalStorageError),
    #[error("Shard store error: {0}")]
    ShardStore(#[from] ShardStoreError),
    #[error("Journal error: {0}")]
    Journal(#[from] JournalError),
    #[error("Quota error: {0}")]
    Quota(#[from] QuotaError),
    #[error("Snapshot error: {0}")]
    Snapshot(#[from] SnapshotError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Storage HAL error: {0}")]
    Storage(#[from] StorageError), // ✨ Added HAL error
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
    #[error("Operation timeout: {0}")]
    Timeout(String),
    #[error("Invalid state: {0}")]
    InvalidState(String),
}

impl Error {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Error::Io(_) | Error::Timeout(_) | Error::ResourceExhausted(_)
        )
    }

    /// Get error severity for observability
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Error::Config(_) | Error::InvalidState(_) => ErrorSeverity::Critical,
            Error::Quota(_) | Error::ResourceExhausted(_) => ErrorSeverity::High,
            Error::Timeout(_) | Error::Io(_) => ErrorSeverity::Medium,
            _ => ErrorSeverity::Low,
        }
    }
}

/// Error severity levels for observability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
}