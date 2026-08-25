# AINTS_SUPPZ — `aurafs/src/redteam/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Offensive-testing **name matrix** (authorized-testing comment in `mod.rs`). Subdirs: chaos, exploit, net, gov, audit_simulator, quantum_breaker, fuzzers, plus `cli.rs`.

**Most files are ~14-line stubs.** A few Love+QDiv files have real LOC: `chaos/node_killer.rs` (~352), `exploit/mesh_partition.rs` (~348), fuzzers (`namespace_fuzzer`, `shard_fuzzer`, `soul_fuzzer`), `quantum_breaker/dilithium_forge.rs`, `entropy_starver.rs`, `kyber_cracker.rs`, `cli.rs`. Inventory: `afs-src-redteam_current_repo_12-28-25.txt`.

`pinktribesuite.md` **missing** at `aurafs/src/pinktribesuite.md`.

## Wired?

**No.** Not in `lib.rs`. No `security-tools` feature in Cargo.toml despite the module comment.

## What not to touch

Do **not** rewrite redteam source. Do not complete stubs, PoCs, or attack procedures. This briefing names folders only. Filenames are not a request to build those capabilities.

`mod.rs` `run_test_suite()` returns zeros (placeholder).

## Relation to welcome

Not SAGES. Not Pink Tribe suite (file absent). Keep isolated from production `lib.rs` until Ross explicitly wants a **defensive-test** feature that checks secure behavior without exploit payloads.

## Honest status

Mostly stubs + a handful of large named files. Treat as unshipped. Do not enable from the default crate.


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
