# AINTS_SUPPZ — `aurafs/src/mesh/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Second mesh stack (alongside `network/`): FractalNode, Chord DHT, XOR distance, gossip, protocol, routing, swarm/circuit breaker. Love + Quantum Division. `gossip.rs` is Love-only and large (~585).

`mod.rs` `xor_distance` uses `crate::gov::BlissId` — legacy identity type.

Declares `full` and `tests` — **not in the file listing.**

## Wired?

**Yes** — `pub mod mesh` (separate from `network`). Duplicate gossip/routing with `network/`.

## Locks

None listed under this path. Meshwerk locks are under `network/meshwerk/`.

## Relation to welcome

Mesh network half of AuraFS. Keep Titan / GhostLink / Starlink names from `aurafs.toml`, not new transport brands.

## Honest status

Parallel implementation. Agents must not “merge” mesh + network without a dedicated pass. Missing `full`/`tests` is a compile risk if declared.


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
