//! Governance module root for AuraFS
//! The Central Nervous System: Wires Identity, Physics, and Consensus together.
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

// --- Module Declarations ---
pub mod api;
pub mod audit_log;
pub mod blissid_manager;
pub mod consensus_integration;
pub mod identity_verifier;
pub mod models;
pub mod policy_enforcer;
pub mod proposal_manager;
pub mod soulsync_engine;
pub mod transaction_type;
pub mod voting_engine;

// --- Re-exports for easy access ---
pub use api::*;
pub use audit_log::*;
pub use blissid_manager::*;
pub use consensus_integration::*;
pub use identity_verifier::*;
pub use models::*;
pub use policy_enforcer::*;
pub use proposal_manager::*;
pub use soulsync_engine::*;
pub use transaction_type::*;
pub use voting_engine::*;

use std::sync::{Arc, RwLock};
use log::info;
use crate::physics::{DecoherenceRecovery, PhysicsViolationError};

/// The Central Governance System
/// This struct holds references to all sub-systems and orchestrates the lifecycle.
pub struct GovernanceSystem {
    pub node_id: String,
    
    // Identity Layer (The Soul)
    pub identity_verifier: Arc<IdentityVerifier>,
    pub blissid_manager: Arc<BlissIDManager>,
    pub soulsync_engine: Arc<SoulSyncEngine>,
    
    // Verification Layer (The Truth)
    pub audit_logger: Arc<AuditLogger>,
    pub consensus: Arc<ConsensusIntegration>,

    // S.A.G.E.S. Sentinels
    pub sages: Arc<RwLock<SagesLoop>>,
    
    // Execution Layer (The Hand)
    pub voting_engine: Arc<VotingEngine>,
    pub proposal_manager: Arc<ProposalManager>,
    pub policy_enforcer: Arc<PolicyEnforcer>,
}

impl GovernanceSystem {
    /// Initialize the complete Governance System with all subsystems wired.
    /// This is called by the `server.rs` binary.
    pub fn new(
        node_id: String,
        validators: Vec<String>,
        ledger_client: Arc<MockLedgerClient>, // In prod, use a trait like Arc<dyn LedgerClient>
        proposal_config: Option<ProposalConfig>,
    ) -> Self {
        info!("⚙️ Initializing Governance Subsystems for Node: {}", node_id);

        // 1. Foundation: Audit Log (Immutable History)
        let audit_logger = Arc::new(AuditLogger::new());
        
        // 2. Identity: Verifier & Soul Manager (1 Soul = 1 Vote)
        // Note: In a real deployment, IdentityVerifier might load keys from disk/KMS
        let identity_verifier = Arc::new(IdentityVerifier::new());
        let blissid_manager = Arc::new(BlissIDManager::new(audit_logger.clone()));

        // 3. Consensus: The Network Layer
        let consensus = Arc::new(ConsensusIntegration::new(
            node_id.clone(),
            validators,
            ledger_client,
        ));

        // 4. S.A.G.E.S. Sentinels
        let sages = Arc::new(RwLock::new(SagesLoop::new()));

        // SoulSync uses the audit log to track revocation events + S.A.G.E.S. quorum
        let soulsync_engine = Arc::new(SoulSyncEngine::new(audit_logger.clone(), sages.clone()));
        
        // 5. Voting: The Decision Engine (Egalitarian Mode)
        let voting_engine = Arc::new(VotingEngine::new(
            identity_verifier.clone(),
            consensus.clone(),
            soulsync_engine.clone(),
        ));

        // 6. Execution: The Physics Enforcer
        let policy_enforcer = Arc::new(PolicyEnforcer::new(audit_logger.clone()));
        
        // 7. Management: The Lifecycle Orchestrator
        let proposal_manager = Arc::new(ProposalManager::new(
            voting_engine.clone(),
            policy_enforcer.clone(),
            audit_logger.clone(),
            proposal_config,
        ));

        info!("⚡ Governance System Online. Ready for Lattice Transmutation.");

        Self {
            node_id,
            identity_verifier,
            blissid_manager,
            soulsync_engine,
            audit_logger,
            consensus,
            sages,
            voting_engine,
            proposal_manager,
            policy_enforcer,
        }
    }

    /// Verify the integrity of the entire system chain (Audit Log Check)
    pub fn verify_system_integrity(&self) -> Result<bool, String> {
        self.audit_logger.verify_integrity()
    }

    /// [Theorem 3.1: Universality]
    /// Dispatch physics violations to the S.A.G.E.S. sentinel loop.
    pub fn handle_physics_violation<T: DecoherenceRecovery>(
        &self,
        error: &PhysicsViolationError,
        subject: &T,
    ) -> anyhow::Result<()> {
        let mut sages = self.sages.write().unwrap();
        sages.handle_physics_violation(error, subject)?;
        Ok(())
    }
}