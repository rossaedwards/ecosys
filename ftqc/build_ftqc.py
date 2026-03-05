#!/usr/bin/env python3
"""
FTQC Divine Pipeline - Modified for Existing Figures
Ross A. Edwards | Aurphyx LLC
Skips figure conversion since figures/ already has PNGs
"""
import re
import subprocess
import sys
import shutil
import tarfile
from pathlib import Path
from datetime import datetime

class FTQCDivinePipeline:
    def __init__(self):
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    
    def log(self, message, level="INFO"):
        """Fancy logging"""
        icons = {"INFO": "ℹ️", "SUCCESS": "✅", "WARNING": "⚠️", "ERROR": "❌", "DIVINE": "✨"}
        print(f"{icons.get(level, '•')} {message}")
    
    def verify_figures(self):
        """Check that figures exist"""
        self.log("Verifying figures...", "DIVINE")
        
        fig_dir = Path('figures')
        if not fig_dir.exists():
            self.log("figures/ directory not found!", "ERROR")
            return False
        
        fig_files = list(fig_dir.glob('*.png')) + list(fig_dir.glob('*.pdf'))
        
        if not fig_files:
            self.log("No figures found in figures/ directory!", "ERROR")
            return False
        
        self.log(f"Found {len(fig_files)} figure files:", "SUCCESS")
        for f in sorted(fig_files)[:10]:  # Show first 10
            self.log(f"  {f.name}", "INFO")
        
        return True
    
    def find_main_tex_file(self):
        """Auto-detect the main TeX file"""
        candidates = [
            'main.tex',
            'rae-ftqc_arxiv_complete_FINAL.tex',
            'rae-ftqc_arxiv_complete.tex',
        ]
        
        for candidate in candidates:
            if Path(candidate).exists():
                self.log(f"Found main file: {candidate}", "SUCCESS")
                return candidate
        
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
        candidates = ['arxiv_ftqc.bib', 'master_citations.bib', 'ftqc.bib']
        
        for candidate in candidates:
            if Path(candidate).exists():
                self.log(f"Found bibliography: {candidate}", "SUCCESS")
                return candidate
        
        bib_files = list(Path('.').glob('*.bib'))
        if bib_files:
            self.log(f"Found bibliography: {bib_files[0]}", "SUCCESS")
            return str(bib_files[0])
        
        self.log("No .bib file found!", "WARNING")
        return None
    
    def ensure_bibliography_commands(self, main_file, bib_file):
        """Ensure bibliography commands are in main.tex"""
        self.log(f"Checking bibliography commands in {main_file}...", "INFO")
        
        with open(main_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        if r'\bibliography' in content:
            self.log("Bibliography commands already present", "SUCCESS")
            return True
        
        if not bib_file:
            self.log("No .bib file to reference", "WARNING")
            return False
        
        self.log("Adding bibliography commands...", "INFO")
        
        bib_base = Path(bib_file).stem
        end_doc = content.find(r'\end{document}')
        
        if end_doc == -1:
            self.log("Could not find \\end{document}", "ERROR")
            return False
        
        bib_block = f"""
\\bibliographystyle{{apsrev4-2}}
\\bibliography{{{bib_base}}}

"""
        content = content[:end_doc] + bib_block + content[end_doc:]
        
        with open(main_file, 'w', encoding='utf-8') as f:
            f.write(content)
        
        self.log(f"Added bibliography commands ({bib_base}.bib)", "SUCCESS")
        return True
    
    def build_latex(self, main_file='main.tex', passes=4):
        """Run LaTeX build with multiple passes"""
        self.log(f"Running {passes}-pass LaTeX build...", "DIVINE")
        
        base_name = Path(main_file).stem
        
        for pass_num in range(1, passes + 1):
            self.log(f"Pass {pass_num}/{passes}...", "INFO")
            
            # pdflatex
            try:
                subprocess.run(
                    ['pdflatex', '-interaction=nonstopmode', main_file],
                    capture_output=True, timeout=120
                )
            except:
                self.log(f"pdflatex warning on pass {pass_num}", "WARNING")
            
            # bibtex on pass 2
            if pass_num == 2:
                try:
                    subprocess.run(['bibtex', base_name], capture_output=True, timeout=60)
                    self.log("  + bibtex", "INFO")
                except:
                    pass
            
            # Check PDF
            pdf_path = Path(f"{base_name}.pdf")
            if pdf_path.exists():
                size_kb = pdf_path.stat().st_size / 1024
                self.log(f"  PDF: {size_kb:.1f} KB", "SUCCESS")
        
        # Final check
        if Path(f"{base_name}.pdf").exists():
            final_size = Path(f"{base_name}.pdf").stat().st_size / 1024
            self.log(f"Build complete! Final PDF: {final_size:.1f} KB", "SUCCESS")
            return True
        else:
            self.log("Build failed. Check main.log", "ERROR")
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
        
        # Copy sections
        for sec in Path('.').glob('section_*.tex'):
            shutil.copy(sec, pkg_dir / sec.name)
        
        # Copy .bbl
        base_name = Path(main_file).stem
        bbl_file = Path(f"{base_name}.bbl")
        if bbl_file.exists():
            shutil.copy(bbl_file, pkg_dir / bbl_file.name)
            self.log(f"  ✓ {bbl_file.name}", "INFO")
        
        # Copy figures
        if Path('figures').exists():
            shutil.copytree('figures', pkg_dir / 'figures')
            fig_count = len(list((pkg_dir / 'figures').glob('*')))
            self.log(f"  ✓ {fig_count} figures", "INFO")
        
        # Create tarball
        archive_name = f"ftqc_arxiv_{self.timestamp}.tar.gz"
        with tarfile.open(archive_name, 'w:gz') as tar:
            tar.add(pkg_dir, arcname='.')
        
        archive_size = Path(archive_name).stat().st_size / 1024 / 1024
        self.log(f"📦 {archive_name} ({archive_size:.2f} MB)", "SUCCESS")
        
        return archive_name
    
    def run_full_pipeline(self):
        """Execute pipeline"""
        self.log("\n" + "="*70, "DIVINE")
        self.log("FTQC PUBLICATION PIPELINE (Existing Figures)", "DIVINE")
        self.log("="*70 + "\n", "DIVINE")
        
        # Check figures
        if not self.verify_figures():
            return False
        
        # Find files
        main_file = self.find_main_tex_file()
        if not main_file:
            return False
        
        bib_file = self.find_bib_file()
        
        # Ensure bibliography commands
        self.ensure_bibliography_commands(main_file, bib_file)
        
        # Build
        if not self.build_latex(main_file, passes=4):
            return False
        
        # Package
        archive = self.create_arxiv_package(main_file)
        
        self.log("\n" + "="*70, "SUCCESS")
        self.log("🎉 COMPLETE!", "SUCCESS")
        self.log("="*70, "SUCCESS")
        self.log(f"\n✅ PDF: {Path(main_file).stem}.pdf", "SUCCESS")
        self.log(f"✅ arXiv Package: {archive}", "SUCCESS")
        
        return True

def main():
    pipeline = FTQCDivinePipeline()
    success = pipeline.run_full_pipeline()
    sys.exit(0 if success else 1)

if __name__ == '__main__':
    main()
