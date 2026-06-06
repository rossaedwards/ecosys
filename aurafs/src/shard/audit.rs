//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Audit Engine - Bio-Resonant Health Verification
//! 🔍 Verifies Integrity, Index Consistency, and Lattice Resonance
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::shard::{ShardId, ShardMetadata, LatticeGeometry, ReplicationStatus};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Comprehensive shard audit report
/// 
/// This is the "Medical Chart" for a shard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardAudit {
    /// Shard ID being audited
    pub shard_id: ShardId,
    
    /// 🟢 HEALTH CHECKS
    /// Storage backend health (Is the file actually on disk/s3?)
    pub storage_healthy: bool,
    /// Index consistency check (Does the map point to the territory?)
    pub index_consistent: bool,
    /// Replication health status (Do we have enough copies?)
    pub replication_healthy: bool,
    /// Quantum signature validity (Is the Dilithium signature valid?)
    pub signature_valid: bool,
    
    /// 🟢 BIO-RESONANCE CHECKS (Phase II)
    /// Does the stored geometry match the current Core Coherence?
    pub geometry_resonant: bool,
    /// The computed coherence score (0.0 - 1.0)
    pub coherence_score: f64,
    
    /// 🟢 ACTIONABLE STATE
    /// Whether shard needs repair (re-download, re-replicate)
    pub needs_repair: bool,
    /// Whether shard needs transmutation (Geometry shift)
    pub needs_transmutation: bool,
    
    /// Audit timestamp (unix nanos)
    pub audit_timestamp_ns: u64,
    
    /// Detailed error messages
    pub errors: Vec<String>,
    /// Warning messages
    pub warnings: Vec<String>,
    
    /// Current replica count
    pub replica_count: usize,
}

impl ShardAudit {
    /// Create new audit report
    pub fn new(shard_id: ShardId) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
            
        Self {
            shard_id,
            storage_healthy: false,
            index_consistent: false,
            replication_healthy: false,
            signature_valid: false,
            
            // Phase II Defaults
            geometry_resonant: true, // Assume innocent until proven dissonant
            coherence_score: 1.0,
            
            needs_repair: false,
            needs_transmutation: false,
            
            audit_timestamp_ns: now,
            errors: Vec::new(),
            warnings: Vec::new(),
            replica_count: 0,
        }
    }

    /// Mark as storage healthy
    pub fn with_storage_health(mut self, healthy: bool, replicas: usize) -> Self {
        self.storage_healthy = healthy;
        self.replica_count = replicas;
        if !healthy {
            self.needs_repair = true;
            self.errors.push("Storage corruption or missing file".to_string());
        }
        self
    }
    
    /// Check Bio-Resonance (Phase II)
    /// compares the shard's fixed geometry against the dynamic "Ideal" geometry
    /// returned by the Core Observer.
    pub fn check_resonance(mut self, current: LatticeGeometry, ideal: LatticeGeometry) -> Self {
        if current == ideal {
            self.geometry_resonant = true;
            self.coherence_score = 1.0;
        } else {
            // Mismatch detected!
            self.geometry_resonant = false;
            // Coherence drops based on how "far" the geometries are (simplified)
            self.coherence_score = 0.5; 
            
            self.needs_transmutation = true;
            self.warnings.push(format!(
                "Resonance Mismatch: Shard is {:?} but Vacuum suggests {:?}. Transmutation recommended.",
                current, ideal
            ));
        }
        self
    }
    
    /// Calculate overall health score (0.0 to 1.0)
    pub fn health_score(&self) -> f64 {
        let mut score = 0.0;
        let mut factors = 0.0;
        
        // Storage (Weighted 30%)
        if self.storage_healthy { score += 0.3; }
        factors += 0.3;
        
        // Index (Weighted 20%)
        if self.index_consistent { score += 0.2; }
        factors += 0.2;
        
        // Replication (Weighted 20%)
        if self.replication_healthy { score += 0.2; }
        factors += 0.2;
        
        // Signature (Weighted 15%)
        if self.signature_valid { score += 0.15; }
        factors += 0.15;
        
        // Resonance (Weighted 15% - Phase II)
        if self.geometry_resonant { score += 0.15; }
        factors += 0.15;
        
        if factors > 0.0 { score / factors } else { 0.0 }
    }
    
    /// Is the shard completely healthy?
    pub fn is_healthy(&self) -> bool {
        self.storage_healthy 
            && self.index_consistent 
            && !self.needs_repair
            && self.errors.is_empty()
    }
    
    /// Does it require immediate intervention?
    pub fn requires_action(&self) -> bool {
        self.needs_repair || self.needs_transmutation
    }
}

// ═══════════════════════════════════════════════════════════════════
// Audit Engine (The Doctor)
// ═══════════════════════════════════════════════════════════════════

/// Service that performs audits
pub struct AuditEngine {
    // Config for thresholds
    pub replication_threshold: usize,
}

impl AuditEngine {
    pub fn new(replication_threshold: usize) -> Self {
        Self { replication_threshold }
    }
    
    /// Verify replication status
    pub fn verify_replication(&self, audit: &mut ShardAudit, status: &ReplicationStatus) {
        match status {
            ReplicationStatus::Healthy => {
                audit.replication_healthy = true;
            },
            ReplicationStatus::Degraded => {
                audit.replication_healthy = false;
                audit.warnings.push("Replication degraded (not enough peers)".to_string());
                // Only repair if critically low
                if audit.replica_count == 0 {
                    audit.needs_repair = true;
                }
            },
            _ => {
                audit.replication_healthy = false;
                audit.warnings.push("Replication status unknown".to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audit_scoring() {
        let shard_id = ShardId::from_content(b"test");
        let mut audit = ShardAudit::new(shard_id);
        
        // Perfect health
        audit.storage_healthy = true;
        audit.index_consistent = true;
        audit.replication_healthy = true;
        audit.signature_valid = true;
        audit.geometry_resonant = true;
        
        assert_eq!(audit.health_score(), 1.0);
        assert!(audit.is_healthy());
        
        // Degraded Resonance
        audit.geometry_resonant = false;
        assert!(audit.health_score() < 1.0);
        assert!(audit.is_healthy()); // Still "healthy" data, just poor physics
        assert!(audit.requires_action()); // Needs transmutation
    }
}