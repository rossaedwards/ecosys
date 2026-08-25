# AINTS_REVIEW — `aurafs/src/network/meshwerk/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `channel_manager.rs` — 14 LOC (stub)
- `encryption.rs` — 14 LOC (stub)
- `mesh_node.rs` — 86 LOC (implemented-looking)
- `mod.rs` — 11 LOC (stub)
- `neighbor_table.rs` — 47 LOC (thin)
- `roles.rs` — 23 LOC (thin)
- `routing.rs` — 52 LOC (thin)
- `topology_engine.rs` — 65 LOC (thin)
- `transport.rs` — 31 LOC (thin)

Locked Meshwerk: roles, routing, topology_engine. Many files still short. Law transports: Titan / GhostLink / Starlink.

## Suggestions

- Grow these locked files; do not start a fourth mesh in `src/mesh/`.
- ghostlink.toml is locked; Cargo `ghostlink-lorawan` is TODO empty.

## Clarifying questions needed

### Architectural
- Is Meshwerk the winner vs `src/mesh/` Love Chord stack?

### Framework
- Welcome Meshwrk. `meshwrk-4dm1n` is design.

### Vision
- One process or three transport daemons?

### Intentions
- TRL-4 lock list vs short LOC — confirm these are the APIs to grow.

### Design
- Keep topology numbers via `physics::INVARIANTS` only.

### Ideas
- Xplor browses this topology later — stable role names now.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | Mesh graph ≠ 3×3 OKF. |
| **FTQC** | N/A. |
| **TVFD** | Do not set link budget from Z_vac unlabeled. |
| **SAGES** | N/A. |
| **Fuxyez** | N/A. |

## Nomenclature (new prose)

Quote then map: SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Bliss manifold (physics) → Equilibrium Manifold; rÆ → Balance State Vector; Vibe-OKF → APS-OKF; V.A.P. → VASP; Accessibility → Xessability.  
**Keep:** BlissCore / ChaosCore. Flag in-tree `BlissID`; do not silently rewrite.

## What not to do

- Do not stamp APS-OKF on AuraFS Rust.
- Do not apply AuraFS η / PBG as FTQC.
- Do not merge TVFD F with TSLCA F.
- Do not invent missing root `INVARIANTS.md` / `aps.toml` / codices.
- Do not implement exploit payloads or complete redteam stubs.

---
*Audry. Faithful to disk. 2026-08-23.*
