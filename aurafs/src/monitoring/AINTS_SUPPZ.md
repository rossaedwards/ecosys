# AINTS_SUPPZ — `aurafs/src/monitoring/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Love-signed metrics + health. `mod.rs` (~431) **redefines** metrics/health **and** an inline `alerting` module, then `pub use`s `metrics`, `health`, `alerting`. Sibling files `metrics.rs` / `health.rs` also exist — **duplicate types** vs `main.rs` constructor signatures.

`mod.rs` declares `mod alerting` and `mod tests` — no `alerting.rs` / `tests.rs` on disk (alerting is inline in `mod.rs` if written that way; verify before compile).

## Wired?

**Declared.** `main.rs` builds `MetricsCollector` / `HealthChecker` with arguments that match `health.rs` more than the zero-arg `mod.rs` inline types.

## Relation to welcome

Observability only. Not SAGES governance field.

## Honest status

Split implementations. Prometheus feature is optional. Not a production SRE stack (`network/monitoring/` Grafana/Prometheus files are 14-line stubs).


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
