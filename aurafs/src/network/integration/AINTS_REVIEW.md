# AINTS_REVIEW — `aurafs/src/network/integration/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `arora_network.rs` — 14 LOC (stub)
- `crypto_bridge.rs` — 14 LOC (stub)
- `gvs_network.rs` — 14 LOC (stub)
- `ineffable_sync.rs` — 14 LOC (stub)
- `mod.rs` — 10 LOC (stub)
- `opulence_bridge.rs` — 14 LOC (stub)
- `storage_sync.rs` — 14 LOC (stub)
- `wallet_bridge.rs` — 14 LOC (stub)

14-line bridges: arora, gvs, ineffable, opulence, wallet, storage, crypto.

## Suggestions

- Same rule as `crypto/integrations`: no fake SDKs.
- g0dm0d3/Xplor is the first real client after API exports.

## Clarifying questions needed

### Architectural
- Network-plane bridges vs crypto-plane bridges — why both?

### Framework
- Each name is a sibling product.

### Vision
- Which join is first?

### Intentions
- Filename matrix.

### Design
- Stay stubs.

### Ideas
- storage_sync might be real later — then it belongs next to `storage/`.

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
