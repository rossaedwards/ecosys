import { useEffect } from "react";
import { LibrarySheet } from "@/components/library-sheet";
import { NowPlayingCard } from "@/components/now-playing";
import { SettingsSheet } from "@/components/settings-sheet";
import { VaspSheet } from "@/components/vasp-sheet";
import { VisualizerCanvas } from "@/components/visualizer-canvas";
import { engine } from "@/lib/audio/engine";
import { usePlayer } from "@/lib/store";
import { cn } from "@/lib/utils";

export function PlayerApp() {
  const hydrate = usePlayer((s) => s.hydrate);
  const importFiles = usePlayer((s) => s.importFiles);
  const setDropActive = usePlayer((s) => s.setDropActive);
  const dropActive = usePlayer((s) => s.dropActive);
  const notice = usePlayer((s) => s.notice);
  const setNotice = usePlayer((s) => s.setNotice);
  const readable = usePlayer((s) => s.settings.readableType);
  const togglePlay = usePlayer((s) => s.togglePlay);


  useEffect(() => {
    void hydrate();
  }, [hydrate]);

  useEffect(() => {
    const unlock = () => engine.unlock();
    window.addEventListener("pointerdown", unlock);
    return () => window.removeEventListener("pointerdown", unlock);
  }, []);

  useEffect(() => {
    if (!notice) return;
    const t = window.setTimeout(() => setNotice(null), 4200);
    return () => window.clearTimeout(t);
  }, [notice, setNotice]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const tag = (e.target as HTMLElement | null)?.tagName;
      if (tag === "INPUT" || tag === "TEXTAREA") return;
      if (e.code === "Space") {
        e.preventDefault();
        void togglePlay();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [togglePlay]);

  return (
    <div
      className={cn("relative h-dvh w-full overflow-hidden bg-bg text-fg", readable && "readable-type")}
      onDragOver={(e) => {
        e.preventDefault();
        setDropActive(true);
      }}
      onDragLeave={() => setDropActive(false)}
      onDrop={(e) => {
        e.preventDefault();
        setDropActive(false);
        if (e.dataTransfer.files.length) void importFiles(e.dataTransfer.files);
      }}
    >
      <VisualizerCanvas />

      <header className="pointer-events-none absolute inset-x-0 top-0 z-10 flex items-start justify-between px-4 pt-[max(0.9rem,env(safe-area-inset-top))]">
        <div className="pointer-events-auto">
          <p className="font-display text-[11px] font-semibold uppercase tracking-[0.28em] text-accent">
            Aurphyx
          </p>
          <h1 className="font-display text-xl font-semibold tracking-tight text-fg">Vibe Audio Player</h1>
          <p className="text-[11px] uppercase tracking-[0.16em] text-muted">VASP 3.69</p>
        </div>
      </header>

      <div className="pointer-events-none absolute inset-x-0 bottom-0 z-10 px-3 pb-[max(3.75rem,calc(env(safe-area-inset-bottom)+2.25rem))]">
        <NowPlayingCard />
      </div>

      {dropActive ? (
        <div className="absolute inset-0 z-30 grid place-items-center bg-bg/70 backdrop-blur-sm">
          <div className="rounded-3xl bg-surface px-8 py-6 text-center shadow-sheet">
            <p className="font-display text-lg font-semibold">Drop audio to add</p>
            <p className="mt-1 text-sm text-muted">MP3, WAV, FLAC, OGG, M4A</p>
          </div>
        </div>
      ) : null}

      {notice ? (
        <div className="absolute inset-x-0 top-20 z-20 mx-auto w-[min(92%,24rem)] rounded-2xl bg-surface px-4 py-3 text-center text-sm text-fg shadow-sheet">
          {notice}
        </div>
      ) : null}

      <LibrarySheet />
      <VaspSheet />
      <SettingsSheet />
    </div>
  );
}
