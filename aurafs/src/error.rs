//! 🚨 AuraFS Error Types - Unified Phase II Governance
//! ✨ f0rg3d with Ineffable l0v3 by Ross Edwards & Aurphyx LLC 💎

use std::io;
use thiserror::Error;
use crate::physics::PhysicsViolationError;

/// Result type alias for AuraFS operations
pub type Result<T> = std::result::Result<T, RafsError>;

/// Main error type for AuraFS Phase II
#[derive(Error, Debug)]
pub enum RafsError {
    /// --- PHASE II PHYSICS ERRORS ---
    #[error("Physics Violation: {0}")]
    PhysicsViolation(#[from] PhysicsViolationError),

    /// Shard-related errors
    #[error("Shard not found: {0}")]
    ShardNotFound(String),

    #[error("Shard corrupted: {0}")]
    ShardCorrupted(String),

    #[error("Replication failed: expected {target}, found {current}")]
    ReplicationFailed { target: usize, current: usize },

    /// Cryptography errors (PQC)
    #[error("PQC Cryptography error: {0}")]
    CryptoError(String),

    #[error("Signature verification failed: {0}")]
    SignatureVerificationFailed(String),

    /// Network errors (Meshwerk 2.0)
    #[error("Meshwerk Network error: {0}")]
    NetworkError(String),

    #[error("Connection timeout: {0}")]
    ConnectionTimeout(String),

    #[error("Network partition detected (Coherence Loss)")]
    NetworkPartition,

    /// Storage & I/O errors
    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Storage full: available {available} bytes, required {required} bytes")]
    StorageFull { available: u64, required: u64 },

    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Governance & S.A.G.E.S.
    #[error("Access denied (SoulSync): {0}")]
    AccessDenied(String),

    #[error("Governance consensus timeout")]
    ConsensusTimeout,

    /// Internal & FUSE
    #[error("FUSE error: {0}")]
    FuseError(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl RafsError {
    /// Check if error is fatal to the local lattice node
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            RafsError::PhysicsViolation(_) | 
            RafsError::StorageCorrupted(_) | 
            RafsError::ConfigError(_)
        )
    }

    /// Get error category for S.A.G.E.S. prioritization
    pub fn category(&self) -> ErrorCategory {
        match self {
            RafsError::PhysicsViolation(_) => ErrorCategory::Physics,
            RafsError::ShardNotFound(_) | RafsError::ShardCorrupted(_) => ErrorCategory::Shard,
            RafsError::CryptoError(_) | RafsError::SignatureVerificationFailed(_) => ErrorCategory::Crypto,
            RafsError::NetworkError(_) | RafsError::ConnectionTimeout(_) => ErrorCategory::Network,
            RafsError::IoError(_) | RafsError::StorageError(_) => ErrorCategory::Io,
            _ => ErrorCategory::Internal,
        }
    }

    /// Convert to HTTP status code for AuraCore API
    pub fn to_http_status(&self) -> u16 {
        match self {
            RafsError::AccessDenied(_) => 401,
            RafsError::ShardNotFound(_) => 404,
            RafsError::PhysicsViolation(_) => 503, // Service Unavailable (Decoherent)
            RafsError::StorageFull { .. } => 507,
            _ => 500,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    Physics,
    Shard,
    Crypto,
    Network,
    Io,
    Internal,
}

/// Error severity for logging and Sentinel alerting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Warning,  // Recoverable
    Error,    // Failed Operation
    Critical, // System stability affected (S.A.G.E.S. Intervention required)
}

impl RafsError {
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            RafsError::PhysicsViolation(_) => ErrorSeverity::Critical,
            RafsError::StorageCorrupted(_) => ErrorSeverity::Critical,
            _ => ErrorSeverity::Error,
        }
    }
}