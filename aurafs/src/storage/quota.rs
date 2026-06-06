//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Quota - Soul-Based Resource Limits Engine
//! 🗄️ Storage Limits + Shard Count + Inode Quotas + Enforcement
//! 
//! ⚛️  Lattice Physics: The "Conservation Laws" of the system.
//!     Ensures Souls do not consume infinite Lattice resources.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::gov::BlissId;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    sync::RwLock,
    time::Instant,
};
use thiserror::Error;
use tracing::{info, warn, debug};
use serde::{Deserialize, Serialize};

/// Soul quota limits for storage resources
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SoulQuota {
    /// Max storage in bytes (e.g. 100GB)
    pub storage_bytes: u64,
    /// Max number of distinct shards
    pub shard_count: u64,
    /// Max number of inodes (files/directories)
    pub inode_count: u64,
}

/// Rate limit configuration for API protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per time window
    pub max_requests: u64,
    /// Time window in seconds
    pub window_secs: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 1000,
            window_secs: 60,
        }
    }
}

/// Usage snapshot for history tracking (Time-Series Data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSnapshot {
    pub timestamp: u64,
    pub storage_bytes: u64,
    pub shard_count: u64,
    pub inode_count: u64,
}

/// Enterprise-grade quota error with detailed context
#[derive(Debug, Error)]
pub enum QuotaError {
    #[error("Quota exceeded for soul {0}: {1}")]
    Exceeded(String, String),
    #[error("Rate limit exceeded for soul {0}. Retry later.")]
    RateLimit(String),
    #[error("Internal quota error: {0}")]
    Internal(String),
    #[error("Invalid quota configuration: {0}")]
    InvalidConfig(String),
    #[error("Quota operation timeout")]
    Timeout,
}

/// Enterprise-grade quota manager with rate limiting and usage history
pub struct SoulQuotaManager {
    /// Hard limits per Soul
    quotas: Arc<RwLock<HashMap<BlissId, SoulQuota>>>,
    
    /// Current resource usage per Soul
    usage: Arc<RwLock<HashMap<BlissId, SoulQuota>>>,
    
    /// Rate limiting state: soul -> (last_check_time, request_count)
    rate_limits: Arc<RwLock<HashMap<BlissId, (Instant, u64)>>>,
    
    /// Usage history for monitoring/analytics
    usage_history: Arc<RwLock<HashMap<BlissId, Vec<UsageSnapshot>>>>,
    
    /// Rate limit configuration
    rate_limit_config: RateLimitConfig,
}

impl SoulQuotaManager {
    /// Create new quota manager with enterprise defaults
    pub fn new() -> Self {
        Self::with_rate_limits(RateLimitConfig::default())
    }
    
    /// Create quota manager with custom rate limits
    pub fn with_rate_limits(config: RateLimitConfig) -> Self {
        Self {
            quotas: Arc::new(RwLock::new(HashMap::new())),
            usage: Arc::new(RwLock::new(HashMap::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            usage_history: Arc::new(RwLock::new(HashMap::new())),
            rate_limit_config: config,
        }
    }
    
    /// Default manager wrapped in Arc
    pub fn default() -> Arc<Self> {
        Arc::new(Self::new())
    }
    
    /// Set quota limits for a specific soul
    pub async fn set_quota(&self, soul: BlissId, quota: SoulQuota) {
        let mut quotas = self.quotas.write().await;
        quotas.insert(soul.clone(), quota);
        info!("⚖️  Quota limits set for soul {}", soul);
    }
    
    /// Check if an operation fits in quota limits AND rate limits
    pub async fn check_quota(
        &self,
        soul: &BlissId,
        shards_delta: u64,
        storage_delta: u64,
        inodes_delta: u64,
    ) -> Result<(), QuotaError> {
        // 1. Rate limiting check (DoS protection)
        if !self.check_rate_limit(soul).await {
            warn!("⛔ Rate limit exceeded for soul {}", soul);
            return Err(QuotaError::RateLimit(soul.to_string()));
        }
        
        // 2. Resource limits check
        let quotas = self.quotas.read().await;
        let usage = self.usage.read().await;
        
        let quota = quotas.get(soul).unwrap_or(&SoulQuota::default());
        let used = usage.get(soul).unwrap_or(&SoulQuota::default());
        
        // Check Shard Count
        if quota.shard_count > 0 && used.shard_count + shards_delta > quota.shard_count {
            let msg = format!("Shard count: {}/{} (requested +{})", 
                used.shard_count, quota.shard_count, shards_delta);
            return Err(QuotaError::Exceeded(soul.to_string(), msg));
        }
        
        // Check Storage Bytes
        if quota.storage_bytes > 0 && used.storage_bytes + storage_delta > quota.storage_bytes {
            let msg = format!("Storage: {:.2}GB/{:.2}GB (requested +{:.2}GB)", 
                used.storage_bytes as f64 / 1e9,
                quota.storage_bytes as f64 / 1e9,
                storage_delta as f64 / 1e9);
            return Err(QuotaError::Exceeded(soul.to_string(), msg));
        }
        
        // Check Inode Count
        if quota.inode_count > 0 && used.inode_count + inodes_delta > quota.inode_count {
            let msg = format!("Inode count: {}/{} (requested +{})", 
                used.inode_count, quota.inode_count, inodes_delta);
            return Err(QuotaError::Exceeded(soul.to_string(), msg));
        }
        
        Ok(())
    }
    
    /// Check and update rate limit for a soul
    async fn check_rate_limit(&self, soul: &BlissId) -> bool {
        let now = Instant::now();
        let mut rate_limits = self.rate_limits.write().await;
        
        let entry = rate_limits.entry(soul.clone()).or_insert_with(|| (now, 0));
        
        // Reset if window expired
        if now.duration_since(entry.0).as_secs() >= self.rate_limit_config.window_secs {
            entry.0 = now;
            entry.1 = 0;
        }
        
        // Check if limit exceeded
        if entry.1 >= self.rate_limit_config.max_requests {
            return false;
        }
        
        entry.1 += 1;
        true
    }
    
    /// Get usage history for analytics
    pub async fn get_usage_history(&self, soul: &BlissId, limit: usize) -> Vec<UsageSnapshot> {
        let history = self.usage_history.read().await;
        history.get(soul)
            .map(|h| h.iter().rev().take(limit).cloned().collect())
            .unwrap_or_default()
    }
    
    /// Atomically increment usage after a successful operation
    pub async fn increment_usage(
        &self,
        soul: &BlissId,
        shards_delta: u64,
        storage_delta: u64,
        inodes_delta: u64,
    ) -> Result<(), QuotaError> {
        // Update usage state
        let (new_shard_count, new_storage_bytes, new_inode_count) = {
            let mut usage = self.usage.write().await;
            let entry = usage.entry(soul.clone()).or_default();
            entry.shard_count += shards_delta;
            entry.storage_bytes += storage_delta;
            entry.inode_count += inodes_delta;
            (entry.shard_count, entry.storage_bytes, entry.inode_count)
        };
        
        // Record usage snapshot for history
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let snapshot = UsageSnapshot {
            timestamp,
            storage_bytes: new_storage_bytes,
            shard_count: new_shard_count,
            inode_count: new_inode_count,
        };
        
        let mut history = self.usage_history.write().await;
        let history_entry = history.entry(soul.clone()).or_insert_with(Vec::new);
        history_entry.push(snapshot);
        
        // Retention policy: Keep last 1000 snapshots
        if history_entry.len() > 1000 {
            history_entry.remove(0);
        }
        
        debug!("📈 Usage update for {}: +{}b, +{} shards", soul, storage_delta, shards_delta);
        Ok(())
    }
    
    /// Decrease usage for soul on deletion
    pub async fn decrement_usage(
        &self,
        soul: &BlissId,
        shards_delta: u64,
        storage_delta: u64,
        inodes_delta: u64,
    ) {
        let mut usage = self.usage.write().await;
        let entry = usage.entry(soul.clone()).or_default();
        entry.shard_count = entry.shard_count.saturating_sub(shards_delta);
        entry.storage_bytes = entry.storage_bytes.saturating_sub(storage_delta);
        entry.inode_count = entry.inode_count.saturating_sub(inodes_delta);
        
        debug!("📉 Usage decrease for {}: -{}b, -{} shards", soul, storage_delta, shards_delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gov::BlissId;

    #[tokio::test]
    async fn test_quota_limits() {
        let quota_mgr = SoulQuotaManager::new();
        let soul = BlissId::new(); // Assuming BlissId::new() exists for testing
        
        quota_mgr.set_quota(soul.clone(), SoulQuota {
            storage_bytes: 1_000,
            shard_count: 10,
            inode_count: 5,
        }).await;
        
        // Within limits
        assert!(quota_mgr.check_quota(&soul, 5, 500, 2).await.is_ok());
        
        // Exceed storage
        assert!(matches!(
            quota_mgr.check_quota(&soul, 1, 1001, 1).await,
            Err(QuotaError::Exceeded(_, _))
        ));
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let config = RateLimitConfig { max_requests: 2, window_secs: 1 };
        let quota_mgr = SoulQuotaManager::with_rate_limits(config);
        let soul = BlissId::new();

        assert!(quota_mgr.check_quota(&soul, 0, 0, 0).await.is_ok());
        assert!(quota_mgr.check_quota(&soul, 0, 0, 0).await.is_ok());
        // Third request should fail
        assert!(matches!(
            quota_mgr.check_quota(&soul, 0, 0, 0).await,
            Err(QuotaError::RateLimit(_))
        ));
    }
}