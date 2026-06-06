//! Governance transaction types for AuraFS consensus and workflow
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::Utc;

/// Options for voting on proposals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum VoteOption {
    Yes,
    No,
    Abstain,
}

/// Result of a proposal after voting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalResult {
    Approved,
    Rejected,
    Executed,
}

/// Transaction types defined for governance consensus and ledger anchoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceTransactionType {
    /// Cast a vote on a proposal
    VoteCast {
        vote_id: String,
        proposal_id: String,
        bliss_id: String,
        option: VoteOption,
        vote_weight: f64,    // Kept as f64 for legacy compatibility, but strictly 1.0 in Phase II
        signature: Vec<u8>,  // Quantum-safe signature bytes
        timestamp: i64,
    },

    /// Proposal creation event
    ProposalCreated {
        proposal_id: String,
        title: String,
        creator_bliss_id: String,
        timestamp: i64,
    },

    /// Proposal finalized event, with result and vote tally
    ProposalFinalized {
        proposal_id: String,
        result: ProposalResult,
        tally: HashMap<VoteOption, u64>, // UPDATED: Phase II uses Integers (Soul Counts), not floats
        timestamp: i64,
    },
}

impl GovernanceTransactionType {
    /// Utility to get the transaction timestamp
    pub fn timestamp(&self) -> i64 {
        match self {
            GovernanceTransactionType::VoteCast { timestamp, .. } => *timestamp,
            GovernanceTransactionType::ProposalCreated { timestamp, .. } => *timestamp,
            GovernanceTransactionType::ProposalFinalized { timestamp, .. } => *timestamp,
        }
    }
}

// === Factory Methods ===

/// Create a new VoteCast transaction
pub fn create_vote_cast(
    vote_id: String,
    proposal_id: String,
    bliss_id: String,
    option: VoteOption,
    signature: Vec<u8>,
) -> GovernanceTransactionType {
    GovernanceTransactionType::VoteCast {
        vote_id,
        proposal_id,
        bliss_id,
        option,
        vote_weight: 1.0, // STRICT Phase II Enforcement: 1 Soul = 1 Vote
        signature,
        timestamp: Utc::now().timestamp(),
    }
}

/// Create a new ProposalCreated transaction
pub fn create_proposal_created(
    proposal_id: String,
    title: String,
    creator_bliss_id: String,
) -> GovernanceTransactionType {
    GovernanceTransactionType::ProposalCreated {
        proposal_id,
        title,
        creator_bliss_id,
        timestamp: Utc::now().timestamp(),
    }
}

/// Create a new ProposalFinalized transaction
pub fn create_proposal_finalized(
    proposal_id: String,
    result: ProposalResult,
    tally: HashMap<VoteOption, u64>,
) -> GovernanceTransactionType {
    GovernanceTransactionType::ProposalFinalized {
        proposal_id,
        result,
        tally,
        timestamp: Utc::now().timestamp(),
    }
}

/// Verify data integrity using stored hash
pub fn verify_hash(data: &[u8], expected_hash: &str) -> bool {
    use sha3::{Digest, Sha3_256}; // Standardized on Sha3_256

    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let computed_hash = hex::encode(hasher.finalize());
    computed_hash == expected_hash
}