# AINTS_SUPPZ — `aurafs/src/snapshot/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

`manager.rs` (~240 LOC, Love-signed `SnapshotManager` create/list/rollback). **`mod.rs` is empty.**

`lib.rs` declares `pub mod snapshot`. An empty `mod.rs` means the crate module compiles as **empty** — `manager.rs` is **not automatically included**.

`src/main.rs` imports `SnapshotManager` from the crate root. `lib.rs` does **not** re-export it. `storage/snapshot.rs` is a third snapshot type.

## Wired?

Declared, effectively a **stub at the module root**. Main binary types do not match `lib.rs` re-exports.

## Locks

None in `[modules.validated]`.

## Relation to welcome

SoulShot is a **universe snapshot at first breath** (welcome) — not this filesystem snapshot manager. Do not conflate.

## Honest status

Empty `mod.rs` is the truth of the declared module. Wire `mod manager` + re-exports before using from `main.rs`.


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
