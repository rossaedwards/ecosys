# Vibe Audio Player

Local music player + TSLCA orb. Nothing is uploaded.

## Two folders

| Where | What |
|---|---|
| **This repo** `C:\rossaedwards\ecosys\vibeaudioplayer` | Web player (Vite / React / TanStack). Orb tab ports `vibe-audio-visualizer` shaders. Library / VASP / Scene / About. |
| **`C:\aurphyx\vibeaudioplayer`** | Android Studio app `org.aurphyx.vibeaudioplayer`. Same four-plus-About tabs, Media3, demos. See `ANDROID_STUDIO_APK.md`. Mirror: `android-studio-dropin/`. |
| **`vibe-audio-visualizer/`** | VLC plugin (CMake, `vibe_visualizer.c`). Leave it. The Orb tab speaks the same renderer language. |

Art: `assets/` (`vap_menu_*`, loading splashes, `app-icon-1024.jpg`, `aurphyx-business-card-qr.png`).

Roadmap (skins, plugs, VST boundary, Vibe Tribe): `BUILD_PLAN.md`.

## Web

```sh
npm run dev
```

Home is the Orb. About → tap the page 13 times for the Aurphyx business-card QR.
