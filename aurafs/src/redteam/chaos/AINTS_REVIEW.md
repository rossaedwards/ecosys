# AINTS_REVIEW — `aurafs/src/redteam/chaos/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `backup_corrupt.rs` — 13 LOC (stub)
- `c2_bridge.rs` — 13 LOC (stub)
- `cascade_engine.rs` — 13 LOC (stub)
- `cert_expiry.rs` — 13 LOC (stub)
- `chaos_orchestrator.rs` — 13 LOC (stub)
- `combo_chains.rs` — 13 LOC (stub)
- `config_drift.rs` — 13 LOC (stub)
- `global_ranking.rs` — 13 LOC (stub)
- `latency_injector.rs` — 13 LOC (stub)
- `metrics.rs` — 13 LOC (stub)
- `node_killer.rs` — 352 LOC (implemented-looking)
- `nodekiller.rs` — 13 LOC (stub)
- `pvp_leaderboard.rs` — 13 LOC (stub)
- `replay_system.rs` — 13 LOC (stub)
- `resource_crusher.rs` — 13 LOC (stub)
- `shard_storm.rs` — 13 LOC (stub)
- `tournament_mode.rs` — 13 LOC (stub)

Offensive-testing **name matrix**. Most files ~14-line stubs; a few large named files exist. Do not complete them.

## Suggestions

- Do not write PoCs, payloads, or attack procedures.
- Keep isolated from the default crate until a defensive-test charter exists.

## Clarifying questions needed

### Architectural
- Authorized lab suite or never-ship?

### Framework
- Not FTQC cryptanalysis. Not SAGES.

### Vision
- Pink Tribe pairing with whitehat names only?

### Intentions
- Filenames are not a build list.

### Design
- `run_test_suite()` placeholder must stay non-operational.

### Ideas
- Document names only in parent `redteam/AINTS_REVIEW.md`.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | N/A. |
| **FTQC** | quantum_breaker names are not FTQC experiments. |
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
