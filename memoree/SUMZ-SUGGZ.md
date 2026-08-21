---
type: implementation-note
title: Memoree — Summary and Suggestions
description: Status of the sovereign TSL memory daemon and a narrow plan so Vibe players can store last-skin, playlists, and session turns without waiting for the full Aura OS.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
  - AuraFS
domains:
  - cognition
  - systems
nodes:
  - SCX⊗SIX
cores:
  - SCX
  - SIX
fields:
  - cognitive-field-tensor
---

# Memoree — SUMZ / SUGGZ

**Folder:** `memoree/`  
**Role:** Three-Squared-Lattice memory substrate. FastAPI daemon on `127.0.0.1:7042`. Qdrant + SQLite WAL. Obsidian as a visual overlay. MCP JSON-RPC for Cursor / LM Studio.

**Gap:** `PROJECT_CONTEXT.md` missing. README claims Python 3.14+ / Fedora 44; this EliteBook is also Windows 11 — document both.

---

## What exists

| File | Job |
|---|---|
| `memoree_service.py` | Uvicorn + FastAPI lifespan. Heartbeat **disabled**. |
| `routes.py` | HTTP + SSE + MCP |
| `memory_engine.py` | Core engine |
| `vector_backend.py` | Qdrant |
| `schemas.py` | Types |
| `aurafs_backend.py` | AuraFS client stub |
| `powersync_client.py` | Sync client |
| `memoree_fs_server.py` | FS overlay |
| Hooks | `gemini_hook.py`, `lmstudio_hook.py`, `perplexity_hook.py`, `supergrok_hook.py` |
| Bridges | `memori_bridge.py`, `memos_overlay.py`, `aurphyx_memori.py` |

Nine TSL nodes in the README (Temporal … Lattice) are a **memory taxonomy**. They are not the OKF 3×3 and not VASP pillars. Keep the names in Memoree files; when writing APS-OKF headers, use SIX/SCX/ICX.

Headers in the README still show APS-SUXS-001 with `balance_coefficient`. Player code should not copy that header. New docs use APS-OKF nine keys only.

## Honest status

- **Current boot is broken.** `schemas.py` starts with a YAML block (`---` … `---`) before Python → `SyntaxError` on `import routes`. That YAML is also not a valid APS-OKF `type`.
- Daemon shape is real (FastAPI + MCP routes). Logs show historical starts on `:7042`. Heartbeat is stubbed.
- `memory_engine.py` constructs `VectorBackend(persist_dir=…, model_name=…)` but the backend takes `config_path`; upsert wants a `vector=` the engine never produces. No embedder wired.
- QUICKSTART is stale (`memoree.core.api` / port 8765). There is no `requirements.txt` or `scaffold_memoree.py` despite README claims.
- Qdrant data dirs exist in-tree — treat as local state, not canon.
- AuraFS backend disabled. Path comments still say `c:\memoree\`.

## What Vibe needs (tiny)

| Memory node | Vibe use |
|---|---|
| Temporal | Session turns: last track, last position, last Scene |
| Operational | Recipes: “open last playlist”, “apply Sodium skin” |
| Entity | One local listener profile (not SoulKey hardware) |
| Relational | Optional: album/artist facts if VASP Genealogical is sparse |

Do not send audio bytes to Memoree. Store paths, VASP JSON, skin id.

## Phased series

### MEM-0

1. Move the YAML header out of `schemas.py` (docstring or sibling `.md`) so `python memoree_service.py` starts.
2. Reconcile engine ↔ `VectorBackend` constructor + add an embedder before upsert.
3. Add `requirements.txt`. Align README/QUICKSTART to port **7042** and this entrypoint.
4. Enable heartbeat **or** delete the tease.

### MEM-1 (players)

1. VMP + VAP write `{skin, playlist, position}` to Temporal.
2. No cloud. 127.0.0.1 only.

### MEM-2

AuraFS backend on AFS-0. Hooks stay optional.

## What not to do

- Do not make the web player require Memoree for v1.
- Do not store secrets or session recordings.
- Do not confuse Memoree’s 9 memory domains with VASP’s 9 pillars.
