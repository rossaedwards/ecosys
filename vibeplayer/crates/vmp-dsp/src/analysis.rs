//! Phase-I analysis helpers (shared math with VAP scoring labels).

/// Kick transient profile label from attack time (ms).
pub fn kick_profile_label(attack_ms: f32) -> &'static str {
    if attack_ms < 10.0 {
        "Sharp (Click)"
    } else if attack_ms <= 30.0 {
        "Punch (Thud)"
    } else {
        "Boom (Sub)"
    }
}

/// Spectral class from centroid Hz.
pub fn spectral_class(centroid_hz: f32) -> &'static str {
    if centroid_hz < 200.0 {
        "Dark/Muddy"
    } else if centroid_hz <= 2000.0 {
        "Warm/Body"
    } else {
        "Bright/Airy"
    }
}

/// Spectral centroid of magnitude spectrum.
pub fn spectral_centroid(mag: &[f32], sample_rate: f32) -> f32 {
    if mag.is_empty() {
        return 0.0;
    }
    let n = mag.len();
    let bin_hz = sample_rate / (n as f32 * 2.0);
    let mut num = 0.0_f32;
    let mut den = 0.0_f32;
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

/// THD-like saturation proxy: harmonic energy / total (bins 1..10).
pub fn saturation_index(mag: &[f32]) -> f32 {
    if mag.len() < 4 {
        return 0.0;
    }
    let fundamental = mag[1];
    let mut harmonics = 0.0_f32;
    for m in mag.iter().take(10.min(mag.len())).skip(2) {
        harmonics += *m;
    }
    let total = fundamental + harmonics;
    if total > 1e-6 {
        (harmonics / total).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// Rough syncopation proxy: upper-half vs total magnitude energy.
pub fn syncopation_proxy(mag: &[f32]) -> f32 {
    if mag.is_empty() {
        return 0.0;
    }
    let half = mag.len() / 2;
    let even: f32 = mag[..half].iter().sum();
    let odd: f32 = mag[half..].iter().sum();
    let total = even + odd;
    if total > 1e-6 {
        (odd / total).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// MET score from BPM (Pillar 8 mapping).
pub fn met_from_bpm(bpm: f32) -> f32 {
    if bpm < 60.0 {
        1.0
    } else if bpm < 100.0 {
        3.0
    } else if bpm < 140.0 {
        6.0
    } else {
        8.0
    }
}

/// VAP chromatic band energies: Sub / Low-Mid / Mid / High.
pub fn chromatic_band_energies(mag: &[f32], sample_rate: f32) -> [f32; 4] {
    let mut bands = [0.0_f32; 4];
    if mag.is_empty() {
        return bands;
    }
    let bin_hz = sample_rate / (mag.len() as f32 * 2.0);
    for (i, &m) in mag.iter().enumerate() {
        let f = i as f32 * bin_hz;
        let idx = if f < 60.0 {
            0
        } else if f < 250.0 {
            1
        } else if f < 2000.0 {
            2
        } else {
            3
        };
        bands[idx] += m;
    }
    let max = bands.iter().cloned().fold(0.0_f32, f32::max).max(1e-6);
    for b in &mut bands {
        *b /= max;
    }
    bands
}

/// RMS of a buffer.
pub fn rms(buf: &[f32]) -> f32 {
    if buf.is_empty() {
        return 0.0;
    }
    let sum: f32 = buf.iter().map(|x| x * x).sum();
    (sum / buf.len() as f32).sqrt()
}
