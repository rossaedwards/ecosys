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

**Folder:** `vibemediaplayer/` (renamed from `vibeplayer/` 2026-08-21)  
**Role:** Rust + Tauri desktop host — one of three **peer** surfaces (alignment lock, 2026-08-23): `vibemediaplayer/` (desktop, canonical engine), `vibeaudioplayer/` (web/Android-iOS/IoT via WebView), `visualizer/` (3D holographic lattice engine, renamed from `vibe-audio-visualizer/` here — not yet renamed in `vibeaudioplayer/`). These three are not being consolidated; do not propose that.  
**Claimed completeness in tribe welcome:** ~20%. That is the right order of magnitude for a *product*. The crate map is farther along than the *experience*.

This file is working law for this folder only. Protocol truth lives in `vasp/`. Language engine truth lives in `fuxyez/`. Do not treat this tree as the VASP schema.

**Gap:** `PROJECT_CONTEXT.md`, `INVARIANTS.md`, and overlay `PHYSICS.md` are missing here.

## Update — 2026-08-23

- Re-ran `cargo test --workspace` post-rename: still **29 passing, 0 failed**. Rename didn't break the build.
- `vasp/` delivered the full VASP 3.69 canon (Intro Specs, Logic Architecture, Official Schema, TechSpec Manual, ExecSum, a Context Simulation script, a Scoring Engine reference impl, and a Cannibal Corpse test fixture). See "VASP 3.1 vs 3.69" below — this is now a concrete, fixable gap, not a vague one.
- `visualizer/` (renamed) is a real, substantial C/OpenGL VLC-plugin codebase (~2,650 lines: `gl_renderer.c` 811, `vap_loader.c` 695, `vibe_visualizer.c` 488, `dsp_engine.c` 79, plus GLSL shaders) implementing a 9-pillar TSLCA cymatic renderer against `libvlc`. It is **not dead code** — `crates/vmp-viz/transmute_raw/{dsp_engine,vap_runtime,vap_runtime_clang,vap_affective}.rs` are the FUTE-transmutation targets for this exact C source; they're just still `todo!()` stubs. Porting this C engine into `vmp-viz` (or a shared crate all three surfaces link) is the real next step, not a rewrite from scratch.
- No `shared/` resource/asset root exists on disk yet anywhere under `ecosys/`, despite the stated intent to share resources/assets across all three surfaces. This needs an actual decision (see Clarification Questions).
- Tidal DJ / VirtualDJ / Mixxx-GPL scope is locked per prior session memory — not re-litigated here.

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
| Visualizer crate | `crates/vmp-viz` | C→Rust transmute scaffolds in `transmute_raw/` are **all `todo!()`**, unwired — but the C source they target (`visualizer/src/*.c`, ~2,650 lines, real VLC-plugin cymatic renderer) genuinely exists and is active per the 2026-08-23 alignment lock. UI still uses CSS bars, not the TSLCA orb. |
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

**Shared with the other two surfaces (per 2026-08-23 alignment lock — sharing is now the direction, not a thing to avoid)**

- TSLCA orb shaders (`visualizer/shaders/vibe.frag`, `post_bloom.frag`) should land in one shared location all three surfaces reference, not be copy-forked per tree. No `shared/` root exists yet — this needs a real decision (see Clarification Questions), not another copy.
- `vap_menu_*` glyphs are Vibe Audio Player (`vibeaudioplayer/`) chrome. VMP File menu can stay native until Skinz import fills it, but if these move to a shared asset store they shouldn't be duplicated by hand in the meantime.

## Honest gaps vs the vision

1. **Skinz is four color chips**, not a Customization Dashboard. No import, no v01d button, no `.wsz` drop zone.
2. **`stage_vskin_from_wsz` copies bytes and writes a JSON stub.** It does not parse WinAmp regions or blit sprites onto modules.
3. **Visualizer is spectrum bars.** The 9-pillar orb lives in `vibeaudioplayer/`, not here.
4. ~~VASP 3.1 vs 3.69~~ **Fully resolved 2026-08-23.** `VASP_TechSpec.md` is the single canon doc in `vasp/`; the field is `VASP_VERSION` at `"3.69"`. Swept across all 16 `.md`-disguised doc/data files *and* `crates/vmp-vap`'s Rust code (`lib.rs`, `types.rs`, `error.rs`, `scoring.rs`) plus `vmp-audio/src/tags.rs`'s embedded-tag detection and the `fixtures/cannibal_corpse_inhumane_harvest.vap.json` fixture. `cargo test --workspace` still passes 29/29 after the rename.
   Separately, `VapObject::defaults()` (`types.rs:163-249`) free-hands pillar sub-field names that don't match the Official Schema: `KEY` vs schema's `KEY_SIGNATURE`; `FREQ_BALANCE` (string) vs schema's `FREQUENCY_BALANCE` (object with `SUB_DOMINANT`/`MID_FORWARD`/`AIR_BRILLIANCE`); `PALETTE_TEMP` vs `PALETTE_TEMPERATURE`; `SATURATION_INDEX` vs `SPECTRAL_SATURATION`; `TUNING` vs `TUNING_STANDARD`; and an `ENERGY.ENTRAINMENT_FACTOR` field the schema doesn't define at all under KINETIC. Because pillars are stored as loose `serde_json::Value` rather than typed structs, none of this is caught at compile time or runtime — the crate will silently accept and round-trip non-conformant VASP objects.
5. **Vinyl Vibez UI is a mode toggle.** The `vmp-vinyl` engine is CLI-only.
6. **File menu items exist as IDs.** Open Disc, Stream, Convert, Network Device are not engines.
7. **Two FUTE trees — confirmed byte-identical, 2026-08-23.** `vibemediaplayer/fute/wip/` and `fuxyez/fute/` are the *same* ~6,962-line tree (`diff` on sampled files returns zero differences; both have empty/missing `Cargo.toml` and `README.md`; neither is wired into any workspace). This isn't two efforts converging — it's one fork nobody reconciled. Fute itself has three distinct pieces worth naming separately from now on:
   - **`fute/` (top-level, real)** — compiles, in the workspace, ~1,600 lines. Does two things for real: (a) package repackaging via `PackageKind` (`Vsix→Volt` and `Directory→Vmpx` are real; `Wsz→Vskin` is not — see below), and (b) C/C++→Rust transmutation with a structural regex/line backend (always on) plus an optional libclang AST backend (`clang-ast` feature). This is the actual engine behind the `vmp-viz`/`vmp-vinyl` `transmute_raw/` scaffolds.
   - **`stage_vskin_from_wsz` (in the real `fute/`) is confirmed-by-reading-the-code a skeleton**, not "mostly there": it never unzips the `.wsz`, never parses `skin.xml`/`pledit.txt`, never extracts a sprite. It writes a hardcoded `manifest.json` (`widgets: ["play","pause","eq_knobs","vu","progress"]` — not derived from the source file at all) with the literal note `"Generated skeleton — replace sprites from original wsz"`, then copies the raw `.wsz` bytes alongside for later harvest. This is the single concrete blocker for Series VMP-1 (Skinz Dashboard) below — there is no WinAmp-region-to-VMP-module mapping logic anywhere yet.
   - **`fute/wip/` and `fuxyez/fute/` (identical, orphaned)** — AST/IR, codegen, a "core engine," pattern matching, an "optimizer" with passes named `quantum_fuse`/`entanglement_map`. No quantum computing content behind those names — it's Fuxyez house branding, not a hidden capability. Contributes zero working code to either repo today. Pick one location, delete the other, and decide whether it's on a resourced roadmap or should be archived — see Clarification Questions.
8. **Mixxx is not in this monorepo listing.** Transmute scripts assume `../mixxx`. Clone path must be documented as a lab sibling, not assumed.

## Feature suggestions (product)

**Skinz Customization Dashboard (the v01d room)**

- Dockable module titled **Skinz**. Default hidden; File → Skinz or `S` opens it.
- Three columns: **Preview** (live player chrome) · **Pack** (tokens, bitmaps, fonts) · **v01d**.
- Big **v01d** button. Drop / pick: `.wsz`, MediaMonkey theme zip, Kodi skin folder/`zip`, Opera GTX theme.
- After transmute: show mapping table (source control → VMP module). User can reassign vis hole → Orb, playlist → Library, EQ → EqRack.
- Built-in packs: **Void** (current), **Sodium**, **Lattice** — names already locked in `vibeaudioplayer/BUILD_PLAN.md`. Keep them identical across players.

**Three customization depths (2026-08-23 request — see chat for full tech-stack exploration)**

- **Simple:** pick a built-in pack (Void/Sodium/Lattice/imported), no editing. Token-swap only, applies instantly, cannot break the layout.
- **Normal:** edit exposed tokens (colors, accent, module visibility/position, orb intensity) through the Skinz dashboard UI. No raw file editing, no way to corrupt a pack.
- **Creator:** full pack authoring — edit/import `.vskin` token+bitmap files directly, remap module slots, drop in custom shaders for the orb (guarded — see sandboxing note in chat), v01d-import from WinAmp/MediaMonkey/Kodi/Opera GTX source packs.
- All three tiers read the same underlying token schema; "Simple" and "Normal" are progressively-unlocked views over the same data a Creator pack authors directly. Do not build three separate skin formats.

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
4. ~~Rename `VAP_VERSION` → `VASP_VERSION`~~ **Done 2026-08-23.**

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
| Orb shaders / cymatic engine source | `visualizer/` (real C/OpenGL, ~2,650 lines — transmute target for `vmp-viz`) |
| Android/iOS/IoT twin | `vibeaudioplayer/` |
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
- Do not propose merging `vibemediaplayer/` / `vibeaudioplayer/` / `visualizer/` into one tree — locked as peers, 2026-08-23.

## Clarification questions (2026-08-23)

1. ~~`VAP_VERSION` vs `VASP_VERSION`.~~ **Resolved 2026-08-23:** `VASP_TechSpec.md` is canon, field is `VASP_VERSION`. All doc copies swept; `crates/vmp-vap` Rust code still needs the same rename (see Series VMP-0 item 4).
2. **Typed pillars or stay loose JSON?** `vmp-vap`'s `Pillars` struct stores each pillar as untyped `serde_json::Value` for flexibility with golden-set data. That means the Official Schema's field names, enums, and numeric ranges (e.g. `AROUSAL` 0.0–1.0, hex pattern on `PRIMARY_HEX`) are never enforced. Worth generating strongly-typed structs from `vasp_schema_v3_1.json`'s 3.69 successor, or is loose JSON intentional so the engine doesn't break every time the canon evolves?
3. **Where does `shared/` actually live?** “Share resources/assets across all three surfaces” has no physical home yet. A new `ecosys/shared/` root the three reference by relative path? A published local crate/npm package? Straight symlinks? This blocks the orb-shader and Skinz-pack sharing work concretely.
4. **Tauri mobile vs. `vibeaudioplayer`'s WebView path.** Tauri 2 has official Android/iOS targets from the same Rust core `vibemediaplayer` already uses. Given the alignment lock keeps `vibeaudioplayer` as its own WebView-based mobile/IoT surface rather than folding into Tauri-mobile, is that a final call, or is “all three can be Rust-based” still open to revisiting how `vibeaudioplayer`'s mobile leg is built (not whether it exists)?
5. **`fute/wip/` / `fuxyez/fute/` fate.** Confirmed byte-identical, ~6,962 lines, neither has a `Cargo.toml`, neither is wired into any workspace. Which location is canonical — wire that one in and delete the other, or archive both if this experimental engine isn't on a near-term roadmap?
6. **`stage_vskin_from_wsz` is the real Skinz blocker.** It's a manifest-only skeleton with no WinAmp region/sprite parsing at all. Is writing the real `.wsz` unzip + `skin.xml`/`pledit.txt` parser + sprite-atlas extraction the next concrete task, or is there appetite to reach for an existing WinAmp-skin-parsing library (if one exists with a compatible license) instead of hand-rolling the format from spec?
