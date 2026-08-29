import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { VerticalPillarTabs } from './VerticalPillarTabs';
import { PillarPanel } from './PillarPanel';
import { EqRack } from './EqRack';
import { DeviceMatrixPanel } from './DeviceMatrix';
import { FileMenuBar } from './FileMenu';
import { FloatingModule, type ModulePos } from './FloatingModule';
import { VinylVibez } from './VinylVibez';
import { AuraphyxCanvas } from '../auraphyx/AuraphyxCanvas';
// Note: useLiveSpectrum (Web Audio AnalyserNode) is no longer used here —
// Auraphyx replaced the CSS-bar visualizer that consumed it — but the hook
// itself is left in ../hooks/useLiveSpectrum.ts as a possibly-reusable utility.
import { CANNIBAL_CORPSE_FIXTURE } from '../vap/fixture';
import type { DeviceInfo, EqMode, PillarId, VapObject } from '../vap/types';
import { PILLAR_TABS } from '../vap/types';
import {
  formatLabel,
  isMediaFile,
  MEDIA_ACCEPT,
  type FileMenuId,
  type PlaylistEntry,
} from '../media/formats';
import {
  isNative,
  nativeOpenFile,
  nativeOpenFolder,
  nativePause,
  nativePickFile,
  nativePickFolder,
  nativePlay,
  nativeSaveVap,
  nativeSeek,
  nativeSetEqBand,
  nativeSetVap,
  nativeSetVolume,
  nativeStatus,
  nativeToggle,
  nativeVersion,
} from '../bridge/native';

type AppMode = 'player' | 'vinyl';

const SKINS: Record<string, Record<string, string>> = {
  soul_cosmic: {
    '--vmp-void': '#0a0a18',
    '--vmp-surface': '#11112a',
    '--vmp-panel': '#18183a',
    '--vmp-violet': '#9b5de5',
    '--vmp-teal': '#00f5d4',
    '--vmp-pink': '#f72585',
  },
  soul_forest: {
    '--vmp-void': '#0a120e',
    '--vmp-surface': '#0f1f18',
    '--vmp-panel': '#152820',
    '--vmp-violet': '#3dcc8c',
    '--vmp-teal': '#7dffb3',
    '--vmp-pink': '#c4ff61',
  },
  blood_moon: {
    '--vmp-void': '#12080a',
    '--vmp-surface': '#1a0c10',
    '--vmp-panel': '#241018',
    '--vmp-violet': '#e63946',
    '--vmp-teal': '#ff6b6b',
    '--vmp-pink': '#ffd166',
  },
  soul_gold: {
    '--vmp-void': '#100e08',
    '--vmp-surface': '#1a160c',
    '--vmp-panel': '#242018',
    '--vmp-violet': '#ffd166',
    '--vmp-teal': '#ffe599',
    '--vmp-pink': '#f4a261',
  },
};

const DEFAULT_DEVICES: DeviceInfo[] = [
  { id: 'default_in', name: 'System Default Input', is_input: true, is_output: false },
  { id: 'default_out', name: 'System Default Output', is_input: false, is_output: true },
  { id: 'vibe_cable', name: 'Vibe Cable (virtual — Phase 5)', is_input: false, is_output: true },
];

const DEFAULT_MODULES: ModulePos[] = [
  { id: 'eq', x: 720, y: 80, w: 340, h: 240, z: 5, visible: true },
  { id: 'devices', x: 720, y: 340, w: 340, h: 200, z: 4, visible: false },
  { id: 'playlist', x: 60, y: 480, w: 520, h: 180, z: 3, visible: true },
  { id: 'viz', x: 600, y: 480, w: 280, h: 180, z: 3, visible: true },
  { id: 'skin', x: 400, y: 120, w: 280, h: 160, z: 6, visible: false },
];

function entryFromFile(file: File): PlaylistEntry {
  return {
    id: `${file.name}-${file.size}-${file.lastModified}`,
    name: file.name,
    title: file.name.replace(/\.[^.]+$/, ''),
    artist: 'Local file',
    format: formatLabel(file.name),
    objectUrl: URL.createObjectURL(file),
    file,
  };
}

function defaultVapForEntry(e: PlaylistEntry): VapObject {
  return {
    VAP_VERSION: '3.1',
    IDENTITY: {
      TITLE: e.title,
      ARTIST: e.artist,
      SOURCE_DNA: e.name,
    },
    PILLARS: {
      ...CANNIBAL_CORPSE_FIXTURE.PILLARS,
      STRUCTURAL: {
        TEMPORAL_DYNAMICS: {
          BPM_RAW: 120,
          GROOVE_QUANTIZATION: 'unknown',
          TIME_SIGNATURE: '4/4',
        },
      },
      GENEALOGICAL: {
        ERA_ANCHORING: { CULTURAL_ERA: 'imported', TIMELESSNESS_SCORE: 0.5 },
        DNA_SAMPLING: { GENRE_TREE: [e.format] },
        TRIBE_ALIGNMENT: { SUBCULTURE: 'general', AUTHENTICITY_SCORE: 0.5 },
      },
    },
  };
}

export default function VmpApp() {
  const [appMode, setAppMode] = useState<AppMode>('player');
  const [vap, setVap] = useState<VapObject>(CANNIBAL_CORPSE_FIXTURE);
  const [activePillar, setActivePillar] = useState<PillarId>('STRUCTURAL');
  const [editMode, setEditMode] = useState(true);
  const [playing, setPlaying] = useState(false);
  const [volume, setVolume] = useState(75);
  const [eqMode, setEqMode] = useState<EqMode>('graphic10');
  const [eqGains, setEqGains] = useState<number[]>([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  const [skin, setSkin] = useState('soul_cosmic');
  const [auraphyxBloom, setAuraphyxBloom] = useState(true);
  const [auraphyxGlitch, setAuraphyxGlitch] = useState(false);
  const [auraphyxLattice, setAuraphyxLattice] = useState(true);
  const [selectedIn, setSelectedIn] = useState('default_in');
  const [selectedOut, setSelectedOut] = useState('default_out');
  const [sampleRate, setSampleRate] = useState(48000);
  const [bufferSize, setBufferSize] = useState(256);
  const [jsonError, setJsonError] = useState<string | null>(null);
  const [playlist, setPlaylist] = useState<PlaylistEntry[]>([]);
  const [currentId, setCurrentId] = useState<string | null>(null);
  const [recent, setRecent] = useState<string[]>([]);
  const [status, setStatus] = useState('Ready — MP3 · M4A · FLAC · OGG · WAV · MP4 · …');
  const [modules, setModules] = useState<ModulePos[]>(DEFAULT_MODULES);
  const [dirty, setDirty] = useState(false);
  const [progress, setProgress] = useState(0);
  const [duration, setDuration] = useState(0);

  // Vinyl Vibez
  const [deckAId, setDeckAId] = useState<string | null>(null);
  const [deckBId, setDeckBId] = useState<string | null>(null);
  const [playA, setPlayA] = useState(false);
  const [playB, setPlayB] = useState(false);
  const [pitchA, setPitchA] = useState(0);
  const [pitchB, setPitchB] = useState(0);
  const [crossfader, setCrossfader] = useState(0.5);
  const [loadTarget, setLoadTarget] = useState<'A' | 'B' | null>(null);
  const [nativePath, setNativePath] = useState<string | null>(null);
  const [nativeBackend, setNativeBackend] = useState<string>(
    isNative() ? 'tauri' : 'browser',
  );
  const [canEmbed, setCanEmbed] = useState(false);

  const audioRef = useRef<HTMLAudioElement | null>(null);
  const fileRef = useRef<HTMLInputElement>(null);
  const manyRef = useRef<HTMLInputElement>(null);
  const folderRef = useRef<HTMLInputElement>(null);

  const current = useMemo(
    () => playlist.find((p) => p.id === currentId) ?? null,
    [playlist, currentId],
  );
  const deckA = useMemo(() => playlist.find((p) => p.id === deckAId) ?? null, [playlist, deckAId]);
  const deckB = useMemo(() => playlist.find((p) => p.id === deckBId) ?? null, [playlist, deckBId]);
  const pillarData = vap.PILLARS[activePillar];

  const applySkin = useCallback((id: string) => {
    setSkin(id);
    const tokens = SKINS[id];
    if (!tokens) return;
    const root = document.documentElement;
    for (const [k, v] of Object.entries(tokens)) root.style.setProperty(k, v);
  }, []);

  const pushRecent = (name: string) => {
    setRecent((r) => [name, ...r.filter((x) => x !== name)].slice(0, 20));
  };

  useEffect(() => {
    if (!isNative()) return;
    void nativeVersion().then((v) => {
      if (v) setStatus('Native shell · Symphonia + cpal + lofty VAP');
    });
    const id = window.setInterval(() => {
      void nativeStatus().then((s) => {
        if (!s) return;
        setProgress(s.position_sec);
        setDuration(s.duration_sec);
        setPlaying(s.playing);
        setNativeBackend(s.backend);
        if (s.ended) setPlaying(false);
      });
    }, 200);
    return () => clearInterval(id);
  }, []);

  const ingestFiles = (files: FileList | File[], replace: boolean) => {
    const list = Array.from(files).filter((f) => isMediaFile(f.name));
    if (list.length === 0) {
      setStatus('No supported media in selection');
      return;
    }
    const entries = list.map(entryFromFile);
    setPlaylist((prev) => (replace ? entries : [...prev, ...entries]));
    const first = entries[0];
    setCurrentId(first.id);
    setVap(defaultVapForEntry(first));
    setDirty(true);
    list.forEach((f) => pushRecent(f.name));
    setStatus(`Loaded ${entries.length} file(s) · ${first.format}`);
    setPlaying(false);
  };

  // HTML5 audio element for browser playback (Tauri/native uses Symphonia path)
  useEffect(() => {
    const a = new Audio();
    a.preload = 'metadata';
    audioRef.current = a;
    const onTime = () => {
      setProgress(a.currentTime);
      setDuration(a.duration || 0);
    };
    const onEnded = () => {
      setPlaying(false);
      // auto-next
      setPlaylist((pl) => {
        const idx = pl.findIndex((p) => p.id === currentId);
        if (idx >= 0 && idx < pl.length - 1) {
          setCurrentId(pl[idx + 1].id);
          setPlaying(true);
        }
        return pl;
      });
    };
    a.addEventListener('timeupdate', onTime);
    a.addEventListener('loadedmetadata', onTime);
    a.addEventListener('ended', onEnded);
    return () => {
      a.pause();
      a.removeEventListener('timeupdate', onTime);
      a.removeEventListener('loadedmetadata', onTime);
      a.removeEventListener('ended', onEnded);
    };
  }, []);

  useEffect(() => {
    if (!current) return;

    // Native-sourced entries (from Open File or the folder browser) carry
    // their absolute path in objectUrl and have no browser File object —
    // load the real decoded status + VASP data via the native engine
    // instead of feeding a path string to the HTML5 <audio> element.
    if (!current.file && isNative()) {
      void nativeOpenFile(current.objectUrl).then((res) => {
        if (!res) return;
        setNativePath(current.objectUrl);
        setVap(res.vap as VapObject);
        setCanEmbed(res.can_embed);
        setDuration(res.status.duration_sec);
        setProgress(0);
        setNativeBackend(res.status.backend);
        if (playing) void nativePlay();
      });
      return;
    }

    const a = audioRef.current;
    if (!a) return;
    a.src = current.objectUrl;
    a.load();
    setVap(defaultVapForEntry(current));
    if (playing) void a.play().catch(() => setStatus('Playback blocked or format needs native decoder'));
  }, [currentId]);

  useEffect(() => {
    if (isNative()) {
      void nativeSetVolume(volume / 100);
      if (playing) void nativePlay();
      else void nativePause();
      return;
    }
    const a = audioRef.current;
    if (!a) return;
    a.volume = volume / 100;
    if (playing) void a.play().catch(() => setPlaying(false));
    else a.pause();
  }, [playing, volume]);

  /** Native track browser: pick a folder, list its media files via the
   * real scan_folder engine, and queue them (real metadata loads lazily
   * per-track when selected — see the currentId effect above). */
  const openNativeFolder = async () => {
    const dir = await nativePickFolder();
    if (!dir) return;
    const paths = await nativeOpenFolder(dir);
    if (!paths || paths.length === 0) {
      setStatus('No supported media found in folder');
      return;
    }
    const entries: PlaylistEntry[] = paths.map((path) => {
      const name = path.split(/[/\\]/).pop() || path;
      return {
        id: path,
        name,
        title: name.replace(/\.[^.]+$/, ''),
        artist: 'Local file',
        format: formatLabel(name),
        objectUrl: path,
      };
    });
    setPlaylist((p) => [...p, ...entries]);
    setCurrentId(entries[0].id);
    setPlaying(false);
    entries.forEach((e) => pushRecent(e.name));
    setStatus(`Loaded ${entries.length} file(s) from ${dir}`);
  };

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;
      if (e.ctrlKey && e.key.toLowerCase() === 'o' && !e.shiftKey) {
        e.preventDefault();
        fileRef.current?.click();
      }
      if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === 'o') {
        e.preventDefault();
        manyRef.current?.click();
      }
      if (e.ctrlKey && e.key.toLowerCase() === 'f') {
        e.preventDefault();
        if (isNative()) void openNativeFolder();
        else folderRef.current?.click();
      }
      if (e.code === 'Space') {
        e.preventDefault();
        setPlaying((p) => !p);
      }
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, []);

  const onPillarJson = (raw: string) => {
    try {
      const parsed = JSON.parse(raw);
      setVap((prev) => ({
        ...prev,
        PILLARS: { ...prev.PILLARS, [activePillar]: parsed },
      }));
      setJsonError(null);
      setDirty(true);
    } catch (e) {
      setJsonError((e as Error).message);
    }
  };

  const exportVap = async () => {
    if (isNative() && nativePath) {
      await nativeSetVap(vap);
      const report = await nativeSaveVap(canEmbed);
      setDirty(false);
      setStatus(
        report
          ? `Native VAP save: ${JSON.stringify(report)}`
          : 'Native VAP save requested',
      );
      return;
    }
    const blob = new Blob([JSON.stringify(vap, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${vap.IDENTITY.TITLE.replace(/\s+/g, '_')}.vap.json`;
    a.click();
    URL.revokeObjectURL(url);
    setDirty(false);
    setStatus(`Exported sidecar ${a.download}`);
  };

  const openNativeOrBrowser = async () => {
    if (isNative()) {
      const path = await nativePickFile();
      if (!path) return;
      const res = await nativeOpenFile(path);
      if (!res) return;
      setNativePath(path);
      setVap(res.vap as VapObject);
      setCanEmbed(res.can_embed);
      setDuration(res.status.duration_sec);
      setProgress(0);
      setNativeBackend(res.status.backend);
      pushRecent(path.split(/[/\\]/).pop() || path);
      setPlaylist((p) => [
        ...p,
        {
          id: path,
          name: path,
          title: (res.vap as VapObject).IDENTITY.TITLE,
          artist: (res.vap as VapObject).IDENTITY.ARTIST,
          format: res.format,
          objectUrl: path,
        },
      ]);
      setCurrentId(path);
      setStatus(`Opened ${res.format} · ${res.vap_source} · ${res.status.backend}`);
      return;
    }
    fileRef.current?.click();
  };

  const savePlaylistM3u = () => {
    let body = '#EXTM3U\n#PLAYLIST:Vibe Now Playing\n';
    for (const t of playlist) {
      body += `#EXTINF:-1,${t.artist} - ${t.title}\n${t.name}\n`;
    }
    const blob = new Blob([body], { type: 'audio/x-mpegurl' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'vibe_playlist.m3u';
    a.click();
    URL.revokeObjectURL(url);
    setStatus('Playlist saved as vibe_playlist.m3u');
  };

  const handleFileMenu = (id: FileMenuId) => {
    switch (id) {
      case 'open_file':
        void openNativeOrBrowser();
        break;
      case 'open_many':
        manyRef.current?.click();
        break;
      case 'open_folder':
        if (isNative()) void openNativeFolder();
        else folderRef.current?.click();
        break;
      case 'open_disc':
        setStatus('Open Disc — CDDA backend staged for native (Phase 1.5)');
        break;
      case 'open_recent':
        break;
      case 'stream': {
        const url = window.prompt('Stream URL (http/https/rtsp):');
        if (url) {
          const entry: PlaylistEntry = {
            id: `stream-${Date.now()}`,
            name: url,
            title: url,
            artist: 'Network Stream',
            format: 'STREAM',
            objectUrl: url,
          };
          setPlaylist((p) => [...p, entry]);
          setCurrentId(entry.id);
          setStatus(`Stream: ${url}`);
        }
        break;
      }
      case 'convert_export':
        setStatus(
          `Convert/Export queued for ${current?.name ?? '—'} → choose target in native pipeline (ffmpeg/symphonia)`,
        );
        break;
      case 'create_playlist':
        setPlaylist([]);
        setCurrentId(null);
        setStatus('Created empty playlist');
        break;
      case 'save_playlist':
        savePlaylistM3u();
        break;
      case 'edit_playlist':
        setModVisible('playlist', true);
        setStatus('Edit playlist module focused');
        break;
      case 'open_network':
        setStatus('Open Network Device — UPnP/DLNA discovery staged');
        break;
      case 'save_quit':
        exportVap();
        setStatus('Saved VAP — browser shell stays open (native Quit via Tauri)');
        break;
      case 'quit':
        setPlaying(false);
        audioRef.current?.pause();
        setStatus('Quit requested (close window / Tauri app.exit)');
        break;
    }
  };

  const setModVisible = (id: string, visible: boolean) => {
    setModules((ms) => ms.map((m) => (m.id === id ? { ...m, visible } : m)));
  };

  const moveMod = (id: string, x: number, y: number) => {
    setModules((ms) => {
      const maxZ = Math.max(...ms.map((m) => m.z));
      return ms.map((m) => (m.id === id ? { ...m, x, y, z: maxZ + 1 } : m));
    });
  };

  const resizeMod = (id: string, w: number, h: number) => {
    setModules((ms) => ms.map((m) => (m.id === id ? { ...m, w, h } : m)));
  };

  const focusMod = (id: string) => {
    setModules((ms) => {
      const maxZ = Math.max(...ms.map((m) => m.z));
      return ms.map((m) => (m.id === id ? { ...m, z: maxZ + 1 } : m));
    });
  };

  const switchMode = (mode: AppMode) => {
    setAppMode(mode);
    setStatus(
      mode === 'vinyl'
        ? 'Vinyl Vibez active — dual decks · Mixxx-class roadmap (v01d mode VinylVibez)'
        : 'Vibe Media Player — modular shell',
    );
  };

  const seek = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!duration) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const pct = Math.min(1, Math.max(0, (e.clientX - rect.left) / rect.width));
    const t = pct * duration;
    if (isNative()) {
      void nativeSeek(t);
      setProgress(t);
      return;
    }
    const a = audioRef.current;
    if (!a) return;
    a.currentTime = t;
  };

  const mod = (id: string) => modules.find((m) => m.id === id)!;

  return (
    <div className="vmp-root">
      <input
        ref={fileRef}
        type="file"
        accept={MEDIA_ACCEPT}
        hidden
        onChange={(e) => e.target.files && ingestFiles(e.target.files, true)}
      />
      <input
        ref={manyRef}
        type="file"
        accept={MEDIA_ACCEPT}
        multiple
        hidden
        onChange={(e) => e.target.files && ingestFiles(e.target.files, false)}
      />
      <input
        ref={folderRef}
        type="file"
        // @ts-expect-error webkitdirectory is non-standard but widely supported
        webkitdirectory=""
        multiple
        hidden
        onChange={(e) => e.target.files && ingestFiles(e.target.files, true)}
      />

      <header className="vmp-titlebar">
        <div className="vmp-logo">
          <div className="vmp-logo-mark" aria-hidden />
          <div>
            <div className="vmp-logo-text">
              {appMode === 'vinyl' ? 'Vinyl Vibez' : 'Vibe Media Player'}
            </div>
            <div className="vmp-version">
              V.A.P. v3.1 · {nativeBackend} · v01d · {dirty ? '● unsaved meta' : 'meta ok'}
            </div>
          </div>
        </div>

        <FileMenuBar
          recent={recent}
          onAction={handleFileMenu}
          onRecent={(name) => setStatus(`Recent: ${name} — re-open via Open File`)}
        />

        <div className="vmp-titlebar-controls">
          <button
            type="button"
            className={`vmp-tb-btn ${appMode === 'player' ? 'active' : ''}`}
            onClick={() => switchMode('player')}
          >
            PLAYER
          </button>
          <button
            type="button"
            className={`vmp-tb-btn ${appMode === 'vinyl' ? 'active' : ''}`}
            onClick={() => switchMode('vinyl')}
          >
            VINYL VIBEZ
          </button>
          <button
            type="button"
            className={`vmp-tb-btn ${mod('eq')?.visible ? 'active' : ''}`}
            onClick={() => setModVisible('eq', !mod('eq')?.visible)}
          >
            EQ
          </button>
          <button
            type="button"
            className={`vmp-tb-btn ${mod('devices')?.visible ? 'active' : ''}`}
            onClick={() => setModVisible('devices', !mod('devices')?.visible)}
          >
            DEVICES
          </button>
          <button
            type="button"
            className={`vmp-tb-btn ${editMode ? 'active' : ''}`}
            onClick={() => setEditMode((e) => !e)}
          >
            META EDIT
          </button>
          <button
            type="button"
            className={`vmp-tb-btn ${mod('skin')?.visible ? 'active' : ''}`}
            onClick={() => setModVisible('skin', !mod('skin')?.visible)}
          >
            SKIN
          </button>
          <button type="button" className="vmp-tb-btn" onClick={() => void exportVap()}>
            SAVE VAP
          </button>
        </div>
      </header>

      {appMode === 'vinyl' ? (
        <div className="vmp-body vinyl-body">
          <VinylVibez
            deckA={deckA}
            deckB={deckB}
            playA={playA}
            playB={playB}
            pitchA={pitchA}
            pitchB={pitchB}
            crossfader={crossfader}
            onPlayA={() => setPlayA((p) => !p)}
            onPlayB={() => setPlayB((p) => !p)}
            onPitchA={setPitchA}
            onPitchB={setPitchB}
            onCross={setCrossfader}
            onLoadA={() => {
              setLoadTarget('A');
              setStatus('Click a library track to load Deck A');
            }}
            onLoadB={() => {
              setLoadTarget('B');
              setStatus('Click a library track to load Deck B');
            }}
            library={playlist}
            onLibraryClick={(id) => {
              if (loadTarget === 'B') {
                setDeckBId(id);
                setLoadTarget(null);
                setStatus('Loaded Deck B');
              } else {
                setDeckAId(id);
                setLoadTarget(null);
                setStatus('Loaded Deck A');
              }
            }}
          />
        </div>
      ) : (
        <div className="vmp-body modular">
          <VerticalPillarTabs active={activePillar} onSelect={setActivePillar} />

          <main className="vmp-main-col">
            <div className="track-strip">
              <div>
                <div className="track-title">{current?.title ?? vap.IDENTITY.TITLE}</div>
                <div className="track-artist">
                  {current?.artist ?? vap.IDENTITY.ARTIST}
                  {current ? ` · ${current.format}` : ' · demo fixture'}
                </div>
              </div>
              <div className="track-strip-meta">
                <span>
                  {PILLAR_TABS.find((t) => t.id === activePillar)?.short} · editor
                </span>
                <span>
                  {sampleRate / 1000}k · buf {bufferSize}
                </span>
                <span>{status}</span>
              </div>
            </div>

            <PillarPanel
              pillarId={activePillar}
              data={pillarData}
              editMode={editMode}
              onChangeJson={onPillarJson}
            />
            {jsonError && <p className="json-error">JSON: {jsonError}</p>}
            <p className="format-hint">
              Formats: MP3, M4A/AAC, FLAC, Ogg Vorbis, Opus, WAV, AIFF, MP4/M4V · VAP sidecar always ·
              native tag embed (ID3 / Vorbis / MP4) via <code>vmp-audio</code> + Tauri
            </p>
          </main>

          {/* Floating modules — drag titlebars like WinAmp/VLC */}
          <FloatingModule
            mod={mod('eq')}
            title="Equalizer"
            onMove={moveMod}
            onResize={resizeMod}
            onFocus={focusMod}
            onClose={(id) => setModVisible(id, false)}
          >
            <EqRack
              gains={eqGains}
              mode={eqMode}
              onGain={(i, v) => {
                setEqGains((g) => {
                  const n = [...g];
                  while (n.length <= i) n.push(0);
                  n[i] = v;
                  return n;
                });
                void nativeSetEqBand(i, v);
              }}
              onMode={(m) => {
                setEqMode(m);
                if (m === 'graphic31' && eqGains.length < 31) {
                  setEqGains((g) => [...g, ...Array(31 - g.length).fill(0)]);
                }
              }}
            />
          </FloatingModule>

          <FloatingModule
            mod={mod('devices')}
            title="Devices · I/O"
            onMove={moveMod}
            onResize={resizeMod}
            onFocus={focusMod}
            onClose={(id) => setModVisible(id, false)}
          >
            <DeviceMatrixPanel
              inputs={DEFAULT_DEVICES.filter((d) => d.is_input)}
              outputs={DEFAULT_DEVICES.filter((d) => d.is_output)}
              selectedInput={selectedIn}
              selectedOutput={selectedOut}
              sampleRate={sampleRate}
              bufferSize={bufferSize}
              onInput={setSelectedIn}
              onOutput={setSelectedOut}
              onSampleRate={setSampleRate}
              onBufferSize={setBufferSize}
            />
          </FloatingModule>

          <FloatingModule
            mod={mod('playlist')}
            title="Playlist"
            onMove={moveMod}
            onResize={resizeMod}
            onFocus={focusMod}
            onClose={(id) => setModVisible(id, false)}
          >
            <div className="pl-module">
              {playlist.length === 0 ? (
                <p className="pillar-empty">File → Open File / Open Many / Open Folder</p>
              ) : (
                playlist.map((t) => (
                  <button
                    key={t.id}
                    type="button"
                    className={`pl-row ${t.id === currentId ? 'active' : ''}`}
                    onClick={() => {
                      setCurrentId(t.id);
                      setPlaying(true);
                    }}
                  >
                    <span className="pl-t">{t.title}</span>
                    <span className="pl-f">{t.format}</span>
                  </button>
                ))
              )}
            </div>
          </FloatingModule>

          <FloatingModule
            mod={mod('viz')}
            title="Auraphyx"
            onMove={moveMod}
            onResize={resizeMod}
            onFocus={focusMod}
            onClose={(id) => setModVisible(id, false)}
            headerActions={
              <>
                <button
                  type="button"
                  title="Toggle bloom"
                  className={auraphyxBloom ? 'active' : ''}
                  onClick={() => setAuraphyxBloom((v) => !v)}
                >
                  B
                </button>
                <button
                  type="button"
                  title="Toggle glitch"
                  className={auraphyxGlitch ? 'active' : ''}
                  onClick={() => setAuraphyxGlitch((v) => !v)}
                >
                  G
                </button>
                <button
                  type="button"
                  title="Toggle Auraphyx lattice"
                  className={auraphyxLattice ? 'active' : ''}
                  onClick={() => setAuraphyxLattice((v) => !v)}
                >
                  L
                </button>
              </>
            }
          >
            <AuraphyxCanvas
              toggles={{
                bloomEnabled: auraphyxBloom,
                visualNoise: auraphyxGlitch,
                auraphyxEnabled: auraphyxLattice,
              }}
            />
          </FloatingModule>

          <FloatingModule
            mod={mod('skin')}
            title="Skinz"
            onMove={moveMod}
            onResize={resizeMod}
            onFocus={focusMod}
            onClose={(id) => setModVisible(id, false)}
          >
            <div className="skin-presets">
              {Object.keys(SKINS).map((id) => (
                <button
                  key={id}
                  type="button"
                  className={`skin-chip ${skin === id ? 'active' : ''}`}
                  onClick={() => applySkin(id)}
                >
                  {id.replace(/_/g, ' ')}
                </button>
              ))}
            </div>
          </FloatingModule>
        </div>
      )}

      <footer className="vmp-transport">
        <button
          type="button"
          className="ctrl-btn"
          onClick={() => {
            const idx = playlist.findIndex((p) => p.id === currentId);
            if (idx > 0) setCurrentId(playlist[idx - 1].id);
          }}
        >
          ⏮
        </button>
        <button
          type="button"
          className="ctrl-btn play-btn"
          onClick={() => {
            if (isNative()) {
              void nativeToggle().then((s) => s && setPlaying(s.playing));
            } else {
              setPlaying((p) => !p);
            }
          }}
        >
          {playing ? '⏸' : '▶'}
        </button>
        <button
          type="button"
          className="ctrl-btn"
          onClick={() => {
            const idx = playlist.findIndex((p) => p.id === currentId);
            if (idx >= 0 && idx < playlist.length - 1) setCurrentId(playlist[idx + 1].id);
          }}
        >
          ⏭
        </button>
        <div className="progress-mock" onClick={seek}>
          <div
            className="progress-fill"
            style={{ width: duration ? `${(progress / duration) * 100}%` : '0%' }}
          />
        </div>
        <span className="time-readout">
          {fmtTime(progress)} / {fmtTime(duration)}
        </span>
        <label className="vol-wrap">
          🔈
          <input
            type="range"
            min={0}
            max={100}
            value={volume}
            onChange={(e) => setVolume(parseInt(e.target.value, 10))}
          />
          <span>{volume}%</span>
        </label>
        <span className="transport-note">{status}</span>
      </footer>
    </div>
  );
}

function fmtTime(s: number): string {
  if (!Number.isFinite(s) || s < 0) return '0:00';
  const m = Math.floor(s / 60);
  const sec = Math.floor(s % 60);
  return `${m}:${sec.toString().padStart(2, '0')}`;
}
