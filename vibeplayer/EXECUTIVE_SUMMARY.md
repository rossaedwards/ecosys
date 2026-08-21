# Vibe Media Player — Executive Summary

**Aurphyx LLC** · 2026-08-21 · VASP 3.69

## One sentence

A native media host that decodes real files, writes Vibe Audio Protocol sidecars, and can grow a Mixxx-class DJ surface — the desktop counterpart to Vibe Audio Player.

## Problem

Desktop players store bibliographic tags. They do not store the 9-dimensional experiential identity SoulSync, SAGES, and context playlists need. Scraping the web into a pane is not analysis.

## Solution

VMP is a Rust workspace + React/Tauri shell:

- **Decode / play** — Symphonia + cpal (`vmp-audio`)
- **Tags** — lofty read/write + `.vap.json` sidecar
- **Score** — `vmp-vap` + reference `VASPScoringEngine`
- **DJ** — Vinyl Vibez via v01d transmute of Mixxx (GPL symbiont)
- **Pack** — v01d / FUTE

Identity from `Artist - Title` filenames and ID3. Pillars from DSP and catalog facts. Unknown when unmeasured.

## What exists today

- Workspace crates, CLI (`vmp`), Vite UI, Tauri app scaffold.
- Protocol copies and golden fixtures in-tree.
- Documented Spotify → VASP mapper that already produced complete profiles when audio-features were available.
- Vinyl / FUTE scaffolding and Mixxx transmute docs.

## What does not exist yet

- A standalone `vap-analyze` Python package as drawn in the old exec-sum tree.
- Full Mixxx engine inside `vmp-vinyl`.
- Shared nested-3.69 store with the web player (Framez pane still flattens).
- First-class TIDAL desktop login (app is registered for the web player).

## Split with Vibe Audio Player

| | Vibe Audio Player | Vibe Media Player |
| --- | --- | --- |
| Home | `vibeaudioplayer/` | `vibeplayer/` |
| Surface | Browser + Framez | CLI + Tauri + modules |
| Strength | Library pins, OAuth, Ask AI | Decode, sidecar, Vinyl, v01d |
| Shared | VASP 3.69, Aurphyx LLC, identity-vs-pillar rule |

## Next

Keep `vmp-vap` aligned to the official 3.69 schema. Use the same unknown/tag policy as the web player so a file opened in either product yields the same honest profile.
