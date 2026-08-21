//! Vibe Media Player DSP — equalizer modes, analysis helpers, Phase-I math.

mod eq;
mod analysis;

pub use analysis::*;
pub use eq::*;

/// Standard 10-band graphic EQ center frequencies (Hz), matching `vmp.html` prototype.
pub const GRAPHIC_10_BANDS_HZ: [f32; 10] =
    [32.0, 63.0, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0];

/// ISO-style 31-band centers (approximation) for pro mode.
pub const GRAPHIC_31_BANDS_HZ: [f32; 31] = [
    20.0, 25.0, 31.5, 40.0, 50.0, 63.0, 80.0, 100.0, 125.0, 160.0, 200.0, 250.0, 315.0, 400.0,
    500.0, 630.0, 800.0, 1000.0, 1250.0, 1600.0, 2000.0, 2500.0, 3150.0, 4000.0, 5000.0, 6300.0,
    8000.0, 10000.0, 12500.0, 16000.0, 20000.0,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biquad_peaking_processes() {
        let mut f = Biquad::peaking(1000.0, 48000.0, 1.0, 6.0);
        let y = f.process(0.5);
        assert!(y.is_finite());
    }

    #[test]
    fn graphic10_default_flat() {
        let mut eq = GraphicEq::new_10(48000.0);
        assert_eq!(eq.band_count(), 10);
        let mut x = 0.25_f32;
        x = eq.process_sample_mut(x);
        assert!(x.is_finite());
    }

    #[test]
    fn kick_and_centroid_helpers() {
        assert_eq!(kick_profile_label(8.0), "Sharp (Click)");
        assert_eq!(spectral_class(1500.0), "Warm/Body");
    }
}
