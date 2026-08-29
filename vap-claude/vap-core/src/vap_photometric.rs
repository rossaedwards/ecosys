//! Port of `vap/vap_photometric.h` — V.A.P. v3.1 Pillar 7: PHOTOMETRIC (The Eye)
//! Per VAP Logic Architecture: frequency -> wavelength -> RGB mapping.

use serde::Deserialize;

/// (freq_low_hz, freq_high_hz, wavelength_nm, r, g, b)
///
/// SACRED — must match `VAP_CHROMATIC_MAP` in `vap_photometric.h` byte-for-byte.
pub const CHROMATIC_MAP: [(f32, f32, f32, f32, f32, f32); 4] = [
    (40.0, 60.0, 700.0, 0.85, 0.05, 0.05),      // Deep Red
    (60.0, 250.0, 600.0, 1.00, 0.55, 0.00),     // Orange/Amber
    (250.0, 2000.0, 520.0, 0.10, 0.75, 0.55),   // Green/Teal
    (2000.0, 20000.0, 450.0, 0.30, 0.15, 0.95), // Blue/UV
];

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FadeMode {
    Sharp,
    #[default]
    Linear,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VapPhotometric {
    // 7.1 Chromatic Map
    //
    // NOTE: `vap_photometric.h` declares `primary_hex`/`secondary_hex` as
    // `uint8_t[3]`, but `vap_runtime.c` (init) and `gl_renderer.c` (uniform
    // upload via glUniform3f) both treat them as floats in 0.0-1.0 — the
    // header type is stale relative to actual usage. Ported here as f32 to
    // match the real behavior the shader depends on.
    pub primary_hex: [f32; 3],
    pub secondary_hex: [f32; 3],
    /// 0.0 = Cool (Blues), 1.0 = Warm (Reds)
    pub palette_temp: f32,

    // 7.2 Lumen Dynamics
    pub brightness_floor: f32,
    pub brightness_ceiling: f32,
    pub strobe_threshold: f32,
    pub fade_mode: FadeMode,
    pub fade_rate: f32,

    // 7.3 Visual Texture
    pub fog_density: f32,
    pub laser_compatible: bool,
    pub visual_noise_mode: bool,
}

impl Default for VapPhotometric {
    fn default() -> Self {
        VapPhotometric {
            // Primary: Aurphyx Violet #7B14C8
            primary_hex: [0.482, 0.078, 0.784],
            // Secondary: Bliss Gold #FFD700
            secondary_hex: [1.000, 0.843, 0.000],
            palette_temp: 0.5,
            brightness_floor: 0.05,
            brightness_ceiling: 1.00,
            strobe_threshold: 1.00, // 1.0 = disabled
            fade_mode: FadeMode::Linear,
            fade_rate: 0.30,
            fog_density: 0.10,
            laser_compatible: false,
            visual_noise_mode: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SACRED — must match `VAP_CHROMATIC_MAP` in `vap_photometric.h` exactly.
    #[test]
    fn chromatic_map_matches_c_reference_byte_for_byte() {
        assert_eq!(
            CHROMATIC_MAP,
            [
                (40.0, 60.0, 700.0, 0.85, 0.05, 0.05),
                (60.0, 250.0, 600.0, 1.00, 0.55, 0.00),
                (250.0, 2000.0, 520.0, 0.10, 0.75, 0.55),
                (2000.0, 20000.0, 450.0, 0.30, 0.15, 0.95),
            ]
        );
    }
}
