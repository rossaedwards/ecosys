//! Voting engine for AuraFS governance - STRICT EGALITARIAN (1 Soul = 1 Vote)
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use anyhow::{Result, anyhow};
use log::{info, warn};

use crate::identity_verifier::IdentityVerifier;
use crate::consensus_integration::{ConsensusIntegration, GovernanceTransaction};
use crate::transaction_type::{VoteOption, GovernanceTransactionType};
use crate::soulsync_engine::SoulSyncEngine;

/// Vote representation stored within the voting engine
#[derive(Debug, Clone, serde::Serialize)]
pub struct Vote {
    pub proposal_id: String,
    pub bliss_id: String,
    pub option: VoteOption,
    pub vote_weight: f64, // Always 1.0 in Phase II
    pub timestamp: i64,
}

/// Aggregated vote tally (Strict Integer Counting)
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct VoteTally {
    pub yes: u64,
    pub no: u64,
    pub abstain: u64,
    pub total_votes: u64,
}

/// Main voting engine struct
pub struct VotingEngine {
    identity_verifier: Arc<IdentityVerifier>,
    consensus: Arc<ConsensusIntegration>,
    soulsync_engine: Arc<SoulSyncEngine>,
    votes: Arc<RwLock<HashMap<String, HashMap<String, Vote>>>>, // proposal_id -> bliss_id -> Vote
}

impl VotingEngine {
    /// Instantiate new VotingEngine with dependencies
    pub fn new(
        identity_verifier: Arc<IdentityVerifier>,
        consensus: Arc<ConsensusIntegration>,
        soulsync_engine: Arc<SoulSyncEngine>,
    ) -> Self {
        Self {
            identity_verifier,
            consensus,
            soulsync_engine,
            votes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Cast a vote with quantum-safe signature verification AND SoulShot validation
    pub async fn cast_vote(
        &self,
        proposal_id: String,
        bliss_id: String,
        option: VoteOption,
        signature: Vec<u8>,
        soul_proof: crate::identity_verifier::SoulProof,
        _metadata: Option<HashMap<String, String>>,
    ) -> Result<Vote> {
        // 1. Verify BlissID Identity (Signature Check)
        if !self.identity_verifier.verify_bliss_id(&bliss_id) {
            return Err(anyhow!("Invalid or inactive BlissID"));
        }

        // 2. Verify SoulShot (Humanity Check)
        // We do NOT calculate a score. We ask: "Is this a valid, unique human?"
        if !self.soulsync_engine.verify_soul_shot(&bliss_id).await? {
            return Err(anyhow!("SoulShot verification failed: Soul not unique or alive"));
        }

        // 3. Verify Quantum Signature
        let message = format!("{}:{:?}", proposal_id, option);
        if !self.identity_verifier.verify_quantum_signature(&bliss_id, message.as_bytes(), &signature)? {
            return Err(anyhow!("Invalid quantum signature"));
        }

        // 4. Verify Soul Proof (Zero-Knowledge Uniqueness Proof)
        if !self.identity_verifier.verify_soul_proof(&bliss_id, &soul_proof) {
            return Err(anyhow!("Invalid soul proof"));
        }

        // 5. Enforce 1 Soul = 1 Vote
        let vote_weight = 1.0;

        let vote = Vote {
            proposal_id: proposal_id.clone(),
            bliss_id: bliss_id.clone(),
            option,
            vote_weight,
            timestamp: Utc::now().timestamp(),
        };

        // 6. Local Storage & Double Vote Check
        {
            let mut all_votes = self.votes.write().unwrap();
            let proposal_votes = all_votes.entry(proposal_id.clone()).or_default();
            
            if proposal_votes.contains_key(&bliss_id) {
                return Err(anyhow!("Soul has already voted on this proposal"));
            }
            
            proposal_votes.insert(bliss_id.clone(), vote.clone());
        }

        // 7. Broadcast to Consensus
        let tx = GovernanceTransaction::VoteCast {
            vote_id: uuid::Uuid::new_v4().to_string(),
            proposal_id,
            bliss_id: bliss_id.clone(),
            option: format!("{:?}", vote.option),
            vote_weight,
            signature,
            timestamp: vote.timestamp,
        };

        self.consensus.submit_transaction(tx).await.map_err(|e| anyhow!(e))?;

        info!("🗳️ Vote Cast: {} (SoulShot Verified)", bliss_id);

        Ok(vote)
    }

    /// Tally votes by strictly counting souls (Integer Math)
    pub fn tally_votes(&self, proposal_id: &str) -> VoteTally {
        let all_votes = self.votes.read().unwrap();
        let mut tally = VoteTally::default();

        if let Some(proposal_votes) = all_votes.get(proposal_id) {
            for vote in proposal_votes.values() {
                tally.total_votes += 1;
                match vote.option {
                    VoteOption::Yes => tally.yes += 1,
                    VoteOption::No => tally.no += 1,
                    VoteOption::Abstain => tally.abstain += 1,
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