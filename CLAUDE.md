# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repository is

This is **Ross A. Edwards' / Aurphyx LLC's personal monorepo** — a loose collection of independent research
projects, software prototypes, and creative-writing projects, not a single application. There is **no
repo-wide build, lint, or test command**. Each subproject at the top level is its own island with its own
tooling (or none at all, if it's a docs/whitepaper folder). Always `cd` into the specific subproject before
running any build/test command.

The unifying theme across the docs-heavy folders is a personal research program ("Aurphyx") spanning fractal
geometry, quantum computing, and a large invented terminology system (Yez/Fuxyez, TVFD, APS, VASP, sages,
etc.). Treat top-level `*_CODEX.md`, `*_TERMINOLOGY.md`, and `aps/` glossary files as the canonical definitions
if you need to interpret domain-specific jargon in prose or comments elsewhere in the repo.

Root-level files named like `rossaedwards_*_tree.txt`, `*_filelist.txt`, `*_repolist.txt` are point-in-time
directory snapshots (some are 1MB+). They are not documentation to keep in sync — ignore them unless the user
specifically asks about repo history/snapshots.

## Subprojects with real code

| Path | What it is | Stack | Common commands |
|---|---|---|---|
| `aurafs/` | "Physics-informed distributed storage" — fractal-geometry/topological-protection storage system | Rust (single crate, `sdk/` and `tts/` are separate crates) | `cargo build` / `cargo test` (run inside `aurafs/`, and separately inside `aurafs/sdk/`, `aurafs/tts/`) |
| `fuxyez/` | Custom "Fuxyez" programming language: compiler, `fuxrt` runtime, `stdlib`, dev tools (fmt/lsp/repl), FUTE transmutation engine | Rust workspace (`fuxyez.toml` is the workspace manifest, not `Cargo.toml`) | See [fuxyez/REVIEW.md](fuxyez/REVIEW.md) first — **as of the last review none of the three Rust crates compiled** (missing brace in diagnostics.rs, missing `[package]` in `fute/Cargo.toml`, missing bench file). The folder is also mid-reorganization: old flat-file docs are staged as deleted while a new structured layout (`compiler/`, `fute/`, `fuxrt/`, `stdlib/`, `tools/`, etc.) sits untracked. Check `git status` before assuming either layout is authoritative. |
| `lapidary/` | "Universal VSIX to Lapce Volt Transmutation Engine" | Rust (tokio async) | `cargo build` / `cargo test` inside `lapidary/` |
| `memoree/` | "Sovereign Memory Substrate" — persistent cross-model memory service for agents | Python 3.11+, FastAPI, Qdrant/LanceDB | `pip install -r requirements.txt`; look for a FastAPI entrypoint (`uvicorn ...`) before assuming one |
| `vibeplayer/` | "Vibe Media Player" (VMP) — Rust audio engine + React/Tauri shell, V.A.P. (Vibe Audio Protocol) scoring | npm (Vite/React/TS) + Rust workspace (`crates/`, `apps/vmp-tauri`), plus nested `fute/` | `npm run dev` / `npm run build` / `npm run lint`; `npm run test:rust` (= `cargo test --workspace`); `npm run tauri:dev` for the desktop shell |
| `vibeaudio/` | Earlier/sibling V.A.P. project: VLC visualization plugin + web UI | npm (Vite/React/TS) for the UI, C + CMake for `vlc-plugin/` | `npm run dev` / `npm run build` / `npm run lint`; CMake build inside `vlc-plugin/` |
| `vibe-audio-visualizer/` | "9-Pillar TSLCA Cymatic Renderer" visualizer | CMake + Python | `pip install -r requirements.txt`; CMake build for the native side |
| `mixxx/` | **Vendored copy of the upstream open-source Mixxx DJ software** (github.com/mixxxdj/mixxx) — not authored here, pulled in as reference/integration target for `vibeplayer`'s "Vinyl Vibez" mode | C++/Qt, CMake, Python (packaging) | Follow upstream Mixxx build docs if you ever need to build it; don't assume changes here are part of the Aurphyx product surface |

When touching any Rust crate, check for a `target/` directory already present in the tree (several are, e.g.
`aurafs/target`, `lapidary/target`) — these are build artifacts that should be gitignored, not source to read.

## Docs-only / research directories

These have no build system — they're whitepapers, appendices, specs, and terminology codices (mostly
Markdown, some LaTeX/PDF). Read/edit them like any prose document; there's nothing to compile:

`tvfd/` (physics formalism, "λ* fixed point"), `tslca/`, `suxs/`, `aps/` (glossaries/specs), `ftqc/`
(fault-tolerant quantum computing thesis material — has a standalone `bib_consolidator.py` citation script,
run with `python ftqc/bib_consolidator.py`), `vasp/` (V.A.P. protocol whitepapers, PDFs), `sages/`, `audry/`,
`docs/`, `overviews/`, `IL_P4A_FWC/` (quantum environmental isolation dev package).

`cartoon-quantum-mechanics/` and `the-420-platoon/` are creative-writing/entertainment projects (animated
series concepts), unrelated to the physics/software work — don't conflate their content with the research
docs when answering questions about "the project."

## Working in this repo

- **Scope tightly.** Before editing, confirm which subproject the user means — directory names overlap in
  theme (e.g. `vibeplayer/` vs `vibeaudio/` vs `vibe-audio-visualizer/` are three separate, related-but-distinct
  projects; `fuxyez/` vs the terminology in `aps/` and root `*_CODEX.md` files are docs vs. code for the same
  invented language).
- **Don't run repo-wide search-and-replace or formatting** across the whole tree — there's no shared style
  guide or lint config at the root, and `mixxx/` in particular is third-party code that should stay diffable
  against upstream.
- **fuxyez/ is actively being reorganized** (see git status: many `fuxyez/APS-YEZ-*` and `fuxyez/*.tex` files
  deleted, many new directories untracked). Don't assume the working tree there is in a finished state; read
  [fuxyez/REVIEW.md](fuxyez/REVIEW.md) for the current honest status before making claims about what does or
  doesn't work.
- When a task involves the invented terminology (Fux/Yez/Sigils/Spinons, TVFD, sages, APS-YEZ volumes, etc.),
  ground definitions in `aps/` glossary files or `SCIENTIFIC_TERMINOLOGY.md` / `MYTHIC_TERM_CODEX.md` rather
  than guessing meaning from the name alone — the same term is sometimes used in both a "mythic/ritual" prose
  register and a technical/spec register across different files.
