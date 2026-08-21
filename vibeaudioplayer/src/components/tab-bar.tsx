import { ART, TAB_SLOTS } from "@/lib/art";
import { usePlayer, type SheetName } from "@/lib/store";
import { cn } from "@/lib/utils";

const TABS: {
  id: "orb" | "library" | "vasp" | "scene" | "about";
  label: string;
  sheet: SheetName;
  slot: keyof typeof TAB_SLOTS;
}[] = [
  { id: "orb", label: "Orb", sheet: "none", slot: "orb" },
  { id: "library", label: "Library", sheet: "library", slot: "library" },
  { id: "vasp", label: "VASP", sheet: "vasp", slot: "vasp" },
  { id: "scene", label: "Scene", sheet: "settings", slot: "scene" },
  { id: "about", label: "About", sheet: "about", slot: "about" },
];

export function TabBar() {
  const sheet = usePlayer((s) => s.sheet);
  const setSheet = usePlayer((s) => s.setSheet);
  const active =
    sheet === "none" ? "orb" : sheet === "settings" ? "scene" : sheet === "about" ? "about" : sheet;

  return (
    <nav
      className="pointer-events-auto absolute inset-x-0 bottom-0 z-20 border-t border-white/8 bg-surface/80 px-2 pt-1 backdrop-blur-xl"
      style={{ paddingBottom: "max(0.35rem, env(safe-area-inset-bottom))" }}
      aria-label="Player sections"
    >
      <div className="mx-auto grid max-w-lg grid-cols-5">
        {TABS.map((tab) => {
          const on = active === tab.id;
          const pos = TAB_SLOTS[tab.slot];
          return (
            <button
              key={tab.id}
              type="button"
              aria-label={tab.label}
              aria-current={on ? "page" : undefined}
              data-testid={`tab-${tab.id}`}
              onClick={() => setSheet(tab.sheet)}
              className={cn(
                "flex min-h-14 flex-col items-center justify-center gap-0.5 rounded-2xl px-1 py-1 text-[11px] font-medium tracking-wide transition-colors",
                on ? "text-accent" : "text-muted hover:text-fg",
              )}
            >
              <span
                className={cn(
                  "size-9 overflow-hidden rounded-full shadow-border",
                  on && "ring-2 ring-accent/80",
                )}
              >
                <img
                  src={ART.menu2Tb}
                  alt=""
                  className="size-full max-w-none scale-[3.4] object-cover"
                  style={{ objectPosition: `${pos.x} ${pos.y}` }}
                />
              </span>
              {tab.label}
            </button>
          );
        })}
      </div>
    </nav>
  );
}
