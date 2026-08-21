import { BLOOM_FRAG, VIBE_FRAG, VIBE_VERT } from "./orb-shaders";
import type { OrbUniforms } from "./orb-uniforms";

function compile(gl: WebGL2RenderingContext, type: number, src: string): WebGLShader | null {
  const sh = gl.createShader(type);
  if (!sh) return null;
  gl.shaderSource(sh, src);
  gl.compileShader(sh);
  if (!gl.getShaderParameter(sh, gl.COMPILE_STATUS)) {
    console.warn("[orb] shader compile", gl.getShaderInfoLog(sh));
    gl.deleteShader(sh);
    return null;
  }
  return sh;
}

function link(
  gl: WebGL2RenderingContext,
  vertSrc: string,
  fragSrc: string,
): WebGLProgram | null {
  const vs = compile(gl, gl.VERTEX_SHADER, vertSrc);
  const fs = compile(gl, gl.FRAGMENT_SHADER, fragSrc);
  if (!vs || !fs) return null;
  const prog = gl.createProgram();
  if (!prog) return null;
  gl.attachShader(prog, vs);
  gl.attachShader(prog, fs);
  gl.bindAttribLocation(prog, 0, "a_pos");
  gl.linkProgram(prog);
  gl.deleteShader(vs);
  gl.deleteShader(fs);
  if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
    console.warn("[orb] program link", gl.getProgramInfoLog(prog));
    gl.deleteProgram(prog);
    return null;
  }
  return prog;
}

type LocMap = Record<string, WebGLUniformLocation | null>;

function cache(gl: WebGL2RenderingContext, prog: WebGLProgram, names: string[]): LocMap {
  const out: LocMap = {};
  for (const n of names) out[n] = gl.getUniformLocation(prog, n);
  return out;
}

const VIBE_UNIFORMS = [
  "u_time",
  "u_resolution",
  "u_centroid",
  "u_saturation",
  "u_syncopation",
  "u_bpm_norm",
  "u_groove",
  "u_dissonance",
  "u_valence",
  "u_arousal",
  "u_scenario_fog",
  "u_primary_rgb",
  "u_secondary_rgb",
  "u_brightness_floor",
  "u_brightness_ceiling",
  "u_strobe_trigger",
  "u_fog_density",
  "u_visual_noise",
  "u_chrom_energy",
  "u_chrom_energy[0]",
  "u_entrainment",
];

/**
 * Two-pass TSLCA renderer: vibe.frag → FBO → bloom (arousal × brightness_ceiling).
 * Clear color is the near-black void from the VLC plugin.
 */
export class OrbWebGL {
  private gl: WebGL2RenderingContext;
  private vibe: WebGLProgram;
  private bloom: WebGLProgram | null;
  private vibeLoc: LocMap;
  private bloomLoc: LocMap;
  private vao: WebGLVertexArrayObject;
  private fbo: WebGLFramebuffer | null = null;
  private colorTex: WebGLTexture | null = null;
  private fboW = 0;
  private fboH = 0;
  private hasBloom = false;

  constructor(gl: WebGL2RenderingContext, vibe: WebGLProgram, bloom: WebGLProgram | null) {
    this.gl = gl;
    this.vibe = vibe;
    this.bloom = bloom;
    this.vibeLoc = cache(gl, vibe, VIBE_UNIFORMS);
    this.bloomLoc = bloom
      ? cache(gl, bloom, ["u_scene", "u_resolution", "u_bloom_strength", "u_fade_amount"])
      : {};
    const vao = gl.createVertexArray();
    const buf = gl.createBuffer();
    if (!vao || !buf) throw new Error("WebGL buffer failed");
    gl.bindVertexArray(vao);
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0);
    gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
    gl.bindVertexArray(null);
    this.vao = vao;
    this.hasBloom = !!bloom;
  }

  static create(canvas: HTMLCanvasElement): OrbWebGL | null {
    const gl = canvas.getContext("webgl2", {
      alpha: false,
      antialias: false,
      depth: false,
      stencil: false,
      powerPreference: "high-performance",
    });
    if (!gl) return null;
    const vibe = link(gl, VIBE_VERT, VIBE_FRAG);
    if (!vibe) return null;
    const bloom = link(gl, VIBE_VERT, BLOOM_FRAG);
    try {
      return new OrbWebGL(gl, vibe, bloom);
    } catch {
      return null;
    }
  }

  private ensureFbo(w: number, h: number) {
    const gl = this.gl;
    if (!this.hasBloom || !this.bloom) return;
    if (this.fbo && this.fboW === w && this.fboH === h) return;
    if (this.colorTex) gl.deleteTexture(this.colorTex);
    if (this.fbo) gl.deleteFramebuffer(this.fbo);
    const tex = gl.createTexture();
    const fbo = gl.createFramebuffer();
    if (!tex || !fbo) {
      this.hasBloom = false;
      return;
    }
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.bindFramebuffer(gl.FRAMEBUFFER, fbo);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, tex, 0);
    const ok = gl.checkFramebufferStatus(gl.FRAMEBUFFER) === gl.FRAMEBUFFER_COMPLETE;
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    if (!ok) {
      this.hasBloom = false;
      return;
    }
    this.colorTex = tex;
    this.fbo = fbo;
    this.fboW = w;
    this.fboH = h;
  }

  private uploadVibe(u: OrbUniforms) {
    const gl = this.gl;
    const L = this.vibeLoc;
    gl.uniform1f(L.u_time, u.time);
    gl.uniform2f(L.u_resolution, u.resolution[0], u.resolution[1]);
    gl.uniform1f(L.u_centroid, u.centroid);
    gl.uniform1f(L.u_saturation, u.saturation);
    gl.uniform1f(L.u_syncopation, u.syncopation);
    gl.uniform1f(L.u_bpm_norm, u.bpmNorm);
    gl.uniform1f(L.u_groove, u.groove);
    gl.uniform1f(L.u_dissonance, u.dissonance);
    gl.uniform1f(L.u_valence, u.valence);
    gl.uniform1f(L.u_arousal, u.arousal);
    gl.uniform1f(L.u_scenario_fog, u.scenarioFog);
    gl.uniform3f(L.u_primary_rgb, u.primaryRgb[0], u.primaryRgb[1], u.primaryRgb[2]);
    gl.uniform3f(L.u_secondary_rgb, u.secondaryRgb[0], u.secondaryRgb[1], u.secondaryRgb[2]);
    gl.uniform1f(L.u_brightness_floor, u.brightnessFloor);
    gl.uniform1f(L.u_brightness_ceiling, u.brightnessCeiling);
    gl.uniform1f(L.u_strobe_trigger, u.strobeTrigger);
    gl.uniform1f(L.u_fog_density, u.fogDensity);
    gl.uniform1f(L.u_visual_noise, u.visualNoise);
    const chromLoc = L.u_chrom_energy ?? L["u_chrom_energy[0]"];
    if (chromLoc) gl.uniform1fv(chromLoc, u.chromEnergy);
    gl.uniform1f(L.u_entrainment, u.entrainment);
  }

  draw(u: OrbUniforms) {
    const gl = this.gl;
    const w = Math.max(1, Math.floor(u.resolution[0]));
    const h = Math.max(1, Math.floor(u.resolution[1]));
    gl.viewport(0, 0, w, h);
    gl.clearColor(0.027, 0.023, 0.047, 1);
    this.ensureFbo(w, h);

    gl.bindVertexArray(this.vao);

    if (this.hasBloom && this.fbo && this.bloom && this.colorTex) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, this.fbo);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.useProgram(this.vibe);
      this.uploadVibe(u);
      gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.useProgram(this.bloom);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, this.colorTex);
      gl.uniform1i(this.bloomLoc.u_scene, 0);
      gl.uniform2f(this.bloomLoc.u_resolution, w, h);
      gl.uniform1f(this.bloomLoc.u_bloom_strength, u.bloomStrength);
      gl.uniform1f(this.bloomLoc.u_fade_amount, u.fadeAmount);
      gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
    } else {
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.useProgram(this.vibe);
      this.uploadVibe(u);
      gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
    }
    gl.bindVertexArray(null);
  }

  destroy() {
    const gl = this.gl;
    if (this.colorTex) gl.deleteTexture(this.colorTex);
    if (this.fbo) gl.deleteFramebuffer(this.fbo);
    gl.deleteProgram(this.vibe);
    if (this.bloom) gl.deleteProgram(this.bloom);
  }
}
