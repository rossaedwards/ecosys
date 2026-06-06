use crate::types::{SynthesisRequest, SynthesisResponse, VoiceMetadata};
use crate::errors::{Result, TtsError};
use crate::core::traits::Tts;
use std::sync::Arc;
use std::collections::HashMap;

/// TTS Voice variants
#[derive(Clone, Debug)]
pub enum TtsVoice {
    Default,
    Ross,
    Audry,
    QuantumAura,
    Custom(String),
}

impl TtsVoice {
    /// Get voice ID
    pub fn id(&self) -> &str {
        match self {
            TtsVoice::Default => "default",
            TtsVoice::Ross => "ross",
            TtsVoice::Audry => "audry",
            TtsVoice::QuantumAura => "quantum-aura",
            TtsVoice::Custom(id) => id,
        }
    }
}

/// Main TTS engine that orchestrates backends
pub struct TtsEngine {
    /// Registered backends
    backends: HashMap<String, Arc<dyn Tts>>,
    /// Default backend name
    default_backend: String,
    /// Default voice
    default_voice: TtsVoice,
}

impl TtsEngine {
    /// Create new TTS engine
    pub fn new() -> Self {
        Self {
            backends: HashMap::new(),
            default_backend: "system".to_string(),
            default_voice: TtsVoice::Default,
        }
    }
    
    /// Register a backend
    pub fn register_backend(&mut self, name: &str, backend: Arc<dyn Tts>) {
        self.backends.insert(name.to_string(), backend);
    }
    
    /// Set default backend
    pub fn set_default_backend(&mut self, name: &str) {
        self.default_backend = name.to_string();
    }
    
    /// Set default voice
    pub fn set_default_voice(&mut self, voice: TtsVoice) {
        self.default_voice = voice;
    }
    
    /// Synthesize text using default or specified backend
    pub async fn synthesize(&self, mut request: SynthesisRequest) -> Result<SynthesisResponse> {
        // Use default voice if not specified
        if request.voice_id.is_empty() || request.voice_id == "default" {
            request.voice_id = self.default_voice.id().to_string();
        }
        
        let backend = self.backends.get(&self.default_backend)
            .ok_or_else(|| TtsError::BackendUnavailable(self.default_backend.clone()))?;
        
        backend.synthesize(request).await
    }
    
    /// List all available voices across all backends
    pub async fn list_all_voices(&self) -> Result<Vec<VoiceMetadata>> {
        let mut all_voices = Vec::new();
        
        for backend in self.backends.values() {
            let voices = backend.list_voices().await?;
            all_voices.extend(voices);
        }
        
        Ok(all_voices)
    }
    
    /// Simple speak method
    pub async fn speak(&self, text: &str) -> Result<()> {
        let backend = self.backends.get(&self.default_backend)
            .ok_or_else(|| TtsError::BackendUnavailable(self.default_backend.clone()))?;
        
        backend.speak(text).await
    }
}

impl Default for TtsEngine {
    fn default() -> Self {
        Self::new()
    }
}
