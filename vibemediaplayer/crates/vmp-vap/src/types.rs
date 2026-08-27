//! VASP v3.69 serde types — flexible enough for nested golden-set and flat engine JSON.

use crate::error::{VapError, VapResult};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Stable pillar identifier for UI tabs and runtime routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PillarId {
    Structural,
    Tonal,
    Timbral,
    Linguistic,
    Affective,
    Contextual,
    Photometric,
    Kinetic,
    Genealogical,
}

impl PillarId {
    pub fn index(self) -> u8 {
        match self {
            Self::Structural => 1,
            Self::Tonal => 2,
            Self::Timbral => 3,
            Self::Linguistic => 4,
            Self::Affective => 5,
            Self::Contextual => 6,
            Self::Photometric => 7,
            Self::Kinetic => 8,
            Self::Genealogical => 9,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Structural => "Structural",
            Self::Tonal => "Tonal",
            Self::Timbral => "Timbral",
            Self::Linguistic => "Linguistic",
            Self::Affective => "Affective",
            Self::Contextual => "Contextual",
            Self::Photometric => "Photometric",
            Self::Kinetic => "Kinetic",
            Self::Genealogical => "Genealogical",
        }
    }

    pub fn all() -> [PillarId; 9] {
        [
            Self::Structural,
            Self::Tonal,
            Self::Timbral,
            Self::Linguistic,
            Self::Affective,
            Self::Contextual,
            Self::Photometric,
            Self::Kinetic,
            Self::Genealogical,
        ]
    }
}

/// Number-or-string helper for mixed golden-set payloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumOrStr {
    Num(f64),
    Str(String),
}

impl NumOrStr {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Num(n) => Some(*n),
            Self::Str(s) => s.parse().ok(),
        }
    }

    pub fn as_str_owned(&self) -> String {
        match self {
            Self::Num(n) => n.to_string(),
            Self::Str(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VapObject {
    #[serde(rename = "VASP_VERSION")]
    pub vasp_version: String,
    #[serde(rename = "IDENTITY")]
    pub identity: Identity,
    #[serde(rename = "PILLARS")]
    pub pillars: Pillars,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "TITLE")]
    pub title: String,
    #[serde(rename = "ARTIST")]
    pub artist: String,
    #[serde(rename = "ISRC", default)]
    pub isrc: Option<String>,
    #[serde(rename = "SOURCE_DNA", default)]
    pub source_dna: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pillars {
    #[serde(rename = "STRUCTURAL", default)]
    pub structural: Option<serde_json::Value>,
    #[serde(rename = "TONAL", default)]
    pub tonal: Option<serde_json::Value>,
    #[serde(rename = "TIMBRAL", default)]
    pub timbral: Option<serde_json::Value>,
    #[serde(rename = "LINGUISTIC", default)]
    pub linguistic: Option<serde_json::Value>,
    #[serde(rename = "AFFECTIVE", default)]
    pub affective: Option<serde_json::Value>,
    #[serde(rename = "CONTEXTUAL", default)]
    pub contextual: Option<serde_json::Value>,
    #[serde(rename = "PHOTOMETRIC", default)]
    pub photometric: Option<serde_json::Value>,
    #[serde(rename = "KINETIC", default)]
    pub kinetic: Option<serde_json::Value>,
    #[serde(rename = "GENEALOGICAL", default)]
    pub genealogical: Option<serde_json::Value>,
}

impl VapObject {
    pub fn from_str(s: &str) -> VapResult<Self> {
        let v: Self = serde_json::from_str(s)?;
        v.validate_version()?;
        Ok(v)
    }

    pub fn from_path(path: impl AsRef<Path>) -> VapResult<Self> {
        let raw = std::fs::read_to_string(path)?;
        Self::from_str(&raw)
    }

    pub fn to_pretty_json(&self) -> VapResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn save_path(&self, path: impl AsRef<Path>) -> VapResult<()> {
        std::fs::write(path, self.to_pretty_json()?)?;
        Ok(())
    }

    pub fn validate_version(&self) -> VapResult<()> {
        if self.vasp_version != crate::VASP_VERSION {
            return Err(VapError::VersionMismatch(self.vasp_version.clone()));
        }
        Ok(())
    }

    /// Neutral defaults when no VAP sidecar/embed exists (spec §3.2).
    pub fn defaults(artist: &str, title: &str) -> Self {
        Self {
            vasp_version: crate::VASP_VERSION.to_string(),
            identity: Identity {
                title: title.to_string(),
                artist: artist.to_string(),
                isrc: None,
                source_dna: Some("defaults".into()),
            },
            pillars: Pillars {
                structural: Some(serde_json::json!({
                    "TEMPORAL_DYNAMICS": {
                        "BPM_RAW": 120.0,
                        "BPM_PERCEIVED": "1.0x",
                        "GROOVE_QUANTIZATION": "machine_lock",
                        "TIME_SIGNATURE": "4/4"
                    },
                    "PERCUSSIVE_DNA": {
                        "KICK_TRANSIENT": "Punch (Thud)",
                        "KICK_ATTACK_MS": 20.0,
                        "SYNCOPATION_INDEX": 0.2
                    }
                })),
                tonal: Some(serde_json::json!({
                    "HARMONIC_PROFILE": {
                        "KEY": "C Major",
                        "CHORD_COMPLEXITY": "triadic",
                        "DISSONANCE_RATING": 0.1
                    },
                    "TUNING": { "REFERENCE_PITCH_HZ": 440, "MICROTONALITY": false }
                })),
                timbral: Some(serde_json::json!({
                    "SPECTRAL_PHYSICS": {
                        "FREQ_BALANCE": "balanced",
                        "SPECTRAL_CENTROID_HZ": 1000.0,
                        "SATURATION_INDEX": 0.15
                    },
                    "PRODUCTION_AESTHETIC": {
                        "FIDELITY": "hi-fi",
                        "DYNAMIC_RANGE_LRA": 10.0,
                        "SPATIAL_WIDTH": "stereo"
                    }
                })),
                linguistic: Some(serde_json::json!({
                    "SEMANTIC_CONTENT": { "EXPLICIT_TIER": "CLEAN", "TOPIC": "unknown" },
                    "LANGUAGE": { "PRIMARY": "und" }
                })),
                affective: Some(serde_json::json!({
                    "THAYER_COORDINATES": {
                        "VALENCE": 0.0,
                        "AROUSAL": 0.5,
                        "DOMINANCE": 0.5
                    }
                })),
                contextual: Some(serde_json::json!({
                    "SCENARIO_ENGINE": { "MACRO": "any", "MICRO": "listening" }
                })),
                photometric: Some(serde_json::json!({
                    "CHROMATIC_MAP": {
                        "PRIMARY_HEX": "#4B0082",
                        "SECONDARY_HEX": "#1a1a2e",
                        "PALETTE_TEMP": "cool"
                    },
                    "LUMEN_DYNAMICS": {
                        "BRIGHTNESS_FLOOR": 0.05,
                        "BRIGHTNESS_CEILING": 1.0,
                        "STROBE_TRIGGER": 1.0,
                        "FADE_RATE": "smooth"
                    },
                    "VISUAL_TEXTURE": {
                        "FOG_DENSITY": 0.2,
                        "LASER_COMPATIBLE": false,
                        "VISUAL_NOISE": 0.0
                    }
                })),
                kinetic: Some(serde_json::json!({
                    "BIOMETRIC_ENTRAINMENT": {
                        "TARGET_HR_ZONE": "100-120",
                        "HRV_IMPACT": "neutral"
                    },
                    "ENERGY": { "MET_SCORE": 3.0, "ENTRAINMENT_FACTOR": 40.0 }
                })),
                genealogical: Some(serde_json::json!({
                    "ERA_ANCHORING": { "CULTURAL_ERA": "unknown", "TIMELESSNESS_SCORE": 0.5 },
                    "TRIBE_ALIGNMENT": { "SUBCULTURE": "general", "AUTHENTICITY_SCORE": 0.5 }
                })),
            },
        }
    }

    /// Extract BPM from nested or flat STRUCTURAL pillar.
    pub fn bpm(&self) -> Option<f64> {
        let s = self.pillars.structural.as_ref()?;
        s.pointer("/TEMPORAL_DYNAMICS/BPM_RAW")
            .or_else(|| s.get("BPM_RAW"))
            .or_else(|| s.get("BPM"))
            .and_then(|v| v.as_f64())
    }

    pub fn valence(&self) -> Option<f64> {
        let a = self.pillars.affective.as_ref()?;
        a.pointer("/THAYER_COORDINATES/VALENCE")
            .or_else(|| a.get("VALENCE"))
            .and_then(|v| v.as_f64())
    }

    pub fn arousal(&self) -> Option<f64> {
        let a = self.pillars.affective.as_ref()?;
        a.pointer("/THAYER_COORDINATES/AROUSAL")
            .or_else(|| a.get("AROUSAL"))
            .and_then(|v| v.as_f64())
    }

    pub fn met_score(&self) -> Option<f64> {
        let k = self.pillars.kinetic.as_ref()?;
        k.pointer("/ENERGY/MET_SCORE")
            .or_else(|| k.get("MET_SCORE"))
            .and_then(|v| v.as_f64())
    }

    pub fn primary_hex(&self) -> Option<String> {
        let p = self.pillars.photometric.as_ref()?;
        p.pointer("/CHROMATIC_MAP/PRIMARY_HEX")
            .or_else(|| p.get("PRIMARY_HEX"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    pub fn palette_temp(&self) -> Option<String> {
        let p = self.pillars.photometric.as_ref()?;
        p.pointer("/CHROMATIC_MAP/PALETTE_TEMP")
            .or_else(|| p.get("PALETTE_TEMP"))
            .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| {
                v.as_f64().map(|n| n.to_string())
            }))
    }

    /// JSON value for a single pillar tab body.
    pub fn pillar_value(&self, id: PillarId) -> Option<&serde_json::Value> {
        match id {
            PillarId::Structural => self.pillars.structural.as_ref(),
            PillarId::Tonal => self.pillars.tonal.as_ref(),
            PillarId::Timbral => self.pillars.timbral.as_ref(),
            PillarId::Linguistic => self.pillars.linguistic.as_ref(),
            PillarId::Affective => self.pillars.affective.as_ref(),
            PillarId::Contextual => self.pillars.contextual.as_ref(),
            PillarId::Photometric => self.pillars.photometric.as_ref(),
            PillarId::Kinetic => self.pillars.kinetic.as_ref(),
            PillarId::Genealogical => self.pillars.genealogical.as_ref(),
        }
    }

    pub fn set_pillar(&mut self, id: PillarId, value: serde_json::Value) {
        match id {
            PillarId::Structural => self.pillars.structural = Some(value),
            PillarId::Tonal => self.pillars.tonal = Some(value),
            PillarId::Timbral => self.pillars.timbral = Some(value),
            PillarId::Linguistic => self.pillars.linguistic = Some(value),
            PillarId::Affective => self.pillars.affective = Some(value),
            PillarId::Contextual => self.pillars.contextual = Some(value),
            PillarId::Photometric => self.pillars.photometric = Some(value),
            PillarId::Kinetic => self.pillars.kinetic = Some(value),
            PillarId::Genealogical => self.pillars.genealogical = Some(value),
        }
    }
}

/// Live Phase-I fields updated every audio buffer (visualizer two-track model).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VapLiveState {
    pub bpm_raw: f32,
    pub groove_quantization: f32,
    pub syncopation_index: f32,
    pub kick_transient_ms: f32,
    pub spectral_centroid_hz: f32,
    pub saturation_index: f32,
    pub dissonance_density: f32,
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub entrainment_factor: f32,
    pub met_score: f32,
    pub chrom_energy: [f32; 4],
    pub phase_time: f32,
    pub frame_count: u64,
}

/// Thayer coordinates result from scoring engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThayerResult {
    pub valence: f64,
    pub arousal: f64,
    pub mood_quadrant: String,
}
