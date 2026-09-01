# VASP Features & Capabilities Catalog

Every feature the Vibe Audio Player could plausibly have, organized around the 9-pillar VASP taxonomy plus the standard music-player surface area around it. This is a brainstorm/roadmap document, not a build spec — treat it as a backlog to pull individual features from into their own AI Studio/Cursor prompts.

**Legend**

- `[Speced]` — already in `VAP_TechSpec_v3.2.0` or `VibeTribe_Pillarz.md`
- `[New]` — natural extension, not yet speced, straightforward-to-moderate lift
- `[Moonshot]` — bigger lift, external hardware/API dependency, or a later-stage idea

---

## 1. Core Playback & Audio Engine

- `[Speced]` Multi-format high-res playback (FLAC/DSD/ALAC/WAV/MP3/AAC/OGG, up to 32-bit/384kHz)
- `[Speced]` Gapless playback
- `[Speced]` Configurable crossfade (0–12s)
- `[Speced]` Pitch-corrected variable speed (0.5x–2.5x)
- `[Speced]` Sleep timer
- `[Speced]` MediaStore + custom URI library scanning
- `[New]` ReplayGain / library-wide loudness normalization (a direct consumer of the `rms_amplitude`/`dynamic_range_lra` data VASP already computes)
- `[New]` Waveform seekbar (reuse the decoded PCM from the DSP analysis pass instead of a flat progress bar)
- `[New]` A-B repeat / loop section
- `[New]` Bookmarks within long tracks (DJ sets, classical movements, podcasts)
- `[New]` Smart resume — remembers exact position per track, including long-form content
- `[Moonshot]` Stem/multi-track playback (isolated vocal/instrumental layers, if source separation is ever added)

## 2. VASP Pillar-Driven Features

The differentiator. Each pillar's data justifies at least one feature beyond just displaying it in an orb.

### Pillar 1 — STRUCTURAL (rhythm/tempo)

- `[New]` **Vibe Mix auto-DJ transitions** — tempo/groove-aware crossfade timing instead of a flat crossfade window
- `[New]` **Cadence-lock running mode** — auto-queue tracks whose BPM matches real-time step cadence (accelerometer)
- `[New]` **Tap-to-correct BPM** — user taps along, corrects a wrong DSP guess, correction feeds the Tribe overlay
- `[New]` **Energy-ordered queue sort** — reorder a playlist by increasing/decreasing BPM or `MET_SCORE` so a set actually builds

### Pillar 2 — TONAL (harmony/key)

- `[New]` **Camelot Wheel harmonic browser** — visual key-compatibility wheel for DJ-style set building (ties to the Camelot question already in `VibeTribe_Pillarz.md`)
- `[New]` **Key-aware auto-transpose** — nudge pitch on transitions so back-to-back tracks land in compatible keys
- `[New]` **Mood-key library filter** — browse by major/minor "feel"

### Pillar 3 — TIMBRAL (texture/fidelity)

- `[New]` **Smart EQ suggestion** — feed `SPECTRAL_CENTROID`/`FIDELITY_SCORE` into the existing 10-band EQ to auto-suggest a starting curve per track
- `[New]` **Low-fidelity flagging** — surface tracks with poor `FIDELITY_SCORE`/bitrate so the user knows what to re-rip or replace
- `[New]` **Output-aware timbral compensation** — different auto-EQ baseline for wired vs. LDAC vs. phone speaker

### Pillar 4 — LINGUISTIC (voice/lyrics)

- `[New]` **Explicit filter / family mode** — auto-skip `Explicit`/`Severe` tracks, direct consumer of the field the engine already computes
- `[New]` **Topic-based smart playlists** — auto-generate "breakup songs," "hype/gym," etc. from `TOPIC_CLUSTERS`
- `[New]` **Language browse/filter** — "play something in Spanish," from `PRIMARY_LANGUAGE`
- `[Speced]` Time-synced karaoke lyrics with tap-to-seek
- `[Moonshot]` Pitch-tracked sing-along scoring ("how close are you to the melody")
- `[Moonshot]` Auto-translated lyric overlay

### Pillar 5 — AFFECTIVE (mood)

- `[New]` **Mood Map** — 2D Valence/Arousal scatter plot of the whole library, tap a point to play that region
- `[New]` **Vibe Radio** — on-device nearest-neighbor auto-queue by Thayer coordinates, no cloud ML required
- `[New]` **Mood history / "your week in vibes"** — lightweight local log of listening mood over time
- `[Moonshot]` Mood-triggered suggestions ("you usually play X around now")

### Pillar 6 — CONTEXTUAL (scene)

- `[New]` **Manual scene presets** — one-tap "Gym / Focus / Sleep / Night Drive" pulling from `MACRO_SETTING`-matched tracks
- `[Moonshot]` **Auto-context detection** — activity recognition + time of day + (optionally) weather API to auto-suggest a scene switch
- `[Moonshot]` Google Assistant voice-triggered scenes

### Pillar 7 — PHOTOMETRIC (light)

- `[Speced]` Dynamic UI background mesh/particles from `PRIMARY_HEX`/`SECONDARY_HEX`
- `[Speced]` Full-screen reactive visualizer (`VisualzScreen.kt`)
- `[Moonshot]` **Smart bulb sync** — Philips Hue / Govee / LIFX / Nanoleaf integration driven by the chromatic map + lumen dynamics in real time
- `[Moonshot]` Android Auto ambient lighting sync, where the car supports it
- `[Moonshot]` Screen-flash strobe effects synced to `STROBE_TRIGGER`
  - **Safety note:** any strobe/flash feature needs a clear seizure-risk warning, opt-in (not default-on), and an always-visible off switch — don't ship this without an accessibility toggle to disable it entirely.

### Pillar 8 — KINETIC (body)

- `[Moonshot]` **Wear OS / Health Connect integration** — read live heart rate, auto-select tracks whose `TARGET_HR_ZONE` matches current real HR
- `[Moonshot]` Post-workout cooldown auto-queue (descending `MET_SCORE`)
- `[New]` **Haptic bass** — phone vibration pulse synced to detected kick transients (novelty feature, cheap to build once `attack_ms`/onset data exists)

### Pillar 9 — GENEALOGICAL (roots)

- `[New]` **Sample DNA explorer** — tap a track, see a visual family tree of what it sampled / what sampled it, linking related library tracks
- `[New]` **Era browser** — browse "by decade it feels like," from `CULTURAL_ERA`
- `[New]` **Genre tree explorer** — interactive Parent → Sub → Micro genre taxonomy browser
- `[Speced]` Tribe authenticity voting (`AUTHENTICITY_SCORE`, `VIRAL_VELOCITY`)

##  3. Vibe Tribe / Social Layer

- `[Speced]` VASP voting button — 3×3 lattice, up to 13 questions per pillar (`VibeTribe_Pillarz.md`)
- `[Speced]` Harvest → DSP → Tribe overlay promotion pipeline
- `[New]` Tribe contribution profile (badges, pillars voted, streaks)
- `[New]` Shareable "Vibe Card" — a track's VASP fingerprint as a shareable graphic, styled off its own photometric palette
- `[Moonshot]` Compare-your-vibe-to-friends — social Valence/Arousal comparison on a shared track
- `[Moonshot]` Collaborative playlists with group-average vibe compatibility scoring

## 4. Discovery & Library Intelligence

- `[New]` VASP similarity search — "more like this" via multi-pillar vector distance
- `[New]` On-device Daily/Weekly Vibe Mix generation (no cloud dependency, uses cached VASP vectors + listening history)
- `[New]` Duplicate detection via ISRC/fingerprint across formats
- `[New]` Library health dashboard — format breakdown, average fidelity, % of library VASP-scored
- `[New]` Background library-wide VASP scan ("scan my whole library overnight while charging") — natural extension of the per-track on-demand DSP pass
- `[New]` Sidecar JSON export/import of VASP profiles, matching `VASP_Official_Schema.md`
- `[Moonshot]` ListenBrainz / Last.fm scrobbling

## 5. Hardware & Ecosystem Integration

- `[Speced]` Bluetooth LE Audio (LC3), Sony LDAC, USB-C bit-perfect DAC, Google Cast, wired 3.5mm
- `[New]` Home screen + lock screen widgets
- `[Moonshot]` Android Auto / Automotive OS support
- `[Moonshot]` Wear OS companion / standalone playback
- `[Moonshot]` Smart lighting integration (see Photometric section)
- `[Moonshot]` Health Connect integration (see Kinetic section)

## 6. Accessibility

Not an afterthought — call these out as first-class, especially given the photometric/strobe features above:

- `[New]` TalkBack / screen-reader support across all screens, including the 9-orb VASP display
- `[New]` Large-text and high-contrast modes
- `[New]` Colorblind-safe alternative palette for anything that relies on hue to convey meaning (chromatic map, mood map, orb colors)
- `[New]` Global "reduce motion / disable flashing effects" switch, on by default for strobe-adjacent features
- `[New]` Captioned/lyric-only mode for hearing-impaired users where lyrics exist

## 7. Standard Player Table-Stakes

- `[New]` In-app ID3/Vorbis tag editor
- `[New]` Folder-based browsing alongside library view
- `[New]` Rule-based smart/dynamic playlists ("Recently Added AND BPM > 120")
- `[New]` Playlist import (M3U, exported CSVs from other services)
- `[New]` Metadata/playlist backup & restore (e.g. to Drive)
- `[Moonshot]` Multi-user profiles on one device
- `[New]` Battery/data-saver mode — pause background VASP scanning on low battery

## 8. Platform & Monetization (optional — include only if you want a paid tier)

- `[Moonshot]` Free tier: playback + basic VASP display; Pro tier: full DSP scanning, Tribe voting, smart-lighting/wearable integrations
- `[Moonshot]` One-time unlock vs. subscription — a business decision, not a technical one, flagged here only so it's not forgotten when scoping later phases

---

## Suggested prioritization

This list is intentionally larger than any one build phase. A reasonable next slice, in rough order of leverage-per-effort:

1. Waveform seekbar + ReplayGain (cheap, reuses DSP output you're already building)
2. Vibe Radio / Mood Map (high perceived value, no new external dependency)
3. Smart EQ suggestion (ties two existing systems — VASP + the 10-band EQ — together)
4. Background library-wide scan (makes everything above actually usable at scale instead of one track at a time)

Everything tagged `[Moonshot]` assumes hardware/API integrations outside the app itself (wearables, smart bulbs, Android Auto) — worth flagging as a distinct later phase rather than mixing into core app work.
