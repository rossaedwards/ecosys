---
type: implementation-note
title: Lapidary — Summary and Suggestions
description: Status of the VSIX-to-Lapce Volt transmutation CLI and how it should become a frontend of canonical FUTE/v01d rather than a third engine.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Fuxyez
domains:
  - systems
nodes:
  - SCX⊗SIX
cores:
  - SCX
fields:
---

# Lapidary — SUMZ / SUGGZ

**Folder:** `lapidary/`  
**Role:** Universal VSIX → Lapce Volt transmutation engine, “powered by FUTE.”

**Gap:** No README. No `PROJECT_CONTEXT.md`. Nested `src/fute/` is a **local** ast/generator, not the VMP v01d crate.

---

## What exists

Small, focused Rust CLI (tokio):

- `main.rs` — extract `package.json` from `.vsix`, run pipeline, write `extz/`
- `parser.rs` / `transformer.rs` / `pipeline.rs` / `context.rs`
- Default mode `XessMode::Sacred` (aggressive bloat strip)
- Fallback input `sample.vsix` if no argv

This is the **proven pack-format pattern** VMP already cites: host-agnostic, identity-preserving port, not a rebrand.

## Honest status

- Scope is one transformation (vsix→volt). Sample `extz/SampleLanguageServer/volt.toml` exists from a prior run.
- FUTE here is not wired to `vibeplayer/fute` or `fuxyez/fute`.
- **Build on this Windows host failed:** `zip` → `libz-ng-sys` needs **CMake** on PATH (or switch zip backend).
- WASM shim is metadata only — does not compile Node→WASM.

## Suggestions

1. Depend on **one** FUTE crate (after FUX-2 move). Lapidary = VSIX frontend.
2. Add README with: `cargo run -- path/to/ext.vsix`, output layout, license of transmuted extensions (do not relicense others’ code).
3. The Skinz v01d button should **rhyme** with this CLI: unzip → manifest remap → stage assets → write host pack. wsz/Kodi/GX are different parsers, same pipeline trait.
4. `cargo test` for manifest remap on a tiny fixture zip (no real marketplace download in CI).

## What not to do

- Do not grow Lapidary into a media player.
- Do not duplicate WinAmp parsing here — that belongs in FUTE pack kinds.
