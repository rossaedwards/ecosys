# Vibe Audio Visualizer
V.A.P. v3.1 — 9-Pillar TSLCA Cymatic Renderer

This folder is the **VLC visualization plugin** (`vibe_visualizer_plugin`): PCM → `dsp_engine` → VAP phases → `gl_renderer` uniforms → `shaders/vibe.frag` + bloom.

The Vibe Audio Player **Orb** tab (parent `src/components/visualizer-canvas.tsx`) speaks the same renderer language — same nine pillars, same chromatic bands, same `vibe.frag` / `post_bloom.frag` uniforms — so the home picture is this cymatic field, not a separate widget. Do not flatten this plugin into the web player; keep CMake and `vibe_visualizer.c`.
