import { hexToRgb } from "@/lib/utils";
import type { FrameSample } from "@/lib/audio/engine";
import type { VisualMapping, VisualSettings } from "@/lib/vasp/types";

type Particle = {
  x: number;
  y: number;
  vx: number;
  vy: number;
  r: number;
  a: number;
  life: number;
};

type Drop = {
  x: number;
  y: number;
  vy: number;
  len: number;
  a: number;
};

export class VisualizerScene {
  particles: Particle[] = [];
  rain: Drop[] = [];
  rot = 0;
  aurora = 0;
  width = 0;
  height = 0;

  resize(w: number, h: number) {
    this.width = w;
    this.height = h;
  }

  tick(dt: number, sample: FrameSample, map: VisualMapping, settings: VisualSettings) {
    const w = this.width;
    const h = this.height;
    if (!w || !h) return;
    const motion = settings.reducedMotion ? 0.15 : settings.motionIntensity;
    this.rot += dt * (0.18 + map.bpm / 220) * motion * (0.6 + sample.energy);
    this.aurora += dt * 0.12 * motion;

    const target = settings.particles && !settings.reducedMotion
      ? Math.round(70 * map.particleDensity * (0.55 + settings.colorIntensity))
      : 0;
    while (this.particles.length < target) {
      this.particles.push(this.spawnParticle(w, h, map));
    }
    if (this.particles.length > target) this.particles.length = target;

    for (const p of this.particles) {
      p.x += p.vx * (1 + sample.bass * 1.4) * motion * 60 * dt;
      p.y += p.vy * (1 + sample.energy) * motion * 60 * dt;
      p.life -= dt * 0.12;
      p.a = Math.max(0, p.life);
      if (p.x < -20 || p.x > w + 20 || p.y < -20 || p.y > h + 20 || p.life <= 0) {
        Object.assign(p, this.spawnParticle(w, h, map));
      }
    }

    const rainTarget = map.rain && !settings.reducedMotion ? Math.round(42 * motion) : 0;
    while (this.rain.length < rainTarget) {
      this.rain.push({
        x: Math.random() * w,
        y: Math.random() * h,
        vy: 280 + Math.random() * 340,
        len: 8 + Math.random() * 18,
        a: 0.08 + Math.random() * 0.12,
      });
    }
    if (this.rain.length > rainTarget) this.rain.length = rainTarget;
    for (const d of this.rain) {
      d.y += d.vy * dt * (0.7 + motion);
      d.x += dt * 40 * motion;
      if (d.y > h + 20) {
        d.y = -20;
        d.x = Math.random() * w;
      }
    }
  }

  private spawnParticle(w: number, h: number, map: VisualMapping): Particle {
    const cx = w / 2;
    const cy = h * 0.42;
    const ang = Math.random() * Math.PI * 2;
    const dist = 30 + Math.random() * Math.min(w, h) * 0.42;
    return {
      x: cx + Math.cos(ang) * dist,
      y: cy + Math.sin(ang) * dist * 0.72,
      vx: Math.cos(ang) * (0.15 + Math.random() * 0.55) * (0.4 + map.movementEnergy),
      vy: Math.sin(ang) * (0.1 + Math.random() * 0.4) - 0.12,
      r: 0.6 + Math.random() * 2.2,
      a: 0.2 + Math.random() * 0.55,
      life: 0.6 + Math.random() * 1.4,
    };
  }
}

export function drawFrame(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  sample: FrameSample,
  map: VisualMapping,
  settings: VisualSettings,
  scene: VisualizerScene,
) {
  const primary = hexToRgb(map.primary);
  const secondary = hexToRgb(map.secondary);
  const intensity = settings.colorIntensity;
  const beat = settings.beatPulse ? sample.beatPulse : 0;
  const motion = settings.reducedMotion ? 0.2 : settings.motionIntensity;
  const fade = settings.reducedMotion ? 1 : Math.min(0.42, map.fade + 0.12);
  const brightness = map.brightness * intensity;

  ctx.globalCompositeOperation = "source-over";
  ctx.fillStyle = `rgba(7, 6, 12, ${0.55 + fade * 0.4})`;
  ctx.fillRect(0, 0, w, h);

  const cx = w / 2;
  const cy = h * 0.4;
  const warm = map.temperature === "warm" ? 28 : map.temperature === "neutral" ? 10 : 0;
  const pr = Math.min(primary.r + warm * map.accentWarmth * 40, 255);
  const pg = primary.g;
  const pb = primary.b;

  const g0 = ctx.createRadialGradient(cx, cy, 8, cx, cy, Math.max(w, h) * 0.72);
  g0.addColorStop(0, `rgba(${pr},${pg},${pb},${0.22 * brightness})`);
  g0.addColorStop(0.38, `rgba(${secondary.r},${secondary.g},${secondary.b},${0.1 * brightness})`);
  g0.addColorStop(1, "rgba(7,6,12,0)");
  ctx.fillStyle = g0;
  ctx.fillRect(0, 0, w, h);

  const ax = cx + Math.cos(scene.aurora) * w * 0.18;
  const ay = cy + Math.sin(scene.aurora * 0.7) * h * 0.08;
  const g1 = ctx.createRadialGradient(ax, ay, 20, ax, ay, w * 0.45);
  g1.addColorStop(0, `rgba(${secondary.r},${secondary.g},${secondary.b},${0.16 * brightness * (0.5 + sample.energy)})`);
  g1.addColorStop(1, "rgba(0,0,0,0)");
  ctx.fillStyle = g1;
  ctx.fillRect(0, 0, w, h);

  const bx = cx + Math.cos(scene.aurora + 2.1) * w * 0.22;
  const by = cy + Math.sin(scene.aurora * 0.9 + 1.2) * h * 0.1;
  const g2 = ctx.createRadialGradient(bx, by, 10, bx, by, w * 0.38);
  g2.addColorStop(0, `rgba(${pr},${pg},${Math.min(255, pb + 40)},${0.12 * brightness})`);
  g2.addColorStop(1, "rgba(0,0,0,0)");
  ctx.fillStyle = g2;
  ctx.fillRect(0, 0, w, h);

  if (scene.rain.length) {
    ctx.strokeStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},0.18)`;
    ctx.lineWidth = 1;
    ctx.beginPath();
    for (const d of scene.rain) {
      ctx.moveTo(d.x, d.y);
      ctx.lineTo(d.x + 3, d.y + d.len);
    }
    ctx.stroke();
  }

  if (settings.particles) {
    for (const p of scene.particles) {
      ctx.beginPath();
      ctx.fillStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},${p.a * 0.7 * intensity})`;
      ctx.arc(p.x, p.y, p.r * (1 + sample.treble * 0.8), 0, Math.PI * 2);
      ctx.fill();
    }
  }

  const orbR = Math.min(w, h) * (0.16 + sample.bass * 0.04 + beat * 0.035 * map.impactScale);
  drawMandala(ctx, cx, cy, orbR, scene.rot, map, sample, primary, secondary, intensity, motion);
  drawOrb(ctx, cx, cy, orbR, sample, primary, secondary, brightness, beat, map);

  if (settings.spectrum) {
    drawSpectrum(ctx, cx, cy, orbR, sample, secondary, primary, intensity, map, scene.rot);
  }

  drawWaveform(ctx, w, h, sample, secondary, intensity, beat);
  drawHorizon(ctx, w, h, primary, secondary, sample, brightness);

  if (map.grain > 0.05 && !settings.reducedMotion) {
    ctx.save();
    ctx.globalAlpha = map.grain * 0.18 * intensity;
    const step = 7;
    for (let y = 0; y < h; y += step) {
      for (let x = 0; x < w; x += step) {
        if (((x * 13 + y * 7 + Math.floor(sample.timeSec * 40)) & 7) === 0) {
          ctx.fillStyle = "#ffffff";
          ctx.fillRect(x, y, 1, 1);
        }
      }
    }
    ctx.restore();
  }
}

function drawOrb(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  r: number,
  sample: FrameSample,
  primary: { r: number; g: number; b: number },
  secondary: { r: number; g: number; b: number },
  brightness: number,
  beat: number,
  map: VisualMapping,
) {
  ctx.save();
  ctx.shadowColor = `rgba(${primary.r},${primary.g},${primary.b},${0.65 * brightness})`;
  ctx.shadowBlur = 36 + beat * 40 * map.impactScale;
  const glow = ctx.createRadialGradient(cx, cy, r * 0.1, cx, cy, r * 1.65);
  glow.addColorStop(0, `rgba(255,255,255,${0.55 * brightness})`);
  glow.addColorStop(0.18, `rgba(${secondary.r},${secondary.g},${secondary.b},${0.55 * brightness})`);
  glow.addColorStop(0.55, `rgba(${primary.r},${primary.g},${primary.b},${0.55 * brightness})`);
  glow.addColorStop(1, "rgba(7,6,12,0)");
  ctx.fillStyle = glow;
  ctx.beginPath();
  ctx.arc(cx, cy, r * 1.55, 0, Math.PI * 2);
  ctx.fill();

  const core = ctx.createRadialGradient(cx - r * 0.22, cy - r * 0.28, r * 0.05, cx, cy, r);
  core.addColorStop(0, `rgba(255,255,255,${0.8 * brightness})`);
  core.addColorStop(0.35, `rgba(${secondary.r},${secondary.g},${secondary.b},0.85)`);
  core.addColorStop(1, `rgba(${primary.r},${primary.g},${primary.b},0.95)`);
  ctx.fillStyle = core;
  ctx.shadowBlur = 18;
  ctx.beginPath();
  ctx.arc(cx, cy, r * (0.72 + sample.energy * 0.08), 0, Math.PI * 2);
  ctx.fill();
  ctx.restore();
}

function drawMandala(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  r: number,
  rot: number,
  map: VisualMapping,
  sample: FrameSample,
  primary: { r: number; g: number; b: number },
  secondary: { r: number; g: number; b: number },
  intensity: number,
  motion: number,
) {
  const sides = Math.max(3, map.geometrySides);
  ctx.save();
  ctx.translate(cx, cy);
  for (let ring = 0; ring < 3; ring++) {
    const rr = r * (1.15 + ring * 0.28 + sample.mid * 0.08);
    ctx.rotate((ring % 2 === 0 ? rot : -rot * 0.7) * motion);
    ctx.strokeStyle =
      ring === 1
        ? `rgba(${secondary.r},${secondary.g},${secondary.b},${0.28 * intensity})`
        : `rgba(${primary.r},${primary.g},${primary.b},${0.22 * intensity})`;
    ctx.lineWidth = ring === 0 ? 1.6 : 1;
    ctx.beginPath();
    for (let i = 0; i <= sides * 2; i++) {
      const a = (i / (sides * 2)) * Math.PI * 2 - Math.PI / 2;
      const rad = i % 2 === 0 ? rr : rr * (0.72 + map.harmonicShift);
      const x = Math.cos(a) * rad;
      const y = Math.sin(a) * rad;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.closePath();
    ctx.stroke();
  }
  ctx.restore();

  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(-rot * 0.35 * motion);
  ctx.strokeStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},${0.35 * intensity})`;
  ctx.lineWidth = 1.2;
  ctx.setLineDash([4, 8]);
  ctx.beginPath();
  ctx.arc(0, 0, r * 1.95, 0, Math.PI * 2);
  ctx.stroke();
  ctx.setLineDash([]);
  ctx.restore();
}

function drawSpectrum(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  orbR: number,
  sample: FrameSample,
  secondary: { r: number; g: number; b: number },
  primary: { r: number; g: number; b: number },
  intensity: number,
  map: VisualMapping,
  rot: number,
) {
  const bars = 64;
  const inner = orbR * 2.15;
  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(rot * 0.08);
  for (let i = 0; i < bars; i++) {
    const bin = Math.floor((i / bars) * (sample.freq.length * 0.55));
    const mag = (sample.freq[bin] ?? 0) / 255;
    const a = (i / bars) * Math.PI * 2 - Math.PI / 2;
    const len = mag * Math.min(88, orbR * 1.15) * map.edgeSharpness * intensity;
    const x0 = Math.cos(a) * inner;
    const y0 = Math.sin(a) * inner;
    const x1 = Math.cos(a) * (inner + len);
    const y1 = Math.sin(a) * (inner + len);
    const mix = i % 2 === 0 ? secondary : primary;
    ctx.strokeStyle = `rgba(${mix.r},${mix.g},${mix.b},${0.28 + mag * 0.55})`;
    ctx.lineWidth = 2.4;
    ctx.lineCap = "round";
    ctx.beginPath();
    ctx.moveTo(x0, y0);
    ctx.lineTo(x1, y1);
    ctx.stroke();
  }
  ctx.restore();
}

function drawWaveform(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  sample: FrameSample,
  secondary: { r: number; g: number; b: number },
  intensity: number,
  beat: number,
) {
  const y = h * 0.78;
  const amp = h * 0.055 * (1 + beat * 0.4);
  ctx.beginPath();
  const n = sample.time.length;
  for (let i = 0; i < n; i++) {
    const x = (i / (n - 1)) * w;
    const v = (sample.time[i]! - 128) / 128;
    const yy = y + v * amp;
    if (i === 0) ctx.moveTo(x, yy);
    else ctx.lineTo(x, yy);
  }
  ctx.strokeStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},${0.55 * intensity})`;
  ctx.lineWidth = 1.8;
  ctx.shadowColor = `rgba(${secondary.r},${secondary.g},${secondary.b},0.45)`;
  ctx.shadowBlur = 8;
  ctx.stroke();
  ctx.shadowBlur = 0;

  ctx.globalAlpha = 0.12 * intensity;
  ctx.lineTo(w, h);
  ctx.lineTo(0, h);
  ctx.closePath();
  ctx.fillStyle = `rgba(${secondary.r},${secondary.g},${secondary.b},0.4)`;
  ctx.fill();
  ctx.globalAlpha = 1;
}

function drawHorizon(
  ctx: CanvasRenderingContext2D,
  w: number,
  h: number,
  primary: { r: number; g: number; b: number },
  secondary: { r: number; g: number; b: number },
  sample: FrameSample,
  brightness: number,
) {
  const g = ctx.createLinearGradient(0, h * 0.62, 0, h);
  g.addColorStop(0, "rgba(7,6,12,0)");
  g.addColorStop(0.45, `rgba(${primary.r},${primary.g},${primary.b},${0.08 * brightness})`);
  g.addColorStop(1, `rgba(${secondary.r},${secondary.g},${secondary.b},${0.1 + sample.bass * 0.08})`);
  ctx.fillStyle = g;
  ctx.fillRect(0, h * 0.62, w, h * 0.38);
}
