#!/usr/bin/env bash
# Build Ghost Link firmware for ESP32-S3 → outputs ghostlink.bin (or equivalent)
# Run from repo root: ./src/network/meshwerk/firmware/build_ghostlink.sh
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
cd "$REPO_ROOT"
echo "Building Ghost Link firmware (ESP32-S3)..."
# If you have an esp32 meshwerk crate:
# cargo build --release -p aurafs-meshwerk-esp32 --target xtensa-esp32s3-none-elf
# cp target/xtensa-esp32s3-none-elf/release/aurafs_meshwerk_esp32.bin "$SCRIPT_DIR/ghostlink.bin"
# Otherwise: placeholder – implement your ESP32 build and copy output to ghostlink.bin
echo "Placeholder: Add your ESP32-S3 build command and copy artifact to $SCRIPT_DIR/ghostlink.bin"
echo "Then flash: cargo espflash --target esp32s3 $SCRIPT_DIR/ghostlink.bin"
