//! Governance proposal lifecycle management for AuraFS
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use uuid::Uuid;
use log::{info, warn, error};

use crate::models::{Proposal, ProposalStatus, ProposalType};
use crate::voting_engine::{VotingEngine, VoteTally};
use crate::audit_log::AuditLogger;
use crate::policy_enforcer::PolicyEnforcer;

/// Configuration parameters for proposal handling
#[derive(Debug, Clone)]
pub struct ProposalConfig {
    pub min_voting_period_hours: i64,
    pub max_voting_period_hours: i64,
    pub quorum_percentage: f64,     // e.g. 10.0 = 10%
    pub approval_threshold: f64,    // e.g. 0.66 = 66% (Supermajority required for Physics changes)
}

impl Default for ProposalConfig {
    fn default() -> Self {
        Self {
            min_voting_period_hours: 24,
            max_voting_period_hours: 168,  // 7 days
            quorum_percentage: 10.0,
            approval_threshold: 0.66,      // 2/3rds Majority
        }
    }
}

pub struct ProposalManager {
    proposals: Arc<RwLock<HashMap<String, Proposal>>>,
    voting_engine: Arc<VotingEngine>,
    policy_enforcer: Arc<PolicyEnforcer>,
    audit_logger: Arc<AuditLogger>,
    config: ProposalConfig,
}

impl ProposalManager {
    pub fn new(
        voting_engine: Arc<VotingEngine>,
        policy_enforcer: Arc<PolicyEnforcer>,
        audit_logger: Arc<AuditLogger>,
        config: Option<ProposalConfig>,
    ) -> Self {
        Self {
            proposals: Arc::new(RwLock::new(HashMap::new())),
            voting_engine,
            policy_enforcer,
            audit_logger,
            config: config.unwrap_or_default(),
        }
    }

    /// Create a new proposal to change the Lattice
    pub fn create_proposal(
        &self,
        creator_bliss_id: String,
        title: String,
        description: String,
        proposal_type: ProposalType,
        voting_period_hours: i64,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Proposal, String> {
        // 1. Validate Config
        if voting_period_hours < self.config.min_voting_period_hours 
           || voting_period_hours > self.config.max_voting_period_hours {
            return Err("Invalid voting period".to_string());
        }

        let now = Utc::now().timestamp();
        let voting_end = now + (voting_period_hours * 3600);

        // 2. Construct Proposal
        let proposal = Proposal {
            id: format!("prop-{}", Uuid::new_v4()),
            title,
            description,
            creator_bliss_id: creator_bliss_id.clone(),
            proposal_type,
            status: ProposalStatus::Active, // In Phase II, we skip 'Draft' for speed
            created_at: now,
            updated_at: now,
            voting_start: Some(now),
            voting_end: Some(voting_end),
            metadata: metadata.unwrap_or_default(),
        };

        // 3. Store
        {
            let mut proposals = self.proposals.write().unwrap();
            proposals.insert(proposal.id.clone(), proposal.clone());
        }

        self.audit_logger.log_event(
            "proposal_created",
            &format!("id={}, creator={}", proposal.id, creator_bliss_id),
        );

        info!("📜 Proposal Created: {} by {}", proposal.title, creator_bliss_id);

        Ok(proposal)
    }

    /// Check if voting period ended and tally votes
    pub async fn finalize_proposal(&self, proposal_id: &str) -> Result<ProposalStatus, String> {
        // Scope the lock to retrieve data, then drop it before async calls
        let proposal_clone = {
            let proposals = self.proposals.read().unwrap();
            proposals.get(proposal_id).cloned().ok_or("Proposal not found")?
        };

        if proposal_clone.status != ProposalStatus::Active {
            return Err("Proposal is not active".to_string());
        }

        if Utc::now().timestamp() < proposal_clone.voting_end.unwrap_or(0) {
            return Err("Voting period has not ended".to_string());
        }

        // 1. Get Tally (Strict Integer Math)
        let tally = self.voting_engine.tally_votes(proposal_id);
        
        let support = tally.yes;
        let opposition = tally.no;
        let total_opinionated = support + opposition;
        
        // 2. Calculate Outcome
        let passed = if total_opinionated == 0 {
            false
        } else {
            let ratio = support as f64 / total_opinionated as f64;
            ratio >= self.config.approval_threshold
        };

        // 3. Update Status
        let mut final_status = if passed { ProposalStatus::Approved } else { ProposalStatus::Rejected };
        
        {
            let mut proposals = self.proposals.write().unwrap();
            if let Some(p) = proposals.get_mut(proposal_id) {
                p.status = final_status;
                p.updated_at = Utc::now().timestamp();
            }
        }

        // 4. Enforce (The "Hand" of Governance)
        if passed {
            info!("✅ Proposal Passed: {}", proposal_id);
            self.audit_logger.log_event("proposal_passed", proposal_id);
            
            // Execute the change (Transmutation, etc.)
            match self.policy_enforcer.enforce_proposal(&proposal_clone).await {
                Ok(_) => {
                    // Mark as executed
                    let mut proposals = self.proposals.write().unwrap();
                    if let Some(p) = proposals.get_mut(proposal_id) {
                        p.status = ProposalStatus::Executed;
                    }
                    final_status = ProposalStatus::Executed;
                    info!("🚀 Proposal Executed Successfully: {}", proposal_id);
                }
                Err(e) => {
                    error!("❌ Execution Failed for {}: {}", proposal_id, e);
                    // Status remains 'Approved' but action failed (needs manual intervention)
                }
            }
        } else {
            info!("❌ Proposal Rejected: {}", proposal_id);
            self.audit_logger.log_event("proposal_rejected", proposal_id);
        }

        Ok(final_status)
    }

    pub fn get_proposal(&self, proposal_id: &str) -> Option<Proposal> {
        self.proposals.read().unwrap().get(proposal_id).cloned()
    }

    pub fn list_proposals(&self) -> Vec<Proposal> {
        self.proposals.read().unwrap().values().cloned().collect()
    }
}