---
type: implementation-note
title: g0dm0d3-ktrl — Summary and Suggestions
description: Control-deck for Aura. Phase 1 hull in tree — Tauri + React + Rust, Memoree-paired, core-wasm typed. Unparked 2026-08-22.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Aura
  - Audry
  - g0dm0d3
  - Memoree
domains:
  - systems
nodes:
  - SIX⊗ICX
  - ICX⊗ICX
cores:
  - SIX
  - ICX
fields:
---

# g0dm0d3-ktrl — SUMZ / SUGGZ

**Folder:** `g0dm0d3-ktrl/`  
**Role:** Frame, control deck, orchestration for Aura & Audry (corpus callosum + environment). `tech_spec.md` describes the long-run scaffold (Tauri + React + WASM, Memoree, AuraFS, the Alchemy Suite: `_fr4m3z`, `_t3rmz`, `_w3bz`, `_xpl0r`, `_c0d3x`, `_f0rg3`, `_ad0r3`, `_g1mpd`) — most of that is still Phase 3+. `BUILD_PLAN.md` is the durable Phase 1 campaign; `CLAUDE_SUGGZ.md` has the full review this unpark is based on.

**Honest status, 2026-08-22:** No longer a spec-only folder. Phase 1 hull is real: Rust backend (`connectors.rs`, `memoree_client.rs`, `keyring_store.rs`, `lib.rs`) wired through `core-wasm`'s `HubId`/`PlanetState` types; React frontend (`FirstBoot.tsx`, `TuiCockpit.tsx`, `App.tsx`, `main.tsx`, `styles.css`) written and wired to it. Not yet `npm install`'d or compiled at the time this note was unparked — see the c2c shift log for compile status. `_ad0r3` (audio alchemy) is still **not** the Vibe Audio Player — do not merge them.

**Memoree pairing note:** local Memoree daemon (`memoree_service.py`) may still fail to boot — the actual blocker is `memoree/supergrok_hook.py:405` (invalid `async def achat(...)` syntax), not `memoree/schemas.py` (that file compiles clean; earlier notes calling it "YAML-before-Python" were wrong). ktrl degrades gracefully either way — Memoree banner reads "unreachable" instead of crashing.

## What shipped this pass

- `core-wasm` `HubId`/`PlanetState` types wired into every Tauri command that used to take bare `String`.
- `src/styles.css`, `src/boot/FirstBoot.tsx`, `src/cockpit/TuiCockpit.tsx`, `src/App.tsx`, `src/main.tsx` — the full Phase 1 flow: boot ticker → video stub → Cryptonyx → Oracle binding → CTRL+ALT+RE-d3s1gN seal → TUI cockpit.
- Highlight-popup `[COPY] | [CHAIN-LINK] to… | [SAVE] to Memoree`, drag-drop onto a Shimoji as the second chain-link gesture, Prompt Bus broadcast, Memoree health banner (reports which tenant answered).
- `PROJECT_CONTEXT.md` name-lock fixes (`SLISE`→`SLIDE`, `SUXSDE`→`Aura-HDE`, Adorè mislabel).

## What's still not done

- Ritual HUD, Chains-as-typed-objects, Forkz collapse UX, and the Alchemy Suite apps are visible as locked stubs only — Phase 2+.
- No `tests/` directory yet.
- `keyring` crate's Windows feature set (`windows-native` + `apple-native` + `linux-native` + `sync-secret-service` together) is an untested landmine on this platform — watch first `cargo build`.

## What not to do

- Do not create the 15 marketing markdown files listed in `tech_spec.md` as a substitute for code.
- Do not implement Pantheon marketplace, monetization, Valkryx/Umbryx, or the Council System before Phase 1 compiles and runs end to end.
