# AINTS_SUPPZ — `aurafs/src/cache/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Love-signed LRU / tiered cache (`lru.rs`, `mod.rs` with in-file types). `ENTERPRISE_IMPROVEMENTS.md` wishlist.

`lib.rs` comment: “LRU Trap-State Monitor (1600μs coherence-aware caching).” Product law: Trap-State is **not** a generic cache rename. This folder still names itself Cache / LRU.

## Wired?

**Yes** as `pub mod cache`. `main.rs` uses `CacheManager` — confirm the type exists on this module before assuming it compiles.

## Locks

Coherence window must come from `INVARIANTS`, not a bare `1600`. `CacheManager::new(100)` in `main.rs` is a **capacity** (MB comment), not η — still review for physics-audit false positives.

## Relation to welcome

Internal performance layer. Not Memoree.

## Honest status

Real LRU code + docs wishlist. Naming vs Trap-State is a law tension — do not “fix” by renaming Trap-State to cache.


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
