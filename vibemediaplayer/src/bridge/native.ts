/**
 * Dual-runtime bridge: Tauri native (Symphonia + cpal + lofty) or browser fallback.
 * Uses window.__TAURI__ so the web build does not require @tauri-apps npm packages.
 */

export type PlayerStatus = {
  path: string | null;
  playing: boolean;
  position_sec: number;
  duration_sec: number;
  volume: number;
  sample_rate: number;
  channels: number;
  ended: boolean;
  backend: string;
};

export type OpenResult = {
  status: PlayerStatus;
  vap: unknown;
  vap_source: string;
  format: string;
  can_embed: boolean;
};

type TauriGlobal = {
  core?: { invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T> };
  invoke?: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
};

function tauri(): TauriGlobal | null {
  if (typeof window === 'undefined') return null;
  const w = window as unknown as {
    __TAURI__?: TauriGlobal;
    __TAURI_INTERNALS__?: unknown;
  };
  return w.__TAURI__ ?? null;
}

export function isNative(): boolean {
  if (typeof window === 'undefined') return false;
  const w = window as unknown as { __TAURI_INTERNALS__?: unknown; __TAURI__?: unknown };
  return Boolean(w.__TAURI_INTERNALS__ || w.__TAURI__);
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const t = tauri();
  if (t?.core?.invoke) return t.core.invoke<T>(cmd, args);
  if (t?.invoke) return t.invoke<T>(cmd, args);
  // Tauri 2 injects via internals
  const internals = (window as unknown as { __TAURI_INTERNALS__?: { invoke: typeof invoke } })
    .__TAURI_INTERNALS__;
  if (internals?.invoke) {
    return (internals.invoke as (c: string, a?: Record<string, unknown>) => Promise<T>)(cmd, args);
  }
  throw new Error('Tauri IPC unavailable');
}

export async function nativeOpenFile(path: string): Promise<OpenResult | null> {
  if (!isNative()) return null;
  return invoke<OpenResult>('open_file', { path });
}

export async function nativePlay(): Promise<PlayerStatus | null> {
  if (!isNative()) return null;
  return invoke<PlayerStatus>('play');
}

export async function nativePause(): Promise<PlayerStatus | null> {
  if (!isNative()) return null;
  return invoke<PlayerStatus>('pause');
}

export async function nativeToggle(): Promise<PlayerStatus | null> {
  if (!isNative()) return null;
  return invoke<PlayerStatus>('toggle');
}

export async function nativeSeek(sec: number): Promise<PlayerStatus | null> {
  if (!isNative()) return null;
  return invoke<PlayerStatus>('seek', { sec });
}

export async function nativeStatus(): Promise<PlayerStatus | null> {
  if (!isNative()) return null;
  return invoke<PlayerStatus>('status');
}

export async function nativeSetVolume(volume: number): Promise<PlayerStatus | null> {
  if (!isNative()) return null;
  return invoke<PlayerStatus>('set_volume', { volume });
}

export async function nativeSetEqBand(index: number, gainDb: number): Promise<void> {
  if (!isNative()) return;
  await invoke('set_eq_band', { index, gain_db: gainDb });
}

export async function nativeSaveVap(embed: boolean): Promise<unknown | null> {
  if (!isNative()) return null;
  return invoke('save_vap', { embed });
}

export async function nativeSetVap(vap: unknown): Promise<void> {
  if (!isNative()) return;
  await invoke('set_vap', { vap });
}

export async function nativeOpenFolder(path: string): Promise<string[] | null> {
  if (!isNative()) return null;
  return invoke<string[]>('open_folder', { path });
}

export async function nativePickFile(): Promise<string | null> {
  if (!isNative()) return null;
  try {
    // dialog plugin command
    const selected = await invoke<string | string[] | null>('plugin:dialog|open', {
      multiple: false,
      filters: [
        {
          name: 'Media',
          extensions: [
            'mp3',
            'm4a',
            'aac',
            'flac',
            'ogg',
            'opus',
            'wav',
            'aiff',
            'aif',
            'mp4',
            'm4v',
          ],
        },
      ],
    });
    if (typeof selected === 'string') return selected;
    if (Array.isArray(selected) && selected[0]) return selected[0];
    return null;
  } catch {
    // Fallback: prompt for absolute path
    return window.prompt('Native path to media file:') || null;
  }
}

export async function nativePickFolder(): Promise<string | null> {
  if (!isNative()) return null;
  try {
    const selected = await invoke<string | string[] | null>('plugin:dialog|open', {
      directory: true,
      multiple: false,
    });
    if (typeof selected === 'string') return selected;
    if (Array.isArray(selected) && selected[0]) return selected[0];
    return null;
  } catch {
    return window.prompt('Native path to a folder:') || null;
  }
}

export async function nativeVersion(): Promise<unknown | null> {
  if (!isNative()) return null;
  return invoke('vmp_version');
}

type TauriInternals = {
  transformCallback: (callback: (payload: unknown) => void, once?: boolean) => number;
};

function internals(): TauriInternals | null {
  if (typeof window === 'undefined') return null;
  return (window as unknown as { __TAURI_INTERNALS__?: TauriInternals }).__TAURI_INTERNALS__ ?? null;
}

/**
 * Subscribe to a backend `app.emit(event, payload)`. Hand-rolled against the
 * documented Tauri v2 core IPC protocol (`plugin:event|listen` /
 * `plugin:event|unlisten` + `__TAURI_INTERNALS__.transformCallback`) rather
 * than `@tauri-apps/api/event`, matching this bridge's no-npm-Tauri-deps
 * pattern. Returns an unsubscribe function; a no-op in the browser build.
 */
export async function nativeListen<T>(
  event: string,
  handler: (payload: T) => void,
): Promise<() => void> {
  if (!isNative()) return () => {};
  const t = internals();
  if (!t) return () => {};

  const handlerId = t.transformCallback((raw: unknown) => {
    handler((raw as { payload: T }).payload);
  });

  const unlistenId = await invoke<number>('plugin:event|listen', {
    event,
    windowLabel: null,
    handler: handlerId,
  });

  let unsubscribed = false;
  return () => {
    if (unsubscribed) return;
    unsubscribed = true;
    invoke('plugin:event|unlisten', { event, eventId: unlistenId }).catch(() => {});
  };
}
