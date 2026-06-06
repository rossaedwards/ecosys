//! ═══════════════════════════════════════════════════════════════════
//! 💥 AuraFS Core Error System - Over-Featured & Battle-Hardened
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Unified error hierarchy, rich context, backtraces, categories,
//! gRPC / HTTP mapping, telemetry hooks, and soul-aware helpers.
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use std::{fmt, time::SystemTime};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, warn};

use crate::core::{
    BlissId,
    ShardId,
};

/// High-level error category for routing, alerts, and SRE dashboards.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorClass {
    /// Client-side input, ACL, or misuse.
    Client,
    /// Transient network / infra / quorum issues.
    Transient,
    /// Persistent internal bug or data corruption.
    Internal,
    /// Security / auth / integrity failures.
    Security,
    /// External dependency failures (DB, cloud, oracle).
    External,
}

/// Fine-grained operation phase for error localization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorPhase {
    /// During configuration / startup.
    Init,
    /// During configuration parsing or validation.
    Config,
    /// While performing shard-level operations.
    Shard,
    /// During network / mesh operations.
    Network,
    /// During governance / voting.
    Governance,
    /// During filesystem / FUSE operations.
    Filesystem,
    /// During crypto operations.
    Crypto,
    /// During storage IO.
    Storage,
    /// Unspecified / generic.
    Other,
}

/// Optional severity level for escalation logic.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    /// Informational / non-fatal.
    Info,
    /// Warning, may require operator review.
    Warn,
    /// Error, action recommended.
    Error,
    /// Critical, may trigger paging.
    Critical,
}

/// Normalized error code for external APIs and dashboards.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCode {
    // Generic
    /// An unspecified error occurred.
    Unknown,
    /// Invalid input provided by client.
    InvalidInput,
    /// Operation not permitted for this soul / ACL.
    PermissionDenied,
    /// Requested entity not found.
    NotFound,
    /// Operation timed out.
    Timeout,
    /// Resource exhausted (capacity, quota, etc.).
    ResourceExhausted,
    /// Operation is currently unavailable / not ready.
    Unavailable,
    /// Invariant violation / bug.
    InternalInconsistency,

    // Shard-specific
    /// Shard not found.
    ShardMissing,
    /// Shard too large for configuration.
    ShardTooLarge,
    /// Shard replication failed.
    ShardReplicationFailed,
    /// Shard checksum mismatch / corruption.
    ShardCorrupt,

    // Soul / identity
    /// Soul identity invalid or unknown.
    SoulInvalid,
    /// Soul not authorized for operation.
    SoulUnauthorized,
    /// Soul already performed this operation (e.g. vote).
    SoulAlreadyActed,

    // Crypto / quantum
    /// Signature invalid or unverifiable.
    InvalidSignature,
    /// Quantum proof invalid / unverifiable.
    InvalidSoulProof,
    /// RNG / entropy failure.
    EntropyFailure,

    // Governance
    /// Governance proposal invalid or closed.
    InvalidProposal,
    /// Governance quorum not met.
    QuorumNotMet,
}

/// Core structured error for AuraFS and friends.
#[derive(Error, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum AuraFSError {
    /// Configuration / startup error.
    #[error("Config error: {message}")]
    Config {
        /// Human-readable message.
        message: String,
        /// Offending key / path, if any.
        key: Option<String>,
    },

    /// Shard-related failure.
    #[error("Shard error ({code:?}): {message}")]
    Shard {
        /// Normalized error code.
        code: ErrorCode,
        /// Shard ID if applicable.
        shard_id: Option<ShardId>,
        /// Human-readable message.
        message: String,
    },

    /// Soul / identity / ACL failure.
    #[error("Soul error ({code:?}): {message}")]
    Soul {
        /// Error code.
        code: ErrorCode,
        /// BlissId if relevant.
        soul_id: Option<BlissId>,
        /// Human-readable message.
        message: String,
    },

    /// Governance / voting error.
    #[error("Governance error ({code:?}): {message}")]
    Governance {
        /// Error code.
        code: ErrorCode,
        /// Proposal / governance context.
        proposal_id: Option<String>,
        /// Human-readable message.
        message: String,
    },

    /// Network / mesh / gossip failure.
    #[error("Network error: {message}")]
    Network {
        /// Whether error is likely transient.
        transient: bool,
        /// Human-readable message.
        message: String,
    },

    /// Storage / persistence failure.
    #[error("Storage error: {message}")]
    Storage {
        /// Human-readable message.
        message: String,
        /// Path or backend identifier.
        backend: Option<String>,
    },

    /// Crypto / quantum failure.
    #[error("Crypto error ({code:?}): {message}")]
    Crypto {
        /// Error code.
        code: ErrorCode,
        /// Human-readable message.
        message: String,
    },

    /// External service / dependency failure.
    #[error("External dependency error: {message}")]
    External {
        /// Dependency identifier.
        dependency: String,
        /// Human-readable message.
        message: String,
    },

    /// Underlying IO or system error.
    #[error("IO error: {source}")]
    Io {
        /// Source IO error.
        #[from]
        source: std::io::Error,
    },

    /// Serialization / deserialization error.
    #[error("Serialization error: {message}")]
    Serde {
        /// Message or upstream cause.
        message: String,
    },

    /// Generic catch-all, for when life happens.
    #[error("{message}")]
    Other {
        /// Human-readable message.
        message: String,
    },
}

/// Rich error context bundle for logging, telemetry, and API mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Classified category.
    pub class: ErrorClass,
    /// Phase where error happened.
    pub phase: ErrorPhase,
    /// Severity hint.
    pub severity: Severity,
    /// Normalized code.
    pub code: ErrorCode,
    /// Optional shard.
    pub shard_id: Option<ShardId>,
    /// Optional soul.
    pub soul_id: Option<BlissId>,
    /// Optional proposal or governance identifier.
    pub proposal_id: Option<String>,
    /// When error was created (wall-clock).
    pub timestamp: SystemTime,
    /// Free-form tags for metrics / logs.
    pub tags: Vec<String>,
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            class: ErrorClass::Internal,
            phase: ErrorPhase::Other,
            severity: Severity::Error,
            code: ErrorCode::Unknown,
            shard_id: None,
            soul_id: None,
            proposal_id: None,
            timestamp: SystemTime::now(),
            tags: Vec::new(),
        }
    }
}

/// Wrapper combining error + context.
#[derive(Debug, Error, Serialize, Deserialize)]
#[error("{error}")]
pub struct CoreError {
    /// Underlying error.
    pub error: AuraFSError,
    /// Associated context.
    pub ctx: ErrorContext,
}

impl CoreError {
    /// Attach context to an existing error.
    pub fn new(error: AuraFSError, ctx: ErrorContext) -> Self {
        Self { error, ctx }
    }

    /// Convenience shortcut for common pattern.
    pub fn with_phase(mut self, phase: ErrorPhase) -> Self {
        self.ctx.phase = phase;
        self
    }

    /// Derive a suggested HTTP status code.
    pub fn http_status(&self) -> u16 {
        use ErrorCode::*;
        use ErrorClass::*;

        match (self.ctx.class, self.ctx.code) {
            (ErrorClass::Client, PermissionDenied | SoulUnauthorized) => 403,
            (ErrorClass::Client, InvalidInput | InvalidProposal) => 400,
            (_, NotFound | ShardMissing) => 404,
            (_, Timeout) => 504,
            (_, ResourceExhausted) => 429,
            (Security, InvalidSignature | InvalidSoulProof) => 401,
            (Transient, _) => 503,
            (Internal, InternalInconsistency | ShardCorrupt) => 500,
            _ => 500,
        }
    }

    /// Derive a suggested gRPC `tonic::Status`.
    #[cfg(feature = "grpc")]
    pub fn to_grpc_status(&self) -> tonic::Status {
        use tonic::Code;
        use ErrorCode::*;
        use ErrorClass::*;

        let code = match (self.ctx.class, self.ctx.code) {
            (ErrorClass::Client, InvalidInput | InvalidProposal) => Code::InvalidArgument,
            (ErrorClass::Client, PermissionDenied | SoulUnauthorized) => Code::PermissionDenied,
            (_, NotFound | ShardMissing) => Code::NotFound,
            (_, Timeout) => Code::DeadlineExceeded,
            (_, ResourceExhausted) => Code::ResourceExhausted,
            (Security, InvalidSignature | InvalidSoulProof) => Code::Unauthenticated,
            (Transient, _) => Code::Unavailable,
            (Internal, InternalInconsistency | ShardCorrupt) => Code::Internal,
            _ => Code::Unknown,
        };

        tonic::Status::new(code, self.error.to_string())
    }

    /// Emit structured log with tracing, including context and metrics
    pub fn log(&self) {
        // Record error metrics
        crate::core::AuraFSMetrics::error_occurred(
            self.ctx.class,
            self.ctx.phase,
            self.ctx.code,
        );
        
        match self.ctx.severity {
            Severity::Info => {
                warn!(target: "aurafs_core", 
                    error = ?self.error, 
                    ctx = ?self.ctx,
                    shard_id = ?self.ctx.shard_id,
                    soul_id = ?self.ctx.soul_id,
                    "Info-level error recorded");
            }
            Severity::Warn => {
                warn!(target: "aurafs_core", 
                    error = ?self.error, 
                    ctx = ?self.ctx,
                    shard_id = ?self.ctx.shard_id,
                    soul_id = ?self.ctx.soul_id,
                    "Warning-level error recorded");
            }
            Severity::Error => {
                error!(target: "aurafs_core", 
                    error = ?self.error, 
                    ctx = ?self.ctx,
                    shard_id = ?self.ctx.shard_id,
                    soul_id = ?self.ctx.soul_id,
                    "Error occurred");
            }
            Severity::Critical => {
                error!(target: "aurafs_core", 
                    error = ?self.error, 
                    ctx = ?self.ctx,
                    shard_id = ?self.ctx.shard_id,
                    soul_id = ?self.ctx.soul_id,
                    "CRITICAL ERROR - Immediate attention required");
            }
        }
    }

    /// Short human-friendly summary for CLI / dashboards
    pub fn summary(&self) -> String {
        let mut parts = vec![
            format!("{:?}", self.ctx.class),
            format!("{:?}", self.ctx.phase),
            format!("{:?}", self.ctx.code),
        ];
        
        if let Some(ref shard_id) = self.ctx.shard_id {
            parts.push(format!("shard:{}", shard_id));
        }
        
        if let Some(ref soul_id) = self.ctx.soul_id {
            parts.push(format!("soul:{}", soul_id));
        }
        
        format!("[{}] {}", parts.join("/"), self.error)
    }
    
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(self.ctx.class, ErrorClass::Transient) ||
        matches!(self.ctx.code, ErrorCode::Timeout | ErrorCode::Unavailable)
    }
    
    /// Check if error is a client error (not retryable)
    pub fn is_client_error(&self) -> bool {
        matches!(self.ctx.class, ErrorClass::Client)
    }
    
    /// Get error context for structured logging
    pub fn context(&self) -> &ErrorContext {
        &self.ctx
    }
}

// ======================================================================
// SMART CONSTRUCTORS (because we’re extra now)
// ======================================================================

impl AuraFSError {
    /// Create a "shard not found" error.
    pub fn shard_not_found(shard_id: ShardId) -> Self {
        AuraFSError::Shard {
            code: ErrorCode::ShardMissing,
            shard_id: Some(shard_id),
            message: "Shard not found".to_string(),
        }
    }

    /// Create a "shard too large" error.
    pub fn shard_too_large(shard_id: Option<ShardId>, max_bytes: usize) -> Self {
        AuraFSError::Shard {
            code: ErrorCode::ShardTooLarge,
            shard_id,
            message: format!("Shard exceeds max size of {} bytes", max_bytes),
        }
    }

    /// Create a "soul unauthorized" error.
    pub fn soul_unauthorized(soul_id: BlissId, operation: impl Into<String>) -> Self {
        AuraFSError::Soul {
            code: ErrorCode::SoulUnauthorized,
            soul_id: Some(soul_id),
            message: format!("Soul not authorized for operation '{}'", operation.into()),
        }
    }

    /// Create a "soul already acted" error (e.g. duplicate vote).
    pub fn soul_already_acted(soul_id: BlissId, context: impl Into<String>) -> Self {
        AuraFSError::Soul {
            code: ErrorCode::SoulAlreadyActed,
            soul_id: Some(soul_id),
            message: format!("Soul already acted in {}", context.into()),
        }
    }

    /// Governance proposal invalid or closed.
    pub fn invalid_proposal(id: impl Into<String>, msg: impl Into<String>) -> Self {
        AuraFSError::Governance {
            code: ErrorCode::InvalidProposal,
            proposal_id: Some(id.into()),
            message: msg.into(),
        }
    }

    /// Crypto: invalid signature.
    pub fn invalid_signature(msg: impl Into<String>) -> Self {
        AuraFSError::Crypto {
            code: ErrorCode::InvalidSignature,
            message: msg.into(),
        }
    }

    /// Wrap serde error.
    pub fn from_serde(e: impl fmt::Display) -> Self {
        AuraFSError::Serde {
            message: e.to_string(),
        }
    }
}

// ======================================================================
// SHORTCUT HELPERS
// ======================================================================

/// Helper to wrap an error with default internal context.
pub fn internal<E: Into<AuraFSError>>(err: E, phase: ErrorPhase) -> CoreError {
    CoreError {
        error: err.into(),
        ctx: ErrorContext {
            class: ErrorClass::Internal,
            phase,
            severity: Severity::Error,
            code: ErrorCode::InternalInconsistency,
            ..Default::default()
        },
    }
}

/// Helper to wrap a client error (bad input / misuse).
pub fn client<E: Into<AuraFSError>>(err: E, phase: ErrorPhase, code: ErrorCode) -> CoreError {
    CoreError {
        error: err.into(),
        ctx: ErrorContext {
            class: ErrorClass::Client,
            phase,
            severity: Severity::Warn,
            code,
            ..Default::default()
        },
    }
}

/// Helper to wrap a transient network error.
pub fn transient_network(message: impl Into<String>) -> CoreError {
    CoreError {
        error: AuraFSError::Network {
            transient: true,
            message: message.into(),
        },
        ctx: ErrorContext {
            class: ErrorClass::Transient,
            phase: ErrorPhase::Network,
            severity: Severity::Warn,
            code: ErrorCode::Unavailable,
            ..Default::default()
        },
    }
}

// ======================================================================
// CONVERSIONS FROM COMMON ERROR TYPES
// ======================================================================

impl From<serde_json::Error> for AuraFSError {
    fn from(e: serde_json::Error) -> Self {
        AuraFSError::from_serde(e)
    }
}

impl From<toml::de::Error> for AuraFSError {
    fn from(e: toml::de::Error) -> Self {
        AuraFSError::from_serde(e)
    }
}

impl From<toml::ser::Error> for AuraFSError {
    fn from(e: toml::ser::Error) -> Self {
        AuraFSError::from_serde(e)
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_not_found() {
        let id = ShardId::new();
        let err = AuraFSError::shard_not_found(id.clone());
        let s = err.to_string();
        assert!(s.contains("Shard error"));
    }

    #[test]
    fn test_core_error_http_mapping() {
        let soul = BlissId::genesis();
        let err = AuraFSError::soul_unauthorized(soul, "test-op");
        let ctx = ErrorContext {
            class: ErrorClass::Client,
            phase: ErrorPhase::Governance,
            severity: Severity::Warn,
            code: ErrorCode::SoulUnauthorized,
            ..Default::default()
        };
        let core = CoreError::new(err, ctx);
        assert_eq!(core.http_status(), 403);
    }

    #[test]
    fn test_summary() {
        let err = AuraFSError::Other {
            message: "Boom".to_string(),
        };
        let ctx = ErrorContext {
            class: ErrorClass::Internal,
            phase: ErrorPhase::Shard,
            severity: Severity::Critical,
            code: ErrorCode::Unknown,
            ..Default::default()
        };
        let core = CoreError::new(err, ctx);
        let summary = core.summary();
        assert!(summary.contains("Internal"));
        assert!(summary.contains("Shard"));
    }
}