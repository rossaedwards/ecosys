//! Core data models for AuraFS governance system
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Proposal status within the governance lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Draft,
    Active,
    Approved,
    Rejected,
    Cancelled,
    Executed,
}

/// Types of governance proposals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalType {
    ACLModification,
    NetworkUpgrade,
    ParameterChange,
    NodeBan,
    TreasuryAllocation,
    ProtocolUpgrade,
}

/// Governance proposal data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator_bliss_id: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub created_at: i64,  // Unix timestamp
    pub updated_at: i64,  // Unix timestamp
    pub voting_start: Option<i64>,
    pub voting_end: Option<i64>,
    pub metadata: HashMap<String, String>,
}

/// Soul coherence metrics used for vote weighting and reputation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulCoherence {
    pub bliss_id: String,
    pub hrv_coherence: f64,           // Heart rate variability score [0.0 - 1.0]
    pub participation_rate: f64,      // Governance participation rate [0.0 - 1.0]
    pub ethical_alignment: f64,       // Ethical alignment score [0.0 - 1.0]
    pub overall_coherence: f64,       // Aggregate coherence computed [0.0 - 1.0]
    pub last_updated: i64,            // Timestamp of last update
}

impl SoulCoherence {
    /// Compute the vote weight based on coherence metrics
    pub fn compute_vote_weight(&self) -> f64 {
        // Base vote weight of 1.0 plus up to 0.5 bonus from overall coherence
        1.0 + (0.5 * self.overall_coherence)
    }
}

/// Network node metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub node_id: String,
    pub address: String,
    pub role: String,              // e.g., "validator", "observer", or "leader"
    pub reputation_score: f64,
    pub last_seen: i64,            // Unix timestamp
    pub active: bool,
}

/// ACL permission entry model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLEntry {
    pub user_id: String,
    pub resource_id: String,
    pub permissions: Vec<String>, // e.g. ["read", "write", "admin"]
    pub granted_at: i64,           // Unix timestamp when granted
    pub expires_at: Option<i64>,   // Optional expiration timestamp
}

/// Governance statistics aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStats {
    pub total_proposals: usize,
    pub active_proposals: usize,
    pub total_votes_cast: usize,
    pub unique_voters: usize,
    pub average_participation_rate: f64,
    pub average_approval_rate: f64,
}