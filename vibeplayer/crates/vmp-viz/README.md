# vmp-viz — VAP cymatic runtime (FUTE-transmuted)

Transmuted from **`main/vibe-audio-visualizer/`** (C) using **v01d / FUTE**.

## Pipeline

```bash
# Structural C → Rust scaffold (raw output under transmute_raw/)
cargo run -p fute --bin v01d -- lang \
  ../vibe-audio-visualizer/src/dsp_engine.c \
  -o crates/vmp-viz/transmute_raw/dsp_engine.rs --from c

# Production symbiont (this crate) is the polished result:
#   src/runtime.rs     ← vap_runtime.c + dsp_engine.c
#   src/photometric.rs ← vap_photometric.h + vap_affective.h
```

## API

```rust
use vmp_viz::VapRuntime;

let mut rt = VapRuntime::init();
rt.load_vap(&vap_object);
rt.update_dsp(&fft_magnitude, 48_000, 1.0 / 60.0);
let uniforms = rt.shader_uniforms(); // for WebGL / wgpu
```

## Provenance

| Field | Value |
|-------|--------|
| Origin | `vibe-audio-visualizer` |
| Engine | v01d (FUTE) |
| Protocol | V.A.P. v3.1 · TSLCA 9-cell |
