#version 330 core
/* Passthrough vertex shader — mechanical GLSL 330 core translation of
   VERT_SRC embedded in src/gl_renderer.c. Shared by both the Chladni
   field pass (vibe.frag) and the bloom pass (post_bloom.frag). */

in vec2 a_pos;
out vec2 v_uv;

void main() {
    v_uv = a_pos * 0.5 + 0.5;
    gl_Position = vec4(a_pos, 0.0, 1.0);
}
