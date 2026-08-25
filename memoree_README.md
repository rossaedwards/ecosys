---
type: sovereign-memory-substrate
title: Memoree - 3²-Lattice Cognitive Memory Substrate
description: Nine-pillar memory platform, universal API router, MCP/ACP/APC surface for models, services, and the Aurphyx ecosystem.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - mcp
  - acp
  - apc
  - hooks
  - chains
  - rituals
  - links
  - g0dm0d3
  - Audry
  - Aura
  - AuraFS
  - Fuxyez
  - SAGES
  - SoulSync
  - SoulShot
  - SoulCrypt
  - SoulKey
  - vibe-audio-visualizer
  - vibeplayer
  - suxs
  - gil
  - ineffable_ledger
  - gvs
  - opulence
  - p4a
  - blissid
  - adore
  - egophyx
  - chakra_datacore
  - duality_kernel
  - dataorb
  - aints
  - aethornyx
  - aurphyx_casino
  - vasp
  - tarot_deck
  - oracle_deck
  - thirteen_month_calendar
  - aurafs_devices
domains:
nodes:
cores:
fields:
---

## ** APS‑TSLCA-MEMOREE **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 3.69 **

# Memoree

**Nine-pillar memory platform · universal API router · MCP / ACP / APC**

Memoree is the memory substrate for the Aurphyx ecosystem and a general memory service for models, agents, and apps. It is **not** a physics simulator. It uses the **structural pillars** of TSLCA the same way VASP uses them for audio: three orthogonal concerns, nine stable memory kinds, one contracted context payload, and a single protocol surface.

Local path: `C:\rossaedwards\ecosys\memoree\`  
GitHub: `rossaedwards/ecosys` → `memoree/`

---

## What it is

| Role | Meaning |
|---|---|
| **9-pillar memory** | Typed storage and recall for models, services, and projects |
| **API router** | REST on `127.0.0.1:7042` plus MCP JSON-RPC for Cursor / LM Studio / Claude Desktop |
| **ACP / APC** | Agent and protocol control so every client speaks the same memory contract |
| **Context assembly** | Ranked, typed recall → one injectable payload (product “fusion”, not lattice arithmetic) |
| **Ecosystem registry** | `projects.json`, `dualities.json`, `invariants.json` |

Closest public foil: hierarchical systems such as MemoryOS. Memoree’s bet is **nine pillars + protocol surface + project registry**, not a six-layer personal OS alone.

---

## Nine pillars (canonical)

| Pillar | Job |
|---|---|
| **Working** | Active context, open loops, uncured session buffer |
| **Episodic** | Conversation turns, session-bound interactions |
| **Semantic** | Project knowledge, facts, relationships, dualities |
| **Procedural** | Workflows, task recipes, automation sequences |
| **Meta** | Verified facts, confidence-tracked beliefs, axioms |
| **Quantum** | Physics / simulation state, lattice snapshots, coherence logs |
| **Sensory** | Perception traces, embodiment, xessability transforms |
| **Identity** | Continuity, provenance tags, self-consistency across renewal |
| **Governance** | Votes, policy, guardian mandates, ledger entries |

Structural alignment to TSLCA cores (not equation runtime):

- **SIX** — Sensory, Working, Episodic (contact / embodiment / session)
- **SCX** — Semantic, Meta, Procedural, Quantum (meaning / structure / process)
- **ICX** — Identity, Governance (continuity / authority)

Fusion operator name in the stack: **SUXS-IFO**. Canon cores: **SIX / SCX / ICX** only. Retired tokens (SIC, SCC, ICC, USAIC) may appear only as quoted historical maps.

Creative content is a **medium / status flag** on Semantic, Sensory, or Identity — not a tenth pillar.

---

## Stack (this repo)

| File | Role |
|---|---|
| `schemas.py` | Pydantic models for all pillars + context / query / diagnostics |
| `memory_engine.py` | Orchestrates vector backend, optional Memori mirror, context assembly |
| `vector_backend.py` | Qdrant / Chroma persistence |
| `routes.py` | FastAPI routes + MCP endpoint |
| `memoree_service.py` | Uvicorn entry (port **7042**) |
| `memori_bridge.py` | Optional mirror (legacy; may be disabled) |
| `aurafs_backend.py` | Stub — disabled until AuraFS integration |
| `*_hook.py` | Thin clients (Perplexity, SuperGrok, Gemini, LM Studio) |
| `projects.json` | Project registry |
| `dualities.json` | Duality pairs |
| `invariants.json` | Hard facts / axioms the platform knows |
| `config.yaml` | Backend and mirror flags |
| `heartbeat.py` | Optional pulse (ingest → embed → summarize → check) |

**Do not treat as source of truth:** any `*.py.md` twin, `aurphyx_memori.py` (old GibsonAI Memori), stale tree dumps.

---

## Protocol surface (current)

```
GET  /health
GET  /diagnostics
GET  /projects
GET  /projects/{key}

POST /memories/events          → Episodic
POST /memories/semantic
POST /memories/procedural
POST /memories/meta
POST /memories/quantum
POST /memories/creative        → demote toward medium flag over time
POST /memories/governance
POST /memories/upsert
POST /memories/bulk

GET  /context/active
GET  /stream/context           → SSE
POST /query

POST /mcp                      → JSON-RPC 2.0 (tools: health, get_context, list_projects, diagnostics)
```

**Upgrade targets (not all implemented yet):**

- `POST /memories/working | sensory | identity`
- Nine collections in the vector backend matching the nine pillars
- Context payload always keyed by pillar, not by legacy seven-type bag only

---

## Naming lock (read this before editing)

| Current | Retired (quote only) |
|---|---|
| SIX | SIC |
| SCX | SCC |
| ICX | ICC |
| SUXS-IFO | USAIC |

Paths and remotes:

- Local: `C:\rossaedwards\ecosys\`, `C:\aurphyx\ecosys\`
- GitHub: `rossaedwards/ecosys`, `aurphyx/ecosys`
- No `.../main/` folder name

Owner: Aurphyx LLC · ORCiD 0009-0008-0539-1289 · EIN 41-3437055

---

## Quick start

```powershell
cd C:\rossaedwards\ecosys\memoree
# Qdrant (example)
docker run -d --name memoree-qdrant -p 6333:6333 qdrant/qdrant
pip install -r requirements.txt
python memoree_service.py
# → http://127.0.0.1:7042/health
```

---

## Design rules for contributors and agents

1. Memoree is a **memory platform + router**, not a TSLCA equation engine. Leave lattice numerics in `tslca/`.
2. Prefer extending `schemas.py` and `routes.py` over inventing parallel stores.
3. Context assembly returns a **contracted, pillar-keyed payload** suitable for system prompts and MCP tools.
4. AuraFS stays stubbed until explicitly enabled.
5. Do not reintroduce SIC / SCC / ICC / USAIC as live scientific names.
6. User may add Vibe-OKF frontmatter and APS headers manually; do not strip them.

---

*f0rg3d in l0v3 · Aurphyx LLC · 2026*
