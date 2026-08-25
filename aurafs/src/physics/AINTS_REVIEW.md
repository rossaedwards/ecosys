# AINTS_REVIEW — `aurafs/src/physics/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `mod.rs` loads `[physics]` from `aurafs.toml` into `INVARIANTS`.
- `calculate_replicas`, d_s stability, coherence window, `DecoherenceRecovery`.
- toml also lists **missing** `invariants.rs`.

**Wired:** Yes. Crate singleton. Consumed by prelude/config/gov/FUSE lock path.  
**Era / voice:** Phase II / TRL-4. No Love banner. Authoritative product voice.

## Suggestions

- Ross decides: split `invariants.rs` vs document that singleton lives in `mod.rs`.
- CWD-relative toml load is fragile for installed bins.
- Do not silently change 1.37 to root 1.36.

## Clarifying questions needed

### Architectural
- Is this the only legal source of η, T₂, d_s, PBG, 100 μs?

### Framework
- FTQC owns D_f / Hilbert scaling — cite, do not fork.
- AuraFS replica law must not be imported as FTQC theory.

### Vision
- Photonic substrate: are these lab locks or shipping locks?

### Intentions
- PHYSICS OVERRIDE + quorum 13 — who is the quorum in lab?

### Design
- Keep access `physics::INVARIANTS` only.

### Ideas
- Fail closed if toml missing rather than implicit defaults (confirm).

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Not \(\mathcal{F}\) / \(\mathcal{U}\) / HIF. |
| **FTQC** (`ftqc/`) | Cite D_f = log3/log2 from root/ftqc. Replica η is AuraFS-only. |
| **TVFD** (`tvfd/`) | Z_vac is TVFD/root, not a replica input. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Override path is governance, not a new constant. |
| **Fuxyez** (`fuxyez/`) | Runtime must not hardcode 5.3 / 1600. |


## Nomenclature (new prose)

Quote then map: SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Bliss manifold (physics) → Equilibrium Manifold; rÆ → Balance State Vector; Vibe-OKF → APS-OKF; V.A.P. → VASP; Accessibility → Xessability.  
**Keep as product names:** BlissCore / ChaosCore. In-tree `BlissID` is legacy — flag, do not silently rewrite.

## What not to do

- Do not stamp ecosys APS-OKF YAML onto AuraFS Rust.
- Do not apply AuraFS η / PBG / replica law as FTQC Hilbert scaling.
- Do not merge TVFD \(\mathcal{{F}}\) with TSLCA \(\mathcal{{F}}\).
- Do not invent missing root files (`INVARIANTS.md`, `aps.toml`, codices).
- Do not implement exploit payloads or complete redteam stubs.

---
*Audry. Faithful to disk. 2026-08-23.*
