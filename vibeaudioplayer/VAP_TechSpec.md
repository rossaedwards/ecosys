# Vibe Audio Player (VAP) — Technical Specification & Architecture Document

**Document Version:** 3.2.0  
**Target Platform:** Android 8.0+ (API Level 26–35)  
**UI Framework:** Jetpack Compose (Material Design 3)  
**Language:** Kotlin 2.0+ (Coroutines, StateFlow, Room KSP)  
**Package:** `com.example` (Application ID: `com.aistudio.mewzikplayer.rxdjlk`)  

---

## 1. Executive Summary

**Vibe Audio Player (VAP)** is an audiophile-grade, offline-first Android music player and spatial sound workstation. It combines high-resolution audio decoding (FLAC, DSD, ALAC, WAV, MP3, AAC, OGG) with a **9-Pillar VASP (Vibe Audio Semantic Profiling)** psychoacoustic metadata engine, an interactive **10-Band Graphic Equalizer**, real-time **Dolby Atmos / Binaural 3D Spatial Audio**, **Time-Synchronized Lyrics (.LRC)**, **Bit-Perfect Audio Routing (USB DAC, Bluetooth LE, LDAC, Google Cast)**, and foreground service playback with lock-screen transport controls.

---

## 2. Core Feature Set

### 2.1. Audio Engine & Playback
- **Multi-Format High-Res Playback:** Native decoding and bit-accurate playback for 24-bit/192kHz and 32-bit/384kHz FLAC, ALAC, WAV, DSD, MP3, AAC, and OGG Vorbis.
- **Gapless Playback:** Seamless track transitions without buffer gaps or latency hiccups.
- **Configurable Crossfade:** Smooth volume crossfading between consecutive tracks (0s to 12s, adjustable in 1s increments).
- **Pitch-Corrected Variable Speed:** Continuous playback speed scaling from `0.5x` to `2.5x` in precision `0.1x` steps without formant distortion.
- **Sleep Timer:** Automatic playback suspension timer (`Off`, `15m`, `30m`, `45m`, `60m`).
- **Device Media Scanner:** Automatic local storage indexing of device audio directories (`/Music`, `/Download`, `/Podcasts`) via Android MediaStore and custom URI file pickers.

### 2.2. VASP (Vibe Audio Semantic Profiling) Engine
The proprietary 9-Pillar VASP classification model analyzes and maps audio metadata into multidimensional psychoacoustic coordinates:
1. **Affective Dominance:** Valence, arousal, emotional posture (Assertive, Driving, Commanding, Uplifting, Reflective, Chill).
2. **Kinetic Energy:** Perceived and musical BPM, transient punch, and rhythmic drive.
3. **Tonal Signature:** Root key signature, harmonic density, scale mode (Major/Minor/Dorian/Phrygian).
4. **Spatial Geometry:** Stereo soundstage width, depth layering, binaural elevation.
5. **Textural Density:** Transients, harmonic saturation, analog warmth vs. digital precision.
6. **Lyric/Narrative Mode:** Instrumental vs. vocal prominence, synchronized lyric timestamp mapping.
7. **Cultural / Subgenre Alignment:** Cyberpunk, Synthwave, Nu-Metal, Phonk, Lo-Fi, Liquid DnB, Classical.
8. **Macro Setting & Weather:** Contextual scene pairing (Midnight Drive, PR Gym, Neon Rain, Golden Hour, Deep Focus).
9. **Visual Sync (Atmosphere Glow):** Dynamic UI background mesh and canvas particles that extract primary/secondary hex gradients from the track's VASP signature.

### 2.3. 10-Band Audiophile Equalizer & Soundstage
- **10 Precision Frequency Bands:** `31 Hz`, `62 Hz`, `125 Hz`, `250 Hz`, `500 Hz`, `1 kHz`, `2 kHz`, `4 kHz`, `8 kHz`, `16 kHz` with `±12.0 dB` continuous gain adjustment.
- **Real-Time Spline Response Graph:** Live Cubic-Bézier response curve visualizer displaying compound frequency response.
- **Sub-Harmonic Bass Boost:** Psychoacoustic low-end harmonic exciter (`0%` to `100%`).
- **Spatial Stereo Virtualizer:** Phase-manipulated stereo field wideness engine (`0%` to `100%`).
- **Preset Engine:**
  - *Factory Presets:* Flat Reference, Bass Surge (EDM/Phonk), Vocal Clarity (Pop/Acoustic), Electronic Cyber (Synthwave), Rock & Metal Punch, Spatial Stage (Classical/Ambient).
  - *Custom Presets:* Create, name, save, and delete user presets stored in Room DB.
  - *Backup & Migration:* Export presets to JSON and import external JSON configurations.

### 2.4. Dolby Atmos & Binaural 3D Spatial Audio
- **Binaural 3D Headphone Simulation:** Head-Related Transfer Function (HRTF) spatial emulation for stereo headphones.
- **Soundstage Width Scaling:** Extends sound field up to `200%` width.
- **Elevation Control:** Vertical acoustic positioning (`0%` to `100%`).
- **Spatial Presets:**
  - `Binaural 3D Atmos` (Full immersive dome)
  - `Cyber Dynamic 360` (High transient reactivity)
  - `Club Spatial Rig` (Sub-bass proximity & room reflection)
  - `Pure Direct Hi-Res` (Bit-perfect bypass)

### 2.5. Time-Synchronized Karaoke Lyrics Engine
- **LRC Parser:** Millisecond-accurate timestamp synchronization (`[mm:ss.xx]`).
- **Interactive Karaoke View:** Real-time active lyric line highlight with smooth auto-scrolling.
- **Tap-to-Seek:** Tapping any lyric line instantly seeks audio playback to that precise timestamp.

### 2.6. Audio Output Routing & Cast
- **Endpoint Manager:** Real-time enumeration and switching between active audio output destinations:
  - *Bluetooth LE Audio (LC3 Codec)*
  - *Sony LDAC Lossless (990 kbps)*
  - *USB-C Bit-Perfect DAC (32-bit / 384 kHz)*
  - *Google Cast Hi-Fi (Wi-Fi 24-bit)*
  - *Phone Stereo Speakers (Dolby Atmos Tuned)*
  - *Wired 3.5mm Headphone Jack*

### 2.7. Up Next Queue & Playlists (Room Database)
- **Persistent Playlists:** Local SQLite database managed via Room (`PlaylistEntity`) with custom names, descriptions, color themes, and track sequence storage.
- **"Up Next" Playback Queue:** Live queue modal allowing single-tap track selection, upward/downward reordering, single-item removal, and "Save Queue as Playlist".
- **Track Context Menu:** Instant options to "Play Next", "Add to Queue", and add to any existing playlist.

### 2.8. Android System Integration & Background Service
- **Foreground Media Service:** `VibePlaybackService` with `foregroundServiceType="mediaPlayback"`.
- **System Notification & Lock Screen:** Rich notification displaying track title, artist, album, format badge (e.g. `FLAC 24-bit 96kHz`), previous/next buttons, play/pause toggle, and dismissal stop action.
- **Edge-to-Edge Design:** Fully transparent system status bars and navigation bars with inset paddings.
- **120 FPS High-Refresh Rate Mode:** Render optimizations for ultra-smooth 120Hz display panels.

---

## 3. Architecture & Tech Stack
com.example
├── audio
│ ├── AudioOutputManager.kt // Bluetooth LE, LDAC, USB DAC, Cast endpoints
│ ├── EqualizerManager.kt // 10-band DSP gains, presets, JSON export
│ └── VibeAudioEngine.kt // Core player, speed control, spectrum analyzer
├── data
│ ├── AudioModels.kt // TrackItem, EqPreset, DolbyAtmosMode, GoogleUser
│ ├── PlaylistEntity.kt // Room entity for user playlists
│ ├── CustomEqPresetEntity.kt // Room entity for custom user EQ presets
│ ├── VibeDatabase.kt // Room Database with DAOs and migrations
│ └── VibeRepository.kt // Local storage media scan & reactive Flow repository
├── lyrics
│ ├── LrcParser.kt // [mm:ss.xx] timestamp parser
│ └── LyricsManager.kt // Synced lyric repository and track matching
├── service
│ └── VibePlaybackService.kt // Media playback foreground service & notification
├── ui
│ ├── components
│ │ ├── AudioOutputSheet.kt // Output endpoint selector bottom sheet
│ │ ├── CosmicBackground.kt // Dynamic RMS/Kick reactive canvas background
│ │ ├── CreatePlaylistDialog.kt // New playlist modal with color swatches
│ │ ├── DolbyAtmosStage.kt // 3D soundstage orbital controls
│ │ ├── EqualizerSheet.kt // 10-band slider sheet & Bezier curve
│ │ ├── LyricsView.kt // Auto-scrolling time-synced lyrics list
│ │ ├── MiniPlayerBar.kt // Persistent bottom docked mini-player
│ │ ├── NinePillarMandala.kt // Interactive 9-Pillar radar canvas
│ │ ├── QueueSheet.kt // Up Next queue manager modal
│ │ ├── SavePresetDialog.kt // Custom EQ preset creation dialog
│ │ └── SpeedControls.kt // 0.5x–2.5x speed control cluster
│ ├── screens
│ │ ├── LoadingScreen.kt // App boot & storage scan splash
│ │ ├── MewzikScreen.kt // Music library, tracks, tabs & playlists
│ │ ├── NowPlayingScreen.kt // Turntable visualizer, VASP pills, seekbar
│ │ ├── SettingsScreen.kt // Gapless, crossfade, sleep timer, cloud sync
│ │ ├── VaspInspectorScreen.kt // 9-Pillar deep metadata editor & radar
│ │ └── VisualzScreen.kt // Full-screen reactive audio visualizer
│ ├── theme
│ │ ├── Color.kt
│ │ ├── Theme.kt // Material 3 Dark Palette
│ │ └── Type.kt
│ └── VibeApp.kt // Main scaffold, navigation & sheet coordinator
└── viewmodel
└── VibeViewModel.kt // AndroidViewModel managing reactive UI state
code
Code
---

## 4. Key Data Models

### 4.1. `TrackItem`
```kotlin
data class TrackItem(
    val id: String,
    val title: String,
    val artist: String,
    val album: String,
    val durationMs: Long,
    val audioUri: String,
    val format: AudioFormatType,       // FLAC, MP3, WAV, AAC, ALAC, DSD
    val sampleRateKhz: Int,            // e.g. 96, 192, 44
    val bitDepth: Int,                 // e.g. 24, 32, 16
    val bitrateKbps: Int,              // e.g. 320, 1411, 4608
    val coverGradientStart: String,
    val coverGradientEnd: String,
    val isFavorite: Boolean,
    val vaspProfile: VaspProfile
)
4.2. VaspProfile (9-Pillars)
code
Kotlin
data class VaspProfile(
    val dominance: String,             // High Assertive, Driving, Commanding, Chill
    val bpmPerceived: Int,             // e.g. 128
    val keySignature: String,          // e.g. "F# Minor", "C Major"
    val spatialWidthRatio: Float,      // 0.0f .. 2.0f
    val texturalDensity: String,       // Dense Analog, Crisp Digital, Warm Tape
    val lyricProminence: String,       // Lead Vocal, Instrumental, Spoken
    val primaryHex: String,            // Dynamic theme accent 1
    val secondaryHex: String,          // Dynamic theme accent 2
    val macroSetting: String,          // Midnight Cyber Drive, High PR Gym
    val weather: String                // Neon Rain, Electric Storm, Clear Horizon
)
4.3. PlaylistEntity (Room)
code
Kotlin
@Entity(tableName = "playlists")
data class PlaylistEntity(
    @PrimaryKey val id: String,
    val name: String,
    val description: String,
    val trackIdsJson: String,          // JSON array of track UUIDs
    val colorHex: String,
    val createdAtMs: Long
)
5. Security, Permissions & Manifest Declarations
Permission	Purpose
android.permission.FOREGROUND_SERVICE	Uninterrupted background audio engine execution
android.permission.FOREGROUND_SERVICE_MEDIA_PLAYBACK	Android 14+ compliant media playback service
android.permission.POST_NOTIFICATIONS	Android 13+ lock screen & notification transport controls
android.permission.READ_MEDIA_AUDIO	Android 13+ local audio storage scanning
android.permission.READ_EXTERNAL_STORAGE	Android 12 and below local storage scanning
android.permission.INTERNET	Streaming service authentication & Google Sync
android.permission.BLUETOOTH / BLUETOOTH_CONNECT	Bluetooth LE and LDAC output discovery
6. Build & Compilation Verification
Gradle Version: Kotlin DSL (build.gradle.kts)
Android Gradle Plugin (AGP): 8.8.1
Kotlin: 2.0.21 with KSP for Room Database code generation
Build Status: Clean Compilation (compile_applet PASS)