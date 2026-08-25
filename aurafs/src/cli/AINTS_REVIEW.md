# AINTS_REVIEW — `aurafs/src/cli/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- `aurafs_cli.rs`, `admin.rs`, `user.rs`, `tui.rs`, `banner.rs`.
- Commented TODOs: core/ai/quantum/game/security/enterprise CLIs — **not implemented**.

**Wired:** `pub mod cli` yes. Declared crate bin does **not** call it.  
**Era / voice:** Love + QDiv. Banner is Love-era — keep it.

## Suggestions

- Either `src/bin/aurafs.rs` becomes this clap app, or `main.rs` calls `cli`. Pick one.
- `admin.rs` is not Audry `afs-4dm1n`. Keep the name honest.
- Leave `game_cli` commented until Aethornyx/Casino scope a shard currency API.

## Clarifying questions needed

### Architectural
- Admin CLI vs API vs FUSE — which is the operator surface for TRL-4?

### Framework
- g0dm0d3 Termz vs this TUI — two terminals?

### Vision
- Should the banner stay Love-signed in production?

### Intentions
- README `init` / `cluster status` — which file implements them?

### Design
- Subcommands map 1:1 to welcome admin slugs only when those modules exist.

### Ideas
- `aurafs status` already sketched in `lib.rs::status()` — wire it.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | N/A. |
| **FTQC** (`ftqc/`) | N/A. |
| **TVFD** (`tvfd/`) | N/A. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | No guardian CLI in the default bin. |
| **Fuxyez** (`fuxyez/`) | Do not hide a Fuxyez REPL inside AuraFS CLI. |


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
