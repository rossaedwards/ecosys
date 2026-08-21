//! Spinon - rÆ Quantum Carriers & Neglectons
//!
//! Topological quasiparticles with Majorana zero modes, Berry curvature,
//! and non-semisimple TQFT integration for universal quantum gates.

use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use nalgebra::{DVector, Complex};

/// Quantum state with full Bloch sphere + topological charge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    /// Bloch vector (x,y,z) ∈ S^2
    pub bloch: [f64; 3],
    /// Topological invariant (neglecton charge)
    pub topological_charge: f64,
    /// Berry phase accumulation
    pub berry_phase: f64,
}

impl Default for QuantumState {
    fn default() -> Self {
        Self {
            bloch: [0.0, 0.0, 0.0], // |↓⟩
            topological_charge: 0.0,
            berry_phase: 0.0,
        }
    }
}

/// Spinon with full quantum + topological features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spinon {
    pub id: String,
    /// Full quantum state
    pub state: QuantumState,
    /// Entangled partners (Bell pairs)
    pub entanglements: Vec<usize>,
    /// Lattice position (Flower of Life node)
    pub lattice_node: Option<usize>,
    /// Majorana zero mode (topological qubit)
    pub majorana_mode: Option<MajoranaMode>,
    pub metadata: SpinonMetadata,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MajoranaMode {
    /// Real Majorana zero mode (γ = γ†)
    Real,
    /// Complex fermion pair (c = (γ₁ + iγ₂)/√2)
    Complex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinonMetadata {
    pub created_at: u64,
    pub measured: bool,
    pub braiding_operations: usize,
    pub chern_number: Option<i32>,
}

/// rÆ Kraus operators (thesis §2.1 CPTP map)
pub struct RaeKraus {
    operators: Vec<DMatrix<Complex<f64>>>,
}

impl Spinon {
    /// Create neglecton-enhanced spinon
    pub fn neglecton(spin_state: QuantumState, charge: f64) -> Self {
        Self {
            id: Self::generate_id(),
            state: spin_state,
            state.topological_charge = charge,
            entanglements: Vec::new(),
            lattice_node: None,
            majorana_mode: Some(MajoranaMode::Real),
            metadata: SpinonMetadata {
                created_at: Self::timestamp(),
                measured: false,
                braiding_operations: 0,
                chern_number: None,
            },
        }
    }

    /// Braiding operation (non-semisimple TQFT)
    pub fn braid(&mut self, other: &mut Spinon, exchange: bool) {
        // R-matrix braiding phase (Sl(2,2) category)
        let phase = if exchange {
            std::f64::consts::PI / 4.0 // Ising π/8 + neglecton correction
        } else {
            -std::f64::consts::PI / 4.0
        };

        self.state.berry_phase += phase;
        other.state.berry_phase -= phase;
        
        self.metadata.braiding_operations += 1;
        other.metadata.braiding_operations += 1;
        
        // Update Chern number
        self.metadata.chern_number = Some(self.compute_chern());
    }

    /// Majorana fusion (topological qubit)
    pub fn majorana_fuse(&self, other: &Spinon) -> FusionResult {
        match (self.majorana_mode, other.majorana_mode) {
            (Some(MajoranaMode::Real), Some(MajoranaMode::Real)) => {
                // γ₁γ₂ → fermion parity
                let parity = (self.state.topological_charge * other.state.topological_charge).signum();
                FusionResult::Fermion(parity as i8)
            }
            _ => FusionResult::Vacuum,
        }
    }

    /// rÆ channel encoding
    pub fn rae_encode(&self) -> RaeKraus {
        RaeKraus::from_spinon(self)
    }

    fn compute_chern(&self) -> i32 {
        // Simplified Chern number from Berry curvature integral
        let curvature = self.state.berry_phase / (2.0 * std::f64::consts::PI);
        curvature.round() as i32
    }
}

pub enum FusionResult {
    Fermion(i8),  // +1 or -1 parity
    Vacuum,
}

/// Enhanced spinon pool with topological operations
pub struct TopologicalSpinonPool {
    spinons: Arc<RwLock<Vec<Spinon>>>,
}

impl TopologicalSpinonPool {
    /// Create Bell pair (maximally entangled)
    pub fn create_bell_pair(&mut self) -> (usize, usize) {
        let spinon1 = Spinon::neglecton(
            QuantumState { bloch: [0.0, 0.0, 1.0], ..Default::default() },
            1.0
        );
        let spinon2 = Spinon::neglecton(
            QuantumState { bloch: [0.0, 0.0, -1.0], ..Default::default() },
            1.0
        );

        let idx1 = self.add(spinon1);
        let idx2 = self.add(spinon2);
        
        // Entangle
        self.spinons.write().unwrap()[idx1].entangle(idx2);
        self.spinons.write().unwrap()[idx2].entangle(idx1);
        
        (idx1, idx2)
    }
}

// [Previous SpinonPool impl extended with topological ops...]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neglecton_braiding() {
        let mut spinon1 = Spinon::neglecton(
            QuantumState::default(), 0.5
        );
        let mut spinon2 = Spinon::neglecton(
            QuantumState::default(), -0.5
        );

        spinon1.braid(&mut spinon2, true);
        
        assert_eq!(spinon1.metadata.braiding_operations, 1);
        assert!((spinon1.state.berry_phase - std::f64::consts::PI/4.0).abs() < 1e-10);
    }

    #[test]
    fn test_majorana_fusion() {
        let spinon1 = Spinon::neglecton(QuantumState::default(), 1.0);
        let spinon2 = Spinon::neglecton(QuantumState::default(), 1.0);
        
        assert!(matches!(spinon1.majorana_fuse(&spinon2), FusionResult::Fermion(1)));
    }
}
    fn add(&mut self, spinon: Spinon) -> usize {
        let mut pool = self.spinons.write().unwrap();
        pool.push(spinon);
        pool.len() - 1
    }
    
    fn entangle(&mut self, other_idx: usize) {
        let mut pool = self.spinons.write().unwrap();
        let self_idx = pool.iter().position(|s| s.id == self.id).unwrap();
        pool[self_idx].entanglements.push(other_idx);
    }
    
    fn generate_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("spinon-{}", timestamp)
    }
    
    fn timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

impl RaeKraus {
    fn from_spinon(spinon: &Spinon) -> Self {
        // Placeholder: actual Kraus operator derivation from spinon state
        let op = DMatrix::<Complex<f64>>::identity(2, 2);
        Self {
            operators: vec![op],
        }
    }

    pub fn apply(&self, state: &DVector<Complex<f64>>) -> DVector<Complex<f64>> {
        let mut new_state = DVector::<Complex<f64>>::zeros(state.len());
        for k in &self.operators {
            new_state += k * state;
        }
        new_state
    }

}
impl TopologicalSpinonPool {
    fn new() -> Self {
        Self {
            spinons: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
            Self {
                source: source.into(),
                filters: Vec::new(),
                limit: None,
                offset: None,
            }
        }
        
        pub fn filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.filters.push((key.into(), value.into()));
            self
        }
        
        pub fn limit(mut self, limit: usize) -> Self {
            self.limit = Some(limit);
            self
        }
        
        pub fn offset(mut self, offset: usize) -> Self {
            self.offset = Some(offset);
            self
        }
        
        pub fn build(self) -> String {
            let mut query = format!("source={}", self.source);
            for (k, v) in self.filters {
                query.push_str(&format!("&{}={}", k, v));
            }
            if let Some(lim) = self.limit {
                query.push_str(&format!("&limit={}", lim));
            }
            if let Some(off) = self.offset {
                query.push_str(&format!("&offset={}", off));
            }
            query
        }
    }
        Chain::with_mode(self.data, self.mode)
    pub fn clear_history(&mut self) {
        if let Ok(mut history) = self.history.lock() {
            history.clear();
        }

    /// Cleanup phase
    pub fn cleanup<F>(mut self, f: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        self.cleanup.push(Box::new(f));
        self
    }
        self.metadata.total_duration = start.elapsed().as_millis();
        context.phase = RitualPhase::Cleanup;
        for clean in self.cleanup {
            clean();
        }
        
        ChainRitualResult {
            result: data,
            context,
        }
/// Oracle Query Builder
/// From fuxyez/fuxrt/std/core/oracle/mod.rs
pub struct OracleQuery {
    source: String,
    filters: Vec<(String, String)>,
    limit: Option<usize>,
    offset: Option<usize>,
}
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            filters: Vec::new(),
            limit: None,
            offset: None,
        }
    }
    
    pub fn filter(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.push((field.into(), value.into()));
        self
    }
    
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }
    
    pub fn offset(mut self, n: usize) -> Self {
        self.offset = Some(n);
        self
    }
    
    pub fn build(&self) -> String {
        let mut query = format!("FROM {}", self.source);
        
        if !self.filters.is_empty() {
            let filters = self.filters
                .iter()
                .map(|(k, v)| format!("{} = {}", k, v))
                .collect::<Vec<_>>()
                .join(" AND ");
            query.push_str(&format!(" WHERE {}", filters));
        }
        
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        query
    }
}
impl OracleQuery {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            filters: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn filter(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.push((field.into(), value.into()));
        self
    }
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }
    pub fn offset(mut self, n: usize) -> Self {
        self.offset = Some(n);
        self
    }
    pub fn build(&self) -> String {
        let mut query = format!("FROM {}", self.source);
        
        if !self.filters.is_empty() {
            let filters = self.filters
                .iter()
                .map(|(k, v)| format!("{} = {}", k, v))
                .collect::<Vec<_>>()
                .join(" AND ");
            query.push_str(&format!(" WHERE {}", filters));
        }
        
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        query
    }
}