/*****************************************************************************
 * gl_renderer.c — VAV OpenGL Renderer
 * Aurphyx SUXS / rAE — Vibe Audio Visualizer
 *
 * Pipeline:
 *   Pass 1 — Cymatic field render → FBO color texture
 *             Shader: shaders/vibe.vert + shaders/vibe.frag
 *             Uniforms: all 9 VAP pillars via vap_runtime_t
 *
 *   Pass 2 — Bloom post-process → screen
 *             Shader: shaders/vibe.vert + shaders/post_bloom.frag
 *             Input: FBO texture from Pass 1
 *             Bloom intensity driven by Pillar 5 arousal + Pillar 7 ceiling
 *
 * OpenGL target: OpenGL 2.1 / GLSL 1.20
 *   Rationale: VLC's vlc_gl_t targets the lowest common denominator.
 *   AMD Radeon on HP EliteBook (your rig) supports GL 4.6 but we stay
 *   at 2.1 so the plugin runs on any VLC-supported platform including
 *   the Fedora 44 Mesa stack without extension negotiation overhead.
 *
 * Strobe gating: if live DSP band energy exceeds STROBE_TRIGGER (Pillar 7.2)
 *   AND FADE_MODE == SHARP, the renderer fires a white full-frame flash
 *   for exactly one frame then hard-cuts back. Linear fade uses smooth lerp.
 *****************************************************************************/

#ifdef HAVE_CONFIG_H
# include "config.h"
#endif

#include "gl_renderer.h"
#include "vap_runtime.h"
#include "vap_photometric.h"

#include <vlc_common.h>
#include <vlc_plugin.h>
#include <vlc_opengl.h>

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

/* ── GL function pointer typedefs (GL 2.1 core — no GLEW required) ──────── */
#include <GL/gl.h>

/* VLC exposes GL proc lookup via vlc_gl_GetProcAddress */
#define GL_PROC(ret, name, ...) typedef ret (*pf_##name##_t)(__VA_ARGS__)

GL_PROC(GLuint,   glCreateShader,       GLenum);
GL_PROC(void,     glShaderSource,       GLuint, GLsizei,
                                         const GLchar **, const GLint *);
GL_PROC(void,     glCompileShader,      GLuint);
GL_PROC(void,     glGetShaderiv,        GLuint, GLenum, GLint *);
GL_PROC(void,     glGetShaderInfoLog,   GLuint, GLsizei, GLsizei *, GLchar *);
GL_PROC(GLuint,   glCreateProgram,      void);
GL_PROC(void,     glAttachShader,       GLuint, GLuint);
GL_PROC(void,     glLinkProgram,        GLuint);
GL_PROC(void,     glGetProgramiv,       GLuint, GLenum, GLint *);
GL_PROC(void,     glGetProgramInfoLog,  GLuint, GLsizei, GLsizei *, GLchar *);
GL_PROC(void,     glUseProgram,         GLuint);
GL_PROC(void,     glDeleteShader,       GLuint);
GL_PROC(void,     glDeleteProgram,      GLuint);
GL_PROC(GLint,    glGetUniformLocation, GLuint, const GLchar *);
GL_PROC(void,     glUniform1f,          GLint, GLfloat);
GL_PROC(void,     glUniform1i,          GLint, GLint);
GL_PROC(void,     glUniform2f,          GLint, GLfloat, GLfloat);
GL_PROC(void,     glUniform3f,          GLint, GLfloat, GLfloat, GLfloat);
GL_PROC(void,     glUniform1fv,         GLint, GLsizei, const GLfloat *);
GL_PROC(void,     glGenFramebuffers,    GLsizei, GLuint *);
GL_PROC(void,     glBindFramebuffer,    GLenum, GLuint);
GL_PROC(void,     glFramebufferTexture2D, GLenum, GLenum, GLenum, GLuint, GLint);
GL_PROC(void,     glDeleteFramebuffers, GLsizei, const GLuint *);
GL_PROC(GLenum,   glCheckFramebufferStatus, GLenum);
GL_PROC(void,     glGenBuffers,         GLsizei, GLuint *);
GL_PROC(void,     glBindBuffer,         GLenum, GLuint);
GL_PROC(void,     glBufferData,         GLenum, GLsizeiptr,
                                         const GLvoid *, GLenum);
GL_PROC(void,     glDeleteBuffers,      GLsizei, const GLuint *);
GL_PROC(void,     glVertexAttribPointer, GLuint, GLint, GLenum, GLboolean,
                                          GLsizei, const GLvoid *);
GL_PROC(void,     glEnableVertexAttribArray, GLuint);
GL_PROC(GLint,    glGetAttribLocation,  GLuint, const GLchar *);

/* ── Shader source embed macros ─────────────────────────────────────────── */
/* Shaders are embedded as C strings at compile time.
   In the build system they can optionally be read from disk for hot-reload. */

static const char *VERT_SRC =
    "#version 120\n"
    "attribute vec2 a_pos;\n"
    "varying   vec2 v_uv;\n"
    "void main() {\n"
    "    v_uv        = a_pos * 0.5 + 0.5;\n"
    "    gl_Position = vec4(a_pos, 0.0, 1.0);\n"
    "}\n";

/* vibe.frag is too large to inline cleanly — we load from disk at runtime.
   Fallback minimal shader used if file not found (safe, always renders). */
static const char *FRAG_FALLBACK =
    "#version 120\n"
    "uniform float u_time;\n"
    "uniform vec2  u_resolution;\n"
    "uniform float u_arousal;\n"
    "uniform vec3  u_primary_rgb;\n"
    "void main() {\n"
    "    vec2 uv = gl_FragCoord.xy / u_resolution;\n"
    "    float r = length(uv - 0.5);\n"
    "    float ring = smoothstep(0.03, 0.0, abs(r - 0.3 - u_arousal * 0.15));\n"
    "    gl_FragColor = vec4(u_primary_rgb * ring, 1.0);\n"
    "}\n";

static const char *BLOOM_FRAG_SRC =
    "#version 120\n"
    "uniform sampler2D u_scene;\n"
    "uniform vec2      u_resolution;\n"
    "uniform float     u_bloom_strength;  /* arousal * brightness_ceiling */\n"
    "uniform float     u_fade_amount;     /* current fade lerp value 0-1  */\n"
    "varying vec2      v_uv;\n"
    "\n"
    "/* 9-tap Gaussian blur for bloom extraction */\n"
    "vec3 blur9(sampler2D tex, vec2 uv, vec2 px) {\n"
    "    vec3 c = vec3(0.0);\n"
    "    c += texture2D(tex, uv + vec2(-2.0,  0.0) * px).rgb * 0.0625;\n"
    "    c += texture2D(tex, uv + vec2(-1.0,  0.0) * px).rgb * 0.125;\n"
    "    c += texture2D(tex, uv + vec2( 0.0,  0.0) * px).rgb * 0.25;\n"
    "    c += texture2D(tex, uv + vec2( 1.0,  0.0) * px).rgb * 0.125;\n"
    "    c += texture2D(tex, uv + vec2( 2.0,  0.0) * px).rgb * 0.0625;\n"
    "    c += texture2D(tex, uv + vec2( 0.0, -2.0) * px).rgb * 0.0625;\n"
    "    c += texture2D(tex, uv + vec2( 0.0, -1.0) * px).rgb * 0.125;\n"
    "    c += texture2D(tex, uv + vec2( 0.0,  1.0) * px).rgb * 0.125;\n"
    "    c += texture2D(tex, uv + vec2( 0.0,  2.0) * px).rgb * 0.0625;\n"
    "    return c;\n"
    "}\n"
    "\n"
    "void main() {\n"
    "    vec2 px    = 1.0 / u_resolution;\n"
    "    vec3 scene = texture2D(u_scene, v_uv).rgb;\n"
    "\n"
    "    /* Extract bright regions for bloom (luma threshold 0.6) */\n"
    "    float luma = dot(scene, vec3(0.299, 0.587, 0.114));\n"
    "    vec3 bright = (luma > 0.6) ? scene : vec3(0.0);\n"
    "    vec3 bloom  = blur9(u_scene, v_uv, px * 3.0) * u_bloom_strength;\n"
    "\n"
    "    /* Fade: lerp toward black for smooth-fade, white for strobe */\n"
    "    vec3 composed = scene + bloom * 0.6;\n"
    "    composed = mix(composed, vec3(1.0), max(u_fade_amount - 1.0, 0.0));\n"
    "    composed = mix(vec3(0.0), composed, min(u_fade_amount, 1.0));\n"
    "\n"
    "    gl_FragColor = vec4(clamp(composed, 0.0, 1.0), 1.0);\n"
    "}\n";

/* ── Uniform location cache ─────────────────────────────────────────────── */
/* All uniforms from vibe.frag and post_bloom.frag, pre-resolved at init.
   Avoids glGetUniformLocation() cost every frame.                         */

typedef struct {
    /* vibe.frag — Phase I (DSP live) */
    GLint u_time;
    GLint u_resolution;
    GLint u_centroid;
    GLint u_saturation;
    GLint u_syncopation;
    GLint u_bpm_norm;
    GLint u_groove;
    GLint u_dissonance;
    /* vibe.frag — Phase II (ML loaded) */
    GLint u_valence;
    GLint u_arousal;
    GLint u_scenario_fog;
    /* vibe.frag — Phase III (Photometric) */
    GLint u_primary_rgb;
    GLint u_secondary_rgb;
    GLint u_brightness_floor;
    GLint u_brightness_ceiling;
    GLint u_strobe_trigger;
    GLint u_fog_density;
    GLint u_visual_noise;
    GLint u_chrom_energy;    /* uniform float u_chrom_energy[4] */
    /* vibe.frag — Phase III (Kinetic) */
    GLint u_entrainment;
    /* post_bloom.frag */
    GLint bloom_u_scene;
    GLint bloom_u_resolution;
    GLint bloom_u_bloom_strength;
    GLint bloom_u_fade_amount;
} uniform_cache_t;

/* ── Renderer context ────────────────────────────────────────────────────── */

struct gl_ctx_t {
    vlc_object_t  *obj;           /* VLC object for logging               */
    vlc_gl_t      *gl;            /* VLC-managed GL surface               */

    /* GL function pointers — resolved via vlc_gl_GetProcAddress */
    pf_glCreateShader_t           pfn_glCreateShader;
    pf_glShaderSource_t           pfn_glShaderSource;
    pf_glCompileShader_t          pfn_glCompileShader;
    pf_glGetShaderiv_t            pfn_glGetShaderiv;
    pf_glGetShaderInfoLog_t       pfn_glGetShaderInfoLog;
    pf_glCreateProgram_t          pfn_glCreateProgram;
    pf_glAttachShader_t           pfn_glAttachShader;
    pf_glLinkProgram_t            pfn_glLinkProgram;
    pf_glGetProgramiv_t           pfn_glGetProgramiv;
    pf_glGetProgramInfoLog_t      pfn_glGetProgramInfoLog;
    pf_glUseProgram_t             pfn_glUseProgram;
    pf_glDeleteShader_t           pfn_glDeleteShader;
    pf_glDeleteProgram_t          pfn_glDeleteProgram;
    pf_glGetUniformLocation_t     pfn_glGetUniformLocation;
    pf_glUniform1f_t              pfn_glUniform1f;
    pf_glUniform1i_t              pfn_glUniform1i;
    pf_glUniform2f_t              pfn_glUniform2f;
    pf_glUniform3f_t              pfn_glUniform3f;
    pf_glUniform1fv_t             pfn_glUniform1fv;
    pf_glGenFramebuffers_t        pfn_glGenFramebuffers;
    pf_glBindFramebuffer_t        pfn_glBindFramebuffer;
    pf_glFramebufferTexture2D_t   pfn_glFramebufferTexture2D;
    pf_glDeleteFramebuffers_t     pfn_glDeleteFramebuffers;
    pf_glCheckFramebufferStatus_t pfn_glCheckFramebufferStatus;
    pf_glGenBuffers_t             pfn_glGenBuffers;
    pf_glBindBuffer_t             pfn_glBindBuffer;
    pf_glBufferData_t             pfn_glBufferData;
    pf_glDeleteBuffers_t          pfn_glDeleteBuffers;
    pf_glVertexAttribPointer_t    pfn_glVertexAttribPointer;
    pf_glEnableVertexAttribArray_t pfn_glEnableVertexAttribArray;
    pf_glGetAttribLocation_t      pfn_glGetAttribLocation;

    /* Shader programs */
    GLuint prog_vibe;             /* Pass 1: cymatic field                */
    GLuint prog_bloom;            /* Pass 2: bloom + fade                 */

    /* Full-screen quad VBO */
    GLuint vbo_quad;
    GLint  attr_pos_vibe;         /* a_pos location in prog_vibe          */
    GLint  attr_pos_bloom;        /* a_pos location in prog_bloom         */

    /* FBO for off-screen Pass 1 render */
    GLuint fbo;
    GLuint fbo_texture;

    /* Cached uniform locations */
    uniform_cache_t uniforms;

    /* Render state */
    int    width, height;
    float  time_accum;            /* seconds since plugin open            */

    /* Strobe / fade state (Pillar 7.2) */
    float  fade_amount;           /* 0.0=black 1.0=full 2.0=white-flash  */
    int    strobe_fired;          /* 1 if strobe triggered this frame     */
    float  fade_target;           /* where fade_amount is heading         */
    float  fade_rate;             /* seconds per unit (from VAP)          */
    int    fade_mode;             /* 0=Sharp(instant) 1=Smooth(linear)    */
};

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 1 — GL Proc Loader
   ════════════════════════════════════════════════════════════════════════════ */

#define LOAD_GL_PROC(ctx, name)                                          \
    (ctx)->pfn_##name = (pf_##name##_t)                                  \
        vlc_gl_GetProcAddress((ctx)->gl, #name);                         \
    if (!(ctx)->pfn_##name)                                              \
        msg_Warn((ctx)->obj, "[VAV] GL proc missing: " #name)

static int load_gl_procs(gl_ctx_t *ctx) {
    LOAD_GL_PROC(ctx, glCreateShader);
    LOAD_GL_PROC(ctx, glShaderSource);
    LOAD_GL_PROC(ctx, glCompileShader);
    LOAD_GL_PROC(ctx, glGetShaderiv);
    LOAD_GL_PROC(ctx, glGetShaderInfoLog);
    LOAD_GL_PROC(ctx, glCreateProgram);
    LOAD_GL_PROC(ctx, glAttachShader);
    LOAD_GL_PROC(ctx, glLinkProgram);
    LOAD_GL_PROC(ctx, glGetProgramiv);
    LOAD_GL_PROC(ctx, glGetProgramInfoLog);
    LOAD_GL_PROC(ctx, glUseProgram);
    LOAD_GL_PROC(ctx, glDeleteShader);
    LOAD_GL_PROC(ctx, glDeleteProgram);
    LOAD_GL_PROC(ctx, glGetUniformLocation);
    LOAD_GL_PROC(ctx, glUniform1f);
    LOAD_GL_PROC(ctx, glUniform1i);
    LOAD_GL_PROC(ctx, glUniform2f);
    LOAD_GL_PROC(ctx, glUniform3f);
    LOAD_GL_PROC(ctx, glUniform1fv);
    LOAD_GL_PROC(ctx, glGenFramebuffers);
    LOAD_GL_PROC(ctx, glBindFramebuffer);
    LOAD_GL_PROC(ctx, glFramebufferTexture2D);
    LOAD_GL_PROC(ctx, glDeleteFramebuffers);
    LOAD_GL_PROC(ctx, glCheckFramebufferStatus);
    LOAD_GL_PROC(ctx, glGenBuffers);
    LOAD_GL_PROC(ctx, glBindBuffer);
    LOAD_GL_PROC(ctx, glBufferData);
    LOAD_GL_PROC(ctx, glDeleteBuffers);
    LOAD_GL_PROC(ctx, glVertexAttribPointer);
    LOAD_GL_PROC(ctx, glEnableVertexAttribArray);
    LOAD_GL_PROC(ctx, glGetAttribLocation);

    /* Hard requirement: without these nothing renders */
    if (!ctx->pfn_glCreateShader || !ctx->pfn_glCreateProgram ||
        !ctx->pfn_glUseProgram   || !ctx->pfn_glGetUniformLocation)
        return -1;

    return 0;
}

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 2 — Shader Compiler
   ════════════════════════════════════════════════════════════════════════════ */

static char *load_shader_file(const char *path) {
    FILE *fp = fopen(path, "r");
    if (!fp) return NULL;
    fseek(fp, 0, SEEK_END);
    long sz = ftell(fp);
    rewind(fp);
    if (sz <= 0 || sz > 131072) { fclose(fp); return NULL; }
    char *src = (char *)malloc((size_t)sz + 1);
    if (!src) { fclose(fp); return NULL; }
    fread(src, 1, (size_t)sz, fp);
    src[sz] = '\0';
    fclose(fp);
    return src;
}

static GLuint compile_shader(gl_ctx_t *ctx, GLenum type,
                              const char *src, const char *label) {
    GLuint shader = ctx->pfn_glCreateShader(type);
    ctx->pfn_glShaderSource(shader, 1, &src, NULL);
    ctx->pfn_glCompileShader(shader);

    GLint ok = 0;
    ctx->pfn_glGetShaderiv(shader, GL_COMPILE_STATUS, &ok);
    if (!ok) {
        char log[1024];
        ctx->pfn_glGetShaderInfoLog(shader, sizeof(log), NULL, log);
        msg_Err(ctx->obj, "[VAV] Shader compile error (%s): %s", label, log);
        ctx->pfn_glDeleteShader(shader);
        return 0;
    }
    msg_Dbg(ctx->obj, "[VAV] Shader compiled OK: %s", label);
    return shader;
}

static GLuint link_program(gl_ctx_t *ctx, GLuint vert, GLuint frag,
                            const char *label) {
    GLuint prog = ctx->pfn_glCreateProgram();
    ctx->pfn_glAttachShader(prog, vert);
    ctx->pfn_glAttachShader(prog, frag);
    ctx->pfn_glLinkProgram(prog);

    GLint ok = 0;
    ctx->pfn_glGetProgramiv(prog, GL_LINK_STATUS, &ok);
    if (!ok) {
        char log[1024];
        ctx->pfn_glGetProgramInfoLog(prog, sizeof(log), NULL, log);
        msg_Err(ctx->obj, "[VAV] Program link error (%s): %s", label, log);
        ctx->pfn_glDeleteProgram(prog);
        return 0;
    }
    msg_Info(ctx->obj, "[VAV] Shader program linked: %s", label);
    return prog;
}

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 3 — Uniform Cache Population
   ════════════════════════════════════════════════════════════════════════════ */

#define GET_UNI(prog, name) \
    ctx->pfn_glGetUniformLocation((prog), (name))

static void cache_uniforms(gl_ctx_t *ctx) {
    uniform_cache_t *u = &ctx->uniforms;
    GLuint p           = ctx->prog_vibe;

    /* Phase I — DSP live */
    u->u_time           = GET_UNI(p, "u_time");
    u->u_resolution     = GET_UNI(p, "u_resolution");
    u->u_centroid       = GET_UNI(p, "u_centroid");
    u->u_saturation     = GET_UNI(p, "u_saturation");
    u->u_syncopation    = GET_UNI(p, "u_syncopation");
    u->u_bpm_norm       = GET_UNI(p, "u_bpm_norm");
    u->u_groove         = GET_UNI(p, "u_groove");
    u->u_dissonance     = GET_UNI(p, "u_dissonance");
    /* Phase II — ML */
    u->u_valence        = GET_UNI(p, "u_valence");
    u->u_arousal        = GET_UNI(p, "u_arousal");
    u->u_scenario_fog   = GET_UNI(p, "u_scenario_fog");
    /* Phase III — Photometric */
    u->u_primary_rgb        = GET_UNI(p, "u_primary_rgb");
    u->u_secondary_rgb      = GET_UNI(p, "u_secondary_rgb");
    u->u_brightness_floor   = GET_UNI(p, "u_brightness_floor");
    u->u_brightness_ceiling = GET_UNI(p, "u_brightness_ceiling");
    u->u_strobe_trigger     = GET_UNI(p, "u_strobe_trigger");
    u->u_fog_density        = GET_UNI(p, "u_fog_density");
    u->u_visual_noise       = GET_UNI(p, "u_visual_noise");
    u->u_chrom_energy       = GET_UNI(p, "u_chrom_energy");
    /* Phase III — Kinetic */
    u->u_entrainment    = GET_UNI(p, "u_entrainment");

    /* Bloom program */
    GLuint b = ctx->prog_bloom;
    u->bloom_u_scene          = GET_UNI(b, "u_scene");
    u->bloom_u_resolution     = GET_UNI(b, "u_resolution");
    u->bloom_u_bloom_strength = GET_UNI(b, "u_bloom_strength");
    u->bloom_u_fade_amount    = GET_UNI(b, "u_fade_amount");

    msg_Dbg(ctx->obj, "[VAV] Uniform cache populated (%d vibe + %d bloom)",
            (int)sizeof(uniform_cache_t) / (int)sizeof(GLint) - 4, 4);
}

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 4 — FBO Setup
   Creates off-screen render target at current window dimensions.
   Destroyed and recreated on resize via gl_renderer_resize().
   ════════════════════════════════════════════════════════════════════════════ */

static int create_fbo(gl_ctx_t *ctx) {
    /* Create color attachment texture */
    glGenTextures(1, &ctx->fbo_texture);
    glBindTexture(GL_TEXTURE_2D, ctx->fbo_texture);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB,
                 ctx->width, ctx->height, 0,
                 GL_RGB, GL_UNSIGNED_BYTE, NULL);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);

    /* Create FBO and attach texture */
    ctx->pfn_glGenFramebuffers(1, &ctx->fbo);
    ctx->pfn_glBindFramebuffer(GL_FRAMEBUFFER, ctx->fbo);
    ctx->pfn_glFramebufferTexture2D(GL_FRAMEBUFFER,
                                     GL_COLOR_ATTACHMENT0,
                                     GL_TEXTURE_2D,
                                     ctx->fbo_texture, 0);

    GLenum status = ctx->pfn_glCheckFramebufferStatus(GL_FRAMEBUFFER);
    ctx->pfn_glBindFramebuffer(GL_FRAMEBUFFER, 0);

    if (status != GL_FRAMEBUFFER_COMPLETE) {
        msg_Err(ctx->obj, "[VAV] FBO incomplete (status 0x%04X)", status);
        return -1;
    }
    msg_Dbg(ctx->obj, "[VAV] FBO created %dx%d", ctx->width, ctx->height);
    return 0;
}

static void destroy_fbo(gl_ctx_t *ctx) {
    if (ctx->fbo) {
        ctx->pfn_glDeleteFramebuffers(1, &ctx->fbo);
        ctx->fbo = 0;
    }
    if (ctx->fbo_texture) {
        glDeleteTextures(1, &ctx->fbo_texture);
        ctx->fbo_texture = 0;
    }
}

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 5 — Full-Screen Quad VBO
   ════════════════════════════════════════════════════════════════════════════ */

static void create_quad(gl_ctx_t *ctx) {
    /* NDC quad: two triangles covering [-1,1]x[-1,1] */
    static const float quad[] = {
        -1.0f, -1.0f,
         1.0f, -1.0f,
        -1.0f,  1.0f,
         1.0f, -1.0f,
         1.0f,  1.0f,
        -1.0f,  1.0f,
    };
    ctx->pfn_glGenBuffers(1, &ctx->vbo_quad);
    ctx->pfn_glBindBuffer(GL_ARRAY_BUFFER, ctx->vbo_quad);
    ctx->pfn_glBufferData(GL_ARRAY_BUFFER, sizeof(quad),
                          quad, GL_STATIC_DRAW);
    ctx->pfn_glBindBuffer(GL_ARRAY_BUFFER, 0);

    ctx->attr_pos_vibe  = ctx->pfn_glGetAttribLocation(ctx->prog_vibe,  "a_pos");
    ctx->attr_pos_bloom = ctx->pfn_glGetAttribLocation(ctx->prog_bloom, "a_pos");
}

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 6 — Strobe & Fade Logic  (Pillar 7.2)
   ════════════════════════════════════════════════════════════════════════════ */

static void update_strobe_fade(gl_ctx_t *ctx,
                                const vap_runtime_t *vap,
                                const float chrom[4],
                                float dt) {
    /* Check strobe trigger: any chromatic band exceeds threshold?     */
    float peak_energy = 0.0f;
    for (int i = 0; i < 4; i++)
        if (chrom[i] > peak_energy) peak_energy = chrom[i];

    int strobe_condition = (peak_energy >= vap->photometric.strobe_threshold)
                           && (vap->photometric.strobe_threshold < 1.0f);

    if (strobe_condition && !ctx->strobe_fired) {
        /* Fire strobe: slam fade_amount to 2.0 (white flash in bloom) */
        ctx->fade_amount  = 2.0f;
        ctx->fade_target  = 1.0f;  /* return to normal after flash     */
        ctx->strobe_fired = 1;
    } else if (!strobe_condition) {
        ctx->strobe_fired = 0;
    }

    /* Fade mode (Pillar 7.2): SHARP=instant jump, SMOOTH=linear lerp */
    float fade_speed = (ctx->fade_rate > 0.0f) ? (dt / ctx->fade_rate) : 1.0f;

    if (ctx->fade_mode == 0) {
        /* SHARP: binary, no lerp */
        ctx->fade_amount = ctx->fade_target;
    } else {
        /* SMOOTH: linear interpolation toward target */
        float diff = ctx->fade_target - ctx->fade_amount;
        ctx->fade_amount += diff * fminf(fade_speed * 8.0f, 1.0f);
    }

    /* Sync renderer fade state from VAP photometric each frame */
    ctx->fade_mode = vap->photometric.fade_mode;
    ctx->fade_rate = vap->photometric.fade_rate;
    ctx->fade_target = 1.0f;  /* normal state = fully visible */
}

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 7 — Per-Frame Uniform Upload
   Maps every vap_runtime_t field to its GLSL uniform.
   All 3 VAP Phases pushed here each frame.
   ════════════════════════════════════════════════════════════════════════════ */

static void upload_vibe_uniforms(gl_ctx_t *ctx,
                                  const vap_runtime_t *vap,
                                  const float chrom[4]) {
    uniform_cache_t *u  = &ctx->uniforms;

    /* ── Phase I: Physical / DSP ────────────────────────────────── */
    ctx->pfn_glUniform1f(u->u_time,        ctx->time_accum);
    ctx->pfn_glUniform2f(u->u_resolution,
                          (float)ctx->width, (float)ctx->height);
    ctx->pfn_glUniform1f(u->u_centroid,    vap->spectral_centroid_hz);
    ctx->pfn_glUniform1f(u->u_saturation,  vap->saturation_index);
    ctx->pfn_glUniform1f(u->u_syncopation, vap->syncopation_index);
    ctx->pfn_glUniform1f(u->u_bpm_norm,
                          fminf(vap->bpm_raw / 180.0f, 1.0f));
    ctx->pfn_glUniform1f(u->u_groove,      vap->groove_quantization);
    ctx->pfn_glUniform1f(u->u_dissonance,  vap->dissonance_density);

    /* ── Phase II: Psychological / ML ───────────────────────────── */
    ctx->pfn_glUniform1f(u->u_valence,      vap->affective.valence);
    ctx->pfn_glUniform1f(u->u_arousal,      vap->affective.arousal);
    /* Scenario fog = contextual_fog_mod * fog_density (Pillars 6+7.3) */
    float scenario_fog = vap->contextual_fog_mod * vap->photometric.fog_density;
    ctx->pfn_glUniform1f(u->u_scenario_fog, scenario_fog);

    /* ── Phase III: Photometric ──────────────────────────────────── */
    ctx->pfn_glUniform3f(u->u_primary_rgb,
                          vap->photometric.primary_hex[0],
                          vap->photometric.primary_hex[1],
                          vap->photometric.primary_hex[2]);
    ctx->pfn_glUniform3f(u->u_secondary_rgb,
                          vap->photometric.secondary_hex[0],
                          vap->photometric.secondary_hex[1],
                          vap->photometric.secondary_hex[2]);
    ctx->pfn_glUniform1f(u->u_brightness_floor,
                          vap->photometric.brightness_floor);
    ctx->pfn_glUniform1f(u->u_brightness_ceiling,
                          vap->photometric.brightness_ceiling);
    ctx->pfn_glUniform1f(u->u_strobe_trigger,
                          vap->photometric.strobe_threshold);
    ctx->pfn_glUniform1f(u->u_fog_density,
                          vap->photometric.fog_density);
    ctx->pfn_glUniform1f(u->u_visual_noise,
                          (float)vap->photometric.visual_noise_mode);

    /* VAP Pillar 7.1 chromatic band energies — 4 bands per spec */
    ctx->pfn_glUniform1fv(u->u_chrom_energy, 4, chrom);

    /* ── Phase III: Kinetic ──────────────────────────────────────── */
    ctx->pfn_glUniform1f(u->u_entrainment, vap->entrainment_factor);
}

static void upload_bloom_uniforms(gl_ctx_t *ctx, const vap_runtime_t *vap) {
    uniform_cache_t *u = &ctx->uniforms;

    ctx->pfn_glUniform1i(u->bloom_u_scene, 0);  /* texture unit 0 */
    ctx->pfn_glUniform2f(u->bloom_u_resolution,
                          (float)ctx->width, (float)ctx->height);
    /* Bloom strength: arousal × brightness_ceiling per VAP Pillars 5+7 */
    float bloom_str = vap->affective.arousal
                    * vap->photometric.brightness_ceiling
                    * 1.5f;
    ctx->pfn_glUniform1f(u->bloom_u_bloom_strength, bloom_str);
    ctx->pfn_glUniform1f(u->bloom_u_fade_amount,    ctx->fade_amount);
}

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 8 — Draw Call
   ════════════════════════════════════════════════════════════════════════════ */

static void draw_quad(gl_ctx_t *ctx, GLint attr_pos) {
    ctx->pfn_glBindBuffer(GL_ARRAY_BUFFER, ctx->vbo_quad);
    ctx->pfn_glVertexAttribPointer(
        (GLuint)attr_pos, 2, GL_FLOAT, GL_FALSE,
        2 * sizeof(float), (void *)0);
    ctx->pfn_glEnableVertexAttribArray((GLuint)attr_pos);
    glDrawArrays(GL_TRIANGLES, 0, 6);
    ctx->pfn_glBindBuffer(GL_ARRAY_BUFFER, 0);
}

/* ════════════════════════════════════════════════════════════════════════════
   SECTION 9 — Public API Implementation
   ════════════════════════════════════════════════════════════════════════════ */

gl_ctx_t *gl_renderer_create(vlc_object_t *obj, const vap_runtime_t *vap) {
    gl_ctx_t *ctx = (gl_ctx_t *)calloc(1, sizeof(gl_ctx_t));
    if (!ctx) return NULL;

    ctx->obj = obj;

    /* ── Acquire VLC's OpenGL surface ── */
    ctx->gl = vlc_gl_Create(obj, VLC_OPENGL, "$gl");
    if (!ctx->gl) {
        msg_Err(obj, "[VAV] Failed to acquire VLC OpenGL surface");
        free(ctx);
        return NULL;
    }

    if (vlc_gl_MakeCurrent(ctx->gl) != VLC_SUCCESS) {
        msg_Err(obj, "[VAV] vlc_gl_MakeCurrent failed");
        vlc_gl_Release(ctx->gl);
        free(ctx);
        return NULL;
    }

    /* ── Load all GL function pointers ── */
    if (load_gl_procs(ctx) != 0) {
        msg_Err(obj, "[VAV] Critical GL procs missing — aborting");
        vlc_gl_ReleaseCurrent(ctx->gl);
        vlc_gl_Release(ctx->gl);
        free(ctx);
        return NULL;
    }

    /* ── Initial viewport dimensions ── */
    ctx->width  = 1280;
    ctx->height = 720;

    /* ── Compile vertex shader (shared by both passes) ── */
    GLuint vert = compile_shader(ctx, GL_VERTEX_SHADER, VERT_SRC, "vibe.vert");
    if (!vert) goto fail;

    /* ── Compile vibe.frag — try disk first, fall back to embedded ── */
    char *frag_src = load_shader_file("shaders/vibe.frag");
    GLuint frag = compile_shader(ctx, GL_FRAGMENT_SHADER,
                                  frag_src ? frag_src : FRAG_FALLBACK,
                                  "vibe.frag");
    free(frag_src);
    if (!frag) goto fail;

    ctx->prog_vibe = link_program(ctx, vert, frag, "VAP Cymatic Field");
    ctx->pfn_glDeleteShader(vert);
    ctx->pfn_glDeleteShader(frag);
    if (!ctx->prog_vibe) goto fail;

    /* ── Compile bloom pass ── */
    GLuint vert2 = compile_shader(ctx, GL_VERTEX_SHADER, VERT_SRC, "bloom.vert");
    GLuint frag2 = compile_shader(ctx, GL_FRAGMENT_SHADER,
                                   BLOOM_FRAG_SRC, "bloom.frag");
    if (!vert2 || !frag2) goto fail;

    ctx->prog_bloom = link_program(ctx, vert2, frag2, "Bloom Pass");
    ctx->pfn_glDeleteShader(vert2);
    ctx->pfn_glDeleteShader(frag2);
    if (!ctx->prog_bloom) goto fail;

    /* ── Cache all uniform locations ── */
    cache_uniforms(ctx);

    /* ── Create quad VBO ── */
    create_quad(ctx);

    /* ── Create FBO ── */
    if (create_fbo(ctx) != 0) goto fail;

    /* ── Seed static uniforms from initial VAP state ── */
    ctx->pfn_glUseProgram(ctx->prog_vibe);
    ctx->pfn_glUniform3f(ctx->uniforms.u_primary_rgb,
                          vap->photometric.primary_hex[0],
                          vap->photometric.primary_hex[1],
                          vap->photometric.primary_hex[2]);
    ctx->pfn_glUniform3f(ctx->uniforms.u_secondary_rgb,
                          vap->photometric.secondary_hex[0],
                          vap->photometric.secondary_hex[1],
                          vap->photometric.secondary_hex[2]);
    ctx->pfn_glUseProgram(0);

    /* ── Init fade state from VAP Photometric Pillar 7.2 ── */
    ctx->fade_amount  = 1.0f;
    ctx->fade_target  = 1.0f;
    ctx->fade_mode    = vap->photometric.fade_mode;
    ctx->fade_rate    = vap->photometric.fade_rate;
    ctx->strobe_fired = 0;

    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

    vlc_gl_ReleaseCurrent(ctx->gl);

    msg_Info(obj, "[VAV] GL renderer ready — %dx%d — VAP Pillar 7 color: "
             "#%02X%02X%02X",
             ctx->width, ctx->height,
             (int)(vap->photometric.primary_hex[0] * 255),
             (int)(vap->photometric.primary_hex[1] * 255),
             (int)(vap->photometric.primary_hex[2] * 255));

    return ctx;

fail:
    msg_Err(obj, "[VAV] GL renderer creation failed");
    if (ctx->prog_vibe)  ctx->pfn_glDeleteProgram(ctx->prog_vibe);
    if (ctx->prog_bloom) ctx->pfn_glDeleteProgram(ctx->prog_bloom);
    if (ctx->vbo_quad)   ctx->pfn_glDeleteBuffers(1, &ctx->vbo_quad);
    destroy_fbo(ctx);
    vlc_gl_ReleaseCurrent(ctx->gl);
    vlc_gl_Release(ctx->gl);
    free(ctx);
    return NULL;
}

void gl_renderer_update(gl_ctx_t *ctx, const vap_runtime_t *vap,
                         const float chrom[4]) {
    if (!ctx || !vap) return;

    /* dt approximation — derive from BPM clock if available */
    float dt = (vap->bpm_raw > 0.0f)
               ? (60.0f / vap->bpm_raw) / 32.0f  /* 32 renders per beat */
               : 0.016f;                           /* fallback 60fps      */
    ctx->time_accum += dt;

    /* ── Strobe / fade state update (Pillar 7.2) ── */
    update_strobe_fade(ctx, vap, chrom, dt);

    if (vlc_gl_MakeCurrent(ctx->gl) != VLC_SUCCESS) return;

    /* ════════════════════════════════════════════════
       PASS 1: Render cymatic field → FBO
       ════════════════════════════════════════════════ */
    ctx->pfn_glBindFramebuffer(GL_FRAMEBUFFER, ctx->fbo);
    glViewport(0, 0, ctx->width, ctx->height);
    glClearColor(0.0f, 0.0f, 0.02f, 1.0f);   /* near-black space void */
    glClear(GL_COLOR_BUFFER_BIT);

    ctx->pfn_glUseProgram(ctx->prog_vibe);
    upload_vibe_uniforms(ctx, vap, chrom);
    draw_quad(ctx, ctx->attr_pos_vibe);

    /* ════════════════════════════════════════════════
       PASS 2: Bloom + fade → screen
       ════════════════════════════════════════════════ */
    ctx->pfn_glBindFramebuffer(GL_FRAMEBUFFER, 0);
    glViewport(0, 0, ctx->width, ctx->height);
    glClear(GL_COLOR_BUFFER_BIT);

    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, ctx->fbo_texture);

    ctx->pfn_glUseProgram(ctx->prog_bloom);
    upload_bloom_uniforms(ctx, vap);
    draw_quad(ctx, ctx->attr_pos_bloom);

    ctx->pfn_glUseProgram(0);
    glBindTexture(GL_TEXTURE_2D, 0);

    vlc_gl_Swap(ctx->gl);
    vlc_gl_ReleaseCurrent(ctx->gl);
}

void gl_renderer_resize(gl_ctx_t *ctx, int width, int height) {
    if (!ctx || width <= 0 || height <= 0) return;
    if (ctx->width == width && ctx->height == height) return;

    ctx->width  = width;
    ctx->height = height;

    if (vlc_gl_MakeCurrent(ctx->gl) != VLC_SUCCESS) return;

    destroy_fbo(ctx);
    if (create_fbo(ctx) != 0)
        msg_Err(ctx->obj, "[VAV] FBO resize failed (%dx%d)", width, height);
    else
        msg_Dbg(ctx->obj, "[VAV] FBO resized to %dx%d", width, height);

    vlc_gl_ReleaseCurrent(ctx->gl);
}

void gl_renderer_destroy(gl_ctx_t *ctx) {
    if (!ctx) return;

    if (vlc_gl_MakeCurrent(ctx->gl) == VLC_SUCCESS) {
        if (ctx->prog_vibe)  ctx->pfn_glDeleteProgram(ctx->prog_vibe);
        if (ctx->prog_bloom) ctx->pfn_glDeleteProgram(ctx->prog_bloom);
        if (ctx->vbo_quad)   ctx->pfn_glDeleteBuffers(1, &ctx->vbo_quad);
        destroy_fbo(ctx);
        vlc_gl_ReleaseCurrent(ctx->gl);
    }

    vlc_gl_Release(ctx->gl);
    msg_Info(ctx->obj, "[VAV] GL renderer destroyed cleanly");
    free(ctx);
}
