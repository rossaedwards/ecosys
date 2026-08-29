---
type: implementation-note
title: VASP Android Gradle module
description: Android library for VASP 3.69 scoring with kotlinx.serialization, for the Google AI Studio Vibe Audio Player.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - SoulSync
domains:
  - audio
  - xessability
nodes:
cores:
  - VASP
fields:
---

# VASP Android module (`:vasp`)

Kotlin Android library for **VASP 3.69**. Drop this folder into the Google AI Studio Android app as module `:vasp`. It does not belong in the Claude `vibeaudioplayer` tree or the Rust `vibemediaplayer` desktop host.

- **minSdk 26** · **compileSdk 35** · Kotlin 2.0+ · AGP 8.8.1 compatible
- Nested protocol types (`VaspObject` / nine `PILLARS`)
- [VaspScoringEngine](src/main/java/org/aurphyx/vasp/VaspScoringEngine.kt) — same heuristics as `VASP_Scoring Engine.py`
- [PlayerVaspProfile](src/main/java/org/aurphyx/vasp/player/PlayerVaspProfile.kt) — flattened readout matching `VAP_TechSpec.md` §4.2 (`TrackItem.vaspProfile`)

## Include in the AI Studio app

Copy this directory to `<studio-project>/vasp/` (so `vasp/build.gradle.kts` sits next to `app/`).

**Root `settings.gradle.kts`**

```kotlin
include(":app")
include(":vasp")
```

**Root `build.gradle.kts` plugins** (versions from the tech spec)

```kotlin
plugins {
    id("com.android.application") version "8.8.1" apply false
    id("com.android.library") version "8.8.1" apply false
    id("org.jetbrains.kotlin.android") version "2.0.21" apply false
    id("org.jetbrains.kotlin.plugin.compose") version "2.0.21" apply false
    id("org.jetbrains.kotlin.plugin.serialization") version "2.0.21" apply false
    id("com.google.devtools.ksp") version "2.0.21-1.0.27" apply false
}
```

**`app/build.gradle.kts`**

```kotlin
plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
    id("org.jetbrains.kotlin.plugin.serialization")
    id("com.google.devtools.ksp")
}

dependencies {
    implementation(project(":vasp"))
    // ... existing Compose / Room / Media3 deps
}
```

If the generated app uses a version catalog, add the serialization plugin there instead of hard-coding versions. The module itself does not declare plugin versions.

## Call from `com.example`

```kotlin
import org.aurphyx.vasp.RawAnalysis
import org.aurphyx.vasp.VaspScoringEngine
import org.aurphyx.vasp.player.PlayerVaspProfile

val engine = VaspScoringEngine()
val full = engine.generate(
    RawAnalysis(
        title = track.title,
        artist = track.artist,
        bpm = 135.0,
        attackMs = 45.0,
        centroidHz = 2200.0,
        keyMode = "Minor",
        sentimentScore = -0.2,
        rmsAmplitude = 0.75,
        dominantFreqHz = 3000.0,
    )
)
val ui: PlayerVaspProfile = engine.generatePlayerProfile(/* same RawAnalysis */)
// full.encodeToString() → nested 3.69 JSON for inspector / ID3 TXXX
```

`PlayerVaspProfile` field names match §4.2 (`dominance`, `bpmPerceived`, `keySignature`, `spatialWidthRatio`, `texturalDensity`, `lyricProminence`, `primaryHex`, `secondaryHex`, `macroSetting`, `weather`). Point `TrackItem.vaspProfile` at this type, or copy fields into the generated `com.example.data.VaspProfile`.

Protocol JSON stays nested. The player profile is a readout. Missing catalog facts stay `null` / `[]` / `"unknown"`.

## What this module is not

- Not the Claude Android player (`vibeaudioplayer/`)
- Not the Rust desktop player (`vibemediaplayer/`)
- Not a DSP decoder — feed it measured `RawAnalysis` (BPM, attack, centroid, key mode, RMS, dominant frequency)
