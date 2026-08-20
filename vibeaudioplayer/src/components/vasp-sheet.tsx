import {
  Activity,
  AudioLines,
  Eye,
  GitBranch,
  Heart,
  MapPin,
  MessageCircle,
  Music2,
  Waves,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";
import { BottomSheet } from "@/components/sheets";
import { flattenPillar } from "@/lib/vasp/catalog";
import { PILLAR_KEYS, PILLAR_META, type PillarKey } from "@/lib/vasp/types";
import { usePlayer } from "@/lib/store";
import { cn } from "@/lib/utils";

const ICONS: Record<PillarKey, LucideIcon> = {
  STRUCTURAL: AudioLines,
  TONAL: Music2,
  TIMBRAL: Waves,
  LINGUISTIC: MessageCircle,
  AFFECTIVE: Heart,
  CONTEXTUAL: MapPin,
  PHOTOMETRIC: Eye,
  KINETIC: Activity,
  GENEALOGICAL: GitBranch,
};

export function VaspSheet() {
  const sheet = usePlayer((s) => s.sheet);
  const setSheet = usePlayer((s) => s.setSheet);
  const activePillar = usePlayer((s) => s.activePillar);
  const setPillar = usePlayer((s) => s.setPillar);
  const track = usePlayer((s) => s.current());
  const meta = PILLAR_META[activePillar];
  const fields = flattenPillar(track.vasp, activePillar);
  const Icon = ICONS[activePillar];

  return (
    <BottomSheet
      open={sheet === "vasp"}
      onOpenChange={(o) => setSheet(o ? "vasp" : "none")}
      eyebrow={`VASP ${track.vasp.VAP_VERSION}`}
      title="Nine-pillar profile"
    >
      <p className="mb-4 text-pretty text-sm leading-relaxed text-muted">
        Creative metadata for how this audio is structured, feels, appears, and moves. Visual mappings only — not medical, psychological, biometric, or scientific diagnoses.
      </p>
      <div className="mb-5 grid grid-cols-3 gap-2">
        {PILLAR_KEYS.map((key) => {
          const m = PILLAR_META[key];
          const KIcon = ICONS[key];
          const on = key === activePillar;
          return (
            <button
              key={key}
              type="button"
              onClick={() => setPillar(key)}
              className={cn(
                "flex min-h-16 flex-col items-start gap-1 rounded-2xl p-2.5 text-left shadow-border transition-colors",
                on ? "bg-white/10 text-fg" : "bg-white/3 text-muted hover:bg-white/6",
              )}
            >
              <KIcon className={cn("size-4", on ? "text-accent" : "text-muted")} />
              <span className="text-[11px] font-medium leading-tight text-fg">{m.label}</span>
            </button>
          );
        })}
      </div>
      <section className="rounded-3xl bg-white/4 p-4 shadow-border">
        <div className="mb-3 flex items-center gap-3">
          <span className="grid size-11 place-items-center rounded-2xl bg-accent/15 text-accent">
            <Icon className="size-5" />
          </span>
          <div>
            <h3 className="font-display text-lg font-semibold text-fg">{meta.label}</h3>
            <p className="text-xs uppercase tracking-[0.16em] text-muted">{meta.archetype}</p>
          </div>
        </div>
        <p className="mb-4 text-pretty text-sm leading-relaxed text-muted">{meta.purpose}</p>
        <dl className="flex flex-col gap-3">
          {fields.map((field) => (
            <div key={field.label} className="flex items-start justify-between gap-4">
              <dt className="text-xs uppercase tracking-[0.12em] text-muted">{field.label}</dt>
              <dd className="max-w-[60%] text-right text-sm font-medium text-fg">
                {field.value}
                {field.status !== "known" ? (
                  <span className="ml-2 text-[10px] uppercase tracking-wider text-muted">{field.status}</span>
                ) : null}
              </dd>
            </div>
          ))}
        </dl>
      </section>
    </BottomSheet>
  );
}
