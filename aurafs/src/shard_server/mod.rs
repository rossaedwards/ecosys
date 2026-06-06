//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Server Module - Enterprise Orchestrator
//! 🌐 Central coordinator for embedding, healing, gossip, ACL, metrics
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

pub mod acl;
pub mod autoheal_daemon;
pub mod server;
pub mod ipfs;
pub mod ipfs_cluster;
pub mod grpc;
pub mod mesh;
pub mod mesh_gossip;
pub mod cli;

use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::signal;
use tracing::{info, warn, error};

/// Enterprise-grade shard server orchestrator
pub struct ShardServer {
    pub autoheal: Option<Arc<autoheal_daemon::AutoHealDaemon>>,
    pub mesh_gossip: Option<Arc<mesh_gossip::MeshGossip>>,
    pub acl_manager: Arc<acl::AclEnforcer>,
    pub gossip_tx: Option<mpsc::Sender<mesh_gossip::ShardGossipMessage>>,
    pub gossip_rx: Option<mpsc::Receiver<mesh_gossip::ShardGossipMessage>>,
}

impl ShardServer {
    /// Create new shard server with enterprise configuration
    pub fn new(
        autoheal: Option<Arc<autoheal_daemon::AutoHealDaemon>>,
        acl_manager: Arc<acl::AclEnforcer>,
    ) -> Self {
        let (gossip_tx, gossip_rx) = mpsc::channel(1000);
        Self {
            autoheal,
            mesh_gossip: None,
            acl_manager,
            gossip_tx: Some(gossip_tx),
            gossip_rx: Some(gossip_rx),
        }
    }

    /// Start all async services with graceful shutdown
    pub async fn start(&self) -> Result<(), ShardServerError> {
        let mut handles: Vec<JoinHandle<Result<(), ShardServerError>>> = Vec::new();

        // Start autoheal daemon if configured
        if let Some(autoheal) = &self.autoheal {
            let autoheal_clone = Arc::clone(autoheal);
            handles.push(tokio::spawn(async move {
                autoheal_clone.run().await
                    .map_err(|e| ShardServerError::ServiceError(format!("Autoheal daemon failed: {}", e)))
            }));
        }

        // Start mesh gossip if configured
        if let Some(mesh_gossip) = &self.mesh_gossip {
            let mesh_gossip_clone = Arc::clone(mesh_gossip);
            handles.push(tokio::spawn(async move {
                mesh_gossip_clone.run().await
                    .map_err(|e| ShardServerError::ServiceError(format!("Mesh gossip failed: {}", e)))
            }));
        }

        // Wait for shutdown signal or service errors
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Shutdown signal received, stopping services...");
            }
            result = async {
                for handle in handles {
                    match handle.await {
                        Ok(Ok(())) => info!("Service exited cleanly"),
                        Ok(Err(e)) => {
                            error!("Service returned error: {}", e);
                            return Err(e);
                        }
                        Err(e) => {
                            error!("Service panicked: {:?}", e);
                            return Err(ShardServerError::ServicePanic(format!("{:?}", e)));
                        }
                    }
                }
                Ok(())
            } => {
                result?;
            }
        }

        info!("Shard server shutdown complete");
        Ok(())
    }

    /// Graceful shutdown with cleanup
    pub async fn shutdown(&self) -> Result<(), ShardServerError> {
        info!("Initiating graceful shutdown...");
        // Cleanup logic here
        Ok(())
    }
}

/// Enterprise-grade shard server errors
#[derive(Debug, Error)]
pub enum ShardServerError {
    #[error("Service error: {0}")]
    ServiceError(String),
    #[error("Service panic: {0}")]
    ServicePanic(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Initialization error: {0}")]
    InitError(String),
}

pub type Result<T> = std::result::Result<T, ShardServerError>;