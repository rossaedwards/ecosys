/*****************************************************************************
 * gl_renderer.h — VAV OpenGL Renderer Interface
 * Aurphyx SUXS / rAE — Vibe Audio Visualizer
 *
 * Owns the full OpenGL lifecycle:
 *   - EGL/WGL context acquisition via VLC's vlc_gl_t
 *   - Shader compilation and linking (vert + frag + post-process bloom)
 *   - Framebuffer Object (FBO) for off-screen cymatic render → bloom pass
 *   - Uniform upload from vap_runtime_t every frame (all 3 VAP Phases)
 *   - Chromatic band energy array from live DSP → u_chrom_energy[4]
 *   - Strobe gating via Pillar 7.2 STROBE_TRIGGER threshold
 *   - Fade mode logic (Sharp vs. Linear per Pillar 7.2 FADE_MODE)
 *****************************************************************************/

#ifndef GL_RENDERER_H
#define GL_RENDERER_H

#include "vap_runtime.h"

/* Opaque renderer context — callers never touch internals */
typedef struct gl_ctx_t gl_ctx_t;

/**
 * gl_renderer_create()
 * Acquire VLC's OpenGL surface, compile all shaders, set up FBO.
 * @param obj     VLC object (for vlc_gl_t acquisition + msg_* logging)
 * @param vap     Initial VAP state (used to seed static uniforms)
 * @return        Heap-allocated gl_ctx_t, or NULL on hard failure
 */
gl_ctx_t *gl_renderer_create(vlc_object_t *obj, const vap_runtime_t *vap);

/**
 * gl_renderer_update()
 * Called every audio frame from DoWork().
 * Uploads all VAP Phase I/II/III uniforms and issues a draw call.
 * @param ctx     Renderer context
 * @param vap     Current VAP runtime state (live DSP + static loaded data)
 * @param chrom   4-element array: per-band chromatic energy [0.0–1.0]
 *                Band order matches VAP Pillar 7.1 chromatic map:
 *                [0]=Sub-Bass 40-60Hz  [1]=Low-Mid 60-250Hz
 *                [2]=Mids 250-2kHz     [3]=Highs 2kHz+
 */
void gl_renderer_update(gl_ctx_t *ctx, const vap_runtime_t *vap,
                        const float chrom[4]);

/**
 * gl_renderer_resize()
 * Called when VLC window is resized. Updates u_resolution uniform
 * and recreates FBO color attachment at new dimensions.
 */
void gl_renderer_resize(gl_ctx_t *ctx, int width, int height);

/**
 * gl_renderer_destroy()
 * Clean teardown: delete shaders, FBO, textures, release GL context.
 */
void gl_renderer_destroy(gl_ctx_t *ctx);

#endif /* GL_RENDERER_H */
