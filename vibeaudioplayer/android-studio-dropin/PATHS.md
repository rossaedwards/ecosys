# Drop-in → already-open Android Studio tree

Copy each **from** this folder **to** `C:\aurphyx\vibeaudioplayer\` (same relative path). Do not change `applicationId`.

| From (this drop-in) | To (Studio project) |
|---|---|
| `gradle/libs.versions.toml` | `C:\aurphyx\vibeaudioplayer\gradle\libs.versions.toml` |
| `app/build.gradle.kts` | `C:\aurphyx\vibeaudioplayer\app\build.gradle.kts` |
| `app/src/main/AndroidManifest.xml` | `C:\aurphyx\vibeaudioplayer\app\src\main\AndroidManifest.xml` |
| `app/src/main/java/org/aurphyx/vibeaudioplayer/` | same under Studio `app\src\main\java\...` |
| `app/src/main/res/values/strings.xml` | Studio `res\values\strings.xml` |
| `app/src/main/res/values/colors.xml` | Studio `res\values\colors.xml` |
| `app/src/main/res/values/themes.xml` | Studio `res\values\themes.xml` |
| `app/src/main/res/drawable/vap_menu_2_tb.png` | Studio drawable |
| `app/src/main/res/drawable/vap_menu_1.png` | Studio drawable |
| `app/src/main/res/drawable/splash_loading.png` | Studio drawable |
| `app/src/main/res/drawable/ic_launcher_photo.jpg` | Studio drawable |
| `app/src/main/res/drawable/aurphyx_business_card_qr.png` | Studio drawable |

This tree is a mirror of files already written into `C:\aurphyx\vibeaudioplayer` when that path was reachable.
