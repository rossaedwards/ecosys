# AuraFS Meshwerk Firmware – Ghost Link (TIER 1)

**Target**: ESP32-S3 Ghost Link nodes (LoRa + WiFi HaLow sensor/repeater).  
**Output**: `ghostlink.bin` – built locally; not committed (binary).

## Build & Flash

### Prerequisites

- Rust + `espflash` / `cargo-espflash`
- ESP32 target: `rustup target add riscv32imc-unknown-none-elf` or `xtensa-esp32s3-none-elf` per [esp-rs](https://github.com/esp-rs)
- [esp-idf](https://docs.espressif.com/projects/esp-idf/) or [espup](https://github.com/esp-rs/espup) for your host

### Build (produces `ghostlink.bin`)

From repo root or this directory:

```bash
# With esp-rs toolchain (example)
cargo build --release --target xtensa-esp32s3-none-elf
# Or use your project's Makefile/just task that outputs ghostlink.bin
```

If your crate outputs a differently named binary, copy or rename to `ghostlink.bin`:

```bash
cp target/xtensa-esp32s3-none-elf/release/ghostlink ghostlink.bin
```

### Flash to device

```bash
cargo espflash --target esp32s3 --monitor target/ghostlink.bin
# Or: espflash write_flash 0x0 ghostlink.bin
```

### Config

Use **TIER 1** config: `meshwerk/configs/ghostlink.toml` (e.g. `mode = "ghost-sensor"`, `shard_size = "1MB"`).

---

**Note**: `ghostlink.bin` is not stored in the repo (binary artifact). Generate it with the steps above.
