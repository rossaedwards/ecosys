//! Error types for VAP load/parse/context operations.

use thiserror::Error;

pub type VapResult<T> = Result<T, VapError>;

#[derive(Debug, Error)]
pub enum VapError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unsupported VASP_VERSION '{0}' (expected 3.69)")]
    VersionMismatch(String),

    #[error("missing required field: {0}")]
    MissingField(String),

    #[error("unknown context tag: {0}")]
    UnknownContext(String),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("{0}")]
    Message(String),
}
