# AINTS_SUPPZ — `aurafs/src/acl/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Love-signed RBAC/ACL: `mod.rs`, `manager.rs` (~267), `acl_config.json`, `ENTERPRISE_IMPROVEMENTS.md`.

## Wired?

**Yes** — `pub mod acl`. `main.rs` uses `AclManager`.

## Locks

None in validated list. Quantum-safe / ZK claims in the module docs are **aspirational** unless `manager.rs` actually verifies Dilithium.

## Relation to welcome

SoulKey = 1 human, 1 soul, 1 account, 1 vote. ACL should eventually bind to that — today it is path/owner RBAC. README still says “SoulSync/BlissID.”

## Honest status

Usable-looking manager + JSON. Not USIS. Separate ACL also exists in `shard_server/acl.rs` (unwired crate).


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
