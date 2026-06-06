# FIX_UPDATE_BUILD — src/config.rs
## AuraFS Physics Compliance Audit
**Date:** 2026-02-10 17:14 EST  
**Auditor:** AURPHYX-CURSOR-DEPLOY-002 v2.0  
**Scope:** `src/config.rs` (NOT TRL-4 locked)

---

## Findings

### Files Scanned: 1 (+ `src/core/config.rs` — separate module, clean)
| File | TRL-4 Locked | Violations | Status |
|------|:---:|:---:|--------|
| `src/config.rs` | NO | 5 | REMEDIATED |

### Detailed Violations

#### `src/config.rs` — 5 violations
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 4 | `use crate::physics::{FRACTAL_SCALING_BIAS, SPECTRAL_DIMENSION, COHERENCE_WINDOW_US, PHOTONIC_BAND_GAP}` | `use crate::physics::INVARIANTS` | Import fix (4 non-existent symbols) |
| 94 | `fn default_scaling_bias() -> f64 { FRACTAL_SCALING_BIAS }` | `{ INVARIANTS.hilbert_scaling_bias }` | η (5.3) |
| 95 | `fn default_spectral_dimension() -> f64 { SPECTRAL_DIMENSION }` | `{ INVARIANTS.spectral_dimension }` | d_s (1.37) |
| 96 | `fn default_coherence_window() -> u64 { COHERENCE_WINDOW_US }` | `{ INVARIANTS.coherence_window_us }` | T₂ (1600μs) |
| 97 | `fn default_pbg() -> f64 { PHOTONIC_BAND_GAP }` | `{ INVARIANTS.photonic_band_gap }` | PBG (0.21) |

### False Positives Identified: 0
No other numeric literals match physics constants.

### Governance Required: NO
File is NOT TRL-4 locked.
