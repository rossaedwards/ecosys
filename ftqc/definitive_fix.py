#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
DEFINITIVE FIX: Based on actual directory listing
Fixes both figures AND citations (only 7 showing)
"""

import re
import subprocess
from pathlib import Path

def fix_all_figures():
    """Fix every figure reference based on actual files"""
    print("="*70)
    print("FIXING ALL FIGURE REFERENCES")
    print("="*70)
    
    # EXACT files that exist (from your dir listing)
    actual_figures = {
        'aurphyx_aurafs_fractal-lattice_sim.png',
        'aurphyx_aurafs_SFS.png',
        'Fig1_Hilbert_Scaling_080646.png',
        'Fig1_Hilbert_Scaling.pdf',
        'Fig1_Hilbert_Scaling.png',
        'Fig10_Technology_Roadmap.pdf',
        'Fig10_Technology_Roadmap.png',
        'Fig2_Coherence_Dynamics_080647.png',
        'Fig2_Coherence_Dynamics.pdf',
        'Fig2_Coherence_Dynamics.png',
        'Fig3_Anderson_Localization.pdf',
        'Fig3_Anderson_Localization.png',
        'Fig3_Fractal_Localization_080647.png',
        'Fig4_Sierpinski_Lattice.pdf',
        'Fig4_Sierpinski_Lattice.png',
        'Fig5_Band_Structure.pdf',
        'Fig5_Band_Structure.png',
        'Fig6_Neglecton_Braiding.pdf',
        'Fig6_Neglecton_Braiding.png',
        'Fig7_Device_Cross_Section.pdf',
        'Fig7_Device_Cross_Section.png',
        'Fig8_Majorana_T_Junction.pdf',
        'Fig8_Majorana_T_Junction.png',
        'Fig9_Information_Scaling.pdf',
        'Fig9_Information_Scaling.png',
        'majorana_t_shape_6dot_schematic.png',
        'trca_band_structure.png',
        'zpe_core_majorana_stability.png',
    }
    
    with open('main.tex', 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Replace ALL \includegraphics with correct filenames
    # Find all current references
    pattern = r'\\includegraphics(?:\[[^\]]*\])?\{figures/([^}]+)\}'
    matches = re.finditer(pattern, content)
    
    replacements = {}
    
    for match in matches:
        current_ref = match.group(1)
        current_full = match.group(0)
        
        # Try to find matching file
        # Remove extension if present
        base_name = current_ref.replace('.jpg', '').replace('.png', '').replace('.pdf', '')
        
        # Look for best match (prefer .png, then .pdf)
        best_match = None
        for ext in ['.png', '.pdf']:
            if f"{base_name}{ext}" in actual_figures:
                best_match = f"{base_name}{ext}"
                break
        
        if best_match and best_match != current_ref:
            replacements[current_full] = current_full.replace(current_ref, best_match)
            print(f"   {current_ref} → {best_match}")
    
    # Apply replacements
    for old, new in replacements.items():
        content = content.replace(old, new)
    
    with open('main.tex', 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"\n✅ Fixed {len(replacements)} figure references\n")

def fix_bibliography_crisis():
    """Fix the 7-citation problem (likely broken .bib file or wrong path)"""
    print("="*70)
    print("FIXING BIBLIOGRAPHY (7 citations → 138)")
    print("="*70)
    
    # Check if arxiv_ftqc_complete.bib exists
    bib_files = list(Path('.').glob('*.bib'))
    print(f"\n📚 Found {len(bib_files)} .bib files:")
    for bib in bib_files:
        with open(bib, 'r', encoding='utf-8') as f:
            entries = f.read().count('@')
        print(f"   {bib.name}: {entries} entries")
    
    # Check what main.tex references
    with open('main.tex', 'r', encoding='utf-8') as f:
        content = f.read()
    
    bib_match = re.search(r'\\bibliography\{([^}]+)\}', content)
    if bib_match:
        bib_ref = bib_match.group(1)
        print(f"\n📖 main.tex references: {bib_ref}")
        
        # Check if file exists
        if not Path(f"{bib_ref}.bib").exists():
            print(f"   ❌ File {bib_ref}.bib NOT FOUND!")
            
            # Find largest .bib file
            largest_bib = max(bib_files, key=lambda x: x.stat().st_size)
            print(f"   💡 Largest .bib file: {largest_bib.name}")
            print(f"   💡 Suggest changing to: \\bibliography{{{largest_bib.stem}}}")
            
            # Auto-fix
            content = content.replace(
                f"\\bibliography{{{bib_ref}}}",
                f"\\bibliography{{{largest_bib.stem}}}"
            )
            with open('main.tex', 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"   ✅ Changed bibliography reference to {largest_bib.stem}")
    
    print()

def nuclear_rebuild():
    """Delete everything and rebuild from scratch"""
    print("="*70)
    print("NUCLEAR REBUILD")
    print("="*70)
    
    # Delete artifacts
    artifacts = ['main.aux', 'main.bbl', 'main.blg', 'main.log', 'main.out']
    for art in artifacts:
        if Path(art).exists():
            Path(art).unlink()
            print(f"   Deleted {art}")
    
    print("\n🔨 Running 6-pass build...\n")
    
    for i in range(1, 7):
        print(f"   Pass {i}/6... ", end='', flush=True)
        result = subprocess.run(
            ['pdflatex', '-interaction=nonstopmode', 'main.tex'],
            capture_output=True, timeout=120
        )
        
        if i in [2, 4]:
            print("+ bibtex ", end='', flush=True)
            subprocess.run(['bibtex', 'main'], capture_output=True, timeout=60)
        
        if Path('main.pdf').exists():
            size = Path('main.pdf').stat().st_size / 1024
            print(f"✓ ({size:.0f} KB)")
        else:
            print("❌")
    
    print()

def final_check():
    """Check citations and figures in output"""
    print("="*70)
    print("FINAL VERIFICATION")
    print("="*70)
    
    # Check .bbl
    if Path('main.bbl').exists():
        with open('main.bbl', 'r', encoding='utf-8') as f:
            bbl = f.read()
            count = bbl.count(r'\bibitem')
            print(f"\n📚 Citations in .bbl: {count}")
            if count < 50:
                print("   ❌ Still too few! Check .blg for errors:")
                if Path('main.blg').exists():
                    with open('main.blg', 'r', encoding='utf-8') as blg:
                        errors = [line for line in blg if 'error' in line.lower()]
                        for err in errors[:5]:
                            print(f"      {err.strip()}")
            else:
                print("   ✅ Bibliography looks good!")
    
    # Check PDF
    if Path('main.pdf').exists():
        size = Path('main.pdf').stat().st_size / 1024
        print(f"\n📄 PDF: {size:.0f} KB")
        if size > 1000:
            print("   ✅ PDF size looks healthy (includes figures)")
        else:
            print("   ⚠️  PDF seems small - may be missing figures")
    
    print("\n" + "="*70)
    print("🎯 DONE - CHECK main.pdf NOW")
    print("="*70)

def main():
    fix_all_figures()
    fix_bibliography_crisis()
    nuclear_rebuild()
    final_check()

if __name__ == '__main__':
    main()
