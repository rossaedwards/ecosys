# FIX_UPDATE_BUILD — src/network/
## AuraFS Physics Compliance Audit
**Date:** 2026-02-10 17:14 EST  
**Auditor:** AURPHYX-CURSOR-DEPLOY-002 v2.0  
**Scope:** `src/network/` (mixed TRL-4 status)

---

## Findings

### Files Scanned: 30+
| File | TRL-4 Locked | Violations | Status |
|------|:---:|:---:|--------|
| `src/network/meshwerk/topology_engine.rs` | YES | 4 | REMEDIATED |
| `src/network/meshwerk/routing.rs` | YES | 1 | REMEDIATED |
| `src/network/meshwerk/mesh_node.rs` | NO | 1 | REMEDIATED |
| `src/network/transport/starlink_client.rs` | YES | 2 | REMEDIATED |
| `src/network/p2p.rs` | NO | 4 | REMEDIATED |
| `src/network/roles.rs` | NO | 3 | REMEDIATED |
| `src/network/meshwerk/mod.rs` | YES | 0 | CLEAN |
| `src/network/meshwerk/roles.rs` | YES | 0 | CLEAN |
| `src/network/node_manager.rs` | YES | 0 | CLEAN |
| `src/network/orchestrator.rs` | YES | 0 | CLEAN |
| `src/network/autoheal_daemon.rs` | YES | 0 | CLEAN |
| All other files | Various | 0 | CLEAN |

### Detailed Violations

#### `src/network/meshwerk/topology_engine.rs` (TRL-4 LOCKED) — 4 violations
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 2 | `use crate::physics::mod::{HILBERT_BIAS, SPECTRAL_DIMENSION, COHERENCE_WINDOW_US}` | `use crate::physics::INVARIANTS` | Import fix |
| 17 | `target_ds: SPECTRAL_DIMENSION` | `target_ds: INVARIANTS.spectral_dimension` | d_s (1.37) |
| 25 | `HILBERT_BIAS` | `INVARIANTS.hilbert_scaling_bias` | η (5.3) |
| 36 | `(total_mesh_size as f64).log(HILBERT_BIAS)` | `...log(INVARIANTS.hilbert_scaling_bias)` | η (5.3) |

**Justification:** Internal refactoring (magic number alias → INVARIANTS accessor). No public API change. Permitted under TRL-4 lock §2.

#### `src/network/meshwerk/routing.rs` (TRL-4 LOCKED) — 1 violation
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 6 | `use crate::physics::{PHOTONIC_BAND_GAP, ...}` | `use crate::physics::{INVARIANTS, ...}` | Import fix |
| 20 | `pbg_floor: PHOTONIC_BAND_GAP` | `pbg_floor: INVARIANTS.photonic_band_gap` | PBG (0.21) |

#### `src/network/meshwerk/mesh_node.rs` (NOT locked) — 1 violation
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 5 | `use crate::physics::{PHOTONIC_BAND_GAP, ...}` | `use crate::physics::{INVARIANTS, ...}` | Import fix |
| 58 | `pbg_floor: PHOTONIC_BAND_GAP` | `pbg_floor: INVARIANTS.photonic_band_gap` | PBG (0.21) |

#### `src/network/transport/starlink_client.rs` (TRL-4 LOCKED) — 2 violations
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 3 | `use crate::physics::mod::{COHERENCE_WINDOW_US}` | `use crate::physics::INVARIANTS` | Import fix (invalid path) |
| 34 | `if latency_us > COHERENCE_WINDOW_US` | `if latency_us > INVARIANTS.coherence_window_us` | T₂ (1600μs) |

#### `src/network/p2p.rs` (NOT locked) — 4 violations
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 16 | `use crate::physics::{COHERENCE_WINDOW_US, ...}` | `use crate::physics::{INVARIANTS, ...}` | Import fix |
| 135 | `limit: COHERENCE_WINDOW_US` | `limit: INVARIANTS.coherence_window_us` | T₂ (1600μs) |
| 147-148 | `COHERENCE_WINDOW_US + 1` / `COHERENCE_WINDOW_US` (×2) | `INVARIANTS.coherence_window_us + 1` / `INVARIANTS.coherence_window_us` | T₂ (1600μs) |

#### `src/network/roles.rs` (NOT locked) — 3 violations
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 1 | `use crate::physics::invariants::{COHERENCE_WINDOW_US, HILBERT_BIAS}` | `use crate::physics::INVARIANTS` | Import fix (invalid module path) |
| 20-21 | `HILBERT_BIAS.log2()` / `HILBERT_BIAS.powi(2)` | `INVARIANTS.hilbert_scaling_bias.log2()` / `.powi(2)` | η (5.3) |
| 27 | `(COHERENCE_WINDOW_US / 10) as u64` | `INVARIANTS.coherence_window_us / 10` | T₂ (1600μs) |

### False Positives Identified: 8
- Port `6000` in `p2p.rs` test — network test address
- `Duration::from_secs(10)`, `Duration::from_secs(1)` in `p2p.rs` — connection timeouts
- `max_connections: 100` in `p2p.rs` — connection pool size
- `max_retries: 3` in `p2p.rs` — retry count
- `gossip_interval_ms = 500` — in aurafs.toml, not source code
- `(8, 2500)`, `(16, 1000)`, `(32, 250)` in `mesh_node.rs` — tier hop/latency limits (not physics)

### Governance Required: NO
All TRL-4 locked changes are internal refactoring (import path + accessor pattern). No public API changes.
