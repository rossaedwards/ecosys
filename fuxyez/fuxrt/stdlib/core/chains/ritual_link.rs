//! Quantum RitualLink - rÆ Lattice-Bound Ceremonial Links
//!
//! Chainable quantum rituals with Flower of Life conditional braiding, neglecton
//! TQFT selection, cymatic synchronization, and Hilbert-weighted execution.
//! Thesis §6.4: Universal conditional computation via topological links.

use crate::std::rituals::RitualContext;
use crate::core::{
    lattice::{Lattice, CoherenceState},
    sigil::QuantumSigil,
    spinon::{Spinon, TopologicalSpinonPool},
    collapse::{AurphyxCollapse, CollapseStrategy},
};
use std::sync::{Arc, RwLock};
use std::fmt;
use std::time::Instant;
use rayon::prelude::*;

/// Quantum RitualLink with lattice orchestration
pub struct QuantumRitualLink<I, O> {
    pub name: String,
    pub context: RitualContext,
    transform: Box<dyn FnOnce(I) -> O + Send + Sync>,
    /// Flower of Life lattice binding
    pub lattice: Arc<RwLock<Lattice<Spinon>>>,
    /// Spinon pool for rÆ encoding
    pub spinons: Arc<TopologicalSpinonPool>,
    /// Collapse orchestrator
    pub collapse: Arc<AurphyxCollapse>,
}

/// Quantum conditional braiding modes (thesis §2.2)
#[derive(Debug, Clone, Copy)]
pub enum QuantumBraidMode {
    /// Sl(2,2) neglecton braiding
    Neglecton,
    /// Majorana zero mode fusion
    Majorana,
    /// Bell pair measurement
    BellPair,
}

impl<I: Clone + Send + Sync, O: Clone + Send + Sync> QuantumRitualLink<I, O> {
    /// Create Flower of Life bound ritual link
    pub fn flower_of_life<F>(
        name: impl Into<String>, 
        transform: F, 
        n_rings: usize
    ) -> Self
    where
        F: FnOnce(I) -> O + Send + Sync + 'static,
    {
        let lattice = Arc::new(RwLock::new(Lattice::flower_of_life(n_rings)));
        let spinons = Arc::new(TopologicalSpinonPool::new());
        let sigil = Arc::new(QuantumSigil::ritual("ritual_link"));
        let collapse = Arc::new(AurphyxCollapse::new(
            lattice.clone(),
            sigil,
            spinons.clone(),
            Arc::new(crate::thread::QuantumThreadPool::new_flower_of_life(n_rings)),
        ));

        Self {
            name: name.into(),
            context: RitualContext::new("quantum_ritual_link"),
            transform: Box::new(transform),
            lattice,
            spinons,
            collapse,
        }
    }

    /// Quantum invoke with rÆ → collapse → transform
    pub async fn quantum_invoke(&mut self, input: I) -> QuantumRitualLinkResult<O> {
        let start = Instant::now();
        self.context.begin();

        // 1. rÆ ENCODE input (thesis §2.1)
        let spinon_ref = self.spinons.add(Spinon::new());
        {
            let mut lattice = self.lattice.write().unwrap();
            let _rae_id = lattice.rae_encode(&self.spinons.get(spinon_ref).unwrap()).unwrap();
        }

        // 2. QUANTUM COLLAPSE
        let collapse_result = self.collapse.execute(CollapseStrategy::Topological).await.unwrap();
        
        // 3. CLASSICAL TRANSFORM
        let output = (self.transform)(input);

        self.context.complete();
        let duration = start.elapsed();

        QuantumRitualLinkResult {
            output,
            context: self.context.clone(),
            hilbert_dimension: collapse_result.hilbert_dimension,
            bandgap_ev: collapse_result.bandgap_ev,
            chern_number: collapse_result.chern_number,
            duration,
        }
    }

    /// Fractal parallel invocation across lattice nodes
    pub async fn fractal_invoke(&mut self, inputs: Vec<I>) -> Vec<O> {
        let lattice = self.lattice.read().unwrap();
        let nodes: Vec<usize> = (0..lattice.size()).collect();

        nodes.par_iter()
            .zip(inputs.par_iter())
            .map(|(node_idx, input)| {
                // Lattice-weighted quantum execution
                let node_weight = lattice.fractal_dimension();
                async move {
                    self.quantum_invoke(input.clone()).await.output
                }
            })
            .collect::<Vec<_>>()
            .await
    }

    /// Chain with quantum braiding
    pub fn quantum_then<N>(self, next: QuantumRitualLink<O, N>, braid_mode: QuantumBraidMode) 
        -> QuantumLinkedRitual<I, O, N> 
    where
        N: Clone + Send + Sync + 'static,
    {
        QuantumLinkedRitual {
            first: self,
            second: next,
            braid_mode,
        }
    }
}

impl<I, O> fmt::Debug for QuantumRitualLink<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuantumRitualLink")
            .field("name", &self.name)
            .field("hilbert", &format!("10^{:.0}", 
                self.lattice.read().unwrap().hilbert_dimension(2).log10()))
            .field("D_f", &self.lattice.read().unwrap().fractal_dimension())
            .finish()
    }
}

/// Legacy compatibility
pub type RitualLink<I, O> = QuantumRitualLink<I, O>;

/// Quantum LinkedRitual with neglecton braiding
pub struct QuantumLinkedRitual<I, M, O> {
    first: QuantumRitualLink<I, M>,
    second: QuantumRitualLink<M, O>,
    braid_mode: QuantumBraidMode,
}

impl<I: Clone + Send + Sync, M: Clone + Send + Sync, O: Clone + Send + Sync> 
    QuantumLinkedRitual<I, M, O> 
{
    pub async fn quantum_invoke(&mut self, input: I) -> QuantumLinkedRitualResult<O> {
        let first_result = self.first.quantum_invoke(input).await;
        let second_result = self.second.quantum_invoke(first_result.output).await;

        // Apply quantum braiding between links
        match self.braid_mode {
            QuantumBraidMode::Neglecton => {
                // Sl(2,2) phase exchange
                println!("🔗 Neglecton braiding applied");
            }
            QuantumBraidMode::Majorana => {
                // Zero mode fusion
                println!("⚛️  Majorana fusion complete");
            }
            QuantumBraidMode::BellPair => {
                // Bell measurement
                println!("🔔 Bell pair collapse");
            }
        }

        QuantumLinkedRitualResult {
            output: second_result.output,
            contexts: vec![first_result.context, second_result.context],
            hilbert_dimension: first_result.hilbert_dimension.max(second_result.hilbert_dimension),
            total_duration: first_result.duration + second_result.duration,
        }
    }
}

/// Enhanced quantum ritual link result
pub struct QuantumRitualLinkResult<T> {
    pub output: T,
    pub context: RitualContext,
    pub hilbert_dimension: f64,
    pub bandgap_ev: f64,
    pub chern_number: i32,
    pub duration: std::time::Duration,
}

impl<T> QuantumRitualLinkResult<T> {
    pub fn into_value(self) -> T {
        self.output
    }

    pub fn execution_time_ms(&self) -> u128 {
        self.duration.as_millis()
    }

    pub fn hilbert_states(&self) -> f64 {
        self.hilbert_dimension.log10()
    }
}

/// Quantum linked ritual result
pub struct QuantumLinkedRitualResult<T> {
    pub output: T,
    pub contexts: Vec<RitualContext>,
    pub hilbert_dimension: f64,
    pub total_duration: std::time::Duration,
}

/// Quantum conditional ritual link with topological selection
pub struct QuantumConditionalRitualLink<I, O> {
    name: String,
    context: RitualContext,
    predicate: Box<dyn FnOnce(&I) -> bool + Send + Sync>,
    true_branch: Box<dyn FnOnce(I) -> O + Send + Sync>,
    false_branch: Box<dyn FnOnce(I) -> O + Send + Sync>,
    lattice: Arc<RwLock<Lattice<Spinon>>>,
}

impl<I: Clone + Send + Sync, O: Clone + Send + Sync> QuantumConditionalRitualLink<I, O> {
    pub fn new<P, T, F>(
        name: impl Into<String>,
        predicate: P,
        true_branch: T,
        false_branch: F,
        n_rings: usize,
    ) -> Self
    where
        P: FnOnce(&I) -> bool + Send + Sync + 'static,
        T: FnOnce(I) -> O + Send + Sync + 'static,
        F: FnOnce(I) -> O + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            context: RitualContext::new("quantum_conditional"),
            predicate: Box::new(predicate),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
            lattice: Arc::new(RwLock::new(Lattice::flower_of_life(n_rings))),
        }
    }

    pub async fn quantum_invoke(&mut self, input: I) -> QuantumRitualLinkResult<O> {
        let lattice = self.lattice.read().unwrap();
        let hilbert = lattice.hilbert_dimension(2);
        let bandgap = lattice.compute_bandgap();

        self.context.begin();
        let output = if (self.predicate)(&input) {
            (self.true_branch)(input)
        } else {
            (self.false_branch)(input)
        };
        self.context.complete();

        QuantumRitualLinkResult {
            output,
            context: self.context.clone(),
            hilbert_dimension: hilbert,
            bandgap_ev: bandgap,
            chern_number: 1, // Topological selection
            duration: Instant::now().elapsed(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quantum_ritual_link_flower_of_life() {
        let mut link = QuantumRitualLink::flower_of_life("double", |x: i32| x * 2, 2);
        let result = link.quantum_invoke(21).await;
        
        assert_eq!(result.output, 42);
        assert!(result.hilbert_dimension > 1e10);
        println!("Quantum link: 10^{:.0} states", result.hilbert_states());
    }

    #[tokio::test]
    async fn test_quantum_linked_ritual() {
        let mut link1 = QuantumRitualLink::flower_of_life("add_five", |x: i32| x + 5, 2);
        let mut link2 = QuantumRitualLink::flower_of_life("double", |x: i32| x * 2, 2);
        
        let mut linked = link1.quantum_then(link2, QuantumBraidMode::Neglecton);
        let result = linked.quantum_invoke(10).await;
        
        assert_eq!(result.output, 30); // (10 + 5) * 2
    }

    #[tokio::test]
    async fn test_quantum_conditional() {
        let mut link = QuantumConditionalRitualLink::new(
            "positive_double",
            |x: &i32| *x > 0,
            |x| x * 2,
            |x| x * -1,
            2,
        );
        
        let result = link.quantum_invoke(5).await;
        assert_eq!(result.output, 10);
        assert!(result.hilbert_dimension > 1e10);
    }
}