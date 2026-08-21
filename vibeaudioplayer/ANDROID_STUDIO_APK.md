# Vibe Audio Player — Android Studio APK (Windows + S24)

The Studio project is already at **`C:\aurphyx\vibeaudioplayer`**. Package `org.aurphyx.vibeaudioplayer`. Do not change `applicationId`. Theme `VibeAudioPlayerTheme`. First launch is the **Orb** tab, not Hello Android.

If this tree was not updated, copy from `android-studio-dropin/` using the table in that folder’s `PATHS.md`.

Art source: `C:\rossaedwards\ecosys\vibeaudioplayer\assets\`.

---

## A. Open, sync, pick a device

1. Android Studio → **File → Open** → `C:\aurphyx\vibeaudioplayer`.
2. Let Gradle sync (status bar). JDK **17**. SDK Platform **35+** is fine; `minSdk` is **26**, `targetSdk` **37**.
3. Top bar currently says **No Devices**:
   - **S24 USB:** phone → Settings → Developer options → USB debugging. Cable in. Allow the RSA prompt. Device dropdown should show the S24.
   - **Emulator:** Device Manager → Create / start an **API 34** Pixel. Wait until home screen is up.

Common “No Devices”: bad cable, USB file-transfer mode (use File transfer / Android Auto), driver, or debugging off.

---

## B. Run (Shift+F10)

1. Run configuration: **app**.
2. **Shift+F10** (or the green Run).
3. First frame must be the **Orb** (void + cymatic field + now-playing), then the splash fades. Not “Hello Android”.
4. Grant **local audio** if asked, or **Play demos only**.

---

## C. Debug APK

**Build → Build Bundle(s) / APK(s) → Build APK(s).**

Output:

`C:\aurphyx\vibeaudioplayer\app\build\outputs\apk\debug\app-debug.apk`

Install on the S24: drag onto the device, or `adb install -r` that path.

---

## D. Optional signed release

**Build → Generate Signed App Bundle or APK.** Use *your* existing keystore. **Do not invent a password.** Do not upload to Play.

---

## E. S24 smoke

- Orb moves on **Night Drive Protocol** (play).
- Skip to **Orbital Lattice** / **Forward Current** — palette shifts (indigo/ice vs teal/violet).
- **Library → Open local audio** — play an MP3. Unsupported formats show a chip, no crash.
- Lock screen **next / prev**.
- Screen off: audio continues (Media3 foreground service).
- **VASP** shows nine pillar cards; photometric hex drives orb color.
- **Scene:** readable type + reduced motion actually change the UI / orb.
- **About:** tap the About page **13 times** — Aurphyx business-card QR appears.

---

## F. Common failures

| Symptom | Fix |
|---|---|
| No Devices | USB debugging, cable, File transfer mode, API 34 emulator |
| Gradle / JDK | File → Settings → Build → Gradle JDK **17**. Sync. |
| `minSdk` complaints | Keep **26**. Do not bump back to 35. |
| FLAC / OGG fail | Chip “Format not supported”. S24 usually decodes both; if not, use MP3/M4A. |
| Permission | About the dialog: demos work without READ_MEDIA_AUDIO. Local files need it (API 33+) or legacy storage (API 26–32). |
| Splash stuck | Cold start ~1.6s then Orb. If frozen, logcat `PlaybackService`. |
| Lock screen missing | POST_NOTIFICATIONS on API 33+; allow the media notification. |

---

## Gradle from a terminal (optional)

```bat
cd C:\aurphyx\vibeaudioplayer
gradlew.bat assembleDebug
```

APK: `app\build\outputs\apk\debug\app-debug.apk`
