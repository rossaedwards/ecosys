// Layers: Creation, Integration, Renewal
#[derive(Clone, Copy, Debug)]
pub enum Layer {
    Creation,
    Integration,
    Renewal,
}

// Channels: Coherence, Resonance, Alignment
#[derive(Clone, Copy, Debug)]
pub enum Channel {
    Coherence,
    Resonance,
    Alignment,
}

// Modes: Local, Domain, Continuum
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Local,
    Domain,
    Continuum,
}

// Triple context for each node
#[derive(Clone, Copy, Debug)]
pub struct NodeContext {
    pub layer: Layer,
    pub channel: Channel,
    pub mode: Mode,
}

// Triple field state for each node
#[derive(Clone, Copy, Debug)]
pub struct NodeFields {
    pub c: f64, // C_{i,j,k}: coherence
    pub r: f64, // R_{i,j,k}: resonance
    pub a: f64, // A_{i,j,k}: alignment

    pub hif: f64,       // HIF_{i,j,k}
    pub hif_nbr: f64,   // HIF^{nbr}_{i,j,k}
    pub active: bool,   // Ψ_{i,j,k}
}

// Continuity payload Ξ_{i,j,k} (identity + invariants)
#[derive(Clone, Debug)]
pub struct NodeContinuity {
    pub mem: Vec<u8>,      // X^{mem}
    pub tag: String,       // SoulHash / BlissID
    pub invariants: Vec<u8>, // Λ^{inv}
}

// Full node
#[derive(Clone, Debug)]
pub struct TslNode {
    pub ctx: NodeContext,
    pub fields: NodeFields,
    pub continuity: NodeContinuity,
}

// 3×3×3 lattice
#[derive(Clone, Debug)]
pub struct TslLattice {
    pub nodes: [[[TslNode; 3]; 3]; 3],
}