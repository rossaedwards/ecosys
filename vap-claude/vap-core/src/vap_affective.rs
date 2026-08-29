//! Port of `vap/vap_affective.h` — V.A.P. v3.1 Pillar 5: AFFECTIVE (The Heart)
//! Thayer Coordinate System.

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResolutionState {
    #[default]
    Triumphant,
    Melancholic,
    Unresolved,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VapAffective {
    /// -1.0 (Despair) -> 0.0 (Neutral) -> +1.0 (Euphoria)
    pub valence: f32,
    /// 0.0 (Sleep) -> 1.0 (Rage/Panic)
    pub arousal: f32,
    /// 0.0 (Vulnerable) -> 1.0 (Aggressive)
    pub dominance: f32,
    /// 0.0 = volatile, 1.0 = constant
    pub mood_stability: f32,
    pub catharsis_potential: f32,
    pub nostalgia_trigger: f32,
    /// Tension Arc
    pub buildup_velocity: f32,
    pub resolution_state: ResolutionState,
}

impl Default for VapAffective {
    fn default() -> Self {
        // Thayer neutral defaults, matching vap_runtime_init()
        VapAffective {
            valence: 0.0,
            arousal: 0.5,
            dominance: 0.5,
            mood_stability: 0.7,
            catharsis_potential: 0.3,
            nostalgia_trigger: 0.2,
            buildup_velocity: 0.4,
            resolution_state: ResolutionState::Triumphant,
        }
    }
}

impl VapAffective {
    /// Exact port of `vap_affective_warmth()`.
    /// Positive valence + high arousal = warm/gold; negative valence + high
    /// arousal = red/aggressive; low arousal = cool/ambient regardless of valence.
    pub fn warmth(&self) -> f32 {
        (self.valence * 0.5 + 0.5) * self.arousal
    }
}
