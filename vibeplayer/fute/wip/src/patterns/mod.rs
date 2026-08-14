//! Fuxyez Mythical Patterns API (mod.rs)
//!
//! This is your top-level patterns orchestrator—exposing all quantum, lattice, classical, hybrid, and ritual
//! pattern detection/analytics infrastructure, plugin extension points, macro test harnesses, and governance hooks.
//!
//! Designed for use in your core engine, transformers, CLI (search/diagnostics), and live plugin ecosystems.

pub mod library;
pub mod matcher;
pub mod detector;
pub mod rules;
pub mod cache;
pub mod analytics;
pub mod ext;
pub mod test_utils;

pub use library::{PatternLibrary, SemanticPattern, PatternDomain, PatternDomain as Domain, TransformationHint};
pub use matcher::{PatternMatcherEngine, PatternMatch};
pub use detector::{PatternDetector, SemanticRegion};
pub use rules::{PatternRuleEngine, PatternRule, RuleContext, RuleResult, RuleAction, RuleCondition};
pub use cache::{PatternCache, NodeKey, EvictionPolicy, CacheStats};
pub use analytics::{PatternAnalytics, PatternStats, GovernanceAlert};
pub use ext::{PatternPack, ExtRegistry, PatternPackMetadata};
pub use test_utils::*;

#[doc(hidden)]
pub const PATTERNS_VERSION: &str = "mythic/1.0.0";

// Convenience prelude for developers and plugins:
pub mod prelude {
    pub use super::{PatternLibrary, PatternMatcherEngine, PatternDetector, SemanticPattern, Domain, PatternPack};
    pub use super::rules::*;
    pub use super::test_utils::*;
}

// Shorthand glue for Fuxyez CLI apps and automation tools.
pub fn initialize_default_patterns() -> PatternLibrary {
    let mut library = PatternLibrary::new();
    // Optionally: register built-in supercharged patterns, or autoload packs
    library
}