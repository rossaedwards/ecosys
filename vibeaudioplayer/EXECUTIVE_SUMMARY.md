# Vibe Audio Player — Executive Summary

**Aurphyx LLC** · 2026-08-21 · VASP 3.69

## One sentence

A web music player whose chrome is Framez and whose catalog is a 9-pillar Vibe Audio profile — identity from files and tags, experience from measurement, never from a scraped artist bio.

## Problem

Players still treat a track as title / artist / album. Mood, tempo, lighting, body-load, and lineage are either missing or hallucinated. A Wikipedia artist page is not lyrics, key, or valence. Context engines (GYM_PEAK, NIGHT_DRIVE) cannot run on slogans; they need `BPM_RAW`, `AROUSAL`, and `MET_SCORE`.

## Solution

Vibe Audio Player plays local files and pinned streaming shelves (TIDAL, YouTube, YouTube Music, Loved) and attaches a **VASP 3.69** object:

Structural · Tonal · Timbral · Linguistic · Affective · Contextual · Photometric · Kinetic · Genealogical

Pipeline: filename/ID3 → catalog facts → DSP / scoring engine → user–pro–tribe overrides. Unknown stays unknown.

## What exists today

- Playable web shell on port 8080 with Framez windows, library pins, VASP pane, Google sign-in via Grok broker.
- Public catalog lookup (MusicBrainz, iTunes, Deezer, AudioDB, Wikipedia).
- Loved shelf schema and TIDAL PKCE callback.
- TIDAL developer app **Vibe Audio Player** (credentials local-only; redirect and scopes still to be saved in the dashboard).
- Golden 3.69 fixtures and the reference `VASPScoringEngine` / `VapEngine` under `vasp/`.

## What does not exist yet

- Honest nested-pillar store replacing flattened lookup text.
- First-class ID3 + analysis services as `known` inputs.
- YouTube likes pull (Premium ≠ Data API).
- Verified Windows 11 clone-and-run by the owner.

## Relationship to Vibe Media Player

VAP is the browser / Framez product. VMP (`vibeplayer/`) is the Rust + Tauri host (decode, sidecar `.vap.json`, Vinyl Vibez). Both speak VASP. Neither is the protocol itself — that is `vasp/`.

## Ask of the next session

1. Finish TIDAL dashboard redirect + scopes, then sign-in on localhost.  
2. Wire identity-only parse + unknown defaults.  
3. Feed scoring-engine numbers into the pane so context overrides can run.
