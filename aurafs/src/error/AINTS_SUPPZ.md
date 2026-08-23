# AINTS_SUPPZ — `aurafs/src/error/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

This **directory** contains `context.rs` (~158 LOC, Love-signed contextual errors) and `ENTERPRISE_IMPROVEMENTS.md`. **No `error/mod.rs`.**

Crate-level `pub mod error` is **`src/error.rs`**: Phase II `RafsError` / `RafsResult`, includes `PhysicsViolation`. Re-exported from `lib.rs`.

`core/error.rs` is a **third** error taxonomy (`AuraFSError`, etc.).

## Wired?

`src/error.rs` is wired. `error/context.rs` is not a crate module unless pulled in elsewhere.

## Locks

None in `[modules.validated]`. Keep physics errors delegated to `PhysicsViolationError`.

## Honest status

Error types are fragmented across `error.rs`, `error/`, and `core/error.rs`. Do not invent a unified crate without Ross. Do not treat ENTERPRISE_IMPROVEMENTS as implemented.


## Nomenclature (new prose only)

Quote retired names, then map: SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Bliss manifold (physics) → Equilibrium Manifold; rÆ → Balance State Vector; Vibe-OKF → APS-OKF; V.A.P. → VASP; Accessibility → Xessability.  
**Product names that stay:** BlissCore / ChaosCore (Chakra Duality Kernel). In-tree `BlissID` types are legacy identity code — flag, do not silently rewrite.

## What not to do

- Do not stamp ecosys APS-OKF YAML onto this file or this folder's Rust.
- Do not apply AuraFS replica-count / PBG / η law as FTQC theory.
- Do not invent missing root files (`INVARIANTS.md`, `aps.toml`, codices).
- Do not create parallel trees or `aps_*` duplicates.
- One folder pass: edits belong under `aurafs/` only.

---
*Audry briefing. Faithful to source on disk. Updated 2026-08-23.*
