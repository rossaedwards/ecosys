# AINTS_SUPPZ — `aurafs/src/shard/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

**Shard lifecycle types** as a dedicated module (distinct from `core/shard.rs`): id, metadata, data, storage, index, audit. Love + Quantum Division; several TRL-4 tagged (`data`, `metadata`, `audit`). `review_suggestion.md` dated 2026.

`mod.rs` claims Void/Trap/Aura orchestration (“Bio-Resonant Reality Forge”).

## Wired?

**Yes** — `pub mod shard`. `storage/mod.rs` imports `crate::shard::{Shard, ShardId, ...}`.

## Locks

Lifecycle names are product law (`aurafs.toml` `[storage]`): **Void-Shard → Trap-State → Aura-Shard**. Do not rename Trap-State to “cache.”

`core/shard.rs` is the **locked** shard file in `[modules.validated]`, not these files. Do not assume this folder is TRL-4 locked except where headers say Phase II.

## Relation to welcome

Shards are the AuraFS atom. Welcome/ecosystem: Aethornyx and Aurphyx Casino may use shards as in-game currency — **design**; do not invent ledger rules here.

## Honest status

Large, dual with `core/shard.rs`. Type-name collisions are likely compile blockers. `mod.rs` declares `tests` submodule — no `tests.rs` seen in this folder listing.


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
