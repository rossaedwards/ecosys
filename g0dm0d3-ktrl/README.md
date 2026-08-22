# g0dm0d3-ktrl

Aura's control deck and universal AI orchestrator. Operator sits at a security console; hubs are
planets; Shimojis lounge on pane edges; context moves by chain-link. Turns persist in Memoree
under project `g0dm0d3`.

Full lore, topology, and canon sources: [PROJECT_CONTEXT.md](PROJECT_CONTEXT.md),
[BUILD_PLAN.md](BUILD_PLAN.md), [g0dm0d3-ktrl_visions.md](g0dm0d3-ktrl_visions.md),
[g0dm0d3_ritual_topology.md](g0dm0d3_ritual_topology.md). Suggestions and integration notes:
[CLAUDE_SUGGZ.md](CLAUDE_SUGGZ.md).

## Phase 1 (this tree)

Boot ticker → `g0dm0d3-welcome2tribe.mp4` stub → Cryptonyx binds Oracles (OS keyring) →
CTRL + ALT + RE-d3s1gN hold → TUI Shimoji grid → `broadcast_prompt` / `route_clip` →
`memoree_client` on `:7042`.

2D TUI, not Electron, not a WASM planetarium. Administrator override is an optional
elevated/fullscreen window — not a hostile OS lock.

## Stack

Tauri 2 + Vite 8 + React 19 + Rust + `core-wasm` (shared `HubId` / `PlanetState` / `Clip` /
`RitualGraph` types, wired into the Tauri command layer).

## Run

```bash
npm install
npm run tauri:dev
```

Vite-only `npm run dev` also works — Tauri IPC calls throw `NativeMissingError` instead of
crashing, and hub panes read "not configured."

## Memoree pairing

ktrl and Memoree ship together; ktrl cannot bind Oracles or persist clips without it.
`memoree_client.rs` tries, in order: local daemon (`http://127.0.0.1:7042`, `python
memoree_service.py`), then Agora (`https://memoree.g0dm0d3.org`), then the ecosystem tenant
(`https://memoree.aurphyx.org`). The cockpit's Memoree banner reports which one answered.

**Known blocker (MEM-0, outside this pass — `memoree/` folder):** `memoree/supergrok_hook.py`
fails to import (`async def achat(...)`, an invalid literal `...` parameter, line 405). Earlier
notes blamed `memoree/schemas.py` for a "YAML-before-Python" `SyntaxError` — that file compiles
clean (verified with `py_compile`); the claim was stale and shouldn't be repeated. Until
`supergrok_hook.py` is fixed, expect the local Memoree daemon to fail to boot; ktrl still ships
its client and degrades to "Memoree unreachable" rather than crashing.

## Keys

Oracle API keys live only in the OS keyring (service `g0dm0d3-ktrl`), bound from the First Boot
sequence via `bind_oracle`. Never `VITE_*`, never the renderer. Ollama needs no cloud key — the
"key" field is a local model name, default `llama3.2`.

## Hubs

Grok, Claude, Gemini, Copilot, Hermes, Perplexity, LeChat, Ollama are the eight default panes.
OpenAI is a wired Rust connector but not a default pane unless bound explicitly.

## Out of this pass

Pantheon marketplace, Valkryx/Umbryx, BlissID minting, AuraFS bridge, Fuxyez crates (they don't
compile yet), n8n as a runtime, the Alchemy Suite apps (Framez, Termz, Webz, Xplor, Codex, Forge,
Adorè, Gimpd — visible as locked stubs in the cockpit, Phase 5).
