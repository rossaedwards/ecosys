# Vinyl Vibez = Mixxx Transmuted into Vibe Audio

## Intent (canonical)

**Vinyl Vibez is not a greenfield DJ UI.**  
The original plan is to **literally transmute Mixxx** (C++ / Qt) into the **Vibe Audio tech stack** using **v01d / FUTE**, then host that symbiont inside Vibe Media Player.

```
Mixxx (C++ · Qt · Engine · Control · Controllers · Effects · Library)
        │
        │  v01d lang  (C/C++ → Rust)
        │  + domain maps (Qt → Tauri/Web, Engine → vmp-audio graph)
        ▼
Vinyl Vibez (Rust · Tauri · vmp-audio · vmp-viz · V.A.P. · TSLCA)
        │
        ▼
Vibe Media Player  ·  SymbioticMode::VinylVibez
```

This is the same class of move as `.vsix → .volt` and `vibe-audio-visualizer` C → `vmp-viz` Rust: **host-agnostic, stack-native, identity-preserving port**, not a rebranded skin.

---

## Why Mixxx (and not “inspired by”)

| Mixxx domain | Role | Vibe stack target |
|--------------|------|-------------------|
| **Engine** (decks, buffers, resampling, sync) | Real-time dual/quad deck DSP | `vmp-audio` graph + `vmp-viz` meters |
| **Control** / **ControlObject** bus | Parameter bus, MIDI/HID | Rust control bus + Tauri IPC |
| **Library** (TrackCollection, analysis) | Crate, BPM/key analysis | `vmp-core` playlist + VAP Phase-I/II |
| **Controllers** (JS mappings + MIDI) | Hardware surface | Controllers pack → Agora / WASM scripts (FUTE later) |
| **Effects** | FX chain | Plugin insert bus (CLAP later; Mixxx effects → Rust first) |
| **Vinyl control / DVS** | Timecode | Vinyl Vibez Phase DVS |
| **Skin / QML-ish UI** | Deck chrome | Modular VMP modules + Skinz (`.vskin`) |
| **Broadcast / recording** | Stream out | Convert/Export + network later |

Mixxx already solved decades of DJ edge cases. **Transmute retains that knowledge** inside V.A.P./TSLCA identity and Rust safety.

---

## FUTE’s job in this plan

1. **`v01d lang --from cpp --to rust`**  
   Structural C++ → Rust scaffolds from Mixxx modules (headers first, then `.cpp`).

2. **Domain remaps (human + rule packs)**  
   - Qt widgets / signals → Tauri commands + React modules  
   - `ControlObject` → typed `ControlBus` in Rust  
   - Mixxx `TrackPointer` metadata → `VapObject` + bibliographic tags  
   - Mixxx analyzer BPM/key → `vmp-dsp` / `vmp-viz` Phase-I  

3. **Package symbiont**  
   `mixxx/` tree or selected crates → `.vmpx` Vinyl pack (`SymbioticMode::VinylVibez`).

4. **Ethical / license continuity**  
   Mixxx is GPL. Vinyl Vibez / VMP packaging of **transmuted Mixxx-derived code** must stay **GPL-compatible** (likely AGPL/GPL for that crate).  
   Keep **clean-room Vibe surface** (UI chrome, VAP) separable from **GPL engine symbiont**.

---

## Transmute phases (execution order)

### Phase VV-0 — Cartography (no full clone required yet)

- Inventory Mixxx module map (Engine, Mixer, Library, Controllers, Effects, VinylControl).
- Choose **first slice**: Engine deck + crossfader + sync (minimum live mix).
- Document ControlObject graph → Rust bus.

### Phase VV-1 — Header / interface transmute

```bash
# Once mixxx source is cloned beside the monorepo:
git clone https://github.com/mixxxdj/mixxx.git ~/rossaedwards/main/mixxx

# Transmute high-value headers into scaffolds
cargo run -p fute --bin v01d -- lang \
  ../mixxx/src/engine/enginebuffer.h \
  -o crates/vmp-vinyl/transmute_raw/enginebuffer.rs --from cpp
```

Target crate: **`crates/vmp-vinyl`** (GPL boundary crate).

### Phase VV-2 — Deck engine in Rust

- Dual `EngineDeck` (A/B) fed by `vmp-audio::decode_file` or streaming decode.
- Pitch / rate, cue, sync lock (port Mixxx sync semantics, not guesswork).
- Crossfader + channel gains → single output via cpal.

### Phase VV-3 — Library + VAP

- Import crate paths; BPM/key from Mixxx analyzer logic (transmuted) **or** librosa-class port.
- Every track carries **V.A.P.** (TSLCA 9-cell identity) as first-class, not optional tags.

### Phase VV-4 — Controllers + effects

- MIDI/HID mappings: start with a few popular controllers.
- Effects: delay/reverb/filter chain as Rust DSP, Mixxx FX math as reference.

### Phase VV-5 — DVS / timecode

- Vinyl control last (highest HW surface area).

### Phase VV-6 — UI

- Replace Qt skins with **VMP modular surface** already labeled Vinyl Vibez  
  (decks, mixer, library) — backed by real engine, not mocks.

---

## Repo layout (target)

```
vibeaudio/
  crates/
    vmp-vinyl/           # GPL symbiont: transmuted Mixxx engine core
      transmute_raw/     # FUTE scaffolds (not shipped raw)
      src/               # polished Rust
    vmp-audio/           # shared decode/play (already)
    vmp-viz/             # VAP live meters (from VAV C)
    vmp-core/            # session · VinylVibez mode
  fute/                  # v01d lang + pack transmute
  docs/
    VINYL_VIBEZ_MIXXX_TRANSMUTE.md   # this file
```

---

## Licensing fence (non-negotiable)

| Component | Suggested license |
|-----------|-------------------|
| VMP shell, VAP schema, TSLCA docs, UI chrome | MIT OR Apache-2.0 (current) |
| **Code derived from Mixxx via transmute** | **GPL-2.0-or-later** (match Mixxx) in `vmp-vinyl` |
| Linking | Dynamic/process boundary or full GPL app build profile |

Do **not** silently relicense Mixxx-derived logic as MIT.

---

## Immediate next actions

1. ~~Clone Mixxx~~ / FUTE clang-ast / dual-deck polish — in progress in-tree.
2. **Live touch + brand deck kits** — see `docs/VINYL_VIBEZ_LIVE_TOUCH_AND_BRAND_DECKS.md`.
3. Drop large vinyl art into `skins/decks/vibe_default/assets/`.
4. Wire VMP **LIVE fullscreen** platters → `VinylEngine` gestures.
5. Booth Match wizard + Pioneer/Numark/Serato/Traktor-class kits (community/OEM).

---

## One-line product definition

> **Vinyl Vibez is Mixxx, symbiontically transmuted by v01d into Vibe Audio’s Rust + V.A.P. + Tauri stack — then living inside Vibe Media Player as `SymbioticMode::VinylVibez`.**
