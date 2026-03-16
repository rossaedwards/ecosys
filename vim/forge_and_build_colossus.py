import os
import re
import shutil
import subprocess
from pathlib import Path

# --- CONFIGURATION ---
ROOT_DIR = Path(".")
BUILD_DIR = ROOT_DIR / "colossus_build"
PDF_DIR = BUILD_DIR / "final_pdfs"

VOL_1_DIR = BUILD_DIR / "Volume_I_Geodesic_Evolution"
VOL_2_DIR = BUILD_DIR / "Volume_II_Thermodynamic_Stabilization"
VOL_3_DIR = BUILD_DIR / "Volume_III_Edwards_Unified_Field"

for d in [VOL_1_DIR, VOL_2_DIR, VOL_3_DIR, PDF_DIR]:
    os.makedirs(d / "src_markdown", exist_ok=True)
    os.makedirs(d / "sections_tex", exist_ok=True)

# --- REGEX CLASSIFIERS ---
patterns = {
    "vim_theory": re.compile(r"vim_section_([a-zA-Z]+)(_[a-c])?\.md$", re.IGNORECASE),
    "hif_engine": re.compile(r"hif_(.+)\.md$", re.IGNORECASE),
    "tsl_engine": re.compile(r"tsl_(.+)\.md$", re.IGNORECASE),
    "sages_protocol": re.compile(r"SAGES_(.+)\.md$", re.IGNORECASE),
    "sages_aux": re.compile(r"AUX[-_]([a-zA-Z0-9]+)[-_]001\.md$", re.IGNORECASE),
    "appendix_canon": re.compile(r"appendix_([a-zA-Z0-9_\-∞Ω]+)\.md$", re.IGNORECASE),
    "semantic_cog": re.compile(r"(tslca|fuxyez|section)[-_](.+)\.md$", re.IGNORECASE)
}

def roman_to_int(s):
    rom_val = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}
    int_val = 0
    s = s.upper()
    try:
        for i in range(len(s)):
            if i > 0 and rom_val[s[i]] > rom_val[s[i - 1]]:
                int_val += rom_val[s[i]] - 2 * rom_val[s[i - 1]]
            else:
                int_val += rom_val[s[i]]
        return int_val
    except KeyError:
        return 999 

def process_file(source_path, target_vol_dir, new_base_name, sort_key):
    md_target = target_vol_dir / "src_markdown" / f"{new_base_name}.md"
    tex_target = target_vol_dir / "sections_tex" / f"{new_base_name}.tex"
    
    shutil.copy2(source_path, md_target)
    try:
        subprocess.run(["pandoc", str(md_target), "-o", str(tex_target)], check=True, capture_output=True)
    except Exception as e:
        print(f"⚠️ Pandoc skipped {new_base_name}: {e}")
        
    return (sort_key, f"sections_tex/{new_base_name}.tex")

print("⚡ IGNITING THE FORGE: Scanning for Colossus Artifacts...")

vol_1_files, vol_2_files, vol_3_files = [], [], []

for root, dirs, files in os.walk(ROOT_DIR):
    if any(skip in root for skip in ["colossus_build", ".git", "node_modules", "venv"]):
        continue

    for filename in files:
        if not filename.endswith(".md"): continue
        filepath = Path(root) / filename
        
        if match := patterns["vim_theory"].match(filename):
            roman, sub = match.group(1), match.group(2) or ""
            name = f"Theory_{roman.upper()}{sub.upper()}"
            vol_1_files.append(process_file(filepath, VOL_1_DIR, name, roman_to_int(roman)))
        elif match := patterns["hif_engine"].match(filename):
            vol_2_files.append(process_file(filepath, VOL_2_DIR, f"Engine_HIF_{match.group(1)}", 10))
        elif match := patterns["tsl_engine"].match(filename):
            vol_2_files.append(process_file(filepath, VOL_2_DIR, f"Lattice_TSL_{match.group(1)}", 20))
        elif match := patterns["semantic_cog"].match(filename):
            if "vim_" not in filename.lower():
                vol_2_files.append(process_file(filepath, VOL_2_DIR, f"Semantic_{match.group(2)}", 30))
        elif match := patterns["appendix_canon"].match(filename):
            id_str = match.group(1).upper()
            sort_key = 100 if id_str == 'Ω' else (101 if id_str in ['INFINITE', '∞'] else ord(id_str[0]) if id_str else 99)
            vol_3_files.append(process_file(filepath, VOL_3_DIR, f"Cosmology_Part_{id_str}", sort_key))
        elif match := patterns["sages_protocol"].match(filename) or patterns["sages_aux"].match(filename):
            vol_3_files.append(process_file(filepath, VOL_3_DIR, f"Governance_{filename.replace('.md', '')}", 200))

vol_1_files.sort(key=lambda x: x[0])
vol_2_files.sort(key=lambda x: x[0])
vol_3_files.sort(key=lambda x: x[0])

# --- PRIME OPUS MASTER TEMPLATE GENERATOR ---
def generate_latex(vol_dir, filename, vol_num, vol_title, vol_sub, abstract, quote, files_list):
    if not files_list: return None

    # Using raw strings to preserve LaTeX formatting and exactly match the user's template
    preamble = r"""%=========================================================
% PRIME OPUS MASTER TEMPLATE
% The Balance Continuum + Aurphyx Standard
%=========================================================
\documentclass[11pt,oneside]{book}

%-------------------------------
% PACKAGES
%-------------------------------
\usepackage[margin=1in]{geometry}
\usepackage{amsmath,amssymb,amsthm,physics}
\usepackage{graphicx}
\usepackage{tikz}
\usepackage{hyperref}
\usepackage{titlesec}
\usepackage{setspace}
\usepackage{tocloft}
\usepackage{lmodern}
\usepackage[T1]{fontenc}
\usepackage[utf8]{inputenc}

%-------------------------------
% HYPERREF SETUP
%-------------------------------
\hypersetup{
  colorlinks=true,
  linkcolor=cyan,
  urlcolor=magenta,
  citecolor=orange,
  pdftitle={The Balance Continuum — Prime Opus},
  pdfauthor={Ross A. Edwards (R.F. Lovezme)}
}

%-------------------------------
% SECTION FORMATTING
%-------------------------------
\titleformat{\chapter}[display]
  {\bfseries\Large}
  {\filright\MakeUppercase{\chaptername}~\thechapter}
  {1ex}
  {\titlerule\vspace{1ex}\filcenter}
  [\vspace{1ex}\titlerule]

\titleformat{\section}
  {\bfseries\large}
  {\thesection}{0.75em}{}

\titleformat{\subsection}
  {\bfseries\normalsize}
  {\thesubsection}{0.5em}{}

%-------------------------------
% CUSTOM MACROS
%-------------------------------
\newcommand{\rae}{r\AE{}}
\newcommand{\HIF}{\mathcal{H}_{\text{IF}}}
\newcommand{\Bliss}{\mathcal{B}_{\text{Bliss}}}
\newcommand{\BalanceLaw}{\mathcal{L}_{\text{Bal}}}
\newcommand{\UCP}{\mathcal{U}_{\text{coh}}}

%-------------------------------
% TITLE METADATA
%-------------------------------
\author{Ross A. Edwards (R.F. Lovezme)\\
\small Aurphyx LLC \& Aurphyx Foundation\\
\small ORCiD: 0009-0008-0539-1289}

\title{\textbf{THE BALANCE CONTINUUM}\\[4pt]
\large Prime Opus of the Aurphyx Standard}

\date{\small 2026}

%=========================================================
\begin{document}
\frontmatter
\maketitle

%-------------------------------
% SIGIL PAGE
%-------------------------------
\thispagestyle{empty}
\vspace*{2cm}
\begin{center}
\begin{tikzpicture}[scale=3, line width=0.6pt]
  % Outer Ring (Bliss Manifold)
  \draw[thick] (0,0) circle (1);
  % Hexa-Crown
  \foreach \i in {0,60,120,180,240,300} {
      \draw[thick] (0,0) -- ({0.8*cos(\i)},{0.8*sin(\i)});
      \fill ({0.8*cos(\i)},{0.8*sin(\i)}) circle (0.015);
  }
  % Inner Tetra
  \draw[thick] (0,0.45) -- (-0.39,-0.225) -- (0.39,-0.225) -- cycle;
  % Central rÆ Glyph
  \fill (0,0) circle (0.03);
\end{tikzpicture}

\vspace{1cm}
{\itshape The Edwards Sigil — Visual Invariant of the Prime Opus}
\end{center}
\clearpage

%-------------------------------
% TABLE OF CONTENTS
%-------------------------------
\tableofcontents
\clearpage

%=========================================================
\mainmatter

%-------------------------------
% VOLUME TITLE PAGE
%-------------------------------
\begin{titlepage}
\centering
{\Large\bfseries THE BALANCE CONTINUUM — VOLUME """ + vol_num + r"""\par}
\vspace{0.5cm}
{\large\bfseries """ + vol_title + r"""\par}
\vspace{0.25cm}
{\large """ + vol_sub + r"""\par}
\vfill
{\large\bfseries Author:}\\[2pt]
{\large Ross A. Edwards (R.F. Lovezme)\par}
{\normalsize Aurphyx LLC \& Aurphyx Foundation\par}
{\normalsize ORCiD: 0009-0008-0539-1289\par}
\vfill
{\bfseries Abstract}\\[6pt]
\begin{minipage}{0.9\textwidth}
\small
""" + abstract + r"""
\end{minipage}
\vfill
{\itshape
""" + quote + r"""
}
\vfill
{\normalsize Prime Opus Series — Architect’s Cut\par}
\end{titlepage}
\clearpage

\part{Volume """ + vol_num + r""": """ + vol_title + r"""}

"""

    for item in files_list:
        preamble += f"\\input{{{item[1]}}}\n"

    preamble += r"""
\appendix
\bibliographystyle{plain}
\bibliography{../references}
\end{document}
"""
    
    tex_path = vol_dir / filename
    with open(tex_path, "w", encoding="utf-8") as f:
        f.write(preamble)
    return tex_path

print("📝 Scaffolding Master LaTeX Files with Prime Opus Template...")

# Volume 1 Data
generate_latex(
    VOL_1_DIR, "Volume_I_Geodesic_Evolution.tex", 
    "I", "THE ORIGIN MANUSCRIPT", "Geodesic Evolution and Global Attractors",
    "Volume I establishes the foundational architecture of the Balance Continuum — the origin manuscript defining the rÆ manifold, the Edwards Flow, the Harmonic Integrity Field, and the Unified Coherence Principle.",
    "“Before there was structure, there was coherence.\\\\Before there was form, there was identity.\\\\Before there was creation, there was Balance.”",
    vol_1_files
)

# Volume 2 Data
generate_latex(
    VOL_2_DIR, "Volume_II_Thermodynamic_Stabilization.tex", 
    "II", "THE META-CREATIVE CYCLE", "Thermodynamic Stabilization in Vacuum Flux Engines",
    "Volume II formalizes the Meta‑Creative Cycle — the Continuum’s mechanism for absorbing newly created worlds, harmonizing emergent domains, reconciling new laws with the Balance Law, and stabilizing cosmological evolution.",
    "“Creation is not the end of the Continuum.\\\\Creation is the moment the Continuum learns to become more.”",
    vol_2_files
)

# Volume 3 Data
generate_latex(
    VOL_3_DIR, "Volume_III_Edwards_Unified_Field.tex", 
    "III", "THE UNIFIED FIELD THEORY", "Holographic Duality and Cosmological Coherence",
    "Volume III scales the architecture to the cosmological level, unifying the rÆ Alphabet, the Eightfold Cosmology, and the SAGES ethical immune system into a singular law governing the Aurphyx meta-organism.",
    "“The universe does not compute, it resonates.\\\\And it resonates with love.”",
    vol_3_files
)

# Create Global Bibliography
bib_content = """@article{edwards2026geodesic, title={The Balance Continuum: Prime Opus}, author={Edwards, Ross A.}, journal={Aurphyx Standard}, year={2026}}\n"""
with open(BUILD_DIR / "references.bib", "w", encoding="utf-8") as f: f.write(bib_content)

# --- 10-PASS PDFLATEX COMPILATION LOOP ---
def compile_pdf(tex_file_path):
    if not tex_file_path: return
    work_dir = tex_file_path.parent
    base_name = tex_file_path.stem
    
    print(f"\n🔄 Initiating 10-Pass Compilation for: {base_name}")
    
    for pass_num in range(1, 11):
        print(f"   ➤ Executing Pass {pass_num}/10...")
        subprocess.run(
            ["pdflatex", "-interaction=nonstopmode", "-halt-on-error", tex_file_path.name], 
            cwd=work_dir, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
        )
        
        if pass_num == 1:
            print(f"   📚 Resolving Bibliography (BibTeX)...")
            subprocess.run(["bibtex", base_name], cwd=work_dir, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            
    final_pdf = work_dir / f"{base_name}.pdf"
    if final_pdf.exists():
        shutil.copy2(final_pdf, PDF_DIR / f"{base_name}.pdf")
        print(f"   ✅ SUCCESS: {base_name}.pdf forged.")
    else:
        print(f"   ❌ FAILED to generate {base_name}.pdf.")

compile_pdf(VOL_1_DIR / "Volume_I_Geodesic_Evolution.tex")
compile_pdf(VOL_2_DIR / "Volume_II_Thermodynamic_Stabilization.tex")
compile_pdf(VOL_3_DIR / "Volume_III_Edwards_Unified_Field.tex")

print(f"\n👑 MISSION ACCOMPLISHED. The Prime Opus is locked and compiled in: {PDF_DIR.absolute()}")
