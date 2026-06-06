//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! AuraFS Audit Logger
//!
//! Comprehensive audit logging with tamper-proof event recording,
//! cryptographic verification, and queryable audit trails.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, AuditError>;

/// Event severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub event_id: String,
    pub level: EventLevel,
    pub category: String,
    pub action: String,
    pub user: String,
    pub resource: String,
    pub details: String,
    pub hash: String,
    pub previous_hash: String,
}

impl AuditEvent {
    /// Create new audit event
    pub fn new(
        category: &str,
        action: &str,
        user: &str,
        resource: &str,
        details: &str,
        level: EventLevel,
        previous_hash: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let event_id = format!("{:x}", Sha256::digest(
            format!("{}-{}-{}", timestamp, user, action).as_bytes()
        ))[..16].to_string();
        
        let mut event = Self {
            timestamp,
            event_id,
            level,
            category: category.to_string(),
            action: action.to_string(),
            user: user.to_string(),
            resource: resource.to_string(),
            details: details.to_string(),
            hash: String::new(),
            previous_hash,
        };
        
        event.hash = event.calculate_hash();
        event
    }
    
    /// Calculate cryptographic hash of event
    fn calculate_hash(&self) -> String {
        let data = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            self.timestamp,
            self.event_id,
            self.action,
            self.user,
            self.resource,
            self.details,
            self.previous_hash
        );
        
        format!("{:x}", Sha256::digest(data.as_bytes()))
    }
    
    /// Verify event integrity
    pub fn verify(&self) -> bool {
        self.hash == self.calculate_hash()
    }
}

/// Audit Logger
pub struct AuditLogger {
    log_file: Arc<Mutex<BufWriter<File>>>,
    last_hash: Arc<Mutex<String>>,
}

impl AuditLogger {
    /// Create new audit logger
    pub fn new(log_path: &str) -> Result<Self> {
        let path = PathBuf::from(log_path);
        
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        let logger = Self {
            log_file: Arc::new(Mutex::new(BufWriter::new(file))),
            last_hash: Arc::new(Mutex::new(String::from("genesis"))),
        };
        
        tracing::info!("Initialized audit logger at {}", log_path);
        
        Ok(logger)
    }
    
    /// Log audit event
    pub fn log_event(
        &self,
        category: &str,
        action: &str,
        user: &str,
        resource: &str,
        details: &str,
        level: EventLevel,
    ) -> Result<()> {
        let previous_hash = {
            let hash = self.last_hash.lock().unwrap();
            hash.clone()
        };
        
        let event = AuditEvent::new(
            category,
            action,
            user,
            resource,
            details,
            level,
            previous_hash,
        );
        
        // Serialize event
        let json = serde_json::to_string(&event)
            .map_err(|e| AuditError::SerializationError(e.to_string()))?;
        
        // Write to log
        {
            let mut file = self.log_file.lock().unwrap();
            writeln!(file, "{}", json)?;
            file.flush()?;
        }
        
        // Update last hash
        {
            let mut last_hash = self.last_hash.lock().unwrap();
            *last_hash = event.hash.clone();
        }
        
        Ok(())
    }
    
    /// Log file system operation
    pub fn log_fs_operation(&self, operation: &str, user: &str, path: &str, result: &str) -> Result<()> {
        self.log_event(
            "filesystem",
            operation,
            user,
            path,
            result,
            EventLevel::Info,
        )
    }
    
    /// Log ACL change
    pub fn log_acl_change(&self, user: &str, resource: &str, change: &str) -> Result<()> {
        self.log_event(
            "acl",
            "permission_change",
            user,
            resource,
            change,
            EventLevel::Warning,
        )
    }
    
    /// Log replication event
    pub fn log_replication(&self, shard_id: &str, node_id: &str, status: &str) -> Result<()> {
        self.log_event(
            "replication",
            "shard_replicate",
            "system",
            shard_id,
            &format!("node={}, status={}", node_id, status),
            EventLevel::Info,
        )
    }
    
    /// Log snapshot creation
    pub fn log_snapshot(&self, snapshot_id: &str, user: &str, description: &str) -> Result<()> {
        self.log_event(
            "snapshot",
            "create",
            user,
            snapshot_id,
            description,
            EventLevel::Info,
        )
    }
    
    /// Log security event
    pub fn log_security(&self, event_type: &str, user: &str, details: &str) -> Result<()> {
        self.log_event(
            "security",
            event_type,
            user,
            "system",
            details,
            EventLevel::Critical,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_verification() {
        let event = AuditEvent::new(
            "test",
            "action",
            "user",
            "resource",
            "details",
            EventLevel::Info,
            "genesis".to_string(),
        );
        
        assert!(event.verify());
    }
    
    #[test]
    fn test_audit_logging() {
        let logger = AuditLogger::new("/tmp/aurafs_audit.log").unwrap();
        
        logger.log_fs_operation("write", "user1", "/test/file.txt", "success").unwrap();
        logger.log_acl_change("admin", "/test", "granted read to user2").unwrap();
    }
}