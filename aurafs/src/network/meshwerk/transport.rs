//! Meshwerk Transport - Physics-aware network backbone traits.

use crate::network::peer::Peer;
use crate::physics::PhysicsViolationError;
use async_trait::async_trait;
use std::time::Duration;
use thiserror::Error;

/// Transport-level errors for mesh operations.
#[derive(Debug, Error)]
pub enum MeshTransportError {
    #[error("Physics violation: {0}")]
    PhysicsViolation(#[from] PhysicsViolationError),
    #[error("Transport IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Transport failure: {0}")]
    Failure(String),
}

/// Mesh transport interface for Titan-tier backbone links.
#[async_trait]
pub trait MeshTransport: Send + Sync {
    /// Establish a connection to a peer and return latency.
    async fn connect(&self, peer: &Peer) -> Result<Duration, MeshTransportError>;

    /// Ping a peer and return measured latency.
    async fn ping(&self, peer: &Peer) -> Result<Duration, MeshTransportError>;

    /// Send a payload to a peer.
    async fn send(&self, peer: &Peer, payload: &[u8]) -> Result<(), MeshTransportError>;
}
