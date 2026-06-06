//! TTS Error Types
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use thiserror::Error;

/// TTS-specific errors
#[derive(Error, Debug)]
pub enum TtsError {
    /// Voice not found
    #[error("Voice not found: {0}")]
    VoiceNotFound(String),
    
    /// Audio encoding error
    #[error("Audio encoding error: {0}")]
    EncodingError(String),
    
    /// Backend unavailable
    #[error("TTS backend unavailable: {0}")]
    BackendUnavailable(String),
    
    /// Streaming error
    #[error("Streaming error: {0}")]
    StreamingError(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for TTS operations
pub type Result<T> = std::result::Result<T, TtsError>;
