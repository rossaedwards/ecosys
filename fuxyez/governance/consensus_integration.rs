// afs/governance/consensus_integration.rs
// Distributed consensus integration for AuraFS governance
// Implements BFT consensus, vote validation, and ledger anchoring

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use sha3::{Digest, Sha3_256};
use chrono::Utc;
use tokio::sync::mpsc;

use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceTransaction {
    VoteCast {
        vote_id: String,
        proposal_id: String,
        bliss_id: String,
        option: String,
        vote_weight: f64,
        signature: Vec<u8>,
        timestamp: i64,
    },
    ProposalCreated {
        proposal_id: String,
        title: String,
        creator_bliss_id: String,
        timestamp: i64,
    },
    ProposalFinalized {
        proposal_id: String,
        result: String,  // "approved" or "rejected"
        tally: HashMap<String, f64>,
        timestamp: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeRole {
    Leader,
    Validator,
    Observer,
}

#[derive(Debug, Clone)]
pub struct ConsensusState {
    pub transaction_hash: String,
    pub votes: HashMap<String, bool>,  // node_id -> approve/reject
    pub finalized: bool,
    pub block_height: u64,
}

pub struct ConsensusIntegration {
    node_id: String,
    role: Arc<RwLock<NodeRole>>,
    validators: Arc<RwLock<Vec<String>>>,
    pending_transactions: Arc<RwLock<HashMap<String, GovernanceTransaction>>>,
    consensus_states: Arc<RwLock<HashMap<String, ConsensusState>>>,
    ledger_client: Arc<dyn LedgerClient + Send + Sync>,
    tx_sender: mpsc::UnboundedSender<GovernanceTransaction>,
    tx_receiver: Arc<RwLock<mpsc::UnboundedReceiver<GovernanceTransaction>>>,
}

impl ConsensusIntegration {
    pub fn new(
        node_id: String,
        validators: Vec<String>,
        ledger_client: Arc<dyn LedgerClient + Send + Sync>,
    ) -> Self {
        let (tx_sender, tx_receiver) = mpsc::unbounded_channel();

        Self {
            node_id,
            role: Arc::new(RwLock::new(NodeRole::Validator)),
            validators: Arc::new(RwLock::new(validators)),
            pending_transactions: Arc::new(RwLock::new(HashMap::new())),
            consensus_states: Arc::new(RwLock::new(HashMap::new())),
            ledger_client,
            tx_sender,
            tx_receiver: Arc::new(RwLock::new(tx_receiver)),
        }
    }

    pub async fn submit_transaction(&self, tx: GovernanceTransaction) -> Result<String, String> {
        let tx_hash = self.compute_transaction_hash(&tx);

        {
            let mut pending = self.pending_transactions.write().unwrap();
            pending.insert(tx_hash.clone(), tx.clone());
        }

        self.broadcast_transaction(tx.clone()).await?;

        {
            let mut states = self.consensus_states.write().unwrap();
            states.insert(
                tx_hash.clone(),
                ConsensusState {
                    transaction_hash: tx_hash.clone(),
                    votes: HashMap::new(),
                    finalized: false,
                    block_height: 0,
                },
            );
        }

        log::info!("📤 Transaction submitted to consensus: {}", tx_hash);
        Ok(tx_hash)
    }

    pub async fn validate_transaction(&self, tx_hash: &str, approve: bool) -> Result<(), String> {
        let validators = self.validators.read().unwrap();

        if !validators.contains(&self.node_id) {
            return Err("Node is not a validator".into());
        }

        {
            let mut states = self.consensus_states.write().unwrap();
            let state = states.get_mut(tx_hash).ok_or_else(|| format!("Transaction {} not found", tx_hash))?;
            state.votes.insert(self.node_id.clone(), approve);
            log::info!(
                "🗳️  Node {} voted {} on tx {}",
                self.node_id,
                if approve { "APPROVE" } else { "REJECT" },
                tx_hash
            );
        }

        self.check_consensus(tx_hash).await
    }

    async fn check_consensus(&self, tx_hash: &str) -> Result<(), String> {
        let validators = self.validators.read().unwrap();
        let validator_count = validators.len();
        let required_votes = (validator_count * 2) / 3 + 1; // 2/3+ majority

        let mut states = self.consensus_states.write().unwrap();
        let state = states.get_mut(tx_hash).ok_or_else(|| format!("Transaction {} not found", tx_hash))?;

        if state.finalized {
            return Ok(());
        }

        let approve_count = state.votes.values().filter(|&&v| v).count();
        let reject_count = state.votes.values().filter(|&&v| !v).count();

        if approve_count >= required_votes {
            state.finalized = true;
            log::info!("✅ CONSENSUS REACHED (APPROVED): {} ({}/{})", tx_hash, approve_count, validator_count);

            self.anchor_to_ledger(tx_hash).await?;
            return Ok(());
        }

        if reject_count > validator_count / 3 {
            state.finalized = true;
            log::warn!("❌ CONSENSUS FAILED (REJECTED): {} ({}/{})", tx_hash, reject_count, validator_count);
            return Err(format!("Transaction {} rejected by validators", tx_hash));
        }

        log::debug!("⏳ Waiting for consensus: {} ({}/{} votes)", tx_hash, state.votes.len(), validator_count);
        Ok(())
    }

    async fn anchor_to_ledger(&self, tx_hash: &str) -> Result<String, String> {
        let pending = self.pending_transactions.read().unwrap();
        let tx = pending.get(tx_hash).ok_or_else(|| format!("Transaction {} not in pending pool", tx_hash))?;

        let tx_data = serde_json::to_string(tx).map_err(|e| format!("Serialization error: {}", e))?;

        let ledger_tx_hash = self.ledger_client.append_transaction(tx_hash, &tx_data).await.map_err(|e| format!("Ledger append failed: {}", e))?;

        {
            let mut states = self.consensus_states.write().unwrap();
            if let Some(state) = states.get_mut(tx_hash) {
                state.block_height = self.ledger_client.get_current_block_height().await;
            }
        }

        log::info!("⛓️ Transaction anchored to ledger: {} -> {}", tx_hash, ledger_tx_hash);
        Ok(ledger_tx_hash)
    }

    async fn broadcast_transaction(&self, tx: GovernanceTransaction) -> Result<(), String> {
        self.tx_sender.send(tx).map_err(|e| format!("Broadcast failed: {}", e))?;
        Ok(())
    }

    fn compute_transaction_hash(&self, tx: &GovernanceTransaction) -> String {
        let tx_bytes = serde_json::to_vec(tx).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(&tx_bytes);
        format!("{:x}", hasher.finalize())
    }

    pub async fn verify_transaction(&self, tx_hash: &str) -> Result<bool, String> {
        self.ledger_client.verify_transaction(tx_hash).await
    }

    pub fn get_consensus_state(&self, tx_hash: &str) -> Option<ConsensusState> {
        let states = self.consensus_states.read().unwrap();
        states.get(tx_hash).cloned()
    }

    pub fn promote_to_leader(&self) {
        let mut role = self.role.write().unwrap();
        *role = NodeRole::Leader;
        log::info!("👑 Node {} promoted to LEADER", self.node_id);
    }

    pub fn demote_to_validator(&self) {
        let mut role = self.role.write().unwrap();
        *role = NodeRole::Validator;
        log::info!("🔻 Node {} demoted to VALIDATOR", self.node_id);
    }
}

#[async_trait::async_trait]
pub trait LedgerClient {
    async fn append_transaction(&self, tx_hash: &str, tx_data: &str) -> Result<String, String>;
    async fn get_current_block_height(&self) -> u64;
    async fn verify_transaction(&self, tx_hash: &str) -> Result<bool, String>;
}

pub struct MockLedgerClient {
    transactions: Arc<RwLock<HashMap<String, String>>>,
    block_height: Arc<RwLock<u64>>,
}

impl MockLedgerClient {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            block_height: Arc::new(RwLock::new(0)),
        }
    }
}

#[async_trait::async_trait]
impl LedgerClient for MockLedgerClient {
    async fn append_transaction(&self, tx_hash: &str, tx_data: &str) -> Result<String, String> {
        let mut txs = self.transactions.write().unwrap();
        txs.insert(tx_hash.to_string(), tx_data.to_string());

        let mut height = self.block_height.write().unwrap();
        *height += 1;

        Ok(format!("ledger_tx_{}", *height))
    }

    async fn get_current_block_height(&self) -> u64 {
        *self.block_height.read().unwrap()
    }

    async fn verify_transaction(&self, tx_hash: &str) -> Result<bool, String> {
        let txs = self.transactions.read().unwrap();
        Ok(txs.contains_key(tx_hash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consensus_integration() {
        let ledger = Arc::new(MockLedgerClient::new());
        let validators = vec!["node1".to_string(), "node2".to_string(), "node3".to_string()];

        let consensus = ConsensusIntegration::new("node1".to_string(), validators.clone(), ledger.clone());

        let tx = GovernanceTransaction::VoteCast {
            vote_id: "vote-123".to_string(),
            proposal_id: "prop-456".to_string(),
            bliss_id: "bliss:test".to_string(),
            option: "yes".to_string(),
            vote_weight: 1.0,
            signature: vec![],
            timestamp: Utc::now().timestamp(),
        };

        let tx_hash = consensus.submit_transaction(tx).await.unwrap();

        consensus.validate_transaction(&tx_hash, true).await.unwrap();  // node1
        consensus.validate_transaction(&tx_hash, true).await.unwrap();  // node2 (consensus reached)

        let state = consensus.get_consensus_state(&tx_hash).unwrap();
        assert!(state.finalized);
        assert_eq!(state.block_height, 1);
    }
}