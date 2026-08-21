import { useEffect, useRef, useState, type RefObject } from 'react';

/** Web Audio spectrum from an HTMLAudioElement (browser path). */
export function useLiveSpectrum(
  audioRef: RefObject<HTMLAudioElement | null>,
  playing: boolean,
  bars = 48,
): number[] {
  const [levels, setLevels] = useState<number[]>(() => Array(bars).fill(0));
  const ctxRef = useRef<AudioContext | null>(null);
  const analyserRef = useRef<AnalyserNode | null>(null);
  const sourceRef = useRef<MediaElementAudioSourceNode | null>(null);
  const rafRef = useRef<number>(0);

  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    const ensure = async () => {
      if (!ctxRef.current) {
        const Ctx = window.AudioContext || (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext;
        ctxRef.current = new Ctx();
      }
      const ctx = ctxRef.current;
      if (ctx.state === 'suspended') await ctx.resume();
      if (!sourceRef.current) {
        try {
          sourceRef.current = ctx.createMediaElementSource(audio);
          const analyser = ctx.createAnalyser();
          analyser.fftSize = 256;
          analyser.smoothingTimeConstant = 0.8;
          sourceRef.current.connect(analyser);
          analyser.connect(ctx.destination);
          analyserRef.current = analyser;
        } catch {
          // Already connected or CORS
        }
      }
    };

    void ensure();

    const tick = () => {
      const analyser = analyserRef.current;
      if (analyser && playing) {
        const data = new Uint8Array(analyser.frequencyBinCount);
        analyser.getByteFrequencyData(data);
        const next = Array.from({ length: bars }, (_, i) => {
          const start = Math.floor((i / bars) * data.length);
          const end = Math.floor(((i + 1) / bars) * data.length);
          let sum = 0;
          for (let j = start; j < end; j++) sum += data[j];
          return sum / Math.max(1, end - start) / 255;
        });
        setLevels(next);
      } else if (!playing) {
        setLevels((prev) => prev.map((v) => v * 0.85));
      }
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [audioRef, playing, bars]);

  return levels;
}
