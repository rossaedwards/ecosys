# AINTS_SUPPZ — `aurafs/src/api/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

`server.rs` (~227, Love) `ApiServer` / `AppState`. `websockets.rs` (~147, Love). **`mod.rs` is empty.**

## Wired?

`pub mod api` is **empty at the root**. `main.rs` imports `api::{ApiServer, AppState}` — **will not resolve** until `mod.rs` exports those types.

`lib.rs` comments “API server and WebSocket AuraCore Hub.” Hub is not a separate crate.

## Relation to welcome

g0dm0d3-ktrl spec wants REST/gRPC to AuraFS — **deferred** (`g0dm0d3-ktrl/CLAUDE_SUGGZ.md`). This HTTP sketch is the natural attach point, currently unwired.

## Honest status

Implementation files exist; the module root is a stub. Bind address `0.0.0.0:8080` in `main.rs` is not production-ready.


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
