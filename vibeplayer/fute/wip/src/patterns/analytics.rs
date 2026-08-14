//! Mythical Analytics for Patterns (Quantum GVS Flavored)
//!
//! Pattern distribution, project health dashboard, drift detection, 
//! quantum-lattice governance reporting, anomaly flagging, and real-time hooks.
//!
//! Engineered for research, machine learning, continuous refactoring, and devops-level observability.

use crate::ast::{AstNode, UniversalAst};
use crate::patterns::library::{PatternLibrary, PatternDomain, SemanticPattern};
use crate::patterns::matcher::PatternMatcherEngine;
use crate::patterns::cache::PatternCache;
use std::collections::HashMap;
use chrono::{Utc, DateTime};

// --- Analytic Result Types ---

#[derive(Debug, Clone)]
pub struct PatternStats {
    pub by_domain: HashMap<PatternDomain, usize>,
    pub by_pattern: HashMap<String, usize>,
    pub entropy_per_domain: HashMap<PatternDomain, f64>,
    pub governance_alerts: Vec<GovernanceAlert>,
    pub health_index: f32,
    pub last_scan: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GovernanceAlert {
    pub message: String,
    pub affected_nodes: Vec<String>,
    pub severity: u8,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
}

// --- Quantum Governance/Analytics Engine ---

pub struct PatternAnalytics;

impl PatternAnalytics {
    /// Full codebase scan and quantum-classical-lattice domain reporting.
    pub fn analyze_codebase(
        library: &PatternLibrary,
        matcher: &PatternMatcherEngine,
        asts: &[UniversalAst],
        cache: &PatternCache,
    ) -> PatternStats {
        let mut by_domain = HashMap::new();
        let mut by_pattern = HashMap::new();
        let mut node_count = 0usize;

        // Scan all ASTs for pattern matches and build multi-domain heatmaps
        for ast in asts {
            for PatternDomain in [PatternDomain::Quantum, PatternDomain::Lattice, PatternDomain::Classical, PatternDomain::Hybrid, PatternDomain::Ritual].iter() {
                let matches = matcher.match_ast(ast);
                for m in &matches {
                    let domain = &m.pattern.domain;
                    *by_domain.entry(domain.clone()).or_insert(0) += 1;
                    *by_pattern.entry(m.pattern.name.clone()).or_insert(0) += 1;
                    node_count += 1;
                }
            }
        }

        // Entropy (diversity) calculations per domain
        let entropy_per_domain = by_domain.iter().map(|(domain, &count)| {
            let entropy = if count > 0 {
                // Simple entropy ~ diversity calculation for illustration
                let p = (count as f64) / (node_count as f64).max(1.);
                if p > 0.0 { -p * p.log2() } else { 0.0 }
            } else { 0.0 };
            (domain.clone(), entropy)
        }).collect();

        // Governance Alerts: Flag hot zones, anomalies, drift, etc.
        let governance_alerts = PatternAnalytics::generate_governance_alerts(&by_domain, &by_pattern, cache);

        // “Semantic Health Index” of project (quantum magic + hype)
        let health_index = PatternAnalytics::compute_health_index(&by_domain, &by_pattern);

        PatternStats {
            by_domain,
            by_pattern,
            entropy_per_domain,
            governance_alerts,
            health_index,
            last_scan: Utc::now(),
        }
    }

    /// Generate GVS-style governance alerts: semantic drift, hot spots, risk nodes.
    pub fn generate_governance_alerts(
        by_domain: &HashMap<PatternDomain, usize>,
        by_pattern: &HashMap<String, usize>,
        cache: &PatternCache,
    ) -> Vec<GovernanceAlert> {
        let mut alerts = Vec::new();

        // Example: flag quantum >80% as “Quantum Dominated Project”
        if let Some(&quantum_count) = by_domain.get(&PatternDomain::Quantum) {
            let total: usize = by_domain.values().sum();
            let percent = if total > 0 { (quantum_count as f64) / (total as f64) } else { 0.0 };
            if percent > 0.8 {
                alerts.push(GovernanceAlert {
                    message: "Quantum patterns dominate this project (>80%)".into(),
                    affected_nodes: vec!["too_many_to_list".into()],
                    severity: 7,
                    timestamp: Utc::now(),
                    tags: vec!["quantum-dominance".into(), "governance".into()],
                });
            }
        }

        // Example: anomaly if some rare pattern spikes
        for (pattern_name, &count) in by_pattern.iter() {
            if count > 1000 {
                alerts.push(GovernanceAlert {
                    message: format!("Pattern '{}' spiking above normal use: {}", pattern_name, count),
                    affected_nodes: cache.get_nodes_for_pattern(pattern_name).iter().map(|k| format!("{:?}", k)).collect(),
                    severity: 6,
                    timestamp: Utc::now(),
                    tags: vec!["anomaly".into(), "spike".into()],
                });
            }
        }

        alerts
    }

    /// “Semantic Health Index” — tunable ML-augmented scoring in advanced engines.
    fn compute_health_index(
        by_domain: &HashMap<PatternDomain, usize>,
        by_pattern: &HashMap<String, usize>
    ) -> f32 {
        // Example: penalize excess dominance, reward diversity
        let total: usize = by_domain.values().sum();
        if total == 0 { return 1.0; }
        let mut penalty = 0.0;
        for &count in by_domain.values() {
            let p = (count as f64) / (total as f64);
            if p > 0.8 {
                penalty += (p - 0.8) * 0.25; // discourage monocultures
            }
        }
        let diversity = by_pattern.len() as f32 / (total as f32).max(1.);
        (1.0 - penalty) * diversity.clamp(0.0, 1.0)
    }

    /// Real-time CLI/project dashboard hook for analytics display.
    pub fn analytics_summary(stats: &PatternStats) -> String {
        use std::fmt::Write;
        let mut output = String::new();
        writeln!(output, "=== Fuxyez Semantic Analytics ===").unwrap();
        writeln!(output, " Completed: {}", stats.last_scan).unwrap();
        writeln!(output, " Health Index: {:.2}", stats.health_index).unwrap();
        writeln!(output, "").unwrap();
        writeln!(output, " Pattern Counts by Domain:").unwrap();
        for (d, n) in &stats.by_domain {
            writeln!(output, "   - {:?}: {}", d, n).unwrap();
        }
        writeln!(output, " Top Patterns:").unwrap();
        for (pat, n) in stats.by_pattern.iter().take(5) {
            writeln!(output, "   - {}: {}", pat, n).unwrap();
        }
        if !stats.governance_alerts.is_empty() {
            writeln!(output, " Governance Alerts ({}):", stats.governance_alerts.len()).unwrap();
            for ga in &stats.governance_alerts {
                writeln!(output, "   >> [{}] {}", ga.severity, ga.message).unwrap();
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::library::PatternLibrary;
    use crate::patterns::matcher::PatternMatcherEngine;
    use crate::patterns::cache::{PatternCache, EvictionPolicy};
    use crate::ast::{UniversalAst, AstNode, Type};

    #[test]
    fn test_random_project_health() {
        let library = PatternLibrary::new();
        let matcher = PatternMatcherEngine::new(&library);
        let cache = PatternCache::new(300, EvictionPolicy::Manual);

        let asts = vec![
            UniversalAst {
                root: AstNode::VarDecl {
                    name: "q0".into(),
                    ty: Some(Type::Named("Qubit".into())),
                    value: None,
                    is_mutable: false,
                    symbol_id: None,
                },
                metadata: Default::default(),
            }
        ];

        let stats = PatternAnalytics::analyze_codebase(&library, &matcher, &asts, &cache);
        assert!(stats.by_domain.get(&PatternDomain::Quantum).is_some());
        assert!(stats.health_index >= 0.0 && stats.health_index <= 1.0);
        // for CLI integration:
        println!("{}", PatternAnalytics::analytics_summary(&stats));
    }
}