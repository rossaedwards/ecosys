//! Lattice Echo - Flower of Life Visualization
pub struct LatticeEcho {
    lattice: Arc<RwLockrate::core::lattice::Latticerate::core::spinon::Spinon>>>,
}

impl LatticeEcho {
    pub fn visualize(&self) {
        let lattice = self.lattice.read().unwrap();
        quantum_info!("🌸 Flower of Life: D_f={:.3}, H=10^{:.0}", 
            lattice.fractal_dimension(), lattice.hilbert_dimension(2).log10());
    }
}