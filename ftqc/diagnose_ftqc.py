#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
FTQC Diagnostic - Find the real LaTeX error
"""
from pathlib import Path
import subprocess

def diagnose_latex_error():
    """Run single pdflatex pass and show actual errors"""
    
    print("="*70)
    print("FTQC LATEX DIAGNOSTIC")
    print("="*70)
    
    # Check if main.tex exists
    if not Path('main.tex').exists():
        print("\n❌ main.tex not found!")
        print("Run ftqc_divine_pipeline.py first to create it.")
        return
    
    print("\n✅ Found main.tex")
    
    # Check for bibliography file
    bib_files = list(Path('.').glob('*.bib'))
    if bib_files:
        print(f"✅ Found bibliography: {[str(f) for f in bib_files]}")
    else:
        print("⚠️  No .bib files found")
    
    # Check if bibliography commands are in main.tex
    with open('main.tex', 'r', encoding='utf-8') as f:
        content = f.read()
        has_bibstyle = r'\bibliographystyle' in content
        has_bib = r'\bibliography{' in content
        
        if has_bibstyle and has_bib:
            print("✅ Bibliography commands present in main.tex")
        else:
            print("❌ Bibliography commands MISSING in main.tex")
            print("   Add before \\end{document}:")
            print("   \\bibliographystyle{apsrev4-2}")
            print("   \\bibliography{master_citations}")
    
    # Run single pdflatex pass
    print("\n" + "="*70)
    print("Running pdflatex to capture errors...")
    print("="*70 + "\n")
    
    result = subprocess.run(
        ['pdflatex', '-interaction=nonstopmode', 'main.tex'],
        capture_output=True,
        text=True,
        timeout=120
    )
    
    # Parse log for errors
    log_file = Path('main.log')
    if log_file.exists():
        with open(log_file, 'r', encoding='utf-8', errors='ignore') as f:
            log_lines = f.readlines()
        
        print("\n🔍 SEARCHING FOR ERRORS IN LOG...\n")
        
        errors_found = []
        for i, line in enumerate(log_lines):
            if line.startswith('!') or 'Error' in line or 'error' in line:
                # Capture error and context (5 lines after)
                error_block = ''.join(log_lines[i:min(i+6, len(log_lines))])
                errors_found.append(error_block)
        
        if errors_found:
            print(f"❌ FOUND {len(errors_found)} ERROR(S):\n")
            for idx, error in enumerate(errors_found, 1):
                print(f"--- Error {idx} ---")
                print(error)
                print()
        else:
            print("✅ No errors found in log!")
            print("   But compilation didn't complete. Checking warnings...")
            
            # Check last 50 lines for issues
            print("\n📋 LAST 50 LINES OF LOG:")
            print("-" * 70)
            for line in log_lines[-50:]:
                print(line.rstrip())
    else:
        print("❌ main.log not found - pdflatex didn't run")
    
    # Check if PDF was created
    if Path('main.pdf').exists():
        size = Path('main.pdf').stat().st_size / 1024
        print(f"\n✅ main.pdf exists ({size:.1f} KB)")
    else:
        print("\n❌ main.pdf was NOT created")
    
    print("\n" + "="*70)
    print("DIAGNOSIS COMPLETE")
    print("="*70)
    print("\n💡 NEXT STEPS:")
    print("   1. Fix any errors shown above in main.tex")
    print("   2. Ensure bibliography commands are present")
    print("   3. Rerun: python ftqc_divine_pipeline.py")

if __name__ == '__main__':
    try:
        diagnose_latex_error()
    except Exception as e:
        print(f"\n❌ Diagnostic failed: {e}")
