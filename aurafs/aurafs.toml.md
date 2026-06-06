# ═══════════════════════════════════════════════════════════════════════════════
# AURAFS PROJECT CONFIGURATION
# ═══════════════════════════════════════════════════════════════════════════════
#
# TRL-4 Lab-Validated State (Feb 2026)
# Reference: Phase II Passive Coherence Validation
#
# Authority:     This file is the SINGLE SOURCE OF TRUTH for all runtime constants.
# Compliance:    compliance/PHYSICS_INVARIANTS.json mirrors these values for CI.
# Proofs:        compliance/ALGORITHM_PROOFS.md contains mathematical derivations.
# Security:      compliance/SECURITY_AUDIT.md defines cryptographic posture.
# Validation:    VALIDATION_REPORT.md (02-07-2026) confirms experimental agreement.
# DARPA:         compliance/DARPA_TRL_VALIDATION.md maps TRL-4 component readiness.
# Cursor Prompt: AURPHYX-CURSOR-DEPLOY-002 v2.0 ingests this file in Phase 0.
#
# ┌──────────────────────────────────────────────────────────────────────────┐
# │ WARNING: Modifying [physics] values requires a PHYSICS OVERRIDE         │
# │ governance vote (min_quorum = 13) per .cursorrules §1 and §5.           │
# │ Any PR changing these values without the override commit tag            │
# │ will be rejected by .github/workflows/physics_audit.yml.               │
# └──────────────────────────────────────────────────────────────────────────┘
#
# ═══════════════════════════════════════════════════════════════════════════════


# ─────────────────────────────────────────────────────────────────────────────
# PROJECT METADATA
# ─────────────────────────────────────────────────────────────────────────────

[project]
name = "aurafs"
version = "0.4.0-phase2-hardening"
authors = ["Ross Edwards <Aurphyx LLC>"]
organization = "Aurphyx LLC"
orcid = "0009-0008-0539-1289"
edition = "2024"
license = "MIT OR Apache-2.0"
repository = "https://github.com/aurphyx/aurafs"
homepage = "https://aurafs.dev"
description = "Physics-informed distributed storage powered by fractal geometry and topological protection."


# ─────────────────────────────────────────────────────────────────────────────
# PHYSICS INVARIANTS (NON-NEGOTIABLE)
# ─────────────────────────────────────────────────────────────────────────────
#
# These five constants are the engine of the AuraFS architecture. They are
# deserialized at startup by src/physics/invariants.rs into the global
# physics::INVARIANTS singleton (OnceLock<PhysicsInvariants>). Every module
# that needs a physics value MUST read from this singleton — never a bare
# literal. Violations are caught by:
#
#   1. .github/workflows/physics_audit.yml  (CI regex enforcement)
#   2. Cursor Phase 1 audit                 (AURPHYX-CURSOR-DEPLOY-002)
#   3. tests/physics/ golden-file tests     (runtime validation)
#
# Cross-reference chain:
#   aurafs.toml [physics]  →  deserialized by  →  src/physics/invariants.rs
#   aurafs.toml [physics]  →  mirrored in      →  compliance/PHYSICS_INVARIANTS.json
#   aurafs.toml [physics]  →  proven in         →  compliance/ALGORITHM_PROOFS.md
#   aurafs.toml [physics]  →  validated in      →  VALIDATION_REPORT.md
#
# ─────────────────────────────────────────────────────────────────────────────

[physics]

# η — Fractal Hilbert Scaling Bias
# Using 5.3x bias per AuraFS Thesis Sec 4.2
# Formula: Replicas = ceil(log_η(Nodes))
# Theorem 2.1: dim(H_acc) = d^(n·D_f^α(k)) where α(k) = log(1+k·η)/log(D_f)
# Validated: 5.3× advantage at N=100, D=5 (VALIDATION_REPORT.md §1)
# Qiskit:   7.1× at n=5 qubits (consistent with 2^{5×0.585} ≈ 7)
# Tolerance: ±0.05 → valid range [5.25, 5.35]
hilbert_scaling_bias = 5.3

# T₂ — Coherence Window (microseconds)
# Max synchronous operation time. 16× improvement via Anderson Localization.
# T₂(fractal) = 1600 μs vs T₂(transmon) = 100 μs → 16× passive gain
# IPR confirmation: mean participation ratio PR ≈ 21.2 at k=4, N=123
# Validated: VALIDATION_REPORT.md §2
# Tolerance: ±100 μs → valid range [1500, 1700]
coherence_window_us = 1600

# d_s — Spectral Dimension of Sierpiński Gasket
# Exact: 2·log(3)/log(5) ≈ 1.365 (Rammal & Toulouse, 1983)
# Implementation clamp: 1.37 (within numerical precision of lattice simulations)
# Prop 2.1: ρ(E) ∝ E^(d_s/2 − 1) → divergent DOS as E→0 → trap states
# Critical: d_s < 2 → no mobility edge → all states localized
# If |measured_d_s − 1.37| > 0.05: trigger decoherence_recovery
# Validated: VALIDATION_REPORT.md §3
# Tolerance: ±0.05 → valid range [1.32, 1.42]
spectral_dimension = 1.37

# PBG — Photonic Band Gap (fractional bandwidth Δω/ω)
# Complete TM gap in C₆ᵥ hexagonal lattice (ε = 12, r/a = 0.2)
# Usable routing capacity = (1 − PBG) × raw bandwidth = 79%
# Validated: 21% TM gap via PWE simulation (VALIDATION_REPORT.md §4)
# Tolerance: ±0.03 → valid range [0.18, 0.24]
photonic_band_gap = 0.21

# Lock Acquisition Timeout (microseconds)
# Derived: coherence_window_us / 16 = 100 μs
# FUSE lock limit for atomic trap-state transitions.
# Physics basis: 1/16 of T₂ ensures lock ops complete within coherence window.
lock_acquisition_timeout_us = 100


# ─────────────────────────────────────────────────────────────────────────────
# DERIVED CONSTANTS (Reference Only — Computed at Runtime, Not Deserialized)
# ─────────────────────────────────────────────────────────────────────────────
#
# fractal_density_Df        = 1.585           # log(3)/log(2), Hausdorff dim | ±0.005
# fidelity_improvement      = 16              # T₂(fractal)/T₂(transmon) = 1600/100
# decoherence_suppression   = 0.0625          # γ_eff/γ₀ = 1/16 (Anderson localization)
# usable_routing_capacity   = 0.79            # 1.0 − photonic_band_gap
# fractal_qec_overhead      = 89              # Prop 2.3 at p_phys=10⁻³, p_L=10⁻¹²
# euclidean_qec_overhead    = 1458            # Surface code baseline, same error rates
# qec_reduction_factor      = 16.4            # 1458/89
# spectral_walk_dimension   = 2.32            # d_w = log(5)/log(2), anomalous diffusion


# ─────────────────────────────────────────────────────────────────────────────
# GOVERNANCE
# ─────────────────────────────────────────────────────────────────────────────

[governance]
sentinel_loop = "active"

# Byzantine fault tolerance: 3f + 1, f = 4 → min_quorum = 13
# Required for: physics constant changes, TRL-4 lock overrides, key rotation.
min_quorum = 13

# Quantum-resistant baseline (FIPS 204 / ML-DSA-87, NIST Level 5)
# Public key: 2,592 bytes | Signature: 4,627 bytes
# Ref: compliance/SECURITY_AUDIT.md §2.1
signature_scheme = "Dilithium-5"

# Override tags recognized in git commit messages.
physics_override_tag = "PHYSICS OVERRIDE"
security_override_tag = "SECURITY OVERRIDE"


# ─────────────────────────────────────────────────────────────────────────────
# NETWORK — Triple-Topology Configuration
# ─────────────────────────────────────────────────────────────────────────────
#
# Horizontal & Recursive Architecture:
#   1. Titan-Libp2p:       Primary high-bandwidth P2P (datacenter/cloud)
#   2. GhostLink-LoRaWAN:  Secondary low-power mesh (edge/IoT/off-grid)
#   3. Starlink-HighOrbit: Tertiary orbital backhaul (global coverage)
#
# ─────────────────────────────────────────────────────────────────────────────

[network]
primary_transport = "Titan-Libp2p"
secondary_transport = "GhostLink-LoRaWAN"
orbital_backhaul = "Starlink-HighOrbit"

[network.meshwerk]
max_peers = 256
gossip_interval_ms = 500
heartbeat_timeout_ms = 5000
# MUST equal [physics].photonic_band_gap — enforced by physics_audit.yml
routing_overhead_factor = 0.21


# ─────────────────────────────────────────────────────────────────────────────
# CRYPTO — Post-Quantum Cryptographic Primitives
# ─────────────────────────────────────────────────────────────────────────────
# Full audit: compliance/SECURITY_AUDIT.md
# Open items: R1 (Kyber-1024), R2 (constant-time Dilithium),
#             R3 (key mgmt), R4 (HSM), R5 (third-party audit), R6 (hybrid sigs)
# ─────────────────────────────────────────────────────────────────────────────

[crypto]
signature_algorithm = "Dilithium-5"         # FIPS 204 (ML-DSA-87)
signature_security_level = 5
kem_algorithm = "Kyber-1024"                # FIPS 203 (ML-KEM-1024)
kem_security_level = 5
kem_status = "in-progress"                  # X25519 hybrid placeholder pending R1
hash_algorithm = "SHA3-256"                 # FIPS 202, quantum-resistant
symmetric_algorithm = "AES-256-GCM"         # FIPS 197


# ─────────────────────────────────────────────────────────────────────────────
# STORAGE — Shard Configuration
# ─────────────────────────────────────────────────────────────────────────────
# Lifecycle: Void-Shard → Trap-State → Aura-Shard (immutable)
#   Void-Shard:  Raw, mutable data before lattice entry
#   Trap-State:  Localized data packet held in coherence (NOT "cache")
#   Aura-Shard:  Immutable, topologically protected, fractalized data
# ─────────────────────────────────────────────────────────────────────────────

[storage]
max_shard_bytes = 104857600                 # 100 MB (100 * 1024 * 1024)
max_children_per_shard = 1000000            # 1M — prevents unbounded fractal growth
max_replicas = 1000                         # Safety ceiling for replica count
default_initial_state = "Void-Shard"


# ─────────────────────────────────────────────────────────────────────────────
# MODULES — TRL-4 LOCK REGISTRY
# ─────────────────────────────────────────────────────────────────────────────
#
# TRL-4 LOCKED FILES (DO NOT MODIFY PUBLIC API WITHOUT 'PHYSICS OVERRIDE')
# Cursor: Refer to these files as Source of Truth for logic hardening.
#
# Internal refactoring (magic number → physics::INVARIANTS) is PERMITTED.
# Public API changes require governance vote (min_quorum = 13).
#
# Consumed by:
#   - Cursor Phase 1 audit    (AURPHYX-CURSOR-DEPLOY-002 §1.4)
#   - physics_audit.yml CI    (.github/workflows/)
#   - CODEOWNERS              (routes PRs to @rossedwards)
#
# ─────────────────────────────────────────────────────────────────────────────

[modules.validated]

core = [
    "src/physics/mod.rs",
    "src/physics/invariants.rs",
    "src/core/shard.rs",
    "src/core/merkle.rs",
    "src/main.rs",
]

network_meshwerk = [
    "src/network/meshwerk/mod.rs",
    "src/network/meshwerk/roles.rs",
    "src/network/meshwerk/routing.rs",
    "src/network/meshwerk/topology_engine.rs",
]

network_control = [
    "src/network/node_manager.rs",
    "src/network/orchestrator.rs",
    "src/network/autoheal_daemon.rs",
]

transports = [
    "src/network/transport/starlink_client.rs",
    "src/network/meshtastic_integration/firmware_bridge.rs",
    "src/network/rns_bridge.rs",
    "src/network/transport/udp_multicast.rs",
]

crypto_audit = [
    "src/crypto/pqc/dilithium_sig.rs",
    "src/audit/holographic_logger.rs",
    "src/gov/sages.rs",
]

ai_compression = [
    "src/ai/fractal_orchestrator.rs",
    "src/compression/lattice.rs",
    "src/compression/manager.rs",
]

configs = [
    "src/network/meshwerk/configs/ghostlink.toml",
]


# ─────────────────────────────────────────────────────────────────────────────
# CI/CD — Test & Compliance Enforcement
# ─────────────────────────────────────────────────────────────────────────────
#
# Consumed by .github/workflows/physics_audit.yml and ci.yml.
# Coverage enforced by cargo-tarpaulin or cargo-llvm-cov.
# Physics audit patterns mirrored in PHYSICS_INVARIANTS.json ci_test_directives.
#
# ─────────────────────────────────────────────────────────────────────────────

[ci]
rust_edition = "2024"
msrv = "1.82.0"
target_os = ["debian-latest", "windows-latest"]

[ci.coverage]
physics = 90            # src/physics/ — highest bar
core = 80               # src/core/ — shard, merkle, soulproof
crypto_pqc = 85         # src/crypto/pqc/ — post-quantum primitives
fuse = 70               # src/fuse/ — filesystem layer
network = 60            # src/network/ — transport + meshwerk
overall = 70            # all modules combined

[ci.physics_audit]
enabled = true
fail_on_violation = true
exclude_paths = [
    "target/",
    "simulations/",
    "docs/",
    "compliance/",
    "*.md",
]


# ─────────────────────────────────────────────────────────────────────────────
# EXPERIMENTAL PROTOCOLS (Reference Only — Not Loaded at Runtime)
# ─────────────────────────────────────────────────────────────────────────────
# Maps to compliance/DARPA_TRL_VALIDATION.md. Total: $1.254M / 18 months.
#
# [experimental]
# P1_nv_diamond     = { budget = 144000, validates = "coherence_window_us" }
# P2_trapped_ion    = { budget = 494000, validates = "hilbert_scaling_bias" }
# P3_photonic_pbg   = { budget = 60000,  validates = "photonic_band_gap" }
# P4_majorana_tjunc = { budget = 456000, validates = "fractal_qec_overhead" }
