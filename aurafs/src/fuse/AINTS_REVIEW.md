# AINTS_REVIEW — `aurafs/src/fuse/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- node, session, timed_lock, filesystem, mount, inode_cache, fusemain.
- Cargo feature `fuse` + optional `fuser`. Dokany mentioned, not proven here.

**Wired:** Yes. Extra `fuse/main.rs`. `main.rs` crate imports `AuraFSFuse` (may live in `storage/fuse.rs`).  
**Era / voice:** Love + QDiv; `session.rs` / `mod.rs` TRL-4.

## Suggestions

- One FUSE implementation (`fuse/` XOR `storage/fuse.rs`).
- `timed_lock` must use 100 μs = T₂/16 from INVARIANTS.
- Crate `main` comments say it starts FUSE — it does not. Fix comment or mount.

## Clarifying questions needed

### Architectural
- Portal on Linux FUSE first, Windows Dokany later — confirm order.

### Framework
- Aura personal OS mounts this; Arora/Biznyx later.

### Vision
- Is the mount the soul’s home directory or a mesh namespace?

### Intentions
- Xplor browses mesh+FS — FUSE is not that UI.

### Design
- Remove extra `fuse/main.rs` or make it a documented example bin.

### Ideas
- Inode ↔ ShardId table is the SIX surface.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Mount is SIX⊗SIX (world contact), not identity. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Mount policy via ACL/gov, not a new guardian. |
| **Fuxyez** (`fuxyez/`) | Developers may use FUSE; compiler should use API/shard IDs. |


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
