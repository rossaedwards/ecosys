#!/usr/bin/env python3
"""
PRECISE FIGURE FILENAME FIX
Based on actual files you have vs what main.tex expects
"""

import re
from pathlib import Path

def fix_exact_figures():
    print("="*70)
    print("PRECISE FIGURE FILENAME CORRECTIONS")
    print("="*70)
    
    # EXACT mapping based on your report
    figure_corrections = {
        # What main.tex has → What actually exists
        'figures/Fig2_Sierpinski_Lattice': 'figures/aurphyx_aurafs_SFS.jpg',  # Or Fig7_AuraFS_Sierpinski.png
        'figures/Fig3_Fractal_Localization': 'figures/Fig3_Fractal_Localization_080647.jpg',
        'figures/Fig4_TRCA_Band_Structure': 'figures/trca_band_structure.jpg',
        'figures/Fig5_Majorana_T_Shape': 'figures/majorana_t_shape_6dot_schematic.jpg',
        'figures/Fig6_ZPE_Majorana_Stability': 'figures/zpe_core_majorana_stability.jpg',
        'figures/Fig7_AuraFS_Sierpinski': 'figures/aurphyx_aurafs_SFS.jpg',
        
        # If paper references these without extension, add all variants
        'figures/Fig1_Hilbert_Scaling': 'figures/Fig1_Hilbert_Scaling_080646.jpg',
        'figures/Fig2_Coherence_Dynamics': 'figures/Fig2_Coherence_Dynamics_080647.jpg',
    }
    
    with open('main.tex', 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Fix each reference
    for wrong, correct in figure_corrections.items():
        # Pattern 1: With .png/.pdf extension
        for ext in ['.png', '.pdf', '.jpg']:
            pattern1 = rf'(\\includegraphics(?:\[[^\]]*\])?){{{re.escape(wrong + ext)}}}'
            if re.search(pattern1, content):
                content = re.sub(pattern1, rf'\1{{{correct}}}', content)
                print(f"   Fixed: {wrong}{ext} → {correct}")
        
        # Pattern 2: Without extension (LaTeX auto-detects)
        pattern2 = rf'(\\includegraphics(?:\[[^\]]*\])?){{{re.escape(wrong)}}}'
        if re.search(pattern2, content):
            content = re.sub(pattern2, rf'\1{{{correct}}}', content)
            print(f"   Fixed: {wrong} → {correct}")
    
    if content != original_content:
        with open('main.tex', 'w', encoding='utf-8') as f:
            f.write(content)
        print("\n✅ Figure references corrected!")
    else:
        print("\n⚠️  No changes made - references may already be correct")
    
    print()

def list_actual_figures():
    """Show what figures actually exist"""
    print("="*70)
    print("ACTUAL FIGURES IN figures/ DIRECTORY")
    print("="*70)
    
    fig_dir = Path('figures')
    if not fig_dir.exists():
        print("❌ figures/ directory not found!")
        return
    
    figures = sorted(fig_dir.glob('*'))
    for i, fig in enumerate(figures, 1):
        print(f"   {i}. {fig.name}")
    
    print()

def quick_rebuild():
    """Quick 3-pass rebuild to test fixes"""
    import subprocess
    
    print("="*70)
    print("QUICK 3-PASS REBUILD")
    print("="*70)
    
    for i in range(1, 4):
        print(f"   Pass {i}/3... ", end='', flush=True)
        subprocess.run(
            ['pdflatex', '-interaction=nonstopmode', 'main.tex'],
            capture_output=True, timeout=120
        )
        if i == 2:
            subprocess.run(['bibtex', 'main'], capture_output=True, timeout=60)
        
        if Path('main.pdf').exists():
            size = Path('main.pdf').stat().st_size / 1024
            print(f"PDF: {size:.1f} KB")
        else:
            print("❌")
    
    print()

def check_missing_figures():
    """Parse main.tex and check which figures are missing"""
    print("="*70)
    print("MISSING FIGURE CHECK")
    print("="*70)
    
    with open('main.tex', 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Find all \includegraphics commands
    pattern = r'\\includegraphics(?:\[[^\]]*\])?\{([^}]+)\}'
    refs = re.findall(pattern, content)
    
    missing = []
    found = []
    
    for ref in refs:
        # Try multiple extensions
        checked = False
        for ext in ['', '.png', '.pdf', '.jpg']:
            full_path = Path(ref + ext)
            if full_path.exists():
                found.append(ref)
                checked = True
                break
        
        if not checked:
            missing.append(ref)
    
    if missing:
        print(f"\n⚠️  {len(missing)} missing figure references:")
        for fig in missing:
            print(f"   - {fig}")
            # Try to suggest alternatives
            fig_name = Path(fig).name
            fig_dir = Path('figures')
            if fig_dir.exists():
                similar = [f.name for f in fig_dir.glob('*') if fig_name.lower() in f.name.lower()]
                if similar:
                    print(f"     Maybe use: {similar[0]}")
    else:
        print("✅ All figures found!")
    
    print(f"\n📊 Summary: {len(found)} found, {len(missing)} missing")
    print()

def main():
    list_actual_figures()
    fix_exact_figures()
    check_missing_figures()
    quick_rebuild()
    
    print("="*70)
    print("✅ FIX COMPLETE")
    print("="*70)
    print("\n💡 Next:")
    print("   1. Open main.pdf")
    print("   2. Check if figures now show")
    print("   3. If still issues, run: ls figures/ and paste output")

if __name__ == '__main__':
    main()
