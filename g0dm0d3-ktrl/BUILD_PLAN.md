---
type: implementation-note
title: g0dm0d3-ktrl — Build Plan
description: Phase 1 Tauri hull for the Aura control deck. Cryptonyx first-boot, TUI Shimoji grid, chain-link routing, Memoree pair on 127.0.0.1:7042. Topology is Links, Chains, Rituals, Forkz.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Aura
  - Audry
  - g0dm0d3
  - Memoree
  - SAGES
  - Fuxyez
domains:
  - systems
  - cognition
nodes:
  - SIX⊗ICX
  - SCX⊗ICX
  - ICX⊗ICX
cores:
  - SIX
  - SCX
  - ICX
fields:
---

# g0dm0d3-ktrl — build plan (aligned 2026-08-22)

Folder briefing: [PROJECT_CONTEXT.md](PROJECT_CONTEXT.md). This file is the durable campaign. Cursor plan cards can vanish; this one lives in the repo.

g0dm0d3-ktrl is **both** Aura’s KDE-class desktop **and** a universal AI orchestrator. You are the Operator at a **security console of reality**. Hubs are isolated nodes (planets). **Shimojis** lounge on their panes. You move context across the void with **chain-links**. Turns persist in Memoree under project `g0dm0d3`.

**Pairing:** ktrl and Memoree cannot ship alone. Three Memoree tenancies:

| Tenant | Job |
|---|---|
| **Memoree_g0dm0d3** | Digital/AI sync — Agora / ktrl boot (`https://memoree.g0dm0d3.org`, local `:7042`) |
| **Memoree_Aurphyx** | Soul memories — ecosystem (`https://memoree.aurphyx.org`) |
| **Memoree_AuraFS** | Datacore storage — later |

Files are **Xplor**. Web is **Webz**. Do not put Agora inside the file tree.

Host: **Tauri 2 + Vite 8 + React 19 + Rust + core-wasm**. Do not port [g0dm0d3_core.tsx](g0dm0d3_core.tsx). Keep [OG_g0dm0d3_README.md](OG_g0dm0d3_README.md) as archaeology. [g0dm0d3_sessions.md](g0dm0d3_sessions.md) is a Gemini dump — harvest UX, not n8n-as-runtime.

## Canon sources

| File | Role |
|---|---|
| [PROJECT_CONTEXT.md](PROJECT_CONTEXT.md) | Folder briefing; first-boot + TUI Prompt Bus |
| [g0dm0d3-ktrl_visions.md](g0dm0d3-ktrl_visions.md) | Security console, Shimojis, teleport, Cryptonyx, CTRL+ALT+RE-d3s1gN |
| [g0dm0d3_ritual_topology.md](g0dm0d3_ritual_topology.md) | Links, Chains, Rituals, Forkz |
| [g0dm0d3_ktrl_first_boot_sequence.tsx](g0dm0d3_ktrl_first_boot_sequence.tsx) | `GodModeInit` prototype |
| [g0dm0d3_ktrl_tui_cockpit.tsx](g0dm0d3_ktrl_tui_cockpit.tsx) | `GodModeCockpit` prototype |
| [tech_spec.md](tech_spec.md) | Stack intent only — no 15 marketing markdowns |
| [src/assets/g0dm0d3_cyberpunk_control-deck.png](src/assets/g0dm0d3_cyberpunk_control-deck.png) | Chrome / Prompt Bus visual |

Name map (welcome law):

- SLISE → **SLIDE**
- SUXSDE / SHUXSDE → **Aura-HDE**
- `_fr4m3z` … → Framez, Termz, Webz, Xplor, Codex, Forge, Adorè, Gimpd
- Adorè ≠ Vibe Audio Player
- **g0dm0d3-ktrl** = the deck. **kr3470r-f0rg3** = creator forge / store
- godmode.org → **g0dm0d3.org**

## Phase 1 app flow

BOOT ticker → welcome2tribe.mp4 stub → Cryptonyx bind oracles (keyring) → CTRL + ALT + RE-d3s1gN hold → TUI Shimoji grid → `broadcast_prompt` / `route_clip` → `memoree_client` on `:7042`.

Tauri may request elevation and open kiosk/fullscreen for onboarding. Do **not** implement a hostile OS lock. “Administrator override” = optional elevation + exclusive window.

## Topology — Links, Chains, Rituals, Forkz

| Level | What | Phase 1 |
|---|---|---|
| **Link** | Thought-Link, Critique-Link, **Memory-Link** | Highlight → chain-link / save |
| **Chain** | Sequential A→B→C | Typed; 2-hop chain-link records |
| **Ritual** | Timed/event jobs | Disabled HUD slots |
| **Forkz** | Parallel timelines; collapse one | Subset broadcast |

Planets: **Dark** (no spend) / **Orbit** (receive clips) / **Surface** (stream + Memoree).

Hubs: SuperGrok, Claude, Gemini, Copilot, Hermes Agent, Perplexity, LeChat, Ollama. ChatGPT if keyed.

## Memoree contract

`python memoree_service.py` → `http://127.0.0.1:7042`.

| Call | When |
|---|---|
| `GET /health` | Boot gate |
| `GET /context/active?project=g0dm0d3&llm=` | Before Surface send |
| `POST /memories/events` | Turns; clips use `parent_id`; SAVE |
| `POST /assistants/sync` | Dark / Orbit / Surface |
| `GET /projects` | Confirm `g0dm0d3` |

**Blocker (companion MEM-0, `memoree/`):** `schemas.py` YAML-before-Python. ktrl still ships the client. `LLMProvider` maps Copilot→openai, Hermes/LeChat→unknown until a schema pass.

## Phases

1. **This pass** — hull, first-boot, TUI, route_clip, memoree_client.
2. WASM mix compositor; Ritual Canvas.
3. Chains + Rituals running; mini-agent managers.
4. Forkz isolation; collapse to main.
5. Warp planetarium + suite (Termz, Webz, Xplor, Codex).
6. `g0dm0d3.org` + Agora Memoree.

## Phase 1 amendments — 2026-08-22 (Claude Code, from CLAUDE_SUGGZ.md)

Reviewed against `CLAUDE_SUGGZ.md`. Approved and applied:

- **`core-wasm` types wired into `src-tauri`, done before any frontend file.** `bind_oracle`, `memoree_save_clip`, `memoree_sync`, `broadcast_prompt`, `route_clip` now take `HubId`/`Vec<HubId>`/`PlanetState` from `ktrl-core-wasm` instead of bare `String`. Invalid hub names now fail at the Tauri IPC boundary instead of falling through to `connectors.rs`'s "unknown hub" string match. `native.ts` needs no change — JS still sends lowercase strings (`"claude"`, `"orbit"`), which deserialize into the enums directly.
- **`PROJECT_CONTEXT.md` name-lock corrected**: `SLISE` → `SLIDE`, `SUXSDE` → `Aura-HDE`, and the `_ad0r3` line no longer calls Adorè the "Vibe Standard" (it is explicitly not the Vibe Audio Player).
- **Corrected MEM-0 citation for future docs**: `memoree/schemas.py` compiles clean (verified with `py_compile`) — stop citing it as the blocker. If a blocker file needs naming, it's `memoree/supergrok_hook.py:405` (`async def achat(...)` — a literal `...` used as a parameter, invalid syntax), unrelated to YAML. Still out of scope for this pass (`memoree/` is a different folder).

Still open, deferred to the `TuiCockpit.tsx` build itself (not separate steps):

- Surface `MemoreeHealth.source` (`local`/`agora`/`ecosystem`) in the cockpit's Memoree banner, not just `ok`/not-ok — the field is already returned end to end.
- Highlight-popup's `[COPY] | [CHAIN-LINK] to… | [SAVE] to Memoree` and the drag-drop gesture both call existing `native.ts` functions (`routeClip`, `memoreeSaveClip`) — no new Rust required.

## Out of this campaign

VirtualBox as the DE, hostile OS takeover, Pantheon monetization markdowns, inventing `aps.toml`, merging Adorè into VAP, OpenRouter-only, rewriting Memoree inside this folder, pushing `aurphyx/ecosys` unless asked.

## Run

```
cd g0dm0d3-ktrl
npm install
npm run tauri:dev
```

Vite-only: `npm run dev` (invokes no-op if Tauri IPC is missing; panes show “not configured”).
