# AINTS_SUPPZ — `aurafs/src/cli/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Love + QDiv: `banner.rs`, `aurafs_cli.rs`, `admin.rs`, `user.rs`, `tui.rs`. `mod.rs` Love-only.

Commented TODOs: `core_commands`, `ai_cli`, `quantum_cli`, `game_cli`, `security_cli`, `enterprise_cli` — **not implemented**.

## Wired?

**Yes** as `pub mod cli`. Crate binary is **not** this clap app: `Cargo.toml` `[[bin]]` points at **missing** `src/bin/aurafs.rs`; actual `src/main.rs` does not use `cli`.

## Relation to welcome

Closest in-tree sketch to `afs-4dm1n` / disk admin — still not Audry. Casino/Aethornyx `game_cli` is explicitly unimplemented.

## Honest status

CLI code exists and is unused by the declared binary. Banner voice is Love-era; keep it; do not shame.


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
