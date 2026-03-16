import os
import re
from pathlib import Path

# Paths relative to script location (works regardless of cwd)
SCRIPT_DIR = Path(__file__).resolve().parent
VIM_DIR = SCRIPT_DIR
OUTPUT_DIR = SCRIPT_DIR / "tex_output"

OUTPUT_DIR.mkdir(exist_ok=True)


def safe_print(msg):
    """Print with fallback for Windows cp1252 Unicode issues."""
    try:
        print(msg)
    except UnicodeEncodeError:
        print(msg.encode("ascii", "replace").decode())

ROMAN_PATTERN = re.compile(r"^vim_section_([ivxlcdm]+)(.*)\.md$", re.IGNORECASE)
APPENDIX_PATTERN = re.compile(r"^appendix_([a-z0-9]+)\.md$", re.IGNORECASE)

SPECIAL_APPENDICES = {
    "omega": "Ω",
    "infinity": "∞",
    "infinite": "∞"
}

ROMAN_VALUES = {
    'I': 1, 'V': 5, 'X': 10,
    'L': 50, 'C': 100, 'D': 500, 'M': 1000
}

def roman_to_int(s):
    s = s.upper()
    total = 0
    prev = 0
    for ch in reversed(s):
        val = ROMAN_VALUES.get(ch, 0)
        if val < prev:
            total -= val
        else:
            total += val
        prev = val
    return total

def latex_header(title, subtitle):
    return f"""
\\documentclass[11pt,oneside]{{book}}

\\usepackage[margin=1in]{{geometry}}
\\usepackage{{amsmath,amssymb,amsthm}}
\\usepackage{{graphicx}}
\\usepackage{{tikz}}
\\usepackage{{hyperref}}
\\usepackage{{titlesec}}
\\usepackage{{setspace}}
\\usepackage{{tocloft}}
\\usepackage{{lmodern}}
\\usepackage[T1]{{fontenc}}
\\usepackage[utf8]{{inputenc}}

\\hypersetup{{
  colorlinks=true,
  linkcolor=cyan,
  urlcolor=magenta,
  citecolor=orange,
  pdftitle={{{title}}},
  pdfauthor={{Ross A. Edwards (R.F. Lovezme)}}
}}

\\author{{Ross A. Edwards (R.F. Lovezme)\\\\
\\small Aurphyx LLC \\& Aurphyx Foundation\\\\
\\small ORCiD: 0009-0008-0539-1289}}

\\title{{\\textbf{{{title}}}\\\\[4pt]
\\large {subtitle}}}

\\date{{\\small 2026}}

\\begin{{document}}
\\frontmatter
\\maketitle
\\tableofcontents
\\clearpage
\\mainmatter
"""

def latex_footer():
    return "\n\\end{document}\n"  # Regular string: {document} is literal


def appendix_sort_key(item):
    """Sort appendices: A..I, ∞, J..O, Ω, P..Z (rÆ Alphabet order)."""
    label, fname = item[0], item[1]
    # Match by filename for robustness (handles Unicode normalization)
    fname_lower = fname.lower()
    if "infinite" in fname_lower or "infinity" in fname_lower:
        return (0, 9.5)   # ∞ between I (9) and J (10)
    if "omega" in fname_lower:
        return (0, 15.5)  # Ω between O (15) and P (16)
    if len(label) == 1 and label.isalpha() and ord(label) < 128:
        return (0, ord(label.upper()) - ord("A") + 1)
    return (1, label)

def main():
    sections = []
    appendices = []

    for file in VIM_DIR.iterdir():
        name = file.name

        m = ROMAN_PATTERN.match(name)
        if m:
            roman = m.group(1)
            suffix = m.group(2)
            index = roman_to_int(roman)
            sections.append((index, roman.upper(), suffix, name))
            continue

        a = APPENDIX_PATTERN.match(name)
        if a:
            key = a.group(1).lower()
            label = SPECIAL_APPENDICES.get(key, key.upper())
            appendices.append((label, name))
            continue

    sections.sort(key=lambda x: x[0])
    appendices.sort(key=appendix_sort_key)

    vol1 = []
    vol2 = []
    vol3 = []

    for idx, roman, suffix, fname in sections:
        if idx <= 70:
            vol1.append(fname)
        elif idx <= 99:
            vol2.append(fname)
        else:
            vol3.append(fname)

    for label, fname in appendices:
        vol3.append(fname)

    v1_path = OUTPUT_DIR / "Volume_I_Geodesic_Evolution.tex"
    v2_path = OUTPUT_DIR / "Volume_II_Thermodynamic_Stabilization.tex"
    v3_path = OUTPUT_DIR / "Volume_III_Edwards_Unified_Field.tex"

    with open(v1_path, "w", encoding="utf-8") as f:
        f.write(latex_header("THE BALANCE CONTINUUM — VOLUME I", "Geodesic Evolution & Theory of Balance"))
        for fname in vol1:
            f.write(f"\\input{{../{fname}}}\n")
        f.write(latex_footer())

    with open(v2_path, "w", encoding="utf-8") as f:
        f.write(latex_header("THE BALANCE CONTINUUM — VOLUME II", "Thermodynamic Stabilization & VIM Physics"))
        for fname in vol2:
            f.write(f"\\input{{../{fname}}}\n")
        f.write(latex_footer())

    with open(v3_path, "w", encoding="utf-8") as f:
        f.write(latex_header("THE BALANCE CONTINUUM — VOLUME III", "Edwards Unified Field & rÆ Cosmology"))
        for fname in vol3:
            f.write(f"\\input{{../{fname}}}\n")
        f.write(latex_footer())

    safe_print("\n✨ Volume extraction complete.")
    safe_print("Generated:")
    safe_print(f"  - {v1_path}")
    safe_print(f"  - {v2_path}")
    safe_print(f"  - {v3_path}\n")

if __name__ == "__main__":
    main()
    