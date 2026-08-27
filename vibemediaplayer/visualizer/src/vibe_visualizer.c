/*****************************************************************************
 * vibe_visualizer.c — Vibe Audio Visualizer — VLC Plugin Entry Point
 * Aurphyx SUXS / rAE
 *
 * V.A.P. v3.1 — 9-Pillar TSLCA Cymatic Engine
 * Three-Squared-Lattice Cognitive Architecture
 *
 * This file is the sole VLC plugin registration point.
 * It owns:
 *   - Module descriptor (vlc_module_begin/end)
 *   - Open()    — allocate sys, load VAP, init DSP + GL renderer
 *   - Close()   — clean teardown of all subsystems
 *   - DoWork()  — per-audio-block: DSP → VAP update → GL render
 *   - Thread-safety via vlc_mutex around vap_runtime_t writes
 *   - Input item observer to reload VAP sidecar on track change
 *   - Config variables exposed to VLC preferences UI
 *
 * Plugin appears in:
 *   VLC → Audio → Visualizations → "Vibe Audio Visualizer"
 *
 * Capabilities registered:
 *   "visualization" — the same capability class as Goom, Spectrum,
 *   projectM. VLC's audio chain inserts this as a passthrough filter.
 *****************************************************************************/

#ifdef HAVE_CONFIG_H
# include "config.h"
#endif

/* VLC core headers */
#include <vlc_common.h>
#include <vlc_plugin.h>
#include <vlc_filter.h>
#include <vlc_aout.h>
#include <vlc_input.h>
#include <vlc_input_item.h>
#include <vlc_url.h>
#include <vlc_threads.h>

/* VAV subsystem headers */
#include "vap_runtime.h"
#include "vap_loader.h"
#include "dsp_engine.h"
#include "gl_renderer.h"

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 1 — Constants & Config Keys
   ═══════════════════════════════════════════════════════════════════════════ */

#define VAV_MODULE_NAME    "vibe_visualizer"
#define VAV_DISPLAY_NAME   "Vibe Audio Visualizer"
#define VAV_DESCRIPTION    N_("Cymatic sacred geometry visualization " \
                               "powered by V.A.P. v3.1 (Aurphyx SUXS)")
#define VAV_HELP           N_("Renders 9-pillar experiential audio metadata " \
                               "as real-time cymatic standing wave geometry.")

/* Config variable keys (exposed in VLC Preferences → Audio → Visualizations) */
#define VAV_CFG_FFT_SIZE    VAV_MODULE_NAME "-fft-size"
#define VAV_CFG_SIDECAR_DIR VAV_MODULE_NAME "-sidecar-dir"
#define VAV_CFG_BLOOM       VAV_MODULE_NAME "-bloom"
#define VAV_CFG_GLITCH      VAV_MODULE_NAME "-glitch-override"

/* DSP constants */
#define VAV_FFT_SIZE_DEFAULT  2048
#define VAV_FFT_SIZE_MIN       512
#define VAV_FFT_SIZE_MAX      8192

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 2 — Private System State  (filter_sys_t)
   One instance per VLC filter chain insertion.
   ═══════════════════════════════════════════════════════════════════════════ */

typedef struct {
    /* ── VAP Runtime ─────────────────────────────────────────────────── */
    vap_runtime_t   vap;           /* Full 9-pillar state                 */
    vlc_mutex_t     vap_lock;      /* Guards writes from DoWork thread    */

    /* ── DSP Engine ──────────────────────────────────────────────────── */
    dsp_ctx_t      *dsp;           /* FFT + onset + chromatic band engine */
    int             fft_size;      /* Configured FFT window size          */
    float          *fft_mag;       /* Heap FFT magnitude output buffer    */
    float           chrom[4];      /* VAP Pillar 7.1 band energies [0-1]  */

    /* ── GL Renderer ─────────────────────────────────────────────────── */
    gl_ctx_t       *gl;            /* Full OpenGL pipeline context        */

    /* ── Track change detection ──────────────────────────────────────── */
    char            current_uri[4096]; /* Last loaded audio URI           */

    /* ── Plugin lifecycle ────────────────────────────────────────────── */
    bool            running;
    vlc_object_t   *obj;           /* Back-pointer for logging            */

} filter_sys_t;

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 3 — Forward Declarations
   ═══════════════════════════════════════════════════════════════════════════ */

static int        Open   (vlc_object_t *);
static void       Close  (vlc_object_t *);
static block_t   *DoWork (filter_t *, block_t *);

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 4 — VLC Module Descriptor
   This is the metadata VLC reads at plugin load time to populate
   the Audio → Visualizations menu and Preferences UI.
   ═══════════════════════════════════════════════════════════════════════════ */

vlc_module_begin()
    set_shortname(N_(VAV_DISPLAY_NAME))
    set_description(VAV_DESCRIPTION)
    set_help(VAV_HELP)
    set_capability("visualization", 0)
    set_category(CAT_AUDIO)
    set_subcategory(SUBCAT_AUDIO_VISUAL)
    set_callbacks(Open, Close)
    add_shortcut("vibe", "vap", "cymatic")

    /* ── User-configurable options ── */
    add_integer(VAV_CFG_FFT_SIZE, VAV_FFT_SIZE_DEFAULT,
                N_("FFT Window Size"),
                N_("Larger values increase frequency resolution "
                   "at the cost of latency. Must be power of 2."))
        change_integer_range(VAV_FFT_SIZE_MIN, VAV_FFT_SIZE_MAX)

    add_string(VAV_CFG_SIDECAR_DIR, "",
               N_("V.A.P. Sidecar Directory"),
               N_("Optional directory to search for .vap.json files. "
                  "Leave empty to look in same directory as audio file."))

    add_bool(VAV_CFG_BLOOM, true,
             N_("Enable Bloom Post-Process"),
             N_("Two-pass bloom for luminance glow. "
                "Intensity driven by V.A.P. Pillar 5 Arousal."))

    add_bool(VAV_CFG_GLITCH, false,
             N_("Force Glitch/Noise Mode"),
             N_("Override V.A.P. Pillar 7.3 Visual Noise with maximum glitch. "
                "Useful for testing or Lo-Fi aesthetic."))

vlc_module_end()

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 5 — VAP Chromatic Band Energy Extraction
   Maps FFT magnitude buffer to the 4 chromatic bands defined in
   V.A.P. Pillar 7.1 Logic Architecture:
     [0] Sub-Bass  40–60 Hz   → 700nm Deep Red
     [1] Low-Mid   60–250 Hz  → 600nm Orange/Amber
     [2] Mids      250–2k Hz  → 520nm Green/Teal
     [3] Highs     2k–20k Hz  → 450nm Blue/UV
   ═══════════════════════════════════════════════════════════════════════════ */

static const float VAP_CHROM_LOW[4]  = {  40.0f,   60.0f,  250.0f,  2000.0f };
static const float VAP_CHROM_HIGH[4] = {  60.0f,  250.0f, 2000.0f, 20000.0f };

static void extract_chromatic_bands(const float *fft_mag, int fft_size,
                                     int sample_rate, float chrom[4]) {
    float bin_hz = (float)sample_rate / (float)(fft_size * 2);

    for (int b = 0; b < 4; b++) {
        int lo  = (int)(VAP_CHROM_LOW[b]  / bin_hz);
        int hi  = (int)(VAP_CHROM_HIGH[b] / bin_hz);
        if (lo  < 0)        lo = 0;
        if (hi  >= fft_size) hi = fft_size - 1;

        float energy = 0.0f;
        int   count  = hi - lo + 1;
        for (int i = lo; i <= hi; i++)
            energy += fft_mag[i] * fft_mag[i];

        chrom[b] = (count > 0) ? sqrtf(energy / (float)count) : 0.0f;
        /* Normalize with smooth clamp — peaks near 1.0 at loud passages */
        chrom[b] = 1.0f - expf(-chrom[b] * 4.0f);
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 6 — Track Change Handling
   Called from DoWork() on every block. Detects when the input URI has
   changed (new track) and triggers a fresh VAP sidecar load.
   This is the mechanism that updates Photometric/Affective/Contextual
   pillar values on track change without restarting the plugin.
   ═══════════════════════════════════════════════════════════════════════════ */

static void check_track_change(filter_t *filter, filter_sys_t *sys) {
    /* Retrieve current playing item URI from VLC input thread */
    input_thread_t *input =
        (input_thread_t *)vlc_object_find(VLC_OBJECT(filter),
                                           VLC_OBJECT_INPUT,
                                           FIND_ANYWHERE);
    if (!input) return;

    input_item_t *item = input_GetItem(input);
    if (!item) {
        vlc_object_release(input);
        return;
    }

    char *uri = input_item_GetURI(item);
    vlc_object_release(input);
    if (!uri) return;

    /* Compare to last loaded URI — only reload if track changed */
    if (strncmp(uri, sys->current_uri, sizeof(sys->current_uri) - 1) == 0) {
        free(uri);
        return;
    }

    /* Track has changed — update stored URI */
    strncpy(sys->current_uri, uri, sizeof(sys->current_uri) - 1);
    sys->current_uri[sizeof(sys->current_uri) - 1] = '\0';

    /* Convert VLC URI to local file path for VAP loader */
    char *path = vlc_uri2path(uri);
    free(uri);

    /* Lock VAP state during update */
    vlc_mutex_lock(&sys->vap_lock);

    /* Re-initialise runtime (clears Phase I live fields, keeps defaults) */
    vap_runtime_init(&sys->vap);

    /* Load VAP data for new track — tries sidecar → ID3 → Vorbis → defaults */
    int result = vap_loader_load(&sys->vap, path);
    free(path);

    vlc_mutex_unlock(&sys->vap_lock);

    msg_Info(VLC_OBJECT(filter), "[VAV] Track change → %s",
             vap_loader_result_str(result));
    msg_Dbg(VLC_OBJECT(filter),
            "[VAV] New track: \"%s\" by \"%s\" | "
            "Valence=%.2f Arousal=%.2f BPM=%.1f "
            "Primary=#%02X%02X%02X Scenario=%s",
            sys->vap.identity_title,
            sys->vap.identity_artist,
            sys->vap.affective.valence,
            sys->vap.affective.arousal,
            sys->vap.bpm_raw,
            (int)(sys->vap.photometric.primary_hex[0] * 255),
            (int)(sys->vap.photometric.primary_hex[1] * 255),
            (int)(sys->vap.photometric.primary_hex[2] * 255),
            sys->vap.scenario_tag);
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 7 — DoWork()
   The audio filter hot path. Called by VLC's audio thread for every
   decoded audio block (typically ~10–30ms of PCM at a time).

   Pipeline per call:
     1. Check for track change → reload VAP sidecar if needed
     2. Feed PCM to DSP engine → update Phase I VAP fields
     3. Extract 4 chromatic band energies (Pillar 7.1)
     4. Push updated VAP state + chrom[] to GL renderer
     5. Pass audio block through UNMODIFIED (we are a passthrough filter)
   ═══════════════════════════════════════════════════════════════════════════ */

static block_t *DoWork(filter_t *filter, block_t *block) {
    filter_sys_t *sys = filter->p_sys;

    if (!sys || !sys->running || !block) return block;

    /* ── 1. Track change detection ── */
    check_track_change(filter, sys);

    /* ── 2. DSP Phase I analysis ── */
    const int   channels   = filter->fmt_in.audio.i_channels;
    const int   n_samples  = block->i_nb_samples;
    const int   sample_rate = filter->fmt_in.audio.i_rate;
    const float *pcm        = (const float *)block->p_buffer;

    vlc_mutex_lock(&sys->vap_lock);

    /* Compute FFT magnitudes */
    float dt = (n_samples > 0 && sample_rate > 0)
               ? (float)n_samples / (float)sample_rate
               : 0.016f;

    dsp_engine_process(sys->dsp, pcm, n_samples, channels,
                       sys->fft_mag, sys->fft_size);

    /* Update Phase I VAP fields from live DSP */
    dsp_engine_update(&sys->vap, sys->fft_mag, sys->fft_size,
                      sample_rate, dt);

    /* ── 3. Extract VAP Pillar 7.1 chromatic band energies ── */
    extract_chromatic_bands(sys->fft_mag, sys->fft_size,
                             sample_rate, sys->chrom);

    /* Take a local snapshot for GL thread safety */
    vap_runtime_t vap_snap;
    float         chrom_snap[4];
    memcpy(&vap_snap,    &sys->vap,   sizeof(vap_runtime_t));
    memcpy(chrom_snap,   sys->chrom,  sizeof(sys->chrom));

    vlc_mutex_unlock(&sys->vap_lock);

    /* ── 4. Push to GL renderer (happens on audio thread — VLC handles sync) */
    if (sys->gl)
        gl_renderer_update(sys->gl, &vap_snap, chrom_snap);

    /* ── 5. Pass audio through unmodified ── */
    return block;
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 8 — Open()
   Called by VLC when the user selects "Vibe Audio Visualizer" from
   Audio → Visualizations, or when --audio-visual=vibe is passed via CLI.

   Responsibilities:
     - Allocate and zero filter_sys_t
     - Read config variables (FFT size, bloom, glitch override)
     - Initialise VAP runtime + load sidecar for currently playing track
     - Allocate FFT magnitude buffer
     - Create DSP engine context
     - Create GL renderer context (acquires OpenGL surface from VLC)
     - Register DoWork audio filter callback
     - Validate audio format (requires float32 PCM — standard in VLC 3+)
   ═══════════════════════════════════════════════════════════════════════════ */

static int Open(vlc_object_t *obj) {
    filter_t     *filter = (filter_t *)obj;
    filter_sys_t *sys    = (filter_sys_t *)calloc(1, sizeof(filter_sys_t));
    if (!sys) return VLC_ENOMEM;

    sys->obj     = obj;
    sys->running = false;

    /* ── Validate audio input format ── */
    /* VLC audio filters receive float32 interleaved PCM */
    if (filter->fmt_in.audio.i_format != VLC_CODEC_FL32) {
        msg_Err(obj, "[VAV] Requires float32 PCM input "
                     "(got fourcc: %4.4s)",
                (char *)&filter->fmt_in.audio.i_format);
        free(sys);
        return VLC_EGENERIC;
    }

    /* ── Read config variables ── */
    sys->fft_size = var_InheritInteger(obj, VAV_CFG_FFT_SIZE);
    /* Clamp to power-of-2 range */
    if (sys->fft_size < VAV_FFT_SIZE_MIN) sys->fft_size = VAV_FFT_SIZE_MIN;
    if (sys->fft_size > VAV_FFT_SIZE_MAX) sys->fft_size = VAV_FFT_SIZE_MAX;

    bool bloom_enabled    = var_InheritBool(obj, VAV_CFG_BLOOM);
    bool glitch_override  = var_InheritBool(obj, VAV_CFG_GLITCH);

    msg_Info(obj, "[VAV] Opening — FFT=%d Bloom=%s Glitch=%s",
             sys->fft_size,
             bloom_enabled   ? "ON" : "OFF",
             glitch_override ? "ON" : "OFF");

    /* ── Mutex init ── */
    vlc_mutex_init(&sys->vap_lock);

    /* ── VAP Runtime init ── */
    vap_runtime_init(&sys->vap);

    /* ── Attempt to load VAP sidecar for current track ── */
    /* Get the URI of the currently playing item */
    input_thread_t *input =
        (input_thread_t *)vlc_object_find(obj, VLC_OBJECT_INPUT,
                                           FIND_ANYWHERE);
    char *audio_path = NULL;
    if (input) {
        input_item_t *item = input_GetItem(input);
        if (item) {
            char *uri = input_item_GetURI(item);
            if (uri) {
                audio_path = vlc_uri2path(uri);
                strncpy(sys->current_uri, uri,
                        sizeof(sys->current_uri) - 1);
                free(uri);
            }
        }
        vlc_object_release(input);
    }

    int vap_result = vap_loader_load(&sys->vap, audio_path);
    free(audio_path);

    msg_Info(obj, "[VAV] %s", vap_loader_result_str(vap_result));
    msg_Info(obj, "[VAV] Track: \"%s\" by \"%s\"",
             sys->vap.identity_title, sys->vap.identity_artist);
    msg_Info(obj, "[VAV] Thayer: Valence=%.2f Arousal=%.2f Dominance=%.2f",
             sys->vap.affective.valence,
             sys->vap.affective.arousal,
             sys->vap.affective.dominance);
    msg_Info(obj, "[VAV] Photometric: Primary=#%02X%02X%02X "
                  "Floor=%.2f Ceiling=%.2f Fog=%.2f",
             (int)(sys->vap.photometric.primary_hex[0] * 255),
             (int)(sys->vap.photometric.primary_hex[1] * 255),
             (int)(sys->vap.photometric.primary_hex[2] * 255),
             sys->vap.photometric.brightness_floor,
             sys->vap.photometric.brightness_ceiling,
             sys->vap.photometric.fog_density);
    msg_Info(obj, "[VAV] Kinetic: Entrainment=%.0f MET=%.1f HR=%s",
             sys->vap.entrainment_factor,
             sys->vap.met_score,
             sys->vap.target_hr_zone);

    /* ── Apply config overrides to VAP state ── */
    if (glitch_override)
        sys->vap.photometric.visual_noise_mode = 1;
    if (!bloom_enabled)
        sys->vap.photometric.brightness_ceiling =
            fminf(sys->vap.photometric.brightness_ceiling, 0.5f);

    /* ── Allocate FFT magnitude buffer ── */
    sys->fft_mag = (float *)calloc((size_t)sys->fft_size, sizeof(float));
    if (!sys->fft_mag) {
        msg_Err(obj, "[VAV] Failed to allocate FFT buffer (%d floats)",
                sys->fft_size);
        goto err_fft;
    }

    /* ── Create DSP engine ── */
    sys->dsp = dsp_engine_create(sys->fft_size,
                                  filter->fmt_in.audio.i_rate,
                                  filter->fmt_in.audio.i_channels);
    if (!sys->dsp) {
        msg_Err(obj, "[VAV] DSP engine creation failed");
        goto err_dsp;
    }

    /* ── Create GL renderer ── */
    sys->gl = gl_renderer_create(obj, &sys->vap);
    if (!sys->gl) {
        msg_Err(obj, "[VAV] GL renderer creation failed");
        goto err_gl;
    }

    /* ── Register audio filter callback ── */
    filter->p_sys           = sys;
    filter->pf_audio_filter = DoWork;

    /* ── Output format = input format (passthrough) ── */
    filter->fmt_out.audio = filter->fmt_in.audio;

    sys->running = true;

    msg_Info(obj, "[VAV] ✓ Vibe Audio Visualizer is LIVE — "
                  "V.A.P. v3.1 · 9 Pillars · TSLCA");
    return VLC_SUCCESS;

    /* ── Error unwind ── */
err_gl:
    dsp_engine_destroy(sys->dsp);
err_dsp:
    free(sys->fft_mag);
err_fft:
    vlc_mutex_destroy(&sys->vap_lock);
    free(sys);
    return VLC_EGENERIC;
}

/* ═══════════════════════════════════════════════════════════════════════════
   SECTION 9 — Close()
   Called by VLC when the visualization is deselected or VLC exits.
   Full reverse-order teardown of all subsystems.
   ═══════════════════════════════════════════════════════════════════════════ */

static void Close(vlc_object_t *obj) {
    filter_t     *filter = (filter_t *)obj;
    filter_sys_t *sys    = filter->p_sys;

    if (!sys) return;

    msg_Info(obj, "[VAV] Closing Vibe Audio Visualizer...");

    /* Signal DoWork to stop processing immediately */
    sys->running = false;

    /* Teardown in reverse-creation order */
    if (sys->gl)      gl_renderer_destroy(sys->gl);
    if (sys->dsp)     dsp_engine_destroy(sys->dsp);
    if (sys->fft_mag) free(sys->fft_mag);

    vlc_mutex_destroy(&sys->vap_lock);

    free(sys);
    filter->p_sys = NULL;

    msg_Info(obj, "[VAV] ✓ Vibe Audio Visualizer closed cleanly.");
}
