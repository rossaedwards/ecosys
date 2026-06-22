rossaedwards/main/vibeaudio/
│
├── README.md                          # VAV project overview (see suggested text below)
│
├── vlc-plugin/                        # The VLC visualization plugin (C)
│   ├── CMakeLists.txt                 # Build system — finds libvlc, OpenGL, FFTW3
│   ├── Makefile                       # Convenience wrapper for VLC plugin conventions
│   ├── vibe_visualizer.c              # ← YOUR FILE (the VLC entry point, already done)
│   ├── vap_runtime.h                  # 9-pillar runtime struct definition
│   ├── vap_runtime.c                  # vap_runtime_init(), field accessors
│   ├── vap_loader.h                   # Loader interface
│   ├── vap_loader.c                   # Sidecar → ID3 → Vorbis → defaults chain
│   ├── dsp_engine.h                   # DSP context interface
│   ├── dsp_engine.c                   # FFTW3 FFT, onset detector, BPM tracker
│   ├── gl_renderer.h                  # GL pipeline interface
│   ├── gl_renderer.c                  # Cymatic geometry shader pipeline
│   └── shaders/
│       ├── cymatic.vert               # Vertex: VAP Pillar 1 BPM → standing wave freq
│       ├── cymatic.frag               # Fragment: Pillar 7 photometric → color field
│       └── bloom.frag                 # Post-process: Pillar 5 arousal → bloom radius
│
├── vap-schema/                        # The protocol definition (version-locked)
│   ├── vap_schema_v3.1.json           # JSON Schema Draft-07 (what vap_loader validates against)
│   ├── CHANGELOG.md                   # Protocol version history
│   └── examples/
│       ├── trap_example.vap.json
│       └── ambient_example.vap.json
│
├── vap-generator/                     # Python tool: .flac/.wav → .vap.json sidecar
│   ├── pyproject.toml
│   ├── src/
│   │   ├── phase1_dsp/
│   │   │   ├── structural.py          # BPM, syncopation, kick
│   │   │   ├── tonal.py               # Key, dissonance, chord complexity
│   │   │   └── timbral.py             # Spectral centroid, THD, LRA
│   │   ├── phase2_ml/
│   │   │   ├── linguistic.py          # NLP: lyrics, dialect, explicit tier
│   │   │   ├── affective.py           # Thayer model: valence/arousal/dominance
│   │   │   └── contextual.py          # Bayesian scenario engine
│   │   ├── phase3_derived/
│   │   │   ├── photometric.py         # Hz → wavelength → hex mapping
│   │   │   ├── kinetic.py             # MET, HRV, entrainment factor
│   │   │   └── genealogical.py        # GVS/VVS tribe alignment
│   │   └── generate_vap.py            # CLI: `python generate_vap.py track.flac`
│   └── tests/
│       └── test_generator.py
│
├── vst-plugin/                        # VST3/CLAP wrapper (future — stub now)
│   └── STUB.md                        # "VST3 host wrapper for vap-generator — planned"
│
├── docs/
│   ├── VAP_TECHNICAL_MANUAL.md        # Full 9-pillar spec (your existing documentation)
│   ├── PILLAR_REFERENCE.md            # Quick lookup: pillar → field → type → range
│   ├── TSLCA_BINDING.md               # Formal pillar ↔ TSLCA dimension mapping
│   ├── VLC_BUILD_GUIDE.md             # How to compile & install the VLC plugin
│   └── GENERATOR_USAGE.md             # How to run the Python sidecar generator
│
└── .github/
    └── workflows/
        ├── schema_validate.yml        # Validates examples/*.vap.json against schema
        ├── plugin_build.yml           # CMake build test on ubuntu-latest
        └── generator_test.yml         # Python pytest for vap-generator
