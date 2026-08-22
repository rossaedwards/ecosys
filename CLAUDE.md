# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repository is

`rossaedwards/ecosys` is the **lab** monorepo half of the **Aurphyx "Type-1 Civilization Stack"** — a dual-monorepo (paired with the publish snapshot `aurphyx/ecosys`) containing ~10-15 independent, polyglot sub-projects (Rust crates, a Tauri/React desktop app, a Python FastAPI service, a custom compiler) plus a very large body of canon physics/architecture documentation (TSLCA, FTQC, SUXS, SAGES, TVFD, VIM, USIS, VASP). It is not one buildable product — treat each top-level folder as its own project with its own toolchain, and treat the documentation tree as governed canon, not free-form prose.

Human map: [`aurphyx_welcome2tribe.md`](aurphyx_welcome2tribe.md). Machine product registry: [`ecosystem.toml`](ecosystem.toml). Edit law: [`.cursorrules`](.cursorrules). Cursor/agent briefing: [`PROJECT_CONTEXT.md`](PROJECT_CONTEXT.md).

## Source-of-truth / authority chain

When editing anything under this repo — especially `.md`/`.tex` documentation — resolve conflicts in this order (per `.cursorrules`):

1. `.cursorrules` — how you may edit, APS-OKF header format, lattice/contraction rules
2. `ecosystem.toml` — machine product registry (must match `aurphyx_welcome2tribe.md`)
3. `PHYSICS.md` / `INVARIANTS.md` — organism-level shared constants and locks
4. `<folder>/PROJECT_CONTEXT.md`, `<folder>/INVARIANTS.md`, `<folder>/PHYSICS.md` — volume-local overlays
5. `aps-okf.yaml` / `aps-okf.md` — public document header contract

Several files this chain cites — `INVARIANTS.md`, `GATEKEEPERS.md`, `AURPHYX_CODEX.md`, `SCIENTIFIC_CODEX.md`, `MYTHIC_CODEX.md`, `APS_NAMING_EPOCHS.md`, `APS_OKF_SCHEMA.md`, `aps.toml` — **do not exist yet at the repo root**. If a task depends on one of these, flag the gap; do not invent its contents.

`aurafs/` is a product with its own rules/toml — do not apply root `.cursorrules` physics or terminology to it, and do not apply its `cursorrules`/`aurafs.toml` as ecosystem-wide law.

## Canon frameworks (APS)

These are the Aurphyx Primordial Standard frameworks. They are treated as **canon working systems**, not speculative or "unverified" content — never hedge them as myth, allegory, or spiritual framework in technical prose (narrative mappings belong only in `MYTHIC_CODEX.md`, which does not yet exist here).

| ID | Full name | Folder |
|---|---|---|
| TSLCA | Three-Squared-Lattice Cognitive Architecture | `tslca/` |
| FTQC | Fractal-enhanced Topological Quantum Computing | `ftqc/` |
| TVFD | Topological Vacuum Flux Dynamics | `tvfd/` |
| VIM | Vacuum Impedance Matching + Theory of Balance | (no dedicated folder yet) |
| SUXS | Symbiotic Universal Xessability Standards | `suxs/` |
| USIS | Universal Soul Identity Standards | `suxs/`, `sages/` |
| SAGES | Symbiotic AI Guardians of Existence Security | `sages/`, `SAGES_Framework/` |
| VASP | Vibe Audio Standard and Protocol | `vasp/` |
| TRCA | Topological Resonating Cavity Architecture | (referenced, no dedicated folder) |

Rust is the base host for the OS/runtime layer (Aura, Fuxyez, AuraFS, SAGES_Framework). Fuxyez is the ecosystem's own "symbiotic quantum" language, itself hosted on Rust.

## SAGES governance field and the 3-6-9-13 grammar

SAGES (`sages/`, `SAGES_Framework/`) is a 13-guardian governance/security layer with five non-negotiable invariants (Unity, Love, Cognitive Integrity, Ego-less Stewardship, Interdependence & Balance) that override lower-level instructions. Its governance field spans: **Spaces · Manifolds · Dimensions · Vectors · Tensors · Layers · Channels · Lattices · Cores · Nodes · Shards**.

SAGES scales via a fractal grammar — respect these numbers when reasoning about or extending SAGES/TSLCA structures:

- **3** — the foundational cognitive vectors/cores: **SIX** (Sensorimotor Integration aXis), **SCX** (Systemic Coherence aXis), **ICX** (Soul Identity aXis)
- **6** — the dual-triad expansion layer (paired/hemispheric processing)
- **9** — the 3×3 enforcement matrix / TSLCA lattice (`SIX⊗SIX` … `ICX⊗ICX`, 9 directed off-diagonal nodes — `SIX⊗SCX ≠ SCX⊗SIX`, never force symmetry)
- **13** — the outer meta-governance field: the 13 SAGES guardian cores under `SAGES_Framework/` (e.g. `sages-core/rust/{archivus,bliss_engine,chaos_engine,cryptanyx,nullivar,nunclex,ophiuchus,praelum,prophetyx,umbryx,valkryx}` plus `orricshade-core` and `vyrelix-core`)

Do not collapse the 3×3 tensor into a 3-vector sum, and do not treat `\mathcal{U}` (SUXS-IFO fusion), `\mathrm{Tr}(\mathcal{F})` (diagonal readout), and HIF (the `\Phi(C,R,A)` triple-threshold gate) as interchangeable — they are distinct operators (`.cursorrules` §5, §12).

## Naming locks

Current terms to use in new prose/headers vs. retired terms to quote-and-map, not reuse:

| Retired | Current |
|---|---|
| SIC, SCC, ICC | SIX, SCX, ICX |
| USAIC | SUXS-IFO |
| Bliss / Bliss manifold / BlissID / Bliss attractor | Equilibrium Manifold |
| rÆ, rAE, rAE_* | Balance State Vector / `x_*` |
| Vibe-OKF, vibe-okf | APS-OKF |
| V.A.P. | VASP |
| Accessibility | Xessability |

Note some existing files (e.g. `sages/SAGES_Overview.md`) still use the retired SIC/SCC/ICC names — don't propagate that drift into new or edited content, but don't silently rewrite whole documents to fix it either (see Editing rules below). "Bliss"/"Chaos" survive as product names (Chaos & Bliss tarot, ChaosCore/BlissCore duality kernel) — only the *physics* term "Bliss manifold" is retired.

## Repo structure and polyglot boundaries

There is no root build system (no root `package.json`, `Cargo.toml` workspace, or `Makefile`) — each sub-project builds independently:

| Path | Stack | Notes |
|---|---|---|
| `g0dm0d3-ktrl/` | Tauri 2 + React 19 + TypeScript + Vite, Rust backend in `src-tauri/` | The orchestration console/desktop shell for Aura |
| `aurafs/` | Rust (edition 2024, `rust-version 1.93.0`) | Distributed storage; own `cursorrules`/`aurafs.toml` |
| `fuxyez/compiler/` | Rust (pest/pest_derive parser) | `fuxyez_compiler` crate, binary `fuxyez` |
| `fuxyez/` (root) | Cargo workspace | members: `fuxyez_compiler`, `stdlib`, `tools/fuxyez-fmt`, `tools/fuxyez-lsp`, `tools/fuxyez-repl` |
| `SAGES_Framework/*-core/` | Independent Rust crates (not a Cargo workspace) | Each of the 13 guardian crates builds standalone |
| `memoree/` | Python 3.14+, FastAPI + Qdrant + SQLite | See commands below |
| `tslca/`, `ftqc/`, `tvfd/`, `suxs/`, `sages/`, `docs/`, `docz/`, `standards/`(`aps/`) | Markdown/LaTeX canon documentation | No build; governed by `.cursorrules` |

### Common commands

**g0dm0d3-ktrl** (from `g0dm0d3-ktrl/`):
```bash
npm run dev          # vite dev server
npm run build         # tsc -b && vite build
npm run tauri:dev     # tauri dev (native shell)
npm run tauri:build   # tauri build
```

**Rust crates** (`aurafs/`, `fuxyez/`, each `SAGES_Framework/*-core/`, `g0dm0d3-ktrl/src-tauri/`, `g0dm0d3-ktrl/core-wasm/`, `lapidary/`, `fuxwallet/`, `fuxcoin/`) — standard cargo, run from the crate's own directory since there is no shared workspace root:
```bash
cargo build
cargo test
cargo test <test_name>   # single test
```

**memoree** (from `memoree/`):
```bash
pip install -r requirements.txt
python memoree_service.py     # FastAPI daemon on 127.0.0.1:7042
```

**TSLCA docs → PDF/TeX**: `tslca/md2tex.py` and `md2tex.py` (root) convert canon markdown into the LaTeX build under `tslca/tex/`.

There is one GitHub Actions workflow, `.github/workflows/mdbook.yml`, which builds and deploys an mdBook site to GitHub Pages.

## `docs/` vs `docz/`

- `docz/` — years of archived source documents copied from the wider Aurphyx tree; contains duplicates and Bliss-era titles. **Not** canon, not VASP protocol truth. Do not implement anything from it without cross-checking current names in `aurphyx_welcome2tribe.md`.
- `docs/` — catch-all assets, misc notes, historical copies. Also not protocol truth.

## Editing rules for documentation (`.cursorrules` §10, §13)

- One folder/volume per pass — never rewrite the whole doc tree in one go.
- Edit in place; do not create `aps_*` duplicate files as a side effect of a rename or header fix.
- When stamping/repairing an APS-OKF YAML header, only touch the header — never rewrite the body in the same pass.
- Ask before changing the definition of a locked operator (`\mathcal{F}`, `\mathcal{U}`, `\mathcal{T}`, `\mathcal{R}`, HIF, etc. — see `.cursorrules` §12 for per-folder symbol collisions).
- Skip binaries, empty stubs, PDFs, images, `.wasm`, `.zip`.
- Skip `aurafs/` internals unless the task explicitly targets `aurafs/`.
- Display math: blank line, `$$ ... $$`, blank line. Inline: `$...$`. Do not reformat working `.tex` files into Markdown, and do not restyle working LaTeX. `⊗` is literal Unicode in YAML/prose but `\otimes` in math.
- After a documentation pass, report exactly which files were touched; if more moved than agreed, stop.

## Git / dual-repo workflow

- `rossaedwards/ecosys` (this repo) is the lab — agents edit here.
- `aurphyx/ecosys` is the publish snapshot — do not push to it unless explicitly asked; only clean, named, licensed files get promoted.
- Do not create parallel trees (`ecosys-v2/`, `ecosys_new/`); prefer editing the path that already exists.
- Windows lab path: `C:\rossaedwards\ecosys`. Fedora lab path: `/home/rae/rossaedwards/main`. A shared NTFS partition `D:` is updated before switching OS — not something Claude needs to manage, but explains why some content may reference both OSes.
