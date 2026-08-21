import { useState } from "react";
import { BottomSheet } from "@/components/sheets";
import { ART } from "@/lib/art";
import { usePlayer } from "@/lib/store";

export function AboutSheet() {
  const sheet = usePlayer((s) => s.sheet);
  const setSheet = usePlayer((s) => s.setSheet);
  const [taps, setTaps] = useState(0);
  const unlocked = taps >= 13;

  const bump = () => setTaps((n) => Math.min(13, n + 1));

  return (
    <BottomSheet
      open={sheet === "about"}
      onOpenChange={(o) => {
        setSheet(o ? "about" : "none");
        if (!o) setTaps(0);
      }}
      eyebrow="Aurphyx LLC"
      title="About Aurphyx"
      onTitleClick={bump}
    >
      <button type="button" onClick={bump} className="w-full text-left" data-testid="about-page">
        <img
          src={ART.icon}
          alt="Vibe Audio Player"
          className="mx-auto mb-4 size-24 rounded-full object-cover shadow-glow"
        />
        <p className="text-pretty text-sm leading-relaxed text-muted">
          Aurphyx LLC — Ross A. Edwards. Vibe Audio Player is a local music surface for the V.A.P.
          nine-pillar TSLCA orb. Files stay on this device. The Orb tab is the same renderer language
          as the VLC cymatic plugin: PCM → chromatic bands → photometric hex → bloom.
        </p>
      </button>
      {unlocked ? (
        <div className="mt-6 rounded-3xl bg-white/4 p-4 text-center shadow-border">
          <p className="text-[11px] uppercase tracking-[0.18em] text-accent">Business card</p>
          <p className="mt-1 font-display text-lg font-semibold text-fg">Ross A. Edwards</p>
          <p className="text-sm text-muted">Founder, Aurphyx LLC</p>
          <img
            src={ART.qr}
            alt="Aurphyx business card QR code"
            className="mx-auto mt-4 w-[min(100%,14rem)] rounded-2xl bg-fg p-2"
            data-testid="aurphyx-qr"
          />
          <p className="mt-3 text-xs text-muted">Scan for vCard · GitHub + LinkedIn</p>
        </div>
      ) : taps >= 8 ? (
        <p className="mt-6 text-center text-xs text-muted">…</p>
      ) : null}
    </BottomSheet>
  );
}
