//! Governance module root for AuraFS
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

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
use std::sync::Arc;
/// Complete governance system struct
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