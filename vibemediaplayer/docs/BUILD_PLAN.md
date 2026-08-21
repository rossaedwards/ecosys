# Vibe Media Player — Living Build Plan

## Vision

Paradigm-shifting **experiential** media player: V.A.P. v3.1 is the identity layer; UI is modular; host is symbiotic via **v01d**.

## Stack status

| Layer | Status |
|-------|--------|
| `vmp-vap` schema/scoring/context | ✅ |
| `vmp-dsp` multi-mode EQ | ✅ |
| `vmp-audio` Symphonia decode + lofty tags + PlayerEngine | ✅ |
| `vmp-core` File menu, modules, Vinyl | ✅ |
| `vmp-cli` (`vmp` binary) | ✅ |
| `fute` / v01d | ✅ (WIP tree in `fute/wip`) |
| React modular UI + File menu + Vinyl Vibez | ✅ |
| Browser playback + live spectrum | ✅ |
| Tauri shell + IPC | ✅ scaffolded (`apps/vmp-tauri`) — needs WebKitGTK + ALSA on Linux |
| cpal hardware out | ✅ ALSA/PipeWire via cpal (default feature) |
| CLAP/VST, Vibe Cable, WebGL cymatics | 📋 roadmap |

## Run

```bash
cargo test --workspace
cargo run -p vmp-cli -- decode track.flac
cargo run -p vmp-cli -- play track.wav --seconds 3
npm run dev
# desktop when deps installed:
# npm run tauri:dev
```

## File menu (complete)

Open File / Many / Folder / Disc · Recent · Stream · Convert/Export ·  
Create / Save / Edit Playlist · Network Device · Save & Quit · Quit

## Next high-leverage work

1. Install system deps → `npm run tauri:dev` with real cpal  
2. Streaming decode (not full-file memory) for long concerts  
3. WebGL VAP cymatic visualizer  
4. CLAP host insert bus  
5. PipeWire Vibe Cable  
