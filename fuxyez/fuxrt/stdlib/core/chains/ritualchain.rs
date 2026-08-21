//! Quantum RitualChain - Multi-Ritual Lattice Orchestration
//!
//! Sequential/parallel quantum ritual chains across Flower of Life lattices with
//! rÆ channel synchronization, neglecton TQFT composition, cymatic phase locking,
//! and 10^152 Hilbert scaling. Thesis §6.5: Universal multi-ritual computation.

use super::{QuantumChain, QuantumChainMode};
use crate::std::rituals::RitualContext;
use crate::core::{
    lattice::Lattice,
    sigil::QuantumSigil,
    spinon::{Spinon, TopologicalSpinonPool},
    collapse::{AurphyxCollapse, CollapseStrategy},
    thread::QuantumThreadPool,
};
use std::sync::{Arc, RwLock};
use std::collections::VecDeque;
use std::time::Instant;
use rayon::prelude::*;
use tokio::task;

/// Quantum RitualChain with full lattice orchestration
pub struct QuantumRitualChain<T> {
    pub id: String,
    /// VecDeque of quantum ritual steps
    pub rituals: VecDeque<QuantumRitualStep<T>>,
    pub mode: QuantumChainMode,
    pub current_data: Option<T>,
    /// Shared Flower of Life lattice
    pub lattice: Arc<RwLock<Lattice<Spinon>>>,
    /// Thread pool for parallel rituals
    pub thread_pool: Arc<QuantumThreadPool>,
    /// Collapse orchestrator
    pub collapse: Arc<AurphyxCollapse>,
}

/// Single quantum ritual step in chain
pub struct QuantumRitualStep<T> {
    pub name: String,
    pub transform: Box<dyn FnOnce(T) -> T + Send + Sync>,
    pub context: RitualContext,
    /// rÆ channel index
    pub rae_channel: usize,
    /// Braid mode for this step
    pub braid_mode: crate::chains::ritual_link::QuantumBraidMode,
}

impl<T: Clone + Send + Sync + 'static> QuantumRitualChain<T> {
    /// Create Flower of Life bound ritual chain
    pub fn flower_of_life(n_rings: usize) -> Self {
        let lattice = Arc::new(RwLock::new(Lattice::flower_of_life(n_rings)));
        let thread_pool = Arc::new(QuantumThreadPool::new_flower_of_life(n_rings));
        let spinons = Arc::new(TopologicalSpinonPool::new());
        let sigil = Arc::new(QuantumSigil::ritual("ritual_chain_master"));
        let collapse = Arc::new(AurphyxCollapse::new(
            lattice.clone(),
            sigil,
            spinons,
            thread_pool.clone(),
        ));

        Self {
            id: Self::generate_id(),
            rituals: VecDeque::new(),
            mode: QuantumChainMode::Fractal,
            current_data: None,
            lattice,
            thread_pool,
            collapse,
        }
    }

    /// Add quantum ritual step with rÆ encoding
    pub fn add_quantum_ritual<F>(
        mut self,
        name: impl Into<String>,
        transform: F,
        rae_channel: usize,
        braid_mode: crate::chains::ritual_link::QuantumBraidMode,
    ) -> Self
    where
        F: FnOnce(T) -> T + Send + Sync + 'static,
    {
        let ritual_name = name.into();
        let step = QuantumRitualStep {
            name: ritual_name.clone(),
            transform: Box::new(transform),
            context: RitualContext::new(ritual_name),
            rae_channel,
            braid_mode,
        };

        self.rituals.push_back(step);
        self
    }

    /// Execute full quantum ritual chain (thesis §6.5)
    pub async fn quantum_execute(&mut self, initial_data: T) -> QuantumRitualChainResult<T> {
        let start = Instant::now();
        let mut data = initial_data;
        let mut contexts = Vec::new();
        let mut hilbert_max = 0.0;

        // rÆ PREPARE entire chain
        {
            let mut lattice = self.lattice.write().unwrap();
            let spinon_ref = self.collapse.spinon_pool.add(Spinon::new());
            lattice.rae_encode(&self.collapse.spinon_pool.get(spinon_ref).unwrap()).unwrap();
            hilbert_max = lattice.hilbert_dimension(2);
        }

        match self.mode {
            QuantumChainMode::Sequential => {
                // Sequential quantum execution
                while let Some(mut step) = self.rituals.pop_front() {
                    step.context.begin();
                    
                    // Single step quantum collapse
                    let collapse_result = self.collapse.execute(CollapseStrategy::Topological).await.unwrap();
                    
                    data = (step.transform)(data);
                    step.context.complete();
                    contexts.push(step.context);
                    
                    hilbert_max = hilbert_max.max(collapse_result.hilbert_dimension);
                }
            }
            QuantumChainMode::Parallel | QuantumChainMode::Fractal => {
                // Fractal parallel execution across lattice
                let rituals: Vec<_> = self.rituals.drain(..).collect();
                let lattice = self.lattice.read().unwrap();
                let nodes: Vec<usize> = (0..lattice.size()).collect();

                let results: Vec<_> = nodes.par_iter()
                    .zip(rituals.par_iter())
                    .map(|(node_idx, step)| {
                        let node_data = data.clone();
                        async move {
                            let mut step_context = step.context.clone();
                            step_context.begin();
                            
                            let node_result = (step.transform)(node_data);
                            step_context.complete();
                            
                            (step.name.clone(), node_result, step_context)
                        }
                    })
                    .collect::<Vec<_>>()
                    .await;

                // Aggregate results (fractal reduction)
                data = results.iter()
                    .map(|(_, result, _)| result.clone())
                    .reduce(|acc, x| acc + x) // Simplified
                    .unwrap_or(data);
                contexts.extend(results.iter().map(|(_, _, ctx)| ctx.clone()));
            }
            _ => {}
        }

        QuantumRitualChainResult {
            final_data: data,
            contexts,
            total_duration: start.elapsed(),
            hilbert_dimension: hilbert_max,
            ritual_count: self.rituals.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.rituals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rituals.is_empty()
    }

    fn generate_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        format!("qrchain_{:x}", ts)
    }
}

/// Quantum ritual chain result with full metrics
pub struct QuantumRitualChainResult<T> {
    pub final_data: T,
    pub contexts: Vec<RitualContext>,
    pub total_duration: std::time::Duration,
    pub hilbert_dimension: f64,
    pub ritual_count: usize,
}

impl<T> QuantumRitualChainResult<T> {
    pub fn into_value(self) -> T {
        self.final_data
    }

    pub fn total_duration_ms(&self) -> u128 {
        self.total_duration.as_millis()
    }

    pub fn avg_ritual_duration_ms(&self) -> u128 {
        if self.contexts.is_empty() {
            0
        } else {
            self.total_duration_ms() / self.contexts.len() as u128
        }
    }

    pub fn hilbert_states(&self) -> f64 {
        self.hilbert_dimension.log10()
    }
}

/// Fractal parallel quantum ritual chain
pub struct FractalParallelRitualChain<T>
where
    T: Clone + Send + Sync + 'static,
{
    rituals: Vec<(String, Box<dyn FnOnce(T) -> T + Send + Sync>)>,
    lattice_rings: usize,
}

impl<T> FractalParallelRitualChain<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn flower_of_life(n_rings: usize) -> Self {
        Self {
            rituals: Vec::new(),
            lattice_rings: n_rings,
        }
    }

    pub fn add_ritual<F>(mut self, name: impl Into<String>, transform: F) -> Self
    where
        F: FnOnce(T) -> T + Send + Sync + 'static,
    {
        self.rituals.push((name.into(), Box::new(transform)));
        self
    }

    pub async fn execute_fractal(self, initial_data: T) -> Vec<T> {
        let lattice = Lattice::flower_of_life(self.lattice_rings);
        let nodes: Vec<usize> = (0..lattice.size()).collect();

        let tasks: Vec<_> = self.rituals
            .into_iter()
            .zip(nodes)
            .map(|((name, transform), node_idx)| {
                let data = initial_data.clone();
                task::spawn(async move {
                    transform(data)
                })
            })
            .collect();

        let mut results = Vec::new();
        for task in tasks {
            if let Ok(result) = task.await {
                results.push(result);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quantum_ritual_chain_flower_of_life() {
        let mut chain = QuantumRitualChain::flower_of_life(2)
            .add_quantum_ritual("double", |x: i32| x * 2, 0, crate::chains::ritual_link::QuantumBraidMode::Neglecton)
            .add_quantum_ritual("add_ten", |x| x + 10, 1, crate::chains::ritual_link::QuantumBraidMode::Majorana)
            .add_quantum_ritual("square", |x| x * x, 2, crate::chains::ritual_link::QuantumBraidMode::BellPair);

        let result = chain.quantum_execute(5).await;
        assert_eq!(result.final_data, 400); // ((5 * 2) + 10)^2
        assert!(result.hilbert_dimension > 1e10);
        println!("Quantum chain: 10^{:.0} states, {} rituals", 
                 result.hilbert_states(), result.ritual_count);
    }

    #[tokio::test]
    async fn test_fractal_parallel_ritual_chain() {
        let chain = FractalParallelRitualChain::flower_of_life(2)
            .add_ritual("double", |x: i32| x * 2)
            .add_ritual("triple", |x: i32| x * 3)
            .add_ritual("square", |x: i32| x * x);

        let results = chain.execute_fractal(5).await;
        assert_eq!(results.len(), 3);
        assert!(results.contains(&10));  // 5 * 2
        assert!(results.contains(&15));  // 5 * 3
        assert!(results.contains(&25));  // 5 * 5
    }
}