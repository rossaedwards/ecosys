import { create } from "zustand";
import { persist } from "zustand/middleware";
import { engine } from "@/lib/audio/engine";
import {
  deleteStoredTrack,
  listStoredTracks,
  putStoredTrack,
} from "@/lib/audio/library-db";
import {
  DEMO_TRACKS,
  NIGHT_DRIVE,
  makeFileVasp,
  mappingFromProfile,
  trackFromFile,
  type LibraryTrack,
} from "@/lib/vasp/catalog";
import type { PillarKey, VisualMapping, VisualSettings } from "@/lib/vasp/types";

export type SheetName = "none" | "library" | "vasp" | "settings" | "about";
export type RepeatMode = "off" | "all" | "one";

const DEFAULT_SETTINGS: VisualSettings = {
  colorIntensity: 0.82,
  motionIntensity: 0.78,
  particles: true,
  spectrum: true,
  beatPulse: true,
  reducedMotion: false,
  readableType: false,
};

type PlayerState = {
  hydrated: boolean;
  library: LibraryTrack[];
  currentId: string;
  playing: boolean;
  currentTime: number;
  duration: number;
  volume: number;
  shuffle: boolean;
  repeat: RepeatMode;
  sheet: SheetName;
  activePillar: PillarKey;
  settings: VisualSettings;
  mapping: VisualMapping;
  dropActive: boolean;
  notice: string | null;
  current: () => LibraryTrack;
  hydrate: () => Promise<void>;
  playTrack: (id: string) => Promise<void>;
  togglePlay: () => Promise<void>;
  next: () => Promise<void>;
  prev: () => Promise<void>;
  seek: (t: number) => void;
  setVolume: (v: number) => void;
  setSheet: (s: SheetName) => void;
  setPillar: (p: PillarKey) => void;
  setSetting: <K extends keyof VisualSettings>(key: K, value: VisualSettings[K]) => void;
  resetSettings: () => void;
  importFiles: (files: FileList | File[]) => Promise<void>;
  removeTrack: (id: string) => Promise<void>;
  setShuffle: (v: boolean) => void;
  setRepeat: (v: RepeatMode) => void;
  setDropActive: (v: boolean) => void;
  setNotice: (v: string | null) => void;
  onEngineEnded: () => void;
  onEngineTime: (t: number, d: number) => void;
};

function orderIds(library: LibraryTrack[], shuffle: boolean, currentId: string) {
  const ids = library.map((t) => t.id);
  if (!shuffle) return ids;
  const rest = ids.filter((id) => id !== currentId);
  for (let i = rest.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    const a = rest[i]!;
    rest[i] = rest[j]!;
    rest[j] = a;
  }
  return [currentId, ...rest];
}

function publishMetadata(track: LibraryTrack) {
  if (typeof navigator === "undefined" || !("mediaSession" in navigator)) return;
  try {
    navigator.mediaSession.metadata = new MediaMetadata({
      title: track.title,
      artist: track.artist,
      album: "Vibe Audio Player",
    });
  } catch {
    /* ignore */
  }
}

export const usePlayer = create<PlayerState>()(
  persist(
    (set, get) => ({
      hydrated: false,
      library: DEMO_TRACKS,
      currentId: NIGHT_DRIVE.id,
      playing: false,
      currentTime: 0,
      duration: 0,
      volume: 0.72,
      shuffle: false,
      repeat: "all",
      sheet: "none",
      activePillar: "PHOTOMETRIC",
      settings: DEFAULT_SETTINGS,
      mapping: mappingFromProfile(NIGHT_DRIVE.vasp),
      dropActive: false,
      notice: null,
      current: () => get().library.find((t) => t.id === get().currentId) ?? get().library[0] ?? NIGHT_DRIVE,
      async hydrate() {
        if (get().hydrated) return;
        engine.setVolume(get().volume);
        engine.setEndedHandler(() => get().onEngineEnded());
        engine.setTimeHandler((t, d) => get().onEngineTime(t, d));
        try {
          const stored = await listStoredTracks();
          if (stored.length) {
            const restored: LibraryTrack[] = stored.map((row) => ({
              id: row.id,
              kind: "file",
              title: row.title,
              artist: row.artist,
              duration: null,
              objectUrl: URL.createObjectURL(row.blob),
              mime: row.mime,
              size: row.size,
              vasp: makeFileVasp(row.title, row.artist),
            }));
            set({ library: [...DEMO_TRACKS, ...restored] });
          }
        } catch {
          /* IDB unavailable */
        }
        const track = get().current();
        set({
          hydrated: true,
          mapping: mappingFromProfile(track.vasp),
        });
        engine.setBpm(track.vasp.PILLARS.STRUCTURAL.bpm.value ?? 120);
        if (typeof navigator !== "undefined" && "mediaSession" in navigator) {
          try {
            navigator.mediaSession.setActionHandler("play", () => void get().togglePlay());
            navigator.mediaSession.setActionHandler("pause", () => void get().togglePlay());
            navigator.mediaSession.setActionHandler("nexttrack", () => void get().next());
            navigator.mediaSession.setActionHandler("previoustrack", () => void get().prev());
          } catch {
            /* optional */
          }
        }
      },
      async playTrack(id: string) {
        const track = get().library.find((t) => t.id === id);
        if (!track) return;
        engine.setBpm(track.vasp.PILLARS.STRUCTURAL.bpm.value ?? 120);
        set({
          currentId: id,
          mapping: mappingFromProfile(track.vasp),
          currentTime: 0,
          duration: track.duration ?? 0,
        });
        publishMetadata(track);
        try {
          if (track.kind === "demo" && track.synth) {
            await engine.playDemo(track.synth);
          } else if (track.objectUrl) {
            await engine.playFile(track.objectUrl);
            set({ duration: engine.duration() || track.duration || 0 });
          }
          set({ playing: true, notice: null });
        } catch (err) {
          set({
            playing: false,
            notice: err instanceof Error ? err.message : "Could not play this track",
          });
        }
      },
      async togglePlay() {
        const { playing, currentId, current } = get();
        const track = current();
        if (!playing) {
          await get().playTrack(currentId);
          return;
        }
        if (track.kind === "file") {
          const nowPlaying = await engine.togglePause();
          set({ playing: nowPlaying });
          return;
        }
        engine.pause();
        set({ playing: false });
      },
      async next() {
        const { library, currentId, shuffle, repeat, playing } = get();
        if (repeat === "one" && playing) {
          await get().playTrack(currentId);
          return;
        }
        const ids = orderIds(library, shuffle, currentId);
        const i = ids.indexOf(currentId);
        const nextId = ids[(i + 1) % ids.length];
        if (!nextId) return;
        if (repeat === "off" && i === ids.length - 1) {
          engine.pause();
          set({ playing: false, currentTime: 0 });
          return;
        }
        await get().playTrack(nextId);
      },
      async prev() {
        const { library, currentId, shuffle, currentTime } = get();
        if (currentTime > 3) {
          engine.seek(0);
          set({ currentTime: 0 });
          return;
        }
        const ids = orderIds(library, shuffle, currentId);
        const i = ids.indexOf(currentId);
        const prevId = ids[(i - 1 + ids.length) % ids.length];
        if (prevId) await get().playTrack(prevId);
      },
      seek(t: number) {
        engine.seek(t);
        set({ currentTime: t });
      },
      setVolume(v: number) {
        engine.setVolume(v);
        set({ volume: v });
      },
      setSheet(s) {
        set({ sheet: s });
      },
      setPillar(p) {
        set({ activePillar: p, sheet: "vasp" });
      },
      setSetting(key, value) {
        set({ settings: { ...get().settings, [key]: value } });
      },
      resetSettings() {
        const demo = NIGHT_DRIVE;
        set({
          settings: DEFAULT_SETTINGS,
          currentId: demo.id,
          mapping: mappingFromProfile(demo.vasp),
        });
      },
      async importFiles(files) {
        const list = Array.from(files).filter(
          (f) => f.type.startsWith("audio/") || /\.(mp3|wav|flac|ogg|m4a|aac|opus|webm)$/i.test(f.name),
        );
        if (!list.length) {
          set({ notice: "No audio files found in that selection." });
          return;
        }
        const added: LibraryTrack[] = [];
        for (const file of list) {
          const track = trackFromFile(file);
          added.push(track);
          try {
            await putStoredTrack({
              id: track.id,
              title: track.title,
              artist: track.artist,
              mime: track.mime ?? file.type,
              size: file.size,
              addedAt: Date.now(),
              blob: file,
            });
          } catch {
            /* quota — keep in-session anyway */
          }
        }
        set({
          library: [...get().library, ...added],
          sheet: "library",
          notice: `Added ${added.length} track${added.length === 1 ? "" : "s"} to your library.`,
        });
        if (!get().playing) await get().playTrack(added[0]!.id);
      },
      async removeTrack(id: string) {
        const track = get().library.find((t) => t.id === id);
        if (!track || track.kind === "demo") return;
        if (track.objectUrl) URL.revokeObjectURL(track.objectUrl);
        try {
          await deleteStoredTrack(id);
        } catch {
          /* ignore */
        }
        const library = get().library.filter((t) => t.id !== id);
        const nextCurrent = get().currentId === id ? (library[0]?.id ?? NIGHT_DRIVE.id) : get().currentId;
        const wasCurrent = get().currentId === id;
        set({ library });
        if (wasCurrent) {
          engine.pause();
          set({ playing: false, currentId: nextCurrent });
          const nextTrack = library.find((t) => t.id === nextCurrent) ?? NIGHT_DRIVE;
          set({ mapping: mappingFromProfile(nextTrack.vasp) });
        }
      },
      setShuffle(v) {
        set({ shuffle: v });
      },
      setRepeat(v) {
        set({ repeat: v });
      },
      setDropActive(v) {
        set({ dropActive: v });
      },
      setNotice(v) {
        set({ notice: v });
      },
      onEngineEnded() {
        void get().next();
      },
      onEngineTime(t, d) {
        set({ currentTime: t, duration: d || get().duration });
      },
    }),
    {
      name: "vibe-player-settings",
      partialize: (s) => ({
        volume: s.volume,
        settings: s.settings,
        shuffle: s.shuffle,
        repeat: s.repeat,
      }),
    },
  ),
);
