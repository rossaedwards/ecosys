# AINTS_REVIEW — `aurafs/`

**Type:** Product-level AINTS review index.  
**Not:** A rewrite of `AURAFS_PROJECT_CONTEXT.md` or an APS-OKF stamp.  
**Voice:** Audry.  
**Source of truth:** [`../aurphyx_welcome2tribe.md`](../aurphyx_welcome2tribe.md).  
**Machine registry:** [`../ecosystem.toml`](../ecosystem.toml).  
**Product law:** `cursorrules`, `aurafs.toml`.

AuraFS iz the File System, Storage System, and Mesh Network. Status in `ecosystem.toml`: **in-development**.

## Where the reviews live

The detailed pass is under `src/` (that is the crate you asked to explore):

| Path | What it is |
|---|---|
| [`src/AINTS_REVIEW.md`](src/AINTS_REVIEW.md) | Phase One crate map + Phase Two readiness / should-could-need / copy-paste prompt |
| [`src/<folder>/AINTS_REVIEW.md`](src/physics/AINTS_REVIEW.md) | Every immediate `src/` module |
| `src/<folder>/<nested>/AINTS_REVIEW.md` | Nested modules (`crypto/pqc`, `network/meshwerk`, whitehat/redteam suites, …) |
| [`AURAFS_SRC_READINESS.md`](AURAFS_SRC_READINESS.md) | Earlier readiness map (same conclusions) |
| `src/<folder>/AINTS_SUPPZ.md` | Inventory supplements from the prior pass |

## Features (product, honest)

- Real product law: η replica count, T₂, d_s clamp **1.37**, PBG, 100 μs lock — via `src/physics/mod.rs` + `aurafs.toml`.
- Dilithium-5 helper exists (`src/crypto/pqc/dilithium_sig.rs`).
- Fat Love + Quantum Division modules in core/shard/storage/network/gov/cli.
- Deploy **shape** in `helm/`, `docker/`, `k8s/`, `deploy/systemd/` — charts assume bins Cargo does not build.
- Sibling crate sketches: `sdk/`, `tts/`, `integrations/` — not wired into `src/lib.rs`.

## Suggestions

1. Next implementation PR is **compile graph** (`src/AINTS_REVIEW.md` PR-menu A), not features.
2. Do not restyle Love / QDiv headers.
3. Do not enable `whitehat/` / `redteam/` on the default crate.
4. Identity: new APIs use SoulKey → SKIM → SIR → SIG; leave BlissID until an identity PR.

## Clarifying questions needed

### Architectural
- One `aurafs` process, or helm's API + shard StatefulSet?

### Framework
- Who owns identity — leftover BlissID, USIS SoulKey pipeline, or SoulCrypt?

### Vision
- Aura personal OS first, or Biznyx-shaped `enterprise/` folder?

### Intentions
- Are Love banners permanent product voice? (This review: yes.)

### Design
- Default features: `dilithium5` only until Kyber and GhostLink exist.

### Ideas
- `lib.rs::status()` as the heartbeat g0dm0d3 / Xplor can poll.

## Alignment (cite, do not reprint)

| Volume / product | Rule for AuraFS |
|---|---|
| **TSLCA** | SIX / SCX / ICX; do not collapse U / Tr / HIF. `src/tslca/` is a stub. |
| **FTQC** | Cite D_f. Do not import AuraFS replica law as Hilbert scaling. |
| **TVFD** | Label TVFD F vs TSLCA F. Z_vac is not a storage knob. |
| **SAGES** | `gov/sages.rs` is a hook; names ≠ `SAGES_Framework/` crates. Do not invent the map. |
| **Fuxyez** | Join is `fuxyez/integrations/aurafs` persist/load. Do not embed FUTE. |

## Phase Two pointer

For production-readiness rank, era taxonomy, and the reusable agent prompt, read **[`src/AINTS_REVIEW.md`](src/AINTS_REVIEW.md)** § Phase Two.

---
*Audry. 2026-08-23.*
