---
type: implementation-note
title: Vibe Audio Player — Summary and Suggestions
description: Honest status of the web and Android Vibe Audio Player, remaining assets, and a phased plan that stays local-first while Skinz and v01d land first on Vibe Media Player.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - VASP
domains:
  - audio
  - xessability
nodes:
  - SIX⊗SIX
cores:
  - SIX
fields:
  - harmonic-integrity-field
---

# Vibe Audio Player — SUMZ / SUGGZ

**Folder:** `vibeaudioplayer/`  
**Role:** Local music player + TSLCA orb. Web (Vite / React / TanStack) and Android Studio drop-in. Nothing is uploaded.  
**Sibling:** `C:\aurphyx\vibeaudioplayer` is the live Studio tree. This folder’s `android-studio-dropin/` is the mirror.

This is **priority 1 of 3** with Vibe Media Player and Fuxyez. It is the *phone + browser* surface. VMP (`vibeplayer/`) is the *desktop crown jewel*. Do not merge the two trees.

**Gap:** `PROJECT_CONTEXT.md` is missing. `BUILD_PLAN.md` and `ANDROID_STUDIO_APK.md` already do that job — keep them.

---

## What this is

Home picture is the nine-pillar TSLCA orb. Tabs: **Orb · Library · VASP · Scene · About**. Menu glyphs crop `vap_menu_*_tb.png`. About → 13 taps reveals the Aurphyx business-card QR.

The VLC plugin in `vibe-audio-visualizer/` is **not rewritten**. The Orb tab speaks the same renderer language (`vibe.frag` + bloom).

## What actually exists

| Surface | Status |
|---|---|
| Web Orb | WebGL port of `vibe.frag` + 2D fallback. Analyser → `u_chrom_energy[4]` and the VLC uniform block. |
| Web engine | `src/lib/audio/engine.ts` — demo synth + local files, FFT, beat, centroid, saturation. |
| Web tabs / sheets | Orb, Library, VASP nine cards, Scene, About, loading splash. |
| Art wiring | `src/lib/art.ts` + `public/chrome/` copies. Tabs crop `vap_menu_2_tb.png`. |
| Android drop-in | Compose single-activity, Media3 `PlaybackService`, SAF library, Canvas orb, splash + launcher, `minSdk 26`. |
| VLC plugin | Leave it. Source of shader truth. |
| Not in v1 (locked by BUILD_PLAN) | `.wsz` loader, VST3, Vibe Tribe store, Play Billing, uploads. |

Grok Build leftover: `AGENTS.md` + `.grok/skills/` describe a sandbox that is **not** this repo. Do not obey its “bind 8080 / never tell the user about localhost” contract when working in ecosys. Keep the player; ignore the sandbox persona.

## Remaining assets

### Web — present, confirm they are the ones the UI actually loads

| Asset | Where | Notes |
|---|---|---|
| Menu chrome | `assets/` + `public/chrome/vap_menu_1.png`, `_tb`, `vap_menu_2.png`, `_tb` | Do not invent new menu art. |
| Loading | `public/chrome/vibeaudioplayer_loading.png`, `loading2.png` | Splash uses these. |
| App icon | `public/chrome/app-icon-1024.jpg` | Need 192 / 512 PWA + favicon derived from this, not a generic Vite mark. |
| QR | `assets/aurphyx-business-card-qr.png` + chrome copy | About 13-tap only. |
| Scene stills | `assets/scenes/night-drive-720x405.jpg`, `deep-space-720x405.jpg` | Photometric palettes. |
| Pillar / transport SVGs | `assets/icons/*.svg` | Large unused set. Either wire them as *optional Skinz glyphs* or leave them until Skinz. Do not replace the cropped menu orbs. |

### Android — drop-in vs Studio

PATHS.md copy list is incomplete vs files on disk. Confirm Studio has:

- `splash_loading.png`
- `ic_launcher_photo.jpg` / foreground / background
- `aurphyx_business_card_qr.png`
- `vap_menu_1.png`, `vap_menu_2_tb.png`
- Adaptive icon mipmap densities (`mdpi`…`xxxhdpi`) — **likely still missing**. Generate from `app-icon-1024.jpg`. `assets/logos/adaptive-foreground-432.jpg` exists and is **not** wired; manifest uses `@drawable/ic_launcher_photo`.
- PWA apple-touch still Grok `public/__grok/icon-180.png`, not the 1024 VAP icon.
- Round icon + Play-style feature graphic (later; no Play upload in v1)

### Screenshots (docs, not runtime)

`screenshots/vibe-{desktop,mobile}-{about,playing,vasp}.png` exist. Keep for README. Do not ship them inside the APK.

## Honest gaps

1. **Two codebases, one product.** Web orb is WebGL. Android orb is Compose Canvas with **simulated** chrom (BPM/position), not PCM FFT. They will drift unless uniforms stay a shared spec (`orb-uniforms.ts` ↔ Kotlin).
2. **Skinz not started** (BUILD_PLAN Phase 1). Tokens + Void / Sodium / Lattice.
3. **No v01d in this app.** WinAmp import belongs on VMP first; this player consumes `.vskin` / token JSON that VMP already transmuted.
4. **VASP cards are display.** Live analyser fills some Phase I fields on web; imports get hash hues and “pending” pillars. No sidecar writer.
5. **Web library already uses IndexedDB** (`library-db.ts`). Android uses SharedPreferences URIs. Missing: album art cache, queue reorder, last-skin persist on Android (`SceneSettings` reset on death).
6. **Demo pause/resume (web):** `togglePause()` stops the synth and does not restart it — user must re-select the track.
7. **Android transport:** repeat hardcoded `REPEAT_MODE_ALL`; **no shuffle UI**.
8. **Grok auth / Neon / PGLite** is template residue. Turn auth off for the shipped player.
9. **Studio tree is outside this git repo.** Drop-in can rot. Treat `android-studio-dropin/` as the committed Kotlin source of truth.

## Feature suggestions

**Stay a phone/web player. Do not become VMP.**

- PWA install on Android Chrome with the 1024 icon (sideload companion to the APK).
- Lock-screen / Media Session already on Android; web needs Media Session metadata + artwork from `art.ts`.
- Equalizer: gentle shelving only. Full EQ rack is VMP.
- Scene presets already named: Night Drive, Orbital Lattice, Forward Current. Keep photometric hex as the only vis driver besides the analyser.
- Optional: write `.vap.json` sidecar next to a local file (same folder). Read it on open. No upload.

**Skinz on this player = consume packs, do not invent a second transmuter**

```
VMP v01d  →  .vskin / vibe-skin.json
                ↓
        Vibe Audio Player (web CSS vars + Compose ColorScheme)
```

Built-ins: Void, Sodium, Lattice. User zip import in Phase 1.5. `.wsz` only if VMP is not running — even then, call the same FUTE crate, do not fork a JS unzip museum.

## Phased series to finish VAP

### Series VAP-0 — Ship v1 (this week)

1. Web: Orb moves on demo + local files; photometric hex changes with Scene. Fix demo pause/resume.
2. Kill or hide Grok login for the product route. Local-only.
3. Android: first frame is Orb. Shuffle + repeat UI. Persist Scene settings.
4. Generate mipmap launcher set from `app-icon-1024.jpg`; wire adaptive foreground from `assets/logos/`.
5. Confirm drop-in ↔ `C:\aurphyx\vibeaudioplayer` is byte-equal for Kotlin + drawables.

### Series VAP-1 — Skin tokens

1. `vibe-skin.json`: bg, surface, fg, accent, display font, radii, glow.
2. Three built-ins. Hot-swap without restarting audio.
3. Vis hole remains the orb.

### Series VAP-2 — Sidecar + library durability

1. Web IndexedDB already stores blobs — add last-played + last-skin. Android: Room or persist SceneSettings; do not pretend IndexedDB is missing.
2. Optional `.vap.json` read/write beside tracks.
3. Align displayed schema with `vasp/` **3.69**.

### Series VAP-3 — Plugs (not VST)

AudioWorklet / Media3 AudioProcessor: bypass, stereo width, limiter, night-drive filter. User WASM/JS from **local files only**.

### Series VAP-4 — Tribe local packs

`tribe.json` manifest. Open pack from disk. No billing. Tab already labeled in the art — do not ship an empty store.

## Binding

| Need | Where |
|---|---|
| Protocol | `vasp/` 3.69 |
| Desktop Skinz + v01d | `vibeplayer/` |
| Shader origin | `vibe-audio-visualizer/` |
| 3×3 / HIF meaning | `tslca/` — orb is visualization, not a new contraction |
| Persistence later | `aurafs/` only after put/get works |

## What not to do

- Do not rewrite the VLC plugin.
- Do not invent new menu chrome.
- Do not put the QR on first paint.
- Do not stub a fake VST scanner.
- Do not block the S24 APK on Fuxyez or AuraFS.
