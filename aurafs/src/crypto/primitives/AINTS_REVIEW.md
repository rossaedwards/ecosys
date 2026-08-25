# AINTS_REVIEW — `aurafs/src/crypto/primitives/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `bech32.rs` — 14 LOC (stub)
- `cbor.rs` — 14 LOC (stub)
- `encoding.rs` — 14 LOC (stub)
- `hashes.rs` — 14 LOC (stub)
- `mod.rs` — 12 LOC (stub)
- `rng.rs` — 14 LOC (stub)

encoding, rng, hashes, bech32, cbor — likely stubs. Product hash law is SHA3-256 / BLAKE3 in toml.

## Suggestions

- Do not add a third hash stack; `crypto/hash.rs` already exists.
- Bech32 is not SoulKey/SIG.

## Clarifying questions needed

### Architectural
- Primitives here vs `hash.rs` / `encryption.rs`?

### Framework
- N/A.

### Vision
- Human-readable shard IDs?

### Intentions
- Stub crate-in-a-folder.

### Design
- One encoding path.

### Ideas
- If real, wrap existing crates only.

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
