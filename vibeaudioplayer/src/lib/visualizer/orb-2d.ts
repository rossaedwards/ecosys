import type { OrbUniforms } from "./orb-uniforms";

function chladni(x: number, y: number, m: number, n: number) {
  return Math.cos(m * Math.PI * x) * Math.cos(n * Math.PI * y)
    - Math.cos(n * Math.PI * x) * Math.cos(m * Math.PI * y);
}

/**
 * Faithful 2D stand-in for vibe.frag when WebGL is unavailable.
 * Same uniform block: Chladni field, 4-band chromatic energy, nine-node ring,
 * photometric hex, bloom = arousal × brightness_ceiling, near-black void.
 */
export function drawOrb2d(
  ctx: CanvasRenderingContext2D,
  cssW: number,
  cssH: number,
  u: OrbUniforms,
) {
  const w = cssW;
  const h = cssH;
  ctx.save();
  ctx.globalCompositeOperation = "source-over";
  ctx.fillStyle = "#07060c";
  ctx.fillRect(0, 0, w, h);

  const cx = w / 2;
  const cy = h * 0.42;
  const scale = Math.min(w, h) * 0.46;
  const [pr, pg, pb] = u.primaryRgb;
  const [sr, sg, sb] = u.secondaryRgb;
  const valenceN = u.valence * 0.5 + 0.5;
  const tr = sr + (pr - sr) * valenceN;
  const tg = sg + (pg - sg) * valenceN;
  const tb = sb + (pb - sb) * valenceN;
  const chrom = u.chromEnergy;
  const specR = 0.85 * chrom[0] + 1.0 * chrom[1] + 0.1 * chrom[2] + 0.3 * chrom[3];
  const specG = 0.05 * chrom[0] + 0.55 * chrom[1] + 0.75 * chrom[2] + 0.15 * chrom[3];
  const specB = 0.05 * chrom[0] + 0.0 * chrom[1] + 0.55 * chrom[2] + 0.95 * chrom[3];
  const fr = specR * 0.6 + tr * 0.4 + (0.1 + 0.9 * valenceN) * u.arousal * 0.3;
  const fg = specG * 0.6 + tg * 0.4 + (0.2 + 0.6 * valenceN) * u.arousal * 0.3;
  const fb = specB * 0.6 + tb * 0.4 + (0.6 - 0.4 * valenceN) * u.arousal * 0.3;

  const bloom = u.bloomStrength;
  const g0 = ctx.createRadialGradient(cx, cy, 8, cx, cy, scale * 2.1);
  g0.addColorStop(0, rgba(fr, fg, fb, 0.22 + bloom * 0.18));
  g0.addColorStop(0.45, rgba(sr, sg, sb, 0.1));
  g0.addColorStop(1, "rgba(7,6,12,0)");
  ctx.fillStyle = g0;
  ctx.fillRect(0, 0, w, h);

  const mNode = 2 + u.syncopation * 6;
  const nNode = mNode + 1 + u.groove * 2;
  const cols = 72;
  const rows = 72;
  ctx.globalAlpha = 0.85;
  for (let j = 0; j < rows; j++) {
    for (let i = 0; i < cols; i++) {
      const nx = (i / (cols - 1)) * 2 - 1;
      const ny = (j / (rows - 1)) * 2 - 1;
      const px = nx * (w / h);
      const py = ny;
      const warp = 1 + u.dissonance * 0.3 * Math.sin(Math.atan2(py, px) * 7 + u.time * 2);
      const v = Math.abs(chladni(px * 0.8 * warp, py * 0.8 * warp, mNode, nNode));
      if (v > 0.055) continue;
      const a = (1 - v / 0.055) * (0.18 + u.arousal * 0.35);
      ctx.fillStyle = rgba(fr, fg, fb, a);
      const x = cx + nx * scale * 1.35;
      const y = cy + ny * scale * 1.35;
      ctx.fillRect(x, y, Math.max(2, w / cols), Math.max(2, h / rows));
    }
  }
  ctx.globalAlpha = 1;

  const pulseR = (0.35 + u.arousal * 0.25 + Math.sin(u.time * u.bpmNorm * 6.28) * 0.05) * scale;
  ctx.strokeStyle = rgba(fr, fg, fb, 0.55 + chrom[1] * 0.3);
  ctx.lineWidth = 2 + (1 - u.groove) * 2;
  ctx.beginPath();
  ctx.arc(cx, cy, pulseR, 0, Math.PI * 2);
  ctx.stroke();

  for (let i = 1; i <= 5; i++) {
    const rr = 0.15 * i * (1 + u.saturation * 0.4) * scale * 1.8;
    ctx.strokeStyle = rgba(sr, sg, sb, u.saturation * (0.35 / i));
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.arc(cx, cy, rr, 0, Math.PI * 2);
    ctx.stroke();
  }

  const orbR = scale * (0.22 + chrom[1] * 0.04 + u.arousal * 0.04);
  ctx.save();
  ctx.shadowColor = rgba(pr, pg, pb, 0.65);
  ctx.shadowBlur = 28 + bloom * 40;
  const core = ctx.createRadialGradient(cx - orbR * 0.25, cy - orbR * 0.3, orbR * 0.08, cx, cy, orbR * 1.45);
  core.addColorStop(0, rgba(1, 1, 1, 0.72));
  core.addColorStop(0.22, rgba(sr, sg, sb, 0.8));
  core.addColorStop(0.62, rgba(pr, pg, pb, 0.9));
  core.addColorStop(1, "rgba(7,6,12,0)");
  ctx.fillStyle = core;
  ctx.beginPath();
  ctx.arc(cx, cy, orbR * 1.45, 0, Math.PI * 2);
  ctx.fill();
  ctx.restore();

  const bands = [
    [0.85, 0.05, 0.05, chrom[0]],
    [1.0, 0.55, 0.0, chrom[1]],
    [0.1, 0.75, 0.55, chrom[2]],
    [0.3, 0.15, 0.95, chrom[3]],
  ] as const;
  for (let b = 0; b < 4; b++) {
    const [br, bg, bb, mag] = bands[b]!;
    const a0 = -Math.PI / 2 + b * (Math.PI / 2) * 0.92;
    const a1 = a0 + (Math.PI / 2) * 0.85;
    ctx.strokeStyle = rgba(br, bg, bb, 0.25 + mag * 0.7);
    ctx.lineWidth = 3 + mag * 7;
    ctx.beginPath();
    ctx.arc(cx, cy, scale * 0.92, a0, a1);
    ctx.stroke();
  }

  for (let i = 0; i < 9; i++) {
    const a = (i / 9) * Math.PI * 2 - Math.PI / 2 + u.time * 0.04;
    const rad = scale * (0.58 + u.arousal * 0.04);
    const x = cx + Math.cos(a) * rad;
    const y = cy + Math.sin(a) * rad;
    const mag = chrom[i % 4]!;
    ctx.beginPath();
    ctx.fillStyle = rgba(
      sr + (pr - sr) * (i / 8),
      sg + (pg - sg) * (i / 8),
      sb + (pb - sb) * (i / 8),
      0.35 + mag * 0.55,
    );
    ctx.arc(x, y, 4 + mag * 5, 0, Math.PI * 2);
    ctx.fill();
  }

  if (u.visualNoise > 0.5) {
    ctx.globalAlpha = u.visualNoise * 0.12;
    ctx.fillStyle = "#ffffff";
    for (let y = 0; y < h; y += 6) {
      for (let x = 0; x < w; x += 6) {
        if (((x * 13 + y * 7 + Math.floor(u.time * 40)) & 7) === 0) ctx.fillRect(x, y, 1, 1);
      }
    }
    ctx.globalAlpha = 1;
  }

  ctx.restore();
}

function rgba(r: number, g: number, b: number, a: number) {
  return `rgba(${Math.round(r * 255)},${Math.round(g * 255)},${Math.round(b * 255)},${a})`;
}
