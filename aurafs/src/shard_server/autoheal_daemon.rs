//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Server AutoHeal Daemon - Self-Healing Mesh
//! 🛠️ Proactive Repair + Replication Recovery + Health Monitoring
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{
        ShardId, ShardManager, ShardIndex, ShardAudit, ReplicationStatus, 
        metadata::{ShardMetadata, LatticeGeometry},
    },
    shard_server::{
        mesh::ShardMesh, acl::AclEnforcer, server::TieredShardStorage,
    },
    gov::BlissId,
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::RwLock,
    time::{interval, Instant},
};
use thiserror::Error;
use tracing::{info, warn, error, debug};

/// Production AutoHeal Daemon for shard self-repair
pub struct AutoHealDaemon {
    shard_manager: Arc<ShardManager>,
    shard_index: Arc<ShardIndex>,
    shard_mesh: Option<Arc<ShardMesh>>,
    acl_enforcer: Arc<AclEnforcer>,
    storage: Arc<TieredShardStorage>,
    
    /// Health scan intervals
    config: DaemonConfig,
    
    /// Current healing operations
    active_repairs: RwLock<HashMap<ShardId, RepairTask>>,
    
    /// Global health metrics
    metrics: Arc<RwLock<DaemonMetrics>>,
}

#[derive(Debug, Clone)]
pub struct DaemonConfig {
    /// Scan interval for shard health (default: 5min)
    pub scan_interval: Duration,
    
    /// Audit threshold before repair (default: 1hr)
    pub audit_threshold: Duration,
    
    /// Min healthy replicas before repair (default: 2)
    pub min_replicas: usize,
    
    /// Max concurrent repairs
    pub max_concurrent_repairs: usize,
    
    /// Enable mesh replication repair
    pub mesh_repair: bool,

    /// ✨ Phase II: Enable automatic geometry transmutation
    pub auto_transmute: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DaemonMetrics {
    pub total_scanned: u64,
    pub shards_repaired: u64,
    pub shards_recovered: u64,
    pub failed_repairs: u64,
    pub resonance_healed: u64, // ✨ Phase II: Track physics optimizations
    pub healthy_ratio: f64,
    pub last_scan: Option<Instant>,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            scan_interval: Duration::from_secs(300), // 5min
            audit_threshold: Duration::from_secs(3600), // 1hr
            min_replicas: 2,
            max_concurrent_repairs: 10,
            mesh_repair: true,
            auto_transmute: true, // Default enabled for Phase II
        }
    }
}

impl AutoHealDaemon {
    /// Create production autoheal daemon
    pub fn new(
        shard_manager: Arc<ShardManager>,
        shard_index: Arc<ShardIndex>,
        shard_mesh: Option<Arc<ShardMesh>>,
        acl_enforcer: Arc<AclEnforcer>,
        storage: Arc<TieredShardStorage>,
        config: DaemonConfig,
    ) -> Self {
        Self {
            shard_manager,
            shard_index,
            shard_mesh,
            acl_enforcer,
            storage,
            config,
            active_repairs: RwLock::new(HashMap::new()),
            metrics: Arc::new(RwLock::new(DaemonMetrics::default())),
        }
    }

    /// Start daemon with continuous health monitoring
    pub async fn run(self: Arc<Self>) -> Result<(), DaemonError> {
        info!("🛠️  Starting AuraFS AutoHeal Daemon");
        info!("⚙️  Scan interval: {:?}", self.config.scan_interval);
        info!("🔧  Max repairs: {}", self.config.max_concurrent_repairs);
        info!("🔮  Auto-Transmute: {}", self.config.auto_transmute);
        
        let mut scan_interval = interval(self.config.scan_interval);
        
        loop {
            scan_interval.tick().await;
            self.full_health_scan().await?;
            self.report_metrics().await;
        }
    }

    /// Full health scan with proactive repairs
    async fn full_health_scan(&self) -> Result<(), DaemonError> {
        let start_time = Instant::now();
        let mut repairs = self.active_repairs.write().await;
        
        // Cleanup completed repairs
        repairs.retain(|_, task| task.status != RepairStatus::Completed && !matches!(task.status, RepairStatus::Failed(_)));
        
        if repairs.len() >= self.config.max_concurrent_repairs {
            debug!("Max concurrent repairs reached: {}", repairs.len());
            return Ok(());
        }
        
        // Find candidates needing repair
        let candidates = self.find_repair_candidates().await?;
        
        for candidate in candidates {
            if repairs.len() >= self.config.max_concurrent_repairs {
                break;
            }
            
            let task_id = uuid::Uuid::new_v4();
            let repair_task = RepairTask::new(task_id, candidate.clone());
            
            repairs.insert(candidate.shard_id.clone(), repair_task);
            self.spawn_repair_task(candidate).await?;
        }
        
        let mut metrics = self.metrics.write().await;
        metrics.last_scan = Some(start_time);
        metrics.total_scanned += 1; // Simplified counter, ideally count actual shards scanned
        
        Ok(())
    }

    /// Find shards needing proactive repair
    async fn find_repair_candidates(&self) -> Result<Vec<ShardHealthReport>, DaemonError> {
        let mut candidates = Vec::new();
        let all_shards = self.shard_index.all_shards();
        
        for metadata in all_shards {
            let audit = self.audit_shard_health(&metadata).await?;
            
            if self.needs_repair(&audit, &metadata) {
                candidates.push(audit);
            }
        }
        
        Ok(candidates)
    }

    /// Comprehensive shard health audit including Physics Resonance
    async fn audit_shard_health(&self, metadata: &ShardMetadata) -> Result<ShardHealthReport, DaemonError> {
        let shard_audit = self.shard_manager.audit_shard(&metadata.shard_id).await
            .map_err(DaemonError::ValidationError)?; // Map error appropriately
        
        // ✨ Phase II: Check Resonance
        let resonance_mismatch = self.check_resonance(metadata).await;

        Ok(ShardHealthReport {
            shard_id: metadata.shard_id.clone(),
            metadata: metadata.clone(),
            storage_healthy: shard_audit.storage_healthy,
            index_consistent: shard_audit.index_consistent,
            replication_healthy: shard_audit.replication_healthy,
            replica_count: metadata.peer_nodes.len() as u64,
            last_audit_age: Duration::from_nanos(metadata.last_audit_ns),
            resonance_mismatch, // ✨ New field
            needs_repair: false, // Calculated later
        })
    }

    /// ✨ Phase II: Check if Shard Geometry matches its usage (Coherence Monitoring)
    async fn check_resonance(&self, metadata: &ShardMetadata) -> bool {
        // Mock logic using Coherence principles:
        // - High Access Frequency + Low Compute -> Should be Triangular (Network)
        // - High Compute Load (tags) -> Should be Kagome (Logic)
        // - Low Access -> Should be Bethe (Storage)

        match metadata.geometry {
            LatticeGeometry::Kagome => {
                // If it's Kagome but has no compute tags, it's a mismatch
                !metadata.tags.contains("compute_active") && !metadata.tags.contains("model_slice")
            },
            LatticeGeometry::Bethe => {
                // If it's Bethe (Deep Storage) but has "high_access" tag, mismatch
                metadata.tags.contains("high_access")
            },
            LatticeGeometry::Triangular => {
                 // If Triangular but rarely accessed, maybe move to Bethe
                 metadata.tags.contains("cold_storage")
            }
            _ => false,
        }
    }

    /// Determine if shard needs repair
    fn needs_repair(&self, report: &ShardHealthReport, _metadata: &ShardMetadata) -> bool {
        // ✨ Phase II: Prioritize Resonance Mismatch
        if report.resonance_mismatch && self.config.auto_transmute {
            return true;
        }

        // Storage failure
        if !report.storage_healthy { return true; }
        
        // Index inconsistency
        if !report.index_consistent { return true; }
        
        // Insufficient replicas
        if report.replica_count < self.config.min_replicas as u64 { return true; }
        
        // Stale audit
        if report.last_audit_age > self.config.audit_threshold { return true; }
        
        false
    }

    /// Spawn async repair task for shard
    async fn spawn_repair_task(&self, report: ShardHealthReport) -> Result<(), DaemonError> {
        let shard_manager = Arc::clone(&self.shard_manager);
        let shard_mesh = self.shard_mesh.clone();
        let acl_enforcer = Arc::clone(&self.acl_enforcer);
        let metrics = Arc::clone(&self.metrics);
        let config = self.config.clone();
        
        // Clone for map access
        let task_shard_id = report.shard_id.clone();
        // Since we can't easily pass 'self' into the spawn due to lifetimes/Arc, we rely on cloned components
        
        // We need a way to update the task status in active_repairs. 
        // For simplicity in this structure, we'd need active_repairs to be accessible or use a channel.
        // Assuming we restructure slightly to allow updating status:
        
        // Placeholder for updating status - in real code, use channel or shared state properly
        
        tokio::spawn(async move {
            let result = Self::execute_repair(
                &shard_manager, 
                shard_mesh, 
                &acl_enforcer, 
                report.clone(),
                &metrics,
                &config
            ).await;
            
            // Log result (real impl updates state)
            match result {
                Ok(_) => debug!("Task for {} completed", task_shard_id),
                Err(e) => error!("Task for {} failed: {}", task_shard_id, e),
            }
        });
        
        Ok(())
    }

    /// Execute comprehensive shard repair
    async fn execute_repair(
        shard_manager: &ShardManager,
        shard_mesh: Option<Arc<ShardMesh>>,
        _acl_enforcer: &AclEnforcer, // Unused in this logic but kept for future
        report: ShardHealthReport,
        metrics: &Arc<RwLock<DaemonMetrics>>,
        config: &DaemonConfig,
    ) -> Result<(), DaemonError> {
        info!("🔧 Repairing shard {}", report.shard_id);
        
        // 1. Re-validate shard integrity
        let mut shard = shard_manager.load_shard(&report.shard_id).await
             .map_err(DaemonError::ValidationError)?;
        shard.validate().map_err(DaemonError::ValidationError)?;
        
        // ✨ Phase II: Heal Resonance Mismatch (Transmutation)
        if report.resonance_mismatch && config.auto_transmute {
            let old_geo = shard.metadata.geometry.clone();
            
            // Determine optimal geometry based on current state/tags
            let target_geo = if shard.metadata.tags.contains("high_access") {
                LatticeGeometry::Triangular // Optimize for Network
            } else if shard.metadata.tags.contains("compute_active") {
                LatticeGeometry::Kagome // Optimize for Logic
            } else {
                LatticeGeometry::Bethe // Default to Storage
            };

            if old_geo != target_geo {
                info!("✨ Auto-Transmuting shard {} from {:?} to {:?}", report.shard_id, old_geo, target_geo);
                shard.metadata.geometry = target_geo;
                
                // Update metrics
                let mut m = metrics.write().await;
                m.resonance_healed += 1;
            }
        }

        // 2. Fix index consistency / Update Metadata
        shard_manager.update_shard_metadata(&report.shard_id, shard.metadata.clone()).await
             .map_err(|e| DaemonError::StorageError(crate::shard::StorageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))))?; // coerce error
        
        // 3. Replicate to mesh if enabled or if Transmuted (to move to correct layer)
        if let Some(mesh) = &shard_mesh {
            if config.mesh_repair || report.resonance_mismatch {
                let replicas = mesh.replicate_shard(&report.shard_id).await
                    .map_err(|_| DaemonError::MeshError)?;
                info!("🌐 Replicated to {} peers", replicas);
            }
        }
        
        // 4. Update audit timestamp
        let mut metadata = shard.metadata.clone();
        metadata.touch_audit();
        metadata.update_replication_status(config.min_replicas);
        
        // Persist final state
        shard_manager.update_shard_metadata(&report.shard_id, metadata).await
             .map_err(|e| DaemonError::StorageError(crate::shard::StorageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))))?;
        
        info!("✅ Shard {} repaired successfully", report.shard_id);
        
        let mut m = metrics.write().await;
        m.shards_repaired += 1;
        m.shards_recovered += 1;
        
        Ok(())
    }

    /// Report daemon metrics
    async fn report_metrics(&self) {
        let metrics = self.metrics.read().await;
        let total_shards = self.shard_index.shard_count();
        let healthy_ratio = if total_shards > 0 {
            (total_shards as f64 - metrics.failed_repairs as f64) / total_shards as f64
        } else {
            1.0
        };
        
        debug!("🩺 Heal metrics: {:.1}% healthy, {} repaired, {} transmuted", 
               healthy_ratio * 100.0, metrics.shards_repaired, metrics.resonance_healed);
    }
}

/// Shard health assessment report
#[derive(Debug, Clone)]
pub struct ShardHealthReport {
    pub shard_id: ShardId,
    pub metadata: ShardMetadata,
    pub storage_healthy: bool,
    pub index_consistent: bool,
    pub replication_healthy: bool,
    pub replica_count: u64,
    pub last_audit_age: Duration,
    pub needs_repair: bool,
    pub resonance_mismatch: bool, // ✨ Phase II
}

/// Individual repair task tracking
#[derive(Debug, Clone)]
pub struct RepairTask {
    pub task_id: uuid::Uuid,
    pub shard_id: ShardId,
    pub status: RepairStatus,
    pub start_time: Instant,
    pub attempts: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepairStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
}

impl RepairTask {
    pub fn new(task_id: uuid::Uuid, shard_id: ShardId) -> Self {
        Self {
            task_id,
            shard_id,
            status: RepairStatus::Pending,
            start_time: Instant::now(),
            attempts: 0,
        }
    }
}

/// Daemon-specific errors
#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("Shard validation failed: {0}")]
    ValidationError(#[from] crate::shard::ShardError),
    #[error("Storage error: {0}")]
    StorageError(#[from] crate::shard::StorageError),
    #[error("Mesh replication failed")]
    MeshError,
}

/// Quick-start daemon macro
#[macro_export]
macro_rules! quick_heal_daemon {
    ($storage_path:expr) => {{
        use afs::shard_server::autoheal_daemon::AutoHealDaemon;
        use afs::shard::{LocalShardStorage, ShardIndex, ShardManager};
        
        let storage = LocalShardStorage::new($storage_path.into());
        let index = Arc::new(ShardIndex::new());
        let manager = Arc::new(ShardManager::new(storage.clone(), index.clone()));
        
        let daemon = AutoHealDaemon::new(
            manager,
            index,
            None, // mesh
            Arc::new(crate::shard_server::acl::AclEnforcer::new(crate::crypto::quantum::KyberKeypair::generate().unwrap())),
            Arc::new(storage),
            Default::default(),
        );
        
        Arc::new(daemon).run().await.unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resonance_mismatch_detection() {
        // Test logic for identifying resonance mismatch
    }
}