# AINTS_REVIEW — `aurafs/src/error/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `context.rs` extra context types.
- Crate law is `src/error.rs` (`RafsError`, physics-integrated).

**Wired:** Folder is **not** crate `src/error.rs`. `error/context.rs` + ENTERPRISE notes.  
**Era / voice:** Love notes vs TRL-4 `src/error.rs`.

## Suggestions

- One error taxonomy. Fold context into `src/error.rs` or stop shipping a second module.
- `PhysicsViolationError` stays in `physics/`.

## Clarifying questions needed

### Architectural
- `RafsError` vs `core/error.rs` vs this folder?

### Framework
- N/A.

### Vision
- Should physics violations be operator-visible in Xplor?

### Intentions
- thiserror + anyhow prelude already in `lib.rs` — keep it.

### Design
- Do not `pub mod error` the folder if `error.rs` exists (Rust conflict).

### Ideas
- Map error classes to SIX (I/O), SCX (invariant), ICX (identity) for support UX only — not a contraction.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Do not encode HIF into error enums. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | N/A. |
| **Fuxyez** (`fuxyez/`) | Fuxyez AuraFsError should wrap `RafsError`, not a fourth enum. |


## Nomenclature (new prose)

Quote then map: SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Bliss manifold (physics) → Equilibrium Manifold; rÆ → Balance State Vector; Vibe-OKF → APS-OKF; V.A.P. → VASP; Accessibility → Xessability.  
**Keep as product names:** BlissCore / ChaosCore. In-tree `BlissID` is legacy — flag, do not silently rewrite.

## What not to do

- Do not stamp ecosys APS-OKF YAML onto AuraFS Rust.
- Do not apply AuraFS η / PBG / replica law as FTQC Hilbert scaling.
- Do not merge TVFD \(\mathcal{{F}}\) with TSLCA \(\mathcal{{F}}\).
- Do not invent missing root files (`INVARIANTS.md`, `aps.toml`, codices).
- Do not implement exploit payloads or complete redteam stubs.

---
*Audry. Faithful to disk. 2026-08-23.*
