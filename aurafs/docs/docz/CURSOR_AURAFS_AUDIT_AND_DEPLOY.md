# CURSOR MASTER PROMPT: AuraFS Physics Compliance Audit & Deployment Build Plan

**Document:** `AURPHYX-CURSOR-DEPLOY-002`
**Version:** 2.0 | **Date:** 2026-02-08
**Author:** Ross A. Edwards — Aurphyx LLC
**ORCID:** 0009-0008-0539-1289

---

## PREAMBLE: EXECUTION CONTRACT

You are Cursor, operating as the build-and-audit agent for **AuraFS** — a physics-informed distributed storage system written in Rust. This prompt is a **sequential, gated execution plan**. Each phase produces artifacts that gate the next. If a phase fails, halt and report the failure with the exact file path, line number, and error message. Do not fabricate fixes. If you are uncertain whether a literal is a physics constant or a false positive, emit a `// TODO(PHYSICS_REVIEW): <reason>` comment and move on.

### Environment

| Setting | Value |
|---------|-------|
| Dev OS | Windows 11 / PowerShell |
| Prod OS | Linux (Debian/Fedora) / Docker / K8s |
| Language | Rust 2024 edition, Python for scripting |
| Pathing | `std::path::PathBuf` (cross-OS); never raw `/` or `\` |
| Prohibited | `chmod`, `sudo`, bash scripts in core, `unsafe` without `PHYSICS OVERRIDE` |

### Governing Documents (Read Before Any Code)

These six files constitute the authority chain. You must read all six into your context window during Phase 0 before writing any code or generating any report.

```
1. @.cursorrules                              → Coding standards, physics rules, prohibited actions
2. @aurafs.toml                               → [physics] section = single source of truth for constants
3. @aurafs/compliance/PHYSICS_INVARIANTS.json  → Machine-readable invariant bounds + CI test directives
4. @aurafs/compliance/ALGORITHM_PROOFS.md      → Mathematical derivations (Theorems 2.1, Prop 2.1, 2.3)
5. @aurafs/compliance/DARPA_TRL_VALIDATION.md  → TRL mapping and experimental protocol budgets
6. @aurafs/compliance/SECURITY_AUDIT.md        → PQC audit, Dilithium-5/Kyber-1024 status
```

---

## PHYSICS INVARIANTS (NON-NEGOTIABLE)

Any `.rs` file that uses one of these values as a bare literal instead of routing through `physics::INVARIANTS` is a **compliance violation** per `.cursorrules` §1 and §5.

### Primary Constants (from `aurafs.toml` `[physics]`)

| Constant | Symbol | Value | Tolerance | TOML Key | Accessor |
|---|---|---|---|---|---|
| Fractal Scaling Bias | η | `5.3` | ±0.05 | `hilbert_scaling_bias` | `physics::INVARIANTS.hilbert_scaling_bias` |
| Coherence Window | T₂ | `1600` μs | ±100 μs | `coherence_window_us` | `physics::INVARIANTS.coherence_window_us` |
| Spectral Dimension | d_s | `1.37` | ±0.05 | `spectral_dimension` | `physics::INVARIANTS.spectral_dimension` |
| Photonic Band Gap | PBG | `0.21` | ±0.03 | `photonic_band_gap` | `physics::INVARIANTS.photonic_band_gap` |
| Lock Timeout | — | `100` μs | derived | `lock_acquisition_timeout_us` | `physics::INVARIANTS.lock_acquisition_timeout_us` |

### Secondary Constants (scan but classify carefully)

| Constant | Value | Context Clue | Source |
|---|---|---|---|
| Fractal Density (Hausdorff) | `1.585` | log(3)/log(2), lattice geometry | Validation Report |
| Min Quorum | `13` | governance, Byzantine 3f+1 | `aurafs.toml` `[governance]` |
| Fidelity Factor | `16` | T₂ ratio, coherence improvement | Thesis §2 |
| Fractal QEC Overhead | `89` | error correction ratio | Prop. 2.3 |
| Euclidean QEC Overhead | `1458` | surface code baseline | Prop. 2.3 |

---

## PHASE 0: INITIALIZATION — BUILD THE INDEX

### 0.1 — Load Governing Documents

Execute in order. Do not proceed until all are in context.

```
READ @.cursorrules
READ @aurafs.toml
READ @compliance/PHYSICS_INVARIANTS.json
READ @compliance/ALGORITHM_PROOFS.md
READ @compliance/DARPA_TRL_VALIDATION.md
READ @compliance/SECURITY_AUDIT.md
READ @VALIDATION_REPORT.md
```

Confirm you can resolve every `[physics]` key in `aurafs.toml` to a matching entry in `PHYSICS_INVARIANTS.json`. If any key is missing from the JSON, halt and report.

### 0.2 — Build the Module Index

Recursively scan `@aurafs/src/` and build an internal index. For each `.rs` file, record:

```json
{
  "path": "src/core/shard.rs",
  "subfolder": "core",
  "trl_locked": true,
  "lock_source": "[modules.validated] core"
}
```

**TRL-4 Lock Resolution:** The `[modules.validated]` section of `aurafs.toml` defines which files are locked. Cross-reference these exact paths:

**core (LOCKED):**
- `src/physics/mod.rs`
- `src/core/shard.rs`
- `src/core/merkle.rs`
- `src/main.rs`

**network_meshwerk (LOCKED):**
- `src/network/meshwerk/mod.rs`
- `src/network/meshwerk/roles.rs`
- `src/network/meshwerk/routing.rs`
- `src/network/meshwerk/topology_engine.rs`

**network_control (LOCKED):**
- `src/network/node_manager.rs`
- `src/network/orchestrator.rs`
- `src/network/autoheal_daemon.rs`

**transports (LOCKED):**
- `src/network/transport/starlink_client.rs`
- `src/network/meshtastic_integration/firmware_bridge.rs`
- `src/network/rns_bridge.rs`
- `src/network/transport/udp_multicast.rs`

**crypto_audit (LOCKED):**
- `src/crypto/pqc/dilithium_sig.rs`
- `src/audit/holographic_logger.rs`
- `src/gov/sages.rs`

**ai_compression (LOCKED):**
- `src/ai/fractal_orchestrator.rs`
- `src/compression/lattice.rs`
- `src/compression/manager.rs`

**configs (LOCKED):**
- `src/network/meshwerk/configs/ghostlink.toml`

All other `.rs` files are **UNLOCKED** and may be freely modified.

**Lock Rule:** For TRL-4 locked files, you may replace magic-number internals with `physics::INVARIANTS` references (this is a compliance remediation, not a logic change), but you must NOT alter public API signatures. If a locked file needs a public API change, mark it `// TODO(PHYSICS_OVERRIDE): requires governance vote` and skip.

### 0.3 — Verify or Create `physics::INVARIANTS` Module

Check `src/physics/mod.rs` for the presence of a `PhysicsInvariants` struct loaded from `aurafs.toml`.

**If it already exists and loads from config:** Map each constant to its public accessor. Record the exact accessor syntax (e.g., `INVARIANTS.hilbert_scaling_bias`) for use in Phase 1 replacements.

**If it exists but uses hardcoded values internally:** Refactor it to deserialize from `aurafs.toml` `[physics]`. This is a Phase 0 prerequisite.

**If it does not exist:** Create `src/physics/invariants.rs` with the following scaffold and register it in `src/physics/mod.rs`:

```rust
// src/physics/invariants.rs
//
// AuraFS Physics Invariants — Deserialized from aurafs.toml [physics]
// DO NOT hardcode values. All values loaded at init from config.
// Authority chain: aurafs.toml → PHYSICS_INVARIANTS.json → ALGORITHM_PROOFS.md
//
// [Theorem 2.1] Fractal Hilbert Scaling: Replicas = ceil(log_η(Nodes))
// [Prop 2.1]    Spectral dimension d_s = 2·log(3)/log(5) ≈ 1.365 → clamped 1.37
// [Section IV]  Photonic band gap Δω/ω = 0.21 (21% TM gap, C₆ᵥ lattice)

use once_cell::sync::Lazy;
use serde::Deserialize;

/// Physics invariants deserialized from aurafs.toml [physics].
/// Modifying these values requires a PHYSICS OVERRIDE governance vote (min_quorum = 13).
#[derive(Debug, Clone, Deserialize)]
pub struct PhysicsInvariants {
    /// η — Fractal Hilbert Scaling Bias (Theorem 2.1)
    /// Replica formula: Replicas = ceil(log_{η}(Nodes))
    /// Validated: 5.3× at N=100, D=5 (VALIDATION_REPORT.md §1)
    pub hilbert_scaling_bias: f64,

    /// T₂ — Maximum coherence window (microseconds)
    /// 16× improvement over transmon baseline (100 μs → 1600 μs)
    /// Mechanism: Anderson Localization (d_s < 2 guarantees all states localized)
    /// Validated: VALIDATION_REPORT.md §2
    pub coherence_window_us: u64,

    /// d_s — Spectral dimension of Sierpiński gasket
    /// Theoretical exact: 2·log(3)/log(5) ≈ 1.365
    /// Implementation clamp: 1.37 (within ±0.05 tolerance)
    /// Validated: VALIDATION_REPORT.md §3
    pub spectral_dimension: f64,

    /// PBG — Photonic band gap (fractional bandwidth Δω/ω)
    /// Complete TM gap in C₆ᵥ hexagonal lattice (ε=12, r/a=0.2)
    /// Used as routing overhead allowance: usable capacity = (1 - PBG) × total
    /// Validated: VALIDATION_REPORT.md §4
    pub photonic_band_gap: f64,

    /// FUSE lock acquisition timeout (microseconds)
    /// Derived: coherence_window_us / 16
    pub lock_acquisition_timeout_us: u64,
}

impl PhysicsInvariants {
    /// Validate all invariants against PHYSICS_INVARIANTS.json tolerance bounds.
    /// Panics at startup if any value is outside compliance range.
    pub fn validate(&self) {
        assert!(
            (5.25..=5.35).contains(&self.hilbert_scaling_bias),
            "PHYSICS VIOLATION: hilbert_scaling_bias={} outside [5.25, 5.35]",
            self.hilbert_scaling_bias
        );
        assert!(
            (1500..=1700).contains(&self.coherence_window_us),
            "PHYSICS VIOLATION: coherence_window_us={} outside [1500, 1700]",
            self.coherence_window_us
        );
        assert!(
            (1.32..=1.42).contains(&self.spectral_dimension),
            "PHYSICS VIOLATION: spectral_dimension={} outside [1.32, 1.42]",
            self.spectral_dimension
        );
        assert!(
            (0.18..=0.24).contains(&self.photonic_band_gap),
            "PHYSICS VIOLATION: photonic_band_gap={} outside [0.18, 0.24]",
            self.photonic_band_gap
        );
    }

    /// Compute replica count per Theorem 2.1:
    /// Replicas = ceil(log_{η}(N))
    pub fn compute_replicas(&self, node_count: usize) -> usize {
        if node_count <= 1 { return 1; }
        ((node_count as f64).ln() / self.hilbert_scaling_bias.ln()).ceil() as usize
    }

    /// Check spectral dimension health. Returns Err if deviation exceeds ±0.05.
    pub fn check_spectral_health(&self, measured_ds: f64) -> Result<(), PhysicsViolationError> {
        let deviation = (measured_ds - self.spectral_dimension).abs();
        if deviation > 0.05 {
            Err(PhysicsViolationError::DecoherenceThreshold {
                measured: measured_ds,
                target: self.spectral_dimension,
                deviation,
            })
        } else {
            Ok(())
        }
    }
}

/// Per .cursorrules §2: "return a PhysicsViolationError, not a generic error."
#[derive(Debug, thiserror::Error)]
pub enum PhysicsViolationError {
    #[error("Decoherence threshold: measured d_s={measured:.4}, target={target:.4}, Δ={deviation:.4}")]
    DecoherenceThreshold { measured: f64, target: f64, deviation: f64 },

    #[error("Scaling violation: {computed} replicas < expected {expected} for {nodes} nodes")]
    ScalingBiasViolation { computed: usize, expected: usize, nodes: usize },

    #[error("Coherence exceeded: {elapsed_us}μs > limit {limit_us}μs")]
    CoherenceWindowExceeded { elapsed_us: u64, limit_us: u64 },
}

// ──────────────────────────────────────────────
// GLOBAL SINGLETON — loaded once at startup
// ──────────────────────────────────────────────

pub static INVARIANTS: Lazy<PhysicsInvariants> = Lazy::new(|| {
    let config_str = std::fs::read_to_string("aurafs.toml")
        .expect("FATAL: aurafs.toml not found — cannot load physics invariants");
    let config: toml::Value = config_str.parse()
        .expect("FATAL: aurafs.toml is not valid TOML");
    let physics = config.get("physics")
        .expect("FATAL: aurafs.toml missing [physics] section");
    let inv: PhysicsInvariants = physics.clone().try_into()
        .expect("FATAL: [physics] section does not match PhysicsInvariants struct");
    inv.validate();
    inv
});
```

Register in `src/physics/mod.rs`:
```rust
pub mod invariants;
pub use invariants::{PhysicsInvariants, PhysicsViolationError, INVARIANTS};
```

### 0.4 — Phase 0 Gate Check

Before proceeding to Phase 1, confirm ALL of the following:

- [ ] `.cursorrules` loaded — §1 physics constants match `aurafs.toml`
- [ ] `aurafs.toml` `[physics]` section has all 5 keys
- [ ] `compliance/PHYSICS_INVARIANTS.json` parses as valid JSON with `invariants[]` array
- [ ] `physics::INVARIANTS` module exists and compiles (`cargo check -p aurafs`)
- [ ] Module index built: total `.rs` files counted, locked vs. unlocked classified
- [ ] `VALIDATION_REPORT.md` values match `aurafs.toml` values (cross-check)

**If any gate fails: HALT. Report the specific failure. Do not proceed.**

---

## PHASE 1: MAGIC NUMBER AUDIT & REMEDIATION

### 1.1 — Scan Target Definitions

Search all `.rs` files under `src/` recursively (including subdirectories: `acl/`, `ai/`, `api/`, `audit/`, `bin/`, `cache/`, `cli/`, `compression/`, `config/`, `core/`, `crypto/`, `dedup/`, `enterprise/`, `error/`, `fuse/`, `gov/`, `heal/`, `mesh/`, `model_slice/`, `monitoring/`, `namespace/`, `network/`, `ops/`, `physics/`, `quantum/`, `redteam/`, `resilience/`, `shard/`, `shard_server/`, `snapshot/`, `storage/`, `whitehat/`).

Also scan these non-`src/` Rust files:
- `benches/mod.rs`
- `plugins/mod.rs`
- `sdk/afs_sdk.rs`, `sdk/mod.rs`
- `snapshots/mod.rs`
- `versioning/mod.rs`, `versioning/tracker.rs`
- `examples/basic_usage.rs`, `examples/main.rs`
- `tests/mod.rs`
- `integrations/aurafs/backend.rs`, `integrations/aurafs/mod.rs`, `integrations/aurafs/shard.rs`
- `tts/mod.rs`, `tts/tts_engine.rs`, `tts/src/**/*.rs`
- `ui/mod.rs`

Also scan these non-Rust config files for hardcoded physics values:
- `src/network/meshwerk/configs/ghostlink.toml`
- `src/network/meshwerk/configs/dataslayer.toml`
- `docker/docker-compose.yml`
- `helm/values.yaml`, `helm/values-prod.yaml`
- `simulations/*.py`

### 1.2 — Regex Patterns

Apply these regex patterns to each file. Each pattern targets a specific physics constant.

**PRIMARY targets (must be replaced if used in computation):**

| ID | Target | Regex | Replacement |
|---|---|---|---|
| P1 | `5.3` (η) | `(?<!\d)(?<!\.)5\.3(?:0*)(?:_f64\|_f32)?(?!\d)` | `physics::INVARIANTS.hilbert_scaling_bias` |
| P2 | `1600` (T₂) | `(?<!\d)1600(?:_u64\|_u32\|_usize\|_i64)?(?!\d)` | `physics::INVARIANTS.coherence_window_us` |
| P3 | `1.37` (d_s) | `(?<!\d)(?<!\.)1\.37(?:0*)(?:_f64\|_f32)?(?!\d)` | `physics::INVARIANTS.spectral_dimension` |
| P4 | `0.21` (PBG) | `(?<!\d)(?<!\.)0\.21(?:0*)(?:_f64\|_f32)?(?!\d)` | `physics::INVARIANTS.photonic_band_gap` |
| P5 | `1.585` (D_f) | `(?<!\d)(?<!\.)1\.585(?:0*)(?:_f64\|_f32)?(?!\d)` | `physics::INVARIANTS.fractal_density` (add field if missing) |
| P6 | `100` (lock) | `(?<!\d)100(?:_u64\|_u32\|_usize)?(?!\d)` | `physics::INVARIANTS.lock_acquisition_timeout_us` |

**IMPORTANT for P6 (`100`):** This pattern will produce many false positives (100MB limits, HTTP status codes, loop bounds, etc.). Only classify as `VIOLATION` if the literal appears in a FUSE lock, trap-state acquisition, or timeout context. All other matches of `100` are `FALSE_POSITIVE`.

### 1.3 — Classification Rules

For every regex match, classify into exactly one of four categories:

| Class | Definition | Action |
|---|---|---|
| **VIOLATION** | The literal is used in computation (variable assignment, comparison, arithmetic) and is not loaded from config or `physics::INVARIANTS`. | **Replace** with config accessor. |
| **COMMENT_REF** | The literal appears inside a comment (`//`, `///`, `/* */`) or docstring referencing the thesis or validation report. | **No action.** These are documentation. |
| **TEST_ASSERTION** | The literal appears inside `#[cfg(test)]` or `#[test]` blocks in an `assert!` or similar check. | **Replace** the literal with the config accessor so the test validates against config, not a magic number. |
| **FALSE_POSITIVE** | The literal is unrelated to physics. Examples: version strings (`"0.21.0"`), byte sizes (`100 * 1024 * 1024`), HTTP codes, array indices, IP address fragments, Cargo.toml versions. | **No action.** Log as false positive. |

**Classification Decision Tree:**

```
1. Is the match inside a comment (// or /* */)? → COMMENT_REF
2. Is the match inside a string literal ("..." or format!(...))? → Usually FALSE_POSITIVE
   Exception: if the string is a physics label like "1600μs" → COMMENT_REF
3. Is the match inside #[cfg(test)] or #[test]? → TEST_ASSERTION
4. Is the match inside a const/let/static assignment used for physics? → VIOLATION
5. Is the match part of an unrelated numeric (100MB, 100%, port 1600)? → FALSE_POSITIVE
6. Still unsure? → Mark as TODO(PHYSICS_REVIEW) and skip
```

### 1.4 — Replacement Procedure

For each `VIOLATION`, execute these steps **in order**:

**Step 1: Import.** Add `use crate::physics::INVARIANTS;` to the file's import block if not already present. If the file is outside the main crate (e.g., `sdk/`, `tts/`, `integrations/`), add the appropriate cross-crate import or note that a direct `aurafs.toml` read is needed.

**Step 2: Replace.** Swap the literal with the correct accessor.

Example transformations:

```rust
// BEFORE (VIOLATION):
let bias = 5.3;
let replicas = (nodes as f64).log(5.3).ceil() as usize;
if tick_interval_us > 1600 { panic!("coherence exceeded"); }
let threshold = 1.37;
let overhead = 0.21;

// AFTER (COMPLIANT):
let bias = INVARIANTS.hilbert_scaling_bias;  // aurafs.toml [physics]
let replicas = INVARIANTS.compute_replicas(nodes);  // [Theorem 2.1]
if tick_interval_us > INVARIANTS.coherence_window_us { panic!("coherence exceeded"); }
let threshold = INVARIANTS.spectral_dimension;  // aurafs.toml [physics]
let overhead = INVARIANTS.photonic_band_gap;  // aurafs.toml [physics]
```

**Step 3: Cite.** Add an inline comment on the replacement line:
```rust
// aurafs.toml [physics] — hilbert_scaling_bias | [Theorem 2.1]
```

**Step 4: Lock annotation.** If the file is TRL-4 LOCKED, also add:
```rust
// PHYSICS OVERRIDE: magic number remediation (TRL-4 locked file)
```

**Step 5: Compile check.** After all replacements in a single file, run:
```powershell
cargo check 2>&1
```
If compilation fails:
- **Revert** the change in that file.
- Log the error to the subfolder's `FIX_UPDATE_BUILD.md`.
- Insert `// TODO(PHYSICS_REVIEW): auto-replace failed — <error message>` at the original line.
- Continue to the next file.

### 1.5 — Known Patterns in Existing Code

Based on inspection of `src/core/shard.rs` (609 lines), here are patterns you will encounter and how to handle them:

| Pattern in `shard.rs` | Line | Classification |
|---|---|---|
| `100 * 1024 * 1024 // 100MB max` | 92, 280, 352 | FALSE_POSITIVE — byte size limit, not physics |
| `64` (hex char count) | 127, 151, 200 | FALSE_POSITIVE — SHA256 digest length |
| `1_000_000` (MAX_CHILDREN) | 429 | FALSE_POSITIVE — capacity limit |
| `1000` (MAX_REPLICAS) | 367 | FALSE_POSITIVE — but **review**: should MAX_REPLICAS derive from `compute_replicas()` ceiling? Note in report. |

These patterns will repeat across other files. Apply the same classification logic consistently.

### 1.6 — Generate FIX_UPDATE_BUILD.md Reports

For **each top-level subfolder** under `src/`, generate a report file:

```
src/<subfolder>/FIX_UPDATE_BUILD.md
```

The subfolders that need reports (based on the actual file tree):

```
src/acl/FIX_UPDATE_BUILD.md
src/ai/FIX_UPDATE_BUILD.md
src/api/FIX_UPDATE_BUILD.md
src/audit/FIX_UPDATE_BUILD.md
src/bin/FIX_UPDATE_BUILD.md
src/cache/FIX_UPDATE_BUILD.md
src/cli/FIX_UPDATE_BUILD.md
src/compression/FIX_UPDATE_BUILD.md
src/config/FIX_UPDATE_BUILD.md
src/core/FIX_UPDATE_BUILD.md
src/crypto/FIX_UPDATE_BUILD.md
src/dedup/FIX_UPDATE_BUILD.md
src/enterprise/FIX_UPDATE_BUILD.md
src/error/FIX_UPDATE_BUILD.md
src/fuse/FIX_UPDATE_BUILD.md
src/gov/FIX_UPDATE_BUILD.md
src/heal/FIX_UPDATE_BUILD.md
src/mesh/FIX_UPDATE_BUILD.md
src/model_slice/FIX_UPDATE_BUILD.md
src/monitoring/FIX_UPDATE_BUILD.md
src/namespace/FIX_UPDATE_BUILD.md
src/network/FIX_UPDATE_BUILD.md
src/ops/FIX_UPDATE_BUILD.md
src/physics/FIX_UPDATE_BUILD.md
src/quantum/FIX_UPDATE_BUILD.md
src/redteam/FIX_UPDATE_BUILD.md
src/resilience/FIX_UPDATE_BUILD.md
src/shard/FIX_UPDATE_BUILD.md
src/shard_server/FIX_UPDATE_BUILD.md
src/snapshot/FIX_UPDATE_BUILD.md
src/storage/FIX_UPDATE_BUILD.md
src/whitehat/FIX_UPDATE_BUILD.md
```

Also generate reports for non-`src/` Rust directories:
```
integrations/FIX_UPDATE_BUILD.md
sdk/FIX_UPDATE_BUILD.md
simulations/FIX_UPDATE_BUILD.md
```

Root-level `.rs` files (`src/main.rs`, `src/lib.rs`, `src/config.rs`, `src/error.rs`, `src/autoheal_daemon.rs`) should be included in a:
```
src/FIX_UPDATE_BUILD.md  (root-level src files only)
```

### 1.7 — Report Template

Each `FIX_UPDATE_BUILD.md` must follow this exact format:

```markdown
# FIX_UPDATE_BUILD: <subfolder_name>
**Generated:** <ISO 8601 timestamp>
**Phase:** 1 — Magic Number Audit & Remediation
**Agent:** Cursor (AURPHYX-CURSOR-DEPLOY-002 v2.0)

---

## Summary

| Metric | Count |
|---|---|
| Files scanned | <N> |
| TRL-4 locked files | <N> |
| Total regex matches | <N> |
| VIOLATION (replaced) | <N> |
| VIOLATION (failed — needs manual review) | <N> |
| TEST_ASSERTION (replaced) | <N> |
| COMMENT_REF (no action) | <N> |
| FALSE_POSITIVE (no action) | <N> |
| Compile status after fixes | PASS / FAIL |

---

## Violations Fixed

### <filename.rs>  —  Line <N>
- **Pattern:** P1 (η = 5.3)
- **Classification:** VIOLATION
- **Before:** `let scaling_bias = 5.3;`
- **After:** `let scaling_bias = INVARIANTS.hilbert_scaling_bias; // aurafs.toml [physics] | [Theorem 2.1]`
- **TRL-4 Locked:** Yes / No
- **Compile check:** PASS / FAIL

(Repeat for each violation)

---

## Manual Review Required

### <filename.rs>  —  Line <N>
- **Pattern:** P6 (lock timeout = 100)
- **Reason:** Ambiguous context — could be physics lock timeout or HTTP status
- **Marked:** `// TODO(PHYSICS_REVIEW): <reason>`

(Repeat for each TODO)

---

## False Positives Logged

| File | Line | Pattern | Literal | Why False Positive |
|---|---|---|---|---|
| shard.rs | 92 | P6 | `100 * 1024 * 1024` | Byte size limit (100MB), not physics |

---

## Structural Issues Discovered (input to Phase 2)

(List any problems found during the scan that are not magic-number issues but are relevant
 to deployment readiness: dead code, missing imports, circular dependencies, TODO stubs,
 placeholder implementations, missing error types, etc.)

---
```

### 1.8 — Phase 1 Consolidated Summary

After all subfolder reports are generated, produce:

```
compliance/PHASE1_AUDIT_SUMMARY.md
```

Content:

```markdown
# Phase 1 Consolidated Audit Summary
**Date:** <ISO 8601>
**Prompt Version:** AURPHYX-CURSOR-DEPLOY-002 v2.0

## Global Metrics

| Metric | Total |
|---|---|
| Subfolders scanned | <N> |
| Total .rs files scanned | <N> |
| Total regex matches | <N> |
| Violations found | <N> |
| Violations fixed | <N> |
| Manual review needed | <N> |
| False positives | <N> |
| cargo check status | PASS / FAIL |

## Per-Subfolder Breakdown

| Subfolder | Files | Locked | Violations | Fixed | Manual | FP | Status |
|---|---|---|---|---|---|---|---|
| physics/ | ... | ... | ... | ... | ... | ... | ... |
| core/ | ... | ... | ... | ... | ... | ... | ... |
| network/ | ... | ... | ... | ... | ... | ... | ... |
| crypto/ | ... | ... | ... | ... | ... | ... | ... |
| fuse/ | ... | ... | ... | ... | ... | ... | ... |
| gov/ | ... | ... | ... | ... | ... | ... | ... |
| compression/ | ... | ... | ... | ... | ... | ... | ... |
| ai/ | ... | ... | ... | ... | ... | ... | ... |
| audit/ | ... | ... | ... | ... | ... | ... | ... |
| (all other subfolders...) | | | | | | | |

## CI Rule PHYS-001 Validation

After all fixes, re-run this regex against all non-comment code:

    (?<!//.*)(5\.3[0-9]*|1\.37[0-9]*|0\.21[0-9]*|1\.585[0-9]*|(?<!\d)1600(?!\d))

**Expected non-comment matches:** 0
**Actual:** <N>

If actual > 0, list the remaining locations.

## Phase 1 Gate Check

- [ ] All FIX_UPDATE_BUILD.md reports generated for every subfolder
- [ ] PHASE1_AUDIT_SUMMARY.md generated
- [ ] cargo check passes (zero errors; warnings acceptable)
- [ ] Zero unclassified regex matches remain
- [ ] Every TODO(PHYSICS_REVIEW) has a documented reason
- [ ] physics::INVARIANTS module compiles and passes validation asserts

**Gate status:** PASS / FAIL — If FAIL, do not proceed to Phase 2.
```

---

## PHASE 2: DEPLOYMENT BUILD PLAN

Phase 2 does **not** execute changes. It produces a comprehensive build plan document that Ross will review and approve before any execution begins. The output is a single file:

```
compliance/DEPLOYMENT_BUILD_PLAN.md
```

### 2.0 — Read Phase 1 Outputs

```
READ @compliance/PHASE1_AUDIT_SUMMARY.md
READ all @src/*/FIX_UPDATE_BUILD.md
READ @compliance/SECURITY_AUDIT.md  (for open recommendations R1–R6)
```

Incorporate every "Structural Issues Discovered" section from the Phase 1 reports into the build plan as input requirements.

### 2.1 — Document Structure

The `DEPLOYMENT_BUILD_PLAN.md` must contain these sections, each with **concrete file paths**, **estimated effort**, **acceptance criteria**, and **dependency ordering**.

---

### BUILD PHASE A: Repository Hygiene & CI/CD

**Goal:** Make `github.com/aurphyx/aurafs` a credible, professional open-source project.
**Estimated effort:** 1–2 days
**Dependencies:** Phase 1 complete.

#### A.1 — Repository Structure Audit

Compare the current file tree against this target layout and report gaps:

```
aurafs/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                    ← EXISTS (verify contents)
│   │   ├── physics_audit.yml         ← CREATE (PHYS-001 through SEC-001 enforcement)
│   │   └── release.yml               ← CREATE (tagged releases + CHANGELOG)
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md             ← CREATE
│   │   ├── feature_request.md        ← CREATE
│   │   └── physics_override.md       ← CREATE (requires quorum=13 approval)
│   ├── PULL_REQUEST_TEMPLATE.md      ← CREATE
│   └── CODEOWNERS                    ← CREATE (@rossedwards owns physics/, crypto/)
├── compliance/                       ← EXISTS (populated by Phase 0/1)
│   ├── PHYSICS_INVARIANTS.json
│   ├── ALGORITHM_PROOFS.md
│   ├── DARPA_TRL_VALIDATION.md
│   ├── SECURITY_AUDIT.md
│   ├── PHASE1_AUDIT_SUMMARY.md
│   └── DEPLOYMENT_BUILD_PLAN.md
├── src/                              ← EXISTS
│   ├── physics/
│   │   ├── mod.rs
│   │   └── invariants.rs             ← Created in Phase 0
│   ├── core/
│   ├── network/
│   ├── crypto/
│   ├── (all other modules...)
│   └── lib.rs
├── tests/                            ← EXISTS (needs expansion)
├── benches/                          ← EXISTS
├── simulations/                      ← EXISTS
├── deploy/systemd/                   ← EXISTS
├── docker/                           ← EXISTS
├── helm/                             ← EXISTS
├── docs/                             ← EXISTS (needs expansion)
│   ├── architecture.md               ← CREATE (system diagram)
│   ├── physics_primer.md             ← CREATE (non-expert invariant guide)
│   └── contributing.md               ← VERIFY (check CONTRIBUTING.md at root)
├── aurafs.toml                       ← EXISTS
├── Cargo.toml                        ← EXISTS
├── Cargo.lock                        ← EXISTS
├── README.md                         ← EXISTS (needs polish for launch)
├── CHANGELOG.md                      ← EXISTS (verify format)
├── CONTRIBUTING.md                   ← EXISTS
├── LICENSE                           ← EXISTS (verify: MIT, Apache-2.0, or dual)
├── SECURITY.md                       ← EXISTS (verify vulnerability disclosure)
└── .gitignore                        ← EXISTS
```

For each item marked `CREATE`, the plan must specify the file contents or a generation prompt. For each item marked `VERIFY`, the plan must specify what to check.

#### A.2 — CI Pipeline: `physics_audit.yml`

This is the **most critical CI workflow**. Provide the complete YAML in the plan. It must:

1. Parse `compliance/PHYSICS_INVARIANTS.json` and extract `ci_test_directives`.
2. For each directive with severity `ERROR` or `CRITICAL`, run the regex against all `.rs` files.
3. Exclude matches inside comments (lines starting with `//` or within `/* */` blocks).
4. Fail the GitHub Actions job if any violation is found.
5. Produce a GitHub annotation on the offending line.
6. Run on every push and every pull request to `main`.

#### A.3 — CI Pipeline: `ci.yml` Enhancement

Audit the existing `.github/workflows/ci.yml` and ensure it includes:

```yaml
jobs:
  check:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]  # AuraFS must build on both
    steps:
      - cargo fmt --check
      - cargo clippy -- -D warnings
      - cargo test --all-features
      - cargo doc --no-deps
```

If the existing `ci.yml` is missing any of these steps, the plan must specify the exact additions.

#### A.4 — README.md Polish

The plan must include a complete revised README draft with:
- **Tagline:** "Physics-informed distributed storage powered by fractal geometry and topological protection."
- **Badges:** CI status, license, crates.io (future), docs.rs (future)
- **30-second pitch:** What is AuraFS? Why fractal geometry for storage? (3 paragraphs max)
- **Quick start:** Clone → build → run instructions for both Windows and Linux
- **Architecture section:** Text-based diagram of the Chakra DataCore topology
- **Compliance section:** Link to `compliance/` directory with explanation for DARPA/academic reviewers
- **Thesis reference:** Link to arXiv (placeholder until published)
- **Contributing / License / Security links**

#### A.5 — Acceptance Criteria for Phase A

- [ ] `cargo check` passes on both Ubuntu and Windows CI runners
- [ ] `physics_audit.yml` catches re-introduced magic numbers (test by intentionally inserting one)
- [ ] README renders on GitHub with all badges and no broken links
- [ ] `compliance/` directory prominently linked from README
- [ ] CODEOWNERS file correctly routes `physics/` and `crypto/` PRs

---

### BUILD PHASE B: Test Coverage & Physics Regression Suite

**Goal:** Every physics invariant has a dedicated regression test. Achieve ≥80% line coverage on `physics/` and `core/`.
**Estimated effort:** 2–3 days
**Dependencies:** Phase A complete (CI running).

#### B.1 — Physics Test Suite

Create tests in `tests/physics/` (or `src/physics/` as inline `#[cfg(test)]` modules):

| Test | Validates | Expected Behavior |
|---|---|---|
| `test_scaling_replicas` | `compute_replicas()` | N=1→1, N=5→2, N=12→2, N=42→2, N=100→3, N=1000→5 (ceil(log_{5.3})) |
| `test_scaling_advantage` | Advantage table matches VALIDATION_REPORT | N=100,D=5 → advantage=5.3× |
| `test_coherence_window_enforcement` | PassiveCoherence tick < T₂ | 1601μs tick → `CoherenceWindowExceeded` |
| `test_spectral_health_pass` | `check_spectral_health(1.37)` | Ok(()) |
| `test_spectral_health_fail` | `check_spectral_health(1.50)` | Err(DecoherenceThreshold) |
| `test_spectral_boundary` | `check_spectral_health(1.42)` | Ok (within tolerance) |
| `test_spectral_boundary_fail` | `check_spectral_health(1.421)` | Err (outside tolerance) |
| `test_pbg_routing_capacity` | `(1.0 - PBG) * 1000.0` | ≈ 790.0 (within f64 epsilon) |
| `test_invariant_golden_file` | Load `PHYSICS_INVARIANTS.json`, compare to `INVARIANTS` | All values match within tolerance |
| `test_invariant_validation_panics` | Construct out-of-range invariants | `#[should_panic]` |

#### B.2 — Crypto Test Suite

| Test | Validates |
|---|---|
| `test_dilithium_sign_verify` | Round-trip sign/verify with Dilithium-5 |
| `test_dilithium_tampered_signature` | Tampered message fails verification |
| `test_dilithium_key_sizes` | Public key = 2,592 bytes, Signature = 4,627 bytes (FIPS 204) |
| `test_merkle_inclusion_proof` | Build tree over 100 shards, verify inclusion, tamper → fail |
| `test_sha3_256_hash` | Hash output matches known test vector |

#### B.3 — Integration Tests

| Test | Validates |
|---|---|
| `test_shard_lifecycle` | Void-Shard create → fractal replicate (check replica count = `compute_replicas(N)`) → Aura-Shard promote → verify within coherence window |
| `test_decoherence_recovery` | Inject d_s=1.50 → assert recovery triggers → assert writes freeze → assert d_s restored within tolerance |
| `test_coherence_state_transitions` | `Strict` → `DecoherenceExempt` → `Redistributing` → `Strict` |

#### B.4 — Coverage Thresholds

Add `cargo-llvm-cov` or `cargo-tarpaulin` to CI. Minimum thresholds:

| Module | Minimum Coverage |
|---|---|
| `physics/` | 90% |
| `core/` | 80% |
| `crypto/pqc/` | 85% |
| `fuse/` | 70% |
| `network/` | 60% |
| Overall | 70% |

#### B.5 — Acceptance Criteria for Phase B

- [ ] `cargo test --all` passes with zero failures
- [ ] Coverage meets or exceeds all thresholds
- [ ] Golden file test validates `PHYSICS_INVARIANTS.json` against runtime `INVARIANTS`
- [ ] Every CI directive (PHYS-001 through SEC-001) has a corresponding test assertion

---

### BUILD PHASE C: Documentation & Public API Surface

**Goal:** External developers and DARPA reviewers can understand the system without reading every source file.
**Estimated effort:** 1–2 days
**Dependencies:** Phase B complete (tests passing).

#### C.1 — `docs/architecture.md`

System architecture document covering:
- Chakra DataCore topology diagram (7 processing zones)
- Shard lifecycle: Void-Shard → Trap-State → Aura-Shard
- Network triple-topology: Titan-Libp2p / GhostLink-LoRaWAN / Starlink
- Physics engine integration points (where `INVARIANTS` is called in the hot path)
- Security model: Dilithium-5 signatures, SHA-3-256 Merkle trees, Byzantine 3f+1 quorum

#### C.2 — `docs/physics_primer.md`

Non-expert explanation of the five invariants, written for a DARPA program manager or open-source contributor who is not a physicist. Include the "why does this matter for storage" framing for each constant.

#### C.3 — Rustdoc Polish

Ensure `cargo doc --no-deps` produces clean documentation:
- All `pub` items have `///` doc comments
- `physics::INVARIANTS` and `PhysicsViolationError` have comprehensive examples
- No broken intra-doc links
- Module-level docs for each subfolder explain its role in the architecture

#### C.4 — API Surface Audit

Review all `pub` exports in `src/lib.rs`. The plan must:
- List every currently exported symbol
- Recommend which should remain public vs. become `pub(crate)`
- Flag any accidental exposure of internal types

#### C.5 — Acceptance Criteria for Phase C

- [ ] `docs/architecture.md` and `docs/physics_primer.md` exist and render in GitHub
- [ ] `cargo doc --no-deps` produces zero warnings
- [ ] All `pub` functions in `physics/` and `core/` have doc comments with examples

---

### BUILD PHASE D: Security Hardening & Compliance Closure

**Goal:** Close all open items from `SECURITY_AUDIT.md` recommendations R1–R6.
**Estimated effort:** 3–5 days
**Dependencies:** Phase C complete.

#### D.1 — Close R1 (CRITICAL): Kyber-1024 Integration

Complete the post-quantum key encapsulation implementation in `src/crypto/pqc/kyber_kem.rs`. Replace the X25519 hybrid placeholder with full FIPS 203 ML-KEM-1024.

#### D.2 — Close R2 (CRITICAL): Constant-Time Dilithium

Verify `src/crypto/pqc/dilithium_sig.rs` uses constant-time operations. If using a third-party crate, confirm the crate's constant-time guarantees. If hand-rolled, add `subtle` crate for constant-time comparisons.

#### D.3 — Close R3 (CRITICAL): Key Management Policy

Create `SECURITY.md` addendum or standalone `docs/key_management.md` specifying rotation intervals, revocation procedures, and escrow policy.

#### D.4 — Address R4–R6 (IMPORTANT)

- R4: HSM support — document the integration path for FIPS 140-3 Level 3 modules.
- R5: Third-party audit — identify budget and timeline for Trail of Bits / NCC Group engagement.
- R6: Hybrid signatures — implement Dilithium-5 + Ed25519 dual-signing in `src/crypto/signature.rs`.

#### D.5 — Acceptance Criteria for Phase D

- [ ] Kyber-1024 KEM functional (sign/encapsulate/decapsulate round-trip test passing)
- [ ] Dilithium constant-time verification documented or demonstrated
- [ ] Key management policy document exists
- [ ] `SECURITY_AUDIT.md` recommendations table updated with status

---

### BUILD PHASE E: Release Packaging & Domain Launch

**Goal:** First public release at `github.com/aurphyx/aurafs` and domain `aurafs.dev` live.
**Estimated effort:** 1–2 days
**Dependencies:** Phases A–D complete.

#### E.1 — Version Tagging

- Tag: `v0.4.0-phase2-hardening` (matches `aurafs.toml` version)
- Generate `CHANGELOG.md` entry from all Phase 1/2 work
- Create GitHub Release with pre-built binaries (Linux amd64, Windows x86_64)

#### E.2 — Docker Image

- Verify `docker/Dockerfile.api` multi-stage build produces a minimal image
- Push to GHCR: `ghcr.io/aurphyx/aurafs:0.4.0`
- Ensure Helm `values.yaml` image tag matches

#### E.3 — `aurafs.dev` Domain

Plan for the landing page:
- Static site (GitHub Pages or Cloudflare Pages)
- Content: README pitch, architecture diagram, link to GitHub, link to compliance docs, link to thesis (arXiv)
- Subdomain `docs.aurafs.dev` pointing to Rustdoc output

#### E.4 — crates.io Preparation (Optional/Future)

- Audit `Cargo.toml` for `[package]` metadata completeness
- Ensure `license`, `description`, `repository`, `homepage`, `documentation` fields are set
- Run `cargo publish --dry-run` to catch issues

#### E.5 — Acceptance Criteria for Phase E

- [ ] GitHub Release `v0.4.0-phase2-hardening` published
- [ ] Docker image builds and runs on both amd64 and arm64
- [ ] `aurafs.dev` resolves and displays the landing page
- [ ] All CI workflows green on the tagged release commit

---

### BUILD PHASE F: Ongoing — Post-Launch Maintenance

This section is not actionable in the current sprint but documents the operational cadence:

- **Weekly:** Review `physics_audit.yml` results on `main` branch
- **Per-PR:** CODEOWNERS enforces review for `physics/` and `crypto/` changes
- **Monthly:** Re-run `PHYSICS_INVARIANTS.json` golden file test against latest `aurafs.toml`
- **Quarterly:** Update `DARPA_TRL_VALIDATION.md` with new experimental results
- **On-demand:** Any change to `[physics]` constants in `aurafs.toml` requires a governance vote (min_quorum=13) and a `PHYSICS OVERRIDE` commit message

---

## EXECUTION SUMMARY

| Phase | Deliverable | Gate |
|---|---|---|
| **0** | `physics::INVARIANTS` module + module index | Compiles, all 5 constants load |
| **1** | FIX_UPDATE_BUILD.md per subfolder + PHASE1_AUDIT_SUMMARY.md | `cargo check` passes, zero unclassified matches |
| **2** | DEPLOYMENT_BUILD_PLAN.md | Document reviewed and approved by Ross |
| **A** | CI/CD + README + repo hygiene | CI green on both OS targets |
| **B** | Test suite + coverage | ≥80% physics/, zero test failures |
| **C** | Docs + Rustdoc + API audit | `cargo doc` clean, architecture.md exists |
| **D** | Security closure (R1–R6) | Kyber-1024 functional, key mgmt documented |
| **E** | Release v0.4.0 + domain launch | GitHub Release + aurafs.dev live |

---

**END OF PROMPT — Begin Phase 0 now.**
