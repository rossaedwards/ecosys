# VASP Field Sourcing Matrix

How every `raw_data` key the scoring engine expects actually gets populated on-device, extending the Harvest/DSP/Tribe layering already defined in `VibeTribe_Pillarz.md`.

## Legend

| Tier | What it means |
| --- | --- |
| **A — Tag Harvest** | ID3/Vorbis tags already on the file, or a network catalog lookup by ISRC/fingerprint. Free, instant, high confidence. |
| **B — On-device DSP** | One-time background job: decode the full file (`MediaExtractor`/`MediaCodec` → PCM, no `RECORD_AUDIO` needed), run TarsosDSP or custom FFT, cache the result. |
| **C — Lyrics NLP** | Only runs if the file has embedded/`.LRC` lyrics. On-device ML Kit for language ID; a single structured Gemini call for anything semantic. |
| **D — Catalog API** | MusicBrainz (free, CC0) / Discogs (needs token) / Beatport-Traxsource (EDM catalog only) / Billboard (hint only). Best-effort, sparse coverage, network-dependent. Spotify intentionally excluded per your existing project constraint. |
| **E — Derived** | Computed deterministically from A/B/C outputs already in memory. Not independently measured, but principled — not a guess. |
| **F — Tribe overlay only** | No honest automated source exists. Stays `null` until a human answers the corresponding question in `VibeTribe_Pillarz.md`. |

Anything not Tier A/B/C/D/E stays `null`/`"unknown"` — never fill a gap with invented precision.

---

## IDENTITY

| Field | Tier | Method |
| --- | --- | --- |
| `title`, `artist` | A | ID3/Vorbis tags |
| `isrc` | A / D | ID3 tag if present, else MusicBrainz lookup |
| `source_dna` | A | Internal catalog id you assign on ingest |

## STRUCTURAL

| Field | Tier | Method |
| --- | --- | --- |
| `bpm` | A → B | ID3 `TBPM` if present; else onset-interval autocorrelation (TarsosDSP) |
| `bpm_perceived` | E | Compare detected tempo against octave-ambiguous candidates the beat tracker already produces |
| `groove_quantization` | B | Variance of onset timing vs. quantized grid |
| `time_signature` | F | Meter detection not practical for v1 (revisit with Essentia in v2) |
| `sectional_markers` | F | Structural segmentation — v2+ |
| `mix_window_index` | F | Depends on segmentation — v2+ |
| `breakdown_depth` | F | Depends on segmentation — v2+ |
| `attack_ms` | B | Percussion onset detector: −60dB → peak time on strongest low-end transient |
| `syncopation_index` | B | Ratio of off-beat to on-beat onset energy |
| `ghost_note_density` | F | Needs velocity-layer analysis — v2+ |

## TONAL

| Field | Tier | Method |
| --- | --- | --- |
| `key_mode` / `key_signature` | B (moderate effort) | Chroma / pitch-class profile + Krumhansl-Schmuckler key-finding on top of TarsosDSP's pitch primitives |
| `chord_complexity` | F | Chord recognition is its own MIR task — v2+ |
| `dissonance_rating` | E | Engine already defaults this from `key_mode`; real interval-tension analysis is v2+ |
| `range_span`, `melodic_motion` | F | Needs isolated melody/pitch-track extraction — v2+ |
| `hook_strength` | F | Needs repetition/self-similarity analysis — v2+ |
| `reference_pitch` | B (low priority) | Precise pitch detection on sustained tones |
| `microtonality` | F | Needs fine pitch-deviation tracking — v2+ |

## TIMBRAL

| Field | Tier | Method |
| --- | --- | --- |
| `centroid_hz` | B | FFT spectral centroid — straightforward, same analysis pass as `dominant_freq_hz` |
| `sub_dominant`, `mid_forward`, `air_brilliance` | B | Band-energy ratios from the same FFT pass |
| `spectral_saturation` | B | Harmonic-to-noise ratio estimate |
| `fidelity_score` | **E (free)** | Already have `sampleRateKhz` / `bitDepth` / `bitrateKbps` on `TrackItem` — just threshold those |
| `dynamic_range_lra` | B | Loudness range across RMS windows over the full decode |
| `spatial_width` | A / B | Channel format flag, refined by L/R correlation if you want more than mono/stereo |
| `texture_surface`, `texture_artifacts` | F | Subjective / needs a specialized classifier — v2+ or Tribe |

## LINGUISTIC

| Field | Tier | Method |
| --- | --- | --- |
| `explicit_filter` | C | Deterministic wordlist scan on lyric text — matches the engine's own documented "Safety List" logic, no LLM needed |
| `topic_clusters`, `narrative_arc` | C | Single structured Gemini call on lyric text |
| `primary_language` | C | ML Kit Language ID — on-device, free, instant |
| `vocal_position`, `delivery_style`, `vocal_processing` | F | Needs vocal source separation + classifier — v2+/Tribe for v1 |
| `dialect_slang` | F | No clean automated source |

## AFFECTIVE

| Field | Tier | Method |
| --- | --- | --- |
| `sentiment_score` | C / E | Gemini call on lyrics if present; default `0.0` for instrumentals (don't leave the required key missing) |
| `rms_amplitude` | B | Average/peak RMS across the full decode |
| `mood_stability` | F | Needs section-level segmentation — v2+ |
| `catharsis_potential`, `nostalgia_trigger` | F | Genuinely subjective — Tribe |
| `build_up_velocity`, `resolution_state` | F | Needs segmentation / ending analysis — v2+ |

## CONTEXTUAL

| Field | Tier | Method |
| --- | --- | --- |
| `MACRO_SETTING` / `MICRO_ACTIVITY` / `SOCIAL_SETTING` / `FUNCTIONAL_GOAL` / `TIME_OF_DAY` | **E (already built)** | Engine already derives these from `bpm` + `arousal` + `sentiment` — no new sourcing needed |
| `weather`, `temperature` | Out of scope | These describe listening context, not the track — a live GPS+weather tag would be a different feature entirely |

## PHOTOMETRIC

| Field | Tier | Method |
| --- | --- | --- |
| `dominant_freq_hz` | B | FFT peak-magnitude bin, same pass as `centroid_hz` |
| `secondary_hex` | E | Second-strongest FFT band through the same hex mapping table |
| `brightness_floor`, `brightness_ceiling` | E | From the RMS dynamic-range data already computed in B |
| `strobe_trigger` | E | From `syncopation_index` / attack sharpness already computed in B |
| `fog_density`, `visual_noise` | E | From `spectral_saturation` |
| `laser_compatibility` | E | Simple BPM/energy threshold |
| `fade_rate` | F | Needs real transition detection from segmentation — v2+ |

## KINETIC

| Field | Tier | Method |
| --- | --- | --- |
| `TARGET_HR_ZONE`, `HRV_IMPACT`, `DRIVE`, `HEAD_NOD`, `MET_SCORE` | **E (already built)** | Engine already derives these from `bpm`/`arousal` |
| `breath_rate` | F | "BPM ÷ 4" is invented precision, not a measurement — leave null unless you do real cadence analysis |
| `sway` | F | No honest automated source |

## GENEALOGICAL

| Field | Tier | Method |
| --- | --- | --- |
| `release_date` | A / D | ID3 date tag, else MusicBrainz lookup by ISRC/fingerprint |
| `cultural_era` | **E (free)** | Deterministic bucket from `release_date` year (pre-1990 / 90s–00s / 2010s / now) |
| `genre_tree` | D | Discogs styles / MusicBrainz genre tags / Beatport genre, mapped through a taxonomy you curate once |
| `sample_lineage`, `interpolation` | D (sparse) | MusicBrainz "Work" relationships only — these are CC0-licensed. Don't scrape WhoSampled or similar; no public API and murky ToS |
| `subculture_id`, `authenticity_score` | F (by design) | These are explicitly the Tribe-vote fields per `VibeTribe_Pillarz.md`'s own "Authenticity Ratio" definition |
| `viral_velocity` | D (hint) + F | Billboard/chart position as a weak hint, reinforced by Tribe votes — matches your existing harvest map |

---

## Recommended build order

1. **Tag harvest + free derivations** — `IDENTITY`, most of `GENEALOGICAL`, `fidelity_score`, `cultural_era`. Zero new dependencies, works today.
2. **Offline on-device DSP pass** (TarsosDSP, decode-once-and-cache) — unlocks real `STRUCTURAL` transient/syncopation data, `TIMBRAL` spectral fields, `AFFECTIVE.AROUSAL`, `PHOTOMETRIC` chromatic mapping.
3. **Lyrics NLP** (ML Kit + one Gemini call, only when lyrics exist) — unlocks real `LINGUISTIC`, refines `AFFECTIVE.VALENCE`/sentiment.
4. **Catalog API enrichment** (MusicBrainz first — free, CC0) — deepens `GENEALOGICAL`.
5. **Tribe overlay** — already fully spec'd in `VibeTribe_Pillarz.md`; this is where everything in tier F belongs, not a gap to solve in code.

Every orb should render as dim/inert for whatever hasn't been reached yet in this sequence — that's a legitimate "unmeasured" state, not a bug.
