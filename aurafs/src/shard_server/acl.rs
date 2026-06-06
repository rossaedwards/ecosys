//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Server ACL - Soul-Based Access Control
//! 🔐 Quantum-Safe Permissions, Fractal Inheritance, Audit Trails
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    gov::{BlissId, SoulProof},
    shard::{ShardId, ShardMetadata},
    crypto::quantum::{DilithiumSignature, KyberKeypair},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

/// Soul-based Access Control List for AuraFS shards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardACL {
    /// Owner BlissId (full control)
    pub owner: BlissId,
    
    /// Read permissions (fractal inheritance aware)
    pub read_perms: BTreeSet<BlissId>,
    
    /// Write permissions
    pub write_perms: BTreeSet<BlissId>,
    
    /// Admin permissions (modify ACL)
    pub admin_perms: BTreeSet<BlissId>,

    /// Transmute permissions (change Lattice Geometry)
    /// Controls who can alter the shard's physics (e.g., Kagome <-> Bethe)
    pub transmute_perms: BTreeSet<BlissId>,
    
    /// Public read access
    pub public_read: bool,
    
    /// Fractal inheritance policy
    pub inheritance: InheritancePolicy,
    
    /// Audit trail signature
    pub acl_signature: Option<DilithiumSignature>,
    
    /// Last ACL modification timestamp
    pub modified_ns: u64,
}

impl ShardACL {
    /// Create default ACL for new shard (owner-only)
    pub fn new(owner: BlissId) -> Self {
        let now_ns = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
            
        Self {
            owner,
            read_perms: BTreeSet::new(),
            write_perms: BTreeSet::new(),
            admin_perms: BTreeSet::new(),
            transmute_perms: BTreeSet::new(),
            public_read: false,
            inheritance: InheritancePolicy::ParentOverride,
            acl_signature: None,
            modified_ns: now_ns,
        }
    }

    /// Sign ACL with quantum-safe Dilithium
    pub fn sign(mut self, keypair: &KyberKeypair) -> Result<Self, AclError> {
        let signature = crate::crypto::quantum::dilithium_sign(
            &bincode::serialize(&self)?.as_slice(), 
            &keypair.private_key
        )?;
        self.acl_signature = Some(signature);
        Ok(self)
    }

    /// Verify ACL signature integrity
    pub fn verify_signature(&self) -> Result<(), AclError> {
        if let Some(sig) = &self.acl_signature {
            sig.verify(&bincode::serialize(self)?.as_slice())?;
        }
        Ok(())
    }

    /// Check read permission for soul
    pub fn can_read(&self, soul: &BlissId) -> bool {
        self.owner == *soul || 
        self.read_perms.contains(soul) || 
        (self.public_read && !self.is_admin_restricted(soul))
    }

    /// Check write permission for soul
    pub fn can_write(&self, soul: &BlissId) -> bool {
        self.owner == *soul || self.write_perms.contains(soul)
    }

    /// Check admin permission for soul
    pub fn can_admin(&self, soul: &BlissId) -> bool {
        self.owner == *soul || self.admin_perms.contains(soul)
    }

    /// Check transmute permission (geometry change)
    pub fn can_transmute(&self, soul: &BlissId) -> bool {
        self.owner == *soul || 
        self.transmute_perms.contains(soul) || 
        self.admin_perms.contains(soul)
    }

    /// Grant read permission to soul
    pub fn grant_read(&mut self, soul: BlissId) -> bool {
        self.read_perms.insert(soul)
    }

    /// Grant write permission to soul
    pub fn grant_write(&mut self, soul: BlissId) -> bool {
        self.write_perms.insert(soul)
    }

    /// Grant admin permission to soul
    pub fn grant_admin(&mut self, soul: BlissId) -> bool {
        self.admin_perms.insert(soul)
    }

    /// Grant transmute permission to soul
    pub fn grant_transmute(&mut self, soul: BlissId) -> bool {
        self.transmute_perms.insert(soul)
    }

    /// Revoke specific permission
    pub fn revoke(&mut self, soul: &BlissId, perm_type: PermissionType) -> bool {
        match perm_type {
            PermissionType::Read => self.read_perms.remove(soul),
            PermissionType::Write => self.write_perms.remove(soul),
            PermissionType::Admin => self.admin_perms.remove(soul),
            PermissionType::Transmute => self.transmute_perms.remove(soul),
        }
    }

    /// Check if admin has restricted public read
    fn is_admin_restricted(&self, soul: &BlissId) -> bool {
        self.admin_perms.contains(soul)
    }
}

/// Permission types for granular control
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PermissionType {
    Read,
    Write,
    Admin,
    Transmute, // ✨ New: Ability to change Lattice Geometry
}

/// Fractal inheritance policies for child shards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InheritancePolicy {
    /// Child inherits parent's ACL
    InheritParent,
    /// Child has independent ACL
    Independent,
    /// Child restricts parent's permissions
    ParentOverride,
    /// Child expands parent's permissions
    ParentExtend,
}

/// Effective permissions considering fractal inheritance
pub struct EffectiveACL {
    pub shard_id: ShardId,
    pub effective_read: BTreeSet<BlissId>,
    pub effective_write: BTreeSet<BlissId>,
    pub effective_admin: BTreeSet<BlissId>,
    pub effective_transmute: BTreeSet<BlissId>,
    pub public_read: bool,
}

impl EffectiveACL {
    /// Compute effective ACL considering fractal lineage
    pub fn compute(
        shard_meta: &ShardMetadata,
        shard_acl: &ShardACL,
        lineage_acls: &[ShardACL],
    ) -> Self {
        let mut effective_read = shard_acl.read_perms.clone();
        let mut effective_write = shard_acl.write_perms.clone();
        let mut effective_admin = shard_acl.admin_perms.clone();
        let mut effective_transmute = shard_acl.transmute_perms.clone();
        let mut public_read = shard_acl.public_read;

        // Apply inheritance based on policy
        match shard_acl.inheritance {
            InheritancePolicy::InheritParent => {
                for parent_acl in lineage_acls {
                    effective_read.extend(&parent_acl.read_perms);
                    effective_write.extend(&parent_acl.write_perms);
                    effective_admin.extend(&parent_acl.admin_perms);
                    effective_transmute.extend(&parent_acl.transmute_perms);
                    public_read = public_read || parent_acl.public_read;
                }
            }
            InheritancePolicy::ParentOverride => {
                if let Some(parent_acl) = lineage_acls.first() {
                    effective_read = parent_acl.read_perms.clone();
                    effective_write = parent_acl.write_perms.clone();
                    effective_admin = parent_acl.admin_perms.clone();
                    effective_transmute = parent_acl.transmute_perms.clone();
                    public_read = parent_acl.public_read;
                }
            }
            InheritancePolicy::ParentExtend => {
                // Shard ACL extends parents (keep local + add parents logic if needed)
            }
            InheritancePolicy::Independent => {
                // No inheritance
            }
        }

        Self {
            shard_id: shard_meta.shard_id.clone(),
            effective_read,
            effective_write,
            effective_admin,
            effective_transmute,
            public_read,
        }
    }
}

/// ACL enforcement middleware for shard server
pub struct AclEnforcer {
    keypair: KyberKeypair,
}

impl AclEnforcer {
    pub fn new(keypair: KyberKeypair) -> Self {
        Self { keypair }
    }

    /// Enforce ACL before shard operation
    pub async fn enforce(
        &self,
        operation: OperationType,
        shard_meta: &ShardMetadata,
        shard_acl: &ShardACL,
        caller_soul: &SoulProof,
    ) -> Result<(), AclError> {
        shard_acl.verify_signature()?;
        
        let bliss_id = caller_soul.verify()?;
        let required_perm = operation.required_permission();
        
        match required_perm {
            PermissionType::Read => {
                if !shard_acl.can_read(&bliss_id) {
                    return Err(AclError::PermissionDenied(bliss_id.to_string()));
                }
            }
            PermissionType::Write => {
                if !shard_acl.can_write(&bliss_id) {
                    return Err(AclError::PermissionDenied(bliss_id.to_string()));
                }
            }
            PermissionType::Admin => {
                if !shard_acl.can_admin(&bliss_id) {
                    return Err(AclError::PermissionDenied(bliss_id.to_string()));
                }
            }
            PermissionType::Transmute => {
                if !shard_acl.can_transmute(&bliss_id) {
                    return Err(AclError::PermissionDenied(bliss_id.to_string()));
                }
            }
        }
        
        Ok(())
    }
}

/// Shard server operations with ACL mapping
#[derive(Debug, Clone, Copy)]
pub enum OperationType {
    Read,
    Write,
    Delete,
    Audit,
    Replicate,
    Admin,
    Transmute, // ✨ New: Changing Lattice Geometry
}

impl OperationType {
    fn required_permission(self) -> PermissionType {
        match self {
            OperationType::Read | OperationType::Audit => PermissionType::Read,
            OperationType::Write | OperationType::Delete | OperationType::Replicate => PermissionType::Write,
            OperationType::Admin => PermissionType::Admin,
            OperationType::Transmute => PermissionType::Transmute,
        }
    }
}

/// ACL-specific errors
#[derive(Debug, Error)]
pub enum AclError {
    #[error("Signature verification failed")]
    SignatureInvalid,
    #[error("Permission denied for soul: {0}")]
    PermissionDenied(String),
    #[error("Invalid soul proof")]
    InvalidSoulProof,
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
}

/// Default ACL policy for public shards
impl Default for ShardACL {
    fn default() -> Self {
        Self::new(BlissId::genesis())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acl_permissions() {
        let mut acl = ShardACL::new(BlissId::genesis());
        let owner = BlissId::genesis();
        let reader = BlissId::from("reader");

        assert!(acl.can_read(&owner));
        assert!(acl.can_write(&owner));
        assert!(acl.can_admin(&owner));
        assert!(acl.can_transmute(&owner));

        assert!(!acl.can_read(&reader));
        acl.grant_read(reader.clone());
        assert!(acl.can_read(&reader));
        assert!(!acl.can_write(&reader));
        assert!(!acl.can_transmute(&reader));
    }

    #[test]
    fn test_transmute_permission() {
        let mut acl = ShardACL::new(BlissId::genesis());
        let alchemist = BlissId::from("alchemist");
        
        assert!(!acl.can_transmute(&alchemist));
        acl.grant_transmute(alchemist.clone());
        assert!(acl.can_transmute(&alchemist));
    }

    #[test]
    fn test_effective_acl_inheritance() {
        let shard_meta = ShardMetadata::default();
        let mut child_acl = ShardACL::new(BlissId::genesis());
        child_acl.inheritance = InheritancePolicy::InheritParent;
        child_acl.grant_read(BlissId::from("parent_reader"));

        let parent_acl = ShardACL::new(BlissId::genesis());
        parent_acl.grant_read(BlissId::from("grandparent_reader"));

        let effective = EffectiveACL::compute(&shard_meta, &child_acl, &[parent_acl]);
        assert!(effective.effective_read.contains(&BlissId::from("parent_reader")));
    }
}