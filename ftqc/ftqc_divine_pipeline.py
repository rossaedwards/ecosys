#!/usr/bin/env python3
"""
FTQC Divine Pipeline - Complete Autonomous Publication System
Ross A. Edwards | Aurphyx LLC
Handles: figure management, LaTeX fixes, bibliography, 10-pass builds, arXiv packaging, .gitignore
Version: 2.0 (Complete)
"""
import re
import subprocess
import sys
import shutil
import tarfile
import os
from pathlib import Path
from datetime import datetime

class FTQCDivinePipeline:
    def __init__(self):
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        # Figure mapping: your files → standardized names
        self.figure_map = {
            'Fig1_Hilbert_Scaling_080646.jpg': 'Fig1_Hilbert_Scaling.png',
            'Fig2_Coherence_Dynamics_080647.jpg': 'Fig2_Coherence_Dynamics.png',
            'Fig3_Fractal_Localization_080647.jpg': 'Fig3_Fractal_Localization.png',
            'trca_band_structure.jpg': 'Fig4_TRCA_Band_Structure.png',
            'majorana_t_shape_6dot_schematic.jpg': 'Fig5_Majorana_T_Shape.png',
            'zpe_core_majorana_stability.jpg': 'Fig6_ZPE_Majorana_Stability.png',
            'aurphyx_aurafs_SFS.jpg': 'Fig7_AuraFS_Sierpinski.png',
            'aurphyx_aurafs_fractal-lattice_sim.jpg': 'Fig8_Fractal_Lattice_Sim.png'
        }
        
        # Section → Figure mapping
        self.section_figures = {
            'section_ii_fractal_hilbert_scaling.tex': [
                ('Fig1_Hilbert_Scaling.png', r'Hilbert space scaling comparison between Sierpiński topology (solid) and Euclidean lattice (dashed), demonstrating $7.1\times$ advantage at $n=5$ qubits, extrapolating to $10^4$ at $n=12$ with full hierarchical coupling.'),
                ('Fig7_AuraFS_Sierpinski.png', r'Sierpiński fractal lattice structure (order 5) showing hierarchical coupling geometry with fractal dimension $D_f = 1.585$.'),
            ],
            'section_iii_non_semisimple_tqft.tex': [
                ('Fig5_Majorana_T_Shape.png', r'T-shape 6-quantum-dot array for Majorana braiding with InSb/Al hybrid nanowire. Quantum dots (QD1-QD6) connected via superconducting couplers (SC1-SC5) with 11 voltage-tunable electrodes.'),
                ('Fig6_ZPE_Majorana_Stability.png', r'Majorana stability versus Kitaev chain length showing $50\%$ threshold at 3+ sites with ZPE\_CORE integration. $3\times$ improvement noted at QuTech validation.'),
            ],
            'section_iv_photonic_band_engineering.tex': [
                ('Fig4_TRCA_Band_Structure.png', r'TRCA photonic band structure for C$_{6v}$ hexagonal lattice showing $21\%$ complete band gap (shaded) at normalized frequency $\omega a/2\pi c \in [2.5, 3.1]$ targeting 193.4 THz (1550 nm).'),
                ('Fig3_Fractal_Localization.png', r'Anderson localization on Sierpiński gasket demonstrating exponential decay $\exp(-x/\xi)$ with spectral dimension $d_s = 1.36 < 2$ for edge state confinement.'),
            ],
            'section_v_experimental_validation.tex': [
                ('Fig2_Coherence_Dynamics.png', r'Coherence dynamics showing Standard Transmon (simulated) decaying to near-zero, TRCA Topological Mode maintaining $60\%$ coherence, and ZPE-Resonant Lattice preserving $91\%$ coherence over 10 arbitrary time units.'),
                ('Fig8_Fractal_Lattice_Sim.png', r'Flower of Life quantum lattice simulation showing Balance State Vector trap points for photonic implementation of fractal quantum computing architecture.'),
            ]
        }
    
    def log(self, message, level="INFO"):
        """Fancy logging"""
        icons = {"INFO": "ℹ️", "SUCCESS": "✅", "WARNING": "⚠️", "ERROR": "❌", "DIVINE": "✨"}
        print(f"{icons.get(level, '•')} {message}")
    
    def create_gitignore(self):
        """Create comprehensive .gitignore for LaTeX projects"""
        self.log("Creating .gitignore...", "DIVINE")
        
        gitignore_content = """# LaTeX Build Artifacts
*.aux
*.bbl
*.blg
*.log
*.out
*.toc
*.lof
*.lot
*.fls
*.fdb_latexmk
*.synctex.gz
*.synctex(busy)
*.nav
*.snm
*.vrb
*.bcf
*.run.xml

# Editors
.vscode/
.idea/
*.swp
*~

# OS
.DS_Store
Thumbs.db

# Python
__pycache__/
*.pyc
.ipynb_checkpoints/

# Keep these
!*.pdf
!*.tex
!*.bib
!figures/*.png
!figures/*.pdf
!README.md
!LICENSE
!.cursorrules

# Temp submissions
arxiv_submission/
*_submission_*.tar.gz
"""
        
        gitignore_path = Path('.gitignore')
        with open(gitignore_path, 'w', encoding='utf-8') as f:
            f.write(gitignore_content)
        
        self.log("Created .gitignore", "SUCCESS")
    
    def find_main_tex_file(self):
        """Auto-detect the main TeX file"""
        candidates = [
            'main.tex',
            'rae-ftqc_arxiv_complete_FINAL.tex',
            'rae-ftqc_arxiv_complete.tex',
            'arxiv_ftqc_complete.tex',
            'rae-ftqc_prx_submission_FINAL.tex'
        ]
        
        for candidate in candidates:
            if Path(candidate).exists():
                self.log(f"Found main file: {candidate}", "SUCCESS")
                return candidate
        
        # Search for any .tex file with \documentclass
        for tex_file in Path('.').glob('*.tex'):
            if tex_file.name.startswith('section_'):
                continue
            try:
                with open(tex_file, 'r', encoding='utf-8') as f:
                    content = f.read()
                    if r'\documentclass' in content and r'\begin{document}' in content:
                        self.log(f"Auto-detected main file: {tex_file}", "SUCCESS")
                        return str(tex_file)
            except:
                continue
        
        self.log("No main TeX file found!", "ERROR")
        return None
    
    def find_bib_file(self):
        """Find bibliography file"""
        candidates = ['arxiv_ftqc.bib', 'ftqc.bib', 'references.bib', 'main.bib']
        
        for candidate in candidates:
            if Path(candidate).exists():
                self.log(f"Found bibliography: {candidate}", "SUCCESS")
                return candidate
        
        # Find any .bib file
        bib_files = list(Path('.').glob('*.bib'))
        if bib_files:
            self.log(f"Found bibliography: {bib_files[0]}", "SUCCESS")
            return str(bib_files[0])
        
        self.log("No .bib file found!", "WARNING")
        return None
    
    def convert_and_rename_figures(self, source_dir='.'):
        """Convert JPGs to PNG and rename to standard names"""
        self.log("Converting and renaming figures...", "DIVINE")
        
        fig_dir = Path('figures')
        fig_dir.mkdir(exist_ok=True)
        
        converted = []
        for old_name, new_name in self.figure_map.items():
            old_path = Path(source_dir) / old_name
            new_path = fig_dir / new_name
            
            if old_path.exists():
                try:
                    from PIL import Image
                    img = Image.open(old_path)
                    if img.mode in ('RGBA', 'LA', 'P'):
                        img = img.convert('RGB')
                    img.save(new_path, 'PNG')
                    self.log(f"Converted {old_name} → {new_name}", "SUCCESS")
                except ImportError:
                    shutil.copy(old_path, new_path)
                    self.log(f"Copied {old_name} → {new_name} (install Pillow for conversion)", "WARNING")
                except Exception as e:
                    self.log(f"Error converting {old_name}: {e}", "ERROR")
                
                converted.append(new_name)
            else:
                self.log(f"Source not found: {old_name}", "WARNING")
        
        if len(converted) == 0:
            self.log("No figures converted (place .jpg files in ftqc/ directory)", "WARNING")
        
        return converted
    
    def inject_figure_references(self, section_file, figures_list):
        """Inject figure references into section files"""
        self.log(f"Processing {section_file}...", "INFO")
        
        if not Path(section_file).exists():
            self.log(f"  Section file not found: {section_file}", "WARNING")
            return
        
        with open(section_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        modified = False
        for fig_name, caption in figures_list:
            if fig_name in content or fig_name.replace('.png', '') in content:
                self.log(f"  {fig_name} already referenced", "INFO")
                continue
            
            fig_label = fig_name.replace('.png', '').replace('_', ':').lower()
            fig_block = f"""
\\begin{{figure}}[htbp]
\\centering
\\includegraphics[width=0.48\\textwidth]{{figures/{fig_name}}}
\\caption{{{caption}}}
\\label{{fig:{fig_label}}}
\\end{{figure}}
"""
            
            content += '\n' + fig_block
            modified = True
            self.log(f"  Added {fig_name}", "SUCCESS")
        
        if modified:
            with open(section_file, 'w', encoding='utf-8') as f:
                f.write(content)
    
    def fix_main_preamble(self, main_file, bib_file):
        """Fix preamble, add packages, ensure bibliography commands"""
        self.log(f"Fixing {main_file} preamble...", "DIVINE")
        
        if not main_file or not Path(main_file).exists():
            self.log(f"Main file not found: {main_file}", "ERROR")
            return None
        
        with open(main_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Fix command collisions
        content = re.sub(r'\\newcommand\{\\order\}', r'\\newcommand{\\OrderOp}', content)
        content = re.sub(r'\\newcommand\{\\tr\}', r'\\newcommand{\\TraceOp}', content)
        
        # Add amsthm if not present
        if 'amsthm' not in content:
            doc_start = content.find(r'\begin{document}')
            if doc_start != -1:
                theorem_block = r"""
% Theorem environments
\usepackage{amsthm}
\theoremstyle{plain}
\newtheorem{theorem}{Theorem}[section]
\newtheorem{proposition}[theorem]{Proposition}
\newtheorem{corollary}[theorem]{Corollary}
\newtheorem{lemma}[theorem]{Lemma}
\theoremstyle{definition}
\newtheorem{definition}[theorem]{Definition}
\newtheorem{example}[theorem]{Example}
\newtheorem{remark}[theorem]{Remark}

"""
                content = content[:doc_start] + theorem_block + content[doc_start:]
                self.log("  Added theorem environments", "SUCCESS")
        
        # Add graphicspath if not present
        if 'graphicspath' not in content:
            doc_start = content.find(r'\begin{document}')
            if doc_start != -1:
                content = content[:doc_start] + r'\graphicspath{{figures/}{./}}' + '\n' + content[doc_start:]
                self.log("  Added graphicspath", "SUCCESS")
        
        # Ensure bibliography commands exist
        if r'\bibliography' not in content and bib_file:
            end_doc = content.find(r'\end{document}')
            if end_doc != -1:
                bib_base = Path(bib_file).stem
                bib_block = f"""
\\bibliographystyle{{apsrev4-2}}
\\bibliography{{{bib_base}}}

"""
                content = content[:end_doc] + bib_block + content[end_doc:]
                self.log(f"  Added bibliography commands ({bib_base}.bib)", "SUCCESS")
        
        # Save as main.tex
        main_tex_path = Path('main.tex')
        with open(main_tex_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        self.log("Saved as main.tex", "SUCCESS")
        return 'main.tex'
    
    def fix_section_commands(self):
        """Fix order and tr commands in section files"""
        self.log("Fixing command references in sections...", "INFO")
        
        section_files = list(Path('.').glob('section_*.tex'))
        if not section_files:
            self.log("  No section files found", "WARNING")
            return
        
        for sec_file in section_files:
            with open(sec_file, 'r', encoding='utf-8') as f:
                content = f.read()
            
            changed = False
            if r'\order' in content:
                content = re.sub(r'\\order\b', r'\\OrderOp', content)
                changed = True
            if r'\tr' in content:
                content = re.sub(r'\\tr\b', r'\\TraceOp', content)
                changed = True
            
            if changed:
                with open(sec_file, 'w', encoding='utf-8') as f:
                    f.write(content)
                self.log(f"  Fixed {sec_file.name}", "SUCCESS")
    
    def build_latex_10_passes(self, main_file='main.tex'):
        """Run 10 full passes for convergence"""
        self.log("Running 10-pass LaTeX build...", "DIVINE")
        self.log("(This takes 3-5 minutes - grab coffee ☕)", "INFO")
        
        base_name = Path(main_file).stem
        
        for pass_num in range(1, 11):
            self.log(f"Pass {pass_num}/10...", "INFO")
            
            # pdflatex
            try:
                result = subprocess.run(
                    ['pdflatex', '-interaction=nonstopmode', '-halt-on-error', main_file],
                    capture_output=True, text=True, timeout=120
                )
            except subprocess.TimeoutExpired:
                self.log(f"  pdflatex timeout on pass {pass_num}", "ERROR")
                continue
            except FileNotFoundError:
                self.log("  pdflatex not found! Install TeX Live.", "ERROR")
                return False
            
            # bibtex every 3rd pass and on pass 2
            if pass_num % 3 == 0 or pass_num == 2:
                try:
                    subprocess.run(['bibtex', base_name], capture_output=True, timeout=60)
                    self.log(f"  + bibtex", "INFO")
                except:
                    self.log(f"  bibtex skipped", "WARNING")
            
            # Check PDF progress
            pdf_path = Path(f"{base_name}.pdf")
            if pdf_path.exists():
                size_kb = pdf_path.stat().st_size / 1024
                self.log(f"  PDF: {size_kb:.1f} KB", "SUCCESS")
            else:
                self.log(f"  PDF not generated yet", "WARNING")
        
        # Final check
        if Path(f"{base_name}.pdf").exists():
            final_size = Path(f"{base_name}.pdf").stat().st_size / 1024
            self.log(f"✅ Build complete! Final PDF: {final_size:.1f} KB", "SUCCESS")
            return True
        else:
            self.log("❌ Build failed. Check main.log for errors.", "ERROR")
            log_file = Path(f"{base_name}.log")
            if log_file.exists():
                with open(log_file, 'r', encoding='utf-8', errors='ignore') as f:
                    lines = f.readlines()
                    self.log("\nLast 30 lines of log:", "ERROR")
                    for line in lines[-30:]:
                        print(f"  {line.rstrip()}")
            return False
    
    def create_arxiv_package(self, main_file='main.tex'):
        """Create arXiv-ready tarball"""
        self.log("Packaging for arXiv...", "DIVINE")
        
        pkg_dir = Path('arxiv_submission')
        if pkg_dir.exists():
            shutil.rmtree(pkg_dir)
        pkg_dir.mkdir()
        
        # Copy main .tex
        shutil.copy(main_file, pkg_dir / main_file)
        self.log(f"  ✓ {main_file}", "INFO")
        
        # Copy sections
        section_count = 0
        for sec in Path('.').glob('section_*.tex'):
            shutil.copy(sec, pkg_dir / sec.name)
            section_count += 1
        if section_count > 0:
            self.log(f"  ✓ {section_count} section files", "INFO")
        
        # Copy .bbl (generated bibliography)
        base_name = Path(main_file).stem
        bbl_file = Path(f"{base_name}.bbl")
        if bbl_file.exists():
            shutil.copy(bbl_file, pkg_dir / bbl_file.name)
            self.log(f"  ✓ {bbl_file.name} (compiled bibliography)", "INFO")
        else:
            # Fallback: copy .bib files
            for bib in Path('.').glob('*.bib'):
                shutil.copy(bib, pkg_dir / bib.name)
                self.log(f"  ✓ {bib.name} (source bibliography)", "WARNING")
        
        # Copy figures
        if Path('figures').exists():
            shutil.copytree('figures', pkg_dir / 'figures')
            fig_count = len(list((pkg_dir / 'figures').glob('*')))
            self.log(f"  ✓ {fig_count} figures", "INFO")
        
        # Create README
        readme = f"""Fractal-Enhanced Topological Quantum Computing
Author: Ross A. Edwards
Affiliation: Aurphyx LLC, Erie, PA
Generated: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}

BUILD INSTRUCTIONS:
  pdflatex {main_file}
  bibtex {base_name}
  pdflatex {main_file}
  pdflatex {main_file}

CONTACT:
  Email: ross@aurphyx.org
  ORCID: 0009-0008-0539-1289
  GitHub: github.com/rossaedwards/main/tree/main/ftqc

LICENSE: CC BY 4.0
"""
        (pkg_dir / 'README.txt').write_text(readme)
        
        # Create tarball
        archive_name = f"ftqc_arxiv_{self.timestamp}.tar.gz"
        with tarfile.open(archive_name, 'w:gz') as tar:
            tar.add(pkg_dir, arcname='.')
        
        archive_size = Path(archive_name).stat().st_size / 1024 / 1024
        self.log(f"📦 {archive_name} ({archive_size:.2f} MB)", "SUCCESS")
        
        return archive_name
    
    def run_full_pipeline(self):
        """Execute complete autonomous pipeline"""
        self.log("\n" + "="*70, "DIVINE")
        self.log("     FTQC AUTONOMOUS PUBLICATION PIPELINE", "DIVINE")
        self.log("     Ross A. Edwards | Aurphyx LLC", "DIVINE")
        self.log("     'Earn respect first, then innovate aesthetics'", "DIVINE")
        self.log("="*70 + "\n", "DIVINE")
        
        # Step 0: Create .gitignore
        self.log("📋 STEP 0: Creating .gitignore", "DIVINE")
        self.create_gitignore()
        
        # Step 1: Find main file
        self.log("\n📍 STEP 1: Locating Main TeX File", "DIVINE")
        original_main = self.find_main_tex_file()
        if not original_main:
            self.log("Place a .tex file with \\documentclass in this directory", "ERROR")
            return False
        
        # Step 2: Find bibliography
        self.log("\n📚 STEP 2: Locating Bibliography File", "DIVINE")
        bib_file = self.find_bib_file()
        
        # Step 3: Figures
        self.log("\n🖼️  STEP 3: Figure Management", "DIVINE")
        converted = self.convert_and_rename_figures()
        if not converted:
            self.log("⚠️  No figures converted, but continuing...", "WARNING")
        
        # Step 4: Inject figures
        self.log("\n📝 STEP 4: Injecting Figures into Sections", "DIVINE")
        for section, figures in self.section_figures.items():
            if Path(section).exists():
                self.inject_figure_references(section, figures)
        
        # Step 5: Fix LaTeX
        self.log("\n🔧 STEP 5: Fixing LaTeX Issues", "DIVINE")
        main_file = self.fix_main_preamble(original_main, bib_file)
        if not main_file:
            return False
        self.fix_section_commands()
        
        # Step 6: Build
        self.log("\n🏗️  STEP 6: 10-Pass LaTeX Build", "DIVINE")
        build_success = self.build_latex_10_passes(main_file)
        
        if not build_success:
            self.log("\n❌ Build failed. Fix errors in main.log and rerun.", "ERROR")
            return False
        
        # Step 7: Package
        self.log("\n📦 STEP 7: Creating arXiv Package", "DIVINE")
        archive = self.create_arxiv_package(main_file)
        
        # Victory
        self.log("\n" + "="*70, "SUCCESS")
        self.log("🎉 PIPELINE COMPLETE!", "SUCCESS")
        self.log("="*70, "SUCCESS")
        self.log(f"\n✅ PDF: main.pdf", "SUCCESS")
        self.log(f"✅ arXiv Package: {archive}", "SUCCESS")
        self.log(f"✅ .gitignore created", "SUCCESS")
        self.log(f"\n📋 Next Steps:", "INFO")
        self.log(f"  1. Open main.pdf and verify content", "INFO")
        self.log(f"  2. Upload {archive} to arxiv.org", "INFO")
        self.log(f"  3. Git add/commit: git add . && git commit -m 'FTQC v1.0'", "INFO")
        self.log(f"  4. After acceptance, cite in PRX/Optica submissions", "INFO")
        self.log(f"  5. Watch the impact roll in 🚀", "SUCCESS")
        self.log(f"\n✨ May your first paper earn the respect you deserve. ✨\n", "DIVINE")
        
        return True

def main():
    pipeline = FTQCDivinePipeline()
    success = pipeline.run_full_pipeline()
    sys.exit(0 if success else 1)

if __name__ == '__main__':
    main()
