//! ═══════════════════════════════════════════════════════════════════
//! Self-Healing & Recovery Module
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎
//!
//! Automated healing strategies for shard recovery, network repair,
//! and system resilience.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};

/// Repair strategies and implementations
pub mod repair;

// ═══════════════════════════════════════════════════════════════════
// HEALING STRATEGIES
// ═══════════════════════════════════════════════════════════════════

/// Healing strategy types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealingStrategy {
    /// Aggressive - heal immediately
    Aggressive,
    /// Conservative - wait and verify
    Conservative,
    /// Adaptive - based on system load
    Adaptive,
    /// Manual - require operator intervention
    Manual,
}

/// Healing priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HealingPriority {
    /// Low priority - heal when convenient
    Low = 0,
    /// Normal priority
    Normal = 1,
    /// High priority - heal soon
    High = 2,
    /// Critical - heal immediately
    Critical = 3,
}

/// A healing task
#[derive(Debug, Clone)]
pub struct HealingTask {
    /// Unique task ID
    pub id: String,
    /// Task type
    pub task_type: HealingTaskType,
    /// Priority level
    pub priority: HealingPriority,
    /// Strategy to use
    pub strategy: HealingStrategy,
    /// Creation time
    pub created_at: Instant,
    /// Number of retry attempts
    pub retries: u32,
    /// Maximum retries allowed
    pub max_retries: u32,
}

/// Types of healing tasks
#[derive(Debug, Clone)]
pub enum HealingTaskType {
    /// Recover a missing shard
    ShardRecovery {
        /// Shard ID to recover
        shard_id: String,
        /// Known replica locations
        replicas: Vec<String>,
    },
    /// Repair network connection
    NetworkRepair {
        /// Peer address
        peer_addr: String,
        /// Failure type
        failure_type: String,
    },
    /// Rebuild index
    IndexRebuild {
        /// Index type
        index_type: String,
    },
    /// Verify data integrity
    IntegrityCheck {
        /// Path to verify
        path: String,
    },
}

// ═══════════════════════════════════════════════════════════════════
// HEALING ENGINE
// ═══════════════════════════════════════════════════════════════════

/// The main healing engine
#[derive(Debug)]
pub struct HealingEngine {
    /// Pending healing tasks
    pending: RwLock<Vec<HealingTask>>,
    /// Active healing tasks
    active: RwLock<HashMap<String, HealingTask>>,
    /// Completed task IDs
    completed: RwLock<HashSet<String>>,
    /// Failed task IDs with error messages
    failed: RwLock<HashMap<String, String>>,
    /// Current strategy
    strategy: RwLock<HealingStrategy>,
    /// Is engine running
    running: RwLock<bool>,
}

impl HealingEngine {
    /// Create new healing engine
    pub fn new(strategy: HealingStrategy) -> Self {
        Self {
            pending: RwLock::new(Vec::new()),
            active: RwLock::new(HashMap::new()),
            completed: RwLock::new(HashSet::new()),
            failed: RwLock::new(HashMap::new()),
            strategy: RwLock::new(strategy),
            running: RwLock::new(false),
        }
    }

    /// Submit a healing task
    pub async fn submit(&self, task: HealingTask) {
        info!("Submitting healing task: {} ({:?})", task.id, task.task_type);
        let mut pending = self.pending.write().await;
        pending.push(task);
        // Sort by priority (highest first)
        pending.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Submit shard recovery task
    pub async fn submit_shard_recovery(
        &self,
        shard_id: &str,
        replicas: Vec<String>,
        priority: HealingPriority,
    ) {
        let task = HealingTask {
            id: uuid::Uuid::new_v4().to_string(),
            task_type: HealingTaskType::ShardRecovery {
                shard_id: shard_id.to_string(),
                replicas,
            },
            priority,
            strategy: *self.strategy.read().await,
            created_at: Instant::now(),
            retries: 0,
            max_retries: 3,
        };
        self.submit(task).await;
    }

    /// Get next pending task
    pub async fn next_task(&self) -> Option<HealingTask> {
        let mut pending = self.pending.write().await;
        if pending.is_empty() {
            return None;
        }
        
        let task = pending.remove(0);
        let mut active = self.active.write().await;
        active.insert(task.id.clone(), task.clone());
        
        Some(task)
    }

    /// Mark task as completed
    pub async fn complete(&self, task_id: &str) {
        let mut active = self.active.write().await;
        if active.remove(task_id).is_some() {
            let mut completed = self.completed.write().await;
            completed.insert(task_id.to_string());
            info!("Healing task completed: {}", task_id);
        }
    }

    /// Mark task as failed
    pub async fn fail(&self, task_id: &str, error: &str) {
        let mut active = self.active.write().await;
        if let Some(task) = active.remove(task_id) {
            if task.retries < task.max_retries {
                // Retry the task
                let mut retry_task = task;
                retry_task.retries += 1;
                warn!(
                    "Healing task {} failed (attempt {}/{}): {}",
                    task_id, retry_task.retries, retry_task.max_retries, error
                );
                self.submit(retry_task).await;
            } else {
                // Max retries exceeded
                error!("Healing task {} permanently failed: {}", task_id, error);
                let mut failed = self.failed.write().await;
                failed.insert(task_id.to_string(), error.to_string());
            }
        }
    }

    /// Get healing statistics
    pub async fn stats(&self) -> HealingStats {
        HealingStats {
            pending: self.pending.read().await.len(),
            active: self.active.read().await.len(),
            completed: self.completed.read().await.len(),
            failed: self.failed.read().await.len(),
        }
    }

    /// Start the healing loop
    pub async fn start(&self) {
        *self.running.write().await = true;
        info!("🏥 Healing engine started");
    }

    /// Stop the healing loop
    pub async fn stop(&self) {
        *self.running.write().await = false;
        info!("🛑 Healing engine stopped");
    }

    /// Check if engine is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

impl Default for HealingEngine {
    fn default() -> Self {
        Self::new(HealingStrategy::Adaptive)
    }
}

/// Healing statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct HealingStats {
    /// Pending tasks
    pub pending: usize,
    /// Active tasks
    pub active: usize,
    /// Completed tasks
    pub completed: usize,
    /// Failed tasks
    pub failed: usize,
}

// ═══════════════════════════════════════════════════════════════════
// REPAIR IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════

/// Execute a healing task
pub async fn execute_task(task: &HealingTask) -> Result<(), String> {
    debug!("Executing healing task: {} ({:?})", task.id, task.task_type);
    
    match &task.task_type {
        HealingTaskType::ShardRecovery { shard_id, replicas } => {
            repair::recover_shard(shard_id, replicas).await
        }
        HealingTaskType::NetworkRepair { peer_addr, failure_type } => {
            repair::repair_network(peer_addr, failure_type).await
        }
        HealingTaskType::IndexRebuild { index_type } => {
            repair::rebuild_index(index_type).await
        }
        HealingTaskType::IntegrityCheck { path } => {
            repair::verify_integrity(path).await
        }
    }
}

/// Initialize healing subsystem
pub fn init() -> HealingEngine {
    info!("🏥 Healing subsystem initialized");
    HealingEngine::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_healing_engine() {
        let engine = HealingEngine::new(HealingStrategy::Conservative);
        
        engine.submit_shard_recovery(
            "shard-123",
            vec!["node1".to_string(), "node2".to_string()],
            HealingPriority::High,
        ).await;

        let stats = engine.stats().await;
        assert_eq!(stats.pending, 1);
        
        let task = engine.next_task().await.unwrap();
        assert_eq!(stats.pending, 1);
        
        engine.complete(&task.id).await;
        let stats = engine.stats().await;
        assert_eq!(stats.completed, 1);
    }

    #[tokio::test]
    async fn test_priority_ordering() {
        let engine = HealingEngine::default();
        
        engine.submit_shard_recovery("low", vec![], HealingPriority::Low).await;
        engine.submit_shard_recovery("critical", vec![], HealingPriority::Critical).await;
        engine.submit_shard_recovery("normal", vec![], HealingPriority::Normal).await;

        // Should get critical first
        let task = engine.next_task().await.unwrap();
        assert_eq!(task.priority, HealingPriority::Critical);
    }
}