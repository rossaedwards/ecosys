//! TTS Backend Implementations
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

pub mod system;

#[cfg(feature = "elevenlabs")]
pub mod elevenlabs;

#[cfg(feature = "piper")]
pub mod piper;

#[cfg(feature = "coqui")]
pub mod coqui;

pub use system::SystemTts;

/// Backend selection
#[derive(Debug, Clone)]
pub enum Backend {
    /// System native TTS
    System,
    /// ElevenLabs cloud TTS
    #[cfg(feature = "elevenlabs")]
    ElevenLabs,
    /// Piper local TTS
    #[cfg(feature = "piper")]
    Piper,
    /// Coqui TTS
    #[cfg(feature = "coqui")]
    Coqui,
}
