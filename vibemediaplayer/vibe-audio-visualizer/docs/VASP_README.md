---
type: overview
title: Vibe Audio Standard and Protocol
description: Human front door for VASP v3.69, the 9-pillar experiential audio metadata standard.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - SoulSync
domains:
  - audio
  - xessability
nodes:
cores:
fields:
---

# Vibe Audio Standard and Protocol (VASP)

**Aurphyx LLC** · **Version 3.69** · **Accessibility = Xessability**

This folder is the VASP volume: a 9-pillar metadata standard for the holographic identity of an audio file. ID3 captures bibliographic data (who / what). VASP captures experiential data (who / what / where / when / why). It uses Three-Squared-Lattice Cognitive Architecture (TSLCA). Retired name: V.A.P. → VASP.

Canonical source of truth: [`VASP_Intro_Specs.md`](VASP_Intro_Specs.md).

A complete VASP profile always includes `VAP_VERSION`, `IDENTITY` (`TITLE` and `ARTIST` required), and all nine pillar keys.

## Nine pillars

| Pillar | Canonical key | Archetype | Scope / purpose |
| ---: | --- | --- | --- |
| 1 | STRUCTURAL | The Skeleton | Mathematical, temporal, and dynamic architecture. |
| 2 | TONAL | The Flesh | Harmonic, melodic, and pitch-based content. |
| 3 | TIMBRAL | The Skin | Texture, fidelity, and sonic physics. |
| 4 | LINGUISTIC | The Voice | Semantic message and vocal delivery. |
| 5 | AFFECTIVE | The Heart | Psychological and emotional impact. |
| 6 | CONTEXTUAL | The Scene | Environmental and situational tagging. |
| 7 | PHOTOMETRIC | The Eye | Visual data for lighting and syncing. |
| 8 | KINETIC | The Body | Biometric and physical response data. |
| 9 | GENEALOGICAL | The Roots | Historical lineage, sampling, and cultural tribe. |

```json
["STRUCTURAL", "TONAL", "TIMBRAL", "LINGUISTIC", "AFFECTIVE", "CONTEXTUAL", "PHOTOMETRIC", "KINETIC", "GENEALOGICAL"]
```

Active protocol version is **3.69**. Schema is JSON Schema Draft-07 with this URI unaltered:

```json
"$schema": "http://json-schema.org/draft-07/schema#"
```

## Documents in this folder

| File | Role |
| --- | --- |
| [`VASP_Intro_Specs.md`](VASP_Intro_Specs.md) | Canonical 9-pillar taxonomy. |
| [`VASP_Official Schema.md`](VASP_Official%20Schema.md) | Draft-07 schema. `VAP_VERSION` const `"3.69"`. All nine pillars required. |
| [`VASP_Logic Architecture.md`](VASP_Logic%20Architecture.md) | Phase I DSP / Phase II NLP & ML / Phase III I/O scoring. Sections 1–9 = pillars 1–9. |
| [`VASP_Scoring Engine.py.md`](VASP_Scoring%20Engine.py.md) | Reference engine (`VASPScoringEngine`). Extract the Python fence to run. |
| [`VASP_Context_Simulation.md`](VASP_Context_Simulation.md) | Contextual override simulation. Complete 9-pillar example tracks. |
| [`VASP_Test_Cannibal Corpse.md`](VASP_Test_Cannibal%20Corpse.md) | Authored complete 9-pillar fixture (Cannibal Corpse — *Inhumane Harvest*). |
| [`VASP_TechSpec_Manual.md`](VASP_TechSpec_Manual.md) | Technical specification and implementation notes. |
| [`VASP_ExecSum_ProjContext.md`](VASP_ExecSum_ProjContext.md) | Executive summary and project context. |

Filenames with spaces are the real names. Do not rename them.

## What not to touch

- Any PDF in this folder. Legacy V.A.P. print materials stay as-is.
- Filenames.
- Existing APS headers on sibling Markdown files (except a title value when that title is inaccurate for that document).
- [`VASP_Intro_Specs.md`](VASP_Intro_Specs.md) body, except an obvious typo or truncated sentence.

Player, visualizer, and packaging code live outside this folder (`vibeplayer/`, `vap-serv/`, `vibeaudio/`). Do not treat those trees as the protocol definition.

## Historical, not active

These Markdown files are in the folder but are **not** the v3.69 contract:

| File | Status |
| --- | --- |
| [`vasp_sdk-api_streaming.md`](vasp_sdk-api_streaming.md) | Streaming/SDK notes still labeled **v3.3**. |
| [`vibe_audio_visualizer.md`](vibe_audio_visualizer.md) | Visualizer dump still labeled **v3.1**; some examples require only a subset of pillars. |

Quote those as historical. Do not implement new work against them.

## Extracting the scoring engine

[`VASP_Scoring Engine.py.md`](VASP_Scoring%20Engine.py.md) is a Markdown wrapper around one Python fence. The fenced body is valid Python. `generate_vap_profile()` emits all nine pillar keys. TONAL, LINGUISTIC, CONTEXTUAL, and GENEALOGICAL heuristics are deterministic: they use `raw_data` when present and emit `null`, `[]`, or `"unknown"` for missing catalog facts (samples, release date, cultural era, tribe alignment). They do not invent those facts.

## Folder overlays

This volume does not yet have `PROJECT_CONTEXT.md`, `PHYSICS.md`, or `INVARIANTS.md`. Do not invent their contents. Until they exist, lock protocol meaning to [`VASP_Intro_Specs.md`](VASP_Intro_Specs.md) and organism law in the repository `.cursorrules`.
