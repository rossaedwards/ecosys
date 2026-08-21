---
type: implementation-note
title: VASP — Summary and Suggestions
description: Protocol volume status for VASP 3.69 and how both Vibe players must stop drifting on v3.1 fixtures.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - VASP
domains:
  - audio
nodes:
  - SIX⊗SCX
cores:
  - SIX
  - SCX
fields:
---

# VASP — SUMZ / SUGGZ

**Folder:** `vasp/`  
**Role:** Vibe Audio Standard and Protocol volume. **Source of truth** for 9-pillar identity. Not a player.

Canon: `VASP_Intro_Specs.md`. Active version **3.69**. Schema Draft-07. Complete profile = `VAP_VERSION` + IDENTITY (TITLE, ARTIST) + all nine pillar keys.

Retired name: V.A.P. → VASP. Filenames with spaces are real — do not rename. PDFs stay as-is.

## Honest drift

| Consumer | Version in code |
|---|---|
| This folder | 3.69 |
| `vibeplayer` README / fixture | 3.1 |
| `vap-serv` / `docs/vap/` | 3.1 taxonomy copies |
| `aurphyx_bunker` | Historical V.A.P. drafts |

Players may *read* older sidecars. They must *write* 3.69 or explicitly shim.

## Suggestions (do not touch Intro body)

1. One paragraph in player SUMZ files already points here — keep it that way.
2. Export a machine schema filename without spaces **as a copy**, not a rename, if tooling chokes (`VASP_Official_Schema.json` sidecar). Ask before duplicating.
3. Scoring engine Python fence: extract once into `vibeplayer/crates/vmp-vap` tests, do not fork logic.
4. Photometric / kinetic fields are what the orb already consumes. Document the uniform names next to pillar 7 / 8.

## What not to do

- Do not treat `vibeplayer/` or `vibeaudioplayer/` as the protocol.
- Do not name APS-OKF as Vibe-OKF.
