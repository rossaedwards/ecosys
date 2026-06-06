//! TTS Test Support Utilities
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use crate::types::{SynthesisRequest, SynthesisResponse, AudioFormat, VoiceMetadata};

/// Create a mock synthesis request
pub fn mock_request(text: &str) -> SynthesisRequest {
    SynthesisRequest {
        text: text.to_string(),
        voice_id: "test-voice".to_string(),
        format: AudioFormat::default(),
        rate: 1.0,
        pitch: 0.0,
    }
}

/// Create a mock synthesis response
pub fn mock_response(duration_ms: u64) -> SynthesisResponse {
    SynthesisResponse {
        audio: vec![0u8; 1024], // Dummy audio data
        format: AudioFormat::default(),
        duration_ms,
    }
}

/// Create mock voice metadata
pub fn mock_voice(id: &str) -> VoiceMetadata {
    VoiceMetadata {
        id: id.to_string(),
        name: format!("Test Voice {}", id),
        language: "en-US".to_string(),
        gender: crate::types::VoiceGender::Neutral,
        backend: "mock".to_string(),
    }
}

/// Test fixture for TTS testing
pub struct TtsTestFixture {
    /// Test voices
    pub voices: Vec<VoiceMetadata>,
}

impl TtsTestFixture {
    /// Create new test fixture
    pub fn new() -> Self {
        Self {
            voices: vec![
                mock_voice("voice-1"),
                mock_voice("voice-2"),
                mock_voice("voice-3"),
            ],
        }
    }
}

impl Default for TtsTestFixture {
    fn default() -> Self {
        Self::new()
    }
}
