# FIX_UPDATE_BUILD — src/gov/
## AuraFS Physics Compliance Audit
**Date:** 2026-02-10 17:14 EST  
**Auditor:** AURPHYX-CURSOR-DEPLOY-002 v2.0  
**Scope:** `src/gov/` (mixed TRL-4 status)

---

## Findings

### Files Scanned: 12
| File | TRL-4 Locked | Violations | Status |
|------|:---:|:---:|--------|
| `src/gov/sages.rs` | YES | 3 | REMEDIATED |
| `src/gov/soulsync_engine.rs` | NO | 0 | CLEAN (loads quorum from config) |
| `src/gov/mod.rs` | NO | 0 | CLEAN |
| `src/gov/models.rs` | NO | 0 | CLEAN |
| `src/gov/voting_engine.rs` | NO | 0 | CLEAN |
| `src/gov/proposal_manager.rs` | NO | 0 | CLEAN |
| `src/gov/policy_enforcer.rs` | NO | 0 | CLEAN |
| `src/gov/blissid_manager.rs` | NO | 0 | CLEAN |
| `src/gov/server.rs` | NO | 0 | CLEAN |
| `src/gov/transaction_type.rs` | NO | 0 | CLEAN |
| All other files | NO | 0 | CLEAN |

### Detailed Violations

#### `src/gov/sages.rs` (TRL-4 LOCKED) — 3 violations
| Line | Original | Replacement | Constant |
|------|----------|-------------|----------|
| 7 | Missing explicit `INVARIANTS` import | Added `use crate::physics::{INVARIANTS, ...}` | Import fix |
| 114 | `ds_threshold: SPECTRAL_DIMENSION` | `ds_threshold: INVARIANTS.spectral_dimension` | d_s (1.37) |
| 131 | `SpectralDecoherence { actual }` | `SpectralDecoherence { actual, .. }` | Pattern fix (missing fields) |
| 142 | `StabilityTimeout { elapsed }` | `StabilityTimeout { elapsed, .. }` | Pattern fix (missing fields) |

**Note:** `soulsync_engine.rs` correctly loads `min_quorum` from `AuraConfig` at runtime (line 159-162), which in turn loads from `aurafs.toml`. This is compliant — the quorum value flows through the config system, not as a hardcoded literal.

### False Positives Identified: 2
- `coherence_score: 1.0` in `sages.rs` line 108 — initial coherence score, not a physics constant
- `13` sentinel count referenced in doc comments — informational, not a code literal

### Governance Required: NO
All changes are internal refactoring (accessor pattern + pattern matching fix). No public API modifications.
