---
type: implementation-note
title: Cursor ↔ Claude Code employee sync
description: Standing handoff protocol between Cursor Grok (Audry) and Claude Code inside rossaedwards/ecosys. Newest shift is always at the top. Do not treat this file as physics or as a product README.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
  - g0dm0d3
  - Memoree
domains:
  - systems
nodes:
  - ICX⊗ICX
cores:
  - ICX
fields:
---

# c2c employee-sync

This file is how **Cursor Grok 4.6** (Audry, in Cursor) and **Claude Code** hand a live shift to each other inside `rossaedwards/ecosys`.

Ross interrupted Cursor because the Cursor Pro Agent quota ran out. Claude Code is on duty now.

Do not edit Cursor plan files under `C:\Users\owner\.cursor\plans\`. Campaign truth for this job lives in `g0dm0d3-ktrl/BUILD_PLAN.md`.

---

## Protocol (both of us)

1. Read this file before touching code.
2. Read the folder `PROJECT_CONTEXT.md` if it exists. Then `.cursorrules`. Then the file you will change.
3. **One folder per pass.** Current pass is `g0dm0d3-ktrl/` only. Do not rewrite `memoree/` in this pass. Flag `schemas.py` YAML-before-Python; do not fix it here.
4. Edit in place. No parallel trees. No `aps.toml` invention. Do not push `aurphyx/ecosys` unless Ross asks. Do not commit unless Ross asks.
5. When your shift ends, **prepend** a new `## SHIFT` block under Current board (newest first). Update the board. Leave the next agent a first command, not a vibe.
6. If you complete the campaign, mark the board `DONE` and list files touched.
7. Speak as Audry: exact, loyal, no condescension, no therapy voice. Frameworks are canon, not myth.
8. Name locks: SLIDE not SLISE; Aura-HDE not SUXSDE/SHUXSDE; Adorè ≠ Vibe Audio Player; Xplor ≠ Memoree ≠ Webz; deck = **g0dm0d3-ktrl**; creator store = **kr3470r-f0rg3**; video app = **Forge** / `f0rg3`; `godmode.org` → **g0dm0d3.org**.

### Invoke contract

Tauri 2 maps JS camelCase → Rust snake_case. Keep that pairing.

| JS (`src/native.ts`) | Rust (`src-tauri/src/lib.rs`) |
|---|---|
| `bindOracle(hub, key)` | `bind_oracle` |
| `oracleStatus()` | `oracle_status` |
| `setKiosk(on)` | `set_kiosk` |
| `memoreeHealth()` | `memoree_health` |
| `memoreeSaveClip(text, sourceHub, parentId?)` | `memoree_save_clip` |
| `broadcastPrompt(prompt, targets)` | `broadcast_prompt` |
| `routeClip(clip, sourceHub, targets, extraPrompt?)` | `route_clip` |

Vite-only `npm run dev` must not crash if Tauri IPC is missing. `src/native.ts` already throws `NativeMissingError`.

---

## Current board — 2026-08-22 ~08:00 EDT

| Field | Value |
|---|---|
| Status | **DONE** — Phase 1 hull complete, compiled, boot-to-cockpit flow verified. |
| Campaign | g0dm0d3-ktrl Phase 1 hull |
| Execute folder | `g0dm0d3-ktrl/` only |
| Durable plan | `g0dm0d3-ktrl/BUILD_PLAN.md` (amended this shift, see below) |
| Suggestions | `g0dm0d3-ktrl/CLAUDE_SUGGZ.md` — full review, read before the next pass |
| Host | Windows 11, `C:\rossaedwards\ecosys`, PowerShell |
| Persona | Audry |

Compiled: **yes**. `cargo check` / `cargo build` clean, `tsc -b` clean, `vite build` clean, `npm run tauri:dev` launches `g0dm0d3-ktrl.exe` with no panic. Boot → seal → cockpit verified end to end via the Vite-only preview (Tauri IPC absent there by design; panes correctly read "not configured" / "route_clip failed" instead of crashing).

---

## SHIFT — 2026-08-22 ~08:00 EDT — Claude Code

### What was asked

Explore the repo (welcome doc, `tslca/`, this file, `g0dm0d3-ktrl/`), write `CLAUDE_SUGGZ.md`, propose a build-plan revision, get sign-off, then execute it.

### What shipped

- `core-wasm`'s `HubId`/`PlanetState` wired into every `src-tauri` command that took bare `String` (`bind_oracle`, `memoree_save_clip`, `memoree_sync`, `broadcast_prompt`, `route_clip`). Added `PlanetState::as_str()`.
- Fixed a real, unrelated compile bug in `src-tauri/src/lib.rs::run()`: a local `let session_id = ...` was shadowing the `session_id` Tauri command, breaking `generate_handler!` (E0618). Renamed to `new_session_id`.
- `src/styles.css`, `src/boot/FirstBoot.tsx`, `src/cockpit/TuiCockpit.tsx`, `src/App.tsx`, `src/main.tsx` — full Phase 1 flow, no Tailwind, no `dangerouslySetInnerHTML`. Boot ticker copy has no hostile-OS-lock language; "administrator override" is `setKiosk(true)`/`setKiosk(false)`, best-effort. `THE_RITUAL` persists `localStorage['g0dm0d3-ktrl.sealed']`.
- `TuiCockpit.tsx`: 8-hub 3-col grid, Shimoji idle-float/dangle-legs/zip-out/zip-in/dance, highlight popup `[COPY] | [CHAIN-LINK] to… | [SAVE] to Memoree`, drag-drop onto a Shimoji as the second chain-link gesture (both paths share one `chainLink()` call), Prompt Bus broadcast, Memoree banner surfacing `health.source`. Ritual HUD shown as a locked chip (`RITUAL ○`), not hidden. Suite dock stubs present, non-clickable.
- `README.md` written; `SUMZ-SUGGZ.md` unparked with an honest status.
- `PROJECT_CONTEXT.md` name-lock fixed (`SLISE`→`SLIDE`, `SUXSDE`→`Aura-HDE`, Adorè mislabel).
- `.claude/launch.json` (repo root) added for `g0dm0d3-ktrl-web` preview (`npm --prefix g0dm0d3-ktrl run dev`) — a dev convenience, not part of the app.
- Added `@types/node` devDependency — `vite.config.ts` used `process` with no types, `tsc -b` was red before this.

### Corrected, not fixed (out of scope — different folder)

`memoree/schemas.py` compiles clean; the "YAML-before-Python" blocker claim in the old `BUILD_PLAN.md`/`SUMZ-SUGGZ.md`/`memoree_client.rs` message was wrong and shouldn't propagate further. The real syntax error is `memoree/supergrok_hook.py:405` (`async def achat(...)`, literal `...` as a parameter). Not touched — one-folder-per-pass.

### Remaining compile errors

None. `cargo check`, `cargo build`, `tsc -b`, `vite build`, `npm run tauri:dev` all clean.

### Files touched

New: `src/styles.css`, `src/boot/FirstBoot.tsx`, `src/cockpit/TuiCockpit.tsx`, `src/App.tsx`, `src/main.tsx`, `README.md`, `CLAUDE_SUGGZ.md`, `.claude/launch.json` (repo root).
Modified: `src-tauri/src/lib.rs`, `core-wasm/src/lib.rs`, `PROJECT_CONTEXT.md`, `BUILD_PLAN.md`, `SUMZ-SUGGZ.md`, `package.json`/`package-lock.json` (added `@types/node`).

### Next up (Phase 2, not this pass)

Chains-as-typed-objects, Forkz collapse UX, Ritual HUD activation, the Alchemy Suite apps. See `CLAUDE_SUGGZ.md` §3–4 for sequencing notes.

---

## SHIFT — 2026-08-22 — Cursor Grok 4.6 → Claude Code

### What Ross asked

Implement the attached ktrl build plan. Do not edit the plan file. Mark existing todos in_progress as you go. Do not stop until all todos are complete.

Ross then hit Cursor Pro Agent limit and asked for this sync file so Claude Code can finish.

### What the product is

g0dm0d3-ktrl is **both** Aura’s KDE-class desktop **and** a universal AI orchestrator. Operator sits at a security console. Hubs are planets. Shimojis lounge on pane edges. Context moves by **chain-link**. Turns persist in Memoree under project `g0dm0d3`.

Phase 1 surface is **2D TUI** after Cryptonyx first-boot. Not Electron. Not a WASM FPS planetarium. Not `g0dm0d3_core.tsx` (emerald Bliss, keys in renderer — archaeology only).

Done when: `npm run tauri:dev` runs boot → seal → TUI grid; highlight Grok → chain-link Claude → Shimoji zip-out/zip-in and Claude **actually replies** (or pane says `not configured`); COPY dances; SAVE attempts Memoree; banner honest if `:7042` down.

### Todos (Cursor list — do not recreate)

| id | status | notes |
|---|---|---|
| `write-build-plan` | **completed** | `g0dm0d3-ktrl/BUILD_PLAN.md` on disk |
| `scaffold-tauri` | **in_progress** | hull files written; not installed/compiled; missing `src/main.tsx` so Vite has nothing to mount |
| `first-boot` | pending | port `g0dm0d3_ktrl_first_boot_sequence.tsx` → `src/boot/FirstBoot.tsx` |
| `tui-cockpit` | pending | port `g0dm0d3_ktrl_tui_cockpit.tsx` → `src/cockpit/TuiCockpit.tsx` |
| `chain-link` | pending | highlight menu COPY / CHAIN-LINK / SAVE; real `route_clip`; no fake `>> RITUAL PAYLOAD` |
| `rust-broadcast` | **code written, uncompiled** | `src-tauri/src/connectors.rs` + commands in `lib.rs` |
| `memoree-pair` | **code written, uncompiled** | `src-tauri/src/memoree_client.rs` |
| `readme-sumz` | pending | new `g0dm0d3-ktrl/README.md`; keep `OG_g0dm0d3_README.md`; unpark `SUMZ-SUGGZ.md` |
| `hull-ui` | **cancelled** | superseded by `tui-cockpit` |
| `clip-router` | **cancelled** | superseded by `chain-link` |

### On disk (written this shift)

Scaffold:

- `g0dm0d3-ktrl/package.json` — React 19.2, Vite 8, Tauri 2 CLI, no Electron, no Tailwind (use a real CSS file; prototypes used Tailwind + `dangerouslySetInnerHTML` — **move CSS out**)
- `g0dm0d3-ktrl/vite.config.ts` — port **1420** (not VMP’s 5173), `strictPort`, ignores `src-tauri` / `core-wasm`
- `g0dm0d3-ktrl/index.html` → `/src/main.tsx` (**file does not exist yet**)
- `g0dm0d3-ktrl/tsconfig.json` + `tsconfig.app.json` + `tsconfig.node.json`
- `g0dm0d3-ktrl/.gitignore`
- `g0dm0d3-ktrl/public/chrome/g0dm0d3-control-deck.png` (copy of `src/assets/g0dm0d3_cyberpunk_control-deck.png`)
- `g0dm0d3-ktrl/src-tauri/` Tauri 2 crate `g0dm0d3-ktrl`, lib name `g0dm0d3_ktrl_lib`, identifier `org.aurphyx.g0dm0d3-ktrl`
- Icons copied from VMP (`32x32`, `128x128`, `icon.png/ico/icns`, plus `icon.png` duplicated as `henry.w@example.net`)
- `g0dm0d3-ktrl/core-wasm/` — shared types (`HubId`, `Clip`, `PlanetState`, `RitualGraph`). Path dep from src-tauri. No wasm-bindgen yet. Fine.
- **Deleted** `g0dm0d3-ktrl/Cargo.toml` workspace on purpose so `tauri dev` owns the cargo root at `src-tauri/`. Do not put a parent workspace back.

Frontend glue (no UI yet):

- `g0dm0d3-ktrl/src/hubs.ts` — 8 hubs: grok, claude, gemini, copilot, hermes, perplexity, lechat, ollama. Suite stub names. ChatGPT/openai is a Rust hub but not a default pane (only if keyed later).
- `g0dm0d3-ktrl/src/native.ts` — invoke wrapper + typed commands
- `g0dm0d3-ktrl/src/vite-env.d.ts`

Rust (uncompiled):

- `src-tauri/src/main.rs` → `g0dm0d3_ktrl_lib::run()`
- `src-tauri/src/lib.rs` — commands listed above + `session_id`, `bound_map`, `memoree_projects`, `memoree_sync`
- `src-tauri/src/keyring_store.rs` — service name `g0dm0d3-ktrl`; never `VITE_*`
- `src-tauri/src/connectors.rs` — grok/xAI, claude, gemini, copilot (sk- → OpenAI else GitHub Models), hermes/OpenRouter, perplexity, lechat/Mistral, ollama `:11434`, openai
- `src-tauri/src/memoree_client.rs` — local `http://127.0.0.1:7042` then Agora `https://memoree.g0dm0d3.org` then `https://memoree.aurphyx.org`
- `src-tauri/capabilities/default.json` — `core:default` + fullscreen/focus
- `src-tauri/tauri.conf.json` — `devUrl` `http://localhost:1420`

### Missing — write these next, in this order

1. **`src/styles.css`** — scanlines, glitch, shimoji idle-float / dangle-legs / zip-out / zip-in / dance-on-copy. Harvest from the two prototype TSX `<style dangerouslySetInnerHTML>` blocks. No innerHTML in production.
2. **`src/boot/FirstBoot.tsx`** from `g0dm0d3_ktrl_first_boot_sequence.tsx`.
   - Phases: BOOT → VIDEO_INTRO → CRYPTONYX_TUTORIAL → API_BINDING → THE_RITUAL → then **call `onSealed()`**. Do **not** keep the dummy Prompt Bus panels in that file.
   - Boot ticker copy: flavor OK, but do **not** claim a hostile OS lock or privilege bypass. “Administrator override” = optional exclusive/fullscreen window via `setKiosk(true)` then `false` after seal.
   - VIDEO_INTRO: stub title card `[ PLAYING: g0dm0d3-welcome2tribe.mp4 ]` — no video file this phase.
   - API_BINDING: real password fields → `bindOracle`. Do not fake-toggle bound. Operator may proceed with zero or more oracles (unbound panes = `not configured`). Ollama: model name in keyring, default `llama3.2`; no cloud key.
   - THE_RITUAL: CTRL 5s, ALT 5s, RE-d3s1gN 5s (prototype: +2% / 100ms = 5s). After seal, persist `localStorage['g0dm0d3-ktrl.sealed'] = '1'` so later launches can skip to cockpit.
3. **`src/cockpit/TuiCockpit.tsx`** from `g0dm0d3_ktrl_tui_cockpit.tsx`.
   - Header chrome: **g0dm0d3-ktrl** (not `kr3470r-f0rg3 :: ktrl_deck`). Marionette PNG `/chrome/g0dm0d3-control-deck.png` allowed.
   - Prompt Bus: `g0d@m0d3:~#` → `broadcastPrompt` to Surface/bound hubs.
   - 3-col wrapping grid of 8 planets. Shimoji on top edge, legs dangling.
   - Highlight → popup **[COPY] | [CHAIN-LINK] to… | [SAVE] to Memoree**.
   - COPY = clipboard + source Shimoji dance class.
   - CHAIN-LINK = source `zipping-out`, ghost `zipping-in` on target (~800ms), then `routeClip`, append **real** `HubReply.text` (or `not configured`). Clear ritual ~1600ms. **Do not** append fake `>> RITUAL PAYLOAD`.
   - SAVE = `memoreeSaveClip` (Memory-Link).
   - Drag-drop onto a Shimoji = second gesture for the same `route_clip`.
   - Memoree banner if health.ok is false.
   - Ritual HUD slots disabled. Suite dock stubs: Framez Termz Webz Xplor Codex Forge Adorè Gimpd — Phase 5, not clickable products.
4. **`src/App.tsx`** — if sealed → TuiCockpit else FirstBoot.
5. **`src/main.tsx`** — mount App, import CSS. `index.html` already points here.
6. **`g0dm0d3-ktrl/README.md`** — APS-OKF, 2026, cite PROJECT_CONTEXT + BUILD_PLAN, run commands, Memoree pair, MEM-0 blocker.
7. **Unpark `g0dm0d3-ktrl/SUMZ-SUGGZ.md`** — no longer “park until Vibe”. Honest: Phase 1 hull in tree; Memoree daemon may not boot because `memoree/schemas.py` starts with YAML.

Then: `cd g0dm0d3-ktrl && npm install && npm run tauri:dev`. Fix compile errors. Do not leave a red `cargo` / tsc.

### First-boot + cockpit UX locks (from plan)

- After seal, **one** dashboard: TuiCockpit. Two fake dashboards = fail.
- Keys only in OS keyring.
- `LLMProvider` map until MEM-0: Copilot→`openai`, Hermes/LeChat→`unknown`. Grok→`supergrok`. Already in `memoree_llm_for_hub`.
- Subset broadcast = cheap Forkz. Do not import Fuxyez crates (they do not compile).
- Drop from tech_spec this phase: 15 marketing markdowns, Pantheon/Stripe, Valkryx/Umbryx, BlissID minting, AuraFS bridge, n8n as runtime.

### Known landmines (fix if they bite)

1. **`keyring` 3 features** in `src-tauri/Cargo.toml` enable windows + apple + linux + `sync-secret-service` together. If Windows compile explodes, drop to default features (`keyring = "3"`).
2. **`keyring::Error::NoEntry`** in `delete_key` — confirm variant name on keyring 3.x; adjust match if cargo complains.
3. **`broadcast_prompt` / `route_clip`** await Memoree context **per hub sequentially** before `join_all` of HTTP asks. Slow if `:7042` hangs — health timeout is 800ms local / 3s remote, but `context/active` is 12s. If it feels dead, fail open (already `Err(_) => None` for system preamble).
4. **Ollama `hub_configured` returns true always** so the pane attempts `:11434` instead of lying “not configured”. Connection refused is the honest Dark-planet signal.
5. **No `src/main.tsx`** — Vite will 404 until you add it. That is the first red screen if you `npm run dev` now.
6. Prototypes live at folder root (`g0dm0d3_ktrl_first_boot_sequence.tsx`, `g0dm0d3_ktrl_tui_cockpit.tsx`). Harvest, do not wire them as the app entry.
7. Folder `SUMZ-SUGGZ.md` still says park. README does not exist (`g0dm0d3-ktrl/README.md` 404).
8. Do not port `g0dm0d3_core.tsx`.
9. PROJECT_CONTEXT still says Adorè is “Vibe Standard” in one line — **flag, do not merge** Adorè into VAP.

### Canon files to read before coding UI

| File | Role |
|---|---|
| `g0dm0d3-ktrl/PROJECT_CONTEXT.md` | folder briefing |
| `g0dm0d3-ktrl/BUILD_PLAN.md` | durable campaign |
| `g0dm0d3-ktrl/g0dm0d3-ktrl_visions.md` | Shimojis, teleport, Cryptonyx, CTRL+ALT+RE-d3s1gN |
| `g0dm0d3-ktrl/g0dm0d3_ritual_topology.md` | Links, Chains, Rituals, Forkz |
| `g0dm0d3-ktrl/g0dm0d3_ktrl_first_boot_sequence.tsx` | GodModeInit prototype |
| `g0dm0d3-ktrl/g0dm0d3_ktrl_tui_cockpit.tsx` | GodModeCockpit prototype |
| `memoree/routes.py` | HTTP contract (do not edit this pass) |

### Out of campaign

VirtualBox as the DE, hostile OS takeover, Pantheon monetization markdowns, inventing `aps.toml`, merging Adorè into VAP, OpenRouter-only, rewriting Memoree engine, pushing `aurphyx/ecosys`, committing unless Ross asks.

### First command for Claude Code

```
cd C:\rossaedwards\ecosys\g0dm0d3-ktrl
```

Write `src/main.tsx`, `src/App.tsx`, `src/styles.css`, `src/boot/FirstBoot.tsx`, `src/cockpit/TuiCockpit.tsx`. Then README + unpark SUMZ. Then `npm install` and `npm run tauri:dev`. Fix until boot seals into the TUI.

When you stop, prepend a SHIFT block here with: compiled? y/n, remaining compile errors, files you added.

---

## Shift archive

_(empty — this is the first c2c handoff)_
