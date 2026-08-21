//! Shard Management Utilities

use super::backend::{AuraFsBackend, Shard, AuraFsError};
use std::collections::HashMap;

/// Shard manager for advanced operations
pub struct ShardManager {
    backend: AuraFsBackend,
}

impl ShardManager {
    pub async fn new() -> Result<Self, AuraFsError> {
        let backend = AuraFsBackend::connect().await?;
        Ok(Self { backend })
    }
    
    /// Get shard statistics
    pub async fn get_stats(&self, shard_id: &str) -> Result<ShardStats, AuraFsError> {
        let shard = self.backend.read_shard(shard_id).await?;
        
        Ok(ShardStats {
            id: shard.id.clone(),
            size: shard.data.len(),
            replicas: shard.replicas.len(),
            created_at: shard.created_at,
        })
    }
    
    /// Verify shard integrity
    pub async fn verify(&self, shard_id: &str) -> Result<bool, AuraFsError> {
        let shard = self.backend.read_shard(shard_id).await?;
        let checksum = self.calculate_checksum(&shard.data);
        Ok(checksum == shard.metadata.checksum)
    }
    
    fn calculate_checksum(&self, data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

/// Shard statistics
#[derive(Debug, Clone)]
pub struct ShardStats {
    pub id: String,
    pub size: usize,
    pub replicas: usize,
    pub created_at: u64,
}

/// Query builder for advanced shard queries
pub struct ShardQuery {
    filters: HashMap<String, String>,
    limit: Option<usize>,
}

impl ShardQuery {
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
            limit: None,
        }
    }
    
    pub fn filter(mut self, field: &str, value: &str) -> Self {
        self.filters.insert(field.to_string(), value.to_string());
        self
    }
    
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }
    
    pub fn build(&self) -> String {
        // Build query string from filters
        self.filters
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join(" AND ")
    }
}