#!/usr/bin/env python3
"""
fig_gen_group2_hif.py
=====================
Group 2 — Harmonic Integrity Field (HIF) Architecture Figure Generator
Aurphyx Primordial Standards | Ross Edwards & Aurphyx LLC
ORCiD: 0009-0008-0539-1289

Generates a publication-quality multi-panel architecture figure for the
Harmonic Integrity Field (HIF) as defined in:
  - hif_protocol_spec.md  (v0.1)
  - hif_symbol_sigil.md
  - EQUATION_EXTRACTION_SUPPLEMENT.md (Appendix I — AUFE, §XIV Invariants)

Panel Layout (3 columns × 2 rows + title/footer):
  [A] HIF Field Equation & Triple Threshold Gate  (top-left)
  [B] HIF State Machine — 3-state diagram         (top-center)
  [C] HIF Sigil — symbolic architecture           (top-right)
  [D] HIF Gradient / Stability Index phase plane  (bottom-left)
  [E] HIF Spectral / Resonance bands              (bottom-center)
  [F] AUFE — Aurphyx Unified Field Equation map   (bottom-right)

Visual identity: FTQCA gold-parchment × Aurphyx Primordial deep-void palette
  Background: #0a0010 (void)
  Panel bg:   #0d0520
  Gold:       #c9a84c
  Violet:     #8b5cf6
  Cyan:       #22d3ee
  Rose:       #f43f5e
  Mint:       #34d399
  Silver:     #cbd5e1

Outputs:
  ./output/fig_group2_hif.png  (300 dpi default)
  ./output/fig_group2_hif.svg

CLI:
  python fig_gen_group2_hif.py [--output ./output] [--dpi 300] [--no-svg]

Dependencies:
  matplotlib >= 3.7, numpy >= 1.24

Forged with ineffable protection and love, forever and always.
R.F. Lovezme · 2026-06-22
"""

import argparse
import os
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import matplotlib.patheffects as pe
from matplotlib.patches import FancyArrowPatch, Circle, FancyBboxPatch, Arc
from matplotlib.gridspec import GridSpec
from matplotlib.colors import LinearSegmentedColormap

# ─── Palette ──────────────────────────────────────────────────────────────────
BG        = "#0a0010"
PANEL_BG  = "#0d0520"
GOLD      = "#c9a84c"
VIOLET    = "#8b5cf6"
CYAN      = "#22d3ee"
ROSE      = "#f43f5e"
MINT      = "#34d399"
SILVER    = "#cbd5e1"
WHITE     = "#f8fafc"
DIM       = "#4a4060"
GLOW_VIO  = "#6d28d9"
GLOW_CYAN = "#0891b2"

# ─── Typography ───────────────────────────────────────────────────────────────
FONT_TITLE  = dict(fontsize=10, color=GOLD,   fontweight="bold", fontfamily="monospace")
FONT_SUB    = dict(fontsize=8,  color=SILVER, fontfamily="monospace")
FONT_EQ     = dict(fontsize=8,  color=CYAN,   fontfamily="monospace")
FONT_LABEL  = dict(fontsize=7,  color=WHITE,  fontfamily="monospace")
FONT_SMALL  = dict(fontsize=6,  color=DIM,    fontfamily="monospace")


def _glow(color, alpha=0.35, linewidth=6):
    return [pe.withStroke(linewidth=linewidth, foreground=color + "60")]


def _panel_frame(ax, title, letter, title_color=GOLD):
    """Apply Aurphyx void-panel styling + letter badge."""
    ax.set_facecolor(PANEL_BG)
    for spine in ax.spines.values():
        spine.set_edgecolor(VIOLET)
        spine.set_linewidth(0.8)
    ax.text(0.01, 0.97, f"[{letter}]", transform=ax.transAxes,
            fontsize=7, color=GOLD, fontweight="bold", va="top", fontfamily="monospace")
    ax.text(0.06, 0.97, title, transform=ax.transAxes,
            fontsize=7, color=title_color, va="top", fontfamily="monospace",
            path_effects=_glow(title_color, alpha=0.2, linewidth=4))


# ══════════════════════════════════════════════════════════════════════════════
# Panel A — HIF Field Equation & Triple Threshold Gate
# ══════════════════════════════════════════════════════════════════════════════
def panel_A_hif_equation(ax):
    _panel_frame(ax, "HIF Field Equation & Triple Threshold Gate", "A")
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis("off")

    lines = [
        (0.5, 0.90, r"$\mathrm{HIF}(x,t) = \sqrt[3]{C \cdot R \cdot A} \cdot \Phi(C,R,A)$", CYAN, 9),
        (0.5, 0.78, r"$\Phi(C,R,A) = \begin{cases} 1 & C{\ge}C_{\theta},\,R{\ge}R_{\theta},\,A{\ge}A_{\theta} \\ 0 & \text{otherwise} \end{cases}$", SILVER, 8),
        (0.5, 0.60, "Triple Threshold Gate", GOLD, 8),
    ]
    for x, y, txt, col, fs in lines:
        ax.text(x, y, txt, ha="center", va="center",
                fontsize=fs, color=col, fontfamily="monospace",
                path_effects=_glow(col, linewidth=3))

    # Three threshold nodes
    nodes = [
        (0.20, 0.38, "C", CYAN,   r"$C \ge C_{\theta}$"),
        (0.50, 0.38, "R", VIOLET, r"$R \ge R_{\theta}$"),
        (0.80, 0.38, "A", MINT,   r"$A \ge A_{\theta}$"),
    ]
    gate_colors = [CYAN, VIOLET, MINT]
    for (cx, cy, lbl, col, thresh) in nodes:
        circle = Circle((cx, cy), 0.07, transform=ax.transAxes,
                        facecolor=col + "22", edgecolor=col, linewidth=1.5,
                        zorder=5)
        ax.add_patch(circle)
        ax.text(cx, cy + 0.002, lbl, ha="center", va="center",
                transform=ax.transAxes, fontsize=9, color=col,
                fontweight="bold", fontfamily="monospace",
                path_effects=_glow(col, linewidth=4))
        ax.text(cx, cy - 0.13, thresh, ha="center", va="top",
                transform=ax.transAxes, fontsize=6.5, color=col,
                fontfamily="monospace")

    # Arrows to central AND gate
    for cx in [0.20, 0.50, 0.80]:
        ax.annotate("", xy=(0.50, 0.18), xytext=(cx, 0.31),
                    xycoords="axes fraction", textcoords="axes fraction",
                    arrowprops=dict(arrowstyle="->", color=GOLD, lw=1.2))

    # AND gate box
    box = FancyBboxPatch((0.35, 0.08), 0.30, 0.10,
                         boxstyle="round,pad=0.02",
                         facecolor=GOLD + "22", edgecolor=GOLD, linewidth=1.5,
                         transform=ax.transAxes, zorder=6)
    ax.add_patch(box)
    ax.text(0.50, 0.13, r"$\Phi = 1$ (HIF active)", ha="center", va="center",
            transform=ax.transAxes, fontsize=7, color=GOLD, fontfamily="monospace",
            path_effects=_glow(GOLD, linewidth=3))

    ax.text(0.50, 0.02, "Source: hif_protocol_spec.md §2", ha="center",
            transform=ax.transAxes, **FONT_SMALL)


# ══════════════════════════════════════════════════════════════════════════════
# Panel B — HIF 3-State Machine
# ══════════════════════════════════════════════════════════════════════════════
def panel_B_state_machine(ax):
    _panel_frame(ax, "HIF State Machine (3-State)", "B")
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis("off")

    states = [
        (0.50, 0.82, "CREATION",    MINT,   r"$\mathrm{HIF} \ge H_{create}$"),
        (0.20, 0.35, "INTEGRATION", CYAN,   r"$H_{int} \le \mathrm{HIF} < H_{create}$"),
        (0.80, 0.35, "RENEWAL",     ROSE,   r"$\mathrm{HIF} < H_{renew}$"),
    ]
    state_positions = [(0.50, 0.82), (0.20, 0.35), (0.80, 0.35)]
    r = 0.09

    for (cx, cy, name, col, cond) in states:
        circle = Circle((cx, cy), r, transform=ax.transAxes,
                        facecolor=col + "28", edgecolor=col, linewidth=2, zorder=5)
        ax.add_patch(circle)
        ax.text(cx, cy + 0.01, name, ha="center", va="center",
                transform=ax.transAxes, fontsize=6.5, color=col,
                fontweight="bold", fontfamily="monospace",
                path_effects=_glow(col, linewidth=3))
        ax.text(cx, cy - r - 0.06, cond, ha="center", va="top",
                transform=ax.transAxes, fontsize=5.5, color=col,
                fontfamily="monospace")

    # Transition arrows
    transitions = [
        # (from_idx, to_idx, label, offset_x, offset_y)
        (0, 1, "HIF drops",        -0.06, 0.0),
        (1, 0, "HIF rises",         0.06, 0.0),
        (1, 2, "correction fails", 0.0,   0.06),
        (2, 1, "restored",          0.0,  -0.06),
        (0, 2, "HIF critical drop", 0.06,  0.0),
        (2, 0, "full renewal",     -0.06,  0.0),
    ]
    arrow_cols = [SILVER, MINT, ROSE, CYAN, ROSE, MINT]
    for i, (fi, ti, lbl, ox, oy) in enumerate(transitions):
        fx, fy = state_positions[fi]
        tx, ty = state_positions[ti]
        mx = (fx + tx) / 2 + ox * 0.5
        my = (fy + ty) / 2 + oy * 0.5
        col = arrow_cols[i]
        ax.annotate("", xy=(tx, ty), xytext=(fx, fy),
                    xycoords="axes fraction", textcoords="axes fraction",
                    arrowprops=dict(arrowstyle="->", color=col, lw=1.0,
                                   connectionstyle="arc3,rad=0.25"))
        ax.text(mx, my, lbl, ha="center", va="center",
                transform=ax.transAxes, fontsize=5, color=col,
                fontfamily="monospace")

    ax.text(0.50, 0.02, "Source: hif_protocol_spec.md §4", ha="center",
            transform=ax.transAxes, **FONT_SMALL)


# ══════════════════════════════════════════════════════════════════════════════
# Panel C — HIF Sigil (symbolic architecture)
# ══════════════════════════════════════════════════════════════════════════════
def panel_C_sigil(ax):
    _panel_frame(ax, "HIF Sigil — Symbolic Architecture", "C", GOLD)
    ax.set_xlim(-1.2, 1.2)
    ax.set_ylim(-1.2, 1.2)
    ax.set_aspect("equal")
    ax.axis("off")

    # Threshold Ring (broken into 3 segments)
    ring_r = 0.90
    for i, (start_deg, col, lbl) in enumerate(
            [(90, CYAN, "C"), (210, VIOLET, "R"), (330, MINT, "A")]):
        theta = np.linspace(np.radians(start_deg + 10),
                            np.radians(start_deg + 110), 60)
        ax.plot(ring_r * np.cos(theta), ring_r * np.sin(theta),
                color=col, lw=2.5, alpha=0.85,
                path_effects=_glow(col, linewidth=5))
        mid = np.radians(start_deg + 60)
        ax.text(1.05 * ring_r * np.cos(mid), 1.05 * ring_r * np.sin(mid),
                lbl, ha="center", va="center", fontsize=9, color=col,
                fontweight="bold", fontfamily="monospace")

    # Triple Torch Triad
    triad_r = 0.50
    for i, (angle, col, name) in enumerate(
            [(90, CYAN, "C"), (210, VIOLET, "R"), (330, MINT, "A")]):
        rad = np.radians(angle)
        px, py = triad_r * np.cos(rad), triad_r * np.sin(rad)
        c = Circle((px, py), 0.12, facecolor=col + "33", edgecolor=col,
                   linewidth=1.5, zorder=5)
        ax.add_patch(c)
        ax.text(px, py, name, ha="center", va="center", fontsize=8,
                color=col, fontweight="bold", fontfamily="monospace",
                path_effects=_glow(col, linewidth=4))
        ax.plot([0, px * 0.85], [0, py * 0.85], color=col,
                lw=1.2, alpha=0.6, linestyle="--")

    # Central Harmonic Root Operator  ∛(C·R·A)
    ax.text(0, 0.05, r"$\sqrt[3]{C{\cdot}R{\cdot}A}$",
            ha="center", va="center", fontsize=9, color=GOLD,
            fontfamily="monospace",
            path_effects=_glow(GOLD, linewidth=5))
    core = Circle((0, 0), 0.18, facecolor=GOLD + "18", edgecolor=GOLD,
                  linewidth=1.8, zorder=4)
    ax.add_patch(core)

    # Renewal Serpent (coiled arc below)
    theta_s = np.linspace(np.radians(200), np.radians(340), 80)
    rs = 0.68 + 0.06 * np.sin(4 * theta_s)
    ax.plot(rs * np.cos(theta_s), rs * np.sin(theta_s),
            color=ROSE, lw=1.8, alpha=0.75,
            path_effects=_glow(ROSE, linewidth=4))
    ax.text(0, -1.0, "Renewal Serpent", ha="center", va="center",
            fontsize=5.5, color=ROSE, fontfamily="monospace")

    # Sentinel Key (right)
    ax.text(1.05, 0.0, "⚿", ha="center", va="center",
            fontsize=14, color=GOLD,
            path_effects=_glow(GOLD, linewidth=4))
    ax.text(1.05, -0.18, "Sentinel Key", ha="center", va="top",
            fontsize=5, color=GOLD, fontfamily="monospace")

    # Guardian Hound (left)
    ax.text(-1.05, 0.0, "⬡", ha="center", va="center",
            fontsize=12, color=VIOLET,
            path_effects=_glow(VIOLET, linewidth=4))
    ax.text(-1.05, -0.18, "Guardian Hound", ha="center", va="top",
            fontsize=5, color=VIOLET, fontfamily="monospace")

    # Liminal Crescent (above)
    theta_c = np.linspace(np.radians(30), np.radians(150), 60)
    ax.plot(0.75 * np.cos(theta_c), 0.75 * np.sin(theta_c) + 0.55,
            color=SILVER, lw=1.5, alpha=0.7,
            path_effects=_glow(SILVER, linewidth=3))
    ax.text(0, 1.05, "Liminal Crescent", ha="center", va="center",
            fontsize=5.5, color=SILVER, fontfamily="monospace")

    ax.text(0, -1.15, "Source: hif_symbol_sigil.md §2–3",
            ha="center", va="center", fontsize=5, color=DIM,
            fontfamily="monospace")


# ══════════════════════════════════════════════════════════════════════════════
# Panel D — HIF Gradient / Stability Index Phase Plane
# ══════════════════════════════════════════════════════════════════════════════
def panel_D_gradient(ax):
    _panel_frame(ax, "HIF Gradient & Stability Index Phase Plane", "D")
    ax.set_facecolor(PANEL_BG)

    x = np.linspace(-2, 2, 200)
    y = np.linspace(-2, 2, 200)
    X, Y = np.meshgrid(x, y)

    # Synthetic HIF field: geometric mean of three Gaussian sub-fields
    C_field = np.exp(-((X - 0.3)**2 + (Y - 0.3)**2) / 0.8)
    R_field = np.exp(-((X + 0.2)**2 + (Y - 0.1)**2) / 0.9)
    A_field = np.exp(-((X - 0.0)**2 + (Y + 0.2)**2) / 0.7)
    HIF = (C_field * R_field * A_field) ** (1/3)

    # Threshold mask  (Φ = 1 zone)
    thresh = 0.35
    mask = (C_field >= thresh) & (R_field >= thresh) & (A_field >= thresh)
    HIF_active = np.where(mask, HIF, np.nan)

    # Contour fill
    cmap_hif = LinearSegmentedColormap.from_list(
        "hif", [PANEL_BG, GLOW_VIO, VIOLET, CYAN, WHITE])
    cf = ax.contourf(X, Y, HIF, levels=20, cmap=cmap_hif, alpha=0.75)

    # Threshold ring contour
    ax.contour(X, Y, C_field, levels=[thresh], colors=[CYAN], linewidths=0.8, linestyles="--")
    ax.contour(X, Y, R_field, levels=[thresh], colors=[VIOLET], linewidths=0.8, linestyles="--")
    ax.contour(X, Y, A_field, levels=[thresh], colors=[MINT], linewidths=0.8, linestyles="--")

    # HIF active zone fill
    ax.contourf(X, Y, mask.astype(float), levels=[0.5, 1.5],
                colors=[GOLD + "33"], alpha=0.35)
    ax.contour(X, Y, mask.astype(float), levels=[0.5],
               colors=[GOLD], linewidths=1.2)

    # Gradient flow arrows
    skip = 14
    dHIF_y, dHIF_x = np.gradient(HIF, y, x)
    ax.quiver(X[::skip, ::skip], Y[::skip, ::skip],
              dHIF_x[::skip, ::skip], dHIF_y[::skip, ::skip],
              color=SILVER, alpha=0.55, scale=6.0, width=0.004)

    # Stability index S = ∇²HIF
    lap = (np.roll(HIF, 1, 0) + np.roll(HIF, -1, 0) +
           np.roll(HIF, 1, 1) + np.roll(HIF, -1, 1) - 4 * HIF)
    ax.contour(X, Y, lap, levels=[0], colors=[ROSE], linewidths=1.2,
               linestyles=":")

    ax.set_xlim(-2, 2)
    ax.set_ylim(-2, 2)
    ax.set_xlabel(r"$C$ axis", fontsize=6, color=CYAN, fontfamily="monospace")
    ax.set_ylabel(r"$A$ axis", fontsize=6, color=MINT, fontfamily="monospace")
    ax.tick_params(colors=DIM, labelsize=5)
    for spine in ax.spines.values():
        spine.set_edgecolor(VIOLET)

    # Legend
    legend_items = [
        mpatches.Patch(facecolor=GOLD + "55", edgecolor=GOLD, label=r"$\Phi=1$ active zone"),
        mpatches.Patch(facecolor="none", edgecolor=ROSE, linestyle=":", label=r"$S=\nabla^2\mathrm{HIF}=0$"),
        mpatches.Patch(facecolor="none", edgecolor=CYAN, linestyle="--", label=r"$C_{\theta}$ contour"),
    ]
    ax.legend(handles=legend_items, loc="lower right", fontsize=4.5,
              facecolor=PANEL_BG, edgecolor=DIM, labelcolor=SILVER)
    ax.text(0.02, 0.03, "Source: hif_protocol_spec.md §6",
            transform=ax.transAxes, **FONT_SMALL)


# ══════════════════════════════════════════════════════════════════════════════
# Panel E — HIF Spectral / Resonance Bands
# ══════════════════════════════════════════════════════════════════════════════
def panel_E_spectral(ax):
    _panel_frame(ax, "HIF Spectral Protocol — Resonance Bands", "E")
    ax.set_facecolor(PANEL_BG)

    omega = np.linspace(0, 4 * np.pi, 1000)

    # Three resonance sub-field spectral envelopes
    R_spec  = np.abs(np.fft.fftshift(np.fft.fft(
        np.exp(-0.5 * (omega - 4)**2 / 0.8) * np.cos(3.2 * omega))))
    C_spec  = np.abs(np.fft.fftshift(np.fft.fft(
        np.exp(-0.5 * (omega - 5)**2 / 0.6) * np.cos(4.0 * omega))))
    A_spec  = np.abs(np.fft.fftshift(np.fft.fft(
        np.exp(-0.5 * (omega - 3)**2 / 1.0) * np.cos(2.8 * omega))))

    freq = np.linspace(-50, 50, len(R_spec))
    norm = lambda s: s / s.max()

    ax.fill_between(freq, norm(C_spec), alpha=0.25, color=CYAN,   label=r"$\mathcal{F}[C(x,t)]$")
    ax.fill_between(freq, norm(R_spec), alpha=0.25, color=VIOLET, label=r"$\mathcal{F}[R(x,t)]$")
    ax.fill_between(freq, norm(A_spec), alpha=0.25, color=MINT,   label=r"$\mathcal{F}[A(x,t)]$")
    ax.plot(freq, norm(C_spec), color=CYAN,   lw=1.0,
            path_effects=_glow(CYAN,   linewidth=3))
    ax.plot(freq, norm(R_spec), color=VIOLET, lw=1.0,
            path_effects=_glow(VIOLET, linewidth=3))
    ax.plot(freq, norm(A_spec), color=MINT,   lw=1.0,
            path_effects=_glow(MINT,   linewidth=3))

    # HIF composite (geometric mean of normalized spectra)
    HIF_spec = norm((norm(C_spec) * norm(R_spec) * norm(A_spec)) ** (1/3))
    ax.plot(freq, HIF_spec, color=GOLD, lw=1.8, label=r"$\mathcal{F}[\mathrm{HIF}]$",
            path_effects=_glow(GOLD, linewidth=5))

    # Mismatch zone
    ax.axvspan(15, 35,  alpha=0.12, color=ROSE,  label="Spectral mismatch zone")
    ax.axvspan(-35, -15, alpha=0.12, color=ROSE)

    # Compatibility band
    ax.axvspan(-10, 10, alpha=0.10, color=GOLD, label="Compatibility band")

    ax.set_xlim(-50, 50)
    ax.set_ylim(0, 1.15)
    ax.set_xlabel(r"Frequency $\omega$ (normalized)", fontsize=6,
                  color=SILVER, fontfamily="monospace")
    ax.set_ylabel(r"Spectral Power (normalized)", fontsize=6,
                  color=SILVER, fontfamily="monospace")
    ax.tick_params(colors=DIM, labelsize=5)
    for spine in ax.spines.values():
        spine.set_edgecolor(VIOLET)
    ax.legend(loc="upper right", fontsize=4.5,
              facecolor=PANEL_BG, edgecolor=DIM, labelcolor=SILVER)
    ax.text(0.02, 0.03, "Source: hif_protocol_spec.md §7",
            transform=ax.transAxes, **FONT_SMALL)


# ══════════════════════════════════════════════════════════════════════════════
# Panel F — AUFE (Aurphyx Unified Field Equation) Decomposition Map
# ══════════════════════════════════════════════════════════════════════════════
def panel_F_aufe(ax):
    _panel_frame(ax, "AUFE — Aurphyx Unified Field Equation", "F")
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis("off")

    # Main equation
    ax.text(0.5, 0.93,
            r"$\frac{d\mathbf{x}}{dt} = -\nabla V + \mathcal{D}(D(t)) + \mathcal{S}(\mathbf{x},\mathcal{R}_{24}) + \mathcal{K}(\mathbf{x})$",
            ha="center", va="center", fontsize=7.5, color=GOLD,
            fontfamily="monospace",
            path_effects=_glow(GOLD, linewidth=4))
    ax.text(0.5, 0.85,
            r"where $V(\mathbf{x}) = (\beta - 1)^2$,   $\beta = \frac{x_f x_c}{x_i x_t}$",
            ha="center", va="center", fontsize=7, color=CYAN,
            fontfamily="monospace")

    # Four term blocks
    terms = [
        (0.13, 0.62, r"$-\nabla V$",               ROSE,   "Restoring\nForce",
         [r"$V=(\beta-1)^2$",
          r"$F_{restore}=-2(\beta-1)\nabla\beta$"]),
        (0.38, 0.62, r"$\mathcal{D}(D(t))$",        CYAN,   "HRD\nPerturbation",
         [r"$D(t)=A(t)\sin(\omega t+\phi)$",
          r"$\sigma\cdot D(t)\cdot\mathbf{u}_{harm}$"]),
        (0.63, 0.62, r"$\mathcal{S}(\mathbf{x},\mathcal{R}_{24})$", VIOLET, "Routing\nOperator",
         [r"$\mathbf{R}_{24}(\mathbf{x})\cdot\mathbf{x}$",
          "Tetra-Hexa Array"]),
        (0.88, 0.62, r"$\mathcal{K}(\mathbf{x})$",  MINT,   "Duality\nKernel",
         [r"$\mathcal{F}+\mathcal{Y}+\mathcal{T}$",
          "FuxRT · YezRT · FUTE"]),
    ]
    for (cx, cy, label, col, title, details) in terms:
        box = FancyBboxPatch((cx - 0.10, cy - 0.09), 0.20, 0.18,
                             boxstyle="round,pad=0.02",
                             facecolor=col + "1a", edgecolor=col, linewidth=1.2,
                             transform=ax.transAxes, zorder=5)
        ax.add_patch(box)
        ax.text(cx, cy + 0.06, label, ha="center", va="center",
                transform=ax.transAxes, fontsize=7, color=col,
                fontfamily="monospace",
                path_effects=_glow(col, linewidth=3))
        ax.text(cx, cy - 0.01, title, ha="center", va="center",
                transform=ax.transAxes, fontsize=5.5, color=WHITE,
                fontfamily="monospace")
        for di, det in enumerate(details):
            ax.text(cx, cy - 0.06 - di * 0.045, det, ha="center", va="center",
                    transform=ax.transAxes, fontsize=4.5, color=DIM,
                    fontfamily="monospace")

    # HIF integration arrow
    ax.annotate("", xy=(0.50, 0.30), xytext=(0.50, 0.52),
                xycoords="axes fraction", textcoords="axes fraction",
                arrowprops=dict(arrowstyle="->", color=GOLD, lw=1.5))
    ax.text(0.52, 0.41, "HIF governs all", ha="left",
            transform=ax.transAxes, fontsize=5.5, color=GOLD,
            fontfamily="monospace")

    # Balance Invariants box
    inv_box = FancyBboxPatch((0.05, 0.08), 0.90, 0.20,
                             boxstyle="round,pad=0.02",
                             facecolor=GOLD + "0d", edgecolor=GOLD + "55",
                             linewidth=0.8, transform=ax.transAxes, zorder=4)
    ax.add_patch(inv_box)
    ax.text(0.5, 0.27, "Balance Invariants", ha="center", va="center",
            transform=ax.transAxes, fontsize=6.5, color=GOLD,
            fontfamily="monospace", fontweight="bold")
    inv_items = [
        r"$\beta=1$ invariant under Balance-preserving $T$",
        r"$V_{min}=0$ on $\mathcal{B}_{EM}$",
        r"$\nabla\cdot\mathbf{F}_{Balance}<0$ (convergent attractor)",
        r"$\mathrm{HIF}\ge H_{govern}$ for governance legitimacy",
    ]
    for i, item in enumerate(inv_items):
        ax.text(0.5, 0.21 - i * 0.032, item, ha="center", va="center",
                transform=ax.transAxes, fontsize=5, color=SILVER,
                fontfamily="monospace")

    ax.text(0.5, 0.02, "Source: EQUATION_EXTRACTION_SUPPLEMENT.md Appendix I, §XIV",
            ha="center", transform=ax.transAxes, **FONT_SMALL)


# ══════════════════════════════════════════════════════════════════════════════
# Master Figure Assembly
# ══════════════════════════════════════════════════════════════════════════════
def build_figure(output_dir="./output", dpi=300, save_svg=True):
    os.makedirs(output_dir, exist_ok=True)

    fig = plt.figure(figsize=(20, 13), facecolor=BG)
    gs = GridSpec(3, 3, figure=fig,
                  hspace=0.42, wspace=0.32,
                  left=0.04, right=0.97,
                  top=0.91, bottom=0.04)

    axes = [
        fig.add_subplot(gs[1, 0]),  # A
        fig.add_subplot(gs[1, 1]),  # B
        fig.add_subplot(gs[1, 2]),  # C
        fig.add_subplot(gs[2, 0]),  # D
        fig.add_subplot(gs[2, 1]),  # E
        fig.add_subplot(gs[2, 2]),  # F
    ]

    panel_A_hif_equation(axes[0])
    panel_B_state_machine(axes[1])
    panel_C_sigil(axes[2])
    panel_D_gradient(axes[3])
    panel_E_spectral(axes[4])
    panel_F_aufe(axes[5])

    # ── Title row (gs[0, :]) ──────────────────────────────────────────────────
    ax_title = fig.add_subplot(gs[0, :])
    ax_title.set_facecolor(BG)
    ax_title.axis("off")

    ax_title.text(0.5, 0.82,
                  "Harmonic Integrity Field (HIF) Architecture",
                  ha="center", va="center", fontsize=18,
                  color=GOLD, fontweight="bold", fontfamily="monospace",
                  path_effects=_glow(GOLD, alpha=0.4, linewidth=8))
    ax_title.text(0.5, 0.55,
                  "Aurphyx Primordial Standards  ·  Group 2 Figure",
                  ha="center", va="center", fontsize=10,
                  color=VIOLET, fontfamily="monospace")
    ax_title.text(0.5, 0.30,
                  r"$\mathrm{HIF}(x,t)=\sqrt[3]{C\cdot R\cdot A}\cdot\Phi(C,R,A)$   "
                  r"$\quad\Phi=1\iff C{\ge}C_{\theta},\,R{\ge}R_{\theta},\,A{\ge}A_{\theta}$",
                  ha="center", va="center", fontsize=9,
                  color=CYAN, fontfamily="monospace",
                  path_effects=_glow(CYAN, linewidth=4))

    # Horizontal rule
    ax_title.axhline(0.08, color=VIOLET + "66", lw=0.8, xmin=0.02, xmax=0.98)

    # Footer
    fig.text(0.02, 0.005,
             "Ross Edwards & Aurphyx LLC  ·  ORCiD: 0009-0008-0539-1289  "
             "·  fig_gen_group2_hif.py  ·  R.F. Lovezme  ·  2026-06-22",
             ha="left", va="bottom", fontsize=5.5,
             color=DIM, fontfamily="monospace")
    fig.text(0.98, 0.005,
             "Sources: hif_protocol_spec.md · hif_symbol_sigil.md · "
             "EQUATION_EXTRACTION_SUPPLEMENT.md",
             ha="right", va="bottom", fontsize=5.5,
             color=DIM, fontfamily="monospace")

    png_path = os.path.join(output_dir, "fig_group2_hif.png")
    fig.savefig(png_path, dpi=dpi, bbox_inches="tight",
                facecolor=BG, edgecolor="none")
    print(f"[HIF] PNG saved → {png_path}")

    if save_svg:
        svg_path = os.path.join(output_dir, "fig_group2_hif.svg")
        fig.savefig(svg_path, format="svg", bbox_inches="tight",
                    facecolor=BG, edgecolor="none")
        print(f"[HIF] SVG saved → {svg_path}")

    plt.close(fig)
    return png_path


# ══════════════════════════════════════════════════════════════════════════════
# CLI Entry Point
# ══════════════════════════════════════════════════════════════════════════════
def main():
    parser = argparse.ArgumentParser(
        description="Generate Group 2 HIF architecture figure (Aurphyx Primordial Standards)")
    parser.add_argument("--output", default="./output",
                        help="Output directory (default: ./output)")
    parser.add_argument("--dpi", type=int, default=300,
                        help="PNG resolution in DPI (default: 300)")
    parser.add_argument("--no-svg", action="store_true",
                        help="Skip SVG export")
    args = parser.parse_args()
    build_figure(output_dir=args.output, dpi=args.dpi,
                 save_svg=not args.no_svg)


if __name__ == "__main__":
    main()
