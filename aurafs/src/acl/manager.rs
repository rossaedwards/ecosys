//! ═══════════════════════════════════════════════════════════════════
//! 🔐 AuraFS ACL (Access Control List) Manager
//! ✨ f0rg3d with Ineffable l0v3 by Ross Edwards & Aurphyx LLC 💎
//!
//! Provides fine-grained permission management with role-based access control,
//! resource-level permissions, and integration with the FUSE filesystem layer.
//! Extended with quantum-safe signature and zero-knowledge proof verification.
//!
//! ## Enterprise Features
//! - Thread-safe configuration management
//! - JSON-based persistence
//! - Default roles (admin, user, guest)
//! - Cryptographic proof verification
//! ═══════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use tracing::{debug, error, info, warn};

use crate::crypto::{verify_signature, verify_zk_proof};

#[derive(Debug, Error)]
pub enum AclError {
    #[error("ACL file not found: {0}")]
    FileNotFound(String),
    
    #[error("Failed to parse ACL data: {0}")]
    ParseError(String),
    
    #[error("Permission denied for user {user} on resource {resource}")]
    PermissionDenied { user: String, resource: String },
    
    #[error("Role not found: {0}")]
    RoleNotFound(String),
    
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AclError>;

/// ACL Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub permissions: HashSet<String>,
}

/// User with role assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub role: String,
    pub public_key: Option<String>,
    pub allowed_shards: HashSet<String>,
}

/// Resource ACL entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAcl {
    pub resource: String,
    pub allowed_users: HashSet<String>,
    pub allowed_groups: HashSet<String>,
}

/// Complete ACL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclConfig {
    pub roles: HashMap<String, Role>,
    pub users: HashMap<String, User>,
    pub resources: HashMap<String, ResourceAcl>,
}

/// ACL Manager - thread-safe permission management with quantum ZK extensions
pub struct AclManager {
    config: Arc<RwLock<AclConfig>>,
    config_path: String,
}

impl AclManager {
    /// Create new ACL manager from configuration file
    pub fn new(config_path: &str) -> Result<Self> {
        let config = Self::load_config(config_path)?;
        
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path: config_path.to_string(),
        })
    }
    
    /// Load ACL configuration from JSON file
    fn load_config(path: &str) -> Result<AclConfig> {
        if !Path::new(path).exists() {
            warn!("ACL config not found, creating default: {}", path);
            return Ok(Self::default_config());
        }
        
        let data = fs::read_to_string(path)?;
        let config: AclConfig = serde_json::from_str(&data)
            .map_err(|e| AclError::ParseError(e.to_string()))?;
        
        info!("Loaded ACL config with {} roles, {} users", 
              config.roles.len(), config.users.len());
        
        Ok(config)
    }
    
    /// Save current configuration to disk
    pub fn save_config(&self) -> Result<()> {
        let config = self.config.read().unwrap();
        let data = serde_json::to_string_pretty(&*config)
            .map_err(|e| AclError::ParseError(e.to_string()))?;
        
        fs::write(&self.config_path, data)?;
        info!("Saved ACL config to {}", self.config_path);
        
        Ok(())
    }
    
    /// Check if user has permission on resource traditionally
    pub fn check_permission(&self, user: &str, resource: &str, permission: &str) -> bool {
        let config = self.config.read().unwrap();
        
        let user_obj = match config.users.get(user) {
            Some(u) => u,
            None => {
                debug!("User not found: {}", user);
                return false;
            }
        };
        
        let role = match config.roles.get(&user_obj.role) {
            Some(r) => r,
            None => {
                warn!("Role not found for user {}: {}", user, user_obj.role);
                return false;
            }
        };
        
        if !role.permissions.contains(permission) {
            debug!("Role {} lacks permission: {}", role.name, permission);
            return false;
        }
        
        if let Some(resource_acl) = config.resources.get(resource) {
            if !resource_acl.allowed_users.contains(user) &&
               !resource_acl.allowed_users.contains("*") {
                debug!("User {} not in resource ACL for {}", user, resource);
                return false;
            }
        }
        
        debug!("Permission granted: user={}, resource={}, permission={}", 
               user, resource, permission);
        true
    }

    /// Check access supporting classical signature or zero-knowledge proof
    pub fn check_access_with_proofs(
        &self,
        user: &str,
        signature: Option<&[u8]>,
        zk_proof: Option<&[u8]>,
        message: &[u8],
    ) -> Result<bool> {
        let config = self.config.read().unwrap();

        let user_obj = config.users.get(user)
            .ok_or_else(|| AclError::UserNotFound(user.to_string()))?;

        match (signature, zk_proof) {
            (Some(sig), None) => {
                if verify_signature(message, sig)? {
                    Ok(true)
                } else {
                    Err(AclError::PermissionDenied { user: user.to_string(), resource: "signature invalid".into() })
                }
            }
            (None, Some(proof)) => {
                if verify_zk_proof(proof, message)? {
                    Ok(true)
                } else {
                    Err(AclError::PermissionDenied { user: user.to_string(), resource: "zk proof invalid".into() })
                }
            }
            (Some(_), Some(_)) => Err(AclError::PermissionDenied { user: user.to_string(), resource: "Both signature and zk proof provided".into() }),
            (None, None) => Err(AclError::PermissionDenied { user: user.to_string(), resource: "No proof or signature provided".into() }),
        }
    }

    /// (rest of your existing methods unchanged)
    // grant_permission, revoke_permission, add_user, add_role, get_user, list_users, list_roles

    fn default_config() -> AclConfig {
        let mut roles = HashMap::new();
        let mut users = HashMap::new();
        
        roles.insert("admin".to_string(), Role {
            name: "admin".to_string(),
            description: "Full system access".to_string(),
            permissions: vec!["read", "write", "delete", "admin"]
                .into_iter()
                .map(String::from)
                .collect(),
        });
        
        roles.insert("user".to_string(), Role {
            name: "user".to_string(),
            description: "Standard user access".to_string(),
            permissions: vec!["read", "write"]
                .into_iter()
                .map(String::from)
                .collect(),
        });
        
        roles.insert("guest".to_string(), Role {
            name: "guest".to_string(),
            description: "Read-only access".to_string(),
            permissions: vec!["read"]
                .into_iter()
                .map(String::from)
                .collect(),
        });
        
        users.insert("admin".to_string(), User {
            id: "admin".to_string(),
            role: "admin".to_string(),
            public_key: None,
            allowed_shards: HashSet::new(),
        });
        
        AclConfig {
            roles,
            users,
            resources: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AclManager::default_config();
        assert_eq!(config.roles.len(), 3);
        assert_eq!(config.users.len(), 1);
    }
    
    #[test]
    fn test_permission_check() {
        let manager = AclManager::new("test_acl.json").unwrap();
        
        // Admin should have all permissions
        assert!(manager.check_permission("admin", "/test", "read"));
        assert!(manager.check_permission("admin", "/test", "write"));
    }
}