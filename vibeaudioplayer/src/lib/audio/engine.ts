import type { DemoSynth } from "@/lib/vasp/catalog";

export type FrameSample = {
  freq: Uint8Array;
  time: Uint8Array;
  bass: number;
  mid: number;
  treble: number;
  rms: number;
  energy: number;
  beat: boolean;
  beatPulse: number;
  timeSec: number;
};

type DemoHandle = {
  stop: () => void;
};

const FFT = 1024;

function clamp01(n: number) {
  return n < 0 ? 0 : n > 1 ? 1 : n;
}

export class VibeAudioEngine {
  private ctx: AudioContext | null = null;
  private master: GainNode | null = null;
  private analyser: AnalyserNode | null = null;
  private mediaEl: HTMLAudioElement | null = null;
  private mediaSrc: MediaElementAudioSourceNode | null = null;
  private demo: DemoHandle | null = null;
  private freq = new Uint8Array(FFT / 2);
  private time = new Uint8Array(FFT);
  private volume = 0.72;
  private lastBeatAt = 0;
  private beatPulse = 0;
  private prevBass = 0;
  private simPhase = 0;
  private bpm = 128;
  private playing = false;
  private mode: "idle" | "demo" | "file" = "idle";
  private onEnded: (() => void) | null = null;
  private onTime: ((t: number, d: number) => void) | null = null;
  private timeTimer: number | null = null;

  isPlaying() {
    return this.playing;
  }

  getMediaElement() {
    return this.mediaEl;
  }

  setBpm(bpm: number) {
    this.bpm = Math.max(60, Math.min(200, bpm));
  }

  setVolume(v: number) {
    this.volume = clamp01(v);
    if (this.master && this.ctx) {
      this.master.gain.setTargetAtTime(this.volume, this.ctx.currentTime, 0.04);
    }
    if (this.mediaEl) this.mediaEl.volume = this.volume;
  }

  setEndedHandler(fn: (() => void) | null) {
    this.onEnded = fn;
  }

  setTimeHandler(fn: ((t: number, d: number) => void) | null) {
    this.onTime = fn;
  }

  /** Call synchronously from a click/tap so the AudioContext unlocks. */
  unlock() {
    if (typeof window === "undefined") return;
    if (this.ctx && this.ctx.state !== "closed") {
      if (this.ctx.state === "suspended") void this.ctx.resume();
      return;
    }
    const Ctx =
      window.AudioContext ||
      (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext;
    const ctx = new Ctx();
    const master = ctx.createGain();
    master.gain.value = this.volume;
    const analyser = ctx.createAnalyser();
    analyser.fftSize = FFT;
    analyser.smoothingTimeConstant = 0.72;
    analyser.minDecibels = -92;
    analyser.maxDecibels = -18;
    master.connect(analyser);
    analyser.connect(ctx.destination);
    this.ctx = ctx;
    this.master = master;
    this.analyser = analyser;
    this.freq = new Uint8Array(analyser.frequencyBinCount);
    this.time = new Uint8Array(analyser.fftSize);
    this.ensureMediaElement();
    if (ctx.state === "suspended") void ctx.resume();
  }

  async ensure(): Promise<AudioContext> {
    this.unlock();
    if (!this.ctx) throw new Error("Audio is not available in this browser.");
    if (this.ctx.state === "suspended") {
      await Promise.race([
        this.ctx.resume(),
        new Promise<void>((resolve) => window.setTimeout(resolve, 400)),
      ]);
    }
    return this.ctx;
  }

  private ensureMediaElement() {
    if (this.mediaEl) return this.mediaEl;
    const el = document.createElement("audio");
    el.preload = "auto";
    el.crossOrigin = "anonymous";
    el.setAttribute("playsinline", "true");
    el.volume = this.volume;
    el.addEventListener("ended", () => this.onEnded?.());
    el.addEventListener("timeupdate", () => {
      if (this.mode === "file") this.onTime?.(el.currentTime, el.duration || 0);
    });
    this.mediaEl = el;
    return el;
  }

  private connectMedia() {
    if (!this.ctx || !this.master || !this.mediaEl) return;
    if (this.mediaSrc) return;
    try {
      this.mediaSrc = this.ctx.createMediaElementSource(this.mediaEl);
      this.mediaSrc.connect(this.master);
    } catch {
      /* already connected */
    }
  }

  async playDemo(synth: DemoSynth) {
    const ctx = await this.ensure();
    this.stopDemo();
    const el = this.ensureMediaElement();
    el.pause();
    this.bpm = synth.bpm;
    this.mode = "demo";
    this.playing = true;
    this.demo = startDemoSynth(ctx, this.master!, synth);
    this.startTimeClock();
    this.publishMediaSession(true);
  }

  async playFile(url: string) {
    const ctx = await this.ensure();
    this.stopDemo();
    const el = this.ensureMediaElement();
    this.connectMedia();
    if (el.src !== url) {
      el.src = url;
    }
    this.mode = "file";
    this.playing = true;
    el.volume = this.volume;
    try {
      await el.play();
    } catch {
      this.playing = false;
      throw new Error("Playback was blocked. Tap play again.");
    }
    this.startTimeClock();
    this.publishMediaSession(true);
    void ctx;
  }

  async togglePause() {
    if (this.mode === "file" && this.mediaEl) {
      if (this.playing) {
        this.mediaEl.pause();
        this.playing = false;
        this.publishMediaSession(false);
      } else {
        await this.ensure();
        await this.mediaEl.play();
        this.playing = true;
        this.publishMediaSession(true);
      }
      return this.playing;
    }
    if (this.mode === "demo") {
      if (this.playing) {
        this.demo?.stop();
        this.demo = null;
        this.playing = false;
        this.publishMediaSession(false);
      } else {
        /* caller restarts with current synth */
      }
      return this.playing;
    }
    return false;
  }

  pause() {
    if (this.mode === "file" && this.mediaEl) this.mediaEl.pause();
    this.stopDemo();
    this.playing = false;
    this.publishMediaSession(false);
  }

  seek(seconds: number) {
    if (this.mode === "file" && this.mediaEl) {
      this.mediaEl.currentTime = Math.max(0, seconds);
    }
  }

  currentTime() {
    if (this.mode === "file" && this.mediaEl) return this.mediaEl.currentTime;
    if (this.mode === "demo" && this.ctx) return this.ctx.currentTime;
    return 0;
  }

  duration() {
    if (this.mode === "file" && this.mediaEl && Number.isFinite(this.mediaEl.duration)) {
      return this.mediaEl.duration;
    }
    return 0;
  }

  stop() {
    this.pause();
    this.mode = "idle";
    if (this.mediaEl) {
      this.mediaEl.removeAttribute("src");
      this.mediaEl.load();
    }
  }

  private stopDemo() {
    this.demo?.stop();
    this.demo = null;
  }

  private startTimeClock() {
    if (this.timeTimer != null) window.clearInterval(this.timeTimer);
    this.timeTimer = window.setInterval(() => {
      if (!this.playing) return;
      if (this.mode === "file" && this.mediaEl) {
        this.onTime?.(this.mediaEl.currentTime, this.mediaEl.duration || 0);
      } else if (this.mode === "demo") {
        this.onTime?.(this.currentTime() % 3600, 0);
      }
    }, 250);
  }

  sample(): FrameSample {
    const now = typeof performance !== "undefined" ? performance.now() / 1000 : 0;
    this.simPhase = now;
    const beatLen = 60 / this.bpm;

    if (this.analyser && this.playing) {
      this.analyser.getByteFrequencyData(this.freq);
      this.analyser.getByteTimeDomainData(this.time);
    } else {
      fillSimulated(this.freq, this.time, now, this.bpm, this.playing);
    }

    const bass = bandMean(this.freq, 0, 6);
    const mid = bandMean(this.freq, 8, 28);
    const treble = bandMean(this.freq, 40, 90);
    let rms = 0;
    for (let i = 0; i < this.time.length; i++) {
      const v = (this.time[i]! - 128) / 128;
      rms += v * v;
    }
    rms = Math.sqrt(rms / this.time.length);
    const energy = clamp01(bass * 0.55 + mid * 0.3 + rms * 0.8);

    const flux = Math.max(0, bass - this.prevBass);
    this.prevBass = bass * 0.65 + this.prevBass * 0.35;
    const minGap = beatLen * 0.46;
    let beat = false;
    if (this.playing && flux > 0.12 && bass > 0.34 && now - this.lastBeatAt > minGap) {
      beat = true;
      this.lastBeatAt = now;
      this.beatPulse = 1;
    } else {
      const kickPhase = (now % beatLen) / beatLen;
      if (this.playing && kickPhase < 0.06 && now - this.lastBeatAt > minGap) {
        beat = true;
        this.lastBeatAt = now;
        this.beatPulse = Math.max(this.beatPulse, 0.85);
      }
    }
    this.beatPulse *= 0.86;

    return {
      freq: this.freq,
      time: this.time,
      bass,
      mid,
      treble,
      rms,
      energy,
      beat,
      beatPulse: this.beatPulse,
      timeSec: now,
    };
  }

  private publishMediaSession(playing: boolean) {
    if (typeof navigator === "undefined" || !("mediaSession" in navigator)) return;
    try {
      navigator.mediaSession.playbackState = playing ? "playing" : "paused";
    } catch {
      /* ignore */
    }
  }
}

function bandMean(data: Uint8Array, from: number, to: number) {
  const end = Math.min(data.length - 1, to);
  const start = Math.min(from, end);
  let s = 0;
  let n = 0;
  for (let i = start; i <= end; i++) {
    s += data[i]!;
    n++;
  }
  return n ? s / n / 255 : 0;
}

function fillSimulated(freq: Uint8Array, time: Uint8Array, t: number, bpm: number, playing: boolean) {
  const beatLen = 60 / bpm;
  const phase = (t % beatLen) / beatLen;
  const kick = playing ? Math.exp(-Math.pow(phase * 7.2, 2)) : 0.12;
  const hat = playing ? Math.pow(Math.max(0, Math.sin(t * Math.PI * (bpm / 60) * 4)), 8) * 0.45 : 0.05;
  const bass = playing ? 0.35 + 0.35 * Math.sin(t * Math.PI * (bpm / 60) * 0.5) + kick * 0.5 : 0.12;
  for (let i = 0; i < freq.length; i++) {
    const bin = i / freq.length;
    const envelope = Math.exp(-bin * 5.4) * bass + Math.exp(-Math.pow(bin - 0.18, 2) * 40) * 0.35 + Math.exp(-Math.pow(bin - 0.55, 2) * 28) * hat;
    const wobble = 0.5 + 0.5 * Math.sin(t * 2.2 + i * 0.17);
    freq[i] = Math.max(0, Math.min(255, envelope * wobble * 255 * (playing ? 1 : 0.35)));
  }
  for (let i = 0; i < time.length; i++) {
    const x = i / time.length;
    const wave =
      Math.sin(x * Math.PI * 8 + t * 6) * (0.18 + kick * 0.45) +
      Math.sin(x * Math.PI * 20 + t * 3.2) * 0.08;
    time[i] = Math.max(0, Math.min(255, 128 + wave * 120 * (playing ? 1 : 0.25)));
  }
}

function startDemoSynth(ctx: AudioContext, dest: AudioNode, synth: DemoSynth): DemoHandle {
  const bpm = synth.bpm;
  const stepSec = 60 / bpm / 4;
  let step = 0;
  let next = ctx.currentTime + 0.06;
  let stopped = false;
  let timer = 0;

  const filter = ctx.createBiquadFilter();
  filter.type = "lowpass";
  filter.frequency.value = 1400 + synth.brightness * 2200;
  filter.Q.value = 0.7;
  filter.connect(dest);

  const padGain = ctx.createGain();
  padGain.gain.value = 0.045 + synth.drive * 0.02;
  padGain.connect(filter);

  const roots = synth.minor
    ? [1, 1, 6 / 5, 3 / 2, 1, 2 / 3, 3 / 2, 1]
    : [1, 1, 5 / 4, 3 / 2, 1, 2 / 3, 3 / 2, 1];

  const padOsc: OscillatorNode[] = [];
  const ratios = synth.minor ? [1, 6 / 5, 3 / 2] : [1, 5 / 4, 3 / 2];
  for (const r of ratios) {
    const o = ctx.createOscillator();
    o.type = "sine";
    o.frequency.value = synth.rootHz * r * 0.5;
    o.connect(padGain);
    o.start();
    padOsc.push(o);
  }

  const kick = (time: number, accent: number) => {
    const o = ctx.createOscillator();
    const g = ctx.createGain();
    o.type = "sine";
    o.frequency.setValueAtTime(150 + synth.drive * 20, time);
    o.frequency.exponentialRampToValueAtTime(38, time + 0.12);
    g.gain.setValueAtTime(0.0001, time);
    g.gain.exponentialRampToValueAtTime(0.85 * accent * synth.drive, time + 0.008);
    g.gain.exponentialRampToValueAtTime(0.0001, time + 0.22);
    o.connect(g);
    g.connect(dest);
    o.start(time);
    o.stop(time + 0.25);
  };

  const hat = (time: number, open: boolean, gain: number) => {
    const buffer = ctx.createBuffer(1, Math.floor(ctx.sampleRate * 0.08), ctx.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < data.length; i++) data[i] = Math.random() * 2 - 1;
    const src = ctx.createBufferSource();
    src.buffer = buffer;
    const bp = ctx.createBiquadFilter();
    bp.type = "highpass";
    bp.frequency.value = open ? 6000 : 8000;
    const g = ctx.createGain();
    g.gain.setValueAtTime(gain, time);
    g.gain.exponentialRampToValueAtTime(0.0001, time + (open ? 0.12 : 0.04));
    src.connect(bp);
    bp.connect(g);
    g.connect(dest);
    src.start(time);
    src.stop(time + 0.14);
  };

  const bass = (time: number, freq: number, len: number) => {
    const o = ctx.createOscillator();
    const g = ctx.createGain();
    const f = ctx.createBiquadFilter();
    o.type = "triangle";
    o.frequency.setValueAtTime(freq, time);
    f.type = "lowpass";
    f.frequency.setValueAtTime(380 + synth.brightness * 240, time);
    g.gain.setValueAtTime(0.0001, time);
    g.gain.exponentialRampToValueAtTime(0.22 * synth.drive, time + 0.02);
    g.gain.exponentialRampToValueAtTime(0.0001, time + len);
    o.connect(f);
    f.connect(g);
    g.connect(dest);
    o.start(time);
    o.stop(time + len + 0.02);
  };

  const blip = (time: number, freq: number) => {
    const o = ctx.createOscillator();
    const g = ctx.createGain();
    o.type = "sine";
    o.frequency.setValueAtTime(freq, time);
    g.gain.setValueAtTime(0.0001, time);
    g.gain.exponentialRampToValueAtTime(0.05 * synth.brightness, time + 0.01);
    g.gain.exponentialRampToValueAtTime(0.0001, time + 0.16);
    o.connect(g);
    g.connect(filter);
    o.start(time);
    o.stop(time + 0.18);
  };

  const schedule = () => {
    if (stopped) return;
    const horizon = ctx.currentTime + 0.12;
    while (next < horizon) {
      const s = step % 16;
      const bar = Math.floor(step / 4) % 8;
      const accent = s === 0 || s === 8 ? 1 : s % 4 === 0 ? 0.78 : 0.55;
      if (s % 4 === 0) kick(next, accent);
      if (s === 4 || s === 12) {
        hat(next, true, 0.045 * synth.hatDensity);
      }
      if (synth.hatDensity > 0.3 && s % 2 === 0) {
        hat(next, false, (s % 4 === 2 ? 0.04 : 0.022) * synth.hatDensity);
      }
      if (s === 0 || s === 6 || s === 10 || s === 14) {
        const r = roots[bar] ?? 1;
        bass(next, synth.rootHz * 0.5 * r, stepSec * 2.2);
      }
      if (s % 4 === 2 && synth.brightness > 0.4) {
        const arp = [1, 3 / 2, 2, 9 / 4][bar % 4] ?? 1;
        blip(next, synth.rootHz * arp);
      }
      next += stepSec;
      step += 1;
    }
    timer = window.setTimeout(schedule, 25);
  };
  schedule();

  return {
    stop() {
      stopped = true;
      window.clearTimeout(timer);
      for (const o of padOsc) {
        try {
          o.stop();
          o.disconnect();
        } catch {
          /* already stopped */
        }
      }
      try {
        padGain.disconnect();
        filter.disconnect();
      } catch {
        /* ignore */
      }
    },
  };
}

export const engine = new VibeAudioEngine();
