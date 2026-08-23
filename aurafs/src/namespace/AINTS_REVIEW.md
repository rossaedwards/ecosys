# AINTS_REVIEW — `aurafs/src/namespace/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `manager.rs`, `shard.rs`, ENTERPRISE notes.

**Wired:** Yes. `main.rs` type-name risk.  
**Era / voice:** Love-signed VFS paths.

## Suggestions

- This is path virtualization for Xplor later — keep API stable.
- Do not encode TSLCA 27-node indices as path components unless Ross asks.

## Clarifying questions needed

### Architectural
- Namespace vs `storage/directory.rs` vs FUSE inodes — one tree.

### Framework
- SUXS Xessability: paths should be human-xessable, not only UUIDs.

### Vision
- Per-SoulKey root vs shared mesh roots?

### Intentions
- Virtualization for multi-mount or multi-soul?

### Design
- Export NamespaceManager to match `main.rs`.

### Ideas
- SIG as the namespace root name.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Paths are not OKF nodes. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | N/A. |
| **Fuxyez** (`fuxyez/`) | lattice_id in Fuxyez should be a namespace path or shard id — pick one. |


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
