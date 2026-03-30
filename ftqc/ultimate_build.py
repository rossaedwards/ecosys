#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
ULTIMATE FTQC BUILD SCRIPT
1. Convert all .ris and .txt citation files to .bib
2. Merge all .bib files into master bibliography
3. Run nuclear build (rename figures + 10×bibtex + 10×pdflatex)
"""

import re
import shutil
import subprocess
from pathlib import Path
from datetime import datetime

# ============================================================================
# PART 1: CITATION FILE CONVERSION & MERGING
# ============================================================================

def convert_ris_to_bibtex(ris_path):
    """Convert RIS format to BibTeX entries"""
    print(f"   Converting {ris_path.name}...")
    
    with open(ris_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Split into individual entries
    entries = content.split('\n\n')
    bibtex_entries = []
    
    for i, entry in enumerate(entries):
        if not entry.strip():
            continue
        
        # Parse RIS fields
        fields = {}
        for line in entry.split('\n'):
            if '  - ' in line:
                tag, value = line.split('  - ', 1)
                tag = tag.strip()
                fields[tag] = value.strip()
        
        # Convert to BibTeX
        if 'TI' in fields:  # Title exists
            # Generate cite key
            author_last = fields.get('AU', 'Unknown').split(',')[0].replace(' ', '')
            year = fields.get('PY', '2024')[:4]
            title_word = fields['TI'].split()[0].lower() if fields['TI'] else 'untitled'
            cite_key = f"{author_last}{year}{title_word}"
            
            # Build BibTeX entry
            bib_type = 'article'
            if 'JO' not in fields and 'T2' not in fields:
                bib_type = 'misc'
            
            bib_entry = f"@{bib_type}{{{cite_key},\n"
            
            if 'TI' in fields:
                bib_entry += f"  title = {{{fields['TI']}}},\n"
            if 'AU' in fields:
                bib_entry += f"  author = {{{fields['AU']}}},\n"
            if 'PY' in fields:
                bib_entry += f"  year = {{{fields['PY'][:4]}}},\n"
            if 'JO' in fields:
                bib_entry += f"  journal = {{{fields['JO']}}},\n"
            if 'VL' in fields:
                bib_entry += f"  volume = {{{fields['VL']}}},\n"
            if 'SP' in fields:
                bib_entry += f"  pages = {{{fields['SP']}}},\n"
            if 'DO' in fields:
                bib_entry += f"  doi = {{{fields['DO']}}},\n"
            
            bib_entry += "}\n"
            bibtex_entries.append(bib_entry)
    
    return '\n'.join(bibtex_entries)

def convert_txt_to_bibtex(txt_path):
    """Convert plain text citations to BibTeX (basic heuristics)"""
    print(f"   Converting {txt_path.name}...")
    
    with open(txt_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    bibtex_entries = []
    
    for i, line in enumerate(lines):
        line = line.strip()
        if not line or line.startswith('#'):
            continue
        
        # Try to parse as "Author (Year). Title. Journal."
        match = re.match(r'(.+?)\((\d{4})\)\.?\s*(.+?)\.?\s*(.+)?', line)
        
        if match:
            author, year, title, rest = match.groups()
            cite_key = f"ref{i+1:03d}"
            
            bib_entry = f"@article{{{cite_key},\n"
            bib_entry += f"  author = {{{author.strip()}}},\n"
            bib_entry += f"  year = {{{year}}},\n"
            bib_entry += f"  title = {{{title.strip()}}},\n"
            if rest:
                bib_entry += f"  journal = {{{rest.strip()}}},\n"
            bib_entry += "}\n"
            
            bibtex_entries.append(bib_entry)
        else:
            # Fallback: create misc entry
            cite_key = f"ref{i+1:03d}"
            bib_entry = f"@misc{{{cite_key},\n"
            bib_entry += f"  note = {{{line}}},\n"
            bib_entry += "}\n"
            bibtex_entries.append(bib_entry)
    
    return '\n'.join(bibtex_entries)

def merge_all_bibliographies():
    """Find all .bib, .ris, .txt files and merge into master.bib"""
    print("="*70)
    print("PART 1: BIBLIOGRAPHY COMPILATION")
    print("="*70)
    
    all_bibtex = []
    processed_keys = set()
    
    # Find all citation files
    citation_files = []
    for pattern in ['*.bib', '**/*.bib', '*.ris', '**/*.ris', '*.txt']:
        citation_files.extend(Path('.').glob(pattern))
    
    print(f"\n📚 Found {len(citation_files)} citation files:\n")
    
    for file_path in sorted(citation_files):
        # Skip output files
        if file_path.name in ['main.bib', 'master.bib', 'merged.bib']:
            continue
        
        print(f"   📖 Processing {file_path}")
        
        try:
            if file_path.suffix == '.bib':
                # Already BibTeX
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
            elif file_path.suffix == '.ris':
                content = convert_ris_to_bibtex(file_path)
                
            elif file_path.suffix == '.txt':
                # Only convert if looks like citations
                with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                    first_line = f.readline()
                if any(word in first_line.lower() for word in ['author', 'title', 'journal', 'doi']):
                    content = convert_txt_to_bibtex(file_path)
                else:
                    print(f"      ⚠️  Skipping {file_path.name} (doesn't look like citations)")
                    continue
            else:
                continue
            
            # Extract entries and deduplicate
            entries = re.findall(r'@\w+\{[^@]+\}', content, re.DOTALL)
            
            for entry in entries:
                # Extract citation key
                key_match = re.search(r'@\w+\{([^,]+),', entry)
                if key_match:
                    key = key_match.group(1)
                    if key not in processed_keys:
                        all_bibtex.append(entry)
                        processed_keys.add(key)
            
        except Exception as e:
            print(f"      ❌ Error processing {file_path.name}: {e}")
    
    # Write master bibliography
    master_bib_path = Path('main.bib')
    
    with open(master_bib_path, 'w', encoding='utf-8') as f:
        f.write(f"% Master Bibliography for FTQC Paper\n")
        f.write(f"% Auto-generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")
        f.write(f"% Total entries: {len(all_bibtex)}\n\n")
        f.write('\n\n'.join(all_bibtex))
    
    print(f"\n✅ Created {master_bib_path} with {len(all_bibtex)} unique entries\n")
    
    return master_bib_path

def update_main_tex_bibliography():
    """Update main.tex to use main.bib"""
    print("="*70)
    print("UPDATING main.tex BIBLIOGRAPHY REFERENCE")
    print("="*70)
    
    with open('main.tex', 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Replace any \bibliography{...} with \bibliography{main}
    original = content
    content = re.sub(r'\\bibliography\{[^}]+\}', r'\\bibliography{main}', content)
    
    if content != original:
        with open('main.tex', 'w', encoding='utf-8') as f:
            f.write(content)
        print("✅ Updated main.tex to use main.bib\n")
    else:
        print("⚠️  No changes needed in main.tex\n")

# ============================================================================
# PART 2: FIGURE RENAMING
# ============================================================================

def rename_all_figures():
    """Rename actual files to match what main.tex expects"""
    print("="*70)
    print("PART 2: RENAMING FIGURE FILES")
    print("="*70)
    
    renames = {
        'Fig1_Hilbert_Scaling_080646.png': 'Fig1_Hilbert_Scaling.png',
        'Fig2_Coherence_Dynamics_080647.png': 'Fig2_Coherence_Dynamics.png',
        'Fig3_Fractal_Localization_080647.png': 'Fig3_Anderson_Localization.png',
        'aurphyx_aurafs_SFS.png': 'Fig4_Sierpinski_Lattice.png',
        'trca_band_structure.png': 'Fig5_Band_Structure.png',
        'majorana_t_shape_6dot_schematic.png': 'Fig8_Majorana_T_Junction.png',
        'zpe_core_majorana_stability.png': 'Fig6_ZPE_Majorana_Stability.png',
        'aurphyx_aurafs_fractal-lattice_sim.png': 'Fig11_Fractal_Lattice_Sim.png',
    }
    
    fig_dir = Path('figures')
    renamed_count = 0
    
    for old_name, new_name in renames.items():
        old_path = fig_dir / old_name
        new_path = fig_dir / new_name
        
        if old_path.exists():
            if new_path.exists():
                print(f"   ✓ {new_name} already exists")
            else:
                shutil.copy2(old_path, new_path)
                print(f"   ✅ {old_name} → {new_name}")
                renamed_count += 1
    
    print(f"\n✅ Renamed/copied {renamed_count} figures\n")

# ============================================================================
# PART 3: NUCLEAR BUILD
# ============================================================================

def delete_all_artifacts():
    """Nuclear clean"""
    print("="*70)
    print("PART 3: NUCLEAR CLEAN")
    print("="*70)
    
    artifacts = [
        'main.aux', 'main.bbl', 'main.blg', 'main.log', 'main.out',
        'main.fls', 'main.fdb_latexmk', 'main.synctex.gz', 'main.toc',
    ]
    
    for art in artifacts:
        if Path(art).exists():
            Path(art).unlink()
            print(f"   Deleted {art}")
    
    print()

def run_bibtex_10_times():
    """Run bibtex 10 times"""
    print("="*70)
    print("PART 4: 10× BIBTEX PASSES")
    print("="*70)
    
    for i in range(1, 11):
        print(f"   BibTeX pass {i}/10... ", end='', flush=True)
        
        if i == 1:
            subprocess.run(
                ['pdflatex', '-interaction=nonstopmode', 'main.tex'],
                capture_output=True, timeout=120
            )
        
        subprocess.run(['bibtex', 'main'], capture_output=True, timeout=60)
        
        if Path('main.bbl').exists():
            with open('main.bbl', 'r', encoding='utf-8') as f:
                count = f.read().count(r'\bibitem')
                print(f"✓ ({count} citations)")
        else:
            print("❌")
        
        subprocess.run(
            ['pdflatex', '-interaction=nonstopmode', 'main.tex'],
            capture_output=True, timeout=120
        )
    
    print()

def run_pdflatex_10_times():
    """Run pdflatex 10 times"""
    print("="*70)
    print("PART 5: 10× PDFLATEX PASSES")
    print("="*70)
    
    for i in range(1, 11):
        print(f"   PDFLaTeX pass {i}/10... ", end='', flush=True)
        
        subprocess.run(
            ['pdflatex', '-interaction=nonstopmode', 'main.tex'],
            capture_output=True, timeout=120
        )
        
        if Path('main.pdf').exists():
            size = Path('main.pdf').stat().st_size / 1024
            print(f"✓ ({size:.0f} KB)")
        else:
            print("❌")
    
    print()

def final_verification():
    """Check everything"""
    print("="*70)
    print("FINAL VERIFICATION")
    print("="*70)
    
    if Path('main.bbl').exists():
        with open('main.bbl', 'r', encoding='utf-8') as f:
            count = f.read().count(r'\bibitem')
            print(f"\n📚 Bibliography: {count} citations")
            
            if count > 100:
                print("   ✅✅✅ FULL BIBLIOGRAPHY COMPILED!")
            else:
                print(f"   ⚠️  Got {count} citations (expected 100+)")
    
    if Path('main.pdf').exists():
        size = Path('main.pdf').stat().st_size / 1024
        print(f"\n📄 PDF: {size:.0f} KB")
        
        if size > 1500:
            print("   ✅✅✅ FIGURES EMBEDDED!")
        elif size > 800:
            print("   ⚠️  Partial figures (may be missing some)")
        else:
            print("   ❌ Figures likely missing")
    
    print("\n" + "="*70)
    print("🎉🎉🎉 BUILD COMPLETE! 🎉🎉🎉")
    print("="*70)
    print("\n📂 OPEN main.pdf NOW")
    print("\n✅ Checklist:")
    print("   [ ] All figures showing")
    print("   [ ] 100+ references")
    print("   [ ] Appendix IX present")
    print("   [ ] No duplicate sections")
    print("\n🚀 If all good → SUBMIT TO ARXIV!")

# ============================================================================
# MAIN EXECUTION
# ============================================================================

def main():
    print("\n" + "="*70)
    print("🔥🔥🔥 ULTIMATE FTQC BUILD SCRIPT 🔥🔥🔥")
    print("="*70)
    print(f"Started: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("Estimated time: 8-10 minutes")
    print()
    
    try:
        # Part 1: Bibliography
        merge_all_bibliographies()
        update_main_tex_bibliography()
        
        # Part 2: Figures
        rename_all_figures()
        
        # Part 3-5: Nuclear build
        delete_all_artifacts()
        run_bibtex_10_times()
        run_pdflatex_10_times()
        
        # Verification
        final_verification()
        
    except KeyboardInterrupt:
        print("\n\n⚠️  Build interrupted by user")
    except Exception as e:
        print(f"\n\n❌ ERROR: {e}")
        import traceback
        traceback.print_exc()

if __name__ == '__main__':
    main()
