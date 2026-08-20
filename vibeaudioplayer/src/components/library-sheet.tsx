import { FolderOpen, Music2, Trash2 } from "lucide-react";
import { useRef } from "react";
import { BottomSheet } from "@/components/sheets";
import { Button } from "@/components/ui/button";
import { engine } from "@/lib/audio/engine";
import { usePlayer } from "@/lib/store";
import { cn, formatTime } from "@/lib/utils";

export function LibrarySheet() {
  const sheet = usePlayer((s) => s.sheet);
  const setSheet = usePlayer((s) => s.setSheet);
  const library = usePlayer((s) => s.library);
  const currentId = usePlayer((s) => s.currentId);
  const playTrack = usePlayer((s) => s.playTrack);
  const importFiles = usePlayer((s) => s.importFiles);
  const removeTrack = usePlayer((s) => s.removeTrack);
  const inputRef = useRef<HTMLInputElement>(null);

  return (
    <BottomSheet
      open={sheet === "library"}
      onOpenChange={(o) => setSheet(o ? "library" : "none")}
      eyebrow="Local library"
      title="Vibe Audio Player"
    >
      <input
        ref={inputRef}
        type="file"
        accept="audio/*,.mp3,.wav,.flac,.ogg,.m4a,.aac,.opus,.webm"
        multiple
        className="hidden"
        onChange={(e) => {
          if (e.target.files?.length) void importFiles(e.target.files);
          e.target.value = "";
        }}
      />
      <p className="mb-4 max-w-prose text-pretty text-sm leading-relaxed text-muted">
        Play built-in VASP demos or open audio from this device. Files stay on your phone — nothing is uploaded.
      </p>
      <Button
        type="button"
        variant="teal"
        className="mb-5 w-full"
        onClick={() => inputRef.current?.click()}
      >
        <FolderOpen />
        Open local audio
      </Button>
      <ul className="flex flex-col gap-2">
        {library.map((track) => {
          const active = track.id === currentId;
          return (
            <li key={track.id}>
              <div
                className={cn(
                  "flex items-center gap-3 rounded-2xl p-2.5 shadow-border",
                  active ? "bg-white/8" : "bg-white/3",
                )}
              >
                <button
                  type="button"
                  onClick={() => {
                    engine.unlock();
                    void playTrack(track.id);
                  }}
                  className="flex min-w-0 flex-1 items-center gap-3 text-left"
                >
                  <span
                    className={cn(
                      "grid size-11 shrink-0 place-items-center rounded-xl",
                      active ? "bg-accent/20 text-accent" : "bg-white/6 text-muted",
                    )}
                  >
                    <Music2 className="size-4" />
                  </span>
                  <span className="min-w-0">
                    <span className="block truncate font-medium text-fg">{track.title}</span>
                    <span className="block truncate text-xs text-muted">
                      {track.artist}
                      {track.kind === "demo" ? " · Demo" : ""}
                      {track.duration ? ` · ${formatTime(track.duration)}` : ""}
                    </span>
                  </span>
                </button>
                {track.kind === "file" ? (
                  <button
                    type="button"
                    aria-label={`Remove ${track.title}`}
                    className="grid size-11 place-items-center rounded-full text-muted hover:bg-white/6 hover:text-fg"
                    onClick={() => void removeTrack(track.id)}
                  >
                    <Trash2 className="size-4" />
                  </button>
                ) : null}
              </div>
            </li>
          );
        })}
      </ul>
    </BottomSheet>
  );
}
