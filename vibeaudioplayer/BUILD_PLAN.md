# Vibe Audio Player — build plan

Two surfaces stay distinct: the **web player** in this folder, and the **Android Studio app** at `C:\aurphyx\vibeaudioplayer` (package `org.aurphyx.vibeaudioplayer`). The VLC plugin in `vibe-audio-visualizer/` is not rewritten.

Art lives in `C:\rossaedwards\ecosys\vibeaudioplayer\assets\`. Do not invent new menu chrome.

---

## Where the code is now (absorbed)

| Surface | Status |
|---|---|
| Web Orb tab | TSLCA orb via WebGL port of `vibe.frag` + bloom (2D fallback). Live analyser → `u_chrom_energy[4]` and the rest of the VLC uniform block. |
| Web tabs | Orb · Library · VASP · Scene · About. Menu glyphs crop `assets/vap_menu_*_tb.png`. |
| About Aurphyx | Sheet/tab. **13 taps** on the About page reveals the Aurphyx business-card QR (`assets/aurphyx-business-card-qr.png`, vCard for Ross A. Edwards / Aurphyx LLC). |
| Android | Greeting replaced. Compose single-activity, four-plus About tabs, Media3 session service, demo WAV generators, SAF library, Compose Canvas orb, splash + launcher from `assets/`. `minSdk 26`. |
| Not in v1 | Winamp `.wsz` loader, VST3 process, Vibe Tribe store, Play Billing, uploads. |

---

## Product north star

A **local** player (no upload) whose home picture is the nine-pillar TSLCA orb, with a **Winamp-grade skin + plug surface**: exotic, fast, user-customizable chrome; DSP/visual plugs; a **Vibe Tribe** catalog for packing those. Native VST is a later **desktop host**, not a lie in the browser or on the S24.

---

## Phase 0 — Ship the v1 player (this pass)

1. Web Orb moves on demo + local files; photometric hex changes with Night Drive / Orbital Lattice / Forward Current.
2. Android first launch is the Orb tab, not Hello Android.
3. Library (SAF / OpenDocument), VASP nine cards, Scene toggles, lock-screen session, screen-off audio.
4. Studio click-path: `ANDROID_STUDIO_APK.md`.

---

## Phase 1 — Skinz (Winamp-like, not a `.wsz` museum)

**Format:** `vibe-skin.json` + bitmap/SVG pack (classic Winamp *language*: main window, transport, EQ, playlist, vis hole — mapped onto Orb / Library / VASP / Scene).

| Piece | v1.1 | Later |
|---|---|---|
| Tokens | bg, surface, fg, accent, display font, radii, glow | Milkdrop-style vis envelope |
| Chrome | Docked transport, EQ sliders, playlist shade, vis viewport | Draggable Winamp windows, double-size, always-on-top (desktop) |
| Loader | Three built-in skins: **Void** (current), **Sodium** (night-drive), **Lattice** | User import zip; optional `.wsz` subset |
| Engine | CSS variables + Compose `ColorScheme`; hot-swap without restarting audio | GPU atlas for bitmap skins |

Do not flatten the orb into a generic vis. The vis hole *is* the TSLCA orb; skins frame it.

---

## Phase 2 — Customizable plugins (player plugs, not VST yet)

A **plug** is a sandboxed AudioWorklet (web) / AudioProcessor (Android Media3) plus optional overlay renderer.

Contract:

```
PCM in → plug.process(frames, vapUniforms) → PCM out
optional: plug.draw(orbUniforms)  // cannot replace vibe.frag unless declared vis-plug
```

Built-ins: bypass, stereo width, gentle limiter, “night drive” filter. User JS/WASM plugs on web only, loaded from local files (same as tracks — no upload).

---

## Phase 3 — VST (honest boundary)

| Host | VST3 / CLAP | Why |
|---|---|---|
| Web | No | Browsers cannot load native `.vst3`. |
| Android S24 | No in v1 | No supported VST host in this APK; NDK + GPL host would block shipping. |
| Later desktop | Yes | Separate Aurphyx host (not a Tauri rewrite of this player). Map VST params → VASP photometric / kinetic. |

Until that host exists, “VST” in the UI means **Vibe Plugs** (Phase 2) with a “VST coming on desktop” line. Do not stub a fake VST scanner.

---

## Phase 4 — Vibe Tribe (marketplace)

The menu art already labels **Vibe Tribe**. This is the catalog, not a social network.

- List **skins** and **plugs** as local-or-signed packages (`tribe.json` manifest: name, author, kind, hash, VASP tags).
- v1.5: curated built-in pack + “open pack from disk” (same local-only rule).
- v2: optional signed remote index. **No Play Billing, no ads** in this app. If money exists later, it is a separate Aurphyx store — not inlined here.
- Keep the Tribe tab distinct from Library.

---

## Phase 5 — About Aurphyx (done in v1)

- Tab uses the Vibe Tribe / right-column orb crop from `vap_menu_2_tb.png`.
- 13 taps on the About page unlocks the business-card QR (vCard).
- Do not put the QR on first paint.

---

## Key decisions

1. **Orb stays TSLCA** — skins and plugs wrap it; they do not replace `vibe.frag` unless a vis-plug is explicit.
2. **Local-first** — files, skins, and plugs are on-device. Tribe remote index is opt-in later.
3. **VST is desktop-later** — do not block the S24 APK or the web preview on a native plugin host.
4. **Two folders** — web in `vibeaudioplayer/`, APK in `C:\aurphyx\vibeaudioplayer`. Drop-in mirror: `android-studio-dropin/`.
5. **No Canva kit / Magic Media / Play Billing / ads** added as product surface.

---

## PR / slice order

1. **P0** v1 player + Android shell + About 13-tap QR (current).
2. **P1** Skin token engine + three built-in skins.
3. **P2** Plug host (Worklet / AudioProcessor) + three built-ins.
4. **P3** Tribe local pack loader + Tribe tab.
5. **P4** Desktop VST host (new tree, not this APK).
