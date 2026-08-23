# AINTS_REVIEW — `aurafs/src/network/meshtastic_integration/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `firmware_bridge.rs` — 14 LOC (stub)
- `lora_config.rs` — 14 LOC (stub)
- `lora_radio.rs` — 14 LOC (stub)
- `mesh_discovery.rs` — 14 LOC (stub)
- `meshtastic_proto.rs` — 14 LOC (stub)
- `mod.rs` — 10 LOC (stub)
- `packet_parser.rs` — 14 LOC (stub)
- `security_analysis.rs` — 14 LOC (stub)

Almost all ~14-line stubs. GhostLink-LoRaWAN is the named product transport, not Meshtastic-as-law.

## Suggestions

- Do not implement Meshtastic firmware from filenames.
- Keep honest: feature empty.

## Clarifying questions needed

### Architectural
- Meshtastic sidecar vs GhostLink native?

### Framework
- Welcome does not name Meshtastic.

### Vision
- Radio for Arora/IoT?

### Intentions
- Placeholder for LoRa experiments.

### Design
- Stay out of default crate.

### Ideas
- If radio happens, Meshwerk roles consume it.

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
