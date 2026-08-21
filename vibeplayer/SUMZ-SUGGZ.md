---
type: implementation-note
title: Vibe Media Player — Summary and Suggestions
description: Honest status of the Rust VMP stack, remaining assets, and a phased plan to finish Skinz Customization Dashboard plus v01d transmutation of WinAmp, MediaMonkey, and Kodi skins.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - VASP
  - Fuxyez
  - AuraFS
domains:
  - audio
  - systems
nodes:
  - SIX⊗SCX
cores:
  - SIX
  - SCX
fields:
  - cognitive-field-tensor
---

# Vibe Media Player — SUMZ / SUGGZ

**Folder:** `vibeplayer/`  
**Role:** Crown-jewel desktop/media host. Rust engine + React shell + Tauri + nested v01d (FUTE).  
**Claimed completeness in tribe welcome:** ~20%. That is the right order of magnitude for a *product*. The crate map is farther along than the *experience*.

This file is working law for this folder only. Protocol truth lives in `vasp/`. Language engine truth lives in `fuxyez/`. Do not treat this tree as the VASP schema.

**Gap:** `PROJECT_CONTEXT.md`, `INVARIANTS.md`, and overlay `PHYSICS.md` are missing here.

---

## What this is

VMP is not another ID3 browser. Every track is a 9-pillar VASP identity. Chrome is modular (WinAmp-class floating modules). Host packaging is symbiotic via **v01d**. Vinyl Vibez is Mixxx transmuted, not a toy DJ skin.

```
Rust engine  →  decode · EQ · tags · playlist · vinyl
     ↕ IPC
React shell  →  pillar tabs · File menu · Skinz · Vinyl Vibez
     ↕
v01d (FUTE)  →  .wsz → .vskin · vsix → volt · Mixxx C++ → Rust scaffolds
```

## What actually exists (2026-08-20)

| Layer | Path | Honest status |
|---|---|---|
| VASP types / scoring | `crates/vmp-vap` | Present. Still labeled **v3.1**. Volume canon in `vasp/` is **3.69**. |
| DSP / EQ | `crates/vmp-dsp` | Present. |
| Decode / tags / engine | `crates/vmp-audio` | Symphonia + lofty + PlayerEngine. CLI can play. |
| Session / File menu | `crates/vmp-core` | Menu IDs exist. Disc / stream / convert are still product holes. |
| Visualizer crate | `crates/vmp-viz` | C→Rust transmute scaffolds in `transmute_raw/`. UI still uses CSS bars, not the TSLCA orb. |
| Vinyl engine | `crates/vmp-vinyl` | Dual-deck + equal-power xfade + BPM rate-match. **GPL.** UI not wired. |
| v01d bridge | `crates/vmp-v01d` | Thin wrapper around nested `fute`. |
| CLI | `crates/vmp-cli` | `version`, `decode`, `play`, `vap`, `v01d`, `vinyl`. |
| Nested FUTE | `fute/` | Compilable v01d. `wsz→vskin` is a **skeleton**, not a WinAmp renderer. Experimental tree in `fute/wip/`. |
| React UI | `src/` | Modular shell, File menu, four CSS palettes, spectrum bars. |
| Tauri | `apps/vmp-tauri` | Commands implemented (`play`, `vap`, `v01d_info`, …). React never calls `v01d_info`. Linux needs WebKitGTK + ALSA. |
| Deck Skinz | `skins/decks/vibe_default` | Manifest + layout. **Vinyl PNGs missing.** |

## Three host lineages to transmute (v01d favorites)

These are the three surfaces VMP should *eat* without becoming a museum of dead UIs. Opera GTX is the web-chrome cousin, not a fourth media player.

| Rank | Source | Why this one | Artifact | VMP target |
|---|---|---|---|---|
| 1 | **WinAmp Classic** | Unmatched skin culture. Dockable chrome. The language users already know. | `.wsz` (zip of BMPs + `skin.xml` / `pledit.txt`) | `.vskin` + Skinz Dashboard |
| 2 | **MediaMonkey** | Library-first columns, playlists, device sync, theme packs. Closest to “real collection OS.” | MM theme / skin zip + color XML | Library module + token map |
| 3 | **Kodi** | Full *media* (video + audio + art). XML window/control skin engine. | `addon.xml` + `Textures.xbt` / XML | Media modules + vis viewport |

**Opera GTX** (GX / Opera theme JSON + images) maps onto the **web/WASM shell** (`SymbioticMode::WasmShell`), not the vinyl engine. Same v01d button, different pack kind.

v01d already knows `PackageKind::Wsz` and `Vskin`. It does **not** yet know MediaMonkey, Kodi, or Opera GTX.

**Build (verified 2026-08-20):** `cargo test --workspace` **passes — 29 tests** on this Windows host (~112s cold). Engine layer is real. Product layer is still ~20%. No frontend test suite.

## Remaining assets (this folder)

**Must drop in before Skinz looks like a player**

- `skins/decks/vibe_default/assets/deck_a_vinyl.png`
- `skins/decks/vibe_default/assets/deck_b_vinyl.png`
- App icon set (Windows `.ico`, Linux `.png` 256/512, Tauri tray)
- Default `.vskin` pack for the four CSS palettes (`soul_cosmic`, `soul_forest`, `blood_moon`, `soul_gold`) so palettes become packs, not hard-coded objects in `VmpApp.tsx`

**Must harvest via v01d (do not invent new chrome)**

- WinAmp `main.bmp` / `cbuttons.bmp` / `titlebar.bmp` / `eqmain.bmp` / `pledit.bmp` / `vis` hole → VMP module atlas
- MediaMonkey theme colors + list fonts → CSS variables + library grid
- Kodi `Dialog*.xml` + `Player*.xml` + `Textures.xbt` extract → window/control map
- Opera GTX `manifest.json` + `images/` → WasmShell chrome tokens

**Must not steal from `vibeaudioplayer/` without a copy contract**

- TSLCA orb shaders (`vibe.frag` / bloom) should be a shared pack, not a second fork
- `vap_menu_*` glyphs are Vibe Audio Player chrome. VMP File menu can stay native until Skinz import fills it.

## Honest gaps vs the vision

1. **Skinz is four color chips**, not a Customization Dashboard. No import, no v01d button, no `.wsz` drop zone.
2. **`stage_vskin_from_wsz` copies bytes and writes a JSON stub.** It does not parse WinAmp regions or blit sprites onto modules.
3. **Visualizer is spectrum bars.** The 9-pillar orb lives in `vibeaudioplayer/`, not here.
4. **VASP 3.1 vs 3.69.** Fixture still Cannibal Corpse from the older protocol epoch.
5. **Vinyl Vibez UI is a mode toggle.** The `vmp-vinyl` engine is CLI-only.
6. **File menu items exist as IDs.** Open Disc, Stream, Convert, Network Device are not engines.
7. **Two FUTE trees.** Working v01d is `vibeplayer/fute`. `fuxyez/fute` README/Cargo are empty. Unify before both rot.
8. **Mixxx is not in this monorepo listing.** Transmute scripts assume `../mixxx`. Clone path must be documented as a lab sibling, not assumed.

## Feature suggestions (product)

**Skinz Customization Dashboard (the v01d room)**

- Dockable module titled **Skinz**. Default hidden; File → Skinz or `S` opens it.
- Three columns: **Preview** (live player chrome) · **Pack** (tokens, bitmaps, fonts) · **v01d**.
- Big **v01d** button. Drop / pick: `.wsz`, MediaMonkey theme zip, Kodi skin folder/`zip`, Opera GTX theme.
- After transmute: show mapping table (source control → VMP module). User can reassign vis hole → Orb, playlist → Library, EQ → EqRack.
- Built-in packs: **Void** (current), **Sodium**, **Lattice** — names already locked in `vibeaudioplayer/BUILD_PLAN.md`. Keep them identical across players.

**Player chrome that earns the “crown jewel” line**

- WinAmp-true: main, EQ, playlist, vis as *separate floating modules* (already started). Add shade mode, double-size, always-on-top (Tauri).
- MediaMonkey-true: album-grid + column browser that is VASP-aware (Affective / Genealogical filters, not just Genre).
- Kodi-true: video surface + fanart + now-playing OSD. Audio-only v1 can stub video as “Phase M.”
- Opera GTX-true: accent + wallpaper + font pack for the web shell only.

**Do not** flatten the vis hole into Milkdrop-only. The vis hole is the TSLCA orb once ported. Skins frame it.

## Phased series to finish VMP

### Series VMP-0 — Honest desktop (1–2 weeks)

1. `npm run tauri:dev` on this EliteBook (Windows) until local FLAC plays through cpal and the React shell.
2. Wire `nativePlay` / seek / volume / EQ to `vmp-audio` for real, not HTML5-only.
3. Streaming decode for long files (BUILD_PLAN item 2).
4. Align `VAP_VERSION` display with `vasp/` **3.69** or explicitly mark the crate as a v3.1 shim.

### Series VMP-1 — Skinz Dashboard + WinAmp v01d (highest leverage)

1. Promote Skinz from chips to Dashboard (preview / pack / v01d).
2. Real `.wsz` unzip + region map → `.vskin` atlas (main, cbuttons, pledit, eq, vis hole).
3. Hot-swap without restarting audio.
4. Ship Void / Sodium / Lattice as `.vskin` files.

### Series VMP-2 — MediaMonkey + Opera GTX

1. New `PackageKind::MmTheme` and `OperaGx`.
2. Token extract (colors, fonts, list metrics) → CSS variables.
3. Library module consumes MM-like columns; VASP pillars as extra columns.

### Series VMP-3 — Orb + VASP identity editor

1. Port orb WebGL from `vibeaudioplayer` into `vmp-viz` (or a shared crate). Replace CSS bars.
2. VASP editor first-class on every open file (README promise). Sidecar `.vap.json` + lofty embed.
3. Photometric hex drives chrome glow, not only vis.

### Series VMP-4 — Vinyl Vibez UI

1. Wire `vmp-vinyl` into `VinylVibez.tsx` (two decks, xfade, cue).
2. Drop vinyl PNGs into `skins/decks/vibe_default/assets/`.
3. Keep Mixxx-derived code **GPL-isolated** from MIT/Apache VMP chrome.

### Series VMP-5 — Kodi-class media (after audio is loved)

1. `PackageKind::KodiSkin` (XML + texture extract).
2. Video decode path (mpv/ffmpeg sidecar or GStreamer) behind File → Open.
3. Fanart from Genealogical / Contextual pillars.

### Series VMP-6 — Later, do not block 0–4

CLAP/VST insert bus · PipeWire Vibe Cable · Vibe Tribe remote index.

## Binding to the rest of ecosys

| Need | Source of truth |
|---|---|
| 9 pillars, schema 3.69 | `vasp/` |
| Orb shaders / Android twin | `vibeaudioplayer/` |
| Language + future pack DSL | `fuxyez/` (unify FUTE here → there) |
| Proven vsix→volt | `lapidary/` (pattern for wsz→vskin) |
| Sidecar / library persistence | `aurafs/` put/get only — after a single working store |
| Session memory (playlists, last skin) | `memoree/` Temporal + Operational nodes |
| 3×3 orb math | `tslca/` — do not invent a tenth pillar |

## What not to do in this folder

- Do not rewrite Mixxx. Transmute, then host.
- Do not add Play Billing, ads, or uploads.
- Do not name APS-OKF “Vibe-OKF.”
- Do not fork `fuxyez/fute` further. One v01d binary.
- Do not treat `fute/wip/` as the API.
