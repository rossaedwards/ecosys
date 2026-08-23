# AINTS_REVIEW — `aurafs/src/config/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `config/manager.rs` duplicate taxonomy vs `src/config.rs`.

**Wired:** Folder `config/` is **not** `src/config.rs`. Crate uses `src/config.rs` (Phase II).  
**Era / voice:** Folder: Love `manager.rs`. Crate file: TRL-4.

## Suggestions

- Declare one ConfigManager. Fold or delete the unused path.
- Hot-reload must go through `physics::INVARIANTS` + `aurafs.toml`.

## Clarifying questions needed

### Architectural
- Which file is law: `src/config.rs` or `config/manager.rs`?

### Framework
- Product `[physics]` vs root `PHYSICS.md` d_s 1.36 vs AuraFS 1.37 — do not unify here.

### Vision
- Installed binary: where does `aurafs.toml` live? CWD load is fragile.

### Intentions
- Hot-reload: operator feature or lab-only?

### Design
- Single `RafsConfig` as `lib.rs` already re-exports.

### Ideas
- Schema version field so 2025 Love configs fail closed.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | N/A. |
| **FTQC** (`ftqc/`) | Do not put dim(H_acc) in AuraFS config. |
| **TVFD** (`tvfd/`) | Z_vac is root/TVFD, not a storage knob. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Governance quorum 13 lives in toml `[governance]`. |
| **Fuxyez** (`fuxyez/`) | Fuxyez AuraFsConfig should match this schema, not a third copy. |


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
