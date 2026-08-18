//! # Fuxrt - Fuxyez Runtime Library
//! 
//! The mystical execution environment for ceremonial programming.
//! 
//! ## Features
//! 
//! - **Lattices**: Fractal data structures for recursive computation
//! - **Spinons**: Quantum-inspired data carriers with entanglement
//! - **Sigils**: Named invocable execution units
//! - **Rituals**: Ceremonial execution contexts
//! - **Chains**: Compositional execution pipelines
//! - **Oracles**: Meta-programming and external data integration
//! - **Echoes**: Advanced logging and output system
//! - **AuraFS Integration**: Distributed fractal shard storage
//! 
//! ## Quick Start
//! 
//! ```
//! use fuxrt::prelude::*;
//! 
//! // Create a lattice
//! let mut lattice = Lattice::new();
//! lattice.weave(42);
//! 
//! // Execute a ritual
//! let result = RitualBuilder::new("my_ritual")
//!     .execute(|| {
//!         println!("Executing ritual...");
//!     })
//!     .perform();
//! 
//! // Chain operations
//! let chain = Chain::new(10)
//!     .map(|x| x * 2)
//!     .map(|x| x + 5);
//! ```

#![warn(missing_docs)]
#![allow(clippy::module_inception)]

// ============================================================================
// Core Modules
// ============================================================================

pub mod core {
    //! Core runtime primitives
    
    pub mod lattice;
    pub mod spinon;
    pub mod thread;
    pub mod collapse;
    pub mod sigil;
    
    pub use lattice::{Lattice, LatticeNode, LatticeError, CoherenceState};
    pub use spinon::{Spinon, SpinState, SpinonPool};
    pub use thread::{Thread, ThreadPool, WeavingPattern, sleep};
    pub use collapse::{
        ritual_collapse, 
        ritual_collapse_timeout,
        collapse_deterministic,
        collapse_probabilistic,
        LazyCollapse,
        CollapseResult,
        CollapseError,
    };
    pub use sigil::{
        Sigil, 
        SigilRegistry, 
        Visibility, 
        Parameter,
        SigilStats,
    };
}

// ============================================================================
// Standard Library
// ============================================================================

pub mod std {
    //! Fuxyez standard library
    
    pub mod rituals;
    pub mod echoes;
    
    #[cfg(feature = "chains")]
    pub mod chains;
    
    #[cfg(feature = "oracles")]
    pub mod oracle;
    
    pub use rituals::{
        RitualContext,
        RitualPhase,
        RitualBuilder,
        RitualResult,
        RitualMetadata,
    };
    
    pub use echoes::{
        Echo,
        EchoLevel,
        EchoSystem,
        EchoHandler,
        ConsoleHandler,
        FileHandler,
        echo, trace, debug, info, warn, error, critical,
    };
    
    #[cfg(feature = "chains")]
    pub use chains::{
        Chain,
        ChainMode,
        ChainBuilder,
        ChainResult,
        chainlink::{ChainLink, FallibleChainLink},
        chainritual::{ChainRitual, ChainRitualBuilder},
        ritualchain::{RitualChain, ParallelRitualChain},
        ritual_link::{RitualLink, ConditionalRitualLink},
        ritual_chainlink::{
            RitualChainLink,
            CeremonialMode,
            LatticeRitualChainLink,
        },
    };
    
    #[cfg(feature = "oracles")]
    pub use oracle::{
        Oracle,
        OracleSource,
        OracleResult,
        OracleQuery,
        OracleError,
        SourceType,
    };
}

// ============================================================================
// Integrations
// ============================================================================

#[cfg(feature = "aurafs")]
pub mod aurafs {
    //! AuraFS integration module
    
    pub use crate::integrations::aurafs::{
        AuraFsBackend,
        AuraFsConfig,
        AuraFsError,
        AuraFsPersistence,
        Shard,
        ShardManager,
        ShardQuery,
        persist,
        load,
    };
}

#[cfg(feature = "aurafs")]
mod integrations {
    pub mod aurafs;
}

// ============================================================================
// Prelude
// ============================================================================

/// The Fuxyez prelude - import commonly used types
pub mod prelude {
    pub use crate::core::{
        Lattice,
        Spinon,
        SpinState,
        Thread,
        Sigil,
        CoherenceState,
    };
    
    pub use crate::std::rituals::{
        RitualBuilder,
        RitualContext,
        RitualPhase,
    };
    
    pub use crate::std::echoes::{
        echo, info, warn, error,
        EchoLevel,
    };
    
    #[cfg(feature = "chains")]
    pub use crate::std::chains::{
        Chain,
        ChainMode,
        RitualChainLink,
        CeremonialMode,
    };
    
    #[cfg(feature = "oracles")]
    pub use crate::std::oracle::{
        Oracle,
        OracleQuery,
    };
    
    #[cfg(feature = "aurafs")]
    pub use crate::aurafs::{
        AuraFsBackend,
        persist,
        load,
    };
}

// ============================================================================
// Version Information
// ============================================================================

/// Fuxrt version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Fuxrt name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Get runtime version string
pub fn version() -> String {
    format!("{} v{}", NAME, VERSION)
}

/// Initialize the Fuxyez runtime
pub fn init() {
    crate::std::echoes::init();
    crate::std::echoes::info(format!("Initialized {}", version()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let ver = version();
        assert!(ver.contains("fuxrt"));
    }

    #[test]
    fn test_prelude() {
        use prelude::*;
        
        let lattice = Lattice::<i32>::new();
        assert_eq!(lattice.size(), 0);
    }
}
// ============================================================================
// Core/Sigil Module
// ============================================================================
pub mod core {
    pub mod sigil;
}
use core::sigil::{Sigil, SigilRegistry, Visibility, Parameter};
// ============================================================================
// Std/Echoes Module
pub mod std {
    pub mod echoes;
}
use std::echoes::{Echo, EchoLevel, EchoSystem, EchoHandler, ConsoleHandler, FileHandler};
// ============================================================================
// Std/Oracle Module
#[cfg(feature = "oracles")]
pub mod std {
    pub mod oracle;
}
#[cfg(feature = "oracles")]
use std::oracle::{Oracle, OracleSource, OracleResult, OracleQuery, OracleError, SourceType};
// ============================================================================
// Std/Chains Module
#[cfg(feature = "chains")]
pub mod std {
    pub mod chains;
}
#[cfg(feature = "chains")]
use std::chains::{
    Chain,
    ChainMode,
    ChainBuilder,
    ChainResult,
    chainlink::{ChainLink, FallibleChainLink},
    chainritual::{ChainRitual, ChainRitualBuilder},
    ritualchain::{RitualChain, ParallelRitualChain},
    ritual_link::{RitualLink, ConditionalRitualLink},
    ritual_chainlink::{
        RitualChainLink,
        CeremonialMode,
        LatticeRitualChainLink,
    },
};
impl<T> ParallelRitualChain<T>
where
    T: Clone + Send + 'static,
{
    /// Execute all rituals in parallel on the initial data
    pub fn execute_parallel(self, initial_data: T) -> Vec<T> {
        use std::thread;
        
        let handles: Vec<_> = self.rituals
            .into_iter()
            .map(|(name, transform)| {
                let data = initial_data.clone();
                thread::spawn(move || {
                    transform(data)
                })
            })
            .collect();
        
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .collect()
    }
}
impl<T> ParallelRitualChain<T>
where
    T: Clone + Send + 'static,
{
    /// Execute all rituals in parallel on the initial data
    pub fn execute_parallel(self, initial_data: T) -> Vec<T> {
        use std::thread;
        
        let handles: Vec<_> = self.rituals
            .into_iter()
            .map(|(name, transform)| {
                let data = initial_data.clone();
                thread::spawn(move || {
                    transform(data)
                })
            })
            .collect();
        
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .collect()
    }
    /// Execute all rituals in parallel on the initial data
    pub fn execute_parallel(self, initial_data: T) -> Vec<T> {
        use std::thread;
        
        let handles: Vec<_> = self.rituals
            .into_iter()
            .map(|(name, transform)| {
                let data = initial_data.clone();
                thread::spawn(move || {
                    transform(data)
                })
            })
            .collect();
        
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .collect()
    }
    /// Execute all rituals in parallel on the initial data
    pub fn execute_parallel(self, initial_data: T) -> Vec<T> {
        use std::thread;
        
        let handles: Vec<_> = self.rituals
            .into_iter()
            .map(|(name, transform)| {
                let data = initial_data.clone();
                thread::spawn(move || {
                    transform(data)
                })
            })
            .collect();
        
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .collect()
    }
}
impl Sigil {
    /// Format sigil signature as string
    pub fn signature(&self) -> String {
        let params = self.parameters
            .iter()
            .map(|p| {
                if let Some(ty) = &p.type_hint {
                    format!("{}: {}", p.name, ty)
                } else {
                    p.name.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let ret = if let Some(ty) = &self.return_type {
            format!(" -> {}", ty)
        } else {
            String::new()
        };
        format!("{}({}){}", self.name, params, ret)
    }
}
impl SigilRegistry {
    /// Get total number of registered sigils
    pub fn total_sigils(&self) -> usize {
        let sigils = self.sigils.read().unwrap();
        sigils.len()
    }
}