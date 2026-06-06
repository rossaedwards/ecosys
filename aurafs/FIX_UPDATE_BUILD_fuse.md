# FIX_UPDATE_BUILD — src/fuse/
## AuraFS Physics Compliance Audit
**Date:** 2026-02-10 17:14 EST  
**Auditor:** AURPHYX-CURSOR-DEPLOY-002 v2.0  
**Scope:** `src/fuse/` (NOT TRL-4 locked, except where noted)

---

## Findings

### Files Scanned: 7
| File | TRL-4 Locked | Violations | Status |
|------|:---:|:---:|--------|
| `src/fuse/mod.rs` | NO | 1 | REMEDIATED |
| `src/fuse/node.rs` | NO | 6 | REMEDIATED |
| `src/fuse/session.rs` | NO | 3 | REMEDIATED |
| `src/fuse/timed_lock.rs` | NO | 4 | REMEDIATED |
| `src/fuse/filesystem.rs` | NO | 0 | CLEAN |
| `src/fuse/inode_cache.rs` | NO | 0 | CLEAN |
| `src/fuse/main.rs` | NO | 0 | CLEAN |

### Detailed Violations

#### `src/fuse/mod.rs` — 1 violation
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 30 | `Ok(SPECTRAL_DIMENSION)` | `Ok(INVARIANTS.spectral_dimension)` | d_s (1.37) |

#### `src/fuse/node.rs` — 6 violations
| Lines | Original | Replacement | Constant |
|-------|----------|-------------|----------|
| 25 | `use crate::physics::{COHERENCE_WINDOW_US, ...}` | `use crate::physics::{INVARIANTS, ...}` | Import fix |
| 253, 255 | `COHERENCE_WINDOW_US` (×2) | `INVARIANTS.coherence_window_us` | T₂ (1600μs) |
| 270, 272 | `COHERENCE_WINDOW_US` (×2) | `INVARIANTS.coherence_window_us` | T₂ (1600μs) |
| 295, 297 | `COHERENCE_WINDOW_US` (×2) | `INVARIANTS.coherence_window_us` | T₂ (1600μs) |

#### `src/fuse/session.rs` — 3 violations
| Lines | Original | Replacement | Constant |
|-------|----------|-------------|----------|
| 99, 102 | `COHERENCE_WINDOW_US` (×2) | `INVARIANTS.coherence_window_us` | T₂ (1600μs) |
| 153 | `Duration::from_micros(COHERENCE_WINDOW_US)` | `Duration::from_micros(INVARIANTS.coherence_window_us)` | T₂ (1600μs) |

#### `src/fuse/timed_lock.rs` — 4 violations (CRITICAL)
| Lines | Original | Replacement | Constant |
|-------|----------|-------------|----------|
| 26, 46 | `Duration::from_micros(100)` (×2) | `Duration::from_micros(INVARIANTS.lock_acquisition_timeout_us)` | Lock timeout (100μs) |
| 36 | `limit: COHERENCE_WINDOW_US` | `limit: INVARIANTS.lock_acquisition_timeout_us` | **Bug fix**: was reporting wrong limit |
| 56 | `limit: COHERENCE_WINDOW_US` | `limit: INVARIANTS.lock_acquisition_timeout_us` | **Bug fix**: was reporting wrong limit |

**Note:** The error type was also corrected from `StabilityTimeout` to `LockTimeout` since these are lock acquisition failures, not coherence window breaches.

### False Positives Identified: 3
- `Duration::from_secs(60)` in `session.rs` line 163 — stream eviction cutoff, not physics
- `mpsc::channel(100)` in `session.rs` line 66 — channel buffer size
- `4096` in `node.rs` lines 110, 157, 198 — filesystem block size

### Governance Required: NO
All files are NOT TRL-4 locked. Changes are internal refactoring.
