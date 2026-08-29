//! PCM -> windowed FFT -> magnitude spectrum, plus onset/beat-phase tracking.
//!
//! Produces the `mag: &[f32]` input the rest of this crate's `analysis` module
//! already consumes (`spectral_centroid`, `saturation_index`, `chromatic_band_energies`).
//! Ported from the standalone `vap-claude/vap-core/src/dsp_engine.rs` prototype.

use std::collections::VecDeque;
use std::sync::Arc;

use rustfft::{num_complex::Complex, Fft, FftPlanner};

const DEFAULT_FFT_SIZE: usize = 2048;
const ENERGY_HISTORY_LEN: usize = 32;
const ONSET_THRESHOLD: f32 = 1.3;
const ONSET_DEBOUNCE_SECS: f32 = 0.05;
const DEFAULT_ONSET_INTERVAL_SECS: f32 = 0.5; // ~120 BPM fallback

/// One analyzed audio frame.
pub struct AnalysisFrame {
    /// Magnitude spectrum, bins `0..=fft_size/2` (DC..Nyquist).
    pub magnitudes: Vec<f32>,
    pub rms: f32,
    pub beat_onset: bool,
    pub beat_phase: f32,
}

pub struct AudioAnalyzer {
    fft: Arc<dyn Fft<f32>>,
    fft_size: usize,
    window: Vec<f32>,
    mono_buffer: VecDeque<f32>,
    energy_history: VecDeque<f32>,
    time_since_onset: f32,
    avg_onset_interval: f32,
}

impl AudioAnalyzer {
    pub fn new() -> Self {
        Self::with_fft_size(DEFAULT_FFT_SIZE)
    }

    pub fn with_fft_size(fft_size: usize) -> Self {
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(fft_size);
        let window = hann_window(fft_size);
        AudioAnalyzer {
            fft,
            fft_size,
            window,
            mono_buffer: VecDeque::with_capacity(fft_size * 2),
            energy_history: VecDeque::with_capacity(ENERGY_HISTORY_LEN),
            time_since_onset: 0.0,
            avg_onset_interval: DEFAULT_ONSET_INTERVAL_SECS,
        }
    }

    /// Feed a chunk of interleaved PCM samples (any length). Downmixes to
    /// mono and accumulates internally; once enough samples exist to fill
    /// one analysis window, runs the FFT and returns an [`AnalysisFrame`].
    /// `dt` is wall-clock time elapsed since the previous call, used for
    /// beat-phase tracking.
    pub fn process(
        &mut self,
        pcm: &[f32],
        channels: usize,
        dt: f32,
    ) -> Option<AnalysisFrame> {
        downmix_into(pcm, channels, &mut self.mono_buffer);

        // Drain every complete window this call produced (a caller may push
        // more samples per call than one hop consumes), keeping only the
        // most recent — prevents unbounded backlog growth when push-rate
        // exceeds hop-rate, and callers only need "current" analysis state.
        let hop = (self.fft_size / 2).max(1);
        let mut latest: Option<AnalysisFrame> = None;
        while self.mono_buffer.len() >= self.fft_size {
            latest = Some(self.analyze_window(dt));
            for _ in 0..hop {
                self.mono_buffer.pop_front();
            }
        }
        latest
    }

    /// Runs the FFT over the current front-of-buffer window and updates
    /// onset/beat-phase state. Does not touch `mono_buffer`.
    fn analyze_window(&mut self, dt: f32) -> AnalysisFrame {
        let mut buf: Vec<Complex<f32>> = self
            .mono_buffer
            .iter()
            .take(self.fft_size)
            .zip(self.window.iter())
            .map(|(&s, &w)| Complex::new(s * w, 0.0))
            .collect();

        self.fft.process(&mut buf);

        let n_bins = self.fft_size / 2 + 1;
        let scale = 1.0 / self.fft_size as f32;
        let magnitudes: Vec<f32> = buf[..n_bins].iter().map(|c| c.norm() * scale).collect();

        let rms = {
            let sum_sq: f32 = magnitudes.iter().map(|m| m * m).sum();
            (sum_sq / n_bins as f32).sqrt()
        };

        let beat_onset = self.detect_onset(rms);
        if beat_onset {
            if self.time_since_onset > ONSET_DEBOUNCE_SECS {
                self.avg_onset_interval =
                    self.avg_onset_interval * 0.7 + self.time_since_onset * 0.3;
            }
            self.time_since_onset = 0.0;
        } else {
            self.time_since_onset += dt;
        }
        let beat_phase =
            (self.time_since_onset / self.avg_onset_interval.max(0.05)).clamp(0.0, 1.0);

        AnalysisFrame {
            magnitudes,
            rms,
            beat_onset,
            beat_phase,
        }
    }

    fn detect_onset(&mut self, current_energy: f32) -> bool {
        let onset = if self.energy_history.is_empty() {
            false
        } else {
            let avg: f32 =
                self.energy_history.iter().sum::<f32>() / self.energy_history.len() as f32;
            current_energy > avg * ONSET_THRESHOLD
        };

        if self.energy_history.len() == ENERGY_HISTORY_LEN {
            self.energy_history.pop_front();
        }
        self.energy_history.push_back(current_energy);

        onset
    }
}

impl Default for AudioAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn hann_window(size: usize) -> Vec<f32> {
    if size <= 1 {
        return vec![1.0; size];
    }
    (0..size)
        .map(|i| 0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / (size as f32 - 1.0)).cos())
        .collect()
}

fn downmix_into(pcm: &[f32], channels: usize, out: &mut VecDeque<f32>) {
    if channels <= 1 {
        out.extend(pcm.iter().copied());
        return;
    }
    for frame in pcm.chunks_exact(channels) {
        let sum: f32 = frame.iter().sum();
        out.push_back(sum / channels as f32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_frame_once_window_fills() {
        let mut analyzer = AudioAnalyzer::with_fft_size(64);
        // Fewer samples than the window: no frame yet.
        assert!(analyzer.process(&[0.0; 32], 1, 1.0 / 60.0).is_none());
        // Enough samples: a frame comes out.
        let frame = analyzer.process(&[0.1; 64], 1, 1.0 / 60.0);
        assert!(frame.is_some());
        let frame = frame.unwrap();
        assert_eq!(frame.magnitudes.len(), 64 / 2 + 1);
        assert!(frame.rms.is_finite());
    }

    #[test]
    fn downmixes_stereo_to_mono() {
        let mut buf = VecDeque::new();
        // L=1.0, R=-1.0 -> mono 0.0
        downmix_into(&[1.0, -1.0, 1.0, -1.0], 2, &mut buf);
        assert_eq!(buf.len(), 2);
        assert!(buf.iter().all(|&s| s.abs() < 1e-6));
    }

    #[test]
    fn onset_fires_on_energy_spike() {
        let mut analyzer = AudioAnalyzer::with_fft_size(32);
        // Warm up the energy history with quiet frames.
        for _ in 0..40 {
            analyzer.process(&vec![0.01; 32], 1, 1.0 / 60.0);
        }
        // A loud frame should register as an onset.
        let frame = analyzer.process(&vec![1.0; 32], 1, 1.0 / 60.0).unwrap();
        assert!(frame.beat_onset);
    }
}
