#version 120
/* ================================================================
   Vibe Audio Visualizer — V.A.P. v3.1 Fragment Shader
   Aurphyx SUXS / rAE

   Uniform sources by VAP Phase:
   Phase I (DSP live)   → u_centroid, u_saturation, u_syncopation
   Phase II (ML loaded) → u_valence, u_arousal, u_scenario_fog
   Phase III (JSON I/O) → u_primary_rgb, u_secondary_rgb,
                          u_brightness_floor, u_brightness_ceiling,
                          u_strobe, u_fade_mode, u_fog_density,
                          u_chrom_energy[4], u_visual_noise
   ================================================================ */

uniform float u_time;
uniform vec2 u_resolution;

/* Phase I — DSP live */
uniform float u_centroid; /* Spectral centroid Hz (Pillar 3) */
uniform float u_saturation; /* THD 0.0-1.0       (Pillar 3)   */
uniform float u_syncopation; /* Off-beat ratio    (Pillar 1)   */
uniform float u_bpm_norm; /* BPM / 180.0       (Pillar 1)   */
uniform float u_groove; /* 0.0=machine 1.0=swing          */
uniform float u_dissonance; /* 0.0-1.0           (Pillar 2)   */

/* Phase II — ML / loaded */
uniform float u_valence; /* -1.0 to +1.0      (Pillar 5)   */
uniform float u_arousal; /*  0.0 to  1.0      (Pillar 5)   */
uniform float u_scenario_fog; /* Contextual haze   (Pillar 6)   */

/* Phase III — Photometric pillar (loaded from .vap.json) */
uniform vec3 u_primary_rgb; /* PRIMARY_HEX       (Pillar 7.1) */
uniform vec3 u_secondary_rgb; /* SECONDARY_HEX     (Pillar 7.1) */
uniform float u_brightness_floor; /* Lumen min         (Pillar 7.2) */
uniform float u_brightness_ceiling; /* Lumen max         (Pillar 7.2) */
uniform float u_strobe_trigger; /* Beat threshold    (Pillar 7.2) */
uniform float u_fog_density; /* Haze              (Pillar 7.3) */
uniform float u_visual_noise; /* 0=Clean 1=Glitch  (Pillar 7.3) */
uniform float u_chrom_energy[4]; /* Per-band energy   (Pillar 7.1) */
/* Band mapping per VAP spec:
   [0] Sub-Bass  40-60Hz  → Red   (~700nm)
   [1] Low-Mid   60-250Hz → Amber (~600nm)
   [2] Mids    250-2kHz   → Teal  (~520nm)
   [3] Highs   2kHz+      → Blue  (~450nm)  */

/* Phase III — Kinetic */
uniform float u_entrainment; /* 0-100             (Pillar 8)   */

#define PI 3.14159265358979323846

/* Chladni nodal pattern — geometry driven by Pillar 1 Structural */
float chladni(vec2 p, float m, float n) {
    return cos(m * PI * p.x) * cos(n * PI * p.y)
        - cos(n * PI * p.x) * cos(m * PI * p.y);
}

/* Hash for visual noise (Pillar 7.3) */
float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

void main() {
    vec2 uv = (gl_FragCoord.xy / u_resolution) * 2.0 - 1.0;
    uv.x *= u_resolution.x / u_resolution.y;

    float r = length(uv);
    float theta = atan(uv.y, uv.x);

    /* ── PILLAR 1: STRUCTURAL → Geometry shape ──────────────────── */
    /* Syncopation drives nodal complexity: low sync=simple 4-node,
           high sync=polyrhythmic chaos (Jazz/Math per spec)            */
    float m_node = 2.0 + u_syncopation * 6.0;
    float n_node = m_node + 1.0 + u_groove * 2.0;

    /* BPM-driven pulse ring — Machine Lock = sharp, Swing = soft  */
    float pulse_sharp = (u_groove < 0.2) ? 1.0 : 0.0;
    float ring_width = mix(0.02, 0.06, u_groove);
    float pulse_ring = smoothstep(ring_width, 0.0,
            abs(r - (0.35 + u_arousal * 0.25 +
                        sin(u_time * u_bpm_norm * 6.28) * 0.05)));

    /* ── PILLAR 3: TIMBRAL → Spectral centroid drives brightness ── */
    /* Dark <200Hz = heavy low glow; Bright >2kHz = high sparkle   */
    float bright_factor = smoothstep(200.0, 2000.0, u_centroid);

    /* Saturation → harmonic shimmer rings (distorted=more rings) */
    float sat_rings = 0.0;
    for (int i = 1; i <= 5; i++) {
        float ring_r = 0.15 * float(i) * (1.0 + u_saturation * 0.4);
        sat_rings += smoothstep(0.015, 0.0, abs(r - ring_r))
                * u_saturation * (1.0 / float(i));
    }

    /* ── PILLAR 2: TONAL → Dissonance warps the nodal field ─────── */
    float tension_warp = u_dissonance * 0.3 * sin(theta * 7.0 + u_time * 2.0);
    vec2 uv_warped = uv * (1.0 + tension_warp);

    /* Core Chladni field */
    float nodal = smoothstep(0.06, 0.0,
            abs(chladni(uv_warped * 0.8, m_node, n_node)));

    /* ── PILLAR 7: PHOTOMETRIC → VAP spec chromatic map ─────────── */
    /* 4 bands per spec: Sub-Bass Red, Low-Mid Amber, Mid Teal, High Blue */
    vec3 col_sub = vec3(0.85, 0.05, 0.05) * u_chrom_energy[0]; /* 700nm */
    vec3 col_low = vec3(1.00, 0.55, 0.00) * u_chrom_energy[1]; /* 600nm */
    vec3 col_mid = vec3(0.10, 0.75, 0.55) * u_chrom_energy[2]; /* 520nm */
    vec3 col_high = vec3(0.30, 0.15, 0.95) * u_chrom_energy[3]; /* 450nm */
    vec3 spectral_color = col_sub + col_low + col_mid + col_high;

    /* Blend with track's static Photometric primary/secondary hex */
    float valence_norm = u_valence * 0.5 + 0.5; /* -1..1 → 0..1   */
    vec3 track_color = mix(u_secondary_rgb, u_primary_rgb, valence_norm);
    vec3 final_color = mix(spectral_color, track_color, 0.4);

    /* ── PILLAR 5: AFFECTIVE → Valence shifts atmosphere ─────────── */
    /* Positive valence: warm gold bloom; Negative: cold blue void  */
    vec3 warm = vec3(1.0, 0.8, 0.2); /* Euphoria gold            */
    vec3 cold = vec3(0.1, 0.2, 0.6); /* Despair blue             */
    vec3 atmosphere = mix(cold, warm, valence_norm) * u_arousal * 0.3;
    final_color += atmosphere;

    /* ── Compose field ──────────────────────────────────────────── */
    float field = nodal + pulse_ring + sat_rings;
    vec3 out_col = final_color * field;

    /* ── PILLAR 7.3: Visual Noise / Glitch mode ──────────────────── */
    if (u_visual_noise > 0.5) {
        float noise = hash(uv + fract(u_time * 0.1));
        out_col += noise * u_visual_noise * 0.15;
        /* Horizontal glitch offset bands */
        float glitch = step(0.98, hash(vec2(floor(uv.y * 20.0), u_time)));
        out_col.r += glitch * u_visual_noise * 0.3;
    }

    /* ── PILLAR 6: CONTEXTUAL → Scenario fog (Night Drive, etc.) ── */
    float fog = u_fog_density * u_scenario_fog;
    out_col = mix(out_col, vec3(0.02, 0.02, 0.06) * fog, fog * 0.5);

    /* ── PILLAR 7.2: Lumen Dynamics — brightness floor/ceiling ───── */
    float luma = dot(out_col, vec3(0.299, 0.587, 0.114));
    luma = clamp(luma, u_brightness_floor, u_brightness_ceiling);
    out_col = out_col * (luma / max(dot(out_col, vec3(0.299, 0.587, 0.114)),
                    1e-5));

    /* ── PILLAR 8: KINETIC → Entrainment Factor body-lock flash ──── */
    /* High entrainment (>70, "Body Lock" per spec) = center flash  */
    float body_lock = step(70.0, u_entrainment) *
            smoothstep(0.1, 0.0, r) * u_arousal;
    out_col += body_lock * u_primary_rgb * 0.4;

    gl_FragColor = vec4(out_col, 1.0);
}
