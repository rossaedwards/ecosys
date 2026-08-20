import { Drawer } from "vaul";
import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

export function BottomSheet({
  open,
  onOpenChange,
  title,
  eyebrow,
  children,
  height = "tall",
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  eyebrow?: string;
  children: ReactNode;
  height?: "tall" | "mid";
}) {
  return (
    <Drawer.Root open={open} onOpenChange={onOpenChange}>
      <Drawer.Portal>
        <Drawer.Overlay className="fixed inset-0 z-40 bg-black/55" />
        <Drawer.Content
          className={cn(
            "fixed inset-x-0 bottom-0 z-50 mx-auto flex max-w-lg flex-col rounded-t-3xl bg-surface text-fg shadow-sheet outline-none",
            height === "tall" ? "h-[min(86dvh,720px)]" : "h-[min(64dvh,560px)]",
          )}
        >
          <div className="flex justify-center pt-3">
            <div className="h-1 w-12 rounded-full bg-white/18" />
          </div>
          <div className="px-5 pb-2 pt-3">
            {eyebrow ? (
              <p className="text-[11px] font-medium uppercase tracking-[0.18em] text-accent">{eyebrow}</p>
            ) : null}
            <Drawer.Title className="font-display text-xl font-semibold tracking-tight text-fg">
              {title}
            </Drawer.Title>
          </div>
          <div className="min-h-0 flex-1 overflow-y-auto px-5 pb-[max(1.5rem,env(safe-area-inset-bottom))]">
            {children}
          </div>
        </Drawer.Content>
      </Drawer.Portal>
    </Drawer.Root>
  );
}
