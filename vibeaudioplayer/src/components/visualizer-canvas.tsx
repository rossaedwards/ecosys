import { useEffect, useRef } from "react";
import { engine } from "@/lib/audio/engine";
import { usePlayer } from "@/lib/store";
import { VisualizerScene, drawFrame } from "@/lib/visualizer/draw";

export function VisualizerCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const mapping = usePlayer((s) => s.mapping);
  const settings = usePlayer((s) => s.settings);
  const mapRef = useRef(mapping);
  const setRef = useRef(settings);
  mapRef.current = mapping;
  setRef.current = settings;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d", { alpha: false });
    if (!ctx) return;
    const scene = new VisualizerScene();
    let raf = 0;
    let last = performance.now();
    let running = true;

    const resize = () => {
      const parent = canvas.parentElement ?? canvas;
      const dpr = Math.min(2, window.devicePixelRatio || 1);
      const w = parent.clientWidth;
      const h = parent.clientHeight;
      canvas.width = Math.max(1, Math.floor(w * dpr));
      canvas.height = Math.max(1, Math.floor(h * dpr));
      canvas.style.width = `${w}px`;
      canvas.style.height = `${h}px`;
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      scene.resize(w, h);
    };
    resize();
    const ro = new ResizeObserver(resize);
    ro.observe(canvas.parentElement ?? canvas);

    const loop = (now: number) => {
      if (!running) return;
      const dt = Math.min(0.05, (now - last) / 1000);
      last = now;
      const sample = engine.sample();
      const map = mapRef.current;
      const set = setRef.current;
      scene.tick(dt, sample, map, set);
      drawFrame(ctx, scene.width, scene.height, sample, map, set, scene);
      raf = requestAnimationFrame(loop);
    };
    raf = requestAnimationFrame(loop);

    const onVis = () => {
      if (document.hidden) {
        running = false;
        cancelAnimationFrame(raf);
      } else {
        running = true;
        last = performance.now();
        raf = requestAnimationFrame(loop);
      }
    };
    document.addEventListener("visibilitychange", onVis);

    return () => {
      running = false;
      cancelAnimationFrame(raf);
      ro.disconnect();
      document.removeEventListener("visibilitychange", onVis);
    };
  }, []);

  return <canvas ref={canvasRef} className="absolute inset-0 h-full w-full" aria-hidden />;
}
