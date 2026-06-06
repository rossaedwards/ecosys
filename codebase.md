# AuraFS Codebase Map

> **Generated:** 2026-04-27 | **Repo:** `rossaedwards/main` | **Architect:** Ross A Edwards @ Aurphyx LLC

---

## Overview

This repository is a monorepo containing **AuraFS** — a physics-informed, quantum-safe, soul-identity-driven distributed filesystem written in Rust — alongside several sister projects, research notebooks, simulation models, and compliance frameworks. The codebase centers around `aurafs/` which is the core Rust workspace.

---

## Root-Level Structure

| Path | Type | Purpose |
|---|---|---|
| `COMMUNITY.md` | Doc | Community guidelines and governance |
| `EXPERIMENTS.md` | Doc | Experimental protocols and physics experiments |
| `PHYSICS.md` | Doc | Physics theory grounding AuraFS design |
| `README.md` | Doc | Top-level project introduction |
| `SCIENTIFIC_TERMINOLOGY.md` | Doc | Canonical scientific vocabulary |
| `nc_aurafs.rs` | Rust | Standalone network-coherence AuraFS helper |
| `rae_04-26-2026_filelist.txt` | Index | Full repo file listing snapshot |
| `aurafs/` | Dir | Primary AuraFS Rust workspace |
| `aps/` | Dir | Aurphyx Physics Simulation |
| `docs/` | Dir | Project-wide documentation |
| `ftqc/` | Dir | Fault-Tolerant Quantum Computing module |
| `fuxyez/` | Dir | Fuxyez subsystem |
| `memoree/` | Dir | Memory / persistent identity subsystem |
| `openscad/` | Dir | Hardware / physical design models (OpenSCAD) |
| `sages/` | Dir | SAGES framework |
| `standards/` | Dir | Compliance and standards documents |
| `tslca/` | Dir | TSLCA protocol |
| `tvfd/` | Dir | Topological Vacuum Flux Dynamics |
| `vim/` | Dir | Vim configuration |

---

## `aurafs/` — Primary Rust Workspace

### Root Files

| File | Purpose |
|---|---|
| `Cargo.toml` | Workspace manifest with all crate definitions |
| `Cargo.lock` | Deterministic dependency lock |
| `aurafs.toml` | AuraFS runtime configuration |
| `.cursorrules` / `cursorrules` | AI/IDE development rules |
| `.gitignore` | Standard Rust ignores |
| `README.md` | AuraFS project overview |
| `LICENSE` | Project license |
| `CHANGELOG.md` | Version history |
| `CONTRIBUTING.md` | Contribution guide |
| `SECURITY.md` | Security policy |
| `VALIDATION_REPORT.md` | Validation and testing report |
| `PHASE1_AUDIT_SUMMARY.md` | Phase 1 audit results |
| `AURAFS_AUDIT_AND_DEPLOY.md` | Deployment and audit procedures |
| `AURAFS_PROJECT_CONTEXT.md` | Full project context (50KB) |
| `AURAFS_TRL4_PRODUCTION_CONTEXT.md` | TRL-4 production context |
| `AURAFS_TRL4_PRODUCTION_CONTEXT_PART2.md` | TRL-4 context part 2 |
| `AURAFS_TRL4_PRODUCTION_CONTEXT_PART3.md` | TRL-4 context part 3 |
| `FIX_UPDATE_BUILD_*.md` | Per-module build fix/update logs |
| `copy_to_md.rs` | Utility: copy sources to markdown |
| `aurphyx_main_aurafs_03-06-2026-repolist.txt` | Repo listing snapshot |

### AuraFS Subdirectories

| Directory | Purpose |
|---|---|
| `src/` | All Rust source code (primary codebase) |
| `benches/` | Criterion benchmarks |
| `compliance/` | Compliance documentation |
| `deploy/` | Deployment scripts and configuration |
| `docker/` | Docker container definitions |
| `docs/` | AuraFS-specific documentation |
| `examples/` | Usage examples |
| `helm/` | Kubernetes Helm charts |
| `integrations/` | Third-party integrations |
| `k8s/` | Kubernetes manifests |
| `plugins/` | Plugin system |
| `scripts/` | Build and maintenance scripts |
| `sdk/` | AuraFS SDK |
| `simulations/` | Physics and system simulations |
| `snapshots/` | State snapshot utilities |
| `tests/` | Integration and system tests |
| `tts/` | Text-to-speech / audio subsystem |
| `ui/` | User interface |
| `versioning/` | Version management |

---

## `aurafs/src/` — Source Modules

| Module | Purpose |
|---|---|
| `core/` | 🔮 **Foundation** — Error handling, identity, crypto, shards, resilience |
| `acl/` | Access Control Lists — soul-based permissions |
| `ai/` | AI integration layer |
| `api/` | HTTP/gRPC API endpoints |
| `audit/` | Audit trail and event logging |
| `bin/` | Binary entrypoints |
| `cache/` | Caching layer |
| `cli/` | Command-line interface |
| `compression/` | Data compression |
| `config/` | Configuration management (module form) |
| `crypto/` | Extended cryptography module |
| `dedup/` | Deduplication engine |
| `enterprise/` | Enterprise-grade features |
| `error/` | Extended error module |
| `fuse/` | FUSE filesystem interface |
| `gov/` | Governance and voting |
| `heal/` | Self-healing subsystem |
| `mesh/` | Mesh networking |
| `model_slice/` | Model slicing for AI/ML data |
| `monitoring/` | Observability and monitoring |
| `namespace/` | Namespace management |
| `network/` | Network orchestration |
| `ops/` | Filesystem operations |
| `physics/` | Physics engine (coherence, T2 windows) |
| `quantum/` | Quantum computing integration |
| `redteam/` | Red-team / adversarial testing |
| `resilience/` | Resilience patterns (retry, backoff) |
| `shard/` | Extended shard module |
| `shard_server/` | Shard server daemon |
| `snapshot/` | Snapshot management |
| `storage/` | Storage backend abstraction |
| `whitehat/` | Whitehat security testing |
| `lib.rs` | Crate root — module orchestration |
| `main.rs` | Binary entrypoint |
| `config.rs` | Top-level config |
| `error.rs` | Top-level error re-export |
| `autoheal_daemon.rs` | Auto-heal background daemon |

---

## `aurafs/src/core/` — Deep Dive

> **This is the architectural heart of AuraFS.** All high-level modules depend on types and traits defined here.

### Files

| File | Lines (approx) | Role |
|---|---|---|
| `mod.rs` | ~200 | Module root, re-exports, `CoreSystem` builder, `AuraFSComponent` trait |
| `error.rs` | ~400 | Comprehensive error hierarchy: `AuraFSError`, `CoreError`, `ErrorContext` |
| `bliss.rs` | ~500 | `BlissId` quantum-safe identity, `BlissIdManager` trait, in-memory + persistent impls |
| `shard.rs` | ~500 | `ShardId`, `ShardHandle`, `ShardMetadata`, `FractalShard`, coherence state machine |
| `merkle.rs` | ~550 | Full Merkle tree: SHA3-256, proof generation/verification, async-pending leaf marking |
| `crypto.rs` | ~350 | `DilithiumKeypair` (post-quantum), SHA3-256, SHAKE256, entropy utilities |
| `soulproof.rs` | ~250 | Zero-knowledge soul proofs: `SoulProof`, `ProofStatus` |
| `health.rs` | ~420 | K8s-compatible health probes: `HealthManager`, `HealthReport`, `ComponentHealth` |
| `config.rs` | ~550 | Hot-reloadable `CoreConfig`, `ConfigManager`, `CryptoConfig`, `ShardConfig` |
| `circuit_breaker.rs` | ~500 | Three-state circuit breaker (Closed/Open/HalfOpen), `CircuitBreakerRegistry` |
| `rate_limiter.rs` | ~450 | Sliding window + token bucket rate limiters, `SoulRateLimiter` |
| `metrics.rs` | ~300 | Prometheus-style `AuraFSMetrics` counters and histograms |
| `tracing.rs` | ~450 | OpenTelemetry-compatible distributed tracing: `Tracer`, `Span`, `TraceContext` |
| `network.rs` | ~550 | `NetworkOrchestrator`, `PeerNode`, `NodeRole`, `ReplicationStrategy`, `NetworkEvent` |
| `persistence.rs` | ~600 | SQLite-backed `SqliteBlissIdManager`, `DatabaseConfig` |
| `ENTERPRISE_IMPROVEMENTS.md` | — | In-source improvement backlog and upgrade notes |

### Core Data Flow

```
Biometric Input
    └─> BlissId::new() [SHA3-256]
            └─> SoulProof generation [ZK]
                    └─> BlissIdManager::register_blissid()
                            └─> PersistentBlissIdManager [Dilithium5 signed audit]
                                    └─> SqliteBlissIdManager [persistence]

Data Write Request
    └─> ShardId::new(data) [SHA256]
            └─> ShardHandle::from_data()
                    └─> MerkleTree::from_leaves()
                            └─> NetworkOrchestrator::replicate()
                                    └─> CircuitBreaker::execute()
                                            └─> RateLimiter check
                                                    └─> Storage backend
```

---

## Key Architectural Concepts

### BlissID (Soul Identity)
Every actor in AuraFS has a `BlissId` — a 64-char hex SHA3-256 content-addressed identity derived from biometric or cryptographic input. All operations carry soul context for full accountability. The `PersistentBlissIdManager` signs every registration/deactivation with **Dilithium5** (NIST PQC standard) for quantum-safe audit trails.

### Fractal Sharding
Data is split into content-addressed `Shard`s with SHA256 IDs, hierarchically organized as `FractalShard` trees. Each shard carries `CoherenceState` (Strict / DecoherenceExempt / Redistributing) for physics-informed storage enforcement based on T2 coherence windows.

### Merkle Integrity
Every shard set is anchored to a `MerkleTree` using SHA3-256 with domain-separated prefixes (0x00 leaf, 0x01 internal, 0x02 async-pending). Proofs are generated and verified per-leaf. Async-pending leaves change the root hash to signal in-flight writes.

### Post-Quantum Cryptography
All signatures use **Dilithium5** (CRYSTALS-Dilithium, FIPS 204). Hashing uses SHA3-256 and BLAKE3. No classical RSA/EC signatures appear in the core identity layer.

### Enterprise Resilience
- **Circuit Breaker**: 3-state (Closed → Open → HalfOpen) with configurable failure/success thresholds and timeout
- **Rate Limiter**: Sliding window + token bucket, soul-aware per-identity quotas
- **Health Checks**: K8s liveness/readiness probe compatible `HealthManager`
- **Distributed Tracing**: OpenTelemetry-compatible `Tracer` with configurable sample rate

---

## Error Architecture

The error system is layered:
1. `AuraFSError` — domain-specific variants (Config, Shard, Soul, Governance, Network, Storage, Crypto, External, IO, Serde, Other)
2. `ErrorContext` — classification (class, phase, severity, code, soul_id, shard_id, timestamp)
3. `CoreError` — wrapper combining both, with HTTP status mapping, gRPC mapping, structured logging, and retryability checks

Helper constructors: `internal()`, `client()`, `transient_network()`.

---

## Sister Modules (Top-Level)

| Module | Description |
|---|---|
| `aps/` | Aurphyx Physics Simulation — simulation models for T2 coherence, topological dynamics |
| `ftqc/` | Fault-tolerant quantum computing integration |
| `fuxyez/` | Fuxyez protocol (details in module) |
| `memoree/` | Persistent memory / identity continuity |
| `sages/` | SAGES (Systems Architecture for Governed Entity Sovereignty) |
| `standards/` | Physical and computational standards docs |
| `tslca/` | TSLCA security framework |
| `tvfd/` | Topological Vacuum Flux Dynamics research |
| `openscad/` | Hardware design for quantum/physical nodes |

---

## Technology Stack

| Layer | Technology |
|---|---|
| Language | Rust (2021 edition) |
| Async runtime | Tokio |
| Identity crypto | Dilithium5 (CRYSTALS, NIST PQC) |
| Hashing | SHA3-256, SHA-256, SHAKE256, BLAKE3, GuardHash, SoulCrypt |
| Serialization | Serde (JSON + TOML) |
| Persistence | SQLite (via sqlx) |
| Observability | OpenTelemetry-compatible tracing, Prometheus metrics |
| Container | Docker + Kubernetes (Helm) |
| Filesystem interface | FUSE |
| Build | Cargo workspace |

---

*This document was auto-generated by recursive codebase crawl on 2026-04-27. For the most current state, regenerate from the live repo.*
