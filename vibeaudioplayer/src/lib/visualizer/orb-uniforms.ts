import { hexToRgb } from "@/lib/utils";
import type { FrameSample } from "@/lib/audio/engine";
import type { VaspProfile, VisualMapping, VisualSettings } from "@/lib/vasp/types";

/** Live + loaded uniforms that drive vibe.frag + post_bloom.frag. */
export type OrbUniforms = {
  time: number;
  resolution: [number, number];
  centroid: number;
  saturation: number;
  syncopation: number;
  bpmNorm: number;
  groove: number;
  dissonance: number;
  valence: number;
  arousal: number;
  scenarioFog: number;
  primaryRgb: [number, number, number];
  secondaryRgb: [number, number, number];
  brightnessFloor: number;
  brightnessCeiling: number;
  strobeTrigger: number;
  fogDensity: number;
  visualNoise: number;
  chromEnergy: [number, number, number, number];
  entrainment: number;
  bloomStrength: number;
  fadeAmount: number;
};

function clamp01(n: number) {
  return n < 0 ? 0 : n > 1 ? 1 : n;
}

function rgb01(hex: string): [number, number, number] {
  const { r, g, b } = hexToRgb(hex);
  return [r / 255, g / 255, b / 255];
}

function parseValence(text: string): number {
  const t = text.toLowerCase();
  if (t.includes("despair") || t.includes("negative")) return -0.72;
  if (t.includes("low-to-neutral") || t.includes("low to neutral")) return -0.22;
  if (t.includes("neutral-high") || t.includes("high valence")) return 0.48;
  if (t.includes("neutral-positive") || t.includes("positive")) return 0.28;
  if (t.includes("euphor")) return 0.82;
  if (t.includes("low")) return -0.4;
  return 0;
}

function parseArousal(text: string): number {
  const t = text.toLowerCase();
  if (t.includes("very high")) return 0.94;
  if (t.includes("high")) return 0.82;
  if (t.includes("mid-low") || t.includes("mid low")) return 0.34;
  if (t.includes("low")) return 0.22;
  if (t.includes("mid")) return 0.5;
  return 0.55;
}

function parseGroove(text: string): number {
  const t = text.toLowerCase();
  if (t.includes("swing") || t.includes("human")) return 0.82;
  if (t.includes("machine") || t.includes("four-on") || t.includes("lock")) return 0.08;
  if (t.includes("orbit") || t.includes("slow")) return 0.42;
  return 0.28;
}

function parseSyncopation(text: string): number {
  const t = text.toLowerCase();
  if (t.includes("jazz") || t.includes("poly") || t.includes("math")) return 0.78;
  if (t.includes("machine") || t.includes("four-on")) return 0.12;
  if (t.includes("high-energy") || t.includes("driving")) return 0.28;
  if (t.includes("orbit") || t.includes("sparse")) return 0.22;
  return 0.2;
}

function parseDissonance(text: string): number {
  const t = text.toLowerCase();
  if (t.includes("high") || t.includes("bright tension")) return 0.62;
  if (t.includes("moderate")) return 0.38;
  if (t.includes("low")) return 0.12;
  return 0.22;
}

function parseEntrainment(text: string, met: number | null): number {
  const t = text.toLowerCase();
  if (t.includes("body-lock") || t.includes("body lock")) return 86;
  if (t.includes("strong")) return 76;
  if (t.includes("breath")) return 38;
  if (met != null) return Math.min(100, 20 + met * 9);
  return 55;
}

function parseFog(profile: VaspProfile): number {
  const scene = (profile.PILLARS.CONTEXTUAL.scenario.value ?? "").toLowerCase();
  const atmo = (profile.PILLARS.CONTEXTUAL.atmosphere.value ?? "").toLowerCase();
  if (atmo.includes("rain") || atmo.includes("wet") || scene.includes("drive") || scene.includes("rain")) {
    return 0.58;
  }
  if (scene.includes("space") || atmo.includes("vacuum") || atmo.includes("star")) return 0.32;
  return 0.24;
}

/**
 * Map VASP 3.69 pillars + live DSP sample onto the VLC shader uniform block.
 * Phase I fields (centroid, chrom, saturation) come from the analyser.
 * Phase II / III fields come from the loaded profile, mixed with live energy.
 */
export function buildOrbUniforms(
  sample: FrameSample,
  map: VisualMapping,
  settings: VisualSettings,
  profile: VaspProfile,
  width: number,
  height: number,
): OrbUniforms {
  const p = profile.PILLARS;
  const motion = settings.reducedMotion ? 0.18 : settings.motionIntensity;
  const color = settings.colorIntensity;
  const valence = parseValence(p.AFFECTIVE.valence.value ?? "");
  const profileArousal = parseArousal(p.AFFECTIVE.arousal.value ?? "");
  const liveArousal = clamp01(profileArousal * 0.55 + sample.energy * 0.45);
  const arousal = settings.beatPulse ? clamp01(liveArousal + sample.beatPulse * 0.12) : liveArousal;
  const groove = parseGroove(p.STRUCTURAL.groove.value ?? "");
  const bpm = map.bpm || 120;
  const floor = 0.04;
  const ceiling = clamp01((p.PHOTOMETRIC.brightness.value ?? map.brightness) * (0.7 + color * 0.5));
  const fog = parseFog(profile);
  const noiseMode = map.grain > 0.14 && !settings.reducedMotion ? 0.7 : map.grain * 0.35;
  const chrom: [number, number, number, number] = [
    clamp01(sample.chromEnergy[0] * (0.65 + color * 0.5)),
    clamp01(sample.chromEnergy[1] * (0.65 + color * 0.5)),
    clamp01(sample.chromEnergy[2] * (0.65 + color * 0.5)),
    clamp01(sample.chromEnergy[3] * (0.65 + color * 0.5)),
  ];

  let fadeAmount = 1;
  if (settings.beatPulse && sample.beatPulse > 0.92 && map.pulseStrength > 0.9) {
    fadeAmount = 1 + sample.beatPulse * 0.18;
  }

  return {
    time: sample.timeSec * (0.35 + motion * 0.9),
    resolution: [Math.max(1, width), Math.max(1, height)],
    centroid: sample.centroidHz,
    saturation: clamp01(sample.saturation * (0.4 + map.edgeSharpness * 0.6)),
    syncopation: parseSyncopation(p.STRUCTURAL.groove.value ?? "") + sample.beatPulse * 0.08,
    bpmNorm: Math.min(1, bpm / 180) * (0.35 + motion * 0.65),
    groove,
    dissonance: parseDissonance(p.TONAL.dissonance.value ?? ""),
    valence,
    arousal,
    scenarioFog: fog,
    primaryRgb: rgb01(map.primary),
    secondaryRgb: rgb01(map.secondary),
    brightnessFloor: floor,
    brightnessCeiling: ceiling,
    strobeTrigger: 0.82,
    fogDensity: fog,
    visualNoise: settings.reducedMotion ? 0 : noiseMode,
    chromEnergy: chrom,
    entrainment: parseEntrainment(p.KINETIC.entrainment.value ?? "", p.KINETIC.metScore.value),
    bloomStrength: arousal * ceiling * 1.5 * color,
    fadeAmount,
  };
}
