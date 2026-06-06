//! Cryptographic append-only audit log for AuraFS governance
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use sha3::{Digest, Sha3_256};
use chrono::{Utc, DateTime};
use log::{debug, error};

/// Audit log entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: u64,
    pub event_type: String,
    pub description: String,
    pub timestamp: i64,
    pub hash: String,
    pub previous_hash: String,
}

#[derive(Debug)]
pub struct AuditLogger {
    entries: Arc<Mutex<Vec<AuditEntry>>>,
}

impl AuditLogger {
    /// Create new empty audit logger
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Append new event immutably
    pub fn log_event(&self, event_type: &str, description: &str) {
        let mut entries = self.entries.lock().unwrap();

        let id = entries.len() as u64;
        let timestamp = Utc::now().timestamp();
        let previous_hash = entries.last()
            .map(|e| e.hash.clone())
            .unwrap_or_else(|| "0".repeat(64));

        let hash = Self::compute_hash(id, event_type, description, timestamp, &previous_hash);

        let entry = AuditEntry {
            id,
            event_type: event_type.to_string(),
            description: description.to_string(),
            timestamp,
            hash,
            previous_hash,
        };

        entries.push(entry);
        debug!("📜 Audit log event: '{}' - {}", event_type, description);
    }

    /// Compute SHA3-256 cryptographic hash for audit entry fields chain
    fn compute_hash(
        id: u64,
        event_type: &str,
        description: &str,
        timestamp: i64,
        previous_hash: &str,
    ) -> String {
        let data = format!("{}:{}:{}:{}:{}", id, event_type, description, timestamp, previous_hash);
        let mut hasher = Sha3_256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify the integrity of the entire audit log chain
    pub fn verify_integrity(&self) -> Result<bool, String> {
        let entries = self.entries.lock().unwrap();

        for (i, entry) in entries.iter().enumerate() {
            let expected_hash = Self::compute_hash(
                entry.id,
                &entry.event_type,
                &entry.description,
                entry.timestamp,
                &entry.previous_hash,
            );

            if entry.hash != expected_hash {
                error!("🔴 Hash mismatch at entry {}: expected={}, got={}", i, expected_hash, entry.hash);
                return Err(format!("Hash mismatch at entry {}", i));
            }

            if i > 0 {
                let prev_entry = &entries[i - 1];
                if entry.previous_hash != prev_entry.hash {
                    error!("🔴 Chain broken at entry {}", i);
                    return Err(format!("Chain broken at entry {}", i));
                }
            }
        }

        Ok(true)
    }

    /// Retrieve a complete clone of all audit entries
    pub fn get_entries(&self) -> Vec<AuditEntry> {
        self.entries.lock().unwrap().clone()
    }

    /// Export audit log entries as pretty JSON string
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.get_entries())
            .map_err(|e| format!("Audit log export failed: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_integrity() {
        let logger = AuditLogger::new();

        logger.log_event("test_event_1", "First event");
        logger.log_event("test_event_2", "Second event");
        logger.log_event("test_event_3", "Third event");

        assert!(logger.verify_integrity().is_ok());

        let entries = logger.get_entries();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[1].previous_hash, entries[0].hash);
    }
}
/// Verify data integrity using stored hash
fn verify_hash(data: &[u8], expected_hash: &str) -> Result<bool> {
    use sha3::{Digest, Sha3_512};

    let mut hasher = Sha3_512::new();
    hasher.update(data);
    let computed_hash = format!("{:x}", hasher.finalize());
    Ok(computed_hash == expected_hash)
}