import type { ReactNode } from "react";
import { Link } from "@tanstack/react-router";
import { useCurrentUserState } from "@/lib/auth/use-current-user";
import { UserButton } from "@/lib/auth/gates";
import { BottomSheet } from "@/components/sheets";
import { Button } from "@/components/ui/button";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";
import { usePlayer } from "@/lib/store";

function Row({
  label,
  hint,
  children,
}: {
  label: string;
  hint?: string;
  children: ReactNode;
}) {
  return (
    <div className="flex items-center justify-between gap-4 py-3">
      <div className="min-w-0">
        <p className="text-sm font-medium text-fg">{label}</p>
        {hint ? <p className="text-pretty text-xs leading-relaxed text-muted">{hint}</p> : null}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

function AccountSlot() {
  const { user, isPending } = useCurrentUserState();
  if (isPending) {
    return <div className="h-11 animate-pulse rounded-full bg-white/8" />;
  }
  if (user) {
    return <UserButton />;
  }
  return (
    <>
      <p className="mb-3 text-pretty text-sm leading-relaxed text-muted">
        Optional. Sign-in is not required to play local audio.
      </p>
      <Button asChild variant="secondary" className="w-full">
        <Link to="/login">Sign in</Link>
      </Button>
    </>
  );
}

export function SettingsSheet() {
  const sheet = usePlayer((s) => s.sheet);
  const setSheet = usePlayer((s) => s.setSheet);
  const settings = usePlayer((s) => s.settings);
  const setSetting = usePlayer((s) => s.setSetting);
  const resetSettings = usePlayer((s) => s.resetSettings);
  const volume = usePlayer((s) => s.volume);
  const setVolume = usePlayer((s) => s.setVolume);

  return (
    <BottomSheet
      open={sheet === "settings"}
      onOpenChange={(o) => setSheet(o ? "settings" : "none")}
      eyebrow="Scene"
      title="Visualizer settings"
      height="mid"
    >
      <div className="divide-y divide-white/8">
        <div className="py-3">
          <p className="mb-2 text-sm font-medium text-fg">Output level</p>
          <Slider
            min={0}
            max={1}
            step={0.01}
            value={[volume]}
            onValueChange={([v]) => setVolume(v ?? 0)}
            aria-label="Volume"
          />
        </div>
        <div className="py-3">
          <p className="mb-2 text-sm font-medium text-fg">Color intensity</p>
          <Slider
            min={0.2}
            max={1}
            step={0.01}
            value={[settings.colorIntensity]}
            onValueChange={([v]) => setSetting("colorIntensity", v ?? 0.8)}
            aria-label="Color intensity"
          />
        </div>
        <div className="py-3">
          <p className="mb-2 text-sm font-medium text-fg">Motion intensity</p>
          <Slider
            min={0}
            max={1}
            step={0.01}
            value={[settings.motionIntensity]}
            onValueChange={([v]) => setSetting("motionIntensity", v ?? 0.8)}
            aria-label="Motion intensity"
          />
        </div>
        <Row label="Particle field" hint="Soft reactive dust around the orb">
          <Switch
            checked={settings.particles}
            onCheckedChange={(v) => setSetting("particles", v)}
            aria-label="Particle effects"
          />
        </Row>
        <Row label="Spectrum rings" hint="Frequency bars around the core">
          <Switch
            checked={settings.spectrum}
            onCheckedChange={(v) => setSetting("spectrum", v)}
            aria-label="Spectrum bars"
          />
        </Row>
        <Row label="Beat pulse" hint="Kick-locked orb scale">
          <Switch
            checked={settings.beatPulse}
            onCheckedChange={(v) => setSetting("beatPulse", v)}
            aria-label="Beat pulse"
          />
        </Row>
        <Row label="Reduced motion" hint="Calmer scene, less rotation">
          <Switch
            checked={settings.reducedMotion}
            onCheckedChange={(v) => setSetting("reducedMotion", v)}
            aria-label="Reduced motion"
          />
        </Row>
        <Row label="Readable type" hint="Larger, more spaced labels">
          <Switch
            checked={settings.readableType}
            onCheckedChange={(v) => setSetting("readableType", v)}
            aria-label="Readable type"
          />
        </Row>
      </div>
      <Button asChild variant="outline" className="mt-5 w-full">
        <Link to="/brand">Canva Brand Kit</Link>
      </Button>
      <Button type="button" variant="outline" className="mt-2 w-full" onClick={resetSettings}>
        Reset to demo profile
      </Button>
      <div className="mt-6 rounded-2xl bg-white/4 p-4 shadow-border">
        <p className="mb-2 text-xs uppercase tracking-[0.16em] text-muted">Account</p>
        <AccountSlot />
      </div>
    </BottomSheet>
  );
}
