# Vibe Audio Player

**Aurphyx LLC** · web player for [VASP 3.69](../vasp/README.md)

Browser player and Framez control deck. Local files and streaming catalogs are treated as **identity** (who is playing). Experiential pillars come from ID3, catalog facts, DSP / scoring, then user–pro–tribe input — never from an artist Wikipedia dump.

The desktop / Rust host is **[Vibe Media Player](../vibeplayer/README.md)**.

## What this is

| Layer | Role |
| --- | --- |
| Player | Local library, queue, EQ / compressor, now-playing |
| Framez | Overlay window manager: vertical tabs, title chrome (Ask AI · minimize · fullscreen · exit), snap / dock / lock |
| VASP pane | Nine nested 3.69 pillars for the current track |
| Library pins | TIDAL, YouTube, YouTube Music, Loved |
| Auth | Google via Grok broker (`grok-google`). TIDAL is a vendor OAuth app named **Vibe Audio Player** |

## Stack

TanStack Start / Router / Query · React 19 · Vite 8 · Tailwind v4 · Zustand · Better Auth · PGLite / Neon · Web Audio

Preview binds `0.0.0.0:8080` (`startup.sh` / `npm run dev`).

## Quick start (Windows 11)

```bash
git clone https://github.com/rossaedwards/ecosys.git
cd ecosys/vibeaudioplayer
npm install
copy .env.example .env.local   # if present; otherwise create .env.local
npm run dev
```

Open `http://localhost:8080` (or `http://127.0.0.1:8080`).

### TIDAL (optional)

Developer app name: **Vibe Audio Player** (product, not the protocol).

`.env.local` — do not commit:

```env
TIDAL_CLIENT_ID=
TIDAL_CLIENT_SECRET=
TIDAL_REDIRECT_URI=http://localhost:8080/api/tidal/callback
```

Dashboard must list those exact redirect URIs **and** read scopes (`user.read`, `collection.read`). Empty Redirect URIs / Allowed scopes will fail authorize before a code is issued.

## VASP pipeline (source of truth)

1. **Identity** — filename (`Artist - Song Title` or `Artist_Album_SongTitle`) and ID3 (`TPE1`, `TIT2`, `TALB`, `TBPM`, `TKEY`, `TCON`).
2. **Catalog** — MusicBrainz / iTunes / Deezer / AudioDB / TIDAL / song-specific Wikipedia only. Artist bios do not fill lyrics, mood, contour, or atmosphere.
3. **Analysis** — ID3 BPM/key when present; then GetSongBPM / ReccoBeats / in-browser DSP. TuneBat has no public API.
4. **Scoring** — `VASPScoringEngine` / `VapEngine` (`../vasp/VASP_Scoring Engine.py.md`, `../vasp/vasp_sdk-api_streaming.md`).
5. **Status** — `known` only with evidence. Else `unknown` / `null` and tag the track for user, professional, or tribe input.
6. **Context** — GYM_PEAK / NIGHT_DRIVE style overrides need numeric `BPM_RAW`, `AROUSAL`, `MET_SCORE`.

Golden fixtures (Cannibal Corpse, Céline Dion, Stuca) are **hand-authored** complete 3.69 objects. They are not the auto-lookup path.

## Docs in this folder

- [PROJECT_CONTEXT.md](PROJECT_CONTEXT.md) — decisions, gaps, auth, Windows notes
- [EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md) — short brief
- [docs/VASP_README.md](docs/VASP_README.md) — protocol front door

Canonical schema and engine live in [`../vasp/`](../vasp/).

## License

See repository root. Product of Aurphyx LLC.
