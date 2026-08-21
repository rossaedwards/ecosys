//! AuraFS Governance API Server Binary
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx

use std::sync::Arc;
use clap::Parser;
use env_logger::Env;
use log::{info, error};

use afs_governance::{
    GovernanceSystem,
    consensus_integration::MockLedgerClient,
    api::start_server,
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logger with environment filter defaulting to CLI value
    env_logger::Builder::from_env(Env::default().default_filter_or(&args.log_level))
        .init();

    info!("🚀 Starting AuraFS Governance Server...");
    info!("Node ID: {}", args.node_id);
    info!("Listening on port: {}", args.port);

    let validators: Vec<String> = args.validators
        .split(',')
        .map(str::trim)
        .map(String::from)
        .collect();

    info!("Validators: {:?}", validators);

    // Initialize governance system with mock ledger client (replace with real in prod)
    let ledger = Arc::new(MockLedgerClient::new());
    let governance = Arc::new(GovernanceSystem::new(
        args.node_id,
        validators,
        ledger,
        None,  // Optional config or hooks
    ));

    info!("✅ Governance system initialized");

    // Kick off governance HTTP API server
    if let Err(e) = start_server(governance, args.port).await {
        error!("Governance API server crashed: {}", e);
        return Err(Box::new(e));
    }

    Ok(())
}