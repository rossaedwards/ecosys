//! TTS REST API
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use crate::core::TtsEngine;
use crate::types::{SynthesisRequest, AudioFormat};
use crate::errors::TtsError;
use std::sync::Arc;

/// API configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Listen address
    pub host: String,
    /// Port
    pub port: u16,
    /// Enable CORS
    pub cors_enabled: bool,
    /// API key requirement
    pub require_api_key: bool,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            cors_enabled: true,
            require_api_key: false,
        }
    }
}

/// TTS API server
pub struct TtsApi {
    /// Engine reference
    engine: Arc<TtsEngine>,
    /// Configuration
    config: ApiConfig,
}

impl TtsApi {
    /// Create new API server
    pub fn new(engine: Arc<TtsEngine>, config: ApiConfig) -> Self {
        Self { engine, config }
    }
    
    /// Start the API server
    pub async fn start(&self) -> Result<(), TtsError> {
        tracing::info!("Starting TTS API on {}:{}", self.config.host, self.config.port);
        // Would use axum/warp here
        Ok(())
    }
}

/// Synthesis request body
#[derive(Debug, serde::Deserialize)]
pub struct SynthesizeBody {
    /// Text to synthesize
    pub text: String,
    /// Voice ID (optional)
    pub voice: Option<String>,
    /// Format (optional)
    pub format: Option<String>,
}
