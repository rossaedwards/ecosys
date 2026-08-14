//! Reference scoring from V.A.P. Logic Architecture / Scoring Engine.

use crate::types::{ThayerResult, VapObject};
use serde_json::json;

/// Port of the Python `VAPScoringEngine` reference implementation.
pub struct VapScoringEngine {
    pub version: String,
}

impl Default for VapScoringEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl VapScoringEngine {
    pub fn new() -> Self {
        Self {
            version: crate::VAP_VERSION.to_string(),
        }
    }

    /// Kick transient profile from attack time (ms).
    /// <10 Sharp, 10–30 Punch, >30 Boom.
    pub fn kick_profile(&self, attack_ms: f64) -> &'static str {
        if attack_ms < 10.0 {
            "Sharp (Click)"
        } else if attack_ms <= 30.0 {
            "Punch (Thud)"
        } else {
            "Boom (Sub)"
        }
    }

    /// Spectral centroid → Dark / Warm / Bright.
    pub fn spectral_color(&self, centroid_hz: f64) -> &'static str {
        if centroid_hz < 200.0 {
            "Dark/Muddy"
        } else if centroid_hz <= 2000.0 {
            "Warm/Body"
        } else {
            "Bright/Airy"
        }
    }

    /// Thayer valence/arousal from key mode, NLP sentiment, RMS.
    pub fn thayer_coordinates(
        &self,
        key_mode: &str,
        sentiment_score: f64,
        rms_amplitude: f64,
    ) -> ThayerResult {
        let base_valence = if key_mode.eq_ignore_ascii_case("Major") {
            0.5
        } else {
            -0.5
        };
        let mut valence = (base_valence + sentiment_score) / 2.0;
        valence = valence.clamp(-1.0, 1.0);
        let arousal = rms_amplitude.clamp(0.0, 1.0);
        let mood_quadrant = Self::quadrant(valence, arousal).to_string();
        ThayerResult {
            valence: (valence * 100.0).round() / 100.0,
            arousal: (arousal * 100.0).round() / 100.0,
            mood_quadrant,
        }
    }

    fn quadrant(valence: f64, arousal: f64) -> &'static str {
        if valence > 0.0 && arousal > 0.5 {
            "Euphoria/Joy"
        } else if valence > 0.0 && arousal <= 0.5 {
            "Calm/Content"
        } else if valence <= 0.0 && arousal > 0.5 {
            "Anger/Fear"
        } else {
            "Depression/Melancholy"
        }
    }

    /// Frequency → photometric hex (Pillar 7 chromatic map approximation).
    pub fn photometric_hex(&self, dominant_freq_hz: f64) -> &'static str {
        if dominant_freq_hz < 60.0 {
            "#8B0000" // Deep Red (Sub Bass)
        } else if dominant_freq_hz < 250.0 {
            "#FF8C00" // Dark Orange
        } else if dominant_freq_hz < 2000.0 {
            "#008080" // Teal
        } else {
            "#4B0082" // Indigo/UV
        }
    }

    /// BPM → MET score (Pillar 8).
    pub fn kinetic_met(&self, bpm: f64) -> f64 {
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

    /// Build a partial VAP object from raw analysis numbers (mock/pipeline entry).
    pub fn generate_profile(
        &self,
        bpm: f64,
        attack_ms: f64,
        centroid_hz: f64,
        key_mode: &str,
        sentiment_score: f64,
        rms_amplitude: f64,
        dominant_freq_hz: f64,
        title: &str,
        artist: &str,
    ) -> VapObject {
        let kick = self.kick_profile(attack_ms);
        let spectral = self.spectral_color(centroid_hz);
        let thayer = self.thayer_coordinates(key_mode, sentiment_score, rms_amplitude);
        let hex = self.photometric_hex(dominant_freq_hz);
        let met = self.kinetic_met(bpm);
        let hr_lo = (bpm - 20.0).max(0.0) as i64;
        let hr_hi = (bpm + 10.0) as i64;

        VapObject {
            vap_version: self.version.clone(),
            identity: crate::types::Identity {
                title: title.into(),
                artist: artist.into(),
                isrc: None,
                source_dna: Some("scoring_engine".into()),
            },
            pillars: crate::types::Pillars {
                structural: Some(json!({
                    "PERCUSSIVE_DNA": {
                        "KICK_TRANSIENT": kick,
                        "KICK_ATTACK_MS": attack_ms
                    },
                    "TEMPORAL_DYNAMICS": { "BPM_RAW": bpm }
                })),
                tonal: Some(json!({
                    "HARMONIC_PROFILE": { "KEY": key_mode }
                })),
                timbral: Some(json!({
                    "SPECTRAL_PHYSICS": {
                        "CLASS": spectral,
                        "SPECTRAL_CENTROID_HZ": centroid_hz
                    }
                })),
                linguistic: None,
                affective: Some(json!({
                    "THAYER_COORDINATES": {
                        "VALENCE": thayer.valence,
                        "AROUSAL": thayer.arousal
                    },
                    "MOOD_QUADRANT": thayer.mood_quadrant
                })),
                contextual: None,
                photometric: Some(json!({
                    "CHROMATIC_MAP": { "PRIMARY_HEX": hex }
                })),
                kinetic: Some(json!({
                    "ENERGY": { "MET_SCORE": met },
                    "BIOMETRIC_ENTRAINMENT": {
                        "TARGET_HR_ZONE": format!("{hr_lo}-{hr_hi}")
                    }
                })),
                genealogical: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_after_dark_profile() {
        let eng = VapScoringEngine::new();
        let p = eng.generate_profile(
            135.0, 45.0, 2200.0, "Minor", -0.2, 0.75, 3000.0, "After Dark", "Mr.Kitty",
        );
        assert_eq!(p.identity.title, "After Dark");
        assert!((p.bpm().unwrap() - 135.0).abs() < 0.01);
        assert!(p.primary_hex().unwrap().starts_with('#'));
    }
}
