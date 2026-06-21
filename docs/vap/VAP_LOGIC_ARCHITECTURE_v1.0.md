# Vibe Audio Protocol (V.A.P.) — Logic Architecture v1.0

> **Aurphyx SUXS — Vibe Audio Standard / Vibe Audio Protocol**  
> Scoring Systems & Rating Definitions  
> Parent Framework: TSLCA (Three-Squared-Lattice Cognitive Architecture)

---

## Phase I: Physical Analysis (DSP Layer)

*Automated extraction of raw audio data using Digital Signal Processing.*

---

### Section 1: Structural Pillar (The Skeleton)

#### Subset 1.1: Percussive DNA

**Metric: Kick Transient Profile (ms)**

- *Logic:* Measurement of time (ms) from signal onset (−60dB) to Peak Amplitude (0dB).
- *Scoring:*
  - **Sharp (Click):** < 10ms — Tech/Metal
  - **Punch (Thud):** 10–30ms — Pop/Rock
  - **Boom (Sub):** > 30ms — Trap/808

**Metric: Syncopation Index (0.0 – 1.0)**

- *Logic:* Ratio of transient energy detected on weak beats vs. strong beats (1, 2, 3, 4).
- *Formula:* `OffBeat_Transients / Total_Transients`
- *Scoring:*
  - `0.0` = Pure March (4/4 stomp)
  - `1.0` = Polyrhythmic Chaos (Jazz/Math Rock)

---

### Section 2: Tonal Pillar (The Flesh)

#### Subset 2.1: Harmonic Profile

**Metric: Dissonance Density (%)**

- *Logic:* Percentage of total duration containing "high tension" intervals (Minor 2nd, Tritone, Major 7th).
- *Scoring:*
  - **Consonant:** < 10% — Pop/Folk
  - **Tense:** 10–40% — Jazz/Noir
  - **Dissonant:** > 40% — Horrorcore/Avant-Garde

---

### Section 3: Timbral Pillar (The Skin)

#### Subset 3.1: Spectral Physics

**Metric: Spectral Centroid (Hz)**

- *Logic:* The "center of gravity" of the frequency spectrum.
- *Scoring:*
  - **Dark/Muddy:** < 200Hz
  - **Warm/Body:** 200Hz – 2000Hz
  - **Bright/Airy:** > 2000Hz

**Metric: Saturation Index (THD)**

- *Logic:* Total Harmonic Distortion measurement relative to the fundamental frequency.
- *Scoring:* `0.0` (Pure Sine) → `1.0` (White Noise/Bitcrush)

---

## Phase II: Psychological Analysis (NLP & ML Layer)

*Machine Learning models analyzing context, semantics, and human perception.*

---

### Section 4: Linguistic Pillar (The Voice)

#### Subset 4.1: Semantic Content

**Metric: Explicit Severity Tier (1–4)**

- *Logic:* Keyword density analysis against Safety Lists (Violence, Substance, Sexual).
- *Scoring:*
  1. **Clean:** 0 matches
  2. **Mild:** < 3 matches (context dependent)
  3. **Explicit:** > 3 matches or specific "Hard" keywords
  4. **Severe:** Hate speech or extreme violence markers — Auto-Filter

---

### Section 5: Affective Pillar (The Heart)

#### Subset 5.1: The Thayer Coordinates

**Metric: Valence Score (−1.0 to +1.0)**

- *Logic:* Sentiment analysis of lyrics + Major/Minor key determination.
- *Scoring:*
  - `-1.0` = Despair/Terror (Minor Key + Negative Sentiment)
  - `0.0` = Neutral/Ambient
  - `+1.0` = Euphoria/Joy (Major Key + Positive Sentiment)

**Metric: Arousal Score (0.0 to 1.0)**

- *Logic:* RMS Amplitude (Loudness) + BPM + Spectral Density.
- *Scoring:* `0.0` (Sleep) → `1.0` (Panic/Rage)

---

### Section 6: Contextual Pillar (The Scene)

#### Subset 6.1: Scenario Probability

**Metric: Scenario Confidence Score (%)**

- *Logic:* Bayesian probability matching audio features to pre-defined clusters (e.g., "Night Drive").
- *Formula:* `P(NightDrive | Synthwave + 100bpm + MinorKey)`
- *Scoring:*
  - ≥ 85% = Auto-Tag applied

---

## Phase III: Environmental & Social Analysis (I/O Layer)

*Output formatting for hardware and community feedback loops.*

---

### Section 7: Photometric Pillar (The Eye)

#### Subset 7.1: Chromatic Map

**Metric: Dominant Wavelength (nm)**

- *Logic:* Mapping Low Frequencies → Long Wavelengths (Red); High Frequencies → Short Wavelengths (Blue/Violet).
- *Scoring:*

| Frequency Band | Range | Wavelength (nm) | Color |
|---|---|---|---|
| Sub-Bass | 28–60Hz | ~700nm | Deep Red |
| Kick/Low-Mid | 60–250Hz | ~600nm | Orange/Amber |
| Mids | 250Hz–2kHz | ~520nm | Green/Teal |
| Highs | 2kHz+ | ~450nm | Blue/Purple/UV |

---

### Section 8: Kinetic Pillar (The Body)

#### Subset 8.1: Biometric Entrainment

**Metric: Entrainment Factor (0–100)**

- *Logic:* Beat Salience (clarity of the pulse) combined with Repetition.
- *Scoring:*
  - < 30: Ambient/Drone — No motor response
  - 30–70: Head Nod — Complex rhythms
  - > 70: Body Lock — Four-on-the-floor dance

**Metric: MET Score (Metabolic Equivalent)**

- *Logic:* BPM mapped to physical exertion levels.
- *Scoring:*

| MET | BPM Range | Activity |
|---|---|---|
| 1.0 | 0–60 BPM | Rest |
| 3.0 | 80–100 BPM | Walk |
| 8.0+ | 140+ BPM | Sprint/HIIT |

---

### Section 9: Genealogical Pillar (The Roots)

#### Subset 9.1: Tribe Alignment (GVS)

**Metric: Authenticity Ratio (GVS)**

- *Logic:* `(Positive Votes by "Tribe Members" / Total Votes)`
- *Definition:* "Tribe Member" = user with > 50 hours listening to that specific sub-genre.
- *Scoring:*
  - **True:** > 80% approval from Tribe
  - **Mainstream/Poser:** < 40% approval from Tribe

---

*Document: VAP Logic Architecture v1.0 — Part of Aurphyx SUXS Vibe Audio Standard*  
*See also: `VAP_TAXONOMY_v3.1.md`, `vap_schema_v3.1.json`, `docs/tslca_overview.md`*
