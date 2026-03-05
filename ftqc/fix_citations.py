#!/usr/bin/env python3
"""
Fix Citations and Figures - Complete Rebuild
"""
from pathlib import Path
import subprocess
import shutil

def fix_and_rebuild():
    print("="*70)
    print("FIXING CITATIONS AND FIGURES")
    print("="*70)
    
    # Step 1: Clean ALL build artifacts
    print("\n🧹 Cleaning build artifacts...")
    artifacts = list(Path('.').glob('main.*'))
    keep = ['main.tex', 'main.pdf']
    for f in artifacts:
        if f.name not in keep:
            try:
                f.unlink()
                print(f"  Deleted {f.name}")
            except:
                pass
    
    # Step 2: Verify .bib file has content
    print("\n📚 Checking bibliography...")
    bib_files = ['arxiv_ftqc.bib', 'master_citations.bib']
    active_bib = None
    
    for bib in bib_files:
        if Path(bib).exists():
            with open(bib, 'r', encoding='utf-8') as f:
                content = f.read()
                entry_count = content.count('@article') + content.count('@book') + content.count('@misc')
                if entry_count > 50:
                    active_bib = bib
                    print(f"✅ {bib}: {entry_count} entries")
                    break
    
    if not active_bib:
        print("❌ No bibliography with >50 entries found!")
        return False
    
    # Step 3: Update main.tex to use correct .bib
    print(f"\n📝 Updating main.tex to use {active_bib}...")
    with open('main.tex', 'r', encoding='utf-8') as f:
        content = f.read()
    
    bib_stem = Path(active_bib).stem
    content = content.replace(r'\bibliography{arxiv_ftqc}', f'\\bibliography{{{bib_stem}}}')
    content = content.replace(r'\bibliography{master_citations}', f'\\bibliography{{{bib_stem}}}')
    
    with open('main.tex', 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"  Set bibliography to: {bib_stem}")
    
    # Step 4: Full rebuild sequence
    print("\n🏗️  Running full LaTeX build (6 passes)...")
    
    for pass_num in range(1, 7):
        print(f"\n  Pass {pass_num}/6...")
        
        # pdflatex
        subprocess.run(
            ['pdflatex', '-interaction=nonstopmode', 'main.tex'],
            capture_output=True, timeout=120
        )
        
        # bibtex on passes 2 and 4
        if pass_num in [2, 4]:
            result = subprocess.run(
                ['bibtex', 'main'],
                capture_output=True, timeout=60, text=True
            )
            print(f"    + bibtex: {result.returncode}")
            if result.returncode != 0:
                print(f"    bibtex stderr: {result.stderr[:200]}")
        
        # Check PDF size
        if Path('main.pdf').exists():
            size = Path('main.pdf').stat().st_size / 1024
            print(f"    PDF: {size:.1f} KB")
    
    # Step 5: Check results
    print("\n" + "="*70)
    print("BUILD COMPLETE - CHECKING RESULTS")
    print("="*70)
    
    # Check .bbl file
    if Path('main.bbl').exists():
        with open('main.bbl', 'r', encoding='utf-8') as f:
            bbl_content = f.read()
            bib_items = bbl_content.count(r'\bibitem')
            print(f"\n✅ Bibliography compiled: {bib_items} references in main.bbl")
    else:
        print("\n❌ main.bbl not generated!")
    
    # Check figures
    fig_dir = Path('figures')
    if fig_dir.exists():
        figs = list(fig_dir.glob('*.png')) + list(fig_dir.glob('*.pdf'))
        print(f"✅ Figures directory: {len(figs)} files")
    else:
        print("❌ figures/ directory missing!")
    
    # Check PDF
    if Path('main.pdf').exists():
        size = Path('main.pdf').stat().st_size / 1024
        print(f"✅ Final PDF: {size:.1f} KB")
        print("\n🎉 Open main.pdf and verify:")
        print("   - References section has 100+ entries")
        print("   - All figures render")
        print("   - Citations show [1], [2], etc. not ??")
    else:
        print("❌ PDF not generated")
    
    return True

if __name__ == '__main__':
    fix_and_rebuild()
