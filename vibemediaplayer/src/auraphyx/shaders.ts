/**
 * Auraphyx shaders — GLSL ES 300 (WebGL2). Mechanically translated from the
 * verified GLSL 330 core sources in `vap-claude/vibe-visualizer/shaders/`,
 * which are themselves a byte-exact-logic port of the original C visualizer
 * (`visualizer/shaders/vibe.frag`) plus the Auraphyx TSL lattice extension.
 *
 * Translation from 330 core -> 300 es: `#version` line, add
 * `precision highp float;` to fragment shaders. Everything else — uniform
 * names, chladni(), the lattice math, the bloom blur — is unchanged.
 *
 * SACRED: chladni() below must not have a single character changed.
 */

export const VERT_SRC = /* glsl */ `#version 300 es
in vec2 a_pos;
out vec2 v_uv;

void main() {
    v_uv = a_pos * 0.5 + 0.5;
    gl_Position = vec4(a_pos, 0.0, 1.0);
}
`;

export const SCENE_FRAG_SRC = /* glsl */ `#version 300 es
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;

/* Phase I — DSP live */
uniform float u_centroid;
uniform float u_saturation;
uniform float u_syncopation;
uniform float u_bpm_norm;
uniform float u_groove;
uniform float u_dissonance;

/* Phase II — ML / loaded */
uniform float u_valence;
uniform float u_arousal;
uniform float u_scenario_fog;

/* Phase III — Photometric */
uniform vec3 u_primary_rgb;
uniform vec3 u_secondary_rgb;
uniform float u_brightness_floor;
uniform float u_brightness_ceiling;
uniform float u_strobe_trigger;
uniform float u_fog_density;
uniform float u_visual_noise;
uniform float u_chrom_energy[4];
/* [0] Sub-Bass 40-60Hz -> Red  [1] Low-Mid 60-250Hz -> Amber
   [2] Mids 250Hz-2kHz  -> Teal [3] Highs 2kHz+      -> Blue */

uniform float u_entrainment;

/* Phase 7 — Auraphyx: TSL (Three-Squared-Lattice) extension */
uniform float u_tsl_x;
uniform float u_tsl_y;
uniform float u_tsl_z;
uniform float u_phase_align;
uniform float u_lattice_rot;
uniform float u_auraphyx_mode;

out vec4 fragColor;

#define PI 3.14159265358979323846

/* Chladni nodal pattern — SACRED, not one character changes. */
float chladni(vec2 p, float m, float n) {
    return cos(m * PI * p.x) * cos(n * PI * p.y)
        - cos(n * PI * p.x) * cos(m * PI * p.y);
}

float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

/* Auraphyx TSL lattice field, layered on top of the Chladni base geometry */
float auraphyx_field(vec2 uv, float tsl_x, float tsl_y, float tsl_z,
                      float phase_align, float lattice_rot) {
    float c = cos(lattice_rot);
    float s = sin(lattice_rot);
    vec2 ruv = vec2(uv.x * c - uv.y * s, uv.x * s + uv.y * c);

    float hex = cos(ruv.x * 6.0 * tsl_x) + cos(ruv.y * 6.0 * tsl_x)
              + cos((ruv.x + ruv.y) * 6.0 * tsl_x);
    hex = smoothstep(1.8, 2.0, hex) * tsl_x;

    float wave = sin(ruv.x * 12.0 * tsl_y + u_time * 2.0)
               * sin(ruv.y * 12.0 * tsl_y + u_time * 1.7) * tsl_y;

    float particles = hash(ruv * 40.0 + u_time * 0.3) * tsl_z;
    particles *= smoothstep(0.85, 1.0, hash(ruv * 80.0));

    float fracture = (1.0 - abs(phase_align)) * 0.3;
    hex += hash(ruv * 20.0 + u_time) * fracture;

    return hex + wave * 0.4 + particles * 0.2;
}

void main() {
    vec2 uv = (gl_FragCoord.xy / u_resolution) * 2.0 - 1.0;
    uv.x *= u_resolution.x / u_resolution.y;

    float r = length(uv);
    float theta = atan(uv.y, uv.x);

    float m_node = 2.0 + u_syncopation * 6.0;
    float n_node = m_node + 1.0 + u_groove * 2.0;

    float ring_width = mix(0.02, 0.06, u_groove);
    float pulse_ring = smoothstep(ring_width, 0.0,
            abs(r - (0.35 + u_arousal * 0.25 +
                        sin(u_time * u_bpm_norm * 6.28) * 0.05)));

    float sat_rings = 0.0;
    for (int i = 1; i <= 5; i++) {
        float ring_r = 0.15 * float(i) * (1.0 + u_saturation * 0.4);
        sat_rings += smoothstep(0.015, 0.0, abs(r - ring_r))
                * u_saturation * (1.0 / float(i));
    }

    float tension_warp = u_dissonance * 0.3 * sin(theta * 7.0 + u_time * 2.0);
    vec2 uv_warped = uv * (1.0 + tension_warp);

    float nodal = smoothstep(0.06, 0.0,
            abs(chladni(uv_warped * 0.8, m_node, n_node)));

    vec3 col_sub = vec3(0.85, 0.05, 0.05) * u_chrom_energy[0];
    vec3 col_low = vec3(1.00, 0.55, 0.00) * u_chrom_energy[1];
    vec3 col_mid = vec3(0.10, 0.75, 0.55) * u_chrom_energy[2];
    vec3 col_high = vec3(0.30, 0.15, 0.95) * u_chrom_energy[3];
    vec3 spectral_color = col_sub + col_low + col_mid + col_high;

    float valence_norm = u_valence * 0.5 + 0.5;
    vec3 track_color = mix(u_secondary_rgb, u_primary_rgb, valence_norm);
    vec3 final_color = mix(spectral_color, track_color, 0.4);

    vec3 warm = vec3(1.0, 0.8, 0.2);
    vec3 cold = vec3(0.1, 0.2, 0.6);
    vec3 atmosphere = mix(cold, warm, valence_norm) * u_arousal * 0.3;
    final_color += atmosphere;

    float field = nodal + pulse_ring + sat_rings;

    if (u_auraphyx_mode > 0.5) {
        float lattice = auraphyx_field(uv, u_tsl_x, u_tsl_y, u_tsl_z, u_phase_align, u_lattice_rot);
        field += lattice * 0.6;
    }

    vec3 out_col = final_color * field;

    if (u_visual_noise > 0.5) {
        float noise = hash(uv + fract(u_time * 0.1));
        out_col += noise * u_visual_noise * 0.15;
        float glitch = step(0.98, hash(vec2(floor(uv.y * 20.0), u_time)));
        out_col.r += glitch * u_visual_noise * 0.3;
    }

    float fog = u_fog_density * u_scenario_fog;
    out_col = mix(out_col, vec3(0.02, 0.02, 0.06) * fog, fog * 0.5);

    float luma = dot(out_col, vec3(0.299, 0.587, 0.114));
    luma = clamp(luma, u_brightness_floor, u_brightness_ceiling);
    out_col = out_col * (luma / max(dot(out_col, vec3(0.299, 0.587, 0.114)), 1e-5));

    float body_lock = step(70.0, u_entrainment) *
            smoothstep(0.1, 0.0, r) * u_arousal;
    out_col += body_lock * u_primary_rgb * 0.4;

    fragColor = vec4(out_col, 1.0);
}
`;

export const BLOOM_FRAG_SRC = /* glsl */ `#version 300 es
precision highp float;

uniform sampler2D u_scene;
uniform vec2 u_resolution;
uniform float u_bloom_strength;
uniform float u_fade_amount;

in vec2 v_uv;
out vec4 fragColor;

vec3 blur9(sampler2D tex, vec2 uv, vec2 px) {
    vec3 c = vec3(0.0);
    c += texture(tex, uv + vec2(-2.0,  0.0) * px).rgb * 0.0625;
    c += texture(tex, uv + vec2(-1.0,  0.0) * px).rgb * 0.125;
    c += texture(tex, uv + vec2( 0.0,  0.0) * px).rgb * 0.25;
    c += texture(tex, uv + vec2( 1.0,  0.0) * px).rgb * 0.125;
    c += texture(tex, uv + vec2( 2.0,  0.0) * px).rgb * 0.0625;
    c += texture(tex, uv + vec2( 0.0, -2.0) * px).rgb * 0.0625;
    c += texture(tex, uv + vec2( 0.0, -1.0) * px).rgb * 0.125;
    c += texture(tex, uv + vec2( 0.0,  1.0) * px).rgb * 0.125;
    c += texture(tex, uv + vec2( 0.0,  2.0) * px).rgb * 0.0625;
    return c;
}

void main() {
    vec2 px    = 1.0 / u_resolution;
    vec3 scene = texture(u_scene, v_uv).rgb;

    vec3 bloom  = blur9(u_scene, v_uv, px * 3.0) * u_bloom_strength;

    vec3 composed = scene + bloom * 0.6;
    composed = mix(composed, vec3(1.0), max(u_fade_amount - 1.0, 0.0));
    composed = mix(vec3(0.0), composed, min(u_fade_amount, 1.0));

    fragColor = vec4(clamp(composed, 0.0, 1.0), 1.0);
}
`;
