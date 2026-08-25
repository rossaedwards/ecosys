# AINTS_REVIEW — `aurafs/src/api/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `server.rs` (~227) `ApiServer` / `AppState`.
- `websockets.rs` (~147) AuraCore Hub sketch.

**Wired:** `pub mod api` exists; **`mod.rs` empty**. `main.rs` imports `ApiServer` — will not resolve.  
**Era / voice:** Love-signed `server.rs` / `websockets.rs`.

## Suggestions

- Export siblings from `mod.rs` **or** stop `main.rs` from lying. This is PR-menu A.
- Bind default `0.0.0.0:8080` is not production; document loopback vs mesh expose.
- Helm `deployment-api` assumes this process exists — keep charts honest until the bin compiles.

## Clarifying questions needed

### Architectural
- Is HTTP the public plane, or is gRPC (`shard_server`) the mesh plane?

### Framework
- g0dm0d3-ktrl wants REST/gRPC — deferred. Which surface is the contract?

### Vision
- AuraCore Hub vs g0dm0d3 as corpus callosum — one brain or two hubs?

### Intentions
- Is WebSocket for Xplor live tree, or for Audry events?

### Design
- Versioned `/v1` now so Fuxyez and Memoree do not bind to a moving sketch.

### Ideas
- Namespace paths as SIX (bytes), metadata as SCX, SoulKey as ICX headers.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Do not expose Φ_ij as JSON unless a real lattice engine exists (`src/tslca/` is a stub). |
| **FTQC** (`ftqc/`) | No quantum API here. |
| **TVFD** (`tvfd/`) | No Z_vac endpoint. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Authn/authz hooks, not guardian crates. |
| **Fuxyez** (`fuxyez/`) | `AuraFsBackend.connect()` needs this API list in writing. |


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
