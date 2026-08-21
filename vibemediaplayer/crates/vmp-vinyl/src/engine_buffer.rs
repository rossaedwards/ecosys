//! Deck buffer — polished Mixxx `EngineBuffer` analogue.
//!
//! Semantics (play / seek / rate / cue) follow Mixxx control model;
//! implementation is Rust + `vmp-audio` decode (linear scale for now;
//! SoundTouch/RubberBand keylock later).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use vmp_audio::{decode_file, DecodedTrack};
use vmp_vap::VapObject;

/// Seek style (from Mixxx EngineBuffer::SeekRequest, simplified).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SeekMode {
    /// Exact frame (bypass quantize)
    Exact,
    /// Phase-aligned seek (placeholder → exact until beat grid)
    Phase,
    #[default]
    /// Standard: phase if quantize else exact
    Standard,
}

/// One playable deck (Mixxx EngineBuffer surface).
#[derive(Debug)]
pub struct EngineBuffer {
    pub group: String,
    pub path: Option<PathBuf>,
    pub playing: bool,
    /// Playback rate ratio: 1.0 = original, 1.08 = +8%
    pub rate: f64,
    /// Fractional position in track frames (sub-sample)
    pub play_pos: f64,
    pub cue_pos: f64,
    pub quantize: bool,
    pub reverse: bool,
    pub slip_enabled: bool,
    pub bpm: Option<f64>,
    pub sample_rate: u32,
    pub channel_count: u16,
    pub vap: Option<VapObject>,
    track: Option<DecodedTrack>,
}

impl EngineBuffer {
    pub fn new(group: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            path: None,
            playing: false,
            rate: 1.0,
            play_pos: 0.0,
            cue_pos: 0.0,
            quantize: true,
            reverse: false,
            slip_enabled: false,
            bpm: None,
            sample_rate: 44100,
            channel_count: 2,
            vap: None,
            track: None,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.track.is_some()
    }

    pub fn duration_sec(&self) -> f64 {
        self.track.as_ref().map(|t| t.duration_sec).unwrap_or(0.0)
    }

    pub fn position_sec(&self) -> f64 {
        let sr = self.sample_rate.max(1) as f64;
        self.play_pos / sr
    }

    pub fn num_frames(&self) -> u64 {
        self.track.as_ref().map(|t| t.frames() as u64).unwrap_or(0)
    }

    /// Load decoded PCM (Mixxx slotTrackLoaded analogue).
    pub fn load_path(&mut self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        let decoded = decode_file(path).map_err(|e| e.to_string())?;
        let vap = vmp_audio::load_media_tags(path)
            .map(|b| b.vap)
            .unwrap_or_else(|_| {
                VapObject::defaults(
                    "Unknown",
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Untitled"),
                )
            });

        self.sample_rate = decoded.sample_rate;
        self.channel_count = decoded.channels;
        self.bpm = vap.bpm();
        self.play_pos = 0.0;
        self.cue_pos = 0.0;
        self.playing = false;
        self.path = Some(path.to_path_buf());
        self.vap = Some(vap);
        self.track = Some(decoded);
        Ok(())
    }

    /// Mixxx slotControlPlayRequest
    pub fn set_play(&mut self, on: bool) {
        if self.track.is_none() {
            self.playing = false;
            return;
        }
        self.playing = on;
    }

    pub fn toggle_play(&mut self) {
        self.set_play(!self.playing);
    }

    /// Mixxx slotControlStop — stop and optionally return to cue
    pub fn stop(&mut self, to_cue: bool) {
        self.playing = false;
        if to_cue {
            self.play_pos = self.cue_pos;
        }
    }

    /// Mixxx seekAbs / slotControlSeek (fractional 0..1)
    pub fn seek_fraction(&mut self, frac: f64, mode: SeekMode) {
        let n = self.num_frames() as f64;
        if n <= 0.0 {
            return;
        }
        let mut pos = frac.clamp(0.0, 1.0) * n;
        if matches!(mode, SeekMode::Phase | SeekMode::Standard) && self.quantize {
            if let Some(bpm) = self.bpm {
                if bpm > 0.0 {
                    let frames_per_beat = (self.sample_rate as f64) * 60.0 / bpm;
                    if frames_per_beat > 0.0 {
                        pos = (pos / frames_per_beat).round() * frames_per_beat;
                    }
                }
            }
        }
        self.play_pos = pos.clamp(0.0, n);
    }

    /// Absolute frame seek (Mixxx seekExact)
    pub fn seek_exact(&mut self, frame: f64) {
        let n = self.num_frames() as f64;
        self.play_pos = frame.clamp(0.0, n.max(0.0));
    }

    /// Set cue at current position (hotcue 0 style)
    pub fn set_cue_here(&mut self) {
        self.cue_pos = self.play_pos;
    }

    /// Jump to cue and stop (Mixxx cue default behaviour simplified)
    pub fn cue_jump(&mut self) {
        self.play_pos = self.cue_pos;
        self.playing = false;
    }

    /// Rate as percent (-8.0 = 0.92x). Mixxx rate range simplified.
    pub fn set_rate_percent(&mut self, percent: f64) {
        let p = percent.clamp(-50.0, 50.0);
        self.rate = 1.0 + p / 100.0;
    }

    pub fn rate_percent(&self) -> f64 {
        (self.rate - 1.0) * 100.0
    }

    /// Effective BPM after rate
    pub fn effective_bpm(&self) -> Option<f64> {
        self.bpm.map(|b| b * self.rate)
    }

    /// Render `out_frames` of audio into interleaved `out` at `out_sr`, scaled by `gain`.
    /// Linear interpolation resample (EngineBufferScaleLinear analogue).
    pub fn process(&mut self, out: &mut [f32], out_channels: usize, out_sr: u32, gain: f32) {
        let ch = out_channels.max(1);
        let frames = out.len() / ch;
        let Some(track) = &self.track else {
            return;
        };
        if !self.playing || frames == 0 {
            return;
        }

        let in_ch = track.channels as usize;
        let total = track.frames() as f64;
        let dir = if self.reverse { -1.0 } else { 1.0 };
        let step = (track.sample_rate as f64 / out_sr.max(1) as f64) * self.rate * dir;

        for i in 0..frames {
            if self.play_pos < 0.0 || self.play_pos >= total {
                self.playing = false;
                break;
            }
            let src = self.play_pos;
            let i0 = src.floor() as usize;
            let i1 = (i0 + 1).min(track.frames().saturating_sub(1));
            let frac = (src - i0 as f64) as f32;
            let base0 = i0 * in_ch;
            let base1 = i1 * in_ch;

            for c in 0..ch {
                let c0 = c.min(in_ch.saturating_sub(1));
                let s0 = track.samples.get(base0 + c0).copied().unwrap_or(0.0);
                let s1 = track.samples.get(base1 + c0).copied().unwrap_or(s0);
                let s = s0 + (s1 - s0) * frac;
                out[i * ch + c] += s * gain;
            }
            self.play_pos += step;
        }
    }

    pub fn snapshot(&self) -> DeckSnapshot {
        DeckSnapshot {
            group: self.group.clone(),
            path: self.path.as_ref().map(|p| p.display().to_string()),
            playing: self.playing,
            rate_percent: self.rate_percent(),
            position_sec: self.position_sec(),
            duration_sec: self.duration_sec(),
            cue_sec: self.cue_pos / self.sample_rate.max(1) as f64,
            bpm: self.bpm,
            effective_bpm: self.effective_bpm(),
            quantize: self.quantize,
            loaded: self.is_loaded(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckSnapshot {
    pub group: String,
    pub path: Option<String>,
    pub playing: bool,
    pub rate_percent: f64,
    pub position_sec: f64,
    pub duration_sec: f64,
    pub cue_sec: f64,
    pub bpm: Option<f64>,
    pub effective_bpm: Option<f64>,
    pub quantize: bool,
    pub loaded: bool,
}
