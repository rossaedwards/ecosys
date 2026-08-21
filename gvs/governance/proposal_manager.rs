//! Governance proposal lifecycle management for AuraFS
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use chrono::{Utc};
use uuid::Uuid;

use crate::models::{Proposal, ProposalStatus, ProposalType};
use crate::voting_engine::{VotingEngine, VoteTally};
use crate::audit_log::AuditLogger;

/// Configuration parameters for proposal handling
#[derive(Debug, Clone)]
pub struct ProposalConfig {
    pub min_voting_period_hours: i64,
    pub max_voting_period_hours: i64,
    pub quorum_percentage: f64,     // e.g. 10.0 = 10%
    pub approval_threshold: f64,    // e.g. 0.67 = 67%
}

impl Default for ProposalConfig {
    fn default() -> Self {
        Self {
            min_voting_period_hours: 24,
            max_voting_period_hours: 168,  // 7 days
            quorum_percentage: 10.0,
            approval_threshold: 0.67,
        }
    }
}

/// Manages proposals with a state machine for lifecycle transitions
pub struct ProposalManager {
    proposals: Arc<RwLock<HashMap<String, Proposal>>>,
    voting_engine: Arc<VotingEngine>,
    audit_logger: Arc<AuditLogger>,
    config: ProposalConfig,
}

impl ProposalManager {
    pub fn new(
        voting_engine: Arc<VotingEngine>,
        audit_logger: Arc<AuditLogger>,
        config: Option<ProposalConfig>,
    ) -> Self {
        Self {
            proposals: Arc::new(RwLock::new(HashMap::new())),
            voting_engine,
            audit_logger,
            config: config.unwrap_or_default(),
        }
    }

    /// Create a new proposal with validation of voting period
    pub fn create_proposal(
        &self,
        creator_bliss_id: String,
        title: String,
        description: String,
        proposal_type: ProposalType,
        voting_period_hours: i64,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Proposal, String> {
        if voting_period_hours < self.config.min_voting_period_hours {
            return Err(format!("Voting period too short (min: {} hours)", self.config.min_voting_period_hours));
        }
        if voting_period_hours > self.config.max_voting_period_hours {
            return Err(format!("Voting period too long (max: {} hours)", self.config.max_voting_period_hours));
        }

        let now = Utc::now().timestamp();
        let voting_end = now + (voting_period_hours * 3600);

        let proposal = Proposal {
            id: format!("prop-{}", Uuid::new_v4()),
            title: title.clone(),
            description: description.clone(),
            creator_bliss_id: creator_bliss_id.clone(),
            proposal_type,
            status: ProposalStatus::Draft,
            created_at: now,
            updated_at: now,
            voting_start: None,
            voting_end: Some(voting_end),
            metadata: metadata.unwrap_or_default(),
        };

        {
            let mut proposals = self.proposals.write().unwrap();
            proposals.insert(proposal.id.clone(), proposal.clone());
        }

        self.audit_logger.log_event(
            "proposal_created",
            &format!("id={}, creator={}, title={}", proposal.id, creator_bliss_id, title),
        );

        log::info!("📝 Proposal created: {} by {}", proposal.id, creator_bliss_id);
        Ok(proposal)
    }

    /// Submit a draft proposal for voting (transition to Active)
    pub fn submit_proposal(&self, proposal_id: &str) -> Result<Proposal, String> {
        let mut proposals = self.proposals.write().unwrap();
        let proposal = proposals.get_mut(proposal_id)
            .ok_or_else(|| format!("Proposal not found: {}", proposal_id))?;

        if proposal.status != ProposalStatus::Draft {
            return Err(format!("Proposal not in draft status: {:?}", proposal.status));
        }

        let now = Utc::now().timestamp();
        proposal.status = ProposalStatus::Active;
        proposal.voting_start = Some(now);
        proposal.updated_at = now;

        self.audit_logger.log_event(
            "proposal_submitted",
            &format!("id={}, voting_start={}", proposal_id, now),
        );

        log::info!("✅ Proposal submitted for voting: {}", proposal_id);
        Ok(proposal.clone())
    }

    /// Finalize proposal after voting ends, checks quorum and approval
    pub fn finalize_proposal(&self, proposal_id: &str) -> Result<Proposal, String> {
        let mut proposals = self.proposals.write().unwrap();
        let proposal = proposals.get_mut(proposal_id)
            .ok_or_else(|| format!("Proposal not found: {}", proposal_id))?;

        if let Some(voting_end) = proposal.voting_end {
            if Utc::now().timestamp() < voting_end {
                return Err(format!("Voting period not ended (ends: {})", voting_end));
            }
        }

        let tally = self.voting_engine.tally_votes(proposal_id);

        let quorum_met = self.check_quorum(&tally)?;
        if !quorum_met {
            proposal.status = ProposalStatus::Rejected;
            proposal.updated_at = Utc::now().timestamp();

            self.audit_logger.log_event(
                "proposal_rejected_quorum",
                &format!("id={}, votes={}, quorum_required={}%", proposal_id, tally.total_souls, self.config.quorum_percentage),
            );

            return Ok(proposal.clone());
        }

        let approval_rate = tally.yes / tally.total_weight;
        let approved = approval_rate >= self.config.approval_threshold;

        proposal.status = if approved { ProposalStatus::Approved } else { ProposalStatus::Rejected };
        proposal.updated_at = Utc::now().timestamp();

        self.audit_logger.log_event(
            if approved { "proposal_approved" } else { "proposal_rejected" },
            &format!("id={}, approval_rate={:.2}%, yes={}, no={}, abstain={}",
                proposal_id, approval_rate * 100.0, tally.yes, tally.no, tally.abstain),
        );

        log::info!("🏁 Proposal finalized: {} - {:?}", proposal_id, proposal.status);
        Ok(proposal.clone())
    }

    /// Check quorum based on tally (currently placeholder logic)
    fn check_quorum(&self, tally: &VoteTally) -> Result<bool, String> {
        const MIN_VOTES: usize = 3; // placeholder
        Ok(tally.total_souls >= MIN_VOTES)
    }

    /// Cancel a draft proposal (only creator allowed)
    pub fn cancel_proposal(&self, proposal_id: &str, bliss_id: &str) -> Result<Proposal, String> {
        let mut proposals = self.proposals.write().unwrap();
        let proposal = proposals.get_mut(proposal_id)
            .ok_or_else(|| format!("Proposal not found: {}", proposal_id))?;

        if proposal.creator_bliss_id != bliss_id {
            return Err("Only proposal creator can cancel".to_string());
        }

        if proposal.status != ProposalStatus::Draft {
            return Err(format!("Cannot cancel proposal in status: {:?}", proposal.status));
        }

        proposal.status = ProposalStatus::Cancelled;
        proposal.updated_at = Utc::now().timestamp();

        self.audit_logger.log_event(
            "proposal_cancelled",
            &format!("id={}, creator={}", proposal_id, bliss_id),
        );

        Ok(proposal.clone())
    }

    /// Retrieve proposal by ID
    pub fn get_proposal(&self, proposal_id: &str) -> Option<Proposal> {
        self.proposals.read().unwrap().get(proposal_id).cloned()
    }

    /// List proposals optionally filtered by status
    pub fn list_proposals(&self, status: Option<ProposalStatus>) -> Vec<Proposal> {
        let proposals = self.proposals.read().unwrap();

        match status {
            Some(filter) => proposals.values().filter(|p| p.status == filter).cloned().collect(),
            None => proposals.values().cloned().collect(),
        }
    }

    /// Get proposals created by a particular BlissID
    pub fn get_proposals_by_creator(&self, bliss_id: &str) -> Vec<Proposal> {
        self.proposals.read().unwrap()
            .values()
            .filter(|p| p.creator_bliss_id == bliss_id)
            .cloned()
            .collect()
    }

    /// Check if voting period is currently open on a proposal
    pub fn is_voting_open(&self, proposal_id: &str) -> bool {
        if let Some(proposal) = self.get_proposal(proposal_id) {
            if proposal.status != ProposalStatus::Active {
                return false;
            }
            if let Some(voting_end) = proposal.voting_end {
                return Utc::now().timestamp() < voting_end;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity_verifier::IdentityVerifier;
    use crate::consensus_integration::{ConsensusIntegration, MockLedgerClient};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_proposal_lifecycle() {
        let identity_verifier = Arc::new(IdentityVerifier::new());
        let ledger = Arc::new(MockLedgerClient::new());
        let consensus = Arc::new(ConsensusIntegration::new(
            "node1".to_string(),
            vec!["node1".to_string()],
            ledger,
        ));
        let voting_engine = Arc::new(VotingEngine::new(identity_verifier, consensus));
        let audit_logger = Arc::new(AuditLogger::new());
        let manager = ProposalManager::new(voting_engine, audit_logger, None);

        // Create proposal
        let proposal = manager.create_proposal(
            "bliss:creator".to_string(),
            "Test Proposal".to_string(),
            "This is a test".to_string(),
            ProposalType::NetworkUpgrade,
            48,
            None,
        ).unwrap();

        assert_eq!(proposal.status, ProposalStatus::Draft);

        // Submit for voting
        let active_proposal = manager.submit_proposal(&proposal.id).unwrap();
        assert_eq!(active_proposal.status, ProposalStatus::Active);
        assert!(manager.is_voting_open(&proposal.id));
    }
}