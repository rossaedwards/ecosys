# AINTS_REVIEW — `aurafs/src/storage/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- filesystem, inode, directory, journal, quota, snapshot, local, shard_store, fuse.

**Wired:** Declared. Imports undeclared `shard_server` — library compile break.  
**Era / voice:** Love + QDiv; directory/journal TRL-4 tagged. Extra `shardstore.rs` not in `mod.rs`.

## Suggestions

- Fix shard_server import or `mod` it.
- One FUSE (this vs `src/fuse/`).
- Storage ceilings from toml `[storage]`, not physics constants.

## Clarifying questions needed

### Architectural
- HAL vs in-tree POSIX FS vs FUSE portal — three layers?

### Framework
- Welcome file system + storage half.

### Vision
- GhostLink/DataSlayer/Titan backends named in `lib.rs` — which exist as code?

### Intentions
- Local-only first on the Aura laptop?

### Design
- Declare or delete `shardstore.rs`.

### Ideas
- Journal as Trap-State durability.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Inodes ≠ 27-node TSL. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | N/A. |
| **Fuxyez** (`fuxyez/`) | Backend bytes live here; API is the contract. |


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
