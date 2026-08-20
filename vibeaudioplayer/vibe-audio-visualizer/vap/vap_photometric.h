#ifndef VAP_PHOTOMETRIC_H
#define VAP_PHOTOMETRIC_H

/* V.A.P. v3.1 — Pillar 7: PHOTOMETRIC (The Eye)
   Per VAP Logic Architecture: frequency → wavelength → RGB mapping
   Sub-Bass  40-60Hz   → 700nm Deep Red
   Low-Mid   60-250Hz  → 600nm Orange/Amber
   Mids      250-2kHz  → 520nm Green/Teal
   Highs     2kHz+     → 450nm Blue/Purple/UV              */

#include <stdint.h>

typedef struct {
    /* 7.1 Chromatic Map */
    uint8_t  primary_hex[3];       /* RGB from VAP JSON PRIMARY_HEX   */
    uint8_t  secondary_hex[3];     /* RGB from VAP JSON SECONDARY_HEX */
    float    palette_temp;         /* 0.0=Cool(Blues) 1.0=Warm(Reds)  */

    /* 7.2 Lumen Dynamics */
    float    brightness_floor;     /* 0.0–1.0 minimum intensity       */
    float    brightness_ceiling;   /* 0.0–1.0 maximum intensity       */
    float    strobe_threshold;     /* Beat energy level to fire strobe */
    int      fade_mode;            /* 0=Sharp(On/Off) 1=Linear Fade   */
    float    fade_rate;            /* Seconds for fade transition      */

    /* 7.3 Visual Texture */
    float    fog_density;          /* 0.0–1.0 haze level              */
    int      laser_compatible;     /* 0=No 1=Yes                      */
    int      visual_noise_mode;    /* 0=Clean 1=Glitch/Static         */
} vap_photometric_t;

/* V.A.P. spec wavelength→color table (Pillar 7, Section 7.1) */
typedef struct {
    float freq_low_hz;
    float freq_high_hz;
    float wavelength_nm;
    float r, g, b;                 /* Pre-computed RGB [0.0–1.0]      */
} vap_chromatic_band_t;

static const vap_chromatic_band_t VAP_CHROMATIC_MAP[] = {
    {  40.0f,   60.0f, 700.0f, 0.85f, 0.05f, 0.05f }, /* Deep Red     */
    {  60.0f,  250.0f, 600.0f, 1.00f, 0.55f, 0.00f }, /* Orange/Amber */
    { 250.0f, 2000.0f, 520.0f, 0.10f, 0.75f, 0.55f }, /* Green/Teal   */
    {2000.0f,20000.0f, 450.0f, 0.30f, 0.15f, 0.95f }, /* Blue/UV      */
};
#define VAP_CHROMATIC_BAND_COUNT 4

#endif /* VAP_PHOTOMETRIC_H */
