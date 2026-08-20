import { hashHue, rgbToHex, uid } from "@/lib/utils";
import {
  VASP_VERSION,
  type Pillars,
  type VaspProfile,
  type VisualMapping,
} from "./types";

function known<T>(value: T) {
  return { value, status: "known" as const };
}

function pending<T>(value: T) {
  return { value, status: "pending" as const };
}

function unknown<T>(value: T) {
  return { value, status: "unknown" as const };
}

export type DemoSynth = {
  bpm: number;
  rootHz: number;
  minor: boolean;
  drive: number;
  brightness: number;
  hatDensity: number;
};

export type LibraryTrack = {
  id: string;
  kind: "demo" | "file";
  title: string;
  artist: string;
  duration: number | null;
  objectUrl?: string;
  mime?: string;
  size?: number;
  vasp: VaspProfile;
  synth?: DemoSynth;
};

function profile(
  title: string,
  artist: string,
  pillars: Pillars,
): VaspProfile {
  return {
    VAP_VERSION: VASP_VERSION,
    IDENTITY: { TITLE: title, ARTIST: artist },
    PILLARS: pillars,
  };
}

export const NIGHT_DRIVE: LibraryTrack = {
  id: "demo-night-drive",
  kind: "demo",
  title: "Night Drive Protocol",
  artist: "Aurphyx Demo",
  duration: null,
  synth: {
    bpm: 128,
    rootHz: 110,
    minor: true,
    drive: 0.86,
    brightness: 0.62,
    hatDensity: 1,
  },
  vasp: profile("Night Drive Protocol", "Aurphyx Demo", {
    STRUCTURAL: {
      bpm: known(128),
      timeSignature: known("4/4"),
      groove: known("Machine-lock groove"),
      kickPulse: known("Strong kick pulse"),
      arrangement: known("Two-bar loop, driving eighths"),
    },
    TONAL: {
      key: known("A Minor"),
      mode: known("Natural minor"),
      dissonance: known("Moderate dissonance"),
      contour: known("Dark melodic contour"),
      tuning: known("A440 equal temperament"),
    },
    TIMBRAL: {
      spectral: known("Bright and airy"),
      fidelity: known("Hi-fi"),
      stereo: known("Wide stereo image"),
      texture: known("Glassy electronic texture"),
      production: known("Night-drive synth, tight low end"),
    },
    LINGUISTIC: {
      lyrics: known("Instrumental"),
      language: known("None"),
      vocalStyle: known("No lyrics"),
      contentTier: known("Clean content tier"),
    },
    AFFECTIVE: {
      valence: known("Low-to-neutral valence"),
      arousal: known("High arousal"),
      dominance: known("Focused control"),
      mood: known("Focused, nocturnal, energetic"),
      tension: known("Forward-drive tension"),
    },
    CONTEXTUAL: {
      scenario: known("Night drive"),
      setting: known("City lights / rain atmosphere"),
      activity: known("Solo listening"),
      timeOfDay: known("Late night"),
      atmosphere: known("Wet asphalt, sodium-to-neon"),
    },
    PHOTOMETRIC: {
      primaryHex: known("#4B0082"),
      secondaryHex: known("#008080"),
      temperature: known("cool"),
      brightness: known(0.62),
      fade: known("Smooth visual fades"),
      lightBehavior: known("Indigo and teal light behavior"),
    },
    KINETIC: {
      movementEnergy: known("High movement energy"),
      entrainment: known("Strong beat entrainment"),
      metScore: known(6),
      response: known("Head-nod and forward-drive response"),
    },
    GENEALOGICAL: {
      genre: known("Electronic"),
      lineage: known("Synthwave-inspired"),
      era: known("Contemporary"),
      tribe: known("Digital nocturnal"),
      aesthetic: known("Digital nocturnal aesthetic"),
    },
  }),
};

export const ORBITAL_LATTICE: LibraryTrack = {
  id: "demo-orbital-lattice",
  kind: "demo",
  title: "Orbital Lattice",
  artist: "Aurphyx Demo",
  duration: null,
  synth: {
    bpm: 96,
    rootHz: 146.83,
    minor: true,
    drive: 0.55,
    brightness: 0.42,
    hatDensity: 0.5,
  },
  vasp: profile("Orbital Lattice", "Aurphyx Demo", {
    STRUCTURAL: {
      bpm: known(96),
      timeSignature: known("4/4"),
      groove: known("Slow-orbit pulse"),
      kickPulse: known("Rounded kick, wide decay"),
      arrangement: known("Long pads over sparse percussion"),
    },
    TONAL: {
      key: known("D Minor"),
      mode: known("Dorian color"),
      dissonance: known("Low dissonance"),
      contour: known("Ascending lattice intervals"),
      tuning: known("A440 equal temperament"),
    },
    TIMBRAL: {
      spectral: known("Dark, sub-heavy"),
      fidelity: known("Hi-fi, filtered"),
      stereo: known("Deep stereo field"),
      texture: known("Velvet drone + glass chime"),
      production: known("Orbital pad, distant bells"),
    },
    LINGUISTIC: {
      lyrics: known("Instrumental"),
      language: known("None"),
      vocalStyle: known("No lyrics"),
      contentTier: known("Clean content tier"),
    },
    AFFECTIVE: {
      valence: known("Neutral-positive valence"),
      arousal: known("Mid-low arousal"),
      dominance: known("Suspended"),
      mood: known("Contemplative, weightless"),
      tension: known("Slow harmonic bloom"),
    },
    CONTEXTUAL: {
      scenario: known("Deep space drift"),
      setting: known("Interior of a dark observatory"),
      activity: known("Solo listening"),
      timeOfDay: known("After midnight"),
      atmosphere: known("Vacuum hush, distant stars"),
    },
    PHOTOMETRIC: {
      primaryHex: known("#1B1464"),
      secondaryHex: known("#5B8DEF"),
      temperature: known("cool"),
      brightness: known(0.44),
      fade: known("Long crossfades"),
      lightBehavior: known("Deep indigo with ice-blue rims"),
    },
    KINETIC: {
      movementEnergy: known("Low-mid movement energy"),
      entrainment: known("Breath-paced pulse"),
      metScore: known(3),
      response: known("Stillness with slow sway"),
    },
    GENEALOGICAL: {
      genre: known("Ambient electronic"),
      lineage: known("Orbital / IDM adjacent"),
      era: known("Contemporary"),
      tribe: known("Lattice listeners"),
      aesthetic: known("Fractal quiet"),
    },
  }),
};

export const FORWARD_CURRENT: LibraryTrack = {
  id: "demo-forward-current",
  kind: "demo",
  title: "Forward Current",
  artist: "Aurphyx Demo",
  duration: null,
  synth: {
    bpm: 140,
    rootHz: 185,
    minor: true,
    drive: 0.92,
    brightness: 0.78,
    hatDensity: 1,
  },
  vasp: profile("Forward Current", "Aurphyx Demo", {
    STRUCTURAL: {
      bpm: known(140),
      timeSignature: known("4/4"),
      groove: known("High-energy lock"),
      kickPulse: known("Hard four-on-the-floor"),
      arrangement: known("Peak-time loop, rolling bass"),
    },
    TONAL: {
      key: known("F# Minor"),
      mode: known("Natural minor"),
      dissonance: known("Bright tension"),
      contour: known("Rising current"),
      tuning: known("A440 equal temperament"),
    },
    TIMBRAL: {
      spectral: known("Crisp highs, present mids"),
      fidelity: known("Hi-fi, slightly overdriven"),
      stereo: known("Wide, kinetic"),
      texture: known("Electric current, metallic hats"),
      production: known("Club-leaning electronic"),
    },
    LINGUISTIC: {
      lyrics: known("Instrumental"),
      language: known("None"),
      vocalStyle: known("No lyrics"),
      contentTier: known("Clean content tier"),
    },
    AFFECTIVE: {
      valence: known("Neutral-high valence"),
      arousal: known("Very high arousal"),
      dominance: known("Assertive"),
      mood: known("Urgent, kinetic, lucid"),
      tension: known("Relentless forward pressure"),
    },
    CONTEXTUAL: {
      scenario: known("Rain circuit"),
      setting: known("Elevated freeway in weather"),
      activity: known("Motion listening"),
      timeOfDay: known("Blue hour into night"),
      atmosphere: known("Wet glass, sodium flares"),
    },
    PHOTOMETRIC: {
      primaryHex: known("#0D7377"),
      secondaryHex: known("#7C6CFF"),
      temperature: known("cool"),
      brightness: known(0.74),
      fade: known("Fast visual cuts"),
      lightBehavior: known("Teal core, violet highlights"),
    },
    KINETIC: {
      movementEnergy: known("Very high movement energy"),
      entrainment: known("Body-lock beat"),
      metScore: known(7),
      response: known("Stride and pulse"),
    },
    GENEALOGICAL: {
      genre: known("Electronic"),
      lineage: known("Techno-adjacent synthwave"),
      era: known("Contemporary"),
      tribe: known("Night circuit"),
      aesthetic: known("Rain-slick current"),
    },
  }),
};

export const DEMO_TRACKS: LibraryTrack[] = [
  NIGHT_DRIVE,
  ORBITAL_LATTICE,
  FORWARD_CURRENT,
];

export function hslToHex(h: number, s: number, l: number): string {
  const sat = s / 100;
  const lig = l / 100;
  const k = (n: number) => (n + h / 30) % 12;
  const a = sat * Math.min(lig, 1 - lig);
  const f = (n: number) =>
    lig - a * Math.max(-1, Math.min(k(n) - 3, Math.min(9 - k(n), 1)));
  return rgbToHex(255 * f(0), 255 * f(8), 255 * f(4));
}

export function makeFileVasp(title: string, artist = "Local file"): VaspProfile {
  const hue = hashHue(title);
  const primary = hslToHex((hue + 260) % 360, 72, 28);
  const secondary = hslToHex((hue + 175) % 360, 64, 38);
  return profile(title, artist, {
    STRUCTURAL: {
      bpm: pending(null),
      timeSignature: unknown("4/4"),
      groove: pending("Pending analysis"),
      kickPulse: pending("Pending analysis"),
      arrangement: unknown("Unknown arrangement"),
    },
    TONAL: {
      key: pending("Pending"),
      mode: unknown("Unknown"),
      dissonance: pending("Pending"),
      contour: unknown("Unknown"),
      tuning: known("Source file"),
    },
    TIMBRAL: {
      spectral: pending("Pending"),
      fidelity: known("Source file"),
      stereo: unknown("Unknown"),
      texture: pending("Pending"),
      production: known("Local playback"),
    },
    LINGUISTIC: {
      lyrics: unknown("Unknown"),
      language: unknown("Unknown"),
      vocalStyle: unknown("Unknown"),
      contentTier: unknown("Unknown"),
    },
    AFFECTIVE: {
      valence: pending("Pending"),
      arousal: pending("Pending"),
      dominance: unknown("Unknown"),
      mood: pending("Listening"),
      tension: pending("Pending"),
    },
    CONTEXTUAL: {
      scenario: known("Local playback"),
      setting: known("On device"),
      activity: known("Solo listening"),
      timeOfDay: unknown("Unknown"),
      atmosphere: known("Player session"),
    },
    PHOTOMETRIC: {
      primaryHex: known(primary),
      secondaryHex: known(secondary),
      temperature: known("cool"),
      brightness: known(0.6),
      fade: known("Smooth visual fades"),
      lightBehavior: known("File-derived palette"),
    },
    KINETIC: {
      movementEnergy: pending("Pending"),
      entrainment: pending("Pending"),
      metScore: pending(null),
      response: unknown("Unknown"),
    },
    GENEALOGICAL: {
      genre: unknown("Unknown"),
      lineage: known("Local library"),
      era: unknown("Unknown"),
      tribe: known("Personal collection"),
      aesthetic: known("Source material"),
    },
  });
}

export function trackFromFile(file: File): LibraryTrack {
  const title = file.name.replace(/\.[^.]+$/, "").replace(/[_-]+/g, " ").trim() || "Untitled";
  return {
    id: uid("file"),
    kind: "file",
    title,
    artist: "Local file",
    duration: null,
    objectUrl: URL.createObjectURL(file),
    mime: file.type || "audio/*",
    size: file.size,
    vasp: makeFileVasp(title),
  };
}

export function mappingFromProfile(profile: VaspProfile): VisualMapping {
  const p = profile.PILLARS;
  const bpm = p.STRUCTURAL.bpm.value ?? 120;
  const kick = (p.STRUCTURAL.kickPulse.value ?? "").toLowerCase();
  const pulseStrength = kick.includes("strong") || kick.includes("hard") ? 1 : kick.includes("round") ? 0.55 : 0.75;
  const key = (p.TONAL.key.value ?? "").toLowerCase();
  const geometrySides = key.includes("minor") ? 6 : key.includes("major") ? 4 : 5;
  const dissonance = (p.TONAL.dissonance.value ?? "").toLowerCase();
  const harmonicShift = dissonance.includes("high") || dissonance.includes("bright") ? 0.18 : dissonance.includes("moderate") ? 0.1 : 0.04;
  const spectral = (p.TIMBRAL.spectral.value ?? "").toLowerCase();
  const particleDensity = spectral.includes("bright") || spectral.includes("crisp") ? 1 : spectral.includes("dark") ? 0.55 : 0.8;
  const texture = (p.TIMBRAL.texture.value ?? "").toLowerCase();
  const blur = texture.includes("velvet") || texture.includes("drone") ? 0.55 : texture.includes("glassy") ? 0.22 : 0.3;
  const grain = texture.includes("metallic") || texture.includes("over") ? 0.18 : 0.08;
  const arousal = (p.AFFECTIVE.arousal.value ?? "").toLowerCase();
  const movementEnergy = arousal.includes("very high") ? 1 : arousal.includes("high") ? 0.82 : arousal.includes("mid") ? 0.45 : 0.32;
  const contrast = arousal.includes("high") ? 0.85 : 0.55;
  const scene = (p.CONTEXTUAL.scenario.value ?? "Night drive").toString();
  const atmosphere = (p.CONTEXTUAL.atmosphere.value ?? "").toLowerCase();
  const rain = atmosphere.includes("rain") || atmosphere.includes("wet") || scene.toLowerCase().includes("rain") || scene.toLowerCase().includes("drive");
  const met = p.KINETIC.metScore.value ?? 5;
  const impactScale = Math.min(1.2, 0.45 + met / 10);
  const lineage = (p.GENEALOGICAL.lineage.value ?? "").toLowerCase();
  const accentWarmth = lineage.includes("synthwave") ? 0.22 : 0.08;
  const temp = p.PHOTOMETRIC.temperature.value ?? "cool";
  return {
    primary: p.PHOTOMETRIC.primaryHex.value ?? "#4B0082",
    secondary: p.PHOTOMETRIC.secondaryHex.value ?? "#008080",
    temperature: temp,
    brightness: p.PHOTOMETRIC.brightness.value ?? 0.6,
    fade: (p.PHOTOMETRIC.fade.value ?? "").toLowerCase().includes("fast") ? 0.35 : 0.18,
    bpm,
    pulseStrength,
    geometrySides,
    harmonicShift,
    particleDensity,
    blur,
    grain,
    edgeSharpness: texture.includes("glassy") || spectral.includes("crisp") ? 0.85 : 0.5,
    contrast,
    movementEnergy,
    scene,
    rain,
    impactScale,
    accentWarmth,
  };
}

export function flattenPillar(profile: VaspProfile, key: keyof Pillars): { label: string; value: string; status: string }[] {
  const pillar = profile.PILLARS[key] as unknown as Record<string, { value: unknown; status: string }>;
  const labels: Record<string, string> = {
    bpm: "BPM",
    timeSignature: "Time signature",
    groove: "Groove",
    kickPulse: "Kick pulse",
    arrangement: "Arrangement",
    key: "Key",
    mode: "Mode",
    dissonance: "Dissonance",
    contour: "Contour",
    tuning: "Tuning",
    spectral: "Spectral balance",
    fidelity: "Fidelity",
    stereo: "Stereo image",
    texture: "Texture",
    production: "Production",
    lyrics: "Lyrics",
    language: "Language",
    vocalStyle: "Vocal style",
    contentTier: "Content tier",
    valence: "Valence",
    arousal: "Arousal",
    dominance: "Dominance",
    mood: "Mood",
    tension: "Tension",
    scenario: "Scenario",
    setting: "Setting",
    activity: "Activity",
    timeOfDay: "Time of day",
    atmosphere: "Atmosphere",
    primaryHex: "Primary hex",
    secondaryHex: "Secondary hex",
    temperature: "Palette temperature",
    brightness: "Brightness",
    fade: "Fades",
    lightBehavior: "Light behavior",
    movementEnergy: "Movement energy",
    entrainment: "Entrainment",
    metScore: "MET score",
    response: "Body response",
    genre: "Genre",
    lineage: "Lineage",
    era: "Era",
    tribe: "Tribe",
    aesthetic: "Aesthetic",
  };
  return Object.entries(pillar).map(([k, field]) => ({
    label: labels[k] ?? k,
    value: field.value == null ? "—" : String(field.value),
    status: field.status,
  }));
}
