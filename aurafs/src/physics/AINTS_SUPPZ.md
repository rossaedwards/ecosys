# AINTS_SUPPZ — `aurafs/src/physics/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Product **physics governance** for AuraFS replica / coherence law. This is **not** the FTQC volume and not root `PHYSICS.md`.

Single file: `mod.rs` (~93 LOC). Loads `[physics]` from `aurafs.toml` into `lazy_static` `INVARIANTS`. Exposes `PhysicsInvariants`, `PhysicsViolationError`, `calculate_replicas` (`ceil(log_η(N))`), `is_ds_stable`, `is_within_coherence_window`, `DecoherenceRecovery`.

## Wired?

**Yes.** `src/lib.rs` `pub mod physics`. Consumed by prelude, `config.rs`, `gov/sages.rs`, FUSE lock path, and anything that must not hardcode η / T₂ / d_s / PBG / 100 μs.

## Era / voice

Phase II / TRL-4 header. No Love-signature on this file. Authoritative product voice.

## Locks

`aurafs.toml` `[modules.validated].core` lists `src/physics/mod.rs` **and** `src/physics/invariants.rs`. **`invariants.rs` is missing.** Do not invent it unless Ross asks; the singleton lives in `mod.rs`. Public API changes need `PHYSICS OVERRIDE` + quorum 13.

Access constants **only** via `physics::INVARIANTS`. Hardcoding `5.3`, `1600`, `1.37`, `0.21`, `100` as those constants is a product compliance violation.

AuraFS d_s clamp is **1.37** (Rammal–Toulouse \(2\log 3/\log 5\)). Root `PHYSICS.md` lists d_s = 1.36 via a different formula. **Do not silently unify.**

## Relation to welcome

Welcome: AuraFS is decentralized, off-grid, photonic, topological storage + mesh. These five numbers are the **product** engine for replica count and Meshwerk routing overhead — not a reprint of TSLCA \(\mathcal{F}\) or SUXS-IFO \(\mathcal{U}\).

## Honest status

Small, central, and actually used. Load path `fs::read_to_string("aurafs.toml")` is CWD-relative (fragile for installed binaries). No dedicated `tests/physics/` tree in this repo copy (coverage targets exist in `aurafs.toml` `[ci.coverage]`).


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
