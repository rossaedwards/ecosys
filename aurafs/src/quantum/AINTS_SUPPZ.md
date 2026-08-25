# AINTS_SUPPZ — `aurafs/src/quantum/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

**One file:** `mod.rs` (~278, Love-signed). Inline modules `pqc_bridge`, `entanglement_sim`, `qrng` (not separate files). Kyber-shaped **placeholder** key exchange (Vec<u8>, not `pqcrypto-kyber`). Entanglement/QRNG are simulations.

`mod.rs` header also `pub use`s those inline modules and has `init()`.

## Wired?

**Declared** in `lib.rs`. Not used by `main.rs`.

## Locks

Do not present this as Majorana-1 or FTQC. `lib.rs` comment “Majorana-1 simulation” is a hook name only. FTQC lives in `ftqc/`.

## Honest status

Educational/sim stubs. Real PQC path is `crypto/pqc/dilithium_sig.rs` + future Kyber (R1).


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
