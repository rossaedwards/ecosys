# AINTS_REVIEW — `aurafs/src/shard_server/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- server, ACL, autoheal, IPFS + cluster, gRPC, mesh, gossip, CLI (~490), server (~592).

**Wired:** **No** `mod` in `lib.rs`, but `storage/` imports it — compile landmine. Has own `main.rs`, no Cargo `[[bin]]`.  
**Era / voice:** Love + QDiv fat orchestrator.

## Suggestions

- Either `pub mod shard_server` + bin, or storage stops importing it.
- IPFS is not a welcome transport (Titan/GhostLink/Starlink are).
- Helm StatefulSet assumes this process — charts are ahead of Cargo.

## Clarifying questions needed

### Architectural
- Sidecar binary vs in-process storage HAL?

### Framework
- Welcome mesh ≠ IPFS. Confirm experimental sidecar.

### Vision
- One node process or API + shard STS as helm draws?

### Intentions
- gRPC for g0dm0d3 or for cluster only?

### Design
- Do not add `[[bin]]` without Ross if it forks the crate identity.

### Ideas
- If it stays, it is the mesh shard daemon; `api/` is the hub.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | N/A. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | N/A. |
| **Fuxyez** (`fuxyez/`) | Should talk HTTP/gRPC, not embed this module. |


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
