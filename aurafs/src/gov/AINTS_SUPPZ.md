# AINTS_SUPPZ — `aurafs/src/gov/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Product-side **governance**: SoulSync engine, voting, proposals, policy, identity verifier, audit log, FastAPI-shaped server stubs, `blissid_manager.rs`, and `sages.rs` (13 sentinel **roles** as an in-crate enum).

15 files, mostly Love-signed; several also Phase II / TRL-4 (`sages.rs`, `voting_engine.rs`, `soulsync_engine.rs`, `policy_enforcer.rs`, `models.rs`, `transaction_type.rs`). `server.rs` is dual Love + Quantum Division + TRL-4.

## Wired?

**Yes** — `pub mod gov`. `aurafs.toml` locks `src/gov/sages.rs`.

## Locks

`sages.rs` public API is TRL-4 locked. It monitors d_s / physics violations and Dilithium via `crypto::pqc::dilithium_sig`. It is **not** the 13 guardian crates.

## SAGES name mismatch (flag, do not “fix” here)

`gov/sages.rs` roles: Vyrellix, Archivus, Sentry, AuraLord, Aegis, Chrona, Heliox, Nexus, Ordo, Paxia, Quanta, Solaris, Umbra.

`SAGES_Framework/` actually has crates such as: archivus, bliss_engine, chaos_engine, cryptanyx, nullivar, nunclex, ophiuchus, praelum, prophetyx, umbryx, valkryx, plus orricshade-core, vyrelix-core.

Do not invent a mapping. Do not rename sentinels in this pass.

## Relation to welcome

Welcome: SAGES protect kernel through meshwrk; GVS is global voting. This folder is AuraFS’s **in-process** hook (SoulSync / votes / proposals). GVS lives at `gvs/` in ecosys — not implemented here.

`fuxyez/governance/` and `SAGES_Framework/governance/` look like **sibling copies** of the same governance file set. Do not merge them from AuraFS.

## Honest status

Real volume of code; identity still `BlissID`. Not wired to SAGES_Framework binaries. Quorum 13 is product law (`aurafs.toml` `[governance]`), not proof these engines run in production.


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
