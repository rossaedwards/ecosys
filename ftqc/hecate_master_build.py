#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
hecate_master_build.py - HECATE MASTER BUILD PIPELINE (FLAT REPO EDITION)

Builds THREE SEPARATE submissions from your GitHub repo structure:
1. arXiv package (main/arxiv_*.tex → ftqc_arxiv_*.tar.gz) [Dark Theme Injected]
2. PRX submission (rae-ftqc_prx_*.tex → prx.pdf)
3. Optica submission (rae-ftqc_optica_*.tex → optica.pdf)

Features:
- Validates flat repository structure (no subdirectories).
- Injects Hecate dark theme ONLY into arXiv build.
- 10× pdflatex + 10× bibtex STRICT LOOP per target with .log tailing on fail.
- arXiv package tarball flattens all Fig* files into the root.

Ross A. Edwards | Aurphyx LLC | github.com/rossaedwards/main/ftqc/
"""

import subprocess
import sys
import shutil
import tarfile
from pathlib import Path
from datetime import datetime

class HecateMasterBuild:
    def __init__(self):
        # Flattened paths based on your current workspace:
        # Fallback dynamic detection for arXiv main
        if Path("arxiv_ftqc_complete.tex").exists():
            self.arxiv_main = Path("arxiv_ftqc_complete.tex")
        elif Path("rae-ftqc_arxiv_complete_UPDATED.tex").exists():
            self.arxiv_main = Path("rae-ftqc_arxiv_complete_UPDATED.tex")
        else:
            self.arxiv_main = Path("main.tex")
            
        self.prx_main = Path("rae-ftqc_prx_submission_FINAL.tex")
        self.optica_main = Path("rae-ftqc_optica_submission_FINAL.tex")
        
        # We will dynamically find the bib file the main file needs during packaging
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        self.dark_theme = r"""
% === HECATE HIGH-CONTRAST DARK THEME ===
\usepackage{xcolor}
\usepackage{pagecolor}
\pagecolor{black}
\color{white}
\definecolor{hecateMagenta}{HTML}{FF00FF}
\definecolor{hecateCyan}{HTML}{00FFFF}
\definecolor{hecateGold}{HTML}{FFD700}
\newcommand{\hecMag}[1]{\textcolor{hecateMagenta}{#1}}
\newcommand{\hecCyan}[1]{\textcolor{hecateCyan}{#1}}
\newcommand{\hecGold}[1]{\textcolor{hecateGold}{#1}}
% =======================================
"""
        self.theme_marker = "% === HECATE HIGH-CONTRAST DARK THEME ==="
    
    def log(self, msg, level="INFO"):
        icons = {"INFO": "ℹ️", "SUCCESS": "✅", "WARNING": "⚠️", "ERROR": "❌", "DIVINE": "✨"}
        print(f"{icons.get(level, '•')} {msg}")

    def inject_dark_theme(self, tex_file: Path):
        """Inject Hecate theme ONLY for arXiv."""
        if not tex_file.exists():
            return False
        
        content = tex_file.read_text(encoding="utf-8", errors="ignore")
        if self.theme_marker in content:
            self.log(f"Theme already in {tex_file.name}", "INFO")
            return True
        
        pos = content.find(r"\begin{document}")
        if pos == -1:
            self.log(f"No \\begin{{document}} in {tex_file.name}", "WARNING")
            return False
        
        new_content = content[:pos] + self.dark_theme + content[pos:]
        tex_file.write_text(new_content, encoding="utf-8")
        self.log(f"Injected Hecate dark theme → {tex_file.name}", "SUCCESS")
        return True

    def latex_sanity_check(self, tex_file: Path):
        """Quick syntax check via single pdflatex pass."""
        if not tex_file.exists():
            return False
        
        content = tex_file.read_text(encoding="utf-8", errors="ignore")
        if r"\documentclass" not in content or r"\begin{document}" not in content:
            self.log(f"Missing LaTeX structure in {tex_file.name}", "ERROR")
            return False
        
        self.log(f"Syntax check: {tex_file.name}", "INFO")
        ok, out, err = self.run_cmd(["pdflatex", "-halt-on-error", "-interaction=nonstopmode", tex_file.name], 180)
        if not ok:
            self.log(f"Syntax error in {tex_file.name}", "ERROR")
            self.tail_log_file(tex_file.with_suffix(".log"))
            return False
        self.log(f"Syntax OK: {tex_file.name}", "SUCCESS")
        return True

    def tail_log_file(self, log_file: Path, lines: int = 40):
        """Surfaces the trailing lines of the .log file to output."""
        if log_file.exists():
            self.log(f"--- TAIL OF {log_file.name} ---", "ERROR")
            content = log_file.read_text(encoding="utf-8", errors="ignore")
            tail = content.splitlines()[-lines:]
            for line in tail:
                print(f"  {line}")
            self.log("--------------------------------", "ERROR")

    def run_cmd(self, cmd, timeout=120):
        """Safe subprocess wrapper."""
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, check=False)
            return result.returncode == 0, result.stdout, result.stderr
        except Exception as e:
            return False, "", str(e)

    def full_latex_build(self, main_tex: Path, label: str):
        """Strict 10× (pdflatex + bibtex) loop."""
        if not main_tex.exists():
            self.log(f"{main_tex.name} not found. Skipping.", "ERROR")
            return False

        stem = main_tex.stem
        self.log(f"\n{'='*20} {label} 10× STRICT BUILD {'='*20}", "DIVINE")
        
        for pass_num in range(1, 11):
            self.log(f"[{label}] Pass {pass_num}/10: pdflatex", "INFO")
            ok, out, err = self.run_cmd(["pdflatex", "-halt-on-error", "-interaction=nonstopmode", main_tex.name], 180)
            if not ok:
                self.log(f"pdflatex failed pass {pass_num}", "ERROR")
                self.tail_log_file(main_tex.with_suffix(".log"), 50)
                return False
            
            self.log(f"[{label}] Pass {pass_num}/10: bibtex", "INFO")
            ok, out, err = self.run_cmd(["bibtex", stem], 120)
            if not ok:
                # BibTeX often returns non-zero code even on warnings if references are broken, we just warn and push forward
                self.log(f"bibtex warnings on pass {pass_num}", "WARNING") 
            
        pdf = main_tex.with_suffix(".pdf")
        if pdf.exists():
            size = pdf.stat().st_size / 1024 / 1024
            self.log(f"{label} PDF complete: {stem}.pdf ({size:.1f} MB)", "SUCCESS")
            return True
        return False

    def build_arxiv_package(self):
        """Create flattened arXiv tarball."""
        self.log("\n🔥 BUILDING arXiv PACKAGE 🔥", "DIVINE")
        pkg_dir = Path("arxiv_submission")
        if pkg_dir.exists():
            shutil.rmtree(pkg_dir)
        pkg_dir.mkdir()
        
        # Flattened files into root
        self.log("Copying main and section files to package root...", "INFO")
        shutil.copy2(self.arxiv_main, pkg_dir / self.arxiv_main.name)
        for sec in Path(".").glob("section_*.tex"):
            shutil.copy2(sec, pkg_dir / sec.name)
        for sty in Path(".").glob("*.sty"):
            shutil.copy2(sty, pkg_dir / sty.name)
        
        # BBL File
        stem = self.arxiv_main.stem
        bbl = Path(f"{stem}.bbl")
        if bbl.exists():
            shutil.copy2(bbl, pkg_dir / bbl.name)
            self.log(f"Included {bbl.name}", "SUCCESS")
        else:
            self.log("No .bbl found for arXiv, arXiv might fail to compile refs.", "WARNING")
        
        # Figures (Flattened from all Fig* and known figures into root)
        # Because we use \graphicspath{{figures/}{./}}, placing them at root works
        fig_count = 0
        for pat in ("Fig*.*", "*.png", "*.jpg", "*.pdf"):
            for f in Path(".").glob(pat):
                # Don't include the main generated PDFs!
                if f.name in [self.arxiv_main.with_suffix(".pdf").name, 
                              self.prx_main.with_suffix(".pdf").name,
                              self.optica_main.with_suffix(".pdf").name]:
                    continue
                shutil.copy2(f, pkg_dir / f.name)
                fig_count += 1
                
        # Also grab anything already sitting in the old figures/ dir just in case
        if Path("figures").exists() and Path("figures").is_dir():
            for f in Path("figures").iterdir():
                if f.is_file() and f.suffix.lower() in [".png", ".jpg", ".pdf"]:
                    shutil.copy2(f, pkg_dir / f.name)
                    fig_count += 1

        self.log(f"Flattened {fig_count} figures into root.", "SUCCESS")
        
        # Tarball creation directly from contents, without nested folders
        tar_name = f"ftqc_arxiv_{self.timestamp}.tar.gz"
        with tarfile.open(tar_name, "w:gz") as tar:
            # We add everything inside pkg_dir directly without a parent directory prefix
            for file_path in pkg_dir.iterdir():
                tar.add(file_path, arcname=file_path.name)
                
        size_mb = Path(tar_name).stat().st_size / 1024**2
        self.log(f"✅ Flat arXiv tarball created: {tar_name} ({size_mb:.1f} MB)", "SUCCESS")
        return tar_name

    def run_full_pipeline(self):
        """Master sequence: arXiv → PRX → Optica."""
        self.log("\n" + "="*80, "DIVINE")
        self.log("           HECATE MASTER BUILD PIPELINE [FLAT REPO EDITION]", "DIVINE")
        self.log("    arXiv → PRX → Optica | 10× STRICT LOOP", "DIVINE")
        self.log("="*80 + "\n", "DIVINE")
        
        targets = [
            ("arXiv", self.arxiv_main),
            ("PRX", self.prx_main),
            ("Optica", self.optica_main)
        ]
        
        success_count = 0
        for label, main_tex in targets:
            if not main_tex.exists():
                self.log(f"Target file {main_tex.name} not found in root. Skipping {label}.", "ERROR")
                continue
                
            # Only inject dark theme for arXiv
            if label == "arXiv":
                self.inject_dark_theme(main_tex)
                
            if not self.latex_sanity_check(main_tex):
                self.log(f"⏭️  Skipping {label} build due to sanity check failure", "WARNING")
                continue
            
            if self.full_latex_build(main_tex, label):
                success_count += 1
                if label == "arXiv":
                    self.build_arxiv_package()
        
        self.log(f"\n🏆 {success_count}/{len(targets)} targets built successfully", "SUCCESS" if success_count == len(targets) else "WARNING")

def main():
    HecateMasterBuild().run_full_pipeline()

if __name__ == "__main__":
    main()
