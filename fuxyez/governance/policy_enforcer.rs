//! Policy enforcement engine for AuraFS governance decisions
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use chrono::Utc;

use crate::models::{Proposal, ProposalType};
use crate::audit_log::AuditLogger;

/// Actions triggered by approved governance policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    UpdateACL { user_id: String, shard_id: String, permissions: Vec<String> },
    ModifyNetworkRule { rule_id: String, action: String },
    UpgradeProtocol { version: String },
    ModifyQuorum { new_percentage: f64 },
    BanNode { node_id: String, reason: String },
}

/// Outcome of running a policy enforcement
#[derive(Debug, Clone)]
pub struct EnforcementResult {
    pub success: bool,
    pub message: String,
    pub timestamp: i64,
}

/// Core policy enforcer maintaining active policies and audit logging
pub struct PolicyEnforcer {
    active_policies: Arc<RwLock<HashMap<String, PolicyAction>>>,
    audit_logger: Arc<AuditLogger>,
}

impl PolicyEnforcer {
    pub fn new(audit_logger: Arc<AuditLogger>) -> Self {
        Self {
            active_policies: Arc::new(RwLock::new(HashMap::new())),
            audit_logger,
        }
    }

    /// Enforce the directives of an approved governance proposal
    pub fn enforce_proposal(&self, proposal: &Proposal) -> Result<EnforcementResult, String> {
        let action = self.parse_policy_action(proposal)?;

        let result = match &action {
            PolicyAction::UpdateACL { user_id, shard_id, permissions } => {
                self.apply_acl_update(user_id, shard_id, permissions)?
            }
            PolicyAction::ModifyNetworkRule { rule_id, action: rule_action } => {
                self.apply_network_rule(rule_id, rule_action)?
            }
            PolicyAction::UpgradeProtocol { version } => {
                self.apply_protocol_upgrade(version)?
            }
            PolicyAction::ModifyQuorum { new_percentage } => {
                self.apply_quorum_change(*new_percentage)?
            }
            PolicyAction::BanNode { node_id, reason } => {
                self.apply_node_ban(node_id, reason)?
            }
        };

        {
            let mut policies = self.active_policies.write().unwrap();
            policies.insert(proposal.id.clone(), action.clone());
        }

        self.audit_logger.log_event(
            "policy_enforced",
            &format!("proposal={}, action={:?}", proposal.id, action),
        );

        log::info!("⚖️  Policy enforced: {} - {}", proposal.id, result.message);
        Ok(result)
    }

    /// Extracts a PolicyAction from the proposal metadata
    fn parse_policy_action(&self, proposal: &Proposal) -> Result<PolicyAction, String> {
        match proposal.proposal_type {
            ProposalType::ACLModification => {
                let user_id = proposal.metadata.get("user_id")
                    .ok_or("Missing user_id in proposal metadata")?;
                let shard_id = proposal.metadata.get("shard_id")
                    .ok_or("Missing shard_id in proposal metadata")?;
                let permissions: Vec<String> = proposal.metadata.get("permissions")
                    .ok_or("Missing permissions in proposal metadata")?
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                Ok(PolicyAction::UpdateACL {
                    user_id: user_id.clone(),
                    shard_id: shard_id.clone(),
                    permissions,
                })
            }
            ProposalType::NetworkUpgrade => {
                let version = proposal.metadata.get("version")
                    .ok_or("Missing version in proposal metadata")?;
                Ok(PolicyAction::UpgradeProtocol { version: version.clone() })
            }
            ProposalType::ParameterChange => {
                let new_quorum = proposal.metadata.get("new_quorum")
                    .ok_or("Missing new_quorum in proposal metadata")?;
                let percentage: f64 = new_quorum.parse()
                    .map_err(|_| "Invalid quorum percentage format".to_string())?;
                Ok(PolicyAction::ModifyQuorum { new_percentage: percentage })
            }
            ProposalType::NodeBan => {
                let node_id = proposal.metadata.get("node_id")
                    .ok_or("Missing node_id in proposal metadata")?;
                let reason = proposal.metadata.get("reason")
                    .ok_or("Missing reason in proposal metadata")?;
                Ok(PolicyAction::BanNode { node_id: node_id.clone(), reason: reason.clone() })
            }
            _ => Err(format!("Unsupported proposal type for policy enforcement: {:?}", proposal.proposal_type)),
        }
    }

    /// Placeholder: Apply ACL updates via ACL manager integration
    fn apply_acl_update(
        &self,
        user_id: &str,
        shard_id: &str,
        permissions: &[String],
    ) -> Result<EnforcementResult, String> {
        log::info!("📋 ACL update: user={}, shard={}, permissions={:?}", user_id, shard_id, permissions);

        // TODO: Integrate actual ACL manager call

        Ok(EnforcementResult {
            success: true,
            message: format!("ACL updated for user '{}' on shard '{}'", user_id, shard_id),
            timestamp: Utc::now().timestamp(),
        })
    }

    /// Placeholder: Apply network rule modifications
    fn apply_network_rule(&self, rule_id: &str, action: &str) -> Result<EnforcementResult, String> {
        log::info!("🌐 Network rule change: rule_id={}, action={}", rule_id, action);

        // TODO: Integrate actual network rules module

        Ok(EnforcementResult {
            success: true,
            message: format!("Network rule '{}' {}", rule_id, action),
            timestamp: Utc::now().timestamp(),
        })
    }

    /// Placeholder: Apply protocol version upgrade logic
    fn apply_protocol_upgrade(&self, version: &str) -> Result<EnforcementResult, String> {
        log::info!("🚀 Initiating protocol upgrade: version {}", version);

        // TODO: Trigger actual upgrade procedures

        Ok(EnforcementResult {
            success: true,
            message: format!("Protocol upgraded to version {}", version),
            timestamp: Utc::now().timestamp(),
        })
    }

    /// Placeholder: Quorum percentage update
    fn apply_quorum_change(&self, new_percentage: f64) -> Result<EnforcementResult, String> {
        log::info!("📊 Quorum percentage set to {}%", new_percentage);

        // TODO: Persist new quorum and trigger related transitions

        Ok(EnforcementResult {
            success: true,
            message: format!("Quorum changed to {}%", new_percentage),
            timestamp: Utc::now().timestamp(),
        })
    }

    /// Placeholder: Ban node handling
    fn apply_node_ban(&self, node_id: &str, reason: &str) -> Result<EnforcementResult, String> {
        log::warn!("🚫 Node banned: id={}, reason={}", node_id, reason);

        // TODO: Enforce ban in network layer

        Ok(EnforcementResult {
            success: true,
            message: format!("Node '{}' banned for reason: {}", node_id, reason),
            timestamp: Utc::now().timestamp(),
        })
    }

    /// Retrieve the map of active policies
    pub fn get_active_policies(&self) -> HashMap<String, PolicyAction> {
        self.active_policies.read().unwrap().clone()
    }
}
    pub fn get_entries(&self) -> Vec<AuditEntry> {
        let entries = self.entries.lock().unwrap();
        entries.clone()
    }

/// Verify data integrity using stored hash
fn verify_hash(data: &[u8], expected_hash: &str) -> Result<bool> {
    use sha3::{Digest, Sha3_512};

    let mut hasher = Sha3_512::new();
    hasher.update(data);
    let computed_hash = format!("{:x}", hasher.finalize());
    Ok(computed_hash == expected_hash)
}
File: afs/src/governance/policy_enforcer.rs
//! Policy enforcement engine for AuraFS governance decisions
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//! Core policy enforcer maintaining active policies and audit logging
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//! Shard healing engine for AuraFS autoheal daemon
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//! Core shard healing logic with retries and verification
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//! Data integrity verification utility
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//! Core identity verifier for AuraFS governance
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//! Identity verification using quantum-safe signatures and zero-knowledge proofs
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use sha3::{Digest, Sha3_256};
use pqcrypto_dilithium::dilithium5;
use log::{info, warn, error};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::models::{Proposal, ProposalType};
use crate::audit_log::AuditLogger;
/// Actions triggered by approved governance policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    UpdateACL { user_id: String, shard_id: String, permissions: Vec<String> },
    ModifyNetworkRule { rule_id: String, action: String },
    UpgradeProtocol { version: String },
    ModifyQuorum { new_percentage: f64 },
    BanNode { node_id: String, reason: String },
}
/// Outcome of running a policy enforcement
#[derive(Debug, Clone)]
pub struct EnforcementResult {
    pub success: bool,
    pub message: String,
    pub timestamp: i64,
}
/// Core policy enforcer maintaining active policies and audit logging
pub struct PolicyEnforcer {
    active_policies: Arc<RwLock<HashMap<String, PolicyAction>>>,
    audit_logger: Arc<AuditLogger>,
}
impl PolicyEnforcer {
    pub fn new(audit_logger: Arc<AuditLogger>) -> Self {
        Self {
            active_policies: Arc::new(RwLock::new(HashMap::new())),
            audit_logger,
        }
    }
    /// Enforce the directives of an approved governance proposal
    pub fn enforce_proposal(&self, proposal: &Proposal) -> Result<Enforcement
Result, String> {
        let action = self.parse_policy_action(proposal)?;
        let result = match &action {
            PolicyAction::UpdateACL { user_id, shard_id, permissions } => {
                self.apply_acl_update(user_id, shard_id, permissions)?
            }
            PolicyAction::ModifyNetworkRule { rule_id, action: rule_action } => {
                self.apply_network_rule(rule_id, rule_action)?
            }
            PolicyAction::UpgradeProtocol { version } => {
                self.apply_protocol_upgrade(version)?
            }
            PolicyAction::ModifyQuorum { new_percentage } => {
                self.apply_quorum_change(*new_percentage)?
            }
            PolicyAction::BanNode { node_id, reason } => {
                self.apply_node_ban(node_id, reason)?
            }
        };
        {
            let mut policies = self.active_policies.write().unwrap();
            policies.insert(proposal.id.clone(), action.clone());
        }
        self.audit_logger.log_event(
            "policy_enforced",
            &format!("proposal={}, action={:?}", proposal.id, action),
        );
        log::info!("⚖️  Policy enforced: {} - {}", proposal.id, result.message);
        Ok(result)
    }
    /// Extracts a PolicyAction from the proposal metadata
    fn parse_policy_action(&self, proposal: &Proposal) -> Result<PolicyAction,
    String> {
            match proposal.proposal_type {
                ProposalType::ACLModification => {
                    let user_id = proposal.metadata.get("user_id")
                        .ok_or("Missing user_id in proposal metadata")?;
                    let shard_id = proposal.metadata.get("shard_id")
                        .ok_or("Missing shard_id in proposal metadata")?;
                    let permissions: Vec<String> = proposal.metadata.get("permissions")
                        .ok_or("Missing permissions in proposal metadata")?
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    Ok(PolicyAction::UpdateACL {
                        user_id: user_id.clone(),
                        shard_id: shard_id.clone(),
                        permissions,
                    })
                }
                ProposalType::NetworkUpgrade => {
                    let version = proposal.metadata.get("version")
                        .ok_or("Missing version in proposal metadata")?;
                    Ok(PolicyAction::UpgradeProtocol { version: version.clone() })
                }
                ProposalType::ParameterChange => {
                    let new_quorum = proposal.metadata.get("new_quorum")
                        .ok_or("Missing new_quorum in proposal metadata")?;
                    let percentage: f64 = new_quorum.parse()
                        .map_err(|_| "Invalid quorum percentage format".to_string())?;
                    Ok(PolicyAction::ModifyQuorum { new_percentage: percentage })
                }
                ProposalType::NodeBan => {
                    let node_id = proposal.metadata.get("node_id")
                        .ok_or("Missing node_id in proposal metadata")?;
                    let reason = proposal.metadata.get("reason")
                        .ok_or("Missing reason in proposal metadata")?;
                    Ok(PolicyAction::BanNode { node_id: node_id.clone(), reason: reason.clone() })
                }
                _ => Err(format!("Unsupported proposal type for policy enforcement: {:?}", proposal.proposal_type)),
            }
        }
    /// Placeholder: Apply ACL updates via ACL manager integration
    fn apply_acl_update(
        &self,
        user_id: &str,
        shard_id: &str,
        permissions: &[String],
    ) -> Result<EnforcementResult, String> {
        log::info!("📋 ACL update: user={}, shard={}, permissions={:?}", user_id, shard_id, permissions);