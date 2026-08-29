import { BLOOM_FRAG_SRC, SCENE_FRAG_SRC, VERT_SRC } from './shaders';
import type { ShaderUniforms } from './types';

// Two triangles covering the full [-1,1] clip-space quad.
const QUAD = new Float32Array([
  -1, -1, 1, -1, 1, 1,
  -1, -1, 1, 1, -1, 1,
]);

function compileShader(gl: WebGL2RenderingContext, type: number, source: string): WebGLShader {
  const shader = gl.createShader(type);
  if (!shader) throw new Error('failed to create shader');
  gl.shaderSource(shader, source);
  gl.compileShader(shader);
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    const log = gl.getShaderInfoLog(shader);
    gl.deleteShader(shader);
    throw new Error(`shader compile failed: ${log}`);
  }
  return shader;
}

function linkProgram(gl: WebGL2RenderingContext, vertSrc: string, fragSrc: string): WebGLProgram {
  const vert = compileShader(gl, gl.VERTEX_SHADER, vertSrc);
  const frag = compileShader(gl, gl.FRAGMENT_SHADER, fragSrc);
  const program = gl.createProgram();
  if (!program) throw new Error('failed to create program');
  gl.attachShader(program, vert);
  gl.attachShader(program, frag);
  gl.linkProgram(program);
  gl.deleteShader(vert);
  gl.deleteShader(frag);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    const log = gl.getProgramInfoLog(program);
    gl.deleteProgram(program);
    throw new Error(`program link failed: ${log}`);
  }
  return program;
}

type SceneUniformLocs = {
  time: WebGLUniformLocation | null;
  resolution: WebGLUniformLocation | null;
  centroid: WebGLUniformLocation | null;
  saturation: WebGLUniformLocation | null;
  syncopation: WebGLUniformLocation | null;
  bpmNorm: WebGLUniformLocation | null;
  groove: WebGLUniformLocation | null;
  dissonance: WebGLUniformLocation | null;
  valence: WebGLUniformLocation | null;
  arousal: WebGLUniformLocation | null;
  scenarioFog: WebGLUniformLocation | null;
  primaryRgb: WebGLUniformLocation | null;
  secondaryRgb: WebGLUniformLocation | null;
  brightnessFloor: WebGLUniformLocation | null;
  brightnessCeiling: WebGLUniformLocation | null;
  strobeTrigger: WebGLUniformLocation | null;
  fogDensity: WebGLUniformLocation | null;
  visualNoise: WebGLUniformLocation | null;
  chromEnergy: WebGLUniformLocation | null;
  entrainment: WebGLUniformLocation | null;
  tslX: WebGLUniformLocation | null;
  tslY: WebGLUniformLocation | null;
  tslZ: WebGLUniformLocation | null;
  phaseAlign: WebGLUniformLocation | null;
  latticeRot: WebGLUniformLocation | null;
  auraphyxMode: WebGLUniformLocation | null;
};

type BloomUniformLocs = {
  scene: WebGLUniformLocation | null;
  resolution: WebGLUniformLocation | null;
  bloomStrength: WebGLUniformLocation | null;
  fadeAmount: WebGLUniformLocation | null;
};

/** Two-pass Chladni field (+ Auraphyx lattice) -> bloom composite renderer. */
export class AuraphyxRenderer {
  private gl: WebGL2RenderingContext;
  private sceneProgram: WebGLProgram;
  private bloomProgram: WebGLProgram;
  private sceneU: SceneUniformLocs;
  private bloomU: BloomUniformLocs;
  private quadVao: WebGLVertexArrayObject;
  private fbo: WebGLFramebuffer;
  private sceneTexture: WebGLTexture;
  private width = 1;
  private height = 1;

  bloomEnabled = true;
  visualNoise = false;
  auraphyxEnabled = true;

  constructor(gl: WebGL2RenderingContext, width: number, height: number) {
    this.gl = gl;
    this.sceneProgram = linkProgram(gl, VERT_SRC, SCENE_FRAG_SRC);
    this.bloomProgram = linkProgram(gl, VERT_SRC, BLOOM_FRAG_SRC);
    this.sceneU = this.locateSceneUniforms();
    this.bloomU = this.locateBloomUniforms();
    this.quadVao = this.buildQuad();
    this.sceneTexture = gl.createTexture()!;
    this.fbo = gl.createFramebuffer()!;
    this.resize(width, height);
  }

  private locateSceneUniforms(): SceneUniformLocs {
    const gl = this.gl;
    const p = this.sceneProgram;
    const at = (name: string) => gl.getUniformLocation(p, name);
    return {
      time: at('u_time'),
      resolution: at('u_resolution'),
      centroid: at('u_centroid'),
      saturation: at('u_saturation'),
      syncopation: at('u_syncopation'),
      bpmNorm: at('u_bpm_norm'),
      groove: at('u_groove'),
      dissonance: at('u_dissonance'),
      valence: at('u_valence'),
      arousal: at('u_arousal'),
      scenarioFog: at('u_scenario_fog'),
      primaryRgb: at('u_primary_rgb'),
      secondaryRgb: at('u_secondary_rgb'),
      brightnessFloor: at('u_brightness_floor'),
      brightnessCeiling: at('u_brightness_ceiling'),
      strobeTrigger: at('u_strobe_trigger'),
      fogDensity: at('u_fog_density'),
      visualNoise: at('u_visual_noise'),
      chromEnergy: at('u_chrom_energy[0]'),
      entrainment: at('u_entrainment'),
      tslX: at('u_tsl_x'),
      tslY: at('u_tsl_y'),
      tslZ: at('u_tsl_z'),
      phaseAlign: at('u_phase_align'),
      latticeRot: at('u_lattice_rot'),
      auraphyxMode: at('u_auraphyx_mode'),
    };
  }

  private locateBloomUniforms(): BloomUniformLocs {
    const gl = this.gl;
    const p = this.bloomProgram;
    const at = (name: string) => gl.getUniformLocation(p, name);
    return {
      scene: at('u_scene'),
      resolution: at('u_resolution'),
      bloomStrength: at('u_bloom_strength'),
      fadeAmount: at('u_fade_amount'),
    };
  }

  private buildQuad(): WebGLVertexArrayObject {
    const gl = this.gl;
    const vao = gl.createVertexArray();
    if (!vao) throw new Error('failed to create VAO');
    gl.bindVertexArray(vao);
    const vbo = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
    gl.bufferData(gl.ARRAY_BUFFER, QUAD, gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0);
    gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
    gl.bindVertexArray(null);
    return vao;
  }

  resize(width: number, height: number): void {
    const gl = this.gl;
    this.width = Math.max(1, width | 0);
    this.height = Math.max(1, height | 0);

    gl.bindTexture(gl.TEXTURE_2D, this.sceneTexture);
    gl.texImage2D(
      gl.TEXTURE_2D, 0, gl.RGBA8, this.width, this.height, 0,
      gl.RGBA, gl.UNSIGNED_BYTE, null,
    );
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

    gl.bindFramebuffer(gl.FRAMEBUFFER, this.fbo);
    gl.framebufferTexture2D(
      gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, this.sceneTexture, 0,
    );
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.bindTexture(gl.TEXTURE_2D, null);
  }

  /** `time` is a locally-owned clock (seconds), independent of the backend's
   * own `uniforms.time` — smoother than pacing motion to the ~60Hz emit rate. */
  render(uniforms: ShaderUniforms, time: number): void {
    const gl = this.gl;

    // ---- Pass 1: Chladni + Auraphyx field -> FBO texture ----
    gl.bindFramebuffer(gl.FRAMEBUFFER, this.fbo);
    gl.viewport(0, 0, this.width, this.height);
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(this.sceneProgram);
    const u = this.sceneU;
    gl.uniform1f(u.time, time);
    gl.uniform2f(u.resolution, this.width, this.height);
    gl.uniform1f(u.centroid, uniforms.centroid);
    gl.uniform1f(u.saturation, uniforms.saturation);
    gl.uniform1f(u.syncopation, uniforms.syncopation);
    gl.uniform1f(u.bpmNorm, uniforms.bpm_norm);
    gl.uniform1f(u.groove, uniforms.groove);
    gl.uniform1f(u.dissonance, uniforms.dissonance);
    gl.uniform1f(u.valence, uniforms.valence);
    gl.uniform1f(u.arousal, uniforms.arousal);
    gl.uniform1f(u.scenarioFog, 0.0);
    gl.uniform3fv(u.primaryRgb, uniforms.primary_rgb);
    gl.uniform3fv(u.secondaryRgb, uniforms.secondary_rgb);
    gl.uniform1f(u.brightnessFloor, uniforms.brightness_floor);
    gl.uniform1f(u.brightnessCeiling, uniforms.brightness_ceiling);
    gl.uniform1f(u.strobeTrigger, uniforms.strobe_trigger);
    gl.uniform1f(u.fogDensity, uniforms.fog_density);
    gl.uniform1f(u.visualNoise, this.visualNoise ? 1.0 : 0.0);
    gl.uniform1fv(u.chromEnergy, uniforms.chrom_energy);
    gl.uniform1f(u.entrainment, uniforms.entrainment);
    gl.uniform1f(u.tslX, uniforms.tsl_x);
    gl.uniform1f(u.tslY, uniforms.tsl_y);
    gl.uniform1f(u.tslZ, uniforms.tsl_z);
    gl.uniform1f(u.phaseAlign, uniforms.phase_align);
    gl.uniform1f(u.latticeRot, uniforms.lattice_rot);
    gl.uniform1f(u.auraphyxMode, this.auraphyxEnabled ? 1.0 : 0.0);

    gl.bindVertexArray(this.quadVao);
    gl.drawArrays(gl.TRIANGLES, 0, 6);

    // ---- Pass 2: bloom composite -> screen (default framebuffer) ----
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, this.width, this.height);
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(this.bloomProgram);
    const b = this.bloomU;
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, this.sceneTexture);
    gl.uniform1i(b.scene, 0);
    gl.uniform2f(b.resolution, this.width, this.height);
    gl.uniform1f(b.bloomStrength, this.bloomEnabled ? bloomStrength(uniforms) : 0.0);
    gl.uniform1f(b.fadeAmount, 1.0);

    gl.bindVertexArray(this.quadVao);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
    gl.bindVertexArray(null);
  }

  dispose(): void {
    const gl = this.gl;
    gl.deleteFramebuffer(this.fbo);
    gl.deleteTexture(this.sceneTexture);
    gl.deleteVertexArray(this.quadVao);
    gl.deleteProgram(this.sceneProgram);
    gl.deleteProgram(this.bloomProgram);
  }
}

/** Bloom strength: arousal x brightness_ceiling, matching `VapRuntime::bloom_strength()`. */
function bloomStrength(uniforms: ShaderUniforms): number {
  return uniforms.arousal * uniforms.brightness_ceiling * 1.5;
}
