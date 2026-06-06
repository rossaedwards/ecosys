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
    Transmuting, // Phase II: Physics change in progress
}

/// Types of governance proposals
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalType {
    ACLModification,
    NetworkUpgrade,
    ParameterChange,
    NodeBan,
    TreasuryAllocation,
    ProtocolUpgrade,
    /// Phase II: Change the geometry/physics of a specific Shard or Region
    LatticeTransmutation {
        target_shard: String,
        new_geometry: String, // "Kagome", "Bethe", "Triangular"
    },
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

/// SoulShot Metadata - Used only for uniqueness verification, NOT weighting.
/// strictly 1 Soul = 1 Vote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulShotVerification {
    pub bliss_id: String,
    pub is_alive: bool,       // Liveness check from SoulSync
    pub is_unique: bool,      // Anti-Sybil check
    pub verified_at: i64,
}

/// Network node metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub node_id: String,
    pub address: String,
    pub role: String,              // e.g., "validator", "observer", or "leader"
    pub reputation_score: f64,     // Still useful for Trust, but NOT for Voting Power
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