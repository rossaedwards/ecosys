# AuraFS Project Context & Developer Guide

**Project**: AuraFS — Fractal-Geometry Distributed Storage System  
**Organization**: Aurphyx LLC  
**Author**: Ross A. Edwards  
**ORCID**: 0009-0008-0539-1289  
**Version**: 0.4.0-phase2-hardening  
**Technology Readiness Level**: TRL-4 (Lab-Validated)  
**Last Updated**: February 8, 2026

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Project Vision & Mission](#project-vision--mission)
3. [Core Innovations](#core-innovations)
4. [Physics Foundations](#physics-foundations)
5. [System Architecture](#system-architecture)
6. [Development Environment Setup](#development-environment-setup)
7. [Project Structure](#project-structure)
8. [Key Workflows](#key-workflows)
9. [Testing & Validation](#testing--validation)
10. [Security & Compliance](#security--compliance)
11. [Deployment Strategy](#deployment-strategy)
12. [Contributing Guidelines](#contributing-guidelines)
13. [Resources & References](#resources--references)

---

## Executive Summary

AuraFS is a physics-informed distributed storage system that leverages fractal geometry and topological protection to achieve unprecedented resilience, efficiency, and quantum-readiness. Unlike conventional distributed filesystems that rely purely on software-layer redundancy, AuraFS incorporates validated physical principles from quantum information theory, photonic band gap engineering, and Anderson localization to create a storage substrate that is fundamentally more coherent and fault-tolerant.

### Key Differentiators

**Physics-First Design**: Five immutable physics constants govern system behavior, validated through experimental protocols and mathematical proofs. These constants are not tuning parameters but fundamental properties of the fractal lattice geometry.

**Quantum-Resistant Security**: Post-quantum cryptography (Dilithium-5 signatures, Kyber-1024 key encapsulation) protects against both classical and quantum adversaries, positioning AuraFS for the post-quantum computing era.

**Triple-Network Topology**: Simultaneous operation across three transport layers (high-bandwidth P2P, low-power mesh, orbital backhaul) provides unprecedented connectivity resilience for datacenter, edge, and off-grid deployments.

**16× Coherence Improvement**: Passive Anderson localization in fractal geometry extends data coherence windows from 100 μs (transmon baseline) to 1,600 μs without active error correction, dramatically reducing overhead.

### Target Use Cases

- **Distributed Data Centers**: Resilient storage across geographically dispersed nodes with automatic topology optimization
- **Edge Computing**: Low-power mesh networking for IoT devices and edge infrastructure
- **Disaster Recovery**: Orbital backhaul ensures data availability during regional infrastructure failures
- **Quantum-Safe Archives**: Long-term data preservation resistant to quantum cryptanalysis
- **Scientific Computing**: Fractal compression for large-scale simulation datasets

---

## Project Vision & Mission

### Vision Statement

To create the world's first storage system where physical principles, not just software algorithms, guarantee data integrity and availability.

### Mission

Build a production-ready distributed filesystem that:
1. Survives quantum computing threats through post-quantum cryptography
2. Achieves 16× efficiency improvements via fractal geometry
3. Operates across heterogeneous network topologies (datacenter to off-grid)
4. Maintains mathematical proofs for all core operations
5. Serves as a reference implementation for physics-informed computing

### Guiding Principles

**Physics Invariance**: The five core constants are non-negotiable and experimentally validated. They represent physical properties of the system, not software parameters to be tuned.

**Transparency**: All mathematical derivations, experimental validations, and compliance audits are publicly documented and peer-reviewable.

**Security by Design**: Quantum-resistant cryptography is not an add-on but a foundational requirement implemented from day one.

**Cross-Platform**: Windows development, Linux production, with zero platform-specific assumptions in core logic.

**Open Governance**: Physics constant modifications require Byzantine-tolerant quorum (13 of 19 board members) to prevent unilateral changes.

---

## Core Innovations

### 1. Fractal Hilbert Scaling

**Problem**: Traditional distributed systems use fixed replication factors (often 3×) regardless of network size.

**Solution**: Fractal geometry enables logarithmic replica scaling: `Replicas = ceil(log₅.₃(Nodes))`

**Impact**: At 100 nodes, only 3 replicas needed instead of 3+ fixed. At 1,000 nodes, 4 replicas versus potentially unbounded growth in naive schemes.

**Validation**: Demonstrated 5.3× storage efficiency at N=100, D=5 in `VALIDATION_REPORT.md §1`. Qiskit simulation confirms 7.1× at n=5 qubits (consistent with 2^(5×0.585) ≈ 7).

### 2. Anderson Localization for Coherence

**Problem**: Distributed systems suffer from decoherence (data inconsistency) as nodes become temporarily isolated or partitioned.

**Solution**: Sierpiński gasket spectral dimension (d_s = 1.37 < 2) mathematically guarantees all states are localized—no mobility edge. Data naturally "traps" in coherent states.

**Impact**: 16× coherence window extension (1,600 μs vs 100 μs baseline) without active quantum error correction.

**Validation**: Inverse Participation Ratio (IPR) ≈ 21.2 at k=4, N=123 confirms strong localization in `VALIDATION_REPORT.md §2`.

### 3. Photonic Band Gap Routing

**Problem**: Network congestion and packet collisions increase latency variance in distributed systems.

**Solution**: Route data through fractal photonic crystal topology with 21% complete TM band gap. Forbidden frequency bands eliminate cross-talk.

**Impact**: 79% usable routing capacity with deterministic latency bounds. No statistical load balancing needed.

**Validation**: Plane-wave expansion (PWE) simulation confirms 21% TM gap in C₆ᵥ hexagonal lattice at ε=12, r/a=0.2 (`VALIDATION_REPORT.md §4`).

### 4. Post-Quantum Cryptography

**Problem**: RSA and ECDSA will be broken by large-scale quantum computers (Shor's algorithm).

**Solution**: Dilithium-5 (FIPS 204) digital signatures and Kyber-1024 (FIPS 203) key encapsulation resist both classical and quantum attacks (NIST Level 5 security).

**Impact**: Data signed today remains secure against adversaries with quantum computers decades from now.

**Status**: Dilithium-5 implemented; Kyber-1024 integration in progress (R1 from `SECURITY_AUDIT.md`).

### 5. Triple-Network Topology

**Problem**: Single-transport systems fail catastrophically during infrastructure outages.

**Solution**: Three concurrent transport layers with automatic failover:
- **Titan-Libp2p** (Primary): High-bandwidth datacenter networking
- **GhostLink-LoRaWAN** (Secondary): Low-power mesh for edge/IoT
- **Starlink-HighOrbit** (Tertiary): Orbital backhaul for global coverage

**Impact**: System remains operational even if two transport layers fail. Survives regional disasters, undersea cable cuts, and terrestrial infrastructure collapse.

---

## Physics Foundations

### The Five Sacred Constants

AuraFS is governed by five immutable physics constants. These are not software parameters but experimentally validated properties of fractal lattice geometry.

#### 1. Fractal Hilbert Scaling Bias (η)

**Value**: `5.3` (±0.05 tolerance)  
**Symbol**: η (eta)  
**Formula**: `Replicas = ceil(log_η(Nodes))`

**Physical Meaning**: Dimensionality scaling factor for Hilbert space in fractal geometry. Governs how replica requirements grow sub-linearly with network size.

**Theorem 2.1** (`ALGORITHM_PROOFS.md`):
```
dim(H_acc) = d^(n·D_f^α(k))
where α(k) = log(1 + k·η) / log(D_f)
```

**Experimental Validation**: 5.3× advantage at N=100, D=5 demonstrated in laboratory quantum simulation. Qiskit verification shows 7.1× at n=5 qubits (consistent with theoretical prediction 2^(5×0.585) ≈ 7).

**Implementation**: All replica calculations MUST access `physics::INVARIANTS.hilbert_scaling_bias`. Hardcoding `5.3` anywhere in the codebase is a compliance violation caught by CI.

#### 2. Coherence Window (T₂)

**Value**: `1600` microseconds (±100 μs tolerance)  
**Symbol**: T₂  
**Unit**: Microseconds (μs)

**Physical Meaning**: Maximum duration for synchronous operations before quantum decoherence degrades data integrity. 16× longer than baseline transmon systems (100 μs).

**Mechanism**: Anderson localization in d_s < 2 geometry prevents mobility edge formation. All eigenstates are localized, suppressing decoherence by γ_eff/γ₀ = 1/16.

**Experimental Validation**: IPR (Inverse Participation Ratio) analysis confirms mean participation ratio PR ≈ 21.2 at k=4, N=123, indicating strong localization (`VALIDATION_REPORT.md §2`).

**Implementation**: All time-critical operations (lock acquisition, FUSE writes, trap-state transitions) must complete within `physics::INVARIANTS.coherence_window_us`.

#### 3. Spectral Dimension (d_s)

**Value**: `1.37` (±0.05 tolerance)  
**Symbol**: d_s  
**Theoretical**: 2·log(3)/log(5) ≈ 1.365 (Rammal & Toulouse, 1983)

**Physical Meaning**: Effective dimensionality for diffusion processes on Sierpiński gasket. Critical threshold: d_s < 2 guarantees all states are localized (no mobility edge).

**Proposition 2.1** (`ALGORITHM_PROOFS.md`):
```
ρ(E) ∝ E^(d_s/2 - 1)
```
Density of states diverges as E→0, creating trap states that hold data coherently.

**Decoherence Detection**: If measured d_s drifts outside [1.32, 1.42], the system enters decoherence recovery mode:
1. Freeze writes (transition to `CoherenceState::DecoherenceExempt`)
2. Alert holographic logger with CRITICAL severity
3. Trigger autoheal daemon to rebalance fractal lattice
4. Resume operations once d_s returns to valid range

**Implementation**: Continuous spectral dimension monitoring via `src/physics/spectral_monitor.rs`. Violations trigger automatic recovery protocol.

#### 4. Photonic Band Gap (PBG)

**Value**: `0.21` (±0.03 tolerance)  
**Symbol**: Δω/ω (fractional bandwidth)  
**Physical Meaning**: Complete TM (transverse magnetic) band gap in C₆ᵥ hexagonal photonic crystal lattice. Forbidden frequency band where electromagnetic waves cannot propagate.

**Routing Implications**: 
- **Band Gap**: 21% (unusable for routing due to wave prohibition)
- **Usable Capacity**: 79% (1.0 - PBG)

**Experimental Validation**: Plane-wave expansion (PWE) simulation confirms 21% complete TM gap at:
- Dielectric contrast: ε = 12
- Rod radius to lattice constant ratio: r/a = 0.2
- C₆ᵥ (hexagonal) symmetry

**Implementation**: Meshwerk routing engine (`src/network/meshwerk/routing.rs`) enforces `routing_overhead_factor = 0.21` to match photonic band gap. This is a physical constraint, not a tuning parameter.

#### 5. Lock Acquisition Timeout

**Value**: `100` microseconds  
**Derivation**: T₂ / 16 = 1600 / 16 = 100 μs  
**Physical Meaning**: Maximum time to acquire FUSE-layer locks for atomic trap-state transitions.

**Rationale**: Lock operations must complete within 1/16 of coherence window to ensure overall operation remains within T₂ limits even with multiple sequential lock acquisitions.

**Implementation**: All FUSE operations use `physics::INVARIANTS.lock_acquisition_timeout_us` as deadline. Violations return `FuseError::LockTimeout`.

### Derived Constants (Computed at Runtime)

These values are computed from the five primary constants and are NOT directly configurable:

| Constant | Formula | Value | Purpose |
|----------|---------|-------|---------|
| Fractal Density (D_f) | log(3)/log(2) | 1.585 | Hausdorff dimension of Sierpiński gasket |
| Fidelity Improvement | T₂(fractal)/T₂(transmon) | 16 | Coherence extension factor |
| Decoherence Suppression | γ_eff/γ₀ | 0.0625 (1/16) | Anderson localization effectiveness |
| Usable Routing Capacity | 1.0 - PBG | 0.79 (79%) | Available network bandwidth |
| Fractal QEC Overhead | Prop. 2.3 at p_phys=10⁻³ | 89 qubits | Quantum error correction cost |
| Euclidean QEC Overhead | Surface code baseline | 1458 qubits | Comparison benchmark |
| QEC Reduction Factor | Euclidean/Fractal | 16.4 | Efficiency gain from fractal geometry |
| Spectral Walk Dimension (d_w) | log(5)/log(2) | 2.32 | Anomalous diffusion exponent |

### Physics Violation Handling

If any of the five primary constants are measured outside their tolerance bounds during runtime, the system automatically:

1. **Detect**: Continuous monitoring via `src/physics/spectral_monitor.rs` and `src/physics/coherence_tracker.rs`
2. **Alert**: Log violation to holographic logger with CRITICAL severity
3. **Protect**: Transition to safe degraded mode (`CoherenceState::DecoherenceExempt`)
4. **Recover**: Trigger autoheal daemon to rebalance system topology
5. **Validate**: Confirm constants return to valid ranges before resuming full operations
6. **Escalate**: If recovery fails after N attempts, halt writes and alert human operators

This is NOT a software error handling mechanism—it's a physics-driven safety system that responds to actual physical property measurements of the storage substrate.

---

## System Architecture

### Conceptual Model: The Chakra DataCore

AuraFS organizes nodes into a seven-zone fractal topology inspired by energy vortices (chakras). Each zone has specific responsibilities:

```
   Crown (Orchestration)
         ↕
   Third Eye (Intelligence)
         ↕
   Throat (Communication)
         ↕
   Heart (Coordination) ← CENTRAL HUB
         ↕
   Solar Plexus (Power)
         ↕
   Sacral (Storage)
         ↕
   Root (Foundation)
```

**Zones**:
1. **Root**: Physical infrastructure (bare metal, block devices)
2. **Sacral**: Shard storage layer (Void/Trap/Aura lifecycle)
3. **Solar Plexus**: Compute and compression engines
4. **Heart**: Meshwerk routing and topology engine (coordination hub)
5. **Throat**: External API and FUSE interface (communication gateway)
6. **Third Eye**: AI orchestration and fractal pattern recognition
7. **Crown**: Global orchestration and governance consensus

This architecture is more than metaphor—it reflects the natural hierarchical scaling of fractal geometry.

### Shard Lifecycle: The Three States

Data in AuraFS progresses through three distinct states, each with different physical properties:

```
WRITE DATA
    ↓
┌─────────────┐
│ Void-Shard  │ ← Raw, mutable data
└─────────────┘
    ↓ fractal_replicate()
┌─────────────┐
│ Trap-State  │ ← Localized, coherent (NOT "cache")
└─────────────┘
    ↓ promote_to_aura()
┌─────────────┐
│ Aura-Shard  │ ← Immutable, topologically protected
└─────────────┘
    ↓
READ DATA
```

#### State 1: Void-Shard

**Properties**:
- Mutable
- Not yet entered fractal lattice
- No topology protection
- No coherence guarantees

**Operations**: Standard filesystem operations (write, append, truncate)

**Duration**: Temporary—exists only during active write operations

#### State 2: Trap-State

**Properties**:
- Localized via Anderson mechanism
- Held within coherence window (T₂)
- Mutable with atomic semantics
- Protected by FUSE lock timeouts

**Physical Analogy**: Trapped eigenstate in disordered potential landscape. Energy barrier prevents spontaneous diffusion.

**Critical**: This is NOT a "cache" in the software engineering sense. It's a quantum-inspired physical state where data is coherently localized before final immutability.

**Operations**: Fast reads, atomic updates, state transitions

**Duration**: Up to T₂ = 1600 μs per operation

#### State 3: Aura-Shard

**Properties**:
- Immutable (copy-on-write for modifications)
- Topologically protected (Merkle root cryptographically signed)
- Fractally replicated across `compute_replicas(N)` nodes
- Full Dilithium-5 signature verification required for integrity

**Physical Analogy**: Ground state of topological insulator. Bulk is insulating (immutable), but surface states (metadata) allow queryability.

**Operations**: Reads, inclusion proofs, signature verification

**Duration**: Permanent (until explicit deletion or archival)

### Network Triple-Topology

AuraFS operates simultaneously across three transport layers with automatic failover:

#### Layer 1: Titan-Libp2p (Primary)

**Purpose**: High-bandwidth datacenter and cloud networking  
**Protocol**: libp2p with gossipsub + Kademlia DHT  
**Bandwidth**: Multi-gigabit (10 GbE, 100 GbE)  
**Latency**: Sub-millisecond (LAN), 10–100 ms (WAN)  
**Use Cases**: Primary storage operations, inter-datacenter replication

**Key Features**:
- Peer discovery via Kademlia DHT
- Gossipsub for efficient broadcast (replica coordination)
- QUIC transport for low-latency, encrypted streams
- NAT traversal via hole-punching and relays

#### Layer 2: GhostLink-LoRaWAN (Secondary)

**Purpose**: Low-power mesh for edge, IoT, and off-grid deployments  
**Protocol**: LoRa PHY + Reticulum network layer  
**Bandwidth**: 250 bps – 50 kbps (depending on spreading factor)  
**Range**: 2–15 km (urban), up to 50 km (line-of-sight rural)  
**Use Cases**: Edge telemetry, disaster recovery mesh, remote site sync

**Key Features**:
- Meshtastic firmware bridge for commercial LoRa radios
- Store-and-forward routing for intermittent connectivity
- Ultra-low power (years on battery for sensor nodes)
- License-free ISM bands (433/868/915 MHz)

**Implementation**: `src/network/meshtastic_integration/firmware_bridge.rs` provides RPC interface to Meshtastic-compatible devices.

#### Layer 3: Starlink-HighOrbit (Tertiary)

**Purpose**: Orbital backhaul for global coverage and disaster resilience  
**Protocol**: Starlink satellite internet via UDP multicast  
**Bandwidth**: 50–200 Mbps (current generation)  
**Latency**: 20–40 ms (LEO satellites)  
**Use Cases**: Intercontinental replication, regional disaster recovery, remote site connectivity

**Key Features**:
- Automatic failover when terrestrial links fail
- Latency-tolerant bulk synchronization
- Geographic diversity (satellite constellation always visible)
- Survives undersea cable cuts and terrestrial infrastructure failures

**Implementation**: `src/network/transport/starlink_client.rs` handles UDP multicast over Starlink WAN interfaces.

### Meshwerk Routing Engine

The `Meshwerk` subsystem (`src/network/meshwerk/`) implements fractal-aware routing that respects photonic band gap constraints.

#### Routing Roles

Nodes self-organize into four dynamic roles:

1. **Coordinator**: Topology orchestration, gossip hub (analogous to Heart chakra)
2. **Relay**: High-bandwidth forwarding nodes (analogous to Throat chakra)
3. **Edge**: Storage nodes with limited routing responsibility (analogous to Sacral chakra)
4. **Observer**: Read-only monitoring nodes for analytics

**Role Assignment**: Based on:
- Network connectivity (bandwidth, latency, reliability)
- Geographic diversity
- Historical uptime and Byzantine fault score
- Voluntary role preference (operators can suggest roles)

#### Routing Algorithm

```rust
// Pseudocode: Photonic-aware routing
fn route_shard(shard: &AuraShard, topology: &FractalTopology) -> Vec<NodeId> {
    let target_replicas = compute_replicas(topology.active_nodes());
    
    // Constrain by photonic band gap
    let usable_capacity = (1.0 - INVARIANTS.photonic_band_gap) * topology.total_bandwidth();
    
    // Fractal distance metric (not Euclidean)
    let destinations = topology.nearest_by_fractal_distance(shard.id(), target_replicas)
        .filter(|node| node.available_bandwidth() < usable_capacity);
    
    destinations.collect()
}
```

**Key Property**: Routing decisions are deterministic given the current topology snapshot. No statistical load balancing—the fractal geometry encodes optimal paths.

### FUSE Filesystem Interface

The `src/fuse/` module exposes AuraFS as a POSIX-compliant filesystem via FUSE (Filesystem in Userspace).

**Operations**:
- Standard POSIX: `open`, `read`, `write`, `close`, `mkdir`, `unlink`, etc.
- Extended attributes: `getxattr`, `setxattr` for shard metadata
- Lock operations: `flock`, `fcntl` with coherence-aware timeout enforcement

**Critical Constraint**: All lock-holding operations must complete within `lock_acquisition_timeout_us = 100 μs` to stay within coherence window limits.

**Implementation**:
```rust
// Simplified FUSE write operation
fn write(&mut self, req: &Request, ino: u64, fh: u64, 
         offset: i64, data: &[u8]) -> Result<u32, c_int> {
    
    let deadline = Instant::now() + Duration::from_micros(
        INVARIANTS.lock_acquisition_timeout_us
    );
    
    let guard = self.inode_table.try_lock_until(ino, deadline)
        .map_err(|_| libc::EAGAIN)?;
    
    // Perform write within coherence window
    guard.write_at(offset, data)?;
    
    Ok(data.len() as u32)
}
```

---

## Development Environment Setup

### Prerequisites

**Operating Systems**:
- **Development**: Windows 11 (PowerShell 7+) or Linux (any modern distro)
- **Production**: Linux (Debian 12, Fedora 39+), Docker, Kubernetes

**Rust Toolchain**:
```powershell
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Set Rust edition 2024 (requires Rust 1.82.0+)
rustup default stable
rustup update

# Verify version
rustc --version  # Should be ≥ 1.82.0
```

**Required Tools**:
```bash
# Formatter and linter
rustup component add rustfmt clippy

# Coverage (choose one)
cargo install cargo-llvm-cov
# OR
cargo install cargo-tarpaulin

# Benchmarking
cargo install cargo-criterion

# Security auditing
cargo install cargo-audit

# Documentation preview
cargo install cargo-watch
```

**Optional but Recommended**:
```bash
# Flamegraph profiling
cargo install flamegraph

# Dependency tree visualization
cargo install cargo-tree

# License checking
cargo install cargo-license
```

### Clone Repository

```bash
git clone https://github.com/aurphyx/aurafs.git
cd aurafs
```

### Install Dependencies

```bash
# Build all dependencies (may take 5–10 minutes first time)
cargo build --release

# Run test suite to verify setup
cargo test --all

# Generate documentation
cargo doc --no-deps --open
```

### IDE Setup (Cursor)

If using Cursor IDE, the `.cursorrules` file will automatically configure:
- Physics invariant enforcement
- TRL-4 file locking awareness
- Code generation patterns aligned with project standards

**Recommended Cursor Settings**:
- Enable Rust language server (rust-analyzer)
- Enable inline error checking
- Set tab width to 4 spaces (matches rustfmt)

### Environment Variables

```bash
# Optional: Customize AuraFS data directory
export AURAFS_DATA_DIR="/var/lib/aurafs"  # Linux
# OR
$env:AURAFS_DATA_DIR="C:\AuraFS\data"  # PowerShell

# Optional: Enable debug logging
export RUST_LOG=aurafs=debug,meshwerk=trace
```

### Verify Installation

```bash
# Check that all five physics constants load correctly
cargo run --bin verify_physics_constants

# Expected output:
# ✓ Loaded hilbert_scaling_bias: 5.3
# ✓ Loaded coherence_window_us: 1600
# ✓ Loaded spectral_dimension: 1.37
# ✓ Loaded photonic_band_gap: 0.21
# ✓ Loaded lock_acquisition_timeout_us: 100
# All physics constants loaded successfully.
```

---

## Project Structure

```
aurafs/
├── .cursorrules                     # Development standards (THIS IS CRITICAL)
├── aurafs.toml                      # Configuration (SINGLE SOURCE OF TRUTH for physics)
├── Cargo.toml                       # Rust package manifest
├── CHANGELOG.md                     # Version history
├── LICENSE-MIT, LICENSE-APACHE      # Dual licensing
├── README.md                        # Project overview
│
├── compliance/                      # Governance and validation documents
│   ├── ALGORITHM_PROOFS.md          # Mathematical theorems and proofs
│   ├── DARPA_TRL_VALIDATION.md      # Experimental protocols and budgets
│   ├── PHYSICS_INVARIANTS.json      # Machine-readable CI validation bounds
│   └── SECURITY_AUDIT.md            # PQC implementation audit and recommendations
│
├── docs/                            # User-facing documentation
│   ├── architecture.md              # System architecture deep-dive
│   ├── physics_primer.md            # Non-expert explanation of physics constants
│   └── key_management.md            # Cryptographic key rotation policy
│
├── src/                             # Rust source code
│   ├── main.rs                      # Entry point (TRL-4 LOCKED)
│   │
│   ├── physics/                     # Physics engine (TRL-4 LOCKED)
│   │   ├── mod.rs                   # Public API surface
│   │   ├── invariants.rs            # PhysicsInvariants struct, singleton loader
│   │   ├── coherence_tracker.rs     # T₂ monitoring
│   │   ├── spectral_monitor.rs      # d_s drift detection
│   │   └── violation_handler.rs     # Decoherence recovery protocol
│   │
│   ├── core/                        # Storage primitives (TRL-4 LOCKED)
│   │   ├── shard.rs                 # Void/Trap/Aura state machine
│   │   ├── merkle.rs                # SHA3-256 Merkle trees
│   │   └── soulproof.rs             # Cryptographic integrity proofs
│   │
│   ├── crypto/                      # Post-quantum cryptography
│   │   └── pqc/                     # (TRL-4 LOCKED)
│   │       ├── dilithium_sig.rs     # Dilithium-5 signatures
│   │       └── kyber_kem.rs         # Kyber-1024 KEM (in-progress)
│   │
│   ├── network/                     # Networking layers
│   │   ├── meshwerk/                # Fractal routing engine (TRL-4 LOCKED)
│   │   │   ├── mod.rs               # Public API
│   │   │   ├── roles.rs             # Coordinator/Relay/Edge/Observer
│   │   │   ├── routing.rs           # Photonic-aware routing algorithm
│   │   │   ├── topology_engine.rs   # Fractal distance calculations
│   │   │   └── configs/
│   │   │       └── ghostlink.toml   # GhostLink-LoRaWAN config (TRL-4 LOCKED)
│   │   │
│   │   ├── node_manager.rs          # Node lifecycle management (TRL-4 LOCKED)
│   │   ├── orchestrator.rs          # Topology orchestration (TRL-4 LOCKED)
│   │   ├── autoheal_daemon.rs       # Decoherence recovery (TRL-4 LOCKED)
│   │   │
│   │   ├── transport/               # Transport layer implementations
│   │   │   ├── starlink_client.rs   # Starlink-HighOrbit (TRL-4 LOCKED)
│   │   │   └── udp_multicast.rs     # UDP multicast (TRL-4 LOCKED)
│   │   │
│   │   ├── meshtastic_integration/  # Meshtastic firmware bridge
│   │   │   └── firmware_bridge.rs   # (TRL-4 LOCKED)
│   │   │
│   │   └── rns_bridge.rs            # Reticulum Network Stack bridge (TRL-4 LOCKED)
│   │
│   ├── fuse/                        # FUSE filesystem interface
│   │   ├── operations.rs            # POSIX operation handlers
│   │   └── lock_manager.rs          # Coherence-aware lock timeouts
│   │
│   ├── compression/                 # Fractal compression (TRL-4 LOCKED)
│   │   ├── lattice.rs               # Lattice-based compression
│   │   └── manager.rs               # Compression orchestration
│   │
│   ├── ai/                          # AI orchestration (TRL-4 LOCKED)
│   │   └── fractal_orchestrator.rs  # Pattern recognition and optimization
│   │
│   ├── audit/                       # Logging and monitoring
│   │   └── holographic_logger.rs    # Immutable append-only log (TRL-4 LOCKED)
│   │
│   └── gov/                         # Governance
│       └── sages.rs                 # Byzantine quorum consensus (TRL-4 LOCKED)
│
├── tests/                           # Integration tests
│   ├── physics/                     # Physics invariant validation
│   │   ├── test_invariants.rs       # Golden-file tests
│   │   └── test_decoherence.rs      # Recovery protocol tests
│   │
│   ├── crypto/                      # Cryptographic test vectors
│   │   ├── test_dilithium.rs        # Signature round-trip tests
│   │   └── test_merkle.rs           # Inclusion proof tests
│   │
│   └── integration/                 # End-to-end workflows
│       └── test_shard_lifecycle.rs  # Void → Trap → Aura full cycle
│
├── benches/                         # Performance benchmarks
│   └── replica_computation.rs       # compute_replicas() benchmark
│
├── simulations/                     # Experimental validation (not compiled)
│   └── qiskit_validation.ipynb      # Quantum simulation notebooks
│
├── docker/                          # Containerization
│   ├── Dockerfile.api               # Multi-stage build for API server
│   └── docker-compose.yml           # Local development stack
│
├── .github/                         # CI/CD configuration
│   ├── workflows/
│   │   ├── ci.yml                   # Continuous integration
│   │   ├── physics_audit.yml        # Physics constant enforcement
│   │   └── release.yml              # Release packaging
│   │
│   └── CODEOWNERS                   # Automatic review assignments
│
└── scripts/                         # Utility scripts
    ├── verify_physics_constants.sh  # Sanity check for constants
    └── generate_compliance_report.py # CI compliance report generator
```

### Key File Purposes

**`.cursorrules`**: The most important file for developers. Contains all development standards, physics enforcement rules, and prohibited patterns. Read this first.

**`aurafs.toml`**: Single source of truth for runtime configuration. The `[physics]` section defines the five immutable constants. Changes require governance vote.

**`compliance/PHYSICS_INVARIANTS.json`**: Machine-readable validation bounds for CI enforcement. Mirrors `aurafs.toml [physics]` values with tolerance ranges.

**`compliance/ALGORITHM_PROOFS.md`**: Mathematical foundations. Contains Theorem 2.1 (Hilbert scaling), Proposition 2.1 (spectral dimension), and Proposition 2.3 (QEC overhead).

**`src/physics/invariants.rs`**: Deserializes `aurafs.toml [physics]` into `physics::INVARIANTS` singleton. All modules access constants through this global.

**TRL-4 Locked Files**: Marked in `aurafs.toml [modules.validated]`. Public API changes require `PHYSICS OVERRIDE` governance approval.

---

## Key Workflows

### Adding a New Feature

1. **Check TRL Lock Status**:
   ```bash
   # Determine if your target file is TRL-4 locked
   grep -A 50 "\[modules.validated\]" aurafs.toml
   ```

2. **Create Feature Branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Implement Feature**:
   - Follow `.cursorrules` standards
   - Use `physics::INVARIANTS` for any physics constants
   - Write unit tests alongside implementation
   - Document all `pub` items with `///` comments

4. **Run Local Validation**:
   ```bash
   cargo fmt      # Format code
   cargo clippy   # Lint
   cargo test     # Run all tests
   cargo doc --no-deps  # Verify documentation
   ```

5. **Submit Pull Request**:
   - Include `CHANGELOG.md` entry under `[Unreleased]`
   - Link to relevant compliance documents if applicable
   - Request review from CODEOWNERS (automatically assigned)

6. **CI Validation**:
   - Physics audit checks for magic number violations
   - Full test suite runs on Ubuntu and Windows
   - Coverage thresholds enforced (see `aurafs.toml [ci.coverage]`)

### Modifying a Physics Constant

**WARNING**: This is a governance-gated operation requiring Byzantine quorum.

1. **Propose Change**:
   - Document scientific justification (new experimental data, peer-reviewed paper, etc.)
   - Submit proposal to Aurphyx LLC board
   - Await min_quorum=13 approval (Byzantine 3f+1 tolerance)

2. **Update Files**:
   ```toml
   # aurafs.toml [physics]
   hilbert_scaling_bias = 5.35  # Updated from 5.3
   ```

   ```json
   // compliance/PHYSICS_INVARIANTS.json
   {
     "hilbert_scaling_bias": 5.35,
     "tolerance": 0.05,
     "ci_test_directive": "PHYS-001: assert_in_range(5.30, 5.40)"
   }
   ```

3. **Commit with Override Tag**:
   ```bash
   git commit -m "physics(invariants): update hilbert_scaling_bias to 5.35

   PHYSICS OVERRIDE

   Board vote: 15/19 approval (min_quorum=13 met)
   Justification: New Qiskit validation shows 5.35 optimal at n=7 qubits
   Experimental data: VALIDATION_REPORT.md §1.2 (updated 2026-02-08)

   Refs: compliance/ALGORITHM_PROOFS.md Theorem 2.1"
   ```

4. **CI Verification**:
   - `.github/workflows/physics_audit.yml` validates override tag
   - Golden-file tests confirm constants match across files
   - Full regression test suite runs

### Handling a Physics Violation (Decoherence)

If spectral dimension drifts out of range during production:

1. **Automatic Detection**:
   ```rust
   // src/physics/spectral_monitor.rs
   if (measured_d_s - INVARIANTS.spectral_dimension).abs() > 0.05 {
       log_critical!("Spectral dimension violation: d_s={}", measured_d_s);
       state.set_coherence(CoherenceState::DecoherenceExempt);
       autoheal_daemon.trigger_redistribution().await?;
   }
   ```

2. **System Response**:
   - Freeze writes (existing reads continue)
   - Holographic logger records CRITICAL event with full topology snapshot
   - Autoheal daemon rebalances fractal lattice topology
   - Metrics dashboard alerts on-call engineer

3. **Human Investigation**:
   - Check `/var/log/aurafs/holographic.log` for full context
   - Analyze topology changes in last 5 minutes (new nodes joined/left?)
   - Verify no malicious Byzantine actors injecting false measurements

4. **Recovery**:
   - Autoheal completes redistribution (typically 30–60 seconds)
   - Spectral dimension returns to [1.32, 1.42] range
   - System resumes `CoherenceState::Strict` mode
   - Post-mortem written in `docs/incidents/YYYY-MM-DD-decoherence.md`

### Benchmarking Performance

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench replica_computation

# Generate flamegraph for hot path profiling
cargo flamegraph --bench replica_computation

# Compare before/after optimization
cargo bench --bench replica_computation -- --save-baseline before
# ... make optimization ...
cargo bench --bench replica_computation -- --baseline before
```

**Critical Benchmarks**:
- `compute_replicas()`: Target <10 ns per call (hot path)
- Merkle proof verification: Target <100 μs (within T₂ constraints)
- FUSE write operation: Target <80 μs (leaves margin for lock acquisition)

---

## Testing & Validation

### Test Pyramid

```
        ╱╲
       ╱  ╲         Unit Tests (fastest, most granular)
      ╱────╲        ~80% of tests
     ╱      ╲
    ╱────────╲      Integration Tests (moderate speed)
   ╱          ╲     ~15% of tests
  ╱────────────╲
 ╱              ╲   End-to-End Tests (slowest, most comprehensive)
╱────────────────╲  ~5% of tests
```

### Unit Tests

Located in `#[cfg(test)] mod tests` blocks within each source file.

**Example** (`src/physics/invariants.rs`):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hilbert_scaling_bias_in_range() {
        let eta = INVARIANTS.hilbert_scaling_bias;
        assert!((eta - 5.3).abs() < 0.05, "η out of tolerance: {}", eta);
    }

    #[test]
    fn test_compute_replicas_at_100_nodes() {
        let replicas = compute_replicas(100);
        assert_eq!(replicas, 3, "Expected 3 replicas at N=100");
    }
}
```

**Run**:
```bash
cargo test --lib
```

### Integration Tests

Located in `tests/` directory. Test interactions between modules.

**Example** (`tests/integration/test_shard_lifecycle.rs`):
```rust
#[tokio::test]
async fn test_void_to_aura_lifecycle() {
    // Create Void-Shard
    let mut shard = Shard::new_void(b"test data".to_vec());
    assert_eq!(shard.state(), ShardState::Void);
    
    // Fractal replicate → Trap-State
    let topology = Topology::new_test(100); // 100 nodes
    shard.fractal_replicate(&topology).await?;
    assert_eq!(shard.state(), ShardState::Trap);
    
    // Verify replica count
    let expected_replicas = compute_replicas(100);
    assert_eq!(shard.replica_count(), expected_replicas);
    
    // Promote to Aura-Shard
    shard.promote_to_aura(&signing_key).await?;
    assert_eq!(shard.state(), ShardState::Aura);
    assert!(shard.is_immutable());
}
```

**Run**:
```bash
cargo test --test test_shard_lifecycle
```

### Golden-File Tests

Validate runtime constants against `compliance/PHYSICS_INVARIANTS.json`.

**Example** (`tests/physics/test_invariants.rs`):
```rust
#[test]
fn test_invariant_golden_file() {
    let json: PhysicsInvariantsJson = serde_json::from_str(
        include_str!("../../compliance/PHYSICS_INVARIANTS.json")
    ).unwrap();
    
    // Check all five constants
    assert_eq!(INVARIANTS.hilbert_scaling_bias, json.hilbert_scaling_bias);
    assert_eq!(INVARIANTS.coherence_window_us, json.coherence_window_us);
    assert_eq!(INVARIANTS.spectral_dimension, json.spectral_dimension);
    assert_eq!(INVARIANTS.photonic_band_gap, json.photonic_band_gap);
    assert_eq!(INVARIANTS.lock_acquisition_timeout_us, json.lock_acquisition_timeout_us);
}
```

**Run**:
```bash
cargo test --test test_invariants
```

### Coverage Thresholds

From `aurafs.toml [ci.coverage]`:

| Module | Minimum | Why |
|--------|---------|-----|
| `src/physics/` | 90% | Core physics engine must be bulletproof |
| `src/core/` | 80% | Storage primitives are high-risk |
| `src/crypto/pqc/` | 85% | Cryptographic correctness is critical |
| `src/fuse/` | 70% | FUSE layer has many edge cases |
| `src/network/` | 60% | Network code has inherent flakiness |
| Overall | 70% | Project-wide minimum |

**Measure Coverage**:
```bash
# Using cargo-llvm-cov
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
cargo llvm-cov report --html

# Using cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage/
```

### CI Test Matrix

`.github/workflows/ci.yml` runs on:

| OS | Rust | Target |
|----|------|--------|
| Ubuntu 22.04 | stable | x86_64-unknown-linux-gnu |
| Windows 11 | stable | x86_64-pc-windows-msvc |

**Test stages**:
1. Format check (`cargo fmt --check`)
2. Lint (`cargo clippy -- -D warnings`)
3. Build (`cargo build --release`)
4. Test (`cargo test --all`)
5. Coverage enforcement
6. Physics audit (magic number scan)

---

## Security & Compliance

### Post-Quantum Cryptography Status

| Primitive | Algorithm | NIST Standard | Status |
|-----------|-----------|---------------|--------|
| Signatures | Dilithium-5 | FIPS 204 (ML-DSA-87) | ✅ Implemented |
| Key Encapsulation | Kyber-1024 | FIPS 203 (ML-KEM-1024) | 🚧 In Progress (R1) |
| Hashing | SHA3-256 | FIPS 202 | ✅ Implemented |
| Symmetric | AES-256-GCM | FIPS 197 | ✅ Implemented |

### Security Audit Recommendations

From `compliance/SECURITY_AUDIT.md`:

**CRITICAL (must be resolved before v1.0):**
- **R1**: Complete Kyber-1024 integration (remove X25519 hybrid placeholder)
- **R2**: Verify Dilithium constant-time implementation
- **R3**: Document key management policy (rotation, revocation, escrow)

**IMPORTANT (should be resolved before production):**
- **R4**: HSM support for key storage (FIPS 140-3 Level 3 modules)
- **R5**: Third-party cryptographic audit (Trail of Bits / NCC Group)
- **R6**: Hybrid signatures (Dilithium-5 + Ed25519 dual-signing)

### Threat Model

**In Scope**:
- Classical adversary with unlimited compute
- Quantum adversary with fault-tolerant quantum computer (Shor's algorithm)
- Byzantine adversary controlling up to f=(n-1)/3 nodes
- Side-channel attacks (timing, power analysis, EM radiation)

**Out of Scope**:
- Physical access to servers (assumes secure datacenter)
- Social engineering (assumes proper key hygiene)
- Supply chain attacks on hardware (assumes trusted boot)
- Compromised compiler toolchain

### Cryptographic Key Lifecycle

1. **Generation**: Dilithium-5 keypair generated using `OsRng` (cryptographically secure)
2. **Storage**: Private keys encrypted at rest with AES-256-GCM, passphrase-derived key
3. **Rotation**: Annual rotation for signing keys, emergency rotation on compromise
4. **Revocation**: Governance quorum vote (min_quorum=13) required for key revocation
5. **Escrow**: M-of-N threshold scheme (e.g., 7 of 13 board members) for master key recovery

See `docs/key_management.md` for full policy.

### Compliance Artifacts

| Document | Purpose | Owner |
|----------|---------|-------|
| `PHYSICS_INVARIANTS.json` | CI validation bounds | Ross Edwards |
| `ALGORITHM_PROOFS.md` | Mathematical foundations | Ross Edwards |
| `SECURITY_AUDIT.md` | PQC implementation status | Security team |
| `DARPA_TRL_VALIDATION.md` | Experimental protocols | Ross Edwards |
| `VALIDATION_REPORT.md` | Lab validation results | Ross Edwards |

These documents are version-controlled and peer-reviewed. Changes require sign-off from CODEOWNERS.

---

## Deployment Strategy

### Development (Local)

```bash
# Build debug binary
cargo build

# Run with test configuration
RUST_LOG=debug ./target/debug/aurafs \
    --config aurafs.toml \
    --mount-point /tmp/aurafs_test

# In another terminal, test FUSE mount
echo "Hello AuraFS" > /tmp/aurafs_test/test.txt
cat /tmp/aurafs_test/test.txt
```

### Staging (Docker)

```bash
# Build multi-stage Docker image
docker build -f docker/Dockerfile.api -t aurafs:latest .

# Run with persistent volume
docker run -d \
    --name aurafs-staging \
    -v /data/aurafs:/var/lib/aurafs \
    -p 8080:8080 \
    aurafs:latest
```

### Production (Kubernetes)

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: aurafs-cluster
spec:
  replicas: 13  # Byzantine 3f+1 minimum for quorum
  selector:
    matchLabels:
      app: aurafs
  template:
    spec:
      containers:
      - name: aurafs
        image: ghcr.io/aurphyx/aurafs:0.4.0
        volumeMounts:
        - name: data
          mountPath: /var/lib/aurafs
        env:
        - name: RUST_LOG
          value: "info"
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 100Gi
```

**Deploy**:
```bash
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
```

### Production Checklist

Before deploying v0.4.0 to production:

- [ ] All CRITICAL security audit items resolved (R1, R2, R3)
- [ ] Coverage thresholds met (≥70% overall, ≥90% physics)
- [ ] Golden-file tests passing (physics constants validated)
- [ ] Load testing complete (1,000 concurrent clients, 10,000 shards)
- [ ] Decoherence recovery tested (inject out-of-range d_s, verify recovery)
- [ ] Key rotation policy documented and tested
- [ ] Disaster recovery runbook created
- [ ] On-call rotation established
- [ ] Metrics dashboard deployed (Grafana + Prometheus)
- [ ] Log aggregation configured (Loki or Elasticsearch)

---

## Contributing Guidelines

### Who Can Contribute

AuraFS is open-source (MIT & Apache-2.0 dual-licensed). Contributions welcome from:
- Academic researchers (physics, cryptography, distributed systems)
- Open-source developers
- Industry practitioners
- Quantum computing enthusiasts

### Contribution Types

**Code Contributions**:
- Bug fixes
- Performance optimizations
- New features (e.g., additional transport layers)
- Test coverage improvements
- Documentation enhancements

**Non-Code Contributions**:
- Peer review of mathematical proofs (`compliance/ALGORITHM_PROOFS.md`)
- Security audit recommendations
- Experimental validation (e.g., Qiskit simulations)
- Translation of documentation to other languages
- UX/UI improvements for monitoring dashboards

### Contribution Workflow

1. **Read Governance Documents**:
   - `.cursorrules` (development standards)
   - `aurafs.toml` (configuration and physics constants)
   - `CONTRIBUTING.md` (if exists)

2. **Open Issue or Discussion**:
   - For bugs: Provide minimal reproducible example
   - For features: Discuss design approach first
   - For physics changes: Requires governance proposal

3. **Fork Repository**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/aurafs.git
   cd aurafs
   git remote add upstream https://github.com/aurphyx/aurafs.git
   ```

4. **Create Branch**:
   ```bash
   git checkout -b feature/your-contribution
   ```

5. **Implement Changes**:
   - Follow `.cursorrules` standards
   - Write tests for new code
   - Update `CHANGELOG.md`

6. **Submit Pull Request**:
   - Clearly describe what changed and why
   - Link to related issues
   - Request review from relevant CODEOWNERS

7. **Address Review Feedback**:
   - Respond to comments promptly
   - Make requested changes
   - Re-request review after updates

8. **Merge**:
   - CI must be green (all tests passing, no clippy warnings)
   - At least 1 approval from CODEOWNERS
   - `PHYSICS OVERRIDE` tag if modifying constants

### Code of Conduct

AuraFS follows the [Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

**Summary**:
- Be respectful and inclusive
- Focus on what's best for the project
- Accept constructive criticism gracefully
- Show empathy towards other contributors

**Enforcement**: Violations reported to ross@aurphyx.org will be reviewed by project maintainers.

---

## Resources & References

### Project Resources

- **Repository**: https://github.com/aurphyx/aurafs
- **Homepage**: https://aurafs.dev
- **Documentation**: https://docs.aurafs.dev
- **Issue Tracker**: https://github.com/aurphyx/aurafs/issues
- **Discussions**: https://github.com/aurphyx/aurafs/discussions

### Technical Papers

- **AuraFS Thesis**: (pending arXiv submission, early 2026)
- **Anderson Localization**: P.W. Anderson, "Absence of Diffusion in Certain Random Lattices", Physical Review 109 (1958)
- **Spectral Dimension**: Rammal & Toulouse, "Random walks on fractal structures and percolation clusters", Journal de Physique Lettres 44 (1983)
- **Photonic Band Gaps**: Joannopoulos et al., "Photonic Crystals: Molding the Flow of Light" (2008)

### NIST PQC Standards

- **FIPS 203** (ML-KEM / Kyber): https://csrc.nist.gov/pubs/fips/203/final
- **FIPS 204** (ML-DSA / Dilithium): https://csrc.nist.gov/pubs/fips/204/final
- **FIPS 202** (SHA-3): https://csrc.nist.gov/pubs/fips/202/final

### Rust Ecosystem

- **Rust Edition 2024 Guide**: https://doc.rust-lang.org/edition-guide/
- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Async Programming in Rust**: https://rust-lang.github.io/async-book/

### Distributed Systems

- **libp2p Documentation**: https://docs.libp2p.io/
- **Reticulum Network Stack**: https://reticulum.network/
- **Meshtastic**: https://meshtastic.org/
- **IPFS Whitepaper**: https://ipfs.io/ipfs/QmR7GSQM93Cx5eAg6a6yRzNde1FQv7uL6X1o4k7zrJa3LX

### Academic Affiliations

- **ORCID**: 0009-0008-0539-1289 (Ross A. Edwards)
- **Organization**: Aurphyx LLC (Erie, PA, USA)
- **Incorporation Date**: January 2026

### Contact

- **Email**: ross@aurphyx.org
- **GitHub**: @rossfive
- **Project Discussions**: https://github.com/aurphyx/aurafs/discussions

---

## Appendix: Glossary

**Anderson Localization**: Phenomenon where disorder in a material causes wave interference, localizing eigenstates and suppressing diffusion. Critical mechanism for AuraFS coherence extension.

**Aura-Shard**: Immutable, topologically protected shard state. Replicated across `compute_replicas(N)` nodes with Dilithium-5 signature verification required for integrity.

**Byzantine Fault Tolerance**: Consensus mechanism resilient to malicious nodes. AuraFS uses 3f+1 quorum (min_quorum=13) to tolerate up to f=4 Byzantine adversaries.

**Coherence Window (T₂)**: Maximum duration for synchronous operations before decoherence degrades data integrity. AuraFS achieves 1,600 μs vs 100 μs baseline.

**Fractal Density (D_f)**: Hausdorff dimension of Sierpiński gasket. Exact value: log(3)/log(2) ≈ 1.585. Governs scaling behavior of fractal lattice.

**Hilbert Scaling Bias (η)**: Dimensionality scaling factor controlling replica growth rate. Formula: `Replicas = ceil(log_η(Nodes))`.

**Photonic Band Gap (PBG)**: Forbidden frequency band in photonic crystal where electromagnetic waves cannot propagate. Used as routing overhead constraint.

**Spectral Dimension (d_s)**: Effective dimensionality for random walks on fractal. AuraFS uses d_s ≈ 1.37 (Sierpiński gasket). Critical: d_s < 2 ensures localization.

**Technology Readiness Level (TRL)**: NASA/DARPA scale from 1 (basic principles) to 9 (flight-proven). AuraFS is at TRL-4 (lab-validated).

**Trap-State**: Coherent, localized shard state held via Anderson mechanism. NOT a "cache"—it's a physics-defined quantum-inspired state.

**Void-Shard**: Raw, mutable data before entering fractal lattice. Temporary state during active write operations.

---

**Document Version**: 1.0  
**Last Updated**: February 8, 2026  
**Next Review**: Quarterly or upon major milestone (v0.5.0, v1.0.0, TRL advancement)

For questions or clarifications, open a discussion at https://github.com/aurphyx/aurafs/discussions or email ross@aurafs.dev.

---

**END OF AURAFS_PROJECT_CONTEXT.MD**
