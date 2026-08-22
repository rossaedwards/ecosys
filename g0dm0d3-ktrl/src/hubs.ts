export type HubId =
  | 'grok'
  | 'claude'
  | 'gemini'
  | 'copilot'
  | 'hermes'
  | 'perplexity'
  | 'lechat'
  | 'ollama'
  | 'openai';

export type PlanetState = 'dark' | 'orbit' | 'surface';

export interface HubSpec {
  id: HubId;
  name: string;
  color: string;
  glow: string;
  icon: string;
  defaultText: string;
}

export const HUBS: HubSpec[] = [
  {
    id: 'grok',
    name: 'Grok',
    color: '#FFFFFF',
    glow: '#aaaaaa',
    icon: '𝕏',
    defaultText:
      'INITIALIZING X_DATAFEED...\nAwaiting overriding command from g0dm0d3.\n\n[SYSTEM] No anomalous behavior detected.',
  },
  {
    id: 'claude',
    name: 'Claude',
    color: '#D97757',
    glow: '#ff9966',
    icon: '☀️',
    defaultText:
      'CONTEXT WINDOW EXPANDED.\nSynthesizing long-form architectural documents.\nStanding by for chain-links.',
  },
  {
    id: 'gemini',
    name: 'Gemini',
    color: '#4285F4',
    glow: '#b666ff',
    icon: '✨',
    defaultText:
      'MULTI-MODAL SENSORS ONLINE.\nVisual rendering protocols on standby. Ready to architect prototypes.',
  },
  {
    id: 'copilot',
    name: 'Copilot',
    color: '#22c55e',
    glow: '#4ade80',
    icon: '⎈',
    defaultText: 'GITHUB / OPENAI-COMPAT NODE.\nIdle. Bind a token to spend.',
  },
  {
    id: 'hermes',
    name: 'Hermes',
    color: '#FF00FF',
    glow: '#ff00ff',
    icon: '🪖',
    defaultText: 'HERMES AGENT SLOT.\nOpenRouter / Nous path. Awaiting bind.',
  },
  {
    id: 'perplexity',
    name: 'Perplexity',
    color: '#22B8CD',
    glow: '#00ffff',
    icon: '⚗️',
    defaultText: 'INDEXING REAL-TIME WEB SHARDS...\nReady to verify Oracles.',
  },
  {
    id: 'lechat',
    name: 'LeChat',
    color: '#fa811b',
    glow: '#ffaa55',
    icon: '🜂',
    defaultText: 'MISTRAL LECHAT NODE.\nIdle. Bind a Mistral key to spend.',
  },
  {
    id: 'ollama',
    name: 'Ollama',
    color: '#9ca3af',
    glow: '#d1d5db',
    icon: '🦙',
    defaultText: 'LOCAL INSTANCE :11434.\nNo cloud key. Model name in keyring (default llama3.2).',
  },
];

export const BIND_HUBS: HubId[] = [
  'grok',
  'claude',
  'gemini',
  'copilot',
  'hermes',
  'perplexity',
  'lechat',
  'ollama',
];

export const SUITE_STUBS = [
  'Framez',
  'Termz',
  'Webz',
  'Xplor',
  'Codex',
  'Forge',
  'Adorè',
  'Gimpd',
] as const;
