//! Port of `src/vap_loader.c` / `vap_loader.h` — `.vap.json` sidecar & ID3 loader.
//!
//! Priority chain (per `vap/vap_schema.json` and V.A.P. spec §3.1-3.2):
//!   1. `<audio_path>.vap.json` sidecar alongside the file
//!   2. `~/Music/vap/<title>.vap.json`, `<title>` = audio filename stem
//!   3. ID3v2 TXXX frame with description `"VAP_OBJECT"`
//!   4. [`VapRuntime::default()`] — safe neutral defaults, never a hard error

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde_json::Value;

use crate::vap_affective::ResolutionState;
use crate::vap_photometric::FadeMode;
use crate::vap_runtime::{ExplicitTier, SpatialWidth, VapRuntime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadSource {
    Sidecar,
    MusicDir,
    Id3,
    Defaults,
}

impl LoadSource {
    pub fn description(&self) -> &'static str {
        match self {
            LoadSource::Sidecar => "VAP v3.1 loaded from .vap.json sidecar",
            LoadSource::MusicDir => "VAP v3.1 loaded from ~/Music/vap sidecar",
            LoadSource::Id3 => "VAP v3.1 loaded from ID3v2 TXXX frame",
            LoadSource::Defaults => "No VAP data found; safe defaults used",
        }
    }
}

/// Master entry point. Given the path of the audio file currently playing,
/// tries each source in priority order and never hard-fails: on total
/// failure returns [`VapRuntime::default()`] tagged as [`LoadSource::Defaults`].
pub fn load(audio_path: &Path) -> (VapRuntime, LoadSource) {
    if let Some(json_src) = try_sidecar(audio_path) {
        let mut vap = VapRuntime::default();
        if parse_json(&mut vap, &json_src).is_ok() {
            vap.vap_loaded = true;
            return (vap, LoadSource::Sidecar);
        }
    }

    if let Some(json_src) = try_music_dir(audio_path) {
        let mut vap = VapRuntime::default();
        if parse_json(&mut vap, &json_src).is_ok() {
            vap.vap_loaded = true;
            return (vap, LoadSource::MusicDir);
        }
    }

    if let Some(json_src) = try_id3(audio_path) {
        let mut vap = VapRuntime::default();
        if parse_json(&mut vap, &json_src).is_ok() {
            vap.vap_loaded = true;
            return (vap, LoadSource::Id3);
        }
    }

    (VapRuntime::default(), LoadSource::Defaults)
}

fn try_sidecar(audio_path: &Path) -> Option<String> {
    let mut sidecar = audio_path.as_os_str().to_owned();
    sidecar.push(".vap.json");
    std::fs::read_to_string(PathBuf::from(sidecar)).ok()
}

fn try_music_dir(audio_path: &Path) -> Option<String> {
    let title = audio_path.file_stem()?.to_str()?;
    let home = home_dir()?;
    let candidate = home.join("Music").join("vap").join(format!("{title}.vap.json"));
    std::fs::read_to_string(candidate).ok()
}

fn try_id3(audio_path: &Path) -> Option<String> {
    let tag = id3::Tag::read_from_path(audio_path).ok()?;
    for frame in tag.frames() {
        if let Some(content) = frame.content().extended_text() {
            if content.description == "VAP_OBJECT" {
                return Some(content.value.clone());
            }
        }
    }
    None
}

fn home_dir() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("HOME") {
        return Some(PathBuf::from(p));
    }
    if let Ok(p) = std::env::var("USERPROFILE") {
        return Some(PathBuf::from(p));
    }
    None
}

/// Parse a `.vap.json` document (from any source) directly into a [`VapRuntime`].
/// Exposed publicly so the ID3 path shares it with the sidecar path.
pub fn parse_json(vap: &mut VapRuntime, json_src: &str) -> Result<()> {
    let root: Value = serde_json::from_str(json_src).context("VAP JSON is not valid JSON")?;

    let version = root
        .get("VAP_VERSION")
        .and_then(Value::as_str)
        .context("VAP JSON missing VAP_VERSION")?;
    if version != "3.1" {
        bail!("VAP_VERSION mismatch: expected \"3.1\", found \"{version}\"");
    }

    let identity = root.get("IDENTITY").context("VAP JSON missing IDENTITY")?;
    vap.title = str_field(identity, "TITLE").unwrap_or_else(|| vap.title.clone());
    vap.artist = str_field(identity, "ARTIST").unwrap_or_else(|| vap.artist.clone());

    let pillars = root.get("PILLARS").context("VAP JSON missing PILLARS")?;

    if let Some(s) = pillars.get("STRUCTURAL") {
        if let Some(bpm) = f32_field(s, "BPM_RAW") {
            vap.bpm_raw = bpm;
            vap.bpm_perceived = bpm;
        }
        if let Some(groove) = str_field(s, "GROOVE_QUANTIZATION") {
            // "MACHINE_LOCK" -> 0.0, "HUMAN_SWING" -> 1.0, else leave as-is
            vap.groove_quantization = match groove.to_uppercase().as_str() {
                "MACHINE_LOCK" => 0.0,
                "HUMAN_SWING" => 1.0,
                _ => vap.groove_quantization,
            };
        }
    }

    if let Some(t) = pillars.get("TONAL") {
        if let Some(key) = str_field(t, "KEY") {
            vap.key = key;
        }
        if let Some(d) = f32_field(t, "DISSONANCE_RATING") {
            vap.dissonance_density = d;
        }
    }

    if let Some(t) = pillars.get("TIMBRAL") {
        if let Some(fidelity) = str_field(t, "FIDELITY") {
            let _ = fidelity; // descriptive only; no numeric runtime field yet
        }
    }

    if let Some(l) = pillars.get("LINGUISTIC") {
        if let Some(tier) = str_field(l, "EXPLICIT_TIER") {
            vap.explicit_tier = match tier.as_str() {
                "CLEAN" => ExplicitTier::Clean,
                "MILD" => ExplicitTier::Mild,
                "EXPLICIT" => ExplicitTier::Explicit,
                "SEVERE" => ExplicitTier::Severe,
                _ => vap.explicit_tier,
            };
        }
    }

    if let Some(a) = pillars.get("AFFECTIVE") {
        if let Some(v) = f32_field(a, "VALENCE") {
            vap.affective.valence = v;
        }
        if let Some(v) = f32_field(a, "AROUSAL") {
            vap.affective.arousal = v;
        }
        if let Some(v) = f32_field(a, "DOMINANCE") {
            vap.affective.dominance = v;
        }
        if let Some(v) = f32_field(a, "MOOD_STABILITY") {
            vap.affective.mood_stability = v;
        }
        if let Some(v) = f32_field(a, "CATHARSIS_POTENTIAL") {
            vap.affective.catharsis_potential = v;
        }
        if let Some(v) = f32_field(a, "NOSTALGIA_TRIGGER") {
            vap.affective.nostalgia_trigger = v;
        }
        if let Some(v) = f32_field(a, "BUILDUP_VELOCITY") {
            vap.affective.buildup_velocity = v;
        }
        if let Some(s) = str_field(a, "RESOLUTION_STATE") {
            vap.affective.resolution_state = match s.as_str() {
                "TRIUMPHANT" => ResolutionState::Triumphant,
                "MELANCHOLIC" => ResolutionState::Melancholic,
                "UNRESOLVED" => ResolutionState::Unresolved,
                _ => vap.affective.resolution_state,
            };
        }
    }

    if let Some(c) = pillars.get("CONTEXTUAL") {
        if let Some(tag) = str_field(c, "SCENARIO_TAG") {
            vap.scenario_tag = tag;
        }
        if let Some(conf) = f32_field(c, "SCENARIO_CONFIDENCE") {
            vap.scenario_confidence = conf;
        }
        if let Some(fog) = f32_field(c, "CONTEXTUAL_FOG_MOD") {
            vap.contextual_fog_mod = fog;
        }
    }

    if let Some(p) = pillars.get("PHOTOMETRIC") {
        if let Some(hex) = str_field(p, "PRIMARY_HEX") {
            if let Some(rgb) = parse_hex_rgb(&hex) {
                vap.photometric.primary_hex = rgb;
            }
        }
        if let Some(hex) = str_field(p, "SECONDARY_HEX") {
            if let Some(rgb) = parse_hex_rgb(&hex) {
                vap.photometric.secondary_hex = rgb;
            }
        }
        if let Some(t) = str_field(p, "PALETTE_TEMP") {
            vap.photometric.palette_temp = match t.to_uppercase().as_str() {
                "COOL" => 0.0,
                "WARM" => 1.0,
                _ => t.parse().unwrap_or(vap.photometric.palette_temp),
            };
        }
        if let Some(v) = f32_field(p, "BRIGHTNESS_FLOOR") {
            vap.photometric.brightness_floor = v;
        }
        if let Some(v) = f32_field(p, "BRIGHTNESS_CEILING") {
            vap.photometric.brightness_ceiling = v;
        }
        if let Some(v) = f32_field(p, "STROBE_THRESHOLD") {
            vap.photometric.strobe_threshold = v;
        }
        if let Some(s) = str_field(p, "FADE_MODE") {
            vap.photometric.fade_mode = match s.to_uppercase().as_str() {
                "SHARP" => FadeMode::Sharp,
                "LINEAR" => FadeMode::Linear,
                _ => vap.photometric.fade_mode,
            };
        }
        if let Some(v) = f32_field(p, "FOG_DENSITY") {
            vap.photometric.fog_density = v;
        }
        if let Some(v) = bool_field(p, "LASER_COMPATIBLE") {
            vap.photometric.laser_compatible = v;
        }
        if let Some(v) = bool_field(p, "VISUAL_NOISE_MODE") {
            vap.photometric.visual_noise_mode = v;
        }
    }

    if let Some(k) = pillars.get("KINETIC") {
        if let Some(zone) = str_field(k, "TARGET_HR_ZONE") {
            vap.target_hr_zone = zone;
        }
        if let Some(met) = f32_field(k, "MET_SCORE") {
            vap.met_score = met;
        }
        if let Some(e) = f32_field(k, "ENTRAINMENT_FACTOR") {
            vap.entrainment_factor = e;
        }
    }

    if let Some(g) = pillars.get("GENEALOGICAL") {
        if let Some(v) = f32_field(g, "TIMELESSNESS_SCORE") {
            vap.timelessness_score = v;
        }
        if let Some(v) = f32_field(g, "AUTHENTICITY_RATIO") {
            vap.authenticity_ratio = v;
        }
        if let Some(v) = f32_field(g, "VIRAL_VELOCITY") {
            vap.viral_velocity = v;
        }
    }

    let _ = SpatialWidth::Stereo; // silence unused-import lint if no JSON path sets it yet

    Ok(())
}

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::to_owned)
}

fn f32_field(v: &Value, key: &str) -> Option<f32> {
    v.get(key).and_then(Value::as_f64).map(|f| f as f32)
}

fn bool_field(v: &Value, key: &str) -> Option<bool> {
    v.get(key).and_then(Value::as_bool)
}

/// Parse `"#RRGGBB"` into 0.0-1.0 floats.
fn parse_hex_rgb(hex: &str) -> Option<[f32; 3]> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0])
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_VAP_JSON: &str = r##"{
        "VAP_VERSION": "3.1",
        "IDENTITY": { "TITLE": "Test Track", "ARTIST": "Test Artist" },
        "PILLARS": {
            "STRUCTURAL": { "BPM_RAW": 128.0 },
            "AFFECTIVE": { "VALENCE": 0.7, "AROUSAL": 0.9 },
            "PHOTOMETRIC": { "PRIMARY_HEX": "#FF8800" },
            "KINETIC": { "MET_SCORE": 6.0 }
        }
    }"##;

    #[test]
    fn parses_minimal_document() {
        let mut vap = VapRuntime::default();
        parse_json(&mut vap, MINIMAL_VAP_JSON).expect("should parse");

        assert_eq!(vap.title, "Test Track");
        assert_eq!(vap.artist, "Test Artist");
        assert_eq!(vap.bpm_raw, 128.0);
        assert_eq!(vap.affective.valence, 0.7);
        assert_eq!(vap.affective.arousal, 0.9);
        assert_eq!(vap.photometric.primary_hex, [1.0, 0x88 as f32 / 255.0, 0.0]);
        assert_eq!(vap.met_score, 6.0);
    }

    #[test]
    fn rejects_wrong_version() {
        let mut vap = VapRuntime::default();
        let bad = MINIMAL_VAP_JSON.replace("3.1", "2.0");
        assert!(parse_json(&mut vap, &bad).is_err());
    }

    #[test]
    fn hex_rgb_roundtrip() {
        assert_eq!(parse_hex_rgb("#7B14C8"), Some([0x7B as f32 / 255.0, 0x14 as f32 / 255.0, 0xC8 as f32 / 255.0]));
        assert_eq!(parse_hex_rgb("not-a-color"), None);
    }
}
