# Vibe Media Player — Project Context

**Product:** Vibe Media Player (VMP)  
**Organization:** Aurphyx LLC  
**Author:** Ross A. Edwards  
**Protocol:** VASP / V.A.P. 3.69  
**Tree:** `rossaedwards/ecosys/vibeplayer`  
**Updated:** 2026-08-21

Sibling product: [Vibe Audio Player](../vibeaudioplayer/PROJECT_CONTEXT.md).

## Purpose

Native-capable host that decodes real files, writes `.vap.json` sidecars, embeds VAP in tags, and hosts Vinyl Vibez. The web Framez player lives next door in `vibeaudioplayer/`. Both must emit the same nested 3.69 shape.

## Non-negotiable rules

1. Identity from filename + lofty/ID3. Two-part `Artist - Title` does not invent an album.
2. `crates/vmp-vap` should converge on **3.69 nested pillars**, not a flattened 3.1 card.
3. Unmeasured fields are `null` / `unknown`. Do not mark them known to satisfy the schema UI.
4. Context engine (`GYM_PEAK`, `NIGHT_DRIVE`) requires numeric BPM, arousal, MET — see `VASP_Context_Simulation.md`.
5. Mixxx-derived Vinyl code stays a GPL symbiont; do not relicense Mixxx algorithms as MIT.
6. No secrets in git. `.env.example` is demo-only.

## Workspace

Rust workspace (`Cargo.toml`) members:

`vmp-vap` · `vmp-dsp` · `vmp-core` · `vmp-audio` · `vmp-v01d` · `vmp-cli` · `vmp-viz` · `vmp-vinyl` · `fute` · `apps/vmp-tauri/src-tauri`

UI: Vite 8 + React 19 (`package.json` name still `vibeaudio`). Desktop: Tauri.

## Protocol vs engine vs player

| Piece | Where | Truth |
| --- | --- | --- |
| Schema 3.69 | `../vasp/VASP_Official Schema.md`, `schema/` | Required nine pillars |
| Scoring reference | `VASP_Scoring Engine.py.md` | Python `generate_vap_profile` |
| Spotify mapper | `vasp_sdk-api_streaming.md` | `VapEngine::transform_spotify` (legacy input) |
| Golden fixtures | Cannibal / Céline / Stuca under `../vasp/` and copies here | Hand-authored, complete |
| Runtime types | `crates/vmp-vap` | Must not silently stay on 3.1 flat fields |
| DSP | `crates/vmp-dsp` + planned `vap-analyze` | Numbers for scoring |

The exec-sum tree (`engine/src/dsp_analyzer.py`, `parser-rs`) is the **intended** layout. This folder currently has the markdown/PDF reference plus Rust crates — not that Python package on disk.

## Why three goldens worked and web lookup failed

Cannibal Corpse, Céline Dion, and Stuca were scored (or authored) into full VASP objects and rendered as PDFs. Tech N9ne — The Waitress went through the **web player** lookup path (wiki/iTunes text → flattened pane). VMP’s job is the goldens’ path: decode → tags → analysis → scoring → sidecar.

## Streaming

- Spotify audio-features is not a future dependency.
- TIDAL app name for user consent: **Vibe Audio Player**. Add a desktop redirect later if VMP signs in directly.
- YouTube Premium does not grant Data API likes.

## Accessibility

Same invariant as VAP: readable chrome, high contrast, no assumption that a dense dark UI is usable.

## Related

- [README.md](README.md)
- [EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md)
- [../vibeaudioplayer/](../vibeaudioplayer/)
- [../vasp/](../vasp/)
