/** GLSL ES 300 ports of vibe-audio-visualizer shaders/vibe.vert + vibe.frag + post_bloom.frag */

export const VIBE_VERT = `#version 300 es
in vec2 a_pos;
out vec2 v_uv;
void main() {
  v_uv = a_pos * 0.5 + 0.5;
  gl_Position = vec4(a_pos, 0.0, 1.0);
}
`;

export const VIBE_FRAG = `#version 300 es
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

/* Phase III — Kinetic */
uniform float u_entrainment;

out vec4 fragColor;

#define PI 3.14159265358979323846

float chladni(vec2 p, float m, float n) {
  return cos(m * PI * p.x) * cos(n * PI * p.y)
    - cos(n * PI * p.x) * cos(m * PI * p.y);
}

float hash(vec2 p) {
  return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
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

  float bright_factor = smoothstep(200.0, 2000.0, u_centroid);

  float sat_rings = 0.0;
  for (int i = 1; i <= 5; i++) {
    float ring_r = 0.15 * float(i) * (1.0 + u_saturation * 0.4);
    sat_rings += smoothstep(0.015, 0.0, abs(r - ring_r))
      * u_saturation * (1.0 / float(i));
  }

  float tension_warp = u_dissonance * 0.3 * sin(theta * 7.0 + u_time * 2.0);
  vec2 uv_warped = uv * (1.0 + tension_warp);

  float nodal = smoothstep(0.06, 0.0, abs(chladni(uv_warped * 0.8, m_node, n_node)));

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
  vec3 out_col = final_color * field;

  /* Central orb body — photometric core sitting in the cymatic field */
  float core = smoothstep(0.30, 0.0, r);
  float rim = smoothstep(0.34, 0.22, r) * smoothstep(0.16, 0.24, r);
  out_col += track_color * core * (0.42 + u_arousal * 0.38 + u_chrom_energy[1] * 0.22);
  out_col += mix(u_secondary_rgb, vec3(1.0), 0.35) * rim * (0.55 + u_chrom_energy[2] * 0.4);
  out_col += vec3(1.0) * smoothstep(0.08, 0.0, r) * (0.18 + bright_factor * 0.2);

  /* Nine-pillar node ring */
  for (int i = 0; i < 9; i++) {
    float a = float(i) / 9.0 * 6.2831853 - 1.5707963 + u_time * 0.04;
    vec2 np = vec2(cos(a), sin(a)) * (0.58 + u_arousal * 0.04);
    float nd = length(uv - np);
    float band = u_chrom_energy[i - (i / 4) * 4];
    vec3 nodeCol = mix(u_secondary_rgb, u_primary_rgb, float(i) / 8.0);
    out_col += nodeCol * smoothstep(0.038, 0.0, nd) * (0.32 + band * 0.55);
  }

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

  float body_lock = step(70.0, u_entrainment) * smoothstep(0.1, 0.0, r) * u_arousal;
  out_col += body_lock * u_primary_rgb * 0.4;

  /* Keep a near-black void outside the field */
  out_col += vec3(0.02, 0.015, 0.04) * (1.0 - smoothstep(0.0, 1.6, r)) * 0.15;

  float unused_strobe = u_strobe_trigger * 0.0;
  out_col += vec3(unused_strobe);

  fragColor = vec4(out_col, 1.0);
}
`;

export const BLOOM_FRAG = `#version 300 es
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
  vec2 px = 1.0 / u_resolution;
  vec3 scene = texture(u_scene, v_uv).rgb;
  float luma = dot(scene, vec3(0.299, 0.587, 0.114));
  vec3 bloom = blur9(u_scene, v_uv, px * 3.0) * u_bloom_strength;
  vec3 composed = scene + bloom * 0.6;
  composed = mix(composed, vec3(1.0), max(u_fade_amount - 1.0, 0.0));
  composed = mix(vec3(0.0), composed, min(u_fade_amount, 1.0));
  fragColor = vec4(clamp(composed, 0.0, 1.0), 1.0);
}
`;
