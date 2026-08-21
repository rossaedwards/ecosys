import { useEffect, useState } from "react";
import { ART } from "@/lib/art";

export function LoadingSplash() {
  const [frame, setFrame] = useState<0 | 1>(0);
  const [gone, setGone] = useState(false);
  const [hiding, setHiding] = useState(false);

  useEffect(() => {
    const a = window.setTimeout(() => setFrame(1), 700);
    const b = window.setTimeout(() => setHiding(true), 1500);
    const c = window.setTimeout(() => setGone(true), 1900);
    return () => {
      window.clearTimeout(a);
      window.clearTimeout(b);
      window.clearTimeout(c);
    };
  }, []);

  if (gone) return null;

  return (
    <div
      className={cnSplash(hiding)}
      aria-hidden={hiding}
      data-testid="loading-splash"
    >
      <img
        src={frame === 0 ? ART.loading : ART.loading2}
        alt="Vibe Audio Player"
        className="h-full w-full object-cover"
      />
    </div>
  );
}

function cnSplash(hiding: boolean) {
  return [
    "absolute inset-0 z-40 bg-bg transition-opacity duration-300",
    hiding ? "pointer-events-none opacity-0" : "opacity-100",
  ].join(" ");
}
