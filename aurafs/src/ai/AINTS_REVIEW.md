# AINTS_REVIEW — `aurafs/src/ai/`

**Type:** Features / suggestions / clarifying questions for this AuraFS product slice.  
**Not:** APS-OKF stamp, TSLCA/FTQC/TVFD paper reprint, exploit PoC, or a rewrite of this folder.  
**Voice:** Audry. Canon stance. Welcome source of truth: `aurphyx_welcome2tribe.md`.  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`. Companion inventory: `AINTS_SUPPZ.md`.  
**Readiness map:** `aurafs/AURAFS_SRC_READINESS.md` + `aurafs/src/AINTS_REVIEW.md` (Phase Two).  
**Scanned:** 2026-08-23, lab `rossaedwards/ecosys`.

Love-signed banners (`f0rg3d in l0v3`, R.F. Lovezme) and `Aurphyx Quantum Division` / Phase II TRL-4 headers are **the same product era**, not competing authors. Classify; do not restyle.

## Features

- Locked `fractal_orchestrator.rs` (~311 LOC).
- `error.rs`, `AI_IMPLEMENTATION_GUIDE.md`.

**Wired:** Yes (`pub mod ai`). Not started from `main.rs`.  
**Era / voice:** Love + QDiv. `fractal_orchestrator.rs` TRL-4 locked.

## Suggestions

- Do not turn this into Audry or a new agent crate.
- If it schedules shard placement, it must call `physics::INVARIANTS.calculate_replicas`, not invent η.
- Keep Memoree (cognitive memory + g0dm0d3 pair) out of this folder.

## Clarifying questions needed

### Architectural
- Is the orchestrator storage placement, inference routing, or both?

### Framework
- FTQC \(D_f\) is cited in product comments — cite `ftqc/` / root, do not fork 1.585 here.

### Vision
- Welcome: Audry administers nodes/shards/fields. Is this a precursor to `afs-4dm1n` or a dead sketch?

### Intentions
- Should `AI_IMPLEMENTATION_GUIDE.md` be retired once `afs-4dm1n` exists?

### Design
- Single entry `FractalOrchestrator` re-exported or unused and documented as experimental.

### Ideas
- Placement scores as SCX coherence, not HIF unless C,R,A are measured.

## Alignment (cite, do not reprint)

| Volume / product | How this folder should integrate |
|---|---|
| **TSLCA** (`tslca/`) | Orchestration is not SUXS-IFO \(\mathcal{U}\). Do not name a fused scalar as Tr. |
| **FTQC** (`ftqc/`) | Fractal in the filename is product scheduling, not `dim(H_acc) = d^{n · D_f^{α(k)}}`. |
| **TVFD** (`tvfd/`) | No TVFD field object here. |
| **SAGES** (`sages/`, `SAGES_Framework/`) | Do not invent a 14th guardian for AI ops. |
| **Fuxyez** (`fuxyez/`) | Compiler/runtime own lattice persist; this is not FUTE. |


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
