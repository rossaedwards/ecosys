# AINTS_REVIEW — `aurafs/src/network/transport/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `dns_client.rs` — 14 LOC (stub)
- `http_api.rs` — 14 LOC (stub)
- `mod.rs` — 10 LOC (stub)
- `quic_client.rs` — 14 LOC (stub)
- `quic_server.rs` — 14 LOC (stub)
- `starlink_client.rs` — 49 LOC (thin)
- `tcp_stack.rs` — 14 LOC (stub)
- `udp_multicast.rs` — 14 LOC (stub)
- `websocket.rs` — 14 LOC (stub)

Named QUIC/TCP/UDP/HTTP/DNS/Starlink. Starlink client thin; most others stubs.

## Suggestions

- Titan-Libp2p is primary in law — do not make QUIC a silent default.
- Starlink is tertiary.

## Clarifying questions needed

### Architectural
- Which transport is lab-default on the Aura laptop?

### Framework
- TRCA referenced in welcome, no folder.

### Vision
- Off-grid LoRa vs Starlink backhaul first?

### Intentions
- Stub matrix for future cfg.

### Design
- Feature gates matching Cargo.

### Ideas
- One Transport trait used by Meshwerk.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | N/A. |
| **FTQC** | N/A. |
| **TVFD** | N/A. |
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
