---
type: implementation-note
title: Fuxyez — Summary and Suggestions
description: Honest status of the Fuxyez language stack mid-reorganization, and a compile-first phased plan so v01d can serve Vibe Media Player skin transmutation without a second engine.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Fuxyez
  - Aura
  - AuraFS
domains:
  - systems
  - cognition
nodes:
  - SCX⊗SCX
cores:
  - SCX
fields:
  - cognitive-field-tensor
---

# Fuxyez — SUMZ / SUGGZ

**Folder:** `fuxyez/`  
**Role:** Symbiotic programming language. Rust host. Compiler + FuxRT + YezRT + FUTE (v01d) + YezL + stdlib. Aura is built on Fuxyez and Rust.  
**Priority:** 3 of 3 with Vibe Audio Player and Vibe Media Player.

This folder is **mid-reorganization**. Git status shows `fuxrt/std/` deleted and `fuxrt/stdlib/` + `yezrt/stdlib/` untracked duplicates. Do not assume either layout is finished.

**Gap:** `REVIEW.md` (cited by root `CLAUDE.md`) is **missing**. `PROJECT_CONTEXT.md` is missing. `compiler/README.md` is empty. `fute/README.md` and `fute/Cargo.toml` in *this* tree are empty.

---

## Intended stack (tribe welcome)

| Piece | Job |
|---|---|
| Fux Compiler | TSL dual-kernel compile of `.fux` / `.yez` |
| FuxRT | Fux runtime |
| YezRT | Yez runtime |
| FUTE / v01d | Universal transmutation (packages + languages) |
| YezL | Legacy language museum / AST / tooling |
| Yez | Sophos (scripting engine) + Gavinium (scripting language) |

`fuxyez.toml` registers extensions: `.fux`, `.fuxrs`, `.fuxpy`, `.fuxjs`, `.yez` and mirrors (`.xuf`, `.zey`, …).

## What the tree actually contains

| Path | Honest status |
|---|---|
| `compiler/` | Real sources. **Cargo.toml only lists pest.** `main.rs` wants tokio + tracing. `diagnostics.rs` wants miette + thiserror + serde, then **corrupts after the test module** (extra `}` plus pasted imports). Workspace member name is `fuxyez_compiler`; directory is `compiler/`. |
| `fuxrt/` | Runtime crate. `lib.rs` still declares inline `pub mod std` and is **not wired** to `stdlib/`. Duplicate `core/` vs `stdlib/core/`. File tails have pasted duplicate fragments. Missing `benches/ritual_benchmarks.rs`. `edition = "2024"`. |
| `yezrt/` | Stdlib **mirror only**. No `Cargo.toml`, no `lib.rs`. |
| `fute/` | **Large language-transmutation scaffold** (U-AST, plugins, fcargo CLI) with **empty `Cargo.toml`**, missing modules (`bridge`, `registry`, `ritual`, `ethical`, `telemetry`), empty `transmute` command, corrupted `lib.rs`/`main.rs` tails. **Working pack/skin v01d is `vibeplayer/fute`.** These are two FUTEs, not one. |
| `stdlib/` | README only (core / io / oracle). |
| `tools/` | README only. fmt / lsp / repl are listed in the workspace but **have no Cargo.toml**. |
| `yez/sophos`, `yez/gavinium` | Readmes. Not a compiler. |
| `yezl/` | rust + python stubs. |
| `examples/` | `.fux` / `.yez` samples including AuraFS bindings. |
| `governance/` | Separate Cargo.toml. |
| `docs/ROADMAP.md` | All boxes unchecked. Still accurate. |

Root `fuxyez.toml` is both `[package]` and `[workspace]` with `edition = "2025"` and members that do not match the directories. That is not a buildable workspace today.

## How this serves Vibe (the reason it is a top-3)

v01d is Fuxyez’s third compiler. VMP already calls it:

- `wsz` → `vskin` (skeleton only)
- `vsix` → `volt`
- C/C++ → Rust (`v01d lang`) for Mixxx and the visualizer

**Until FUTE lives here and compiles, Vibe Media Player owns a nested copy.** That is the wrong long-term shape. Lapidary (`lapidary/`) is the proven vsix→volt *pattern*. Unify:

```
fuxyez/fute     = canonical v01d crate + `v01d` binary
vibeplayer/fute = path dependency (or git path) — delete the fork after move
lapidary        = VSIX-specific frontend over the same engine
```

Skinz Dashboard’s **v01d button** is a FUTE job: WinAmp / MediaMonkey / Kodi / Opera GTX → `.vskin`. Language work (`.fux` compile) can wait one phase behind pack transmute, because VMP needs packs *now*.

## Honest compile blockers (do these first; do not add features)

Widespread pattern: **valid code, then pasted duplicate/unrelated fragments at EOF.** Fix tails before adding features. Confirmed in `compiler/src/diagnostics.rs`, `fute/src/lib.rs`, `fute/src/main.rs`, `fuxrt/lib.rs`, `fuxrt/core/mod.rs`, `integrations/aurafs/mod.rs`.

1. Quarantine corrupted EOF tails. `diagnostics.rs` extra `}` after tests is a hard fail.
2. Fix workspace members → real paths (`compiler`, `fuxrt`, later `fute`). Remove nonexistent `tools/*` until they exist.
3. Compiler `Cargo.toml`: add crates `main.rs` / `diagnostics.rs` already import (tokio, tracing, miette, thiserror, serde) — or delete unused imports.
4. Pick **one** stdlib path: `fuxrt/stdlib`. Wire `lib.rs` to it. Stop cloning into `yezrt` until YezRT has a different ABI.
5. Do **not** dump `vibeplayer/fute` (pack/skin v01d) into `fuxyez/fute` (language U-AST) as a blind overwrite. Federate: shared traits, two frontends — or keep v01d as product FUTE until language FUTE compiles.
6. Edition: stop mixing 2021 / 2024 / 2025 until rustc on the EliteBook agrees.
7. `governance/Cargo.toml`: duplicate `uuid` key.

Do not implement oracle, quantum collapse, or distributed chorus until `fuxyez file.fux` prints a diagnostic and exits 0 on a hello example.

## Phased series to finish Fuxyez

### Series FUX-0 — One binary that runs (this is the whole game)

1. `cargo build -p fuxyez_compiler` succeeds.
2. `fuxyez examples/...` parses one `.fux` and one `.yez` (error quality can be ugly).
3. Workspace root `fuxyez.toml` members match directories.
4. Write the missing `REVIEW.md` as a 20-line honesty sheet (compile matrix). No poetry.

### Series FUX-1 — One runtime

1. FuxRT loads compiler bytecode **or** interprets AST. Pick one. Document it.
2. Stdlib: `core` (lattice types as data, not a second physics), `io`, then AuraFS *client* stubs that call `aurafs` only if that crate’s put/get works.
3. YezRT = thin host for Sophos/Gavinium scripts, not a pasted fuxrt.

### Series FUX-2 — Canonical v01d (Vibe unblocks here)

1. Repair `fuxyez/fute` **language** pipeline until `fcargo transmute sample.rs --from rust` emits `.fux` — or delete dead imports (`ethical`, `telemetry`, …) and stub the rest.
2. Keep `vibeplayer/fute` as the **pack** engine (`Wsz`/`Vskin`) until this crate compiles. Then share traits; do not overwrite pack v01d with U-AST scaffolding.
3. Add `MmTheme`, `KodiSkin`, `OperaGx` on the **pack** crate (whichever survives as v01d).
4. Real WinAmp region harvest (not copy-bytes). This is the Skinz v01d button backend.
5. Lapidary path-depends the same pack crate for vsix→volt.

### Series FUX-3 — Tools (ROADMAP Phase 2)

fmt, lsp, repl as real crates. VS Code / Zed / Lapce via Lapidary volts. Syntax highlight last.

### Series FUX-4 — Language depth (ROADMAP Phase 3)

Oracle, collapse modes, chorus, WASM target. Only after hello-world and v01d packs.

## Binding

| Consumer | What they need from Fuxyez |
|---|---|
| `vibeplayer/` | v01d packs + later a `.fux` skin DSL if we want it |
| `vibeaudioplayer/` | Nothing until `.vskin` JSON exists |
| `aurafs/` | Optional persist of lattices — not a compile dependency |
| `lapidary/` | Shared FUTE, not a second AST |
| `g0dm0d3-ktrl` | `_c0d3x` editor later — do not scaffold it here |
| `tslca/` | Duality kernel *meaning*. Do not paste TSLCA papers into the compiler. |

## What not to do

- Do not invent a fourth fusion operator in the type checker.
- Do not keep three FUTE copies (`fuxyez/fute`, `vibeplayer/fute`, `lapidary/src/fute`).
- Do not treat ritual vocabulary in comments as a runtime requirement.
- Do not stamp the whole tree in one pass. One crate per compile fix.
