import type { ReactNode } from "react";
import {
  Pause,
  Play,
  Repeat,
  Repeat1,
  Shuffle,
  SkipBack,
  SkipForward,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { engine } from "@/lib/audio/engine";
import { usePlayer } from "@/lib/store";
import { cn, formatTime } from "@/lib/utils";

export function NowPlayingCard() {
  const track = usePlayer((s) => s.current());
  const playing = usePlayer((s) => s.playing);
  const currentTime = usePlayer((s) => s.currentTime);
  const duration = usePlayer((s) => s.duration);
  const togglePlay = usePlayer((s) => s.togglePlay);
  const next = usePlayer((s) => s.next);
  const prev = usePlayer((s) => s.prev);
  const seek = usePlayer((s) => s.seek);
  const shuffle = usePlayer((s) => s.shuffle);
  const setShuffle = usePlayer((s) => s.setShuffle);
  const repeat = usePlayer((s) => s.repeat);
  const setRepeat = usePlayer((s) => s.setRepeat);
  const mapping = usePlayer((s) => s.mapping);
  const p = track.vasp.PILLARS;
  const bpm = p.STRUCTURAL.bpm.value;
  const key = p.TONAL.key.value;
  const mood = p.AFFECTIVE.mood.value;
  const scene = p.CONTEXTUAL.scenario.value;
  const isDemo = track.kind === "demo";
  const canSeek = !isDemo && duration > 0;

  return (
    <section className="pointer-events-auto mx-auto w-full max-w-lg rounded-[28px] bg-surface/75 p-3 shadow-sheet backdrop-blur-xl">
      <div className="rounded-2xl bg-white/4 px-3 py-3">
        <div className="mb-2 flex items-start justify-between gap-3">
          <div className="min-w-0">
            <p className="truncate font-display text-lg font-semibold leading-tight tracking-tight text-fg">
              {track.title}
            </p>
            <p className="truncate text-sm text-muted">{track.artist}</p>
          </div>
          <span className="shrink-0 rounded-full bg-accent/15 px-2.5 py-1 text-[10px] font-medium uppercase tracking-[0.14em] text-accent">
            {scene ?? "Scene"}
          </span>
        </div>
        <div className="mb-3 flex flex-wrap gap-1.5">
          {bpm ? <Chip>{String(bpm)} BPM</Chip> : null}
          {key ? <Chip>{key}</Chip> : null}
          {mood ? <Chip>{mood}</Chip> : null}
        </div>
        <div className="mb-1 flex items-center gap-3">
          <span className="w-8 text-[11px] tabular-nums text-muted">
            {isDemo ? "LIVE" : formatTime(currentTime)}
          </span>
          {canSeek ? (
            <Slider
              min={0}
              max={duration}
              step={0.1}
              value={[currentTime]}
              onValueChange={([v]) => seek(v ?? 0)}
              aria-label="Seek"
            />
          ) : (
            <div className="relative h-1.5 w-full overflow-hidden rounded-full bg-white/10">
              <div
                className={cn(
                  "absolute inset-y-0 left-0 rounded-full bg-accent",
                  playing ? "w-2/3 opacity-90" : "w-1/4 opacity-50",
                )}
              />
            </div>
          )}
          <span className="w-8 text-right text-[11px] tabular-nums text-muted">
            {isDemo ? "∞" : formatTime(duration)}
          </span>
        </div>
      </div>

      <div className="mt-2 flex items-center justify-between px-1">
        <IconBtn
          label={shuffle ? "Shuffle on" : "Shuffle off"}
          onClick={() => setShuffle(!shuffle)}
          active={shuffle}
        >
          <Shuffle className="size-4" />
        </IconBtn>
        <IconBtn label="Previous" onClick={() => void prev()}>
          <SkipBack className="size-5" />
        </IconBtn>
        <Button
          type="button"
          size="play"
          variant="teal"
          aria-label={playing ? "Pause" : "Play"}
          data-testid="play-toggle"
          onClick={() => {
            engine.unlock();
            void togglePlay();
          }}
          className="shadow-glow-teal"
        >
          {playing ? (
            <Pause className="size-7 fill-current" />
          ) : (
            <Play className="ml-0.5 size-7 fill-current" />
          )}
        </Button>
        <IconBtn label="Next" onClick={() => void next()}>
          <SkipForward className="size-5" />
        </IconBtn>
        <IconBtn
          label={`Repeat ${repeat}`}
          onClick={() => setRepeat(repeat === "off" ? "all" : repeat === "all" ? "one" : "off")}
          active={repeat !== "off"}
        >
          {repeat === "one" ? <Repeat1 className="size-4" /> : <Repeat className="size-4" />}
        </IconBtn>
      </div>

      <p className="sr-only">Active mapping palette {mapping.primary} {mapping.secondary}</p>
    </section>
  );
}

function Chip({ children }: { children: ReactNode }) {
  return (
    <span className="rounded-full bg-white/6 px-2 py-0.5 text-[11px] font-medium text-fg/90">
      {children}
    </span>
  );
}

function IconBtn({
  children,
  label,
  onClick,
  active,
}: {
  children: ReactNode;
  label: string;
  onClick: () => void;
  active?: boolean;
}) {
  return (
    <button
      type="button"
      aria-label={label}
      onClick={onClick}
      className={cn(
        "grid size-11 place-items-center rounded-full text-muted transition-colors hover:bg-white/6 hover:text-fg",
        active && "text-accent",
      )}
    >
      {children}
    </button>
  );
}


