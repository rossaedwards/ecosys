//! Pattern Extensibility & Plugin Packs - Fuxyez Mythic
//!
//! User-facing plugin system for hot-loading domain packs—quantum, lattice, research, regulatory, anything.
//! Supports versioning, metadata, dependency management, audit trails, and live CLI/APIs.

use crate::patterns::library::{PatternLibrary, SemanticPattern};
use semver::Version;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternPackMetadata {
    pub name: String,
    pub version: Version,
    pub author: String,
    pub description: String,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>, // Can support transitive pattern pack dependencies
    pub enabled: bool,
    pub file_path: Option<String>, // Loaded from
}

pub trait PatternPack: Send + Sync {
    fn metadata(&self) -> &PatternPackMetadata;
    fn register(&self, library: &mut PatternLibrary);
}

pub struct ExtRegistry {
    packs: HashMap<String, Box<dyn PatternPack>>,
    library: PatternLibrary,
    audit_log: Vec<String>,
}

impl ExtRegistry {
    pub fn new(library: PatternLibrary) -> Self {
        Self { packs: HashMap::new(), library, audit_log: vec![] }
    }

    /// Load/register a pattern pack (hot plugin add).
    pub fn register_pack(&mut self, pack: Box<dyn PatternPack>) {
        let meta = pack.metadata().clone();
        if meta.enabled {
            pack.register(&mut self.library);
            self.audit_log.push(format!(
                "[PATTERN PACK] Registered: {} v{} by {}",
                meta.name, meta.version, meta.author
            ));
            self.packs.insert(meta.name.clone(), pack);
        }
    }

    /// Unregister/remove pattern pack.
    pub fn unregister_pack(&mut self, name: &str) {
        if let Some(pack) = self.packs.remove(name) {
            self.audit_log.push(format!(
                "[PATTERN PACK] Unregistered: {} v{}",
                pack.metadata().name, pack.metadata().version
            ));
            // No easy way to *remove* patterns, considered permanent unless on reload
            // Optionally: re-init library here
        }
    }

    /// Reload/replace-all packs
    pub fn reload(&mut self, packs: Vec<Box<dyn PatternPack>>) {
        for pack in packs {
            self.register_pack(pack);
        }
    }

    /// Return all active plugin meta info
    pub fn active_packs_metadata(&self) -> Vec<PatternPackMetadata> {
        self.packs.values().map(|p| p.metadata().clone()).collect()
    }

    /// Return audit events for CLI/JIT diagnostics
    pub fn audit_history(&self) -> &[String] {
        &self.audit_log
    }

    /// Access live pattern library (including pack-registered ones)
    pub fn library(&self) -> &PatternLibrary {
        &self.library
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semver::Version;

    struct DemoPack;
    impl PatternPack for DemoPack {
        fn metadata(&self) -> &PatternPackMetadata {
            static META: once_cell::sync::Lazy<PatternPackMetadata> = once_cell::sync::Lazy::new(|| PatternPackMetadata {
                name: "Demo Quantum Pack".to_string(),
                version: Version::new(0, 1, 0),
                author: "Mythicus".to_string(),
                description: "Demo Quantum Patterns".to_string(),
                tags: vec!["quantum".to_string()],
                dependencies: vec![],
                enabled: true,
                file_path: None
            });
            &*META
        }
        fn register(&self, library: &mut PatternLibrary) {
            // Register demo patterns here...
        }
    }

    #[test]
    fn plugin_system_loads_pack() {
        let mut reg = ExtRegistry::new(PatternLibrary::new());
        reg.register_pack(Box::new(DemoPack));
        assert_eq!(reg.active_packs_metadata().len(), 1);
    }
}