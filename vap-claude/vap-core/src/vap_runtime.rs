//! Port of `src/vap_runtime.c` / `vap_runtime.h` — V.A.P. v3.1 full runtime state.
//!
//! Phase I fields are updated every audio frame by [`crate::dsp_engine`].
//! Phase II/III fields are loaded from a `.vap.json` sidecar at track start
//! via [`crate::vap_loader`].

use crate::vap_affective::VapAffective;
use crate::vap_photometric::VapPhotometric;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SpatialWidth {
    Mono,
    #[default]
    Stereo,
    Immersive,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ExplicitTier {
    #[default]
    Clean,
    Mild,
    Explicit,
    Severe,
}

#[inline]
fn clampf(v: f32, lo: f32, hi: f32) -> f32 {
    v.max(lo).min(hi)
}

#[inline]
fn ema(prev: f32, next: f32, alpha: f32) -> f32 {
    prev + alpha * (next - prev)
}

pub struct VapRuntime {
    // ── PHASE I: DSP (live, per-frame) ──────────────────────────
    // Pillar 1: Structural
    pub bpm_raw: f32,
    pub bpm_perceived: f32,
    /// 0.0 = Machine Lock, 1.0 = Human Swing
    pub groove_quantization: f32,
    pub syncopation_index: f32,
    pub kick_transient_ms: f32,

    // Pillar 2: Tonal
    pub key: String,
    /// 0.0-1.0, % duration dissonant
    pub dissonance_density: f32,
    /// 0.0 = Triadic, 1.0 = Extended 13th
    pub chord_complexity: f32,

    // Pillar 3: Timbral
    pub spectral_centroid_hz: f32,
    /// THD: 0.0 = Sine, 1.0 = Bitcrush
    pub saturation_index: f32,
    pub dynamic_range_lra: f32,
    pub spatial_width: SpatialWidth,

    // Pillar 4: Linguistic
    pub explicit_tier: ExplicitTier,

    // ── PHASE II: ML (loaded from .vap.json) ────────────────────
    pub affective: VapAffective,
    /// Pillar 6: Bayesian scenario confidence %
    pub scenario_confidence: f32,
    /// e.g. "Night_Drive"
    pub scenario_tag: String,
    pub contextual_fog_mod: f32,

    // ── PHASE III: I/O (loaded from .vap.json) ──────────────────
    pub photometric: VapPhotometric,
    /// Pillar 8: 0-100 motor response
    pub entrainment_factor: f32,
    /// Pillar 8: Metabolic equivalent
    pub met_score: f32,
    pub target_hr_zone: String,
    pub motor_drive: f32,
    pub motor_sway: f32,
    pub head_nod: f32,

    // Pillar 9: Genealogical
    pub timelessness_score: f32,
    pub authenticity_ratio: f32,
    pub viral_velocity: f32,

    // Identity
    pub title: String,
    pub artist: String,

    // Live DSP outputs — per-band chromatic energy (Pillar 7.1), EMA-smoothed.
    // Band order: [0]=Sub-Bass [1]=Low-Mid [2]=Mids [3]=Highs
    pub chroma_energy: [f32; 4],
    /// RMS of the current analysis frame's magnitude spectrum.
    pub rms: f32,
    /// True on the frame a beat onset was detected (see [`crate::dsp_engine`]).
    pub beat_onset: bool,
    /// 0.0-1.0 linear ramp since the last detected onset.
    pub beat_phase: f32,

    // ── Internal runtime ─────────────────────────────────────────
    pub phase_time: f32,
    pub frame_count: u32,
    pub vap_loaded: bool,
}

impl Default for VapRuntime {
    fn default() -> Self {
        VapRuntime {
            bpm_raw: 120.0,
            bpm_perceived: 120.0,
            groove_quantization: 0.3,
            syncopation_index: 0.3,
            kick_transient_ms: 15.0,

            key: "C".to_string(),
            dissonance_density: 0.1,
            chord_complexity: 0.3,

            spectral_centroid_hz: 800.0,
            saturation_index: 0.2,
            dynamic_range_lra: 8.0,
            spatial_width: SpatialWidth::Stereo,

            explicit_tier: ExplicitTier::Clean,

            affective: VapAffective::default(),
            scenario_confidence: 0.0,
            scenario_tag: "NONE".to_string(),
            contextual_fog_mod: 0.2,

            photometric: VapPhotometric::default(),
            entrainment_factor: 50.0,
            met_score: 3.0,
            target_hr_zone: "90-110".to_string(),
            motor_drive: 0.5,
            motor_sway: 0.5,
            head_nod: 0.5,

            timelessness_score: 0.5,
            authenticity_ratio: 0.5,
            viral_velocity: 0.0,

            title: "Unknown".to_string(),
            artist: "Unknown".to_string(),

            chroma_energy: [0.0; 4],
            rms: 0.0,
            beat_onset: false,
            beat_phase: 0.0,

            phase_time: 0.0,
            frame_count: 0,
            vap_loaded: false,
        }
    }
}

impl VapRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    /// `u_bpm_norm` uniform value, per gl_renderer.c upload_frame_uniforms().
    pub fn bpm_norm(&self) -> f32 {
        (self.bpm_raw / 180.0).min(1.0)
    }

    /// `u_scenario_fog` uniform value: contextual_fog_mod * photometric.fog_density.
    pub fn scenario_fog(&self) -> f32 {
        self.contextual_fog_mod * self.photometric.fog_density
    }

    /// Bloom strength: arousal x brightness_ceiling, per gl_renderer.c upload_bloom_uniforms().
    pub fn bloom_strength(&self) -> f32 {
        self.affective.arousal * self.photometric.brightness_ceiling * 1.5
    }

    /// Port of `vap_runtime_update_dsp()`. Call once per audio analysis frame
    /// with the magnitude spectrum `mag` (length `fft_size/2 + 1` bins, DC..Nyquist).
    pub fn update_dsp(&mut self, mag: &[f32], sample_rate: u32, dt: f32) {
        if mag.len() < 4 || sample_rate < 8000 {
            return;
        }
        let n = mag.len();

        self.phase_time += dt;
        self.frame_count += 1;

        // Pillar 3 — TIMBRAL
        let centroid = compute_centroid(mag, sample_rate);
        let saturation = compute_saturation(mag);
        let alpha_120 = clampf(dt / 0.12, 0.0, 1.0);

        self.spectral_centroid_hz = ema(self.spectral_centroid_hz, centroid, alpha_120);
        self.saturation_index = ema(self.saturation_index, saturation, alpha_120);

        // Pillar 1 — STRUCTURAL
        let synco = compute_syncopation(mag);
        self.syncopation_index = ema(self.syncopation_index, synco, clampf(dt / 0.20, 0.0, 1.0));
        self.bpm_perceived = ema(self.bpm_perceived, self.bpm_raw, clampf(dt / 2.0, 0.0, 1.0));

        // Pillar 5 — AFFECTIVE (live arousal)
        let rms_sq: f32 = mag.iter().map(|m| m * m).sum::<f32>() / n as f32;
        let rms = rms_sq.sqrt();

        let bpm_norm = clampf(self.bpm_raw / 180.0, 0.0, 1.0);
        let cent_norm = clampf(centroid / 5000.0, 0.0, 1.0);
        let arousal_t = clampf(rms * 0.5 + bpm_norm * 0.3 + cent_norm * 0.2, 0.0, 1.0);
        self.affective.arousal = ema(self.affective.arousal, arousal_t, alpha_120);

        // Pillar 7.1 — CHROMATIC BAND ENERGIES
        let fresh = compute_band_energies(mag, sample_rate);
        let alpha_band = clampf(dt / 0.08, 0.0, 1.0);
        for (energy, fresh) in self.chroma_energy.iter_mut().zip(fresh) {
            *energy = ema(*energy, fresh, alpha_band);
        }
    }
}

/// Spectral Centroid: center of gravity of the frequency spectrum.
fn compute_centroid(mag: &[f32], sample_rate: u32) -> f32 {
    let n = mag.len();
    let bin_hz = sample_rate as f32 / (n as f32 * 2.0);
    let mut num = 0.0f32;
    let mut den = 0.0f32;
    for (i, &m) in mag.iter().enumerate().skip(1) {
        let f = i as f32 * bin_hz;
        num += f * m;
        den += m;
    }
    if den > 1e-6 {
        num / den
    } else {
        0.0
    }
}

/// Saturation Index: THD approximation via harmonic energy ratio.
fn compute_saturation(mag: &[f32]) -> f32 {
    let n = mag.len();
    if n < 4 {
        return 0.0;
    }
    let fundamental = mag[2] * mag[2];
    let mut harmonics = 0.0f32;
    for &m in &mag[3..n.min(12)] {
        harmonics += m * m;
    }
    let total = fundamental + harmonics;
    if total > 1e-6 {
        harmonics / total
    } else {
        0.0
    }
}

/// Syncopation Index: simplified energy variance between beat subdivisions.
fn compute_syncopation(mag: &[f32]) -> f32 {
    let n = mag.len();
    let half = n / 2;
    let even: f32 = mag[..half].iter().sum();
    let odd: f32 = mag[half..].iter().sum();
    let total = even + odd;
    if total > 1e-6 {
        odd / total
    } else {
        0.0
    }
}

/// Per-band chromatic energy, following `VAP_CHROMATIC_MAP` band edges.
fn compute_band_energies(mag: &[f32], sample_rate: u32) -> [f32; 4] {
    const BAND_LO: [f32; 4] = [40.0, 60.0, 250.0, 2000.0];
    const BAND_HI: [f32; 4] = [60.0, 250.0, 2000.0, 20000.0];

    let n = mag.len();
    let bin_hz = sample_rate as f32 / (n as f32 * 2.0);
    let mut sums = [0.0f32; 4];
    let mut counts = [0u32; 4];

    for (i, &m) in mag.iter().enumerate().skip(1) {
        let f = i as f32 * bin_hz;
        for b in 0..4 {
            if f >= BAND_LO[b] && f < BAND_HI[b] {
                sums[b] += m;
                counts[b] += 1;
            }
        }
    }

    let mut out = [0.0f32; 4];
    for b in 0..4 {
        let avg = if counts[b] > 0 {
            sums[b] / counts[b] as f32
        } else {
            0.0
        };
        out[b] = clampf(avg, 0.0, 1.0);
    }
    out
}
