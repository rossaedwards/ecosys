//! System Native TTS Backend
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use crate::core::traits::Tts;
use crate::types::{SynthesisRequest, SynthesisResponse, VoiceMetadata, AudioFormat};
use crate::errors::{Result, TtsError};
use async_trait::async_trait;

/// System-native TTS implementation
pub struct SystemTts {
    /// Available voices
    voices: Vec<VoiceMetadata>,
}

impl SystemTts {
    /// Create new system TTS instance
    pub fn new() -> Result<Self> {
        Ok(Self {
            voices: vec![
                VoiceMetadata {
                    id: "system-default".to_string(),
                    name: "System Default".to_string(),
                    language: "en-US".to_string(),
                    gender: crate::types::VoiceGender::Neutral,
                    backend: "system".to_string(),
                },
            ],
        })
    }
}

#[async_trait]
impl Tts for SystemTts {
    async fn synthesize(&self, request: SynthesisRequest) -> Result<SynthesisResponse> {
        // Placeholder - would use platform-specific TTS APIs
        // Windows: SAPI, macOS: NSSpeechSynthesizer, Linux: espeak
        Ok(SynthesisResponse {
            audio: Vec::new(),
            format: request.format,
            duration_ms: 0,
        })
    }
    
    async fn list_voices(&self) -> Result<Vec<VoiceMetadata>> {
        Ok(self.voices.clone())
    }
    
    fn name(&self) -> &str {
        "system"
    }
}

impl Default for SystemTts {
    fn default() -> Self {
        Self::new().expect("Failed to initialize system TTS")
    }
}
