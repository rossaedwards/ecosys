/** Mirrors `vmp_viz::ShaderUniforms` (Rust) — field names match verbatim
 * (default serde, no renaming), since this is deserialized directly from
 * the `auraphyx-frame` Tauri event payload. */
export type ShaderUniforms = {
  centroid: number;
  saturation: number;
  syncopation: number;
  bpm_norm: number;
  groove: number;
  dissonance: number;
  valence: number;
  arousal: number;
  primary_rgb: [number, number, number];
  secondary_rgb: [number, number, number];
  brightness_floor: number;
  brightness_ceiling: number;
  strobe_trigger: number;
  fog_density: number;
  visual_noise: number;
  chrom_energy: [number, number, number, number];
  entrainment: number;
  time: number;
  tsl_x: number;
  tsl_y: number;
  tsl_z: number;
  phase_align: number;
  lattice_rot: number;
  auraphyx_mode: number;
};

/** Idle-state uniforms rendered before the first live frame arrives, or
 * whenever nothing is playing — a dim, still Chladni field. */
export const IDLE_UNIFORMS: ShaderUniforms = {
  centroid: 800,
  saturation: 0.1,
  syncopation: 0.2,
  bpm_norm: 0.5,
  groove: 0.3,
  dissonance: 0.1,
  valence: 0,
  arousal: 0.15,
  primary_rgb: [0.482, 0.078, 0.784],
  secondary_rgb: [1.0, 0.843, 0.0],
  brightness_floor: 0.03,
  brightness_ceiling: 0.6,
  strobe_trigger: 1.0,
  fog_density: 0.1,
  visual_noise: 0,
  chrom_energy: [0.05, 0.05, 0.05, 0.05],
  entrainment: 0,
  time: 0,
  tsl_x: 0,
  tsl_y: 0,
  tsl_z: 0,
  phase_align: 0,
  lattice_rot: 0,
  auraphyx_mode: 1,
};
