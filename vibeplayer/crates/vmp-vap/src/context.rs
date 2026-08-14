//! Contextual override engine — Scenario Computing layer (GYM_PEAK, NIGHT_DRIVE, …).

use crate::error::{VapError, VapResult};
use crate::types::VapObject;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContextStatus {
    Passthrough,
    Accepted,
    Borderline,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextVerdict {
    pub context: Option<String>,
    pub track_title: String,
    pub status: ContextStatus,
    pub compatibility_score: f64,
    pub overrides_applied: Vec<String>,
    pub rejection_log: Vec<String>,
    pub final_hex: Option<String>,
    pub target_hr: Option<f64>,
}

/// Alias for API clarity.
pub type ContextResult = ContextVerdict;

#[derive(Debug, Clone)]
struct ContextRules {
    min_bpm: Option<f64>,
    max_bpm: Option<f64>,
    min_arousal: Option<f64>,
    min_valence: Option<f64>,
    min_met: Option<f64>,
    lighting_modifier: Option<&'static str>,
    narrative_filter: Vec<&'static str>,
}

/// Filters and modifies track metadata based on active scenarios.
pub struct ContextEngine {
    active: Option<String>,
    rules: HashMap<String, ContextRules>,
}

impl Default for ContextEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextEngine {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        rules.insert(
            "GYM_PEAK".into(),
            ContextRules {
                min_bpm: Some(120.0),
                max_bpm: None,
                min_arousal: Some(0.7),
                min_valence: None,
                min_met: Some(6.0),
                lighting_modifier: Some("FORCE_RED_SHIFT"),
                narrative_filter: vec!["relax", "sleep", "sad"],
            },
        );
        rules.insert(
            "NIGHT_DRIVE".into(),
            ContextRules {
                min_bpm: None,
                max_bpm: Some(130.0),
                min_arousal: None,
                min_valence: Some(-0.5),
                min_met: None,
                lighting_modifier: Some("FORCE_COOL_SHIFT"),
                narrative_filter: vec!["hype", "scream"],
            },
        );
        rules.insert(
            "DEEP_WORK".into(),
            ContextRules {
                min_bpm: None,
                max_bpm: Some(110.0),
                min_arousal: None,
                min_valence: None,
                min_met: None,
                lighting_modifier: Some("FORCE_COOL_SHIFT"),
                narrative_filter: vec!["party", "scream"],
            },
        );
        Self {
            active: None,
            rules,
        }
    }

    pub fn set_context(&mut self, tag: &str) -> VapResult<()> {
        if self.rules.contains_key(tag) {
            self.active = Some(tag.to_string());
            Ok(())
        } else {
            Err(VapError::UnknownContext(tag.into()))
        }
    }

    pub fn clear_context(&mut self) {
        self.active = None;
    }

    pub fn active_context(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn known_contexts(&self) -> Vec<&str> {
        self.rules.keys().map(|s| s.as_str()).collect()
    }

    pub fn apply_override(&self, track: &VapObject) -> ContextVerdict {
        let Some(ctx) = &self.active else {
            return ContextVerdict {
                context: None,
                track_title: track.identity.title.clone(),
                status: ContextStatus::Passthrough,
                compatibility_score: 1.0,
                overrides_applied: vec![],
                rejection_log: vec![],
                final_hex: track.primary_hex(),
                target_hr: track.bpm(),
            };
        };

        let rules = &self.rules[ctx];
        let mut score = 1.0_f64;
        let mut modifications = Vec::new();
        let mut rejection_reasons = Vec::new();

        let track_bpm = track.bpm().unwrap_or(0.0);
        let track_met = track.met_score().unwrap_or(0.0);
        let track_arousal = track.arousal().unwrap_or(0.0);
        let track_valence = track.valence().unwrap_or(0.0);

        if let Some(min_bpm) = rules.min_bpm {
            if track_bpm < min_bpm {
                score -= 0.4;
                rejection_reasons.push(format!(
                    "BPM {track_bpm} too low for {ctx}"
                ));
            }
        }
        if let Some(max_bpm) = rules.max_bpm {
            if track_bpm > max_bpm {
                score -= 0.3;
                rejection_reasons.push(format!(
                    "BPM {track_bpm} too high for {ctx}"
                ));
            }
        }
        if let Some(min_met) = rules.min_met {
            if track_met < min_met {
                score -= 0.3;
                rejection_reasons.push(format!(
                    "MET Score {track_met} insufficient for workout"
                ));
            }
        }
        if let Some(min_arousal) = rules.min_arousal {
            if track_arousal < min_arousal {
                score -= 0.3;
                rejection_reasons.push("Energy level too low".into());
            }
        }
        if let Some(min_valence) = rules.min_valence {
            if track_valence < min_valence {
                score -= 0.2;
                rejection_reasons.push("Valence below scenario floor".into());
            }
        }

        let mut final_hex = track.primary_hex();
        if let Some(modif) = rules.lighting_modifier {
            if modif == "FORCE_RED_SHIFT" {
                let temp = track.palette_temp().unwrap_or_default().to_lowercase();
                if temp.contains("cool") || temp.contains("blood") {
                    // still allow red push for intensity contexts
                    final_hex = Some("#FF4500".into());
                    modifications.push("Lighting forced to High-Energy Red".into());
                }
            } else if modif == "FORCE_COOL_SHIFT" {
                final_hex = Some("#1E90FF".into());
                modifications.push("Lighting forced to Cool Blue".into());
            }
        }

        let _ = &rules.narrative_filter; // reserved for linguistic topic filtering

        let status = if score < 0.6 {
            ContextStatus::Rejected
        } else if score < 0.8 {
            ContextStatus::Borderline
        } else {
            ContextStatus::Accepted
        };

        ContextVerdict {
            context: Some(ctx.clone()),
            track_title: track.identity.title.clone(),
            status,
            compatibility_score: (score.max(0.0) * 100.0).round() / 100.0,
            overrides_applied: modifications,
            rejection_log: rejection_reasons,
            final_hex,
            target_hr: rules.min_bpm.or(Some(track_bpm)),
        }
    }
}
