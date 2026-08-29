import { useEffect, useRef } from 'react';
import { nativeListen } from '../bridge/native';
import { AuraphyxRenderer } from './renderer';
import { IDLE_UNIFORMS, type ShaderUniforms } from './types';

export type AuraphyxToggles = {
  bloomEnabled: boolean;
  visualNoise: boolean;
  auraphyxEnabled: boolean;
};

type Props = {
  toggles: AuraphyxToggles;
};

const AURAPHYX_EVENT = 'auraphyx-frame';

/** Full Auraphyx visualizer surface: WebGL2 canvas, live-driven by the
 * `auraphyx-frame` event the backend's analysis thread emits (falls back to
 * a dim idle field, animated only by the local clock, when nothing is
 * playing or in the browser build where no backend event ever arrives). */
export function AuraphyxCanvas({ toggles }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const latestUniforms = useRef<ShaderUniforms>(IDLE_UNIFORMS);
  const togglesRef = useRef(toggles);

  useEffect(() => {
    togglesRef.current = toggles;
  }, [toggles]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const gl = canvas.getContext('webgl2', { antialias: false, alpha: false });
    if (!gl) {
      console.error('[auraphyx] WebGL2 unavailable');
      return;
    }

    let renderer: AuraphyxRenderer;
    try {
      renderer = new AuraphyxRenderer(gl, canvas.clientWidth || 1, canvas.clientHeight || 1);
    } catch (e) {
      console.error('[auraphyx] renderer init failed', e);
      return;
    }

    let unsubscribe: (() => void) | null = null;
    nativeListen<ShaderUniforms>(AURAPHYX_EVENT, (payload) => {
      latestUniforms.current = payload;
    }).then((unlisten) => {
      unsubscribe = unlisten;
    });

    const resizeObserver = new ResizeObserver(() => {
      const dpr = window.devicePixelRatio || 1;
      const w = Math.max(1, Math.round(canvas.clientWidth * dpr));
      const h = Math.max(1, Math.round(canvas.clientHeight * dpr));
      if (canvas.width !== w || canvas.height !== h) {
        canvas.width = w;
        canvas.height = h;
        renderer.resize(w, h);
      }
    });
    resizeObserver.observe(canvas);

    const start = performance.now();
    let rafId = 0;
    const frame = () => {
      const t = togglesRef.current;
      renderer.bloomEnabled = t.bloomEnabled;
      renderer.visualNoise = t.visualNoise;
      renderer.auraphyxEnabled = t.auraphyxEnabled;
      renderer.render(latestUniforms.current, (performance.now() - start) / 1000);
      rafId = requestAnimationFrame(frame);
    };
    rafId = requestAnimationFrame(frame);

    return () => {
      cancelAnimationFrame(rafId);
      resizeObserver.disconnect();
      unsubscribe?.();
      renderer.dispose();
    };
  }, []);

  return <canvas ref={canvasRef} className="auraphyx-canvas" />;
}
