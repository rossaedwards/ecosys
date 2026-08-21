# Vibe Media Player

**Aurphyx LLC** · native / desktop host for [VASP 3.69](../vasp/README.md)

Experiential player, not another ID3 browser. Every open file is a 9-pillar V.A.P. identity. Modular WinAmp / VLC-style shell, multi-format decode, **Vinyl Vibez** (Mixxx-class DJ surface), packaging via **v01d (FUTE)**.

The browser / Framez product is **[Vibe Audio Player](../vibeaudioplayer/README.md)**.

```
Rust engine  →  decode · EQ · tags · playlist · context
     ↕ IPC
React shell  →  pillar tabs · floating modules · File menu · Vinyl Vibez
     ↕
v01d         →  symbiotic packs (standalone / guest / WASM / VinylVibez)
```

## Why it shifts the paradigm

| Old players | Vibe Media Player |
|-------------|-------------------|
| Title / artist / album | Experiential How/Why metadata (Thayer, MET, photometric, tribe) |
| Fixed chrome | Dockable modules |
| Tags as afterthought | VAP editor first-class; `.vap.json` sidecar |
| One surface | Player + Vinyl Vibez |
| Host-locked | v01d symbiotic packaging |

Filename (`Artist - Song Title` or `Artist_Album_SongTitle`) and ID3 are **identity**. Key, valence, MET, and lighting come from DSP + scoring or stay `unknown`.

## Formats

MP3 · M4A/AAC · FLAC · Ogg Vorbis · Opus · WAV · AIFF · MP4/M4V · CAF

VAP as `.vap.json` sidecar; native embed via ID3 / Vorbis / MP4 freeform (`vmp-audio` + lofty).

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

# Desktop (WebKitGTK + ALSA on Linux — see apps/vmp-tauri/README.md)
npm run tauri:dev
```

### System audio (cpal / ALSA)

```bash
cargo run -p vmp-cli -- version          # cpal + device counts
cargo run -p vmp-cli -- devices
cargo run -p vmp-cli -- play track.flac
```

## File menu

Open File · Open Many Files · Open Folder · Open Disc · Open Recent Media · Stream · Convert/Export · Create / Save / Edit Playlist · Open Network Device · Save & Quit · Quit

## Layout

| Path | Role |
|------|------|
| `crates/vmp-vap` | V.A.P. types, scoring, context engine |
| `crates/vmp-dsp` | Multi-mode EQ, analysis math |
| `crates/vmp-audio` | Symphonia decode, lofty tags, PlayerEngine |
| `crates/vmp-core` | Session, File menu, modules, Vinyl state |
| `crates/vmp-v01d` | Bridge to FUTE |
| `crates/vmp-cli` | `vmp` binary |
| `crates/vmp-viz` | Visualizer transmute |
| `crates/vmp-vinyl` | Vinyl Vibez decks |
| `fute/` | v01d packaging engine |
| `apps/vmp-tauri` | Desktop shell |
| `src/` | Modular React UI |
| `schema/` | Protocol schema copies |

Canonical protocol docs: [`../vasp/`](../vasp/). Product docs: [PROJECT_CONTEXT.md](PROJECT_CONTEXT.md), [EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md).

## Vinyl Vibez

Plan: transmute [Mixxx](https://github.com/mixxxdj/mixxx) (C++) with **v01d / FUTE**, not a toy DJ skin.

```
Mixxx C++  --v01d lang-->  crates/vmp-vinyl (GPL symbiont)  -->  VMP SymbioticMode::VinylVibez
```

- Spec: [`docs/VINYL_VIBEZ_MIXXX_TRANSMUTE.md`](docs/VINYL_VIBEZ_MIXXX_TRANSMUTE.md)
- Live touch + brand booths: [`docs/VINYL_VIBEZ_LIVE_TOUCH_AND_BRAND_DECKS.md`](docs/VINYL_VIBEZ_LIVE_TOUCH_AND_BRAND_DECKS.md)

```bash
cargo run -p fute --bin v01d --features clang-ast -- lang ../mixxx/src/engine/enginebuffer.h \
  -o crates/vmp-vinyl/transmute_raw/enginebuffer.rs --from cpp

cargo run -p vmp-cli -- vinyl trackA.flac trackB.flac --xfade 0.5 --seconds 5
```

## Streaming note

`vasp_sdk-api_streaming.md` documents `VapEngine::transform_spotify`. That path fed `/v1/tracks` + `/v1/audio-features` into scoring for the golden set. New Spotify apps no longer get audio-features; TIDAL catalog + local DSP is the replacement input. TIDAL OAuth belongs to **Vibe Audio Player** (same Aurphyx client is reusable if a desktop redirect is added).

## License

MIT OR Apache-2.0
