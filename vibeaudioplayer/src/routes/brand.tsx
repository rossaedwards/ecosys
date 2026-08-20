import { createFileRoute, Link } from "@tanstack/react-router";
import { ArrowLeft, Check, Copy, Download } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  ART,
  AUDIO_CONFIG,
  BRAND_KIT_STEPS,
  CANVA_BRAND,
  CANVA_SIZES,
  COLORS,
  COMPONENTS,
  FIRST_BATCH,
  MAGIC_PROMPTS,
  NAV_ICONS,
  PILLARS,
  SCREENS,
  SETTINGS_SECTIONS,
} from "@/lib/canva/kit";
import { cn } from "@/lib/utils";

export const Route = createFileRoute("/brand")({ component: BrandStudio });

function BrandStudio() {
  const [theme, setTheme] = useState<"dark" | "light">("dark");
  const palette = COLORS[theme];

  return (
    <main className="min-h-dvh overflow-y-auto bg-bg pb-20 text-fg">
      <header className="sticky top-0 z-20 flex items-center gap-3 border-b border-white/8 bg-bg/90 px-4 py-3 backdrop-blur-md">
        <Link
          to="/"
          className="grid size-11 place-items-center rounded-full text-muted hover:bg-white/6 hover:text-fg"
          aria-label="Back to player"
        >
          <ArrowLeft className="size-5" />
        </Link>
        <div className="min-w-0 flex-1">
          <p className="text-[11px] font-medium uppercase tracking-[0.18em] text-accent">Canva Business kit</p>
          <h1 className="truncate font-display text-lg font-semibold">{CANVA_BRAND.product}</h1>
        </div>
        <a href="/canva-kit/VASP-Canva-Brand-Kit.zip" download>
          <Button type="button" variant="teal" size="sm">
            <Download className="size-4" />
            Zip
          </Button>
        </a>
      </header>

      <div className="mx-auto flex max-w-3xl flex-col gap-10 px-4 py-6">
        <section>
          <h2 className="font-display text-2xl font-semibold tracking-tight">Short VASP summary</h2>
          <p className="mt-3 text-pretty text-base leading-relaxed text-muted">
            Vibe Audio Standard and Protocol (VASP) v{CANVA_BRAND.version} describes the experiential identity of
            music through nine connected metadata pillars. The Visualizer turns that profile into sound-reactive
            color, motion, texture, context, and playback controls. Build one coherent asset system — not a pile of
            neon widgets.
          </p>
          <p className="mt-2 text-pretty text-sm leading-relaxed text-muted">{CANVA_BRAND.style}</p>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Nine pillars</h2>
          <ol className="mt-4 flex flex-col gap-2">
            {PILLARS.map((p) => (
              <li key={p.key} className="flex items-start gap-3 rounded-2xl bg-surface p-3 shadow-border">
                <img src={p.icon} alt="" className="mt-0.5 size-8 shrink-0" />
                <div className="min-w-0">
                  <p className="font-medium text-fg">
                    {p.n}. {p.key}
                    <span className="ml-2 text-xs uppercase tracking-[0.14em] text-muted">{p.archetype}</span>
                  </p>
                  <p className="text-pretty text-sm leading-relaxed text-muted">{p.role}</p>
                </div>
                <CopyChip value={p.hex} />
              </li>
            ))}
          </ol>
        </section>

        <section>
          <div className="flex items-center justify-between gap-3">
            <h2 className="font-display text-xl font-semibold">Brand colors</h2>
            <div className="flex rounded-full bg-surface p-1 shadow-border">
              {(["dark", "light"] as const).map((t) => (
                <button
                  key={t}
                  type="button"
                  onClick={() => setTheme(t)}
                  className={cn(
                    "h-9 rounded-full px-3 text-xs font-medium capitalize",
                    theme === t ? "bg-accent text-bg" : "text-muted",
                  )}
                >
                  {t}
                </button>
              ))}
            </div>
          </div>
          <p className="mt-2 text-pretty text-sm text-muted">
            Paste these into Canva Brand Kit → Colors. Light mode is not an invert: less glow, stronger borders.
          </p>
          <ul className="mt-4 grid grid-cols-1 gap-2 sm:grid-cols-2">
            {palette.map((c) => (
              <li key={c.token} className="flex items-center gap-3 rounded-2xl bg-surface p-2.5 shadow-border">
                <span className="size-11 shrink-0 rounded-xl shadow-border" style={{ background: c.hex }} />
                <div className="min-w-0 flex-1">
                  <p className="font-medium">{c.token}</p>
                  <p className="text-xs text-muted">{c.use}</p>
                </div>
                <CopyChip value={c.hex} />
              </li>
            ))}
          </ul>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Load this into Canva Business</h2>
          <ol className="mt-4 flex list-decimal flex-col gap-3 pl-5 text-pretty text-sm leading-relaxed text-muted">
            {BRAND_KIT_STEPS.map((step) => (
              <li key={step}>{step}</li>
            ))}
          </ol>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Canva page sizes</h2>
          <div className="mt-4 overflow-x-auto rounded-2xl bg-surface shadow-border">
            <table className="w-full min-w-[28rem] text-left text-sm">
              <thead className="text-xs uppercase tracking-[0.12em] text-muted">
                <tr>
                  <th className="px-3 py-2 font-medium">Asset</th>
                  <th className="px-3 py-2 font-medium">Pixels</th>
                  <th className="px-3 py-2 font-medium">Canva preset</th>
                </tr>
              </thead>
              <tbody>
                {CANVA_SIZES.map((s) => (
                  <tr key={s.name} className="border-t border-white/8">
                    <td className="px-3 py-2">{s.name}</td>
                    <td className="px-3 py-2 tabular-nums">
                      {s.w} × {s.h}
                    </td>
                    <td className="px-3 py-2 text-muted">{s.canva}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Art to upload</h2>
          <p className="mt-2 text-pretty text-sm text-muted">
            Long-press or download, then drop into Brand Kit → Logos / Graphics. Replace any leftover model
            lettering with Syne / Lexend in Canva.
          </p>
          <div className="mt-4 grid grid-cols-2 gap-3">
            {ART.map((item) => (
              <figure key={item.title} className="overflow-hidden rounded-2xl bg-surface shadow-border">
                <img src={item.src} alt={item.title} className="aspect-square w-full object-cover sm:aspect-video" />
                <figcaption className="px-3 py-2">
                  <p className="text-sm font-medium">{item.title}</p>
                  <p className="text-xs text-muted">{item.hint}</p>
                </figcaption>
              </figure>
            ))}
          </div>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Icon family (SVG)</h2>
          <p className="mt-2 text-pretty text-sm text-muted">
            24×24 canvas, 1.75 stroke, round caps. Recolor in Canva with Brand Kit signal / teal. Pillar icons are
            in the same folder as <code className="text-accent">pillar-*</code>.
          </p>
          <div className="mt-4 grid grid-cols-4 gap-2 sm:grid-cols-6">
            {NAV_ICONS.map((name) => (
              <a
                key={name}
                href={`/canva-kit/icons/${name}.svg`}
                download
                className="flex flex-col items-center gap-1 rounded-2xl bg-surface p-3 shadow-border hover:bg-white/6"
              >
                <img src={`/canva-kit/icons/${name}.svg`} alt="" className="size-8" />
                <span className="w-full truncate text-center text-[10px] text-muted">{name}</span>
              </a>
            ))}
          </div>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Buttons, knobs, toggles</h2>
          <ul className="mt-4 flex flex-col gap-2">
            {COMPONENTS.map((c) => (
              <li key={c.name} className="rounded-2xl bg-surface p-3 shadow-border">
                <p className="font-medium">{c.name}</p>
                <p className="text-pretty text-sm leading-relaxed text-muted">{c.spec}</p>
              </li>
            ))}
          </ul>
          <div className="mt-4 flex flex-wrap items-center gap-3 rounded-3xl bg-surface p-4 shadow-border">
            <button type="button" className="h-12 rounded-full bg-primary px-5 text-sm font-medium text-fg">
              Primary 48
            </button>
            <button type="button" className="h-12 rounded-full px-5 text-sm font-medium shadow-border">
              Secondary
            </button>
            <button type="button" className="size-[72px] rounded-full bg-accent text-bg shadow-glow-teal">
              Play
            </button>
            <span className="inline-flex h-8 w-[52px] items-center rounded-full bg-accent/80 p-1">
              <span className="ml-auto size-6 rounded-full bg-fg" />
            </span>
          </div>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Screens</h2>
          <ul className="mt-4 flex flex-col gap-2">
            {SCREENS.map((s) => (
              <li key={s.name} className="rounded-2xl bg-surface p-3 shadow-border">
                <p className="font-medium">{s.name}</p>
                <p className="text-pretty text-sm text-muted">{s.assets}</p>
              </li>
            ))}
          </ul>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Audio configuration page</h2>
          <ul className="mt-4 list-disc space-y-2 pl-5 text-pretty text-sm leading-relaxed text-muted">
            {AUDIO_CONFIG.map((line) => (
              <li key={line}>{line}</li>
            ))}
          </ul>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Settings page</h2>
          <ul className="mt-4 flex flex-col gap-2">
            {SETTINGS_SECTIONS.map((s) => (
              <li key={s.name} className="rounded-2xl bg-surface p-3 shadow-border">
                <p className="font-medium">{s.name}</p>
                <p className="text-pretty text-sm text-muted">{s.items}</p>
              </li>
            ))}
          </ul>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">Magic Media prompts</h2>
          <p className="mt-2 text-pretty text-sm text-muted">
            Canva → Magic Media. Set the custom size first, paste the prompt, then overlay Brand Kit type. Never
            keep model-generated letters.
          </p>
          <div className="mt-4 flex flex-col gap-3">
            {MAGIC_PROMPTS.map((p) => (
              <article key={p.title} className="rounded-2xl bg-surface p-4 shadow-border">
                <div className="mb-2 flex items-start justify-between gap-3">
                  <div>
                    <h3 className="font-medium">{p.title}</h3>
                    <p className="text-xs text-muted">{p.size}</p>
                  </div>
                  <CopyChip value={p.prompt} label="Copy prompt" />
                </div>
                <p className="text-pretty text-sm leading-relaxed text-muted">{p.prompt}</p>
              </article>
            ))}
          </div>
        </section>

        <section>
          <h2 className="font-display text-xl font-semibold">First asset batch</h2>
          <ol className="mt-4 list-decimal space-y-2 pl-5 text-pretty text-sm leading-relaxed text-muted">
            {FIRST_BATCH.map((item) => (
              <li key={item}>{item}</li>
            ))}
          </ol>
        </section>

        <p className="text-center text-xs text-muted">
          {CANVA_BRAND.owner} · {CANVA_BRAND.player} · VASP {CANVA_BRAND.version}
        </p>
      </div>
    </main>
  );
}

function CopyChip({ value, label = "Copy" }: { value: string; label?: string }) {
  const [done, setDone] = useState(false);
  return (
    <button
      type="button"
      className="inline-flex h-9 shrink-0 items-center gap-1 rounded-full bg-white/6 px-2.5 text-[11px] font-medium tabular-nums text-fg hover:bg-white/10"
      onClick={async () => {
        try {
          await navigator.clipboard.writeText(value);
          setDone(true);
          window.setTimeout(() => setDone(false), 1400);
        } catch {
          /* ignore */
        }
      }}
    >
      {done ? <Check className="size-3.5 text-accent" /> : <Copy className="size-3.5" />}
      {done ? "Copied" : label === "Copy" ? value : label}
    </button>
  );
}
