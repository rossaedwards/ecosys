# AINTS_SUPPZ — `aurafs/src/config/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

This **directory** contains only `manager.rs` (~229 LOC, Love-signed hot-reload manager). **There is no `config/mod.rs`.**

Crate-level `pub mod config` in `lib.rs` resolves to **`src/config.rs`** (Phase II / TRL-4, `AuraConfig` + physics defaults from `INVARIANTS`), not this folder.

## Wired?

`src/config.rs` is the wired module. `config/manager.rs` is an **orphaned sibling** unless something `include!`s it (nothing does from `lib.rs`).

## Locks

Config that changes `[physics]` defaults must stay aligned with `aurafs.toml`. Do not hardcode η/T₂/d_s/PBG.

## Relation to welcome

Audry `afs-4dm1n` / settings are **design**. This is crate config, not Audry admin.

## Honest status

Split-brain: two config implementations. Agents must edit `src/config.rs` for the declared module; do not assume `config/` is `mod config`.


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
