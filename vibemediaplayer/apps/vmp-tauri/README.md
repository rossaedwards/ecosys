# Vibe Media Player — Tauri shell

Native desktop host for VMP:

- **Decode/play**: Symphonia + cpal (`vmp-audio`)
- **VAP tags**: lofty sidecar + embed
- **UI**: shared Vite/React frontend
- **v01d**: symbiotic mode bindings

## System dependencies

### Linux
```bash
# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel alsa-lib-devel

# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev libasound2-dev
```

### Then
```bash
# from repo root (use cargo-tauri, NOT the npm `tauri` binary)
cargo install tauri-cli --version "^2" --locked   # provides: cargo tauri
npm install

# desktop — npm scripts call `cargo tauri` (fixed for cargo-installed CLI)
npm run tauri:dev
# equivalent:
#   cd apps/vmp-tauri/src-tauri && cargo tauri dev
```

ALSA is installed on this machine — CLI hardware play works:

```bash
cargo run -p vmp-cli -- play track.flac
cargo run -p vmp-cli -- devices
```

Still needed for **Tauri desktop UI**: WebKitGTK (+ usual GTK stack).
