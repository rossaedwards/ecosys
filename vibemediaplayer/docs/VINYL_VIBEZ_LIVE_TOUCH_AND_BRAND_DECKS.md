# Vinyl Vibez — Live Touch Decks & Brand Ecosystem

**Status:** Vision locked (pause for vinyl artwork assets)  
**Stack:** VMP · `vmp-vinyl` · Skinz · v01d · (future) controller HID/MIDI  

---

## 1. Product moment

User has a **touch screen**. They go **Live / Fullscreen**:

- Two **physical-looking vinyl platters** (user-created large records)
- Center / bottom **mixer** (real-deck proportioned)
- **Fingers** spin, brake, scratch, cue — same motor affordances as a DJ set
- Optional **match my real booth**: Pioneer / Numark / Serato / Traktor / … virtual skin + control map that mirrors their hardware + catalog apps

This is not a toy skin. It is **Vinyl Vibez as a live instrument surface** on top of the Mixxx-symbiont engine (`vmp-vinyl`).

---

## 2. Modes

| Mode | Behavior |
|------|----------|
| **Studio** | Modular VMP chrome (current multi-panel) |
| **Live** | Fullscreen dual-deck + mixer; hide non-DJ chrome |
| **Live + Match Booth** | Live layout scaled to detected/selected brand kit |

Toggle: titlebar **LIVE** / F11 / gesture (three-finger or corner affordance).

---

## 3. Touch interaction model (platters)

Each platter is a hit-target with:

| Gesture | Mapping (engine) |
|---------|------------------|
| Rotate / drag tangent | Scratch / jog → temporary rate override + playhead nudge |
| Inertia after release | Spindown curve (vinyl physics) |
| Tap center / cue pad | Cue jump / set cue |
| Two-finger pinch on platter | Zoom waveform (visual only) or pitch bend range |
| Long-press label | Load track / library |
| Vertical drag on edge | Pitch fader (rate %) |

Mixer:

| Control | Touch |
|---------|--------|
| Crossfader | Horizontal strip |
| Channel faders | Vertical strips |
| EQ knobs | Rotary drag |
| Filter / FX | Knobs / pads |

All gestures → `VinylEngine` (`play`, `seek`, `set_rate_percent`, `set_crossfader`, …) already started in `vmp-vinyl`.

---

## 4. Brand deck kits (Pioneer, Numark, Serato, Traktor, …)

### 4.1 Kit package (`.vdeck` or directory under `skins/decks/`)

```
skins/decks/pioneer_cdj_xdj_kit/
  manifest.json          # brand, models, license, scale
  layout.toml            # positions of platters, mixer, pads (normalized 0..1)
  assets/
    deck_a.png / .webp   # high-res faceplates
    deck_b.png
    mixer.png
    platter_ring.png
    needle.png
    led_on.png …
  control_map.toml       # UI control id → VinylEngine / MIDI CC
  midi_map.toml          # optional: HID/MIDI for real hardware twin
  legal.md               # trademark/asset licensing notes
```

### 4.2 Manifest sketch

```json
{
  "id": "pioneer_booth_standard",
  "brand": "Pioneer-inspired",
  "models": ["CDJ-class", "DJM-class"],
  "version": "0.1.0",
  "layout": "layout.toml",
  "assets_dpi": 2,
  "touch_profile": "capacitive_default",
  "engine": "vmp-vinyl",
  "notes": "Community / user-supplied assets; not an official OEM product"
}
```

### 4.3 Asset pipeline (user’s pause task)

You create **large vinyl record** art for deck A/B. Later:

- Drop into `skins/decks/<kit>/assets/`
- Or `File → Import Deck Kit…`
- Skinz layer: recolor / logo / texture without rebuilding layout

### 4.4 Legal fence (critical)

- **Do not ship trademarked logos/faceplates as official Pioneer/Serato/etc. product** without license.
- Prefer: **“inspired-by / community kits”**, user-imported photos, or licensed OEM packs.
- Schematics + manuals → **control geometry + CC maps**, not pirated trademark skins.
- VMP provides the **kit format + wizard**; brand packs can be user/OEM content.

---

## 5. Booth Match wizard (auto-setup)

### 5.1 Inputs

1. **USB/MIDI/HID** controllers already known to OS  
2. **User photos** of booth (optional CV later)  
3. **Manual pick**: brand + model list  
4. **Catalog / library services** (Serato library path, Rekordbox export, Traktor collection — read-only import plugins)

### 5.2 Flow

```
Detect devices → Suggest kit (e.g. 2× CDJ-class + DJM-class)
     → Import / download kit assets (user path)
     → Map MIDI/HID → VinylEngine
     → Optional: import library (crate)
     → Enter Live fullscreen
```

Auto-setup when confidence high; wizard when ambiguous.

### 5.3 “Virtual = real”

| Real booth | Virtual Live |
|------------|--------------|
| Left deck | Deck A platter + load |
| Right deck | Deck B |
| Mixer | Center strip + xfade |
| Controller pads | On-skin pads |
| Catalog app | Crate panel / sidebar (collapsible in Live) |

---

## 6. Skinz vs Deck kits

| Layer | Changes |
|-------|---------|
| **Deck kit** | Geometry, brand faceplates, control map |
| **Skinz / theme** | Colors, LED hues, wood/metal texture, **logos, artwork**, fonts |
| **Vinyl records** | Per-track or default large disc art (your current asset work) |

User can: Pioneer-shaped kit + custom Blood Moon skinz + personal record labels.

---

## 7. Engine / software map

```
Touch / Live UI
    → control events
vmp-vinyl VinylEngine   (load, play, seek, rate, cue, xfade, sync)
    → process() PCM
vmp-audio cpal out
    → VAP runtimes per deck (vmp-viz) for meters / lights
```

FUTE/libclang: continue transmuting Mixxx engine modules into richer behaviour behind the same API.

Controller import (later):

- MIDI learn  
- Existing Mixxx-style mapping JSON → FUTE → VMP map  
- HID for specific OS-visible devices  

---

## 8. Implementation phases (when you return)

| Phase | Deliverable |
|-------|-------------|
| **LT-0** | Live fullscreen layout; two circular platters + xfade (web touch events) |
| **LT-1** | Platter inertia + jog → `EngineBuffer` rate/seek |
| **LT-2** | Vinyl record textures (your assets) + needle |
| **LT-3** | Deck kit format + 1 open “generic booth” kit |
| **LT-4** | Booth Match wizard (MIDI detect + kit suggest) |
| **LT-5** | Brand community kits + Skinz recolor |
| **LT-6** | Catalog importers (Rekordbox/Serato/Traktor read paths) |

---

## 9. Asset drop folder (ready for your vinyl art)

```
vibeaudio/skins/decks/
  _template/
    manifest.json
    layout.toml
    assets/.gitkeep
  vibe_default/
    manifest.json
    layout.toml
    assets/   ← drop large vinyl PNGs/WebPs here (deck_a_vinyl.png, deck_b_vinyl.png)
```

Suggested names:

- `deck_a_vinyl.png` / `deck_b_vinyl.png` — square or circular, ≥ 2048²  
- `platter_metal.png` — optional rim  
- `mixer_face.png` — optional  

---

## 10. One-line north star

> **Live fullscreen dual-vinyl touch decks that can mirror a real Pioneer/Numark/Serato/Traktor booth, skinned your way, driven by a Mixxx-transmuted engine and V.A.P. identity.**

Enjoy making the records — when you’re back, we start **LT-0 Live touch** with your art.
