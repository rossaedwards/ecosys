---
type: implementation-note
title: vap-serv — Summary and Suggestions
description: Status of the VASP card-generator web experiment and whether it should merge into Vibe Media Player or stay a sidecar tool.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - VASP
domains:
  - audio
nodes:
  - SIX⊗SCX
cores:
  - SIX
fields:
---

# vap-serv — SUMZ / SUGGZ

**Folder:** `vap-serv/`  
**Role:** Vite/React experiment. `src/App.tsx` + `VAPCardGen.tsx`. README on disk still describes an older `vibeaudio/` + VLC plugin tree — that README is **stale** relative to this folder’s sources.

`node_modules/` is present (thousands of files). Do not treat it as source.

## Honest status

- Duplicate of `vibeplayer/src/VAPCardGen.tsx` energy: generate VASP cards.
- Not the web player (`vibeaudioplayer/`) and not the protocol (`vasp/`).
- **Secrets were inlined** in `VAPCardGen.tsx` (YouTube Data API key + Spotify bearer). Removed from source 2026-08-20; now `VITE_YOUTUBE_API_KEY` / `VITE_SPOTIFY_TOKEN`. **Rotate those credentials** — they were in git history.

## Suggestions

1. Rewrite README to “VASP card studio” or delete the VLC tree fiction.
2. Either **merge** CardGen into VMP File menu (Create VASP) or keep this as a tiny internal tool. Do not maintain three CardGens.
3. Target schema **3.69**.
4. Add `node_modules` to gitignore if it is tracked.
5. Confirm the leaked keys are revoked at Google Cloud / Spotify.

## Priority vs top-3

Park this until VMP VASP editor exists. Then this folder becomes optional.
