# Vibe Audio Player

**Aurphyx LLC** · web player for [VASP 3.69](../vasp/README.md)

Browser player and Framez control deck. Local files and streaming catalogs are **identity**. Experiential pillars come from ID3, catalog facts, DSP / scoring, then Tribe overlay — never from an artist Wikipedia dump.

Desktop / Rust host: **[Vibe Media Player](../vibeplayer/README.md)**.  
Tribe questions (canon only): **[../vasp/VibeTribe_Pillarz.md](../vasp/VibeTribe_Pillarz.md)**.

## Two trees

| Where | What |
| --- | --- |
| This folder (`ecosys/vibeaudioplayer`) | Web player (Vite / React / TanStack). Framez, library, VASP pane. |
| `vibe-audio-visualizer/` | VLC plugin tree. Leave it. |

## What this is

| Layer | Role |
| --- | --- |
| Player | Local library, queue, EQ / compressor, now-playing |
| Framez | Overlay WM: vertical tabs, title chrome, snap / dock / lock |
| VASP pane | Nine nested 3.69 pillars |
| Library pins | TIDAL, YouTube, YouTube Music, Loved |
| Auth | Google via Grok broker (`grok-google`). TIDAL vendor OAuth app name: **Vibe Audio Player** |

## Stack

TanStack Start / Router / Query · React 19 · Vite 8 · Tailwind v4 · Zustand · Better Auth · PGLite / Neon · Web Audio

Preview: `0.0.0.0:8080` (`startup.sh` / `npm run dev`).

```bash
cd ecosys/vibeaudioplayer
npm install
npm run dev
```

Open `http://localhost:8080`.

### TIDAL (optional)

`.env.local` — do not commit:

```env
TIDAL_CLIENT_ID=
TIDAL_CLIENT_SECRET=
TIDAL_REDIRECT_URI=http://localhost:8080/api/tidal/callback
```

Dashboard needs those exact redirect URIs and read scopes (`user.read`, `collection.read`).

## Pipeline

1. Identity — filename / ID3.
2. Harvest — catalogs and credits. Bios stay out of pillars.
3. DSP / scoring — numbers from the file.
4. Tribe — VASP button, 3×3, up to 13 questions. Overlay first. See the Pillarz spec.
5. Unknown stays `null`.

## Docs

- [PROJECT_CONTEXT.md](PROJECT_CONTEXT.md)
- [EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md)
- [docs/VASP_README.md](docs/VASP_README.md)
- [`../vasp/`](../vasp/)
