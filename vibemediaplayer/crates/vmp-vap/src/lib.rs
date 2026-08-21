//! Vibe Audio Protocol (V.A.P.) v3.1 — types, loader, scoring, context.
//!
//! Captures experiential (How/Why) identity across nine TSLCA pillars.
//! Compatible with nested golden-set payloads and flat scoring-engine outputs.

mod context;
mod error;
mod loader;
mod scoring;
mod types;

pub use context::{ContextEngine, ContextResult, ContextStatus, ContextVerdict};
pub use error::{VapError, VapResult};
pub use loader::{LoadOutcome, LoadSource, VapLoader};
pub use scoring::VapScoringEngine;
pub use types::*;

/// Protocol version constant.
pub const VAP_VERSION: &str = "3.1";

/// Ordered pillar identifiers for UI tabs (Firefox/Opera-style vertical rail).
pub const PILLAR_TABS: &[(PillarId, &str, &str)] = &[
    (PillarId::Structural, "P1", "Structural"),
    (PillarId::Tonal, "P2", "Tonal"),
    (PillarId::Timbral, "P3", "Timbral"),
    (PillarId::Linguistic, "P4", "Linguistic"),
    (PillarId::Affective, "P5", "Affective"),
    (PillarId::Contextual, "P6", "Contextual"),
    (PillarId::Photometric, "P7", "Photometric"),
    (PillarId::Kinetic, "P8", "Kinetic"),
    (PillarId::Genealogical, "P9", "Genealogical"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/cannibal_corpse_inhumane_harvest.vap.json")
    }

    #[test]
    fn loads_cannibal_corpse_fixture() {
        let vap = VapObject::from_path(&fixture_path()).expect("load fixture");
        assert_eq!(vap.vap_version, "3.1");
        assert_eq!(vap.identity.artist, "Cannibal Corpse");
        assert_eq!(vap.identity.title, "Inhumane Harvest");
        assert!(vap.pillars.structural.is_some());
        assert!(vap.pillars.affective.is_some());
        assert!(vap.pillars.photometric.is_some());
        assert!(vap.pillars.kinetic.is_some());
        assert!(vap.pillars.genealogical.is_some());
    }

    #[test]
    fn pillar_tabs_are_nine() {
        assert_eq!(PILLAR_TABS.len(), 9);
    }

    #[test]
    fn scoring_engine_kick_and_met() {
        let eng = VapScoringEngine::new();
        assert_eq!(eng.kick_profile(5.0), "Sharp (Click)");
        assert_eq!(eng.kick_profile(20.0), "Punch (Thud)");
        assert_eq!(eng.kick_profile(45.0), "Boom (Sub)");
        assert!((eng.kinetic_met(50.0) - 1.0).abs() < f64::EPSILON);
        assert!((eng.kinetic_met(150.0) - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn context_gym_accepts_high_energy() {
        let mut eng = ContextEngine::new();
        eng.set_context("GYM_PEAK").unwrap();
        let vap = VapObject::from_path(&fixture_path()).unwrap();
        let r = eng.apply_override(&vap);
        assert_eq!(r.status, ContextStatus::Accepted);
        assert!(r.compatibility_score >= 0.6);
    }

    #[test]
    fn defaults_are_valid_json() {
        let d = VapObject::defaults("Unknown", "Untitled");
        let s = serde_json::to_string_pretty(&d).unwrap();
        let back: VapObject = serde_json::from_str(&s).unwrap();
        assert_eq!(back.vap_version, VAP_VERSION);
    }

    #[test]
    fn thayer_quadrant() {
        let eng = VapScoringEngine::new();
        let t = eng.thayer_coordinates("Major", 0.8, 0.9);
        assert!(t.valence > 0.0);
        assert_eq!(t.mood_quadrant, "Euphoria/Joy");
    }
}
