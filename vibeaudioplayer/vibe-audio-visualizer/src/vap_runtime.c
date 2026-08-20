/*
 * vap_runtime.c  —  V.A.P. v3.1 Runtime State Implementation
 * Vibe Audio Visualizer  |  Aurphyx © SUXS rAE
 *
 * Responsibilities:
 *   1. vap_runtime_init()       — zero + safe neutral defaults (§3.2)
 *   2. vap_runtime_update_dsp() — Phase I / live Phase II per audio frame
 *   3. vap_runtime_load_json()  — shim → vap_loader_parse_json()
 */

#include "vap_runtime.h"
#include "vap_loader.h"

#include <string.h>
#include <math.h>
#include <stdint.h>

/* ── Internal helpers ─────────────────────────────────────────────────── */

static inline float clampf(float v, float lo, float hi)
{
    return v < lo ? lo : (v > hi ? hi : v);
}

static inline float ema(float prev, float next, float alpha)
{
    return prev + alpha * (next - prev);
}

/* ── §1  Spectral helpers ─────────────────────────────────────────────── */

static float compute_centroid(const float *mag, int n, int sr)
{
    float num = 0.0f, den = 0.0f;
    float bin_hz = (float)sr / (float)(n * 2);
    for (int i = 1; i < n; ++i) {
        float f = (float)i * bin_hz;
        num += f * mag[i];
        den += mag[i];
    }
    return den > 1e-6f ? num / den : 0.0f;
}

static float compute_saturation(const float *mag, int n)
{
    if (n < 4) return 0.0f;
    float fundamental = mag[2] * mag[2];
    float harmonics   = 0.0f;
    for (int i = 3; i < n && i < 12; ++i)
        harmonics += mag[i] * mag[i];
    float total = fundamental + harmonics;
    return total > 1e-6f ? harmonics / total : 0.0f;
}

static float compute_syncopation(const float *mag, int n,
                                  float bpm, int sr, float dt)
{
    (void)bpm; (void)sr; (void)dt;
    float even = 0.0f, odd = 0.0f;
    int half = n / 2;
    for (int i = 0;    i < half; ++i) even += mag[i];
    for (int i = half; i < n;   ++i) odd  += mag[i];
    float total = even + odd;
    return total > 1e-6f ? odd / total : 0.0f;
}

static void compute_band_energies(const float *mag, int n, int sr,
                                   float out[4])
{
    static const float BAND_LO[4] = {  40.f,   60.f,  250.f, 2000.f };
    static const float BAND_HI[4] = {  60.f,  250.f, 2000.f, 20000.f};

    float bin_hz = (float)sr / (float)(n * 2);
    float sums[4]   = {0};
    int   counts[4] = {0};

    for (int i = 1; i < n; ++i) {
        float f = (float)i * bin_hz;
        for (int b = 0; b < 4; ++b) {
            if (f >= BAND_LO[b] && f < BAND_HI[b]) {
                sums[b]   += mag[i];
                counts[b] += 1;
            }
        }
    }
    for (int b = 0; b < 4; ++b) {
        float avg = counts[b] > 0 ? sums[b] / (float)counts[b] : 0.0f;
        out[b] = clampf(avg, 0.0f, 1.0f);
    }
}

/* ── §2  Public API ───────────────────────────────────────────────────── */

void vap_runtime_init(vap_runtime_t *vap)
{
    if (!vap) return;
    memset(vap, 0, sizeof(vap_runtime_t));

    /* Identity */
    strncpy(vap->identity.title,  "Unknown", sizeof(vap->identity.title)  - 1);
    strncpy(vap->identity.artist, "Unknown", sizeof(vap->identity.artist) - 1);

    /* Pillar 1 — STRUCTURAL */
    vap->bpm_raw             = 120.0f;
    vap->bpm_perceived       = 120.0f;
    vap->groove_quantization = 0.3f;
    vap->syncopation_index   = 0.3f;
    vap->kick_transient_ms   = 15.0f;

    /* Pillar 2 — TONAL */
    strncpy(vap->key, "C", sizeof(vap->key) - 1);
    vap->dissonance_density = 0.1f;
    vap->chord_complexity   = 0.3f;

    /* Pillar 3 — TIMBRAL */
    vap->spectral_centroid_hz = 800.0f;
    vap->saturation_index     = 0.2f;
    vap->dynamic_range_lra    = 8.0f;
    vap->spatial_width        = 1;   /* STEREO */

    /* Pillar 4 — LINGUISTIC */
    vap->explicit_tier = 0;  /* CLEAN */

    /* Pillar 5 — AFFECTIVE (Thayer neutral) */
    vap->affective.valence            =  0.0f;
    vap->affective.arousal            =  0.5f;
    vap->affective.dominance          =  0.5f;
    vap->affective.mood_stability     =  0.7f;
    vap->affective.catharsis_potential = 0.3f;
    vap->affective.nostalgia_trigger   = 0.2f;
    vap->affective.buildup_velocity    = 0.4f;
    vap->affective.resolution_state   = 0;   /* TRIUMPHANT */

    /* Pillar 6 — CONTEXTUAL */
    vap->scenario_confidence = 0.0f;
    strncpy(vap->scenario_tag, "NONE", sizeof(vap->scenario_tag) - 1);
    vap->contextual_fog_mod  = 0.2f;

    /* Pillar 7 — PHOTOMETRIC */
    /* Primary: Aurphyx Violet  #7B14C8 */
    vap->photometric.primary_hex[0] = 0.482f;
    vap->photometric.primary_hex[1] = 0.078f;
    vap->photometric.primary_hex[2] = 0.784f;
    /* Secondary: Bliss Gold  #FFD700 */
    vap->photometric.secondary_hex[0] = 1.000f;
    vap->photometric.secondary_hex[1] = 0.843f;
    vap->photometric.secondary_hex[2] = 0.000f;
    vap->photometric.palette_temp       = 0.5f;
    vap->photometric.brightness_floor   = 0.05f;
    vap->photometric.brightness_ceiling = 1.00f;
    vap->photometric.strobe_threshold   = 1.00f;  /* 1.0 = disabled */
    vap->photometric.fade_mode          = 1;       /* SMOOTH */
    vap->photometric.fade_rate          = 0.30f;
    vap->photometric.fog_density        = 0.10f;
    vap->photometric.laser_compatible   = 0;
    vap->photometric.visual_noise_mode  = 0;       /* CLEAN */

    /* Pillar 8 — KINETIC */
    vap->entrainment_factor = 50.0f;
    vap->met_score          =  3.0f;
    strncpy(vap->target_hr_zone, "90-110", sizeof(vap->target_hr_zone) - 1);
    vap->motor_drive        =  0.5f;
    vap->motor_sway         =  0.5f;
    vap->head_nod           =  0.5f;

    /* Pillar 9 — GENEALOGICAL */
    vap->timelessness_score = 0.5f;
    vap->authenticity_ratio = 0.5f;
    vap->viral_velocity     = 0.0f;

    /* Runtime bookkeeping */
    vap->phase_time  = 0.0f;
    vap->frame_count = 0;
    vap->vap_loaded  = 0;
}

void vap_runtime_update_dsp(vap_runtime_t *vap,
                              const float   *fft_mag,
                              int            fft_size,
                              int            sample_rate,
                              float          dt)
{
    if (!vap || !fft_mag || fft_size < 4 || sample_rate < 8000) return;

    int n = fft_size / 2 + 1;

    vap->phase_time  += dt;
    vap->frame_count += 1;

    /* Pillar 3 — TIMBRAL */
    float centroid   = compute_centroid(fft_mag, n, sample_rate);
    float saturation = compute_saturation(fft_mag, n);
    float alpha_120  = clampf(dt / 0.12f, 0.0f, 1.0f);

    vap->spectral_centroid_hz = ema(vap->spectral_centroid_hz, centroid,   alpha_120);
    vap->saturation_index     = ema(vap->saturation_index,     saturation, alpha_120);

    /* Pillar 1 — STRUCTURAL */
    float synco = compute_syncopation(fft_mag, n,
                                       vap->bpm_raw, sample_rate, dt);
    vap->syncopation_index = ema(vap->syncopation_index,
                                  synco,
                                  clampf(dt / 0.20f, 0.0f, 1.0f));
    vap->bpm_perceived = ema(vap->bpm_perceived,
                              vap->bpm_raw,
                              clampf(dt / 2.0f, 0.0f, 1.0f));

    /* Pillar 5 — AFFECTIVE (live arousal) */
    float rms_sq = 0.0f;
    for (int i = 0; i < n; ++i) rms_sq += fft_mag[i] * fft_mag[i];
    float rms = sqrtf(rms_sq / (float)n);

    float bpm_norm   = clampf(vap->bpm_raw / 180.0f,  0.0f, 1.0f);
    float cent_norm  = clampf(centroid      / 5000.0f, 0.0f, 1.0f);
    float arousal_t  = clampf(rms * 0.5f + bpm_norm * 0.3f + cent_norm * 0.2f,
                               0.0f, 1.0f);
    vap->affective.arousal = ema(vap->affective.arousal, arousal_t, alpha_120);

    /* Pillar 7.1 — CHROMATIC BAND ENERGIES */
    float fresh[4];
    compute_band_energies(fft_mag, n, sample_rate, fresh);
    float alpha_band = clampf(dt / 0.08f, 0.0f, 1.0f);
    for (int b = 0; b < 4; ++b) {
        vap->chroma_energy[b] = ema(vap->chroma_energy_smooth[b],
                                     fresh[b], alpha_band);
        vap->chroma_energy_smooth[b] = vap->chroma_energy[b];
    }
}

int vap_runtime_load_json(vap_runtime_t *vap, const char *json_src)
{
    if (!vap || !json_src || !json_src[0]) return VAP_LOAD_ERR_CORRUPT;
    return vap_loader_parse_json(vap, json_src);
}
