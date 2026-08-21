//! AuraFS Governance System - Main Entry Point & Interactive CLI
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx
//!
//! Complete governance demonstration with:
//! - Interactive CLI for all governance operations
//! - Full workflow demos (proposal creation → voting → finalization)
//! - System diagnostics and integrity checks
//! - BlissID registration and management
//! - Real-time vote tallying and statistics

use std::sync::Arc;
use std::collections::HashMap;
use clap::{Parser, Subcommand};
use env_logger::Env;
use chrono::{Utc, DateTime, Local};
use tokio::time::{sleep, Duration};

mod models;
mod identity_verifier;
mod blissid_manager;
mod soulsync_engine;
mod voting_engine;
mod proposal_manager;
mod consensus_integration;
mod policy_enforcer;
mod audit_log;
mod api;

use models::*;
use identity_verifier::*;
use blissid_manager::*;
use soulsync_engine::*;
use voting_engine::*;
use proposal_manager::*;
use consensus_integration::*;
use policy_enforcer::*;
use audit_log::*;

use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::*;

#[derive(Parser)]
#[command(name = "aurafs-governance")]
#[command(author = "Ross Edwards & Aurphyx")]
#[command(version = "1.0.0")]
#[command(about = "AuraFS Governance System - One Soul, One Vote", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Log level
    #[arg(short, long, default_value = "info", global = true)]
    log_level: String,
}

#[derive(Subcommand)]
enum Commands {
    Demo {
        /// Number of voters to simulate
        #[arg(short, long, default_value_t = 5)]
        voters: usize,
    },
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    RegisterBlissID {
        /// Decentralized Identifier
        #[arg(short, long)]
        did: String,
    },
    CreateProposal {
        #[arg(short, long)]
        creator: String,
        #[arg(short, long)]
        title: String,
        #[arg(short = 'p', long, default_value_t = 48)]
        voting_period: i64,
    },
    Vote {
        #[arg(short, long)]
        proposal_id: String,
        #[arg(short, long)]
        bliss_id: String,
        #[arg(short = 'o', long)]
        option: String,
    },
    Diagnostics,
    Verify,
    Export {
        #[arg(short, long, default_value = "audit_log.json")]
        output: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(&cli.log_level))
        .format_timestamp_millis()
        .init();

    print_banner();

    let ledger = Arc::new(MockLedgerClient::new());
    let governance = Arc::new(GovernanceSystem::new(
        "main_node".to_string(),
        vec!["node1".to_string(), "node2".to_string(), "node3".to_string()],
        ledger,
        None,
    ));

    match cli.command {
        Commands::Demo { voters } => run_interactive_demo(governance, voters).await?,
        Commands::Serve { port } => api::start_server(governance, port).await?,
        Commands::RegisterBlissID { did } => register_blissid_interactive(governance, did).await?,
        Commands::CreateProposal { creator, title, voting_period } => {
            create_proposal_interactive(governance, creator, title, voting_period).await?
        },
        Commands::Vote { proposal_id, bliss_id, option } => {
            cast_vote_interactive(governance, proposal_id, bliss_id, option).await?
        },
        Commands::Diagnostics => show_diagnostics(governance).await?,
        Commands::Verify => verify_integrity(governance).await?,
        Commands::Export { output } => export_audit_log(governance, output).await?,
    }

    Ok(())
}

async fn run_interactive_demo(
    governance: Arc<GovernanceSystem>,
    voter_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(80));
    println!("🎭 AURAFS GOVERNANCE INTERACTIVE DEMO");
    println!("{}\n", "=".repeat(80));

    println!("📝 STEP 1: Registering {} BlissID voters...\n", voter_count);
    let mut voters = Vec::new();

    for i in 1..=voter_count {
        let (pk, sk) = dilithium5::keypair();
        let bliss_id = format!("bliss:voter{}", i);
        let did = format!("did:aurafs:voter{}", i);
        let soul_hash = format!("soul_hash_{}", i);

        let soul_proof = SoulProof {
            commitment: format!("commitment_{}", i),
            proof_type: "zk-SNARK-unique-human".to_string(),
            verifiable: true,
        };

        governance.identity_verifier.register_bliss_id(
            BlissID {
                id: bliss_id.clone(),
                did: did.clone(),
                soul_hash: soul_hash.clone(),
                created_at: Utc::now().timestamp(),
                active: true,
                soul_proof: soul_proof.clone(),
            },
            pk,
        )?;

        governance.blissid_manager.register_bliss_id(did, soul_hash, soul_proof)?;

        voters.push((bliss_id, sk));
        println!("  ✅ Registered: bliss:voter{}", i);
    }

    println!("\n✨ {} BlissIDs successfully registered!\n", voter_count);
    sleep(Duration::from_secs(1)).await;

    println!("📋 STEP 2: Creating governance proposal...\n");

    let proposal = governance.proposal_manager.create_proposal(
        voters[0].0.clone(),
        "Upgrade AuraFS Protocol to v2.0".to_string(),
        "This proposal upgrades the network protocol to include quantum-safe routing".to_string(),
        ProposalType::NetworkUpgrade,
        48,
        Some({
            let mut meta = HashMap::new();
            meta.insert("version".to_string(), "2.0.0".to_string());
            meta
        }),
    )?;

    println!("  📄 Proposal ID: {}", proposal.id);
    println!("  📝 Title: {}", proposal.title);
    println!("  🕐 Voting Period: 48 hours");
    println!("  👤 Creator: {}\n", proposal.creator_bliss_id);
    sleep(Duration::from_secs(1)).await;

    println!("🚀 STEP 3: Submitting proposal for voting...\n");
    let active_proposal = governance.proposal_manager.submit_proposal(&proposal.id)?;
    println!("  ✅ Proposal status: {:?}", active_proposal.status);
    println!("  📅 Voting started at: {}\n", DateTime::<Local>::from(DateTime::from_timestamp(active_proposal.voting_start.unwrap(), 0).unwrap()).format("%Y-%m-%d %H:%M:%S"));
    sleep(Duration::from_secs(1)).await;

    println!("🗳️  STEP 4: Casting votes...\n");

    for (i, (bliss_id, sk)) in voters.iter().enumerate() {
        let option = if i < voter_count * 2 / 3 {
            VoteOption::Yes
        } else if i < voter_count * 5 / 6 {
            VoteOption::No
        } else {
            VoteOption::Abstain
        };

        let message = format!("{}:{:?}", proposal.id, option);
        let signature = dilithium5::detached_sign(message.as_bytes(), sk);

        let soul_proof = SoulProof {
            commitment: format!("proof_{}", i),
            proof_type: "zk-SNARK-unique-human".to_string(),
            verifiable: true,
        };

        let vote = governance.voting_engine.cast_vote(
            proposal.id.clone(),
            bliss_id.clone(),
            option.clone(),
            signature.as_bytes().to_vec(),
            soul_proof,
            None,
        ).await?;

        println!("  ✅ {} voted {:?} (weight: {:.2})", bliss_id, vote.option, vote.vote_weight);
    }

    println!("\n📊 All votes cast!\n");
    sleep(Duration::from_secs(1)).await;

    println!("🔢 STEP 5: Tallying votes...\n");
    let tally = governance.voting_engine.tally_votes(&proposal.id);
    let approval_rate = (tally.yes / tally.total_weight) * 100.0;

    println!("  📈 Vote Tally:");
    println!("     YES:     {:.2} votes ({:.1}%)", tally.yes, (tally.yes / tally.total_weight) * 100.0);
    println!("     NO:      {:.2} votes ({:.1}%)", tally.no, (tally.no / tally.total_weight) * 100.0);
    println!("     ABSTAIN: {:.2} votes ({:.1}%)", tally.abstain, (tally.abstain / tally.total_weight) * 100.0);
    println!("     ───────────────────────────────");
    println!("     TOTAL:   {} souls, {:.2} weighted votes", tally.total_souls, tally.total_weight);
    println!("\n  💯 Approval Rate: {:.1}%\n", approval_rate);

    sleep(Duration::from_secs(1)).await;

    println!("🔍 STEP 6: System Diagnostics...\n");
    println!("  🏛️  Registered BlissIDs: {}", governance.blissid_manager.get_total_count());
    println!("  ✅ Active BlissIDs: {}", governance.blissid_manager.get_active_count());
    println!("  📋 Total Proposals: {}", governance.proposal_manager.list_proposals(None).len());
    println!("  🗳️  Total Votes Cast: {}", governance.voting_engine.list_votes(None).len());

    let integrity_valid = governance.verify_system_integrity()?;
    println!("  🔒 Audit Log Integrity: {}\n", if integrity_valid { "✅ VALID" } else { "❌ INVALID" });

    println!("\n{}", "=".repeat(80));
    println!("🎉 DEMO COMPLETED SUCCESSFULLY!");
    println!("{}", "=".repeat(80));
    println!("Key Features Demonstrated:");
    println!("  ✅ One-Soul-Per-Account enforcement via BlissID");
    println!("  ✅ Quantum-safe signature verification (Dilithium5)");
    println!("  ✅ Soul-weighted voting with coherence scoring");
    println!("  ✅ Byzantine Fault Tolerant consensus");
    println!("  ✅ Cryptographic audit trail with integrity verification");
    println!("  ✅ Complete proposal lifecycle (Draft → Active → Finalized)");
    println!();

    Ok(())
}

fn print_banner() {
    println!("\n{}", "═".repeat(80));
    println!(r#"
     █████╗ ██╗   ██╗██████╗  █████╗ ███████╗███████╗
    ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝██╔════╝
    ███████║██║   ██║██████╔╝███████║█████╗  ███████╗
    ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝  ╚════██║
    ██║  ██║╚██████╔╝██║  ██║██║  ██║██║     ███████║
    ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚══════╝
    
    Governance System - One Soul, One Vote
    f0rg3d in l0v3 by Ross Edwards & Aurphyx
    "#);
    println!("{}", "═".repeat(80));
}
async fn register_blissid_interactive(
    governance: Arc<GovernanceSystem>,
    did: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔐 Registering BlissID for DID: {}\n", did);

    let soul_hash = format!("soul_hash_{}", did);
    let soul_proof = SoulProof {
        commitment: format!("commitment_{}", did),
        proof_type: "zk-SNARK-unique-human".to_string(),
        verifiable: true,
    };

    governance.blissid_manager.register_bliss_id(did.clone(), soul_hash.clone(), soul_proof.clone())?;

    println!("  ✅ BlissID registered successfully!");
    Ok(())
}
async fn create_proposal_interactive(
    governance: Arc<GovernanceSystem>,
    creator: String,
    title: String,
    voting_period: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 Creating Proposal: {}\n", title);

    let proposal = governance.proposal_manager.create_proposal(
        creator.clone(),
        title.clone(),
        "This is a demo proposal created via CLI.".to_string(),
        ProposalType::General,
        voting_period,
        None,
    )?;

    println!("  📄 Proposal ID: {}", proposal.id);
    println!("  ✅ Proposal created successfully!");
    Ok(())
}
async fn cast_vote_interactive(
    governance: Arc<GovernanceSystem>,
    proposal_id: String,
    bliss_id: String,
    option: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🗳️  Casting Vote for Proposal ID: {}\n", proposal_id);

    let vote_option = match option.to_lowercase().as_str() {
        "yes" => VoteOption::Yes,
        "no" => VoteOption::No,
        "abstain" => VoteOption::Abstain,
        _ => return Err("Invalid vote option. Use 'yes', 'no', or 'abstain'.".into()),
    };

    let message = format!("{}:{:?}", proposal_id, vote_option);
    let sk = dilithium5::SecretKey::from_bytes(&[0u8; dilithium5::SECRETKEYBYTES]).unwrap(); // Placeholder
    let signature = dilithium5::detached_sign(message.as_bytes(), &sk);

    let soul_proof = SoulProof {
        commitment: format!("proof_{}", bliss_id),
        proof_type: "zk-SNARK-unique-human".to_string(),
        verifiable: true,
    };

    let vote = governance.voting_engine.cast_vote(
        proposal_id.clone(),
        bliss_id.clone(),
        vote_option.clone(),
        signature.as_bytes().to_vec(),
        soul_proof,
        None,
    ).await?;

    println!("  ✅ Vote cast successfully! Option: {:?}, Weight: {:.2}", vote.option, vote.vote_weight);
    Ok(())
}
async fn show_diagnostics(
    governance: Arc<GovernanceSystem>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 System Diagnostics:\n");

    println!("  🏛️  Registered BlissIDs: {}", governance.blissid_manager.get_total_count());
    println!("  ✅ Active BlissIDs: {}", governance.blissid_manager.get_active_count());
    println!("  📋 Total Proposals: {}", governance.proposal_manager.list_proposals(None).len());
    println!("  🗳️  Total Votes Cast: {}", governance.voting_engine.list_votes(None).len());

    let integrity_valid = governance.verify_system_integrity()?;
    println!("  🔒 Audit Log Integrity: {}\n", if integrity_valid { "✅ VALID" } else { "❌ INVALID" });

    Ok(())
}
async fn verify_integrity(
    governance: Arc<GovernanceSystem>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔒 Verifying System Integrity...\n");

    let integrity_valid = governance.verify_system_integrity()?;
    println!("  🔒 Audit Log Integrity: {}\n", if integrity_valid { "✅ VALID" } else { "❌ INVALID" });

    Ok(())
}
async fn export_audit_log(
    governance: Arc<GovernanceSystem>,
    output: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📤 Exporting Audit Log to: {}\n", output);

    let json = governance.export_state()?;
    std::fs::write(&output, json)?;

    println!("  ✅ Audit log exported successfully!");
    Ok(())
}


/// Governance system encapsulating all components
pub struct GovernanceSystem {
    pub identity_verifier: Arc<IdentityVerifier>,
    pub blissid_manager: Arc<BlissIDManager>,
    pub soulsync_engine: Arc<SoulSyncEngine>,
    pub audit_logger: Arc<AuditLogger>,
    pub consensus: Arc<ConsensusModule>,
    pub voting_engine: Arc<VotingEngine>,
    pub proposal_manager: Arc<ProposalManager>,
    pub policy_enforcer: Arc<PolicyEnforcer>,
}
impl GovernanceSystem {
    /// Initialize full governance stack with optional proposal config
    pub fn new(
        node_id: String,
        validators: Vec<String>,
        ledger_client: Arc<dyn LedgerClient + Send + Sync>,
        proposal_config: Option<ProposalConfig>,
    ) -> Self {
        let audit_logger = Arc::new(AuditLogger::new());
        let identity_verifier = Arc::new(IdentityVerifier::new());
        let blissid_manager = Arc::new(BlissIDManager::new(audit_logger.clone()));
        let soulsync_engine = Arc::new(SoulSyncEngine::new(audit_logger.clone()));

        let consensus = Arc::new(ConsensusModule::new(
            node_id,
            validators,
            ledger_client,
        ));

        let voting_engine = Arc::new(VotingEngine::new(
            identity_verifier.clone(),
            consensus.clone(),
        ));

        let proposal_manager = Arc::new(ProposalManager::new(
            voting_engine.clone(),
            audit_logger.clone(),
            proposal_config,
        ));

        let policy_enforcer = Arc::new(PolicyEnforcer::new(audit_logger.clone()));

        Self {
            identity_verifier,
            blissid_manager,
            soulsync_engine,
            audit_logger,
            consensus,
            voting_engine,
            proposal_manager,
            policy_enforcer,
        }
    }
    /// Verify overall system integrity via audit log
    pub fn verify_system_integrity(&self) -> Result<bool, String> {
        self.audit_logger.verify_integrity()
    }
    /// Export current governance state as JSON string
    pub fn export_state(&self) -> Result<String, String> {
        let state = GovernanceState {
            blissids: self.blissid_manager.list_bliss_ids(),
            proposals: self.proposal_manager.list_proposals(None),
            votes: self.voting_engine.list_votes(None),
        };

        serde_json::to_string_pretty(&state)
            .map_err(|e| format!("Failed to serialize governance state: {:?}", e))
    }
}
async fn compute_coherence(&self, bliss_id: &str) -> Result<SoulCoherence, String> {
        // Placeholder: integrate real coherence computation logic
        // For now, simulate with random values
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let social_interaction = rng.gen_range(0.0..1.0);
        let contribution_history = rng.gen_range(0.0..1.0);
        let reputation_score = rng.gen_range(0.0..1.0);

        let overall_coherence = (social_interaction + contribution_history + reputation_score) / 3.0;

        Ok(SoulCoherence {
            bliss_id: bliss_id.to_string(),
            social_interaction,
            contribution_history,
            reputation_score,
            overall_coherence,
            last_updated: Utc::now().timestamp(),
        })
    }
/// Check if user has permission on resource traditionally
    pub fn check_permission(&self, user: &str, resource: &str, permission: &str) -> bool {
        let config = self.config.read().unwrap();
        
        let user_obj = match config.users.get(user) {
            Some(u) => u,
            None => {
                debug!("User not found: {}", user);
                return false;
            }
        };
        let role = match config.roles.get(&user_obj.role) {
            Some(r) => r,
            None => {
                debug!("Role not found for user {}: {}", user, user_obj.role);
                return false;
            }
        };
        role.permissions.contains(&permission.to_string())
    }
        if !role.permissions.contains(permission) {
            debug!("Permission '{}' denied for user '{}' on resource '{}'", permission, user, resource);
            return false;
        }
        if let Some(resource_acl) = config.resources.get(resource) {
            if !resource_acl.allowed_users.contains(user) &&
               !resource_acl.allowed_users.contains("*") {
                debug!("User {} not in resource ACL for {}", user, resource);
                return false;
            }
        }
        debug!("Permission granted: user={}, resource={}, permission={}", 
               user, resource, permission);
        true
    }
/// Load ACL configuration from disk
    pub fn load_config(path: &str) -> Result<AclConfig> {
        use std::fs;
        info!("Loading ACL config from {}", path);
        if !std::path::Path::new(path).exists() {
            return Err(AclError::FileNotFound(path.to_string()));
        }
    
        let config: AclConfig = serde_json::from_str(&data)
            .map_err(|e| AclError::ParseError(e.to_string()))?;
        info!("Loaded ACL config with {} roles, {} users", 
              config.roles.len(), config.users.len());
        Ok(config)
    }