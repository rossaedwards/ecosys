//! Pillar 7 photometric map — from `vap/vap_photometric.h` (FUTE C→Rust).

use serde::{Deserialize, Serialize};

/// V.A.P. chromatic band (Hz → wavelength → RGB).
#[derive(Debug, Clone, Copy)]
pub struct ChromaticBand {
    pub freq_low_hz: f32,
    pub freq_high_hz: f32,
    pub wavelength_nm: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// Canonical V.A.P. §7.1 map (identical to vibe-audio-visualizer).
pub const VAP_CHROMATIC_MAP: [ChromaticBand; 4] = [
    ChromaticBand {
        freq_low_hz: 40.0,
        freq_high_hz: 60.0,
        wavelength_nm: 700.0,
        r: 0.85,
        g: 0.05,
        b: 0.05,
    },
    ChromaticBand {
        freq_low_hz: 60.0,
        freq_high_hz: 250.0,
        wavelength_nm: 600.0,
        r: 1.00,
        g: 0.55,
        b: 0.00,
    },
    ChromaticBand {
        freq_low_hz: 250.0,
        freq_high_hz: 2000.0,
        wavelength_nm: 520.0,
        r: 0.10,
        g: 0.75,
        b: 0.55,
    },
    ChromaticBand {
        freq_low_hz: 2000.0,
        freq_high_hz: 20000.0,
        wavelength_nm: 450.0,
        r: 0.30,
        g: 0.15,
        b: 0.95,
    },
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photometric {
    pub primary_rgb: [f32; 3],
    pub secondary_rgb: [f32; 3],
    pub palette_temp: f32,
    pub brightness_floor: f32,
    pub brightness_ceiling: f32,
    pub strobe_threshold: f32,
    pub fade_mode: i32,
    pub fade_rate: f32,
    pub fog_density: f32,
    pub laser_compatible: bool,
    pub visual_noise: f32,
    /// Live band energies [sub, low, mid, high]
    pub chrom_energy: [f32; 4],
}

impl Default for Photometric {
    fn default() -> Self {
        Self {
            // Aurphyx violet #7B14C8
            primary_rgb: [0.482, 0.078, 0.784],
            // Bliss gold #FFD700
            secondary_rgb: [1.0, 0.843, 0.0],
            palette_temp: 0.5,
            brightness_floor: 0.05,
            brightness_ceiling: 1.0,
            strobe_threshold: 1.0,
            fade_mode: 1,
            fade_rate: 0.30,
            fog_density: 0.10,
            laser_compatible: false,
            visual_noise: 0.0,
            chrom_energy: [0.0; 4],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affective {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub mood_stability: f32,
    pub catharsis_potential: f32,
    pub nostalgia_trigger: f32,
    pub buildup_velocity: f32,
    pub resolution_state: i32,
}

impl Default for Affective {
    fn default() -> Self {
        Self {
            valence: 0.0,
            arousal: 0.5,
            dominance: 0.5,
            mood_stability: 0.7,
            catharsis_potential: 0.3,
            nostalgia_trigger: 0.2,
            buildup_velocity: 0.4,
            resolution_state: 0,
        }
    }
}

impl Affective {
    /// Warmth factor for shader atmosphere (from vap_affective.h).
    pub fn warmth(&self) -> f32 {
        (self.valence * 0.5 + 0.5) * self.arousal
    }
}
