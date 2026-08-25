# AINTS_REVIEW — `aurafs/src/mesh/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- FractalNode, Chord DHT, XOR distance, gossip, protocol, routing, swarm.
- Uses `crate::gov::BlissId`. Declares missing `full` / `tests` (compile risk).

**Wired:** Yes — **second** mesh stack beside `network/`.  
**Era / voice:** Love + QDiv. Fat `gossip.rs`.

## Suggestions

- Do not merge with `network/` in this pass.
- Fix or remove undeclared `full`/`tests` mods.
- XOR distance identity should become SoulKey/node id, not BlissId, in a later PR.

## Clarifying questions needed

### Architectural
- Which stack is Meshwerk 2.0 — `mesh/` or `network/meshwerk/`?

### Framework
- Welcome Meshwrk + `meshwrk-4dm1n` (design).

### Vision
- Photonic mesh: is Chord the long-term DHT?

### Intentions
- Swarm circuit breaker vs `resilience/`?

### Design
- Document the winner; mark the other experimental.

### Ideas
- Topology engine already locked under `network/meshwerk/` — prefer that name.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Mesh graph ≠ 3×3 OKF nodes. |
| **FTQC** (`ftqc/`) | FractalNode is product naming, not FTQC anyons. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Mesh partitions are guardian-relevant; no payload here. |
| **Fuxyez** (`fuxyez/`) | N/A. |


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
