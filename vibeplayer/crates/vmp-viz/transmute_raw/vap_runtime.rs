//! Transmuted by v01d (FUTE) — C/C++ → Rust
//! Origin: ../vibe-audio-visualizer/src/vap_runtime.h
//! Engine: Fuxyez Universal Transmutation Engine

#![allow(dead_code, non_snake_case, unused_variables, unused_mut)]

// #ifndef VAP_RUNTIME_H
// #define VAP_RUNTIME_H

// use crate::vap_photometric; // from #include "vap_photometric.h"
// use crate::vap_affective; // from #include "vap_affective.h"





#[derive(Debug, Clone)]
pub struct VapRuntime {
    pub bpm_raw: f32,
    pub bpm_perceived: f32,
    pub groove_quantization: f32,
    pub syncopation_index: f32,
    pub kick_transient_ms: f32,
    pub key: [u8; 8],
    pub dissonance_density: f32,
    pub chord_complexity: f32,
    pub spectral_centroid_hz: f32,
    pub saturation_index: f32,
    pub dynamic_range_lra: f32,
    pub spatial_width: i32,
    pub affective: VapAffective,
    pub scenario_confidence: f32,
    pub scenario_tag: [u8; 32],
    pub photometric: VapPhotometric,
    pub entrainment_factor: f32,
    pub met_score: f32,
    pub phase_time: f32,
    pub frame_count: u32,
    pub vap_loaded: i32,
}


pub fn vap_runtime_init(vap: &mut VapRuntime); // declaration

pub fn vap_runtime_load_json(vap: &mut VapRuntime, filepath: &[u8] /* [transmute] ptr→slice */); // declaration

void vap_runtime_update_dsp(vap_runtime_t *vap,
const float *fft_mag, int fft_size,
int sample_rate, float dt);

// #endif
