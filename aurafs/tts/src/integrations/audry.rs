//! Audry AI Integration
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use crate::core::TtsEngine;
use crate::types::SynthesisRequest;
use crate::errors::Result;
use std::sync::Arc;

/// Audry TTS integration for AI responses
pub struct AudryIntegration {
    /// TTS engine
    engine: Arc<TtsEngine>,
    /// Default voice for Audry
    default_voice: String,
}

impl AudryIntegration {
    /// Create new Audry integration
    pub fn new(engine: Arc<TtsEngine>) -> Self {
        Self {
            engine,
            default_voice: "audry-voice-1".to_string(),
        }
    }
    
    /// Speak Audry's response
    pub async fn speak(&self, text: &str) -> Result<Vec<u8>> {
        let request = SynthesisRequest {
            text: text.to_string(),
            voice_id: self.default_voice.clone(),
            format: Default::default(),
            rate: 1.0,
            pitch: 0.0,
        };
        
        let response = self.engine.synthesize(request).await?;
        Ok(response.audio)
    }
    
    /// Set Audry's voice
    pub fn set_voice(&mut self, voice_id: String) {
        self.default_voice = voice_id;
    }
}
