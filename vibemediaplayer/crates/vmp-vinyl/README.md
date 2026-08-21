# vmp-vinyl — Mixxx symbiont (FUTE / libclang)

**Plan:** Mixxx C++ **engine** is transmuted with **v01d + libclang** into this crate, then hosted as Vinyl Vibez inside VMP.

## Status

| Piece | Status |
|-------|--------|
| libclang on host | ✅ |
| FUTE `clang-ast` | ✅ scaffolds in `transmute_raw/` |
| **Polished dual-deck runtime** | ✅ `EngineBuffer` + `EngineMixer` + sync |
| load / play / seek / rate / cue | ✅ |
| equal-power crossfade | ✅ |
| BPM sync (rate match) | ✅ (beat-grid phase later) |
| VMP UI wiring | 🔲 |

```bash
cargo test -p vmp-vinyl
cargo run -p vmp-cli -- vinyl /path/a.flac --deck-b /path/b.flac --xfade 0.5 --seconds 3
```

## Batch transmute

```bash
# default: ../mixxx next to vibeaudio
./scripts/fute_transmute_mixxx_engine.sh

# or:
MIXXX_SRC=/path/to/mixxx ./scripts/fute_transmute_mixxx_engine.sh
```

## Manual one-shot

```bash
cargo run -p fute --bin v01d --features clang-ast -- lang \
  ../mixxx/src/engine/enginebuffer.h \
  -o crates/vmp-vinyl/transmute_raw/enginebuffer.rs --from cpp
```

## License

**GPL-2.0-or-later** for Mixxx-derived code. Do not relicense as MIT.

## Next polish targets (from Mixxx names)

1. `EngineBuffer` → deck playhead / scale / cue  
2. `EngineMixer` → crossfader / channel gains  
3. `BpmControl` / `SyncControl` → beat sync  
4. Wire into VMP **VINYL VIBEZ** UI  
