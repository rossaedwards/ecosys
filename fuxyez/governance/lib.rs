//! AuraFS Governance Module - Root
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! Complete governance system with BlissID identity, soul-weighted voting,
//! distributed consensus, and policy enforcement.

pub mod models;
pub mod identity_verifier;
pub mod blissid_manager;
pub mod soulsync_engine;
pub mod voting_engine;
pub mod proposal_manager;
pub mod consensus_integration;
pub mod policy_enforcer;
pub mod audit_log;

// Re-export key types for convenient external use
pub use models::*;
pub use identity_verifier::{IdentityVerifier, BlissID, SoulProof};
pub use blissid_manager::BlissIDManager;
pub use soulsync_engine::SoulSyncEngine;
pub use voting_engine::{VotingEngine, Vote, VoteOption, VoteTally};
pub use proposal_manager::{ProposalManager, ProposalConfig};
pub use consensus_integration::{ConsensusIntegration, GovernanceTransaction, LedgerClient};
pub use policy_enforcer::{PolicyEnforcer, PolicyAction};
pub use audit_log::{AuditLogger, AuditEntry};

use std::sync::Arc;

/// Complete governance system assembler
pub struct GovernanceSystem {
    pub identity_verifier: Arc<IdentityVerifier>,
    pub blissid_manager: Arc<BlissIDManager>,
    pub soulsync_engine: Arc<SoulSyncEngine>,
    pub audit_logger: Arc<AuditLogger>,
    pub consensus: Arc<ConsensusIntegration>,
    pub voting_engine: Arc<VotingEngine>,
    pub proposal_manager: Arc<ProposalManager>,
    pub policy_enforcer: Arc<PolicyEnforcer>,
}

impl GovernanceSystem {
    /// Initialize full governance stack with optional proposal config
    pub fn new(
        node_id: String,
        validators: Vec<String>,
        ledger_client: Arc<dyn LedgerClient + Send + Sync>,
        proposal_config: Option<ProposalConfig>,
    ) -> Self {
        let audit_logger = Arc::new(AuditLogger::new());
        let identity_verifier = Arc::new(IdentityVerifier::new());
        let blissid_manager = Arc::new(BlissIDManager::new(audit_logger.clone()));
        let soulsync_engine = Arc::new(SoulSyncEngine::new(audit_logger.clone()));

        let consensus = Arc::new(ConsensusIntegration::new(
            node_id,
            validators,
            ledger_client,
        ));

        let voting_engine = Arc::new(VotingEngine::new(
            identity_verifier.clone(),
            consensus.clone(),
        ));

        let proposal_manager = Arc::new(ProposalManager::new(
            voting_engine.clone(),
            audit_logger.clone(),
            proposal_config,
        ));

        let policy_enforcer = Arc::new(PolicyEnforcer::new(audit_logger.clone()));

        Self {
            identity_verifier,
            blissid_manager,
            soulsync_engine,
            audit_logger,
            consensus,
            voting_engine,
            proposal_manager,
            policy_enforcer,
        }
    }

    /// Verify full system audit log integrity
    pub fn verify_system_integrity(&self) -> Result<bool, String> {
        self.audit_logger.verify_integrity()
    }

    /// Export entire governance system state as JSON string
    pub fn export_state(&self) -> Result<String, String> {
        self.audit_logger.export_json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consensus_integration::MockLedgerClient;

    #[test]
    fn test_governance_system_initialization() {
        let ledger = Arc::new(MockLedgerClient::new());
        let system = GovernanceSystem::new(
            "test_node".to_string(),
            vec!["node1".to_string(), "node2".to_string()],
            ledger,
            None,
        );

        assert!(system.verify_system_integrity().is_ok());
        assert_eq!(system.blissid_manager.get_total_count(), 0);
    }
}