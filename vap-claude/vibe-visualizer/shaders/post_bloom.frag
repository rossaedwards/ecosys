#version 330 core
/* Two-pass bloom post-process — mechanical GLSL 330 core port of
   BLOOM_FRAG_SRC embedded in src/gl_renderer.c (post_bloom.frag on disk
   was an empty stub in the reference; gl_renderer.c's inline string is
   the actual shader source and the ground truth ported here).

   Changes from the C reference (mechanical only, logic untouched):
     - #version 120 -> #version 330 core
     - varying -> in
     - texture2D() -> texture() (texture2D is unavailable in 330 core)
     - gl_FragColor -> out vec4 fragColor */

uniform sampler2D u_scene;
uniform vec2      u_resolution;
uniform float     u_bloom_strength;  /* arousal * brightness_ceiling */
uniform float     u_fade_amount;     /* current fade lerp value 0-1  */

in vec2 v_uv;
out vec4 fragColor;

/* 9-tap Gaussian blur for bloom extraction */
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

    /* Extract bright regions for bloom (luma threshold 0.6) */
    float luma = dot(scene, vec3(0.299, 0.587, 0.114));
    vec3 bright = (luma > 0.6) ? scene : vec3(0.0);
    vec3 bloom  = blur9(u_scene, v_uv, px * 3.0) * u_bloom_strength;

    /* Fade: lerp toward black for smooth-fade, white for strobe */
    vec3 composed = scene + bloom * 0.6;
    composed = mix(composed, vec3(1.0), max(u_fade_amount - 1.0, 0.0));
    composed = mix(vec3(0.0), composed, min(u_fade_amount, 1.0));

    fragColor = vec4(clamp(composed, 0.0, 1.0), 1.0);
}
