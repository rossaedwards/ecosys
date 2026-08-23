# AINTS_SUPPZ — `aurafs/src/storage/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Storage HAL + in-tree FS: filesystem, inode, directory, journal, quota, snapshot, local, shard_store, fuse, plus a second `shardstore.rs` (not declared in `mod.rs`).

Love + Quantum Division. `directory.rs` / `journal.rs` also TRL-4 tagged.

`mod.rs` imports `crate::shard_server::acl::AclEnforcer` — **`shard_server` is not `mod`’d from `lib.rs`.** That makes this module fail to compile in the library as declared.

## Wired?

**Declared** in `lib.rs`. Runtime path is blocked by the `shard_server` import unless the crate is restructured.

Duplicate FUSE lives in `src/fuse/` and `storage/fuse.rs`. Duplicate snapshot types vs `src/snapshot/`.

## Locks

Do not treat storage ceilings as physics: `max_shard_bytes` etc. live in `aurafs.toml` `[storage]`.

## Relation to welcome

This is the “file system + storage” half of AuraFS. Mesh is `network/` + `mesh/`. Xplor (`xpl0r`) should eventually browse this — still **design** in g0dm0d3.

## Honest status

Substantial code, not production-wired. Fix the `shard_server` dependency or `mod shard_server` before claiming a HAL. Do not delete `shardstore.rs` without checking callers.


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
