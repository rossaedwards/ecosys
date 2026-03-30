#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
Figure 8: Majorana T-Junction Device Schematic
PRX Submission - Aurphyx

Generates publication-quality schematic of the T-shape 6-dot Majorana
device for Protocol 4, including gate layout and braiding geometry.
"""

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle, Circle, FancyBboxPatch, Polygon, Ellipse
from matplotlib.patches import FancyArrowPatch, Arc
from matplotlib.collections import PatchCollection
import matplotlib.gridspec as gridspec

# Publication settings
plt.rcParams.update({
    'font.family': 'serif',
    'font.size': 10,
    'axes.labelsize': 11,
    'axes.titlesize': 12,
    'xtick.labelsize': 9,
    'ytick.labelsize': 9,
    'legend.fontsize': 9,
    'figure.dpi': 300,
    'savefig.dpi': 300,
    'savefig.bbox': 'tight',
    'text.usetex': False,
})


def draw_t_junction_device(ax):
    """Draw the T-shape 6-dot Majorana device schematic."""
    
    # InP substrate
    substrate = Rectangle((-1, -0.8), 8, 0.5, facecolor='#D4A574', edgecolor='black')
    ax.add_patch(substrate)
    ax.text(3.5, -0.55, 'InP substrate', ha='center', va='center', fontsize=9)
    
    # InSb nanowire - main horizontal segment (spine)
    wire_h = 0.25
    wire_color = '#4169E1'  # Royal blue
    
    # Horizontal spine (4 dots)
    spine = Rectangle((0, 0), 5, wire_h, facecolor=wire_color, edgecolor='black', linewidth=1)
    ax.add_patch(spine)
    
    # Vertical branch 1 (at position 2)
    branch1 = Rectangle((1.5, 0), wire_h, 1.5, facecolor=wire_color, edgecolor='black', linewidth=1)
    ax.add_patch(branch1)
    
    # Vertical branch 2 (at position 3)
    branch2 = Rectangle((2.75, 0), wire_h, 1.5, facecolor=wire_color, edgecolor='black', linewidth=1)
    ax.add_patch(branch2)
    
    ax.text(5.3, 0.125, 'InSb nanowire\n(d = 100 nm)', ha='left', va='center', fontsize=8)
    
    # Al superconducting shell (half-shell)
    al_color = '#C0C0C0'
    al_spine = Rectangle((0, wire_h), 5, 0.08, facecolor=al_color, edgecolor='gray', linewidth=0.5)
    ax.add_patch(al_spine)
    al_branch1 = Rectangle((1.5 + wire_h, 0.25), 0.08, 1.25, facecolor=al_color, edgecolor='gray', linewidth=0.5)
    ax.add_patch(al_branch1)
    al_branch2 = Rectangle((2.75 + wire_h, 0.25), 0.08, 1.25, facecolor=al_color, edgecolor='gray', linewidth=0.5)
    ax.add_patch(al_branch2)
    
    ax.text(5.3, wire_h + 0.04, 'Al shell (7 nm)', ha='left', va='center', fontsize=7, color='gray')
    
    # Quantum dots (6 total: 4 on spine + 2 on branches)
    dot_radius = 0.12
    dot_color = '#FFD700'  # Gold
    
    dot_positions = [
        (0.5, 0.125, '1'),      # Spine dot 1
        (1.625, 0.125, '2'),    # Spine dot 2 (junction)
        (2.875, 0.125, '3'),    # Spine dot 3 (junction)
        (4.5, 0.125, '4'),      # Spine dot 4
        (1.625, 1.2, '5'),      # Branch 1 dot
        (2.875, 1.2, '6'),      # Branch 2 dot
    ]
    
    for x, y, label in dot_positions:
        dot = Circle((x, y), dot_radius, facecolor=dot_color, edgecolor='black', linewidth=1, zorder=5)
        ax.add_patch(dot)
        ax.text(x, y, label, ha='center', va='center', fontsize=8, fontweight='bold', zorder=6)
    
    # Majorana zero modes (MZMs) at wire ends
    mzm_color = '#E94F37'
    mzm_positions = [
        (0.15, 0.125),    # Left end
        (4.85, 0.125),    # Right end
        (1.625, 1.45),    # Branch 1 top
        (2.875, 1.45),    # Branch 2 top
    ]
    
    for x, y in mzm_positions:
        mzm = Circle((x, y), 0.08, facecolor=mzm_color, edgecolor='black', linewidth=1, zorder=5)
        ax.add_patch(mzm)
    
    ax.text(0.15, 0.4, 'γ₁', ha='center', fontsize=9, color=mzm_color, fontweight='bold')
    ax.text(4.85, 0.4, 'γ₄', ha='center', fontsize=9, color=mzm_color, fontweight='bold')
    ax.text(1.625, 1.7, 'γ₂', ha='center', fontsize=9, color=mzm_color, fontweight='bold')
    ax.text(2.875, 1.7, 'γ₃', ha='center', fontsize=9, color=mzm_color, fontweight='bold')
    
    # Gate electrodes (plunger gates)
    gate_color = '#90EE90'
    gate_positions = [
        (0.5, -0.25, 'P₁'),
        (1.625, -0.25, 'P₂'),
        (2.875, -0.25, 'P₃'),
        (4.5, -0.25, 'P₄'),
        (1.4, 0.7, 'P₅'),
        (2.65, 0.7, 'P₆'),
    ]
    
    for x, y, label in gate_positions:
        gate = Rectangle((x - 0.1, y - 0.08), 0.2, 0.08, facecolor=gate_color, 
                         edgecolor='darkgreen', linewidth=1)
        ax.add_patch(gate)
        ax.text(x, y - 0.2, label, ha='center', va='top', fontsize=7, color='darkgreen')
    
    # Barrier gates
    barrier_color = '#FFB6C1'
    barrier_positions = [
        (1.0, 0.125),   # Between 1-2
        (2.25, 0.125),  # Between 2-3
        (3.6, 0.125),   # Between 3-4
        (1.625, 0.6),   # Branch 1
        (2.875, 0.6),   # Branch 2
    ]
    
    for x, y in barrier_positions:
        barrier = Rectangle((x - 0.06, y - 0.125), 0.12, 0.25, facecolor=barrier_color,
                            edgecolor='#8B0000', linewidth=0.5, alpha=0.7)
        ax.add_patch(barrier)
    
    # Legend
    legend_y = 2.2
    ax.add_patch(Circle((-0.5, legend_y), 0.08, facecolor=dot_color, edgecolor='black'))
    ax.text(-0.25, legend_y, 'Quantum dot', va='center', fontsize=8)
    
    ax.add_patch(Circle((1.5, legend_y), 0.08, facecolor=mzm_color, edgecolor='black'))
    ax.text(1.75, legend_y, 'Majorana ZM', va='center', fontsize=8)
    
    ax.add_patch(Rectangle((3.3, legend_y - 0.06), 0.15, 0.12, facecolor=gate_color, edgecolor='darkgreen'))
    ax.text(3.6, legend_y, 'Plunger gate', va='center', fontsize=8)
    
    ax.add_patch(Rectangle((5.2, legend_y - 0.06), 0.1, 0.12, facecolor=barrier_color, edgecolor='#8B0000'))
    ax.text(5.45, legend_y, 'Barrier', va='center', fontsize=8)
    
    ax.set_xlim(-1.2, 6.5)
    ax.set_ylim(-1, 2.6)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_title('(a) T-Shape 6-Dot Majorana Device', fontsize=11, fontweight='bold')


def draw_braiding_sequence(ax):
    """Draw the braiding sequence for T-junction."""
    
    # Three snapshots of MZM positions during braiding
    snapshots = [
        # Initial: γ₁ left, γ₂ top-left, γ₃ top-right, γ₄ right
        {'title': 't = 0', 'positions': [(0, 0), (1, 1.5), (2, 1.5), (3, 0)]},
        # Mid: γ₂ moving to junction
        {'title': 't = τ/2', 'positions': [(0, 0), (1.5, 0.5), (2, 1.5), (3, 0)]},
        # Final: γ₂ and γ₃ exchanged
        {'title': 't = τ', 'positions': [(0, 0), (2, 1.5), (1, 1.5), (3, 0)]},
    ]
    
    colors = ['#E94F37', '#2E86AB', '#F18F01', '#6B8E23']
    labels = ['γ₁', 'γ₂', 'γ₃', 'γ₄']
    
    for i, snap in enumerate(snapshots):
        x_offset = i * 4
        
        # Draw T-junction wire outline
        wire_x = [x_offset, x_offset + 3]
        wire_y = [0, 0]
        ax.plot(wire_x, wire_y, 'k-', linewidth=3)
        ax.plot([x_offset + 1, x_offset + 1], [0, 1.5], 'k-', linewidth=3)
        ax.plot([x_offset + 2, x_offset + 2], [0, 1.5], 'k-', linewidth=3)
        
        # Draw MZMs
        for j, (x, y) in enumerate(snap['positions']):
            circle = Circle((x_offset + x, y), 0.15, facecolor=colors[j], 
                           edgecolor='black', linewidth=1, zorder=5)
            ax.add_patch(circle)
            ax.text(x_offset + x, y, labels[j], ha='center', va='center',
                   fontsize=8, fontweight='bold', color='white', zorder=6)
        
        # Time label
        ax.text(x_offset + 1.5, -0.5, snap['title'], ha='center', fontsize=9)
        
        # Arrow to next snapshot
        if i < len(snapshots) - 1:
            ax.annotate('', xy=(x_offset + 3.5, 0.75), xytext=(x_offset + 3.2, 0.75),
                       arrowprops=dict(arrowstyle='->', color='black', lw=1.5))
    
    # Braiding result annotation
    ax.text(6, 2.2, 'Result: γ₂ ↔ γ₃ exchange\nPhase: θ = π/4', 
            ha='center', fontsize=9,
            bbox=dict(boxstyle='round', facecolor='lightyellow', edgecolor='gray'))
    
    ax.set_xlim(-0.5, 12)
    ax.set_ylim(-1, 2.8)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_title('(b) Adiabatic Braiding Sequence', fontsize=11, fontweight='bold')


def draw_measurement_scheme(ax):
    """Draw the RF reflectometry measurement scheme."""
    
    # Device representation
    device = FancyBboxPatch((1, 0.5), 2, 1.5, boxstyle='round,pad=0.05',
                             facecolor='#E8E8E8', edgecolor='black', linewidth=1.5)
    ax.add_patch(device)
    ax.text(2, 1.25, 'T-junction\ndevice', ha='center', va='center', fontsize=9)
    
    # RF source
    rf_source = FancyBboxPatch((-1, 1), 1.5, 0.8, boxstyle='round,pad=0.05',
                                facecolor='#87CEEB', edgecolor='black')
    ax.add_patch(rf_source)
    ax.text(-0.25, 1.4, 'RF source\n(~100 MHz)', ha='center', va='center', fontsize=8)
    
    # Directional coupler
    coupler = Circle((0.5, 1.4), 0.2, facecolor='white', edgecolor='black')
    ax.add_patch(coupler)
    ax.text(0.5, 1.4, 'DC', ha='center', va='center', fontsize=7)
    
    # Connections
    ax.plot([0.5, 1], [1.4, 1.4], 'k-', linewidth=1.5)  # To device
    ax.plot([0.2, 0.5], [1.4, 1.4], 'k-', linewidth=1.5)  # From RF
    
    # Reflected signal
    ax.annotate('', xy=(0.5, 0.8), xytext=(0.5, 1.2),
               arrowprops=dict(arrowstyle='->', color='#E94F37', lw=1.5))
    
    # Digitizer
    digitizer = FancyBboxPatch((-0.5, -0.5), 2, 0.8, boxstyle='round,pad=0.05',
                                facecolor='#90EE90', edgecolor='black')
    ax.add_patch(digitizer)
    ax.text(0.5, -0.1, 'Digitizer\n(10 kHz BW)', ha='center', va='center', fontsize=8)
    
    ax.plot([0.5, 0.5], [0.3, 0.8], 'k-', linewidth=1.5)
    
    # Parity readout equation
    ax.text(3.5, 0, r'$P_z = (-1)^{n_L + n_R}$', fontsize=11, ha='center',
            bbox=dict(boxstyle='round', facecolor='white', edgecolor='black'))
    
    # Dilution fridge
    ax.add_patch(Rectangle((-1.5, -1), 6, 3.5, fill=False, edgecolor='blue', 
                           linestyle='--', linewidth=1.5))
    ax.text(1.5, 2.7, 'Dilution fridge (T = 20 mK)', ha='center', fontsize=9, color='blue')
    
    ax.set_xlim(-2, 5)
    ax.set_ylim(-1.5, 3)
    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_title('(c) RF Reflectometry Setup', fontsize=11, fontweight='bold')


def draw_fidelity_comparison(ax):
    """Draw fidelity comparison between linear and T-junction."""
    
    configs = ['Linear\n3-site', 'Linear\n6-site', 'T-shape\n6-site']
    parity_fidelity = [99.0, 98.5, 99.5]
    gate_fidelity = [95, 92, 98]
    
    x = np.arange(len(configs))
    width = 0.35
    
    bars1 = ax.bar(x - width/2, parity_fidelity, width, label='Parity fidelity (%)',
                   color='#2E86AB', edgecolor='black')
    bars2 = ax.bar(x + width/2, gate_fidelity, width, label='Gate fidelity (%)',
                   color='#E94F37', edgecolor='black')
    
    ax.set_ylabel('Fidelity (%)', fontsize=10)
    ax.set_xticks(x)
    ax.set_xticklabels(configs, fontsize=9)
    ax.set_ylim(85, 101)
    ax.legend(loc='lower right', fontsize=8)
    ax.grid(True, alpha=0.3, axis='y')
    
    # Annotate improvement
    ax.annotate('', xy=(2, 98), xytext=(1, 92),
               arrowprops=dict(arrowstyle='->', color='green', lw=2))
    ax.text(1.8, 94, '+6%', fontsize=10, color='green', fontweight='bold')
    
    # Target line
    ax.axhline(99, color='gray', linestyle='--', alpha=0.5)
    ax.text(2.5, 99.2, 'Target', fontsize=8, color='gray')
    
    ax.set_title('(d) Fidelity: Linear vs T-Junction', fontsize=11, fontweight='bold')


def main():
    """Generate Figure 8: Majorana T-junction schematic."""
    
    fig = plt.figure(figsize=(12, 10))
    gs = gridspec.GridSpec(2, 2, hspace=0.35, wspace=0.25)
    
    # (a) T-junction device
    ax_device = fig.add_subplot(gs[0, 0])
    draw_t_junction_device(ax_device)
    
    # (b) Braiding sequence
    ax_braid = fig.add_subplot(gs[0, 1])
    draw_braiding_sequence(ax_braid)
    
    # (c) Measurement scheme
    ax_measure = fig.add_subplot(gs[1, 0])
    draw_measurement_scheme(ax_measure)
    
    # (d) Fidelity comparison
    ax_fidelity = fig.add_subplot(gs[1, 1])
    draw_fidelity_comparison(ax_fidelity)
    
    plt.suptitle('Figure 8: Majorana T-Junction Device for Topological Quantum Computing',
                 fontsize=13, fontweight='bold', y=0.98)
    
    plt.savefig('Fig8_Majorana_T_Junction.png', dpi=300, bbox_inches='tight',
                facecolor='white', edgecolor='none')
    plt.savefig('Fig8_Majorana_T_Junction.pdf', bbox_inches='tight',
                facecolor='white', edgecolor='none')
    
    print("Figure 8 saved: Fig8_Majorana_T_Junction.png/pdf")
    
    plt.close()


if __name__ == '__main__':
    main()
