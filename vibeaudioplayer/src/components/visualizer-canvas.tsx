import { useEffect, useRef } from "react";
import { engine } from "@/lib/audio/engine";
import { usePlayer } from "@/lib/store";
import { drawOrb2d } from "@/lib/visualizer/orb-2d";
import { buildOrbUniforms } from "@/lib/visualizer/orb-uniforms";
import { OrbWebGL } from "@/lib/visualizer/orb-webgl";

export function VisualizerCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const mapping = usePlayer((s) => s.mapping);
  const settings = usePlayer((s) => s.settings);
  const profile = usePlayer((s) => s.current().vasp);
  const mapRef = useRef(mapping);
  const setRef = useRef(settings);
  const profileRef = useRef(profile);
  mapRef.current = mapping;
  setRef.current = settings;
  profileRef.current = profile;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    let glRenderer: OrbWebGL | null = null;
    let ctx2d: CanvasRenderingContext2D | null = null;
    try {
      glRenderer = OrbWebGL.create(canvas);
    } catch {
      glRenderer = null;
    }
    if (!glRenderer) {
      ctx2d = canvas.getContext("2d", { alpha: false });
      if (!ctx2d) return;
    }

    let raf = 0;
    let running = true;
    let cssW = 1;
    let cssH = 1;

    const resize = () => {
      const parent = canvas.parentElement ?? canvas;
      const dpr = Math.min(2, window.devicePixelRatio || 1);
      cssW = Math.max(1, parent.clientWidth);
      cssH = Math.max(1, parent.clientHeight);
      canvas.width = Math.max(1, Math.floor(cssW * dpr));
      canvas.height = Math.max(1, Math.floor(cssH * dpr));
      canvas.style.width = `${cssW}px`;
      canvas.style.height = `${cssH}px`;
      if (ctx2d) ctx2d.setTransform(dpr, 0, 0, dpr, 0, 0);
    };
    resize();
    const ro = new ResizeObserver(resize);
    ro.observe(canvas.parentElement ?? canvas);

    const loop = () => {
      if (!running) return;
      const sample = engine.sample();
      const map = mapRef.current;
      const set = setRef.current;
      const profile = profileRef.current;
      if (glRenderer) {
        const u = buildOrbUniforms(sample, map, set, profile, canvas.width, canvas.height);
        glRenderer.draw(u);
      } else if (ctx2d) {
        const u = buildOrbUniforms(sample, map, set, profile, cssW, cssH);
        drawOrb2d(ctx2d, cssW, cssH, u);
      }
      raf = requestAnimationFrame(loop);
    };
    raf = requestAnimationFrame(loop);

    const onVis = () => {
      if (document.hidden) {
        running = false;
        cancelAnimationFrame(raf);
      } else {
        running = true;
        raf = requestAnimationFrame(loop);
      }
    };
    document.addEventListener("visibilitychange", onVis);

    return () => {
      running = false;
      cancelAnimationFrame(raf);
      ro.disconnect();
      document.removeEventListener("visibilitychange", onVis);
      glRenderer?.destroy();
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 h-full w-full"
      aria-hidden
      data-testid="orb-canvas"
    />
  );
}
