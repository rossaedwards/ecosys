//! Live VAP runtime — polished transmute of `vap_runtime.c` + `dsp_engine.c`.

use crate::photometric::{Affective, Photometric, VAP_CHROMATIC_MAP};
use serde::{Deserialize, Serialize};
use vmp_dsp::{chromatic_band_energies, saturation_index, spectral_centroid, syncopation_proxy};
use vmp_vap::{VapLiveState, VapObject};

/// Full runtime state (two-track model: static VAP + live Phase-I).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VapRuntime {
    // Phase I — Structural
    pub bpm_raw: f32,
    pub bpm_perceived: f32,
    pub groove_quantization: f32,
    pub syncopation_index: f32,
    pub kick_transient_ms: f32,
    // Phase I — Tonal
    pub key: String,
    pub dissonance_density: f32,
    pub chord_complexity: f32,
    // Phase I — Timbral
    pub spectral_centroid_hz: f32,
    pub saturation_index: f32,
    pub dynamic_range_lra: f32,
    pub spatial_width: i32,
    // Phase II
    pub affective: Affective,
    pub scenario_confidence: f32,
    pub scenario_tag: String,
    // Phase III
    pub photometric: Photometric,
    pub entrainment_factor: f32,
    pub met_score: f32,
    // Bookkeeping
    pub phase_time: f32,
    pub frame_count: u32,
    pub vap_loaded: bool,
}

impl Default for VapRuntime {
    fn default() -> Self {
        Self {
            bpm_raw: 120.0,
            bpm_perceived: 120.0,
            groove_quantization: 0.3,
            syncopation_index: 0.3,
            kick_transient_ms: 15.0,
            key: "C".into(),
            dissonance_density: 0.1,
            chord_complexity: 0.3,
            spectral_centroid_hz: 800.0,
            saturation_index: 0.2,
            dynamic_range_lra: 8.0,
            spatial_width: 1,
            affective: Affective::default(),
            scenario_confidence: 0.0,
            scenario_tag: "NONE".into(),
            photometric: Photometric::default(),
            entrainment_factor: 50.0,
            met_score: 3.0,
            phase_time: 0.0,
            frame_count: 0,
            vap_loaded: false,
        }
    }
}

impl VapRuntime {
    pub fn init() -> Self {
        Self::default()
    }

    /// Seed static pillars from a V.A.P. object (track load).
    pub fn load_vap(&mut self, vap: &VapObject) {
        if let Some(bpm) = vap.bpm() {
            self.bpm_raw = bpm as f32;
            self.bpm_perceived = bpm as f32;
        }
        if let Some(v) = vap.valence() {
            self.affective.valence = v as f32;
        }
        if let Some(a) = vap.arousal() {
            self.affective.arousal = a as f32;
        }
        if let Some(m) = vap.met_score() {
            self.met_score = m as f32;
        }
        if let Some(hex) = vap.primary_hex() {
            if let Some(rgb) = hex_to_rgb_norm(&hex) {
                self.photometric.primary_rgb = rgb;
            }
        }
        if let Some(key) = vap.pillars.tonal.as_ref().and_then(|t| {
            t.pointer("/HARMONIC_PROFILE/KEY")
                .or_else(|| t.get("KEY"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        }) {
            self.key = key;
        }
        self.vap_loaded = true;
    }

    /// Phase-I DSP update each audio buffer (from `vap_runtime_update_dsp` / `dsp_engine_update`).
    pub fn update_dsp(&mut self, fft_mag: &[f32], sample_rate: u32, dt: f32) {
        if fft_mag.len() < 4 || sample_rate < 8000 {
            return;
        }
        let n = fft_mag.len();
        let sr = sample_rate as f32;
        let alpha_120 = clampf(dt / 0.12, 0.0, 1.0);

        self.phase_time += dt;
        self.frame_count = self.frame_count.wrapping_add(1);

        // Pillar 3 — TIMBRAL
        let centroid = spectral_centroid(fft_mag, sr);
        let sat = saturation_index(fft_mag);
        self.spectral_centroid_hz = ema(self.spectral_centroid_hz, centroid, alpha_120);
        self.saturation_index = ema(self.saturation_index, sat, alpha_120);

        // Pillar 1 — STRUCTURAL
        let synco = syncopation_proxy(fft_mag);
        self.syncopation_index = ema(self.syncopation_index, synco, clampf(dt / 0.20, 0.0, 1.0));
        self.bpm_perceived = ema(
            self.bpm_perceived,
            self.bpm_raw,
            clampf(dt / 2.0, 0.0, 1.0),
        );

        // Pillar 5 — live arousal
        let mut rms_sq = 0.0f32;
        for m in fft_mag {
            rms_sq += m * m;
        }
        let rms = (rms_sq / n as f32).sqrt();
        let bpm_norm = clampf(self.bpm_raw / 180.0, 0.0, 1.0);
        let cent_norm = clampf(centroid / 5000.0, 0.0, 1.0);
        let arousal_t = clampf(rms * 0.5 + bpm_norm * 0.3 + cent_norm * 0.2, 0.0, 1.0);
        self.affective.arousal = ema(self.affective.arousal, arousal_t, alpha_120);

        // Pillar 7 — chromatic band energies
        let bands = chromatic_band_energies(fft_mag, sr);
        let alpha_band = clampf(dt / 0.08, 0.0, 1.0);
        for b in 0..4 {
            self.photometric.chrom_energy[b] =
                ema(self.photometric.chrom_energy[b], bands[b], alpha_band);
        }
        let _ = &VAP_CHROMATIC_MAP; // keep map linked for shader consumers
    }

    /// Snapshot for UI / WebGL uniforms.
    pub fn to_live_state(&self) -> VapLiveState {
        VapLiveState {
            bpm_raw: self.bpm_raw,
            groove_quantization: self.groove_quantization,
            syncopation_index: self.syncopation_index,
            kick_transient_ms: self.kick_transient_ms,
            spectral_centroid_hz: self.spectral_centroid_hz,
            saturation_index: self.saturation_index,
            dissonance_density: self.dissonance_density,
            valence: self.affective.valence,
            arousal: self.affective.arousal,
            dominance: self.affective.dominance,
            entrainment_factor: self.entrainment_factor,
            met_score: self.met_score,
            chrom_energy: self.photometric.chrom_energy,
            phase_time: self.phase_time,
            frame_count: self.frame_count as u64,
        }
    }

    /// Shader uniform bundle (names match vibe.frag).
    pub fn shader_uniforms(&self) -> ShaderUniforms {
        ShaderUniforms {
            centroid: self.spectral_centroid_hz,
            saturation: self.saturation_index,
            syncopation: self.syncopation_index,
            bpm_norm: clampf(self.bpm_raw / 180.0, 0.0, 1.0),
            groove: self.groove_quantization,
            dissonance: self.dissonance_density,
            valence: self.affective.valence,
            arousal: self.affective.arousal,
            primary_rgb: self.photometric.primary_rgb,
            secondary_rgb: self.photometric.secondary_rgb,
            brightness_floor: self.photometric.brightness_floor,
            brightness_ceiling: self.photometric.brightness_ceiling,
            strobe_trigger: self.photometric.strobe_threshold,
            fog_density: self.photometric.fog_density,
            visual_noise: self.photometric.visual_noise,
            chrom_energy: self.photometric.chrom_energy,
            entrainment: self.entrainment_factor,
            time: self.phase_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderUniforms {
    pub centroid: f32,
    pub saturation: f32,
    pub syncopation: f32,
    pub bpm_norm: f32,
    pub groove: f32,
    pub dissonance: f32,
    pub valence: f32,
    pub arousal: f32,
    pub primary_rgb: [f32; 3],
    pub secondary_rgb: [f32; 3],
    pub brightness_floor: f32,
    pub brightness_ceiling: f32,
    pub strobe_trigger: f32,
    pub fog_density: f32,
    pub visual_noise: f32,
    pub chrom_energy: [f32; 4],
    pub entrainment: f32,
    pub time: f32,
}

#[inline]
fn clampf(v: f32, lo: f32, hi: f32) -> f32 {
    v.clamp(lo, hi)
}

#[inline]
fn ema(prev: f32, next: f32, alpha: f32) -> f32 {
    prev + alpha * (next - prev)
}

fn hex_to_rgb_norm(hex: &str) -> Option<[f32; 3]> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&h[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&h[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&h[4..6], 16).ok()? as f32 / 255.0;
    Some([r, g, b])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsp_update_moves_centroid() {
        let mut rt = VapRuntime::init();
        // Fake FFT: energy at high bins → bright centroid
        let mut mag = vec![0.0f32; 64];
        for i in 40..64 {
            mag[i] = 1.0;
        }
        rt.update_dsp(&mag, 44100, 1.0 / 60.0);
        assert!(rt.spectral_centroid_hz > 200.0);
        assert!(rt.frame_count == 1);
        assert!(rt.affective.arousal > 0.0);
    }

    #[test]
    fn load_vap_seeds_bpm() {
        let vap = VapObject::defaults("Test", "Song");
        let mut rt = VapRuntime::init();
        rt.load_vap(&vap);
        assert!(rt.vap_loaded);
        assert!((rt.bpm_raw - 120.0).abs() < 0.01);
    }
}
