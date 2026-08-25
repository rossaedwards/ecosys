# AINTS_REVIEW — `aurafs/src/network/monitoring/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Parents:** [`../../AINTS_REVIEW.md`](../AINTS_REVIEW.md) · crate plan [`../../AINTS_REVIEW.md`](../../AINTS_REVIEW.md).  
**Scanned:** 2026-08-23.

Love-signed and Aurphyx Quantum Division banners are the same product era. ~14-line `init()` files are placeholders.

## Features

- `anomaly_detection.rs` — 14 LOC (stub)
- `grafana_dash.rs` — 14 LOC (stub)
- `log_export.rs` — 14 LOC (stub)
- `mod.rs` — 9 LOC (stub)
- `net_metrics.rs` — 14 LOC (stub)
- `prometheus.rs` — 14 LOC (stub)
- `topology_map.rs` — 14 LOC (stub)

Named Prometheus/Grafana/anomaly. Stubs. Overlaps `src/monitoring/`.

## Suggestions

- One metrics facade — prefer `src/monitoring/`.
- Helm should not assume Grafana in-crate.

## Clarifying questions needed

### Architectural
- In-proc metrics vs sidecar scrape?

### Framework
- d_s variance is product lock (1.37), not root 1.36.

### Vision
- g0dm0d3 operator UX or Grafana?

### Intentions
- Placeholder dashboards.

### Design
- Do not duplicate exporters.

### Ideas
- Alert on `PhysicsViolationError`.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** | Do not plot HIF without C,R,A. |
| **FTQC** | Cite, don't re-derive. |
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
