//! TTS Type Definitions
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use serde::{Deserialize, Serialize};

/// Audio format specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFormat {
    /// Sample rate in Hz
    pub sample_rate: u32,
    /// Number of channels (1 = mono, 2 = stereo)
    pub channels: u8,
    /// Bits per sample
    pub bits_per_sample: u8,
    /// Encoding format
    pub encoding: AudioEncoding,
}

/// Audio encoding formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioEncoding {
    /// Raw PCM
    Pcm,
    /// MP3
    Mp3,
    /// Opus
    Opus,
    /// WAV
    Wav,
    /// FLAC
    Flac,
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self {
            sample_rate: 22050,
            channels: 1,
            bits_per_sample: 16,
            encoding: AudioEncoding::Pcm,
        }
    }
}

/// Voice metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceMetadata {
    /// Voice identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Language code (e.g., "en-US")
    pub language: String,
    /// Voice gender
    pub gender: VoiceGender,
    /// Backend that provides this voice
    pub backend: String,
}

/// Voice gender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoiceGender {
    Male,
    Female,
    Neutral,
}

/// Synthesis request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisRequest {
    /// Text to synthesize
    pub text: String,
    /// Voice ID
    pub voice_id: String,
    /// Output format
    pub format: AudioFormat,
    /// Speaking rate (0.5 - 2.0)
    pub rate: f32,
    /// Pitch adjustment (-1.0 to 1.0)
    pub pitch: f32,
}

/// Synthesis response
#[derive(Debug, Clone)]
pub struct SynthesisResponse {
    /// Audio data
    pub audio: Vec<u8>,
    /// Format of the audio
    pub format: AudioFormat,
    /// Duration in milliseconds
    pub duration_ms: u64,
}
