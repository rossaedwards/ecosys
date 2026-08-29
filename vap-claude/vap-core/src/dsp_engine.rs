//! Live DSP front-end: PCM -> windowed FFT -> magnitude spectrum -> [`VapRuntime`].
//!
//! Extends `src/dsp_engine.c` (Phase I physical analysis) with a real FFT
//! pipeline (rustfft) and simple onset/beat-phase tracking; the actual
//! Pillar 1/3/5/7.1 scoring lives in [`crate::vap_runtime::VapRuntime::update_dsp`],
//! which is the authoritative, EMA-smoothed implementation from `vap_runtime.c`
//! (superseding the simpler duplicate in the original `dsp_engine.c`).

use std::collections::VecDeque;
use std::sync::Arc;

use rustfft::{num_complex::Complex, Fft, FftPlanner};

use crate::vap_runtime::VapRuntime;

const DEFAULT_FFT_SIZE: usize = 2048;
const ENERGY_HISTORY_LEN: usize = 32;
const ONSET_THRESHOLD: f32 = 1.3;
const ONSET_DEBOUNCE_SECS: f32 = 0.05;
const DEFAULT_ONSET_INTERVAL_SECS: f32 = 0.5; // ~120 BPM fallback

/// One analyzed audio frame, ready to be folded into a [`VapRuntime`].
pub struct DspFrame {
    /// Magnitude spectrum, bins `0..=fft_size/2` (DC..Nyquist).
    pub magnitudes: Vec<f32>,
    pub rms: f32,
    pub beat_onset: bool,
    pub beat_phase: f32,
}

pub struct DspEngine {
    fft: Arc<dyn Fft<f32>>,
    fft_size: usize,
    window: Vec<f32>,
    mono_buffer: VecDeque<f32>,
    energy_history: VecDeque<f32>,
    time_since_onset: f32,
    avg_onset_interval: f32,
}

impl DspEngine {
    pub fn new() -> Self {
        Self::with_fft_size(DEFAULT_FFT_SIZE)
    }

    pub fn with_fft_size(fft_size: usize) -> Self {
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(fft_size);
        let window = hann_window(fft_size);
        DspEngine {
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
    /// one analysis window, runs the FFT and returns a [`DspFrame`].
    /// `dt` is the wall-clock time elapsed since the previous call, used
    /// for beat-phase tracking.
    pub fn process(
        &mut self,
        pcm: &[f32],
        channels: usize,
        sample_rate: u32,
        dt: f32,
    ) -> Option<DspFrame> {
        downmix_into(pcm, channels, &mut self.mono_buffer);

        if self.mono_buffer.len() < self.fft_size {
            return None;
        }

        let mut buf: Vec<Complex<f32>> = self
            .mono_buffer
            .iter()
            .take(self.fft_size)
            .zip(self.window.iter())
            .map(|(&s, &w)| Complex::new(s * w, 0.0))
            .collect();
        // Slide the window forward by half the FFT size (50% overlap).
        let hop = self.fft_size / 2;
        for _ in 0..hop.min(self.mono_buffer.len()) {
            self.mono_buffer.pop_front();
        }

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
        let beat_phase = (self.time_since_onset / self.avg_onset_interval.max(0.05)).clamp(0.0, 1.0);

        let _ = sample_rate; // sample_rate is consumed by VapRuntime::update_dsp, not here.

        Some(DspFrame {
            magnitudes,
            rms,
            beat_onset,
            beat_phase,
        })
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

impl Default for DspEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply a [`DspFrame`] to a [`VapRuntime`]: runs Pillar 1/3/5/7.1 scoring
/// and records the live rms/beat outputs.
pub fn apply_frame(vap: &mut VapRuntime, frame: &DspFrame, sample_rate: u32, dt: f32) {
    vap.update_dsp(&frame.magnitudes, sample_rate, dt);
    vap.rms = frame.rms;
    vap.beat_onset = frame.beat_onset;
    vap.beat_phase = frame.beat_phase;
}

fn hann_window(size: usize) -> Vec<f32> {
    if size <= 1 {
        return vec![1.0; size];
    }
    (0..size)
        .map(|i| {
            0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / (size as f32 - 1.0)).cos()
        })
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
