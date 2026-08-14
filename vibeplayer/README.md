# Vibe Media Player

**Paradigm-shifting experiential media** — not another ID3 browser.

VMP treats every track as a **9-dimensional V.A.P. identity** (Structural → Genealogical), with a modular WinAmp/VLC shell, multi-format fidelity, and **Vinyl Vibez** (Mixxx-class DJ surface). Host-agnostic packaging rides **v01d (FUTE)**.

```
Rust engine  →  decode · EQ · tags · playlist · context
     ↕ IPC
React shell  →  vertical pillar tabs · floating modules · File menu · Vinyl Vibez
     ↕
v01d         →  symbiotic packs (standalone / guest / WASM / VinylVibez)
```

## Why it shifts the paradigm

| Old players | Vibe Media Player |
|-------------|-------------------|
| Title / artist / album | **Experiential** How/Why metadata (Thayer, MET, photometric, tribe…) |
| Fixed chrome | **Dockable modules** you drag like WinAmp — faster |
| Tags as afterthought | **VAP editor** first-class on every open file |
| One surface | **Player** + **Vinyl Vibez → Mixxx** |
| Host-locked | **v01d** symbiotic packaging |

## Formats

MP3 · M4A/AAC · FLAC · Ogg Vorbis · Opus · WAV · AIFF · MP4/M4V · CAF  

VAP always as `.vap.json` sidecar; native embed via ID3 / Vorbis / MP4 freeform (`vmp-audio` + lofty).

## Quick start

```bash
# Tests (protocol, decode, modules, v01d)
cargo test --workspace

# Headless CLI
cargo run -p vmp-cli -- version
cargo run -p vmp-cli -- decode ./song.flac
cargo run -p vmp-cli -- play ./song.wav --seconds 5
cargo run -p vmp-cli -- vap ./song.flac --json
cargo run -p vmp-cli -- vap-save ./song.flac --embed
cargo run -p vmp-cli -- v01d

# UI (browser path — HTML5 + live spectrum)
npm install
npm run dev

# Desktop (needs WebKitGTK + ALSA headers — see apps/vmp-tauri/README.md)
npm run tauri:dev
```

### System audio (cpal / ALSA)

With `alsa-lib-devel` installed, playback is **on by default**:

```bash
cargo run -p vmp-cli -- version          # shows cpal ENABLED + device counts
cargo run -p vmp-cli -- devices          # list inputs/outputs
cargo run -p vmp-cli -- play track.flac  # hardware output
```

## File menu

Open File · Open Many Files · Open Folder · Open Disc · Open Recent Media · Stream ·  
Convert/Export · Create / Save / Edit Playlist · Open Network Device · Save & Quit · Quit

## Layout

| Path | Role |
|------|------|
| `crates/vmp-vap` | V.A.P. v3.1 types, scoring, context engine |
| `crates/vmp-dsp` | Multi-mode EQ, analysis math |
| `crates/vmp-audio` | Symphonia decode, lofty tags, PlayerEngine |
| `crates/vmp-core` | Session, File menu, modules, Vinyl state |
| `crates/vmp-v01d` | Bridge to FUTE |
| `crates/vmp-cli` | `vmp` binary |
| `fute/` | **v01d** packaging engine (`fute/wip` = experimental tree) |
| `apps/vmp-tauri` | Desktop shell |
| `src/` | Modular React UI |

## Vinyl Vibez

**Original plan:** *literally transmute [Mixxx](https://github.com/mixxxdj/mixxx) (C++) into the Vibe Audio stack* with **v01d / FUTE**, not invent a toy DJ skin.

```
Mixxx C++  --v01d lang-->  crates/vmp-vinyl (GPL symbiont)  -->  VMP SymbioticMode::VinylVibez
```

- Spec: [`docs/VINYL_VIBEZ_MIXXX_TRANSMUTE.md`](docs/VINYL_VIBEZ_MIXXX_TRANSMUTE.md)  
- Scaffold: `crates/vmp-vinyl` (decks/mixer API; Mixxx algorithms land via FUTE)  
- UI toggle **VINYL VIBEZ** hosts that symbiont  

```bash
# After cloning Mixxx beside the monorepo:
cargo run -p fute --bin v01d --features clang-ast -- lang ../mixxx/src/engine/enginebuffer.h \
  -o crates/vmp-vinyl/transmute_raw/enginebuffer.rs --from cpp

# Dual-deck engine demo
cargo run -p vmp-cli -- vinyl trackA.flac trackB.flac --xfade 0.5 --seconds 5
```

**Live touch + brand booths** (Pioneer/Numark/Serato/Traktor-class kits, finger platters, Skinz):  
[`docs/VINYL_VIBEZ_LIVE_TOUCH_AND_BRAND_DECKS.md`](docs/VINYL_VIBEZ_LIVE_TOUCH_AND_BRAND_DECKS.md)  

Drop big vinyl images into: `skins/decks/vibe_default/assets/`

## License

MIT OR Apache-2.0
