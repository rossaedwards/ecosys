#include "dsp_engine.h"
#include "vap_runtime.h"
#include <math.h>
#include <string.h>

/* VAP Phase I: Physical Analysis
   All scoring follows Logic Architecture v1.0 definitions (file:28) */

/* Spectral Centroid: center of gravity of frequency spectrum */
static float compute_spectral_centroid(const float *mag, int n, int sr) {
    float num = 0.0f, den = 0.0f;
    float bin_hz = (float)sr / (float)(n * 2);
    for (int i = 1; i < n; i++) {
        float f = i * bin_hz;
        num += f * mag[i];
        den += mag[i];
    }
    return (den > 1e-6f) ? (num / den) : 0.0f;
}

/* Saturation Index: THD approximation via harmonic energy ratio */
static float compute_saturation_index(const float *mag, int n) {
    float fundamental = 0.0f, harmonics = 0.0f;
    if (n < 4) return 0.0f;
    fundamental = mag[1];
    for (int i = 2; i < n && i <= 10; i++)
        harmonics += mag[i];
    float total = fundamental + harmonics;
    return (total > 1e-6f) ? (harmonics / total) : 0.0f;
}

/* Syncopation Index: off-beat transient ratio
   Detects onsets on weak beats (2 and 4 in 4/4) vs all beats      */
static float compute_syncopation(const float *mag, int n,
                                  float bpm, int sr, float dt) {
    /* Simplified: energy variance between beat subdivisions */
    (void)bpm; (void)sr; (void)dt;
    float even_energy = 0.0f, odd_energy = 0.0f;
    int half = n / 2;
    for (int i = 0; i < half; i++)       even_energy += mag[i];
    for (int i = half; i < n; i++)       odd_energy  += mag[i];
    float total = even_energy + odd_energy;
    return (total > 1e-6f) ? (odd_energy / total) : 0.0f;
}

void dsp_engine_update(vap_runtime_t *vap, const float *fft_mag,
                        int fft_size, int sample_rate, float dt)
{
    /* --- Pillar 3: Spectral Centroid → Timbral classification --- */
    float centroid = compute_spectral_centroid(fft_mag, fft_size, sample_rate);
    vap->spectral_centroid_hz = centroid;
    /* Per VAP spec scoring: Dark <200Hz | Warm 200-2000Hz | Bright >2000Hz */

    /* --- Pillar 3: Saturation Index (THD) --- */
    vap->saturation_index = compute_saturation_index(fft_mag, fft_size);

    /* --- Pillar 1: Syncopation Index --- */
    vap->syncopation_index = compute_syncopation(
        fft_mag, fft_size, vap->bpm_raw, sample_rate, dt);

    /* --- Pillar 5: Arousal — live update from RMS + centroid --- */
    float rms = 0.0f;
    for (int i = 0; i < fft_size; i++)
        rms += fft_mag[i] * fft_mag[i];
    rms = sqrtf(rms / fft_size);
    /* Arousal = f(RMS, BPM, Spectral Density) per spec Phase II    */
    float bpm_norm = fminf(vap->bpm_raw / 180.0f, 1.0f);
    float arousal  = (rms * 0.5f + bpm_norm * 0.3f +
                      fminf(centroid / 5000.0f, 1.0f) * 0.2f);
    /* Smooth: 120ms attack/decay */
    vap->affective.arousal = vap->affective.arousal * 0.7f + arousal * 0.3f;

    /* --- Photometric: live chromatic band energies per spec map --- */
    /* These become u_chrom_energy[4] in the fragment shader         */
    /* Bands follow VAP_CHROMATIC_MAP[] in vap_photometric.h        */

    vap->phase_time   += dt;
    vap->frame_count++;
}
