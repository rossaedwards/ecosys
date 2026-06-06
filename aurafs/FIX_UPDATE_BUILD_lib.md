# FIX_UPDATE_BUILD — src/lib.rs
## AuraFS Physics Compliance Audit
**Date:** 2026-02-10 17:14 EST  
**Auditor:** AURPHYX-CURSOR-DEPLOY-002 v2.0  
**Scope:** `src/lib.rs` (NOT TRL-4 locked)

---

## Findings

### Files Scanned: 1
| File | TRL-4 Locked | Violations | Status |
|------|:---:|:---:|--------|
| `src/lib.rs` | NO | 3 | REMEDIATED |

### Detailed Violations

#### `src/lib.rs` — 3 violations (prelude re-exports)
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 29 | `FRACTAL_SCALING_BIAS,` | Removed (replaced by `INVARIANTS`) | η (5.3) |
| 30 | `COHERENCE_WINDOW_US,` | Removed (replaced by `INVARIANTS`) | T₂ (1600μs) |
| 31 | `SPECTRAL_DIMENSION,` | Removed (replaced by `INVARIANTS`) | d_s (1.37) |

The prelude now exports:
```rust
pub use crate::physics::{
    PhysicsViolationError,
    DecoherenceRecovery,
    INVARIANTS,
};
```

All modules that `use crate::prelude::*` now get `INVARIANTS` in scope and access constants via `INVARIANTS.field_name`.

### False Positives Identified: 2
- `"5.3x Scaling Bias Active"` in line 133 — string literal in status message
- `"1.37 ds"` references in doc comments — informational annotations

### Governance Required: NO
File is NOT TRL-4 locked.
