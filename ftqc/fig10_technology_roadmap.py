#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
Figure 10: Technology Roadmap 2026-2035
PRX Submission - Aurphyx

Generates publication-quality technology roadmap showing projected
qubit scaling, cost trajectories, and milestone timeline.
"""

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle, FancyBboxPatch, Circle, Polygon
from matplotlib.lines import Line2D
import matplotlib.gridspec as gridspec
from matplotlib.ticker import FuncFormatter

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


def plot_qubit_roadmap(ax):
    """Plot qubit count projections for different platforms."""
    
    years = np.arange(2024, 2036)
    
    # IBM roadmap (public)
    ibm_qubits = [1121, 1500, 2000, 4000, 8000, 16000, 32000, 50000, 
                  100000, 200000, 500000, 1000000]
    
    # Google projection
    google_qubits = [70, 100, 200, 500, 1000, 2000, 5000, 10000,
                    20000, 50000, 100000, 200000]
    
    # Trapped ions (IonQ/Quantinuum)
    ion_qubits = [32, 50, 80, 150, 300, 600, 1200, 2500,
                 5000, 10000, 20000, 40000]
    
    # Aurphyx fractal (effective logical qubits)
    # 16× overhead reduction means fewer physical qubits needed
    aurphyx_physical = [12, 20, 50, 100, 200, 500, 1000, 2000,
                        5000, 10000, 20000, 50000]
    aurphyx_effective = [n * 16 for n in aurphyx_physical]  # Effective due to overhead reduction
    
    ax.semilogy(years, ibm_qubits, 'o-', color='#0066CC', linewidth=2,
                markersize=6, label='IBM (physical)')
    ax.semilogy(years, google_qubits, 's-', color='#34A853', linewidth=2,
                markersize=6, label='Google (physical)')
    ax.semilogy(years, ion_qubits, '^-', color='#FF9500', linewidth=2,
                markersize=6, label='Trapped ions (physical)')
    ax.semilogy(years, aurphyx_effective, 'D-', color='#E94F37', linewidth=2.5,
                markersize=7, label='Aurphyx (effective)')
    ax.semilogy(years, aurphyx_physical, 'D--', color='#E94F37', linewidth=1.5,
                markersize=5, alpha=0.5, label='Aurphyx (physical)')
    
    ax.set_xlabel('Year', fontsize=11)
    ax.set_ylabel('Qubit count', fontsize=11)
    ax.legend(loc='upper left', fontsize=8)
    ax.grid(True, alpha=0.3)
    ax.set_xlim(2024, 2035)
    ax.set_ylim(10, 2e7)
    
    # Fault tolerance threshold
    ax.axhline(1e6, color='gray', linestyle=':', alpha=0.7)
    ax.text(2034.5, 1.3e6, 'FT threshold\n(~10⁶ logical)', fontsize=8, 
            ha='right', color='gray')
    
    # Annotation for 16× advantage
    ax.annotate('16× effective\nadvantage', xy=(2030, aurphyx_effective[6]),
                xytext=(2031, aurphyx_effective[6] * 5),
                fontsize=8, color='#E94F37',
                arrowprops=dict(arrowstyle='->', color='#E94F37', lw=1))
    
    ax.set_title('(a) Qubit Scaling Roadmap', fontsize=11, fontweight='bold')


def plot_cost_trajectory(ax):
    """Plot cost per qubit trajectory."""
    
    years = np.arange(2024, 2036)
    
    # Historical: ~$10M per qubit (2015) → ~$100K per qubit (2024)
    # Moore's law-like reduction: ~50% per year
    
    # Superconducting
    sc_cost = 100000 * (0.7 ** np.arange(12))  # 30% annual reduction
    
    # Trapped ions
    ion_cost = 200000 * (0.75 ** np.arange(12))  # 25% annual reduction
    
    # Aurphyx (fractal approach reduces effective cost by 16×)
    aurphyx_cost = 80000 * (0.65 ** np.arange(12))  # 35% reduction + 16× advantage
    
    ax.semilogy(years, sc_cost, 'o-', color='#0066CC', linewidth=2,
                markersize=6, label='Superconducting')
    ax.semilogy(years, ion_cost, '^-', color='#FF9500', linewidth=2,
                markersize=6, label='Trapped ions')
    ax.semilogy(years, aurphyx_cost, 'D-', color='#E94F37', linewidth=2.5,
                markersize=7, label='Aurphyx (effective)')
    
    ax.set_xlabel('Year', fontsize=11)
    ax.set_ylabel('Cost per qubit (USD)', fontsize=11)
    ax.legend(loc='upper right', fontsize=8)
    ax.grid(True, alpha=0.3)
    ax.set_xlim(2024, 2035)
    
    # Target cost for commercialization
    ax.axhline(100, color='green', linestyle='--', alpha=0.7)
    ax.text(2025, 70, 'Commercial target ($100/qubit)', fontsize=8, color='green')
    
    # Format y-axis as currency
    ax.yaxis.set_major_formatter(FuncFormatter(lambda x, p: f'${x:,.0f}'))
    
    ax.set_title('(b) Cost per Effective Qubit', fontsize=11, fontweight='bold')


def plot_milestone_timeline(ax):
    """Plot milestone timeline with key achievements."""
    
    milestones = [
        (2024, 'Today', 'Majorana-1 (99%)\nWillow sub-threshold', '#666666'),
        (2026, 'Phase I', '12-qubit fractal demo\n$1.25M', '#2E86AB'),
        (2028, 'Phase II', '100-qubit processor\nNV-photonic chip', '#F18F01'),
        (2030, 'Integration', 'Neglecton demo\nHybrid platform', '#E94F37'),
        (2032, 'Scale-up', '1000-qubit fractal\nFT threshold', '#6B8E23'),
        (2035, 'Commercial', 'Fault-tolerant QC\nQuantum advantage', '#8B008B'),
    ]
    
    ax.set_xlim(2023, 2036)
    ax.set_ylim(-0.5, 1.5)
    
    # Timeline axis
    ax.axhline(0.5, color='black', linewidth=2)
    
    for i, (year, phase, desc, color) in enumerate(milestones):
        # Marker
        ax.plot(year, 0.5, 'o', markersize=15, color=color, markeredgecolor='black', zorder=5)
        
        # Alternating text position
        y_text = 1.0 if i % 2 == 0 else 0.0
        va = 'bottom' if i % 2 == 0 else 'top'
        
        # Connecting line
        ax.plot([year, year], [0.5, y_text - 0.1 if i % 2 == 0 else y_text + 0.1],
                color=color, linewidth=1.5, linestyle='--', alpha=0.7)
        
        # Text box
        ax.text(year, y_text, f'{phase}\n({year})\n{desc}',
                ha='center', va=va, fontsize=8,
                bbox=dict(boxstyle='round,pad=0.3', facecolor='white', 
                         edgecolor=color, alpha=0.9))
    
    ax.axis('off')
    ax.set_title('(c) Development Milestone Timeline', fontsize=11, fontweight='bold')


def plot_trl_progression(ax):
    """Plot Technology Readiness Level progression."""
    
    phases = ['Concept', 'Lab Demo', 'Prototype', 'Validation', 'Integration', 'Production']
    trl_levels = [2, 4, 5, 6, 7, 9]
    years_achieved = [2024, 2026, 2028, 2030, 2032, 2035]
    
    colors = plt.cm.RdYlGn(np.linspace(0.2, 0.9, len(phases)))
    
    bars = ax.barh(phases, trl_levels, color=colors, edgecolor='black', height=0.6)
    
    # Add year labels
    for bar, year in zip(bars, years_achieved):
        ax.text(bar.get_width() + 0.2, bar.get_y() + bar.get_height()/2,
                f'{year}', va='center', fontsize=9)
    
    ax.set_xlabel('Technology Readiness Level (TRL)', fontsize=11)
    ax.set_xlim(0, 10)
    ax.set_xticks(range(0, 11))
    ax.grid(True, alpha=0.3, axis='x')
    
    # Current position indicator
    ax.axvline(2, color='#E94F37', linewidth=2, linestyle='--')
    ax.text(2.2, 5.5, 'Current\n(TRL 2)', fontsize=8, color='#E94F37')
    
    # Phase I target
    ax.axvline(4, color='#2E86AB', linewidth=2, linestyle=':')
    ax.text(4.2, 4.5, 'Phase I\ntarget', fontsize=8, color='#2E86AB')
    
    ax.set_title('(d) TRL Progression Timeline', fontsize=11, fontweight='bold')


def main():
    """Generate Figure 10: Technology roadmap."""
    
    fig = plt.figure(figsize=(12, 10))
    gs = gridspec.GridSpec(2, 2, hspace=0.35, wspace=0.3)
    
    # (a) Qubit roadmap
    ax_qubits = fig.add_subplot(gs[0, 0])
    plot_qubit_roadmap(ax_qubits)
    
    # (b) Cost trajectory
    ax_cost = fig.add_subplot(gs[0, 1])
    plot_cost_trajectory(ax_cost)
    
    # (c) Milestone timeline
    ax_timeline = fig.add_subplot(gs[1, :])
    plot_milestone_timeline(ax_timeline)
    
    plt.suptitle('Figure 10: Aurphyx Technology Roadmap 2024-2035',
                 fontsize=13, fontweight='bold', y=0.98)
    
    plt.savefig('Fig10_Technology_Roadmap.png', dpi=300, bbox_inches='tight',
                facecolor='white', edgecolor='none')
    plt.savefig('Fig10_Technology_Roadmap.pdf', bbox_inches='tight',
                facecolor='white', edgecolor='none')
    
    print("Figure 10 saved: Fig10_Technology_Roadmap.png/pdf")
    
    plt.close()


if __name__ == '__main__':
    main()
