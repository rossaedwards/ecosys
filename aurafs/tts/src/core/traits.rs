use async_trait::async_trait;
use crate::types::{SynthesisRequest, SynthesisResponse, VoiceMetadata};
use crate::errors::Result;

/// Core TTS trait for all backends
#[async_trait]
pub trait Tts: Send + Sync {
    /// Synthesize text to audio
    async fn synthesize(&self, request: SynthesisRequest) -> Result<SynthesisResponse>;
    
    /// List available voices
    async fn list_voices(&self) -> Result<Vec<VoiceMetadata>>;
    
    /// Get backend name
    fn name(&self) -> &str;
    
    /// Simple speak method for backwards compatibility
    async fn speak(&self, text: &str) -> Result<()> {
        let request = SynthesisRequest {
            text: text.to_string(),
            voice_id: "default".to_string(),
            format: Default::default(),
            rate: 1.0,
            pitch: 0.0,
        };
        let _ = self.synthesize(request).await?;
        Ok(())
    }
}
