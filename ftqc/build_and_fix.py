#!/usr/bin/env python3
import re
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent

def fix_preamble(main_file):
    """Fix command collisions and add missing environments"""
    print(f"Fixing preamble in {main_file}...")
    
    with open(main_file, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Fix command collisions: \order -> \OrderOp, \tr -> \TraceOp
    content = re.sub(r'\\newcommand\{\\order\}', r'\\newcommand{\\OrderOp}', content)
    content = re.sub(r'\\newcommand\{\\tr\}', r'\\newcommand{\\TraceOp}', content)
    
    # Add amsthm and theorem environments if not present
    if 'amsthm' not in content:
        # Find \begin{document} and insert before it
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
    
    # Add graphicspath if not present
    if 'graphicspath' not in content:
        graphics_line = r'\graphicspath{{figures/}{./}}' + '\n'
        doc_start = content.find(r'\begin{document}')
        if doc_start != -1:
            content = content[:doc_start] + graphics_line + content[doc_start:]
    
    with open(main_file, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print("[OK] Preamble fixed")

def fix_references_in_files(tex_files):
    r"""Replace \order and \tr in all section files."""
    print("Fixing command references in section files...")
    
    for tex_file in tex_files:
        if not tex_file.exists():
            continue
            
        with open(tex_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Replace command calls (not definitions)
        changed = False
        if r'\order{' in content or r'\order ' in content:
            content = re.sub(r'\\order\b', r'\\OrderOp', content)
            changed = True
        if r'\tr{' in content or r'\tr ' in content:
            content = re.sub(r'\\tr\b', r'\\TraceOp', content)
            changed = True
        
        if changed:
            with open(tex_file, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  [OK] Fixed {tex_file.name}")

def build_latex(main_file):
    """Run standard LaTeX build sequence for the selected root file."""
    print("\n=== Building LaTeX Document ===")

    bib_stem = Path(main_file).stem
    commands = [
        ['pdflatex', '-interaction=nonstopmode', main_file],
        ['bibtex', bib_stem],
        ['pdflatex', '-interaction=nonstopmode', main_file],
        ['pdflatex', '-interaction=nonstopmode', main_file]
    ]
    
    for i, cmd in enumerate(commands, 1):
        print(f"\nStep {i}/4: {' '.join(cmd)}")
        result = subprocess.run(cmd, capture_output=False, cwd=SCRIPT_DIR)
        if result.returncode != 0:
            print(f"[WARN] {cmd[0]} returned code {result.returncode}")
            if i == 1:  # First pdflatex failed critically
                print("Build failed. Check main.log for errors.")
                sys.exit(1)
    
    print(f"\n[OK] Build complete: {bib_stem}.pdf")

def choose_main_file(cli_arg=None):
    """Resolve the root TeX file from CLI, default names, or discovery."""
    if cli_arg:
        candidate = SCRIPT_DIR / cli_arg
        if not candidate.exists():
            raise FileNotFoundError(f"Requested main file not found: {candidate}")
        return candidate

    preferred = [
        "main.tex",
        "arxiv_ftqc_complete.tex",
        "rae-ftqc_arxiv_complete_UPDATED.tex",
        "rae-ftqc_arxiv_complete_FINAL.tex",
    ]
    for name in preferred:
        candidate = SCRIPT_DIR / name
        if candidate.exists():
            return candidate

    # Fallback: choose any TeX file that looks like a document root.
    for tex_file in sorted(SCRIPT_DIR.glob("*.tex")):
        content = tex_file.read_text(encoding="utf-8", errors="ignore")
        if r"\begin{document}" in content:
            return tex_file

    raise FileNotFoundError(
        "No root TeX file found. Pass one explicitly, e.g. "
        "'python build_and_fix.py arxiv_ftqc_complete.tex'."
    )

def main():
    cli_main = sys.argv[1] if len(sys.argv) > 1 else None
    root_file = choose_main_file(cli_main)
    print(f"Using root file: {root_file.name}")

    # Fix preamble
    fix_preamble(root_file)

    # Fix all section files
    section_files = list(SCRIPT_DIR.glob("section_*.tex"))
    fix_references_in_files(section_files)

    # Build
    build_latex(root_file.name)

if __name__ == '__main__':
    main()
