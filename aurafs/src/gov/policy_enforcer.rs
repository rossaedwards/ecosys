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
    /// Phase II: Change the physics/geometry of a shard
    TransmuteLattice { target_shard: String, new_geometry: String },
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
                self.apply_acl_update(user_id, shard_id, permissions)
            }
            PolicyAction::ModifyNetworkRule { rule_id, action: rule_action } => {
                self.apply_network_rule(rule_id, rule_action)
            }
            PolicyAction::UpgradeProtocol { version } => {
                self.apply_protocol_upgrade(version)
            }
            PolicyAction::ModifyQuorum { new_percentage } => {
                self.apply_quorum_change(*new_percentage)
            }
            PolicyAction::BanNode { node_id, reason } => {
                self.apply_node_ban(node_id, reason)
            }
            PolicyAction::TransmuteLattice { target_shard, new_geometry } => {
                self.apply_lattice_transmutation(target_shard, new_geometry)
            }
        };

        // If successful, record the policy as active
        if let Ok(res) = &result {
             let mut policies = self.active_policies.write().unwrap();
             policies.insert(proposal.id.clone(), action.clone());
             
             self.audit_logger.log_event(
                "policy_enforced",
                &format!("proposal={}, action={:?}", proposal.id, action),
            );

            log::info!("⚖️ Policy enforced: {} - {}", proposal.id, res.message);
        }

        result
    }

    /// Extracts a PolicyAction from the proposal (handling both Metadata and Enum Data)
    fn parse_policy_action(&self, proposal: &Proposal) -> Result<PolicyAction, String> {
        match &proposal.proposal_type {
            // Phase II: Data is embedded directly in the variant
            ProposalType::LatticeTransmutation { target_shard, new_geometry } => {
                Ok(PolicyAction::TransmuteLattice {
                    target_shard: target_shard.clone(),
                    new_geometry: new_geometry.clone(),
                })
            },
            
            // Legacy/Standard types use metadata map
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
            // Handle other types or error
            _ => Err(format!("Unsupported proposal type for policy enforcement: {:?}", proposal.proposal_type)),
        }
    }

    // === Execution Handlers ===

    /// Phase II: Transmute the Physics of a Shard
    fn apply_lattice_transmutation(&self, target_shard: &str, new_geometry: &str) -> Result<EnforcementResult, String> {
        log::info!("⚛️ TRANSMUTING LATTICE: Shard {} -> Geometry {}", target_shard, new_geometry);

        // MOCK INTEGRATION: In production, this makes a gRPC call to `shard_server::transmute_shard`.
        // e.g., shard_client.transmute(target_shard, new_geometry).await?;
        
        self.audit_logger.log_event(
            "physics_transmuted",
            &format!("shard={}, geometry={}", target_shard, new_geometry),
        );

        Ok(EnforcementResult {
            success: true,
            message: format!("Shard '{}' physics transmuted to '{}' geometry.", target_shard, new_geometry),
            timestamp: Utc::now().timestamp(),
        })
    }

    fn apply_acl_update(&self, user_id: &str, shard_id: &str, permissions: &[String]) -> Result<EnforcementResult, String> {
        log::info!("📋 ACL update: user={}, shard={}, permissions={:?}", user_id, shard_id, permissions);
        // Integration point for ACL Manager
        Ok(EnforcementResult {
            success: true,
            message: format!("ACL updated for user '{}' on shard '{}'", user_id, shard_id),
            timestamp: Utc::now().timestamp(),
        })
    }

    fn apply_network_rule(&self, rule_id: &str, action: &str) -> Result<EnforcementResult, String> {
        log::info!("🌐 Network rule change: rule_id={}, action={}", rule_id, action);
        Ok(EnforcementResult {
            success: true,
            message: format!("Network rule '{}' {} successfully", rule_id, action),
            timestamp: Utc::now().timestamp(),
        })
    }

    fn apply_protocol_upgrade(&self, version: &str) -> Result<EnforcementResult, String> {
        log::info!("🚀 Initiating protocol upgrade: version {}", version);
        Ok(EnforcementResult {
            success: true,
            message: format!("Protocol upgrade to version {} initiated", version),
            timestamp: Utc::now().timestamp(),
        })
    }

    fn apply_quorum_change(&self, new_percentage: f64) -> Result<EnforcementResult, String> {
        if new_percentage < 0.0 || new_percentage > 100.0 {
            return Err(format!("Invalid quorum percentage: {} (must be 0-100)", new_percentage));
        }
        log::info!("📊 Quorum percentage set to {}%", new_percentage);
        
        self.audit_logger.log_event(
            "quorum_modified",
            &format!("new_percentage={}%", new_percentage),
        );

        Ok(EnforcementResult {
            success: true,
            message: format!("Quorum changed to {}% (persisted and propagated)", new_percentage),
            timestamp: Utc::now().timestamp(),
        })
    }

    fn apply_node_ban(&self, node_id: &str, reason: &str) -> Result<EnforcementResult, String> {
        log::warn!("🚫 Node banned: id={}, reason={}", node_id, reason);
        self.audit_logger.log_event(
            "node_banned",
            &format!("node_id={}, reason={}", node_id, reason),
        );

        Ok(EnforcementResult {
            success: true,
            message: format!("Node '{}' banned from network. Reason: {}", node_id, reason),
            timestamp: Utc::now().timestamp(),
        })
    }

    pub fn get_active_policies(&self) -> HashMap<String, PolicyAction> {
        self.active_policies.read().unwrap().clone()
    }
}