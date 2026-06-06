//! TTS Streaming Module
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use crate::types::{AudioFormat, SynthesisRequest};
use crate::errors::Result;
use tokio::sync::mpsc;

/// Audio chunk for streaming
#[derive(Debug, Clone)]
pub struct AudioChunk {
    /// Audio data
    pub data: Vec<u8>,
    /// Sequence number
    pub sequence: u64,
    /// Is this the final chunk?
    pub is_final: bool,
}

/// Streaming TTS session
pub struct StreamingSession {
    /// Session ID
    pub id: String,
    /// Output format
    pub format: AudioFormat,
    /// Chunk sender
    sender: mpsc::Sender<AudioChunk>,
    /// Sequence counter
    sequence: u64,
}

impl StreamingSession {
    /// Create a new streaming session
    pub fn new(id: String, format: AudioFormat, buffer_size: usize) -> (Self, mpsc::Receiver<AudioChunk>) {
        let (sender, receiver) = mpsc::channel(buffer_size);
        (
            Self {
                id,
                format,
                sender,
                sequence: 0,
            },
            receiver,
        )
    }
    
    /// Send an audio chunk
    pub async fn send_chunk(&mut self, data: Vec<u8>, is_final: bool) -> Result<()> {
        let chunk = AudioChunk {
            data,
            sequence: self.sequence,
            is_final,
        };
        self.sequence += 1;
        
        self.sender.send(chunk).await
            .map_err(|e| crate::errors::TtsError::StreamingError(e.to_string()))
    }
}

/// Streaming synthesis trait
#[async_trait::async_trait]
pub trait StreamingSynth: Send + Sync {
    /// Start streaming synthesis
    async fn synthesize_streaming(
        &self,
        request: SynthesisRequest,
    ) -> Result<mpsc::Receiver<AudioChunk>>;
}
