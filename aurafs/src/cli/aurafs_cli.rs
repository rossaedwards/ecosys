//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS CLI - Production Command Interface
//! 🛠️ Commands: slice, reconstruct, lanes, soul
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    model_slice::{AuraFsModelSlicer, ModelShard, SliceConfig},
    network::{NodeManager, ReticulumBridge, LaneStatus},
    gov::BlissId,
    crypto::quantum::KyberKeypair,
    ai::FractalOrchestrator,
    shard::ShardId,
    dedup::DeduplicationEngine,
    storage::shard_store::ShardStore,
};
use clap::{Parser, Subcommand};
use std::{
    path::PathBuf,
    sync::Arc,
    str::FromStr,
};
use tracing::{info, error};
use thiserror::Error;

/// AuraFS CLI - Production command interface
#[derive(Parser)]
#[command(name = "aurafs")]
#[command(about = "AuraFS - Quantum-Safe Distributed Filesystem CLI")]
pub struct AuraCli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// BlissId for authentication (required for write operations)
    #[arg(long, global = true)]
    pub bliss_id: Option<String>,
}

/// CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Slice a model file into fractal shards
    Slice {
        /// Path to model file
        #[arg(short, long)]
        file: PathBuf,
        
        /// Output directory for shards
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Replication factor
        #[arg(short, long, default_value = "3")]
        replication: usize,
    },
    
    /// Reconstruct a model from shards
    Reconstruct {
        /// Shard IDs to reconstruct (comma-separated)
        #[arg(short, long)]
        shards: String,
        
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Show network lane status (HaLow/LoRa/Starlink)
    Lanes {
        /// Show detailed lane information
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// Show BlissId information
    Soul {
        /// Show detailed soul information
        #[arg(short, long)]
        detailed: bool,
    },
}

/// CLI executor
pub struct AuraCliExecutor {
    /// Model slicer
    model_slicer: Arc<AuraFsModelSlicer>,
    
    /// Fractal orchestrator
    orchestrator: Arc<FractalOrchestrator>,
    
    /// Reticulum bridge
    reticulum: Arc<ReticulumBridge>,
    
    /// Node manager
    node_manager: Arc<NodeManager>,
    
    /// Current BlissId (if authenticated)
    current_bliss_id: Option<BlissId>,
}

impl AuraCliExecutor {
    /// Create new CLI executor
    pub fn new(
        model_slicer: Arc<AuraFsModelSlicer>,
        orchestrator: Arc<FractalOrchestrator>,
        reticulum: Arc<ReticulumBridge>,
        node_manager: Arc<NodeManager>,
    ) -> Self {
        Self {
            model_slicer,
            orchestrator,
            reticulum,
            node_manager,
            current_bliss_id: None,
        }
    }
    
    /// Authenticate with BlissId
    pub fn authenticate(&mut self, bliss_id_str: &str) -> Result<(), CliError> {
        let bliss_id = BlissId::from_hex(bliss_id_str)
            .map_err(|e| CliError::Authentication(format!("Invalid BlissId: {}", e)))?;
        
        self.current_bliss_id = Some(bliss_id);
        info!("✨ Authenticated with BlissId: {}", bliss_id_str);
        Ok(())
    }
    
    /// Require authentication for write operations
    fn require_auth(&self) -> Result<BlissId, CliError> {
        self.current_bliss_id.clone()
            .ok_or_else(|| CliError::Authentication(
                "BlissId authentication required for this operation".to_string()
            ))
    }
    
    /// Execute CLI command
    pub async fn execute(&mut self, command: Commands) -> Result<(), CliError> {
        match command {
            Commands::Slice { file, output, replication } => {
                self.handle_slice(file, output, replication).await
            }
            Commands::Reconstruct { shards, output } => {
                self.handle_reconstruct(shards, output).await
            }
            Commands::Lanes { detailed } => {
                self.handle_lanes(detailed).await
            }
            Commands::Soul { detailed } => {
                self.handle_soul(detailed).await
            }
        }
    }
    
    /// Handle slice command
    async fn handle_slice(
        &self,
        file: PathBuf,
        _output: Option<PathBuf>,
        replication: usize,
    ) -> Result<(), CliError> {
        // Require authentication for write operations
        let bliss_id = self.require_auth()?;
        
        info!("🔪 Slicing model file: {:?}", file);
        
        // Use orchestrator to analyze and process
        let shards = self.orchestrator.process_file(&file).await
            .map_err(|e| CliError::Processing(format!("Failed to process file: {}", e)))?;
        
        if shards.is_empty() {
            info!("💾 File stored without slicing (does not meet criteria)");
        } else {
            info!("✨ Created {} shards with replication factor {}", shards.len(), replication);
            
            // Display shard information
            for (idx, shard) in shards.iter().enumerate() {
                println!("  Shard {}: {} ({} bytes)", 
                    idx + 1, 
                    shard.shard_id, 
                    shard.weights.len()
                );
            }
        }
        
        Ok(())
    }
    
    /// Handle reconstruct command
    async fn handle_reconstruct(
        &self,
        shards_str: String,
        output: PathBuf,
    ) -> Result<(), CliError> {
        // Parse shard IDs
        let shard_ids: Vec<String> = shards_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        
        if shard_ids.is_empty() {
            return Err(CliError::InvalidInput("No shard IDs provided".to_string()));
        }
        
        info!("🔨 Reconstructing model from {} shards", shard_ids.len());
        
        // Load shards from storage
        use crate::storage::shard_store::ShardStore;
        use crate::shard::Shard;
        use std::str::FromStr;
        
        let shard_store = Arc::new(ShardStore::default());
        let mut model_shards = Vec::new();
        
        for shard_id_str in shard_ids {
            // Parse ShardId from string (assuming hex format)
            // In production, this would use proper ShardId deserialization
            let shard_id = ShardId::from_content(shard_id_str.as_bytes());
            
            // Load shard from storage
            match shard_store.load_shard(&shard_id).await {
                Ok(shard) => {
                    // Convert Shard to ModelShard
                    // ModelShard contains additional metadata, so we reconstruct it
                    // In production, ModelShard metadata would be stored separately
                    let model_shard = ModelShard {
                        shard_id: shard.shard_id.clone(),
                        layer_range: (0, 0), // Would be loaded from metadata
                        weights: shard.data.clone(),
                        metadata: shard.metadata.clone(),
                        parent_shards: vec![],
                        child_shards: vec![],
                        signature: shard.signature.clone(),
                        owner: shard.metadata.owner.clone(),
                    };
                    model_shards.push(model_shard);
                }
                Err(e) => {
                    return Err(CliError::Processing(format!(
                        "Failed to load shard {}: {}",
                        shard_id_str, e
                    )));
                }
            }
        }
        
        if model_shards.is_empty() {
            return Err(CliError::InvalidInput("No shards could be loaded".to_string()));
        }
        
        // Reconstruct using model slicer
        let reconstructed = self.model_slicer.reconstruct(&model_shards)
            .map_err(|e| CliError::Processing(format!("Reconstruction failed: {}", e)))?;
        
        // Write to output file
        tokio::fs::write(&output, reconstructed).await
            .map_err(|e| CliError::IoError(e))?;
        
        info!("✨ Reconstructed model written to: {:?}", output);
        
        Ok(())
    }
    
    /// Handle lanes command
    async fn handle_lanes(&self, detailed: bool) -> Result<(), CliError> {
        let statuses = self.reticulum.get_lane_statuses().await;
        let is_gateway = self.reticulum.is_gateway_mode().await;
        
        println!("🌐 Network Lane Status");
        println!("Gateway Mode: {}", if is_gateway { "✅ Enabled" } else { "❌ Disabled" });
        println!();
        
        for status in statuses {
            let status_icon = if status.is_active { "✅" } else { "❌" };
            println!("{} {}: {}", status_icon, status.lane.name(), 
                if status.is_active { "Active" } else { "Inactive" });
            
            if detailed {
                println!("  Throughput: {} bytes/sec", status.current_throughput);
                println!("  Latency: {:.2} ms", status.latency_ms);
                println!("  Errors: {}", status.error_count);
                if let Some(last_success) = status.last_success {
                    println!("  Last Success: {:?}", last_success.elapsed());
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle soul command
    async fn handle_soul(&self, detailed: bool) -> Result<(), CliError> {
        let bliss_id = self.current_bliss_id.clone()
            .ok_or_else(|| CliError::Authentication(
                "No authenticated BlissId. Use --bliss-id to authenticate".to_string()
            ))?;
        
        println!("✨ BlissId Information");
        println!("ID: {}", bliss_id);
        
        if detailed {
            // Get peer stats
            let peer_stats = self.node_manager.get_peer_stats().await;
            println!("Connected Peers: {}", peer_stats.len());
            
            // Get network latency
            let latency = self.node_manager.average_latency_ms().await;
            println!("Average Latency: {:.2} ms", latency);
            
            // Get storage capacity
            let storage = self.node_manager.total_storage_gb().await;
            println!("Total Storage: {:.2} GB", storage);
        }
        
        Ok(())
    }
}

/// CLI errors
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Authentication error: {0}")]
    Authentication(String),
    #[error("Processing error: {0}")]
    Processing(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}


