//! afs-tts
//! No samples. Only weapons-grade scaffolding.

pub mod core;
pub mod errors;
pub mod types;
pub mod util;

pub mod streaming;
pub mod backends;
pub mod voicepacks;
pub mod cache;
pub mod api;
pub mod cli;
pub mod integrations;
pub mod testsupport;

pub use core::{Tts, TtsEngine, TtsVoice};
