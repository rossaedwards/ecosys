# AINTS_SUPPZ — `aurafs/src/core/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

μkernel-style **types and enterprise patterns** claimed as AuraFS core: errors, shard primitives, soulproof, metrics, identity (`bliss.rs` — `BlissId*`), crypto helpers, config, health, circuit breaker, rate limiter, merkle, tracing, network orchestration, persistence.

On disk (18 files): substantial Rust (`merkle.rs` ~653, `persistence.rs` ~740, `shard.rs` ~609, `bliss.rs` ~606, etc.) plus `ENTERPRISE_IMPROVEMENTS.md`, `review_suggestion.md` (2026 notes). **`lattice.rs` is empty** (~2 LOC).

`mod.rs` does **not** declare `lattice`.

## Wired?

**Yes** as `pub mod core` from `lib.rs`. Overlaps `src/shard/`, `src/crypto/`, `src/error.rs`, `src/network/` — duplicate concepts, not a single type system.

## Era / voice

Almost all Love-signed **and** Aurphyx Quantum Division. `core/shard.rs` also tagged Phase II / TRL-4. `review_suggestion.md` is 2026 maintainer notes.

## Locks

`aurafs.toml` locks `src/core/shard.rs` and `src/core/merkle.rs`. Do not change public APIs without `PHYSICS OVERRIDE`.

Identity here is **legacy `BlissID`**. New prose: SoulKey → SKIM → SIR → SIG; one soul, one account, one vote. Do not rewrite `bliss.rs` in a header pass.

## Relation to welcome

Soulproof / identity hooks sit next to USIS / SoulSync (welcome pipeline). This folder is **not** Memoree and **not** SAGES_Framework crates.

## Honest status

Largest “implemented” type dump after network/crypto. Compilability is unproven in this pass (overlapping `Shard` types vs `src/shard/`). Empty `lattice.rs` is a stub. Treat `ENTERPRISE_IMPROVEMENTS.md` as wishlist, not runtime truth.


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
