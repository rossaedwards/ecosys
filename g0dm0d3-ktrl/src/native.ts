export interface HubReply {
  hub: string;
  ok: boolean;
  text: string;
  configured: boolean;
}

export interface MemoreeHealth {
  ok: boolean;
  base: string | null;
  source: string;
  detail: string;
}

export class NativeMissingError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'NativeMissingError';
  }
}

function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) {
    throw new NativeMissingError(
      'Tauri IPC missing — run npm run tauri:dev for keyring, route_clip, and Memoree.',
    );
  }
  const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
  return tauriInvoke<T>(cmd, args);
}

export async function bindOracle(hub: string, key: string): Promise<string> {
  return invoke('bind_oracle', { hub, key });
}

export async function oracleStatus(): Promise<{ hub: string; bound: boolean }[]> {
  return invoke('oracle_status');
}

export async function setKiosk(on: boolean): Promise<void> {
  await invoke('set_kiosk', { on });
}

export async function memoreeHealth(): Promise<MemoreeHealth> {
  return invoke('memoree_health');
}

export async function memoreeSaveClip(
  text: string,
  sourceHub: string,
  parentId?: string,
): Promise<unknown> {
  return invoke('memoree_save_clip', {
    text,
    sourceHub,
    parentId: parentId ?? null,
  });
}

export async function broadcastPrompt(prompt: string, targets: string[]): Promise<HubReply[]> {
  return invoke('broadcast_prompt', { prompt, targets });
}

export async function routeClip(
  clip: string,
  sourceHub: string,
  targets: string[],
  extraPrompt?: string,
): Promise<HubReply[]> {
  return invoke('route_clip', {
    clip,
    sourceHub,
    targets,
    extraPrompt: extraPrompt ?? null,
  });
}

export { isTauri };
