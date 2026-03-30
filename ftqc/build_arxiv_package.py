#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
Build arXiv submission package for FTQC manuscript.
Fixes LaTeX issues, runs full build, creates submission package and .tar.gz archive.
"""

import re
import shutil
import subprocess
import sys
import tarfile
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
ARXIV_DIR = SCRIPT_DIR / "arxiv_submission"
EXCLUDE_EXTENSIONS = {".aux", ".log", ".out", ".pdf", ".blg", ".synctex.gz", ".fls", ".fdb_latexmk"}


def log(msg: str, level: str = "info") -> None:
    """Print formatted message."""
    prefix = {"info": "[*]", "ok": "[OK]", "warn": "[WARN]", "err": "[ERROR]"}.get(level, "[*]")
    print(f"{prefix} {msg}")


def fix_latex_issues() -> bool:
    """Fix critical LaTeX issues in main.tex and section_*.tex files."""
    log("Fixing LaTeX issues...")
    main_file = SCRIPT_DIR / "main.tex"
    if not main_file.exists():
        log("main.tex not found", "err")
        return False

    # Fix main.tex
    with open(main_file, "r", encoding="utf-8") as f:
        content = f.read()

    # Rename \order -> \OrderOp, \tr -> \TraceOp in definitions
    content = re.sub(r"\\newcommand\{\\order\}", r"\\newcommand{\\OrderOp}", content)
    content = re.sub(r"\\newcommand\{\\tr\}", r"\\newcommand{\\TraceOp}", content)

    # Add amsthm and theorem environments if not present
    if "amsthm" not in content:
        doc_start = content.find(r"\begin{document}")
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
        log("  Added amsthm package and theorem environments")

    # Add graphicspath if not present
    if "graphicspath" not in content:
        doc_start = content.find(r"\begin{document}")
        if doc_start != -1:
            graphics_line = r"\graphicspath{{figures/}{./}}" + "\n"
            content = content[:doc_start] + graphics_line + content[doc_start:]
        log("  Added \\graphicspath{{figures/}{./}}")

    with open(main_file, "w", encoding="utf-8") as f:
        f.write(content)
    log("  Fixed main.tex", "ok")

    # Fix section_*.tex files
    section_files = list(SCRIPT_DIR.glob("section_*.tex"))
    for tex_file in section_files:
        with open(tex_file, "r", encoding="utf-8") as f:
            content = f.read()
        changed = False
        if r"\order" in content:
            content = re.sub(r"\\order\b", r"\\OrderOp", content)
            changed = True
        if r"\tr" in content:
            content = re.sub(r"\\tr\b", r"\\TraceOp", content)
            changed = True
        if changed:
            with open(tex_file, "w", encoding="utf-8") as f:
                f.write(content)
            log(f"  Fixed {tex_file.name}", "ok")

    return True


def run_latex_build() -> bool:
    """Run full LaTeX build sequence: pdflatex -> bibtex -> pdflatex -> pdflatex."""
    log("Running LaTeX build sequence...")
    commands = [
        ["pdflatex", "-interaction=nonstopmode", "main.tex"],
        ["bibtex", "main"],
        ["pdflatex", "-interaction=nonstopmode", "main.tex"],
        ["pdflatex", "-interaction=nonstopmode", "main.tex"],
    ]
    build_ok = True
    for i, cmd in enumerate(commands, 1):
        log(f"  Step {i}/4: {' '.join(cmd)}")
        result = subprocess.run(cmd, cwd=SCRIPT_DIR, capture_output=True, text=True)
        if result.returncode != 0:
            log(f"  {cmd[0]} returned code {result.returncode}", "warn")
            if result.stderr:
                for line in result.stderr.strip().split("\n")[-5:]:
                    print(f"    {line}")
            build_ok = False
            # Continue running remaining steps (bibtex may still produce .bbl)
    if build_ok:
        log("Build complete", "ok")
    else:
        log("Build completed with errors. Check main.log for details.", "warn")
    return build_ok


def get_bibliography_name() -> str:
    """Extract bibliography name from main.tex."""
    main_file = SCRIPT_DIR / "main.tex"
    with open(main_file, "r", encoding="utf-8") as f:
        content = f.read()
    match = re.search(r"\\bibliography\{([^}]+)\}", content)
    return match.group(1).strip() if match else "main"


def create_arxiv_package() -> tuple[list[Path], bool]:
    """Create arxiv_submission/ folder with required files."""
    log("Creating arXiv submission package...")

    if ARXIV_DIR.exists():
        shutil.rmtree(ARXIV_DIR)
    ARXIV_DIR.mkdir(parents=True)

    copied: list[Path] = []

    # Copy main.tex
    main_tex = SCRIPT_DIR / "main.tex"
    if main_tex.exists():
        shutil.copy2(main_tex, ARXIV_DIR / "main.tex")
        copied.append(Path("main.tex"))

    # Copy section_*.tex files
    for sec in sorted(SCRIPT_DIR.glob("section_*.tex")):
        shutil.copy2(sec, ARXIV_DIR / sec.name)
        copied.append(Path(sec.name))

    # Copy bibliography
    bib_name = get_bibliography_name()
    bib_file = SCRIPT_DIR / f"{bib_name}.bib"
    if bib_file.exists():
        shutil.copy2(bib_file, ARXIV_DIR / f"{bib_name}.bib")
        copied.append(Path(f"{bib_name}.bib"))
    else:
        log(f"  Bibliography file {bib_name}.bib not found", "warn")

    # Copy main.bbl
    bbl_file = SCRIPT_DIR / "main.bbl"
    if bbl_file.exists():
        shutil.copy2(bbl_file, ARXIV_DIR / "main.bbl")
        copied.append(Path("main.bbl"))
    else:
        log("  main.bbl not found (run build first)", "warn")

    # Copy figures
    figures_src = SCRIPT_DIR / "figures"
    if figures_src.exists() and figures_src.is_dir():
        figures_dst = ARXIV_DIR / "figures"
        figures_dst.mkdir(parents=True)
        for fig in figures_src.iterdir():
            if fig.is_file():
                shutil.copy2(fig, figures_dst / fig.name)
                copied.append(Path("figures") / fig.name)
        if list(figures_dst.iterdir()):
            log(f"  Copied {len(list(figures_dst.iterdir()))} figure(s)", "ok")
    else:
        log("  No figures/ directory found (creating empty for structure)", "info")
        (ARXIV_DIR / "figures").mkdir(exist_ok=True)

    # Create README.txt
    readme = ARXIV_DIR / "README.txt"
    readme_content = """arXiv Submission - Fractal-Enhanced Topological Quantum Computing
=============================================================

Build Instructions
------------------
1. pdflatex -interaction=nonstopmode main.tex
2. bibtex main
3. pdflatex -interaction=nonstopmode main.tex
4. pdflatex -interaction=nonstopmode main.tex

Required: pdflatex, bibtex (or equivalent)

Output: main.pdf
"""
    readme.write_text(readme_content, encoding="utf-8")
    copied.append(Path("README.txt"))

    return copied, True


def create_tar_gz() -> Path | None:
    """Create arxiv_submission.tar.gz archive."""
    log("Creating arxiv_submission.tar.gz...")
    archive_path = SCRIPT_DIR / "arxiv_submission.tar.gz"
    try:
        with tarfile.open(archive_path, "w:gz") as tar:
            tar.add(ARXIV_DIR, arcname="arxiv_submission")
        log(f"  Created {archive_path.name}", "ok")
        return archive_path
    except Exception as e:
        log(f"Failed to create archive: {e}", "err")
        return None


def get_dir_size(path: Path) -> int:
    """Get total size of directory in bytes."""
    total = 0
    for p in path.rglob("*"):
        if p.is_file():
            total += p.stat().st_size
    return total


def format_size(size_bytes: int) -> str:
    """Format size for display."""
    for unit in ("B", "KB", "MB"):
        if size_bytes < 1024:
            return f"{size_bytes:.1f} {unit}"
        size_bytes /= 1024
    return f"{size_bytes:.1f} GB"


def main() -> int:
    print("=" * 60)
    print("arXiv Package Builder - FTQC Manuscript")
    print("=" * 60)

    # Step 1: Fix LaTeX issues
    if not fix_latex_issues():
        return 1

    # Step 2: Run LaTeX build
    build_ok = run_latex_build()

    # Step 3: Create arXiv package (proceed even if build had errors)
    copied, _ = create_arxiv_package()

    # Step 4: Create .tar.gz
    archive_path = create_tar_gz()
    if not archive_path:
        return 1

    # Step 5: Print summary
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print(f"Build status:        {'SUCCESS' if build_ok else 'COMPLETED WITH ERRORS'}")
    print(f"Files in package:    {len(copied)}")
    for f in sorted(copied):
        print(f"  - {f}")
    pkg_size = get_dir_size(ARXIV_DIR)
    print(f"Package size:        {format_size(pkg_size)}")
    archive_size = archive_path.stat().st_size
    print(f"Archive size:        {format_size(archive_size)}")
    print(f"Archive location:    {archive_path.resolve()}")
    print("=" * 60)

    return 0


if __name__ == "__main__":
    sys.exit(main())
