# AINTS_SUPPZ — `aurafs/src/fuse/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Host VFS “Portal”: node, session, timed_lock (wired from `mod.rs`). Also `filesystem.rs`, `mount.rs`, `fusemain.rs`, `inode_cache.rs`, **`main.rs`** (extra entry — not the crate bin). Love + QDiv; `mod.rs` / `session.rs` TRL-4 tagged.

`timed_lock.rs` should honor `lock_acquisition_timeout_us` (100 μs = T₂/16).

## Wired?

**Yes** — `pub mod fuse`. `main.rs` imports `fuse::AuraFSFuse` — that type may live in `storage/fuse.rs` instead. Name mismatch risk.

Cargo feature `fuse` + optional `fuser`. Dokany/Windows mentioned in README; unix `nix`/`libc` in Cargo.

## Locks

FUSE timeout is derived physics. Coverage target 70% in `aurafs.toml` — no `tests/` Rust suite in this copy.

## Relation to welcome

This is how Aura (personal OS) and later Arora/Biznyx would mount shards. Not Xplor UI.

## Honest status

Multiple FUSE implementations (`fuse/`, `storage/fuse.rs`). Crate bin does not start FUSE despite `src/main.rs` comments (“Starts FUSE…”) — the shown `main` starts API only and never mounts.


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
