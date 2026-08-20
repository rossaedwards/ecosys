#!/bin/sh
set -eu
ROOT=/workspace
ART="$ROOT/artifacts/imagine_images"
OUT="$ROOT/public/canva-kit"
mkdir -p "$OUT/logos" "$OUT/scenes" "$OUT/illustrations" "$OUT/sprites" "$OUT/icons"

ff() {
  ffmpeg -y -hide_banner -loglevel error "$@"
}

# App icon masters
ff -i "$ART/0eaf9e00-dc6f-47b1-b046-b8c27b96a50f.jpg" \
  -vf "scale=1024:1024:force_original_aspect_ratio=increase,crop=1024:1024" -q:v 3 \
  "$OUT/logos/app-icon-1024.jpg"
ff -i "$OUT/logos/app-icon-1024.jpg" -vf "scale=512:512" -q:v 3 "$OUT/logos/app-icon-512.jpg"
ff -i "$OUT/logos/app-icon-1024.jpg" -vf "scale=192:192" -q:v 3 "$OUT/logos/app-icon-192.jpg"
ff -i "$OUT/logos/app-icon-1024.jpg" -vf "scale=432:432" -q:v 3 "$OUT/logos/adaptive-foreground-432.jpg"

# Splash / poster (titleless-ish + titled)
ff -i "$ART/518d6454-e9aa-4ca4-9bfd-ac85609ffd7b.jpg" \
  -vf "scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920" -q:v 3 \
  "$OUT/illustrations/splash-1080x1920.jpg"
ff -i "$ART/518d6454-e9aa-4ca4-9bfd-ac85609ffd7b.jpg" \
  -vf "scale=1080:2400:force_original_aspect_ratio=increase,crop=1080:2400" -q:v 3 \
  "$OUT/illustrations/splash-1080x2400.jpg"
ff -i "$ART/109a15a9-9c08-40cb-ac4f-c57e065b46e8.jpg" \
  -vf "scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920" -q:v 3 \
  "$OUT/illustrations/poster-1080x1920.jpg"

# Onboarding
ff -i "$ART/ef98ba65-a056-43ad-a647-d86d7bbd4ed6.jpg" \
  -vf "scale=1080:1440:force_original_aspect_ratio=increase,crop=1080:1440" -q:v 3 \
  "$OUT/illustrations/onboarding-1080x1440.jpg"
ff -i "$ART/ef98ba65-a056-43ad-a647-d86d7bbd4ed6.jpg" \
  -vf "scale=1080:1350:force_original_aspect_ratio=increase,crop=1080:1350" -q:v 3 \
  "$OUT/illustrations/onboarding-1080x1350.jpg"

# Scenes
ff -i "$ART/6c19de5f-0fb5-4570-8f27-8fe800652bbe.jpg" \
  -vf "scale=720:405:force_original_aspect_ratio=increase,crop=720:405" -q:v 3 \
  "$OUT/scenes/night-drive-720x405.jpg"
ff -i "$ART/6c19de5f-0fb5-4570-8f27-8fe800652bbe.jpg" \
  -vf "scale=360:202:force_original_aspect_ratio=increase,crop=360:202" -q:v 3 \
  "$OUT/scenes/night-drive-360x202.jpg"
ff -i "$ART/6af77a9e-5bc4-4780-852b-7f297f4d606b.jpg" \
  -vf "scale=720:405:force_original_aspect_ratio=increase,crop=720:405" -q:v 3 \
  "$OUT/scenes/focus-720x405.jpg"
ff -i "$ART/6af77a9e-5bc4-4780-852b-7f297f4d606b.jpg" \
  -vf "scale=360:202:force_original_aspect_ratio=increase,crop=360:202" -q:v 3 \
  "$OUT/scenes/focus-360x202.jpg"
ff -i "$ART/f0610c05-d0f4-4592-ac69-b98cd0c7c762.jpg" \
  -vf "scale=720:405:force_original_aspect_ratio=increase,crop=720:405" -q:v 3 \
  "$OUT/scenes/gym-peak-720x405.jpg"
ff -i "$ART/f0610c05-d0f4-4592-ac69-b98cd0c7c762.jpg" \
  -vf "scale=360:202:force_original_aspect_ratio=increase,crop=360:202" -q:v 3 \
  "$OUT/scenes/gym-peak-360x202.jpg"
ff -i "$ART/9996137d-02a6-4a9f-84e2-68b369608894.jpg" \
  -vf "scale=720:405:force_original_aspect_ratio=increase,crop=720:405" -q:v 3 \
  "$OUT/scenes/deep-space-720x405.jpg"
ff -i "$ART/9996137d-02a6-4a9f-84e2-68b369608894.jpg" \
  -vf "scale=360:202:force_original_aspect_ratio=increase,crop=360:202" -q:v 3 \
  "$OUT/scenes/deep-space-360x202.jpg"

# Illustrations
ff -i "$ART/f30550d7-6d4d-41c0-866e-e4d07a83fd05.jpg" \
  -vf "scale=1440:1440:force_original_aspect_ratio=increase,crop=1440:1440" -q:v 3 \
  "$OUT/illustrations/empty-library-1440.jpg"
ff -i "$ART/f30550d7-6d4d-41c0-866e-e4d07a83fd05.jpg" \
  -vf "scale=720:720:force_original_aspect_ratio=increase,crop=720:720" -q:v 3 \
  "$OUT/illustrations/empty-library-720.jpg"
ff -i "$ART/88acbaf5-a721-4701-a389-5b63281ba5b3.jpg" \
  -vf "scale=1024:1024:force_original_aspect_ratio=increase,crop=1024:1024" -q:v 3 \
  "$OUT/illustrations/album-placeholder-1024.jpg"
ff -i "$ART/88acbaf5-a721-4701-a389-5b63281ba5b3.jpg" \
  -vf "scale=192:192:force_original_aspect_ratio=increase,crop=192:192" -q:v 3 \
  "$OUT/illustrations/album-placeholder-192.jpg"
ff -i "$ART/88acbaf5-a721-4701-a389-5b63281ba5b3.jpg" \
  -vf "scale=96:96:force_original_aspect_ratio=increase,crop=96:96" -q:v 3 \
  "$OUT/illustrations/album-placeholder-96.jpg"
ff -i "$ART/dc3fa131-83b5-4eac-8808-3db4c409f848.jpg" \
  -vf "scale=1024:1024:force_original_aspect_ratio=increase,crop=1024:1024" -q:v 3 \
  "$OUT/illustrations/nine-pillar-diagram-1024.jpg"

# Glow sprites
ff -i "$ART/8012977f-8aab-4138-8586-abbcd89863f6.jpg" \
  -vf "scale=1024:1024:force_original_aspect_ratio=increase,crop=1024:1024" -q:v 3 \
  "$OUT/sprites/glow-1024.jpg"
ff -i "$ART/8012977f-8aab-4138-8586-abbcd89863f6.jpg" \
  -vf "scale=512:512:force_original_aspect_ratio=increase,crop=512:512" -q:v 3 \
  "$OUT/sprites/glow-512.jpg"
ff -i "$ART/8012977f-8aab-4138-8586-abbcd89863f6.jpg" \
  -vf "scale=256:256:force_original_aspect_ratio=increase,crop=256:256" -q:v 3 \
  "$OUT/sprites/glow-256.jpg"

echo "exported raster kit to $OUT"
