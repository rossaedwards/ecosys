/** V.A.P. v3.1 types mirrored from vmp-vap (UI layer). */

export type PillarId =
  | 'STRUCTURAL'
  | 'TONAL'
  | 'TIMBRAL'
  | 'LINGUISTIC'
  | 'AFFECTIVE'
  | 'CONTEXTUAL'
  | 'PHOTOMETRIC'
  | 'KINETIC'
  | 'GENEALOGICAL';

export interface PillarTab {
  id: PillarId;
  index: number;
  short: string;
  label: string;
  accent: string;
}

export const PILLAR_TABS: PillarTab[] = [
  { id: 'STRUCTURAL', index: 1, short: 'P1', label: 'Structural', accent: '#9b5de5' },
  { id: 'TONAL', index: 2, short: 'P2', label: 'Tonal', accent: '#c77dff' },
  { id: 'TIMBRAL', index: 3, short: 'P3', label: 'Timbral', accent: '#00f5d4' },
  { id: 'LINGUISTIC', index: 4, short: 'P4', label: 'Linguistic', accent: '#80ffee' },
  { id: 'AFFECTIVE', index: 5, short: 'P5', label: 'Affective', accent: '#f72585' },
  { id: 'CONTEXTUAL', index: 6, short: 'P6', label: 'Contextual', accent: '#ff6eb4' },
  { id: 'PHOTOMETRIC', index: 7, short: 'P7', label: 'Photometric', accent: '#ffd166' },
  { id: 'KINETIC', index: 8, short: 'P8', label: 'Kinetic', accent: '#ffe599' },
  { id: 'GENEALOGICAL', index: 9, short: 'P9', label: 'Genealogical', accent: '#9b5de5' },
];

export interface VapIdentity {
  TITLE: string;
  ARTIST: string;
  ISRC?: string;
  SOURCE_DNA?: string;
}

export interface VapObject {
  VAP_VERSION: string;
  IDENTITY: VapIdentity;
  PILLARS: Partial<Record<PillarId, unknown>>;
}

export type EqMode =
  | 'graphic10'
  | 'graphic31'
  | 'parametric'
  | 'vap_guided'
  | 'context_linked'
  | 'bypass';

export const EQ_10_BANDS = [
  '32Hz',
  '63',
  '125',
  '250',
  '500',
  '1k',
  '2k',
  '4k',
  '8k',
  '16k',
] as const;

export interface DeviceInfo {
  id: string;
  name: string;
  is_input: boolean;
  is_output: boolean;
}
