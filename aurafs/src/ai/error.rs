//! ═══════════════════════════════════════════════════════════════════
//! 🤖 AuraFS AI - Unified Error Handling
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! Comprehensive error types for all AI modules
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use std::fmt;
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::core::{CoreError, ErrorCode, ErrorPhase, internal, client};

/// AI module result type
pub type Result<T> = std::result::Result<T, AIError>;

/// Main AI error type
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum AIError {
    /// Model-related errors
    #[error("Model error: {message}")]
    Model {
        message: String,
        model_id: Option<String>,
        code: AIErrorCode,
    },
    
    /// Inference errors
    #[error("Inference error: {message}")]
    Inference {
        message: String,
        request_id: Option<String>,
        code: AIErrorCode,
    },
    
    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        key: Option<String>,
    },
    
    /// Safety/content filter errors
    #[error("Safety check failed: {message}")]
    Safety {
        message: String,
        category: Option<String>,
        severity: SafetySeverity,
    },
    
    /// Quota/rate limit errors
    #[error("Quota exceeded: {message}")]
    Quota {
        message: String,
        soul_id: Option<String>,
        limit_type: QuotaType,
    },
    
    /// Timeout errors
    #[error("Operation timeout: {message} (timeout: {timeout_ms}ms)")]
    Timeout {
        message: String,
        timeout_ms: u64,
        operation: String,
    },
    
    /// Resource errors
    #[error("Resource error: {message}")]
    Resource {
        message: String,
        resource_type: ResourceType,
    },
    
    /// Validation errors
    #[error("Validation error: {message}")]
    Validation {
        message: String,
        field: Option<String>,
    },
    
    /// Quantum processing errors
    #[error("Quantum processing error: {message}")]
    Quantum {
        message: String,
        qubit_count: Option<usize>,
        fidelity: Option<f64>,
    },
    
    /// Network/API errors
    #[error("Network error: {message}")]
    Network {
        message: String,
        endpoint: Option<String>,
        status_code: Option<u16>,
    },
    
    /// Wrapped core error
    #[error("Core error: {0}")]
    Core(#[from] CoreError),
    
    /// Generic error
    #[error("AI error: {message}")]
    Other {
        message: String,
        code: AIErrorCode,
    },
}

/// AI-specific error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AIErrorCode {
    ModelNotFound,
    ModelLoadFailed,
    ModelInferenceFailed,
    InvalidModelConfig,
    InferenceTimeout,
    InvalidInput,
    SafetyCheckFailed,
    QuotaExceeded,
    RateLimitExceeded,
    ResourceExhausted,
    QuantumDecoherence,
    QuantumGateError,
    NetworkError,
    Unknown,
}

/// Safety severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Quota types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuotaType {
    RequestRate,
    TokenLimit,
    ComputeTime,
    Storage,
    Custom(String),
}

/// Resource types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Memory,
    GPU,
    CPU,
    Storage,
    Network,
    QuantumProcessor,
}

impl AIError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AIError::Timeout { .. }
                | AIError::Network { .. }
                | AIError::Resource { .. }
                | AIError::Inference { code: AIErrorCode::InferenceTimeout, .. }
        )
    }
    
    /// Check if error is fatal
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            AIError::Model { code: AIErrorCode::ModelLoadFailed, .. }
                | AIError::Config { .. }
                | AIError::Safety { severity: SafetySeverity::Critical, .. }
        )
    }
    
    /// Get error code
    pub fn error_code(&self) -> AIErrorCode {
        match self {
            AIError::Model { code, .. } => *code,
            AIError::Inference { code, .. } => *code,
            AIError::Safety { .. } => AIErrorCode::SafetyCheckFailed,
            AIError::Quota { .. } => AIErrorCode::QuotaExceeded,
            AIError::Timeout { .. } => AIErrorCode::InferenceTimeout,
            AIError::Quantum { .. } => AIErrorCode::QuantumDecoherence,
            AIError::Network { .. } => AIErrorCode::NetworkError,
            AIError::Other { code, .. } => *code,
            _ => AIErrorCode::Unknown,
        }
    }
    
    /// Convert to core error
    pub fn to_core_error(self, phase: ErrorPhase) -> CoreError {
        let code = match self.error_code() {
            AIErrorCode::InvalidInput => ErrorCode::InvalidInput,
            AIErrorCode::QuotaExceeded => ErrorCode::ResourceExhausted,
            AIErrorCode::InferenceTimeout => ErrorCode::Timeout,
            AIErrorCode::SafetyCheckFailed => ErrorCode::InvalidInput,
            _ => ErrorCode::Unknown,
        };
        
        let class = match self {
            AIError::Validation { .. } | AIError::Safety { .. } => crate::core::ErrorClass::Client,
            AIError::Timeout { .. } | AIError::Network { .. } => crate::core::ErrorClass::Transient,
            AIError::Model { .. } | AIError::Inference { .. } => crate::core::ErrorClass::Internal,
            _ => crate::core::ErrorClass::Internal,
        };
        
        internal(
            crate::core::AuraFSError::Other {
                message: self.to_string(),
            },
            phase,
        )
    }
}

impl fmt::Display for AIErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIErrorCode::ModelNotFound => write!(f, "MODEL_NOT_FOUND"),
            AIErrorCode::ModelLoadFailed => write!(f, "MODEL_LOAD_FAILED"),
            AIErrorCode::ModelInferenceFailed => write!(f, "MODEL_INFERENCE_FAILED"),
            AIErrorCode::InvalidModelConfig => write!(f, "INVALID_MODEL_CONFIG"),
            AIErrorCode::InferenceTimeout => write!(f, "INFERENCE_TIMEOUT"),
            AIErrorCode::InvalidInput => write!(f, "INVALID_INPUT"),
            AIErrorCode::SafetyCheckFailed => write!(f, "SAFETY_CHECK_FAILED"),
            AIErrorCode::QuotaExceeded => write!(f, "QUOTA_EXCEEDED"),
            AIErrorCode::RateLimitExceeded => write!(f, "RATE_LIMIT_EXCEEDED"),
            AIErrorCode::ResourceExhausted => write!(f, "RESOURCE_EXHAUSTED"),
            AIErrorCode::QuantumDecoherence => write!(f, "QUANTUM_DECOHERENCE"),
            AIErrorCode::QuantumGateError => write!(f, "QUANTUM_GATE_ERROR"),
            AIErrorCode::NetworkError => write!(f, "NETWORK_ERROR"),
            AIErrorCode::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// Helper functions for creating errors
impl AIError {
    pub fn model_not_found(model_id: impl Into<String>) -> Self {
        AIError::Model {
            message: format!("Model not found: {}", model_id.into()),
            model_id: Some(model_id.into()),
            code: AIErrorCode::ModelNotFound,
        }
    }
    
    pub fn inference_timeout(timeout_ms: u64, operation: impl Into<String>) -> Self {
        AIError::Timeout {
            message: format!("Inference operation timed out"),
            timeout_ms,
            operation: operation.into(),
        }
    }
    
    pub fn safety_check_failed(message: impl Into<String>, category: Option<String>, severity: SafetySeverity) -> Self {
        AIError::Safety {
            message: message.into(),
            category,
            severity,
        }
    }
    
    pub fn quota_exceeded(soul_id: Option<String>, limit_type: QuotaType) -> Self {
        AIError::Quota {
            message: format!("Quota limit exceeded"),
            soul_id,
            limit_type,
        }
    }
    
    pub fn invalid_input(message: impl Into<String>, field: Option<String>) -> Self {
        AIError::Validation {
            message: message.into(),
            field,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_retryable() {
        let err = AIError::inference_timeout(5000, "test");
        assert!(err.is_retryable());
        
        let err = AIError::model_not_found("test-model");
        assert!(!err.is_retryable());
    }
    
    #[test]
    fn test_error_fatal() {
        let err = AIError::Model {
            message: "Failed to load".to_string(),
            model_id: None,
            code: AIErrorCode::ModelLoadFailed,
        };
        assert!(err.is_fatal());
    }
}

