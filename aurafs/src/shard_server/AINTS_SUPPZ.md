# AINTS_SUPPZ — `aurafs/src/shard_server/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Love + QDiv “enterprise orchestrator”: server, ACL, autoheal, IPFS + ipfs_cluster, gRPC, mesh, gossip, CLI, **`main.rs`** (would-be binary). Substantial LOC (server ~592, cli ~490).

## Wired?

**No** `mod` from `lib.rs`. **But** `storage/mod.rs` already `use crate::shard_server::...` — so the library **depends on an undeclared module**.

`Cargo.toml` has no `[[bin]]` for shard_server. Extra `main.rs` here is not the crate entry.

## Relation to welcome

IPFS is not named in welcome as AuraFS transport. Product transports are Titan / GhostLink / Starlink. Treat IPFS as an experimental sidecar, not law.

## Honest status

Fat, unwired, and a compile landmine via `storage`. Do not add a bin without Ross. Do not treat gRPC/IPFS as production mesh.


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
