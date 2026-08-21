#ifndef VAP_RUNTIME_H
#define VAP_RUNTIME_H

#include "vap_photometric.h"
#include "vap_affective.h"

/* V.A.P. v3.1 — Full Runtime State
   Phase I fields updated every audio frame by dsp_engine.c
   Phase II/III fields loaded from .vap.json sidecar at track start  */

typedef struct {
    /* ── PHASE I: DSP (live, per-frame) ─────────────────────────── */
    /* Pillar 1: Structural */
    float    bpm_raw;
    float    bpm_perceived;
    float    groove_quantization;   /* 0.0=Machine Lock 1.0=Human Swing */
    float    syncopation_index;     /* 0.0–1.0 */
    float    kick_transient_ms;

    /* Pillar 2: Tonal */
    char     key[8];                /* e.g. "F#m", "Bb_Dorian"        */
    float    dissonance_density;    /* 0.0–1.0 (% duration dissonant)  */
    float    chord_complexity;      /* 0.0=Triadic 1.0=Extended 13th   */

    /* Pillar 3: Timbral */
    float    spectral_centroid_hz;
    float    saturation_index;      /* THD: 0.0=Sine 1.0=Bitcrush      */
    float    dynamic_range_lra;
    int      spatial_width;         /* 0=Mono 1=Stereo 2=Immersive     */

    /* ── PHASE II: ML (loaded from .vap.json) ───────────────────── */
    vap_affective_t  affective;     /* Pillar 5: Thayer coords         */
    float    scenario_confidence;   /* Pillar 6: Bayesian scenario %   */
    char     scenario_tag[32];      /* e.g. "Night_Drive"              */

    /* ── PHASE III: I/O (loaded from .vap.json) ─────────────────── */
    vap_photometric_t photometric;  /* Pillar 7: Lights & color        */
    float    entrainment_factor;    /* Pillar 8: 0–100 motor response  */
    float    met_score;             /* Pillar 8: Metabolic equivalent  */

    /* ── Internal runtime ────────────────────────────────────────── */
    float    phase_time;            /* Accumulates seconds             */
    uint32_t frame_count;
    int      vap_loaded;            /* 1 if .vap.json sidecar present  */
} vap_runtime_t;

void vap_runtime_init(vap_runtime_t *vap);
void vap_runtime_load_json(vap_runtime_t *vap, const char *filepath);
void vap_runtime_update_dsp(vap_runtime_t *vap,
                             const float *fft_mag, int fft_size,
                             int sample_rate, float dt);

#endif /* VAP_RUNTIME_H */
