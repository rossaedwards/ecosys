//! TTS (Text-to-Speech) module for AuraFS
//! 
//! Provides text-to-speech functionality with support for multiple voices
//! and cross-platform TTS engines.

pub mod tts_engine;

pub use tts_engine::{Tts, TtsEngine, TtsVoice};