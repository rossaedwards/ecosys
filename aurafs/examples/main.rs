//! AuraFS Governance - Main Library Entry
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

pub mod models;
pub mod identity_verifier;
pub mod blissid_manager;
pub mod soulsync_engine;
pub mod voting_engine;
pub mod proposal_manager;
pub mod consensus_integration;
pub mod policy_enforcer;
pub mod audit_log;
pub mod api;

pub use models::*;
pub use identity_verifier::*;
pub use blissid_manager::*;
pub use soulsync_engine::*;
pub use voting_engine::*;
pub use proposal_manager::*;
pub use consensus_integration::*;
pub use policy_enforcer::*;
pub use audit_log::*;

use std::sync::Arc;

/// Complete governance system
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

    pub fn verify_system_integrity(&self) -> Result<bool, String> {
        self.audit_logger.verify_integrity()
    }
}