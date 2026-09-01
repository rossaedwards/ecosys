# Vibe Audio Player — Project Context

**Product:** Vibe Audio Player (VAP)  
**Organization:** Aurphyx LLC  
**Author:** Ross A. Edwards  
**Protocol:** VASP / V.A.P. 3.69 (TSLCA 3² lattice)  
**Tree:** `rossaedwards/ecosys/vibeaudioplayer`  
**Updated:** 2026-08-21

Sibling product: [Vibe Media Player](../vibeplayer/PROJECT_CONTEXT.md).

## Purpose

A web player that plays local and catalog audio **and** emits a complete, honest 9-pillar VASP profile for whatever is playing. Framez is the chrome (Fluxbox-style snap + Compiz-style compositing + vertical title tabs).

## Non-negotiable rules

1. Filename and ID3 are sources of truth for **IDENTITY** only (artist, title, album).
2. Experiential pillars are nested 3.69 JSON (`PILLARS.STRUCTURAL.TEMPORAL_DYNAMICS.BPM_RAW`, …), not a flattened Wikipedia string.
3. No field is `known` without catalog, tag, or DSP evidence.
4. Missing measurement → `unknown` / `null` + input tag (`user` | `professional` | `tribe`).
5. Artist biography never fills Linguistic, Affective, Photometric, Kinetic, or Tonal.
6. Grok transmute is user-initiated (Refresh), cached, and must not invent key, valence, or genre.
7. Secrets stay in `.env.local`. Never commit TIDAL or Google credentials.
8. Tribe answers land in a **Tribe overlay** first. Do not extra-key official `PILLARS` from a vote. Canon: [`../vasp/VibeTribe_Pillarz.md`](../vasp/VibeTribe_Pillarz.md).

## Architecture (as built)

```
Browser UI (React 19 + Framez)
    ↔ TanStack Start server fns + authMiddleware
    ↔ PGLite / Neon  (loved_tracks, vasp_cache, tidal_oauth, tidal_tokens)
    ↔ Public catalogs (MusicBrainz, iTunes, Deezer, AudioDB, Wikipedia song pages)
    ↔ TIDAL OAuth PKCE  (login.tidal.com / auth.tidal.com)
    ↔ Google session via Grok broker  (grok-google) — not YouTube Data API likes
    ↔ Web Audio EQ / compressor
```

Key paths:

| Path | Role |
| --- | --- |
| `src/lib/vasp/parse.ts` | `Artist - Title` / underscore parse, cache keys |
| `src/lib/vasp/catalog.ts` | Library sources, file → identity, demo tracks |
| `src/lib/vasp/lookup.server.ts` | Parallel public lookup + optional Grok merge |
| `src/lib/vasp/actions.ts` | enrich, Loved, TIDAL begin / sync |
| `src/routes/api/tidal/callback.ts` | PKCE token exchange |
| `src/components/framez/` | Desktop, panes, snap |
| `migrations/0002_vasp_library.sql` | Loved + cache + TIDAL tables |

Reference engine (not yet a `vap-analyze` CLI in this tree):

- `../vasp/VASP_Scoring Engine.py.md` — `VASPScoringEngine`
- `../vasp/vasp_sdk-api_streaming.md` — `VapEngine::transform_spotify`
- `../vasp/VASP_Official Schema.md`
- `../vasp/VASP_Context_Simulation.md`
- `../vasp/VibeTribe_Pillarz.md` — 9 × 13 voter copy (overlay only this pass)

## Auth status (2026-08-21)

| Provider | Status |
| --- | --- |
| Google / YouTube | Sign-in via Grok broker. Session is not `youtube.readonly`. Do not claim Liked Videos pull until a Data API client exists. |
| TIDAL | Developer app **Vibe Audio Player** created. Client ID + secret belong in `.env.local`. Dashboard still needs Redirect URIs and Allowed scopes (`user.read`, `collection.read`). Redirect: `http://localhost:8080/api/tidal/callback` and `http://127.0.0.1:8080/api/tidal/callback`. |
| Spotify | Historical mapper exists (`transform_spotify`). New-app audio-features is 403; do not depend on it for new tracks. |

## Why Tech N9ne — The Waitress looked wrong

Lookup hits existed. Mapping stuffed artist wiki text into lyrics / mood / contour and marked local placeholders (`Local playback`, `Local library`) as `known`. Golden PDFs worked because they are complete authored profiles, not that path.

Fix direction: identity from name/ID3 → catalog facts only where evidenced → DSP/scoring for numbers → `unknown` otherwise.

## Current gaps

- Nested 3.69 JSON is not yet the pane’s canonical store (flattening still leaks).
- No on-disk `engine/src/dsp_analyzer.py` in repo; scoring is the markdown reference plus `crates/vmp-vap` in VMP.
- ID3 parse and Essentia / GetSongBPM not wired as first-class `known` sources.
- YouTube Music / TIDAL Loved auto-pin depends on completed OAuth + scopes.
- Windows 11 launch not yet verified by the owner after clone.
- VASP 3×3 Tribe UI not built this pass.

## File naming convention

Preferred: `Artist - Song Title`. Also accepted: `Artist_Album_SongTitle`. Parser must not invent album from a two-part hyphen name.

## Accessibility

High-contrast Framez chrome. Prefer OpenDyslexic / readable type where the shell allows it. Ask AI is a first-class title-tab action.

## Related

- [README.md](README.md)
- [EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md)
- [../vibeplayer/](../vibeplayer/) — native player
- [../vasp/](../vasp/) — protocol
- [Vibe Tribe Pillarz](../vasp/VibeTribe_Pillarz.md) — VASP 3.69 voter copy (canon)
