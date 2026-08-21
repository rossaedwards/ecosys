//! Voting engine for AuraFS governance with quantum-safe signatures and tallying
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use chrono::Utc;
use anyhow::{Result, anyhow};

use crate::identity_verifier::IdentityVerifier;
use crate::consensus_integration::{ConsensusIntegration, GovernanceTransaction};
use crate::transaction_type::{VoteOption, GovernanceTransactionType};

/// Vote representation stored within the voting engine
#[derive(Debug, Clone)]
pub struct Vote {
    pub proposal_id: String,
    pub bliss_id: String,
    pub option: VoteOption,
    pub vote_weight: f64,
    pub timestamp: i64,
}

/// Aggregated vote tally
#[derive(Debug, Clone, Default)]
pub struct VoteTally {
    pub yes: f64,
    pub no: f64,
    pub abstain: f64,
    pub total_weight: f64,
    pub total_souls: usize,
}

/// Main voting engine struct
pub struct VotingEngine {
    identity_verifier: Arc<IdentityVerifier>,
    consensus: Arc<ConsensusIntegration>,
    votes: Arc<RwLock<HashMap<String, HashMap<String, Vote>>>>, // proposal_id -> bliss_id -> Vote
}

impl VotingEngine {
    /// Instantiate new VotingEngine with dependencies
    pub fn new(identity_verifier: Arc<IdentityVerifier>, consensus: Arc<ConsensusIntegration>) -> Self {
        Self {
            identity_verifier,
            consensus,
            votes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Cast a vote with quantum-safe signature verification
    pub async fn cast_vote(
        &self,
        proposal_id: String,
        bliss_id: String,
        option: VoteOption,
        signature: Vec<u8>,
        soul_proof: crate::identity_verifier::SoulProof,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Vote> {
        // Verify BlissID active
        if !self.identity_verifier.verify_bliss_id(&bliss_id) {
            return Err(anyhow!("Invalid or inactive BlissID"));
        }

        // Build message to verify signature: "<proposal_id>:<option>"
        let message = format!("{}:{:?}", proposal_id, option);
        if !self.identity_verifier.verify_quantum_signature(&bliss_id, message.as_bytes(), &signature)? {
            return Err(anyhow!("Invalid quantum signature"));
        }

        // Verify soul proof (zero-knowledge uniqueness proof)
        if !self.identity_verifier.verify_soul_proof(&bliss_id, &soul_proof) {
            return Err(anyhow!("Invalid soul proof"));
        }

        // Compute vote weight based on soul coherence
        let coherence_engine = Arc::clone(&self.consensus); // To get Soulsync or similar engine (requires wiring)
                                                        // For now, fallback weight 1.0
        let vote_weight = 1.0; // TODO: query SoulSync engine for weight

        let vote = Vote {
            proposal_id: proposal_id.clone(),
            bliss_id: bliss_id.clone(),
            option,
            vote_weight,
            timestamp: Utc::now().timestamp(),
        };

        {
            let mut all_votes = self.votes.write().unwrap();
            let proposal_votes = all_votes.entry(proposal_id.clone()).or_insert_with(HashMap::new);
            proposal_votes.insert(bliss_id.clone(), vote.clone());
        }

        // Submit vote as GovernanceTransaction to consensus layer
        let tx = GovernanceTransaction::VoteCast {
            vote_id: uuid::Uuid::new_v4().to_string(),
            proposal_id,
            bliss_id,
            option: format!("{:?}", vote.option),
            vote_weight,
            signature,
            timestamp: vote.timestamp,
        };

        self.consensus.submit_transaction(tx).await.map_err(|e| anyhow!(e))?;

        Ok(vote)
    }

    /// Tally votes for a proposal aggregating weights, counts, and totals
    pub fn tally_votes(&self, proposal_id: &str) -> VoteTally {
        let all_votes = self.votes.read().unwrap();
        let mut tally = VoteTally::default();

        if let Some(proposal_votes) = all_votes.get(proposal_id) {
            for vote in proposal_votes.values() {
                tally.total_weight += vote.vote_weight;
                tally.total_souls += 1;
                match vote.option {
                    VoteOption::Yes => tally.yes += vote.vote_weight,
                    VoteOption::No => tally.no += vote.vote_weight,
                    VoteOption::Abstain => tally.abstain += vote.vote_weight,
                }
            }
        }

        tally
    }

    /// List all votes optionally filtered by proposal_id
    pub fn list_votes(&self, proposal_id: Option<&str>) -> Vec<Vote> {
        let all_votes = self.votes.read().unwrap();
        let mut votes = Vec::new();
        match proposal_id {
            Some(pid) => {
                if let Some(proposal_votes) = all_votes.get(pid) {
                    votes.extend(proposal_votes.values().cloned());
                }
            }
            None => {
                for proposal_votes in all_votes.values() {
                    votes.extend(proposal_votes.values().cloned());
                }
            }
        }
        votes
    }
}