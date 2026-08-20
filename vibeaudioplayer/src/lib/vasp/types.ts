export const VASP_VERSION = "3.69" as const;

export const PILLAR_KEYS = [
  "STRUCTURAL",
  "TONAL",
  "TIMBRAL",
  "LINGUISTIC",
  "AFFECTIVE",
  "CONTEXTUAL",
  "PHOTOMETRIC",
  "KINETIC",
  "GENEALOGICAL",
] as const;

export type PillarKey = (typeof PILLAR_KEYS)[number];

export type FieldStatus = "known" | "unknown" | "pending" | "unavailable";

export type VaspField<T> = {
  value: T;
  status: FieldStatus;
};

export type StructuralPillar = {
  bpm: VaspField<number | null>;
  timeSignature: VaspField<string>;
  groove: VaspField<string>;
  kickPulse: VaspField<string>;
  arrangement: VaspField<string>;
};

export type TonalPillar = {
  key: VaspField<string>;
  mode: VaspField<string>;
  dissonance: VaspField<string>;
  contour: VaspField<string>;
  tuning: VaspField<string>;
};

export type TimbralPillar = {
  spectral: VaspField<string>;
  fidelity: VaspField<string>;
  stereo: VaspField<string>;
  texture: VaspField<string>;
  production: VaspField<string>;
};

export type LinguisticPillar = {
  lyrics: VaspField<string>;
  language: VaspField<string>;
  vocalStyle: VaspField<string>;
  contentTier: VaspField<string>;
};

export type AffectivePillar = {
  valence: VaspField<string>;
  arousal: VaspField<string>;
  dominance: VaspField<string>;
  mood: VaspField<string>;
  tension: VaspField<string>;
};

export type ContextualPillar = {
  scenario: VaspField<string>;
  setting: VaspField<string>;
  activity: VaspField<string>;
  timeOfDay: VaspField<string>;
  atmosphere: VaspField<string>;
};

export type PhotometricPillar = {
  primaryHex: VaspField<string>;
  secondaryHex: VaspField<string>;
  temperature: VaspField<"cool" | "neutral" | "warm">;
  brightness: VaspField<number>;
  fade: VaspField<string>;
  lightBehavior: VaspField<string>;
};

export type KineticPillar = {
  movementEnergy: VaspField<string>;
  entrainment: VaspField<string>;
  metScore: VaspField<number | null>;
  response: VaspField<string>;
};

export type GenealogicalPillar = {
  genre: VaspField<string>;
  lineage: VaspField<string>;
  era: VaspField<string>;
  tribe: VaspField<string>;
  aesthetic: VaspField<string>;
};

export type Pillars = {
  STRUCTURAL: StructuralPillar;
  TONAL: TonalPillar;
  TIMBRAL: TimbralPillar;
  LINGUISTIC: LinguisticPillar;
  AFFECTIVE: AffectivePillar;
  CONTEXTUAL: ContextualPillar;
  PHOTOMETRIC: PhotometricPillar;
  KINETIC: KineticPillar;
  GENEALOGICAL: GenealogicalPillar;
};

export type VaspIdentity = {
  TITLE: string;
  ARTIST: string;
};

export type VaspProfile = {
  VAP_VERSION: typeof VASP_VERSION;
  IDENTITY: VaspIdentity;
  PILLARS: Pillars;
};

export type PillarMeta = {
  key: PillarKey;
  label: string;
  archetype: string;
  purpose: string;
};

export const PILLAR_META: Record<PillarKey, PillarMeta> = {
  STRUCTURAL: {
    key: "STRUCTURAL",
    label: "Structural",
    archetype: "The Skeleton",
    purpose: "Tempo, time signature, rhythmic behavior, arrangement, and percussive DNA",
  },
  TONAL: {
    key: "TONAL",
    label: "Tonal",
    archetype: "The Flesh",
    purpose: "Key, harmony, melody, dissonance, pitch, and tuning",
  },
  TIMBRAL: {
    key: "TIMBRAL",
    label: "Timbral",
    archetype: "The Skin",
    purpose: "Spectral balance, fidelity, spatial character, production aesthetic, and texture",
  },
  LINGUISTIC: {
    key: "LINGUISTIC",
    label: "Linguistic",
    archetype: "The Voice",
    purpose: "Lyrics, semantic content, vocal style, language, and explicit-content classification",
  },
  AFFECTIVE: {
    key: "AFFECTIVE",
    label: "Affective",
    archetype: "The Heart",
    purpose: "Valence, arousal, dominance, emotional complexity, and tension movement",
  },
  CONTEXTUAL: {
    key: "CONTEXTUAL",
    label: "Contextual",
    archetype: "The Scene",
    purpose: "Scenario, setting, activity, intent, time of day, weather, and environmental match",
  },
  PHOTOMETRIC: {
    key: "PHOTOMETRIC",
    label: "Photometric",
    archetype: "The Eye",
    purpose: "Color palette, visual texture, brightness, lighting behavior, and synchronization output",
  },
  KINETIC: {
    key: "KINETIC",
    label: "Kinetic",
    archetype: "The Body",
    purpose: "Entrainment, movement response, energy expenditure, and physical activity metadata",
  },
  GENEALOGICAL: {
    key: "GENEALOGICAL",
    label: "Genealogical",
    archetype: "The Roots",
    purpose: "Era, sampling lineage, genre tree, cultural context, and tribe alignment",
  },
};

export type VisualMapping = {
  primary: string;
  secondary: string;
  temperature: "cool" | "neutral" | "warm";
  brightness: number;
  fade: number;
  bpm: number;
  pulseStrength: number;
  geometrySides: number;
  harmonicShift: number;
  particleDensity: number;
  blur: number;
  grain: number;
  edgeSharpness: number;
  contrast: number;
  movementEnergy: number;
  scene: string;
  rain: boolean;
  impactScale: number;
  accentWarmth: number;
};

export type VisualSettings = {
  colorIntensity: number;
  motionIntensity: number;
  particles: boolean;
  spectrum: boolean;
  beatPulse: boolean;
  reducedMotion: boolean;
  readableType: boolean;
};
