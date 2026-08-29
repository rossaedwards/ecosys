//! Phase 7 — Auraphyx: a TSL (Three-Squared-Lattice) volumetric phase
//! alignment engine layered on top of the existing Chladni VAP visualizer.
//!
//! Maps audio into the 3 planes of the lattice:
//!   - X (base nodes):        sub-bass 40-250Hz  -> anchors lattice geometry
//!   - Y (harmonic osc.):     mids 250Hz-2kHz     -> drives nodal wave motion
//!   - Z (particle emiss.):   highs 2kHz+          -> sparkle/surface refraction
//!
//! Stereo L/R phase coherence rotates the lattice around its Z-axis: high
//! coherence gives a stable symmetric form, cancellation fractures/glitches it.
//!
//! This module ADDS to the Chladni field; it never replaces it.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuraphyxFrame {
    /// Sub-bass lattice anchor strength, 0.0-1.0.
    pub tsl_x: f32,
    /// Mid harmonic oscillation strength, 0.0-1.0.
    pub tsl_y: f32,
    /// High-frequency particle emissivity, 0.0-1.0.
    pub tsl_z: f32,
    /// Stereo L/R phase coherence, -1.0 (cancelling) to +1.0 (in-phase).
    pub phase_align: f32,
    /// Accumulated lattice rotation, radians.
    pub lattice_rot: f32,
}

pub struct AuraphyxEngine {
    lattice_rot: f32,
}

impl AuraphyxEngine {
    pub fn new() -> Self {
        AuraphyxEngine { lattice_rot: 0.0 }
    }

    /// `pcm_stereo` is interleaved L/R samples for the current analysis
    /// window; `chrom` is the current Pillar 7.1 chromatic band energy
    /// (`Photometric::chrom_energy`: [sub-bass, low-mid, mids, highs]).
    /// `dt` is the wall-clock time elapsed since the previous call.
    pub fn process(&mut self, pcm_stereo: &[f32], chrom: &[f32; 4], dt: f32) -> AuraphyxFrame {
        let phase_align = stereo_phase_coherence(pcm_stereo);

        let tsl_x = (chrom[0] + chrom[1]) * 0.5;
        let tsl_y = chrom[2];
        let tsl_z = chrom[3];

        // Rotate the lattice in proportion to phase coherence; cancelling
        // audio (phase_align near -1) spins it the opposite way, feeding
        // the shader's fracture term.
        self.lattice_rot += phase_align * dt * std::f32::consts::PI * 0.5;
        self.lattice_rot %= std::f32::consts::TAU;

        AuraphyxFrame {
            tsl_x,
            tsl_y,
            tsl_z,
            phase_align,
            lattice_rot: self.lattice_rot,
        }
    }
}

impl Default for AuraphyxEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Zero-lag normalized cross-correlation between L and R channels.
/// +1.0 = perfectly in-phase (mono-like), -1.0 = fully out-of-phase (cancelling).
fn stereo_phase_coherence(pcm_stereo: &[f32]) -> f32 {
    let mut sum_lr = 0.0f32;
    let mut sum_ll = 0.0f32;
    let mut sum_rr = 0.0f32;

    for frame in pcm_stereo.chunks_exact(2) {
        let l = frame[0];
        let r = frame[1];
        sum_lr += l * r;
        sum_ll += l * l;
        sum_rr += r * r;
    }

    let denom = (sum_ll * sum_rr).sqrt();
    if denom > 1e-6 {
        (sum_lr / denom).clamp(-1.0, 1.0)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_phase_stereo_gives_coherence_near_one() {
        let mut engine = AuraphyxEngine::new();
        let pcm: Vec<f32> = (0..64)
            .map(|i| (i as f32 * 0.1).sin())
            .flat_map(|s| [s, s]) // identical L/R
            .collect();
        let frame = engine.process(&pcm, &[0.0; 4], 1.0 / 60.0);
        assert!(frame.phase_align > 0.99);
    }

    #[test]
    fn fully_cancelling_stereo_gives_coherence_near_negative_one() {
        let mut engine = AuraphyxEngine::new();
        let pcm: Vec<f32> = (0..64)
            .map(|i| (i as f32 * 0.1).sin())
            .flat_map(|s| [s, -s]) // inverted R
            .collect();
        let frame = engine.process(&pcm, &[0.0; 4], 1.0 / 60.0);
        assert!(frame.phase_align < -0.99);
    }

    #[test]
    fn chromatic_bands_map_to_tsl_axes() {
        let mut engine = AuraphyxEngine::new();
        let chrom = [0.2, 0.4, 0.6, 0.8];
        let frame = engine.process(&[], &chrom, 1.0 / 60.0);
        assert!((frame.tsl_x - 0.3).abs() < 1e-6); // (0.2+0.4)/2
        assert_eq!(frame.tsl_y, 0.6);
        assert_eq!(frame.tsl_z, 0.8);
    }

    #[test]
    fn lattice_rotation_accumulates_over_time() {
        let mut engine = AuraphyxEngine::new();
        let pcm: Vec<f32> = (0..64).map(|i| (i as f32 * 0.1).sin()).flat_map(|s| [s, s]).collect();
        let f1 = engine.process(&pcm, &[0.0; 4], 1.0);
        let f2 = engine.process(&pcm, &[0.0; 4], 1.0);
        assert!(f2.lattice_rot.abs() > f1.lattice_rot.abs());
    }
}
