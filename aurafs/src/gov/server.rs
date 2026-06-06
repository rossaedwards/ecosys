//! AuraFS Governance API Server Binary
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨

use std::sync::Arc;
use clap::Parser;
use env_logger::Env;
use log::{info, error, warn};

// Assuming 'afs_governance' is defined as a lib in Cargo.toml
// If running inside the aurafs crate, change this to: use crate::gov::{...};
use afs_governance::{
    GovernanceSystem,
    consensus_integration::MockLedgerClient,
    api::start_server,
    models::ProposalConfig,
};

/// CLI arguments for governance server
#[derive(Parser, Debug)]
#[command(name = "aurafs-governance-server")]
#[command(about = "AuraFS Governance API Server", long_about = None)]
struct Args {
    /// Port to listen on (default: 8080)
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Node ID for consensus (default: node1)
    #[arg(short, long, default_value = "node1")]
    node_id: String,

    /// Validator nodes (comma-separated, default: node1,node2,node3)
    #[arg(short, long, default_value = "node1,node2,node3")]
    validators: String,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Set the voting quorum percentage (default: 10.0)
    #[arg(long, default_value_t = 10.0)]
    quorum: f64,

    /// Set the approval threshold (default: 0.66 for Supermajority)
    #[arg(long, default_value_t = 0.66)]
    threshold: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logger with environment filter defaulting to CLI value
    env_logger::Builder::from_env(Env::default().default_filter_or(&args.log_level))
        .init();

    info!("🚀 Starting AuraFS Governance Server...");
    info!(":: Phase II: Lattice Physics Control ::");
    info!("Node ID: {}", args.node_id);
    info!("Listening on port: {}", args.port);

    let validators: Vec<String> = args.validators
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect();

    info!("Validators: {:?}", validators);

    // Initialize governance system with mock ledger client (replace with real in prod)
    // NOTE: Ensure MockLedgerClient is public in consensus_integration.rs
    let ledger = Arc::new(MockLedgerClient::new());
    
    // Configure Proposal Logic
    let config = ProposalConfig {
        quorum_percentage: args.quorum,
        approval_threshold: args.threshold,
        ..Default::default()
    };
    info!("Config: Quorum {:.1}%, Threshold {:.2}", config.quorum_percentage, config.approval_threshold);

    let governance = Arc::new(GovernanceSystem::new(
        args.node_id,
        validators,
        ledger,
        Some(config), 
    ));

    info!("✅ Governance System Initialized (Strict Egalitarian Mode)");

    // Kick off governance HTTP API server
    // Note: The crate name 'afs_governance' implies this is running as a separate bin 
    // that depends on the lib.
    if let Err(e) = start_server(governance, args.port).await {
        error!("Governance API server crashed: {}", e);
        return Err(e);
    }

    Ok(())
}