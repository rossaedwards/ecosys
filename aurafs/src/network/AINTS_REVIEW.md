# AINTS_REVIEW — `aurafs/src/network/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- Meshwerk (locked roles/routing/topology + ghostlink.toml).
- P2P, gossip, replication, firewall, secure tunnel, Reticulum/RNS (Rust+Python).
- Transports in law: Titan-Libp2p, GhostLink-LoRaWAN (TODO feature), Starlink-HighOrbit (thin).
- Integration stubs: arora, gvs, ineffable, opulence, wallet.
- meshtastic_integration almost all stubs.

**Wired:** Yes. Largest wired surface. `NodeManager` in `main.rs` is ~25 LOC.  
**Era / voice:** Fat Love+QDiv vs locked short Meshwerk vs 14-line stubs.

## Suggestions

- Do not implement GVS/Opulence from stub names.
- Treat RNS Python as sidecar, not crate API.
- GhostLink feature is empty — keep status honest.

## Clarifying questions needed

### Architectural
- Winner vs `src/mesh/`? Three transports — which is lab-default on the Ryzen laptop?

### Framework
- Welcome: decentralized off-grid mesh. TRCA referenced, no folder.

### Vision
- Photonic + LoRa + Starlink in one process or three daemons?

### Intentions
- IPFS lives in `shard_server/` — in or out of Meshwerk law?

### Design
- Lock list already names Meshwerk files — grow those, not a fourth stack.

### Ideas
- Routing overhead from PBG / d_s via INVARIANTS only.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Mesh ≠ activation lattice. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | Do not set link budget from Z_vac without a labeled TVFD note. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Defense/ subfolder is names only; no attack code. |
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
