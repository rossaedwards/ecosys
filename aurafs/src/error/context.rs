//! ═══════════════════════════════════════════════════════════════════
//! 🔗 Enhanced Error Context with Correlation IDs
//! ✨ f0rg3d with Ineffable l0v3 by Ross Edwards & Aurphyx LLC 💎
//!
//! Provides rich error context with correlation IDs for distributed tracing.
//! Integrates with OpenTelemetry for full observability support.
//! ═══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::error::{RafsError, ErrorCategory};
use uuid::Uuid;

/// Error context with correlation ID and metadata
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub shard_id: Option<String>,
    pub node_id: Option<String>,
    pub correlation_id: String,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

impl ErrorContext {
    /// Create new error context
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            shard_id: None,
            node_id: None,
            correlation_id: Uuid::new_v4().to_string(),
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    /// Add shard ID to context
    pub fn with_shard_id(mut self, shard_id: impl Into<String>) -> Self {
        self.shard_id = Some(shard_id.into());
        self
    }

    /// Add node ID to context
    pub fn with_node_id(mut self, node_id: impl Into<String>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }

    /// Add correlation ID
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = correlation_id.into();
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Generate correlation ID
    pub fn generate_correlation_id() -> String {
        Uuid::new_v4().to_string()
    }
}

/// Enhanced error with context
#[derive(Debug, Clone)]
pub struct ContextualError {
    pub error: RafsError,
    pub context: ErrorContext,
    pub retryable: bool,
}

impl ContextualError {
    /// Create new contextual error
    pub fn new(error: RafsError, context: ErrorContext) -> Self {
        let retryable = error.is_retryable();
        Self {
            error,
            context,
            retryable,
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        self.retryable
    }

    /// Get error category
    pub fn category(&self) -> ErrorCategory {
        self.error.category()
    }
}

impl std::fmt::Display for ContextualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}: {} (correlation_id: {})",
            self.context.operation,
            self.error,
            self.context.correlation_id,
            self.error
        )
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Log error with context
pub fn log_error_with_context(error: &RafsError, context: &ErrorContext) {
    tracing::error!(
        error = %error,
        operation = %context.operation,
        correlation_id = %context.correlation_id,
        shard_id = ?context.shard_id,
        node_id = ?context.node_id,
        metadata = ?context.metadata,
        "Operation failed"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("read_shard")
            .with_shard_id("shard-123")
            .with_node_id("node-456")
            .with_metadata("key", "value");

        assert_eq!(context.operation, "read_shard");
        assert_eq!(context.shard_id, Some("shard-123".to_string()));
        assert_eq!(context.node_id, Some("node-456".to_string()));
        assert_eq!(context.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_contextual_error() {
        let error = RafsError::ShardNotFound("test".to_string());
        let context = ErrorContext::new("test_operation");
        let contextual = ContextualError::new(error, context);

        assert!(!contextual.is_retryable());
    }
}

