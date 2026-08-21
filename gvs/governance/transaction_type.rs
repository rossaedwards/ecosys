//! Governance transaction types for AuraFS consensus and workflow
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{Utc};

/// Transaction types defined for governance consensus and ledger anchoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceTransactionType {
    /// Cast a vote on a proposal
    VoteCast {
        vote_id: String,
        proposal_id: String,
        bliss_id: String,
        option: VoteOption,
        vote_weight: f64,
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
        tally: HashMap<VoteOption, f64>,
        timestamp: i64,
    },
}

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
/// Create a new VoteCast transaction
pub fn create_vote_cast(
    vote_id: String,
    proposal_id: String,
    bliss_id: String,
    option: VoteOption,
    vote_weight: f64,
    signature: Vec<u8>,
) -> GovernanceTransactionType {
    GovernanceTransactionType::VoteCast {
        vote_id,
        proposal_id,
        bliss_id,
        option,
        vote_weight,
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
    tally: HashMap<VoteOption, f64>,
) -> GovernanceTransactionType {
    GovernanceTransactionType::ProposalFinalized {
        proposal_id,
        result,
        tally,
        timestamp: Utc::now().timestamp(),
    }
}
/// Verify the quantum-safe signature of a VoteCast transaction
pub fn verify_vote_cast_signature(
    vote: &GovernanceTransactionType,
    public_key: &[u8],
) -> Result<bool, String> {
    use crate::crypto::verify_signature;

    if let GovernanceTransactionType::VoteCast {
        vote_id,
        proposal_id,
        bliss_id,
        option,
        vote_weight,
        signature,
        ..
    } = vote
    {
        let data = format!(
            "{}:{}:{}:{:?}:{}",
            vote_id, proposal_id, bliss_id, option, vote_weight
        );
        verify_signature(data.as_bytes(), signature, public_key)
            .map_err(|e| format!("Signature verification failed: {:?}", e))
    } else {
        Err("Not a VoteCast transaction".to_string())
    }
}
/// Verify the quantum-safe signature of a VoteCast transaction
pub fn verify_vote_cast_signature(
    vote: &GovernanceTransactionType,
    public_key: &[u8],
) -> Result<bool, String> {
    use crate::crypto::verify_signature;

    if let GovernanceTransactionType::VoteCast {
        vote_id,
        proposal_id,
        bliss_id,
        option,
        vote_weight,
        signature,
        ..
    } = vote
    {
        let data = format!(
            "{}:{}:{}:{:?}:{}",
            vote_id, proposal_id, bliss_id, option, vote_weight
        );
        verify_signature(data.as_bytes(), signature, public_key)
            .map_err(|e| format!("Signature verification failed: {:?}", e))
    } else {
        Err("Not a VoteCast transaction".to_string())
    }
}
/// Verify data integrity using stored hash
pub fn verify_hash(data: &[u8], expected_hash: &str) -> Result<bool> {
    use sha3::{Digest, Sha3_512};

    let mut hasher = Sha3_512::new();
    hasher.update(data);
    let computed_hash = format!("{:x}", hasher.finalize());
    Ok(computed_hash == expected_hash)
}