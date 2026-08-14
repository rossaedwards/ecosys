//! # Vinyl Vibez — Mixxx symbiont engine
//!
//! Dual-deck engine polished from FUTE-transmuted Mixxx surfaces
//! (`EngineBuffer`, `EngineMixer`, sync).
//!
//! - Scaffolds: `transmute_raw/` (libclang AST)
//! - Runtime: this module (load / play / seek / rate / crossfade / BPM sync)
//! - Spec: `docs/VINYL_VIBEZ_MIXXX_TRANSMUTE.md`
//!
//! License: **GPL-2.0-or-later** (Mixxx-compatible fence).

mod engine_buffer;
mod mixer;
mod sync;

pub use engine_buffer::{DeckSnapshot, EngineBuffer, SeekMode};
pub use mixer::{CrossfaderCurve, EngineMixer};
pub use sync::{quantize_to_beat, sync_follower_to_leader};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use vmp_viz::VapRuntime;

pub const ORIGIN_PLAN: &str = "mixxx → v01d/libclang → vmp-vinyl";
pub const LICENSE_NOTE: &str = "GPL-2.0-or-later";

#[derive(Debug, Error)]
pub enum VinylError {
    #[error("audio: {0}")]
    Audio(String),
    #[error("deck {0} empty")]
    EmptyDeck(char),
    #[error("{0}")]
    Message(String),
}

/// Full Vinyl Vibez / Mixxx-class dual-deck engine.
pub struct VinylEngine {
    pub deck_a: EngineBuffer,
    pub deck_b: EngineBuffer,
    pub mixer: EngineMixer,
    pub runtime_a: VapRuntime,
    pub runtime_b: VapRuntime,
    /// Soft master sample rate for process()
    pub output_sample_rate: u32,
}

impl Default for VinylEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl VinylEngine {
    pub fn new() -> Self {
        Self {
            deck_a: EngineBuffer::new("[Channel1]"),
            deck_b: EngineBuffer::new("[Channel2]"),
            mixer: EngineMixer::default(),
            runtime_a: VapRuntime::init(),
            runtime_b: VapRuntime::init(),
            output_sample_rate: 48000,
        }
    }

    pub fn load(&mut self, which: char, path: impl AsRef<std::path::Path>) -> Result<(), VinylError> {
        match which {
            'B' | 'b' | '2' => {
                self.deck_b
                    .load_path(path)
                    .map_err(VinylError::Audio)?;
                if let Some(vap) = &self.deck_b.vap {
                    self.runtime_b.load_vap(vap);
                }
            }
            _ => {
                self.deck_a
                    .load_path(path)
                    .map_err(VinylError::Audio)?;
                if let Some(vap) = &self.deck_a.vap {
                    self.runtime_a.load_vap(vap);
                }
            }
        }
        Ok(())
    }

    pub fn play(&mut self, which: char, on: bool) {
        match which {
            'B' | 'b' | '2' => self.deck_b.set_play(on),
            _ => self.deck_a.set_play(on),
        }
    }

    pub fn toggle(&mut self, which: char) {
        match which {
            'B' | 'b' | '2' => self.deck_b.toggle_play(),
            _ => self.deck_a.toggle_play(),
        }
    }

    pub fn stop(&mut self, which: char, to_cue: bool) {
        match which {
            'B' | 'b' | '2' => self.deck_b.stop(to_cue),
            _ => self.deck_a.stop(to_cue),
        }
    }

    pub fn seek_fraction(&mut self, which: char, frac: f64) {
        match which {
            'B' | 'b' | '2' => self.deck_b.seek_fraction(frac, SeekMode::Standard),
            _ => self.deck_a.seek_fraction(frac, SeekMode::Standard),
        }
    }

    pub fn set_rate_percent(&mut self, which: char, percent: f64) {
        match which {
            'B' | 'b' | '2' => self.deck_b.set_rate_percent(percent),
            _ => self.deck_a.set_rate_percent(percent),
        }
    }

    pub fn set_crossfader(&mut self, x: f32) {
        self.mixer.set_crossfader(x);
    }

    pub fn cue(&mut self, which: char) {
        match which {
            'B' | 'b' | '2' => self.deck_b.cue_jump(),
            _ => self.deck_a.cue_jump(),
        }
    }

    pub fn set_cue_here(&mut self, which: char) {
        match which {
            'B' | 'b' | '2' => self.deck_b.set_cue_here(),
            _ => self.deck_a.set_cue_here(),
        }
    }

    /// Sync B → A (follower matches leader effective BPM).
    pub fn sync_b_to_a(&mut self) {
        sync_follower_to_leader(&self.deck_a, &mut self.deck_b);
    }

    /// Sync A → B
    pub fn sync_a_to_b(&mut self) {
        sync_follower_to_leader(&self.deck_b, &mut self.deck_a);
    }

    /// Process one mixer callback (Mixxx EngineMixer::process analogue).
    pub fn process(&mut self, out: &mut [f32], out_channels: usize) {
        out.fill(0.0);
        let (ga, gb) = self.mixer.channel_gains();
        let sr = self.output_sample_rate;
        self.deck_a.process(out, out_channels, sr, ga);
        self.deck_b.process(out, out_channels, sr, gb);
        // Soft clip / headroom
        for s in out.iter_mut() {
            *s = s.clamp(-1.0, 1.0);
        }
    }

    pub fn snapshot(&self) -> VinylSnapshot {
        VinylSnapshot {
            deck_a: self.deck_a.snapshot(),
            deck_b: self.deck_b.snapshot(),
            crossfader: self.mixer.crossfader,
            gain_a: self.mixer.gain_a,
            gain_b: self.mixer.gain_b,
            origin: ORIGIN_PLAN.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VinylSnapshot {
    pub deck_a: DeckSnapshot,
    pub deck_b: DeckSnapshot,
    pub crossfader: f32,
    pub gain_a: f32,
    pub gain_b: f32,
    pub origin: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn make_wav(path: &std::path::Path, freq: u32, secs: f32) -> bool {
        Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "lavfi",
                "-i",
                &format!("sine=frequency={freq}:duration={secs}"),
                "-ar",
                "44100",
                path.to_str().unwrap(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    #[test]
    fn dual_deck_mix_has_energy() {
        let dir = std::env::temp_dir().join("vmp_vinyl_test");
        let _ = std::fs::create_dir_all(&dir);
        let a = dir.join("a.wav");
        let b = dir.join("b.wav");
        if !make_wav(&a, 440, 0.5) || !make_wav(&b, 660, 0.5) {
            return;
        }

        let mut eng = VinylEngine::new();
        eng.output_sample_rate = 44100;
        eng.load('A', &a).unwrap();
        eng.load('B', &b).unwrap();
        eng.play('A', true);
        eng.play('B', true);
        eng.set_crossfader(0.5);

        let mut buf = vec![0.0f32; 4096];
        eng.process(&mut buf, 2);
        assert!(
            buf.iter().any(|s| s.abs() > 0.001),
            "mixed buffer should be non-silent"
        );

        // seek + rate
        eng.seek_fraction('A', 0.5);
        eng.set_rate_percent('B', 8.0);
        assert!((eng.deck_b.rate_percent() - 8.0).abs() < 0.01);
        eng.sync_b_to_a(); // no BPM → no-op but must not panic
    }

    #[test]
    fn cue_and_stop() {
        let dir = std::env::temp_dir().join("vmp_vinyl_cue");
        let _ = std::fs::create_dir_all(&dir);
        let a = dir.join("c.wav");
        if !make_wav(&a, 220, 1.0) {
            return;
        }
        let mut eng = VinylEngine::new();
        eng.load('A', &a).unwrap();
        eng.seek_fraction('A', 0.25);
        eng.set_cue_here('A');
        eng.play('A', true);
        eng.cue('A');
        assert!(!eng.deck_a.playing);
        assert!((eng.deck_a.play_pos - eng.deck_a.cue_pos).abs() < 1.0);
    }
}
