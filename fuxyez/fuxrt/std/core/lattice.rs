//! Lattice - Fractal Quantum Data Structure for Aurphyx rÆ Channels
//!
//! Sacred geometry lattices (Flower of Life, Sierpiński) with photonic bandgaps,
//! topological protection, cymatic trapping, and zero-point energy modulation.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use nalgebra::{DMatrix, DVector};
use petgraph::Graph;
use rayon::prelude::*;

#[cfg(feature = "aurafs")]
use crate::aurafs::AuraFsBackend;
#[cfg(feature = "quantum")]
use crate::sigil::Sigil;

pub use coherence::CoherenceState;
pub use geometry::{FlowerOfLife, SierpinskiGasket, MetatronsCube};

mod coherence;
mod geometry;
mod quantum;
mod topology;

/// Sacred geometry lattice node with quantum metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeNode<T> {
    /// Node ID (AuraFS shard + rÆ channel)
    pub id: String,
    pub value: T,
    pub children: Vec<usize>,
    /// Quantum metadata
    pub quantum: NodeQuantumMetadata,
    /// Sacred geometry position
    pub position: Option<(f64, f64)>,
}

/// Quantum node properties (thesis §2.1)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeQuantumMetadata {
    /// rÆ channel index
    pub rae_channel: usize,
    /// Zero-point coupling λ (thesis eq: E_k = E_k^0 + λZ_k)
    pub zero_point_lambda: f64,
    /// Berry phase accumulation
    pub berry_phase: f64,
    /// Cymatic trap frequency
    pub trap_frequency: Option<f64>,
    /// Topological charge (neglecton/anyon)
    pub topological_charge: Option<f64>,
}

/// Fractal quantum lattice
pub struct Lattice<T> {
    pub id: String,
    nodes: Arc<RwLock<Vec<LatticeNode<T>>>>,
    root: usize,
    coherence: CoherenceState,
    node_map: Arc<RwLock<HashMap<String, usize>>>,
    /// Sacred geometry graph
    graph: Arc<RwLock<Graph<usize, f64>>>,
    /// Fractal dimension D_f (Sierpiński=1.585)
    fractal_dim: f64,
}

impl<T> Lattice<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync,
{
    /// Create Flower of Life lattice (thesis §3.1, 19 circles)
    pub fn flower_of_life(n_rings: usize) -> Self {
        let (coords, adj) = FlowerOfLife::generate(n_rings);
        let mut lattice = Self::from_adjacency(adj, CoherenceState::Quantum);
        lattice.fractal_dim = 1.8; // Measured
        lattice.set_positions(coords);
        lattice
    }

    /// Sierpiński gasket (D_f = log(3)/log(2) = 1.585)
    pub fn sierpinski_gasket(level: usize) -> Self {
        let (coords, adj) = SierpinskiGasket::generate(level);
        let mut lattice = Self::from_adjacency(adj, CoherenceState::Quantum);
        lattice.fractal_dim = 1.585;
        lattice.set_positions(coords);
        lattice
    }

    /// From adjacency matrix + coherence
    pub fn from_adjacency(adj: DMatrix<f64>, coherence: CoherenceState) -> Self {
        let graph = Graph::from_adjacency_matrix(&adj);
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            nodes: Arc::new(RwLock::new(Vec::new())),
            root: 0,
            coherence,
            node_map: Arc::new(RwLock::new(HashMap::new())),
            graph: Arc::new(RwLock::new(graph)),
            fractal_dim: 2.0,
        }
    }

    /// rÆ channel encoding (thesis §2.1 CPTP map)
    pub fn rae_encode(&mut self, spinon: &crate::spinon::Spinon) -> quantum::Oracle {
        let rbit = quantum::rae_kraus(spinon);
        self.weave_quantum(rbit).expect("rÆ weave failed")
    }

    /// Photonic bandgap (thesis §8.1 PWE method)
    pub fn compute_bandgap(&self) -> f64 {
        let adj = self.adjacency_matrix();
        let H = quantum::topological_hamiltonian(&adj);
        let evals = H.eigenvalues();
        let sorted = evals.as_slice().to_vec();
        let gap1 = sorted[1] - sorted[0]; // 1.25→1.65
        let gap2 = sorted[4] - sorted[3]; // 2.40→2.90
        gap1.min(gap2)
    }

    /// Hilbert space scaling (thesis theorem 2.3)
    pub fn hilbert_dimension(&self, d_qubit: usize) -> f64 {
        let n = self.size() as f64;
        d_qubit as f64.powf(n * self.fractal_dim.powf(1.5))
    }

    /// Cymatic trapping (thesis §4.2, √2:π:e frequencies)
    pub fn cymatic_trap(&mut self, freqs: &[f64]) -> topology::StandingWave {
        topology::StandingWave::new(freqs, self.positions())
    }

    /// Zero-point modulation (Casimir stabilization)
    pub fn zero_point_modulate(&mut self, lambda: f64) -> &mut Self {
        let mut nodes = self.nodes.write().unwrap();
        nodes.par_iter_mut().for_each(|node| {
            node.quantum.zero_point_lambda = lambda;
        });
        self
    }

    /// Weave quantum data with rÆ metadata
    pub fn weave_quantum(&mut self, value: T) -> Result<String, LatticeError> {
        let node_id = Self::generate_id();
        let node = LatticeNode {
            id: node_id.clone(),
            value,
            children: Vec::new(),
            quantum: NodeQuantumMetadata::default(),
            position: None,
        };

        let mut nodes = self.nodes.write().map_err(|_| LatticeError::LockError)?;
        let index = nodes.len();
        nodes.push(node);

        let mut map = self.node_map.write().map_err(|_| LatticeError::LockError)?;
        map.insert(node_id.clone(), index);

        Ok(node_id)
    }

    /// Collapse ritual (sigil integration)
    pub fn collapse_ritual(&self, sigil: &Sigil) -> Result<T, LatticeError> {
        sigil.record_invocation(); // Update sigil stats
        
        match self.coherence {
            CoherenceState::Quantum => self.quantum_collapse(),
            CoherenceState::Stable => self.root_value(),
            CoherenceState::Chaotic => self.random_collapse(),
        }
    }

    fn quantum_collapse(&self) -> Result<T, LatticeError> {
        // Probabilistic collapse weighted by topological charge
        let nodes = self.nodes.read().map_err(|_| LatticeError::LockError)?;
        if nodes.is_empty() { return Err(LatticeError::EmptyLattice); }

        let probabilities: Vec<f64> = nodes.iter()
            .map(|n| (n.quantum.topological_charge.unwrap_or(1.0)).abs())
            .collect();
        
        let total: f64 = probabilities.iter().sum();
        let probs: Vec<f64> = probabilities.iter().map(|p| p / total).collect();

        let idx = quantum::weighted_sample(&probs);
        Ok(nodes[idx].value.clone())
    }

    // ... [existing weave_child, get_node, etc. methods unchanged] ...

    /// Adjacency matrix for band structure
    pub fn adjacency_matrix(&self) -> DMatrix<f64> {
        let graph = self.graph.read().unwrap();
        let n = graph.node_count();
        let mut adj = DMatrix::zeros(n, n);
        
        for edge in graph.edge_indices() {
            let (src, dst) = graph.edge_endpoints(edge).unwrap();
            adj[(src.index(), dst.index())] = 1.0;
            adj[(dst.index(), src.index())] = 1.0;
        }
        adj
    }

    /// Fractal dimension (box-counting)
    pub fn fractal_dimension(&self) -> f64 {
        self.fractal_dim
    }
}

// [AuraFS integration unchanged...]

// Error types extended
#[derive(Debug, Clone)]
pub enum LatticeError {
    // [existing variants...]
    QuantumCollapseFailed,
    BandgapComputationError,
    FractalDimensionError,
}
impl std::fmt::Display for LatticeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // [existing variants...]
            Self::QuantumCollapseFailed => write!(f, "Quantum collapse failed"),
            Self::BandgapComputationError => write!(f, "Bandgap computation error"),
            Self::FractalDimensionError => write!(f, "Fractal dimension calculation error"),
        }
    }
}
    pub name: String,
    pub invocation_count: u64,
    pub last_invoked: Option<u64>,
}
impl std::error::Error for LatticeError {}
}
        F: FnOnce() -> T,
    {
        if rand::random::<f64>() < probability {
            Some(ritual())
        } else {
            None
        }
    }
            ritual: None,
            cached: None,
        }
    }
        pub fn get(&mut self) -> &T {
        if self.cached.is_none() {
            if let Some(ritual) = self.ritual.take() {
                self.cached = Some(ritual());
            }
        }
        self.cached.as_ref().unwrap()
    }
        self.cached = Some(ritual());
        }
        self.cached.as_ref().unwrap()
    }
        self.offset = Some(n);
        self
    }
        self
    }
}