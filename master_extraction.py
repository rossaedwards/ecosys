#!/usr/bin/env python3
"""
x MATH EXTRACTION ENGINE - v3.2 (plain ASCII version)
Priority folders: ftqc -> vim -> tvfd -> tslca -> fuxyez -> sages -> standards
Then remaining files in repo root

Handles .md via pandoc -> pylatexenc
.tex direct pylatexenc
.py string/comment scan
.ipynb markdown cells

Run from inside your /main/ folder
"""

import subprocess
import re
import json
import hashlib
from pathlib import Path
from datetime import datetime
from collections import defaultdict
import tempfile
import os

try:
    from pylatexenc.latexwalker import LatexWalker, LatexMathNode, LatexEnvironmentNode
except ImportError:
    print("ERROR: missing pylatexenc  -->  pip install pylatexenc")
    exit(1)

try:
    import nbformat
except ImportError:
    nbformat = None


def _safe_print(msg: str) -> None:
    """Print with fallback for Windows cp1252 Unicode errors."""
    try:
        print(msg)
    except UnicodeEncodeError:
        print(msg.encode("ascii", errors="replace").decode("ascii"))

# ────────────────────────────────────────────────
# CONFIG
# ────────────────────────────────────────────────

REPO_ROOT = Path(__file__).resolve().parent
OUT_DIR = REPO_ROOT / "vim" / "extracted_math_v32"
OUT_DIR.mkdir(exist_ok=True, parents=True)

FOLDER_ORDER = [
    "ftqc",
    "vim",
    "tvfd",
    "tslca",
    "fuxyez",
    "sages",
    "standards",
]

FW_KEYWORDS = {
    "FTQC":      ["ftqc", "fault-tolerant", "surface code", "stabilizer", "magic state", "threshold theorem"],
    "TVFD":      ["tvfd", "vacuum field", "topological vacuum", "ryu-takayanagi", "holographic", "zero-point"],
    "TSLCA":     ["tslca", "three-squared", "lattice", "tslqca", "tfsl", "27 node", "fractal spiral"],
    "Fuxyez":    ["fuxyez", "duality kernel", "fuxrt", "yezrt", "fute", "bliss manifold"],
    "SAGES":     ["sages", "governance", "alignment", "ecology", "soul", "ethical field"],
    "Standards": ["standards", "invariant", "si units", "planck", "codata", "vacuum impedance"],
}

# ────────────────────────────────────────────────
# Pandoc helper
# ────────────────────────────────────────────────

def pandoc_md_to_latex(md_path: Path) -> str:
    cmd = [
        "pandoc",
        "-f", "markdown+tex_math_dollars+tex_math_single_backslash+raw_tex+raw_attribute+pipe_tables+simple_tables",
        "-t", "latex",
        "--wrap=none",
        str(md_path)
    ]
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=90, check=True)
        return result.stdout
    except Exception as e:
        _safe_print(f"pandoc failed: {md_path}  -->  {e}")
        return md_path.read_text(encoding="utf-8", errors="ignore")

# ────────────────────────────────────────────────
# Math extraction core
# ────────────────────────────────────────────────

def extract_math_from_latex(latex_str: str, source: str) -> list[dict]:
    equations = []
    try:
        walker = LatexWalker(latex_str, tolerant_parsing=True)
        nodes, _, _ = walker.get_latex_nodes(pos=0)
    except Exception as e:
        _safe_print(f"latexwalker parse failed: {source}  -->  {e}")
        # fallback regex
        patterns = [
            r'\\begin\{(equation|align|align\*|gather|gather\*|multline|multline\*)\}([\s\S]*?)\\end\{\1\}',
            r'\\\[([\s\S]*?)\\\]',
            r'\$\$([\s\S]*?)\$\$',
            r'\$([^\$\\n]+?)\$',
        ]
        for pat in patterns:
            for m in re.finditer(pat, latex_str, re.DOTALL):
                content = (m.group(2) if len(m.groups()) == 2 else m.group(1)).strip()
                if len(content) > 6 and re.search(r'[=+\-*/^_\\int\\sum\\partial\\nabla]', content):
                    h = hashlib.sha256(content.encode()).hexdigest()[:12]
                    equations.append({"hash": h, "raw": content, "type": "regex_fallback", "source": source})
        return equations

    def recurse(node):
        if node is None:
            return
        if isinstance(node, LatexMathNode):
            raw = node.latex_verbatim().strip()
            if len(raw) > 6 and re.search(r'[=+\-*/^_\\]', raw):
                h = hashlib.sha256(raw.encode()).hexdigest()[:12]
                equations.append({
                    "hash": h,
                    "raw": raw,
                    "type": node.displaytype or "inline",
                    "source": source
                })
        elif isinstance(node, LatexEnvironmentNode):
            env = node.environmentname
            if env in {"equation", "align", "align*", "gather", "gather*", "multline", "multline*", "bmatrix", "pmatrix", "vmatrix", "cases"}:
                raw = node.latex_verbatim().strip()
                h = hashlib.sha256(raw.encode()).hexdigest()[:12]
                equations.append({
                    "hash": h,
                    "raw": raw,
                    "type": f"env:{env}",
                    "source": source
                })
        if hasattr(node, "nodelist"):
            for child in node.nodelist or []:
                recurse(child)

    for top in nodes:
        recurse(top)

    return equations

# ────────────────────────────────────────────────
# File handlers
# ────────────────────────────────────────────────

def process_md(path: Path) -> list:
    latex = pandoc_md_to_latex(path)
    return extract_math_from_latex(latex, str(path.relative_to(REPO_ROOT)))

def process_tex(path: Path) -> list:
    content = path.read_text(encoding="utf-8", errors="ignore")
    return extract_math_from_latex(content, str(path.relative_to(REPO_ROOT)))

def process_py(path: Path) -> list:
    text = path.read_text(encoding="utf-8", errors="ignore")
    candidates = []
    for m in re.finditer(r'(?s)(r?"""|\'\'\'|\"\"\"|\'\'\')(.*?)(?=\1|$)', text):
        c = m.group(2).strip()
        if any(x in c for x in ["\\begin{", "\\[", "$$"]):
            candidates.append(c)
    for m in re.finditer(r"#.*?(?:\\begin\{|\\\[|\$\$|\\\(|\$)", text):
        c = m.group(0).lstrip("# ").strip()
        if len(c) > 10:
            candidates.append(c)
    eqs = []
    for chunk in candidates:
        eqs.extend(extract_math_from_latex(chunk, str(path.relative_to(REPO_ROOT))))
    return eqs

def process_ipynb(path: Path) -> list:
    if not nbformat:
        return []
    try:
        with open(path, encoding="utf-8") as f:
            nb = nbformat.read(f, as_version=4)
        eqs = []
        for cell in nb.cells:
            if cell.cell_type == "markdown":
                md = "".join(cell.source)
                with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False, encoding="utf-8") as tmp:
                    tmp.write(md)
                    tmp.flush()
                    latex = pandoc_md_to_latex(Path(tmp.name))
                eqs.extend(extract_math_from_latex(latex, str(path.relative_to(REPO_ROOT))))
                os.unlink(tmp.name)
        return eqs
    except Exception as e:
        _safe_print(f"ipynb failed {path}: {e}")
        return []

# ────────────────────────────────────────────────
# Framework classification
# ────────────────────────────────────────────────

def classify_framework(rel_path: str, sample_raws: str = "") -> list[str]:
    lower = (rel_path.lower() + " " + sample_raws.lower()[:3000])
    matched = []
    for fw, words in FW_KEYWORDS.items():
        if any(w.lower() in lower for w in words):
            matched.append(fw)
    return matched or ["Uncategorized"]

# ────────────────────────────────────────────────
# Main
# ────────────────────────────────────────────────

def main():
    print("x Math Extraction v3.2 (plain ASCII)")
    print("Output folder:", OUT_DIR)

    registry = {}
    by_framework = defaultdict(list)
    total_files = 0
    total_math = 0
    processed = set()

    # Priority folders first
    for folder in FOLDER_ORDER:
        p = REPO_ROOT / folder
        if not p.is_dir():
            continue
        print(f"\nScanning: {folder}/")
        for file in p.rglob("*"):
            if not file.is_file():
                continue
            total_files += 1
            ext = file.suffix.lower()
            if ext not in {".md", ".tex", ".py", ".ipynb"}:
                continue
            rel = str(file.relative_to(REPO_ROOT))
            if rel in processed:
                continue
            processed.add(rel)

            if ext == ".md":
                items = process_md(file)
            elif ext == ".tex":
                items = process_tex(file)
            elif ext == ".py":
                items = process_py(file)
            elif ext == ".ipynb":
                items = process_ipynb(file)
            else:
                continue

            sample = "".join(it["raw"] for it in items[:3])
            fws = classify_framework(rel, sample)

            for item in items:
                h = item["hash"]
                if h not in registry:
                    item["frameworks"] = fws
                    registry[h] = item
                    for fw in fws:
                        by_framework[fw].append(item)
                    total_math += 1

    print(f"\nProcessed files: {total_files}")
    print(f"Unique math items found: {total_math}")

    # Write per-framework markdown files
    for fw, items in by_framework.items():
        out_md = OUT_DIR / f"CODEX_{fw}.md"
        lines = [f"# {fw}", f"Found: {len(items)} unique items", "---"]
        for it in items:
            lines.append(f"**{it['type']}**   {it['source']}")
            lines.append("```latex")
            preview = it["raw"][:400] + ("…" if len(it["raw"]) > 400 else "")
            lines.append(preview)
            lines.append("```\n")
        out_md.write_text("\n".join(lines), encoding="utf-8")
        print(f"Wrote: {out_md}")

    # Basic summary json
    summary = {
        "generated": datetime.now().isoformat(),
        "total_unique_math": total_math,
        "framework_counts": {fw: len(lst) for fw, lst in by_framework.items()}
    }
    (OUT_DIR / "summary.json").write_text(json.dumps(summary, indent=2), encoding="utf-8")

    print("\nFinished.")

if __name__ == "__main__":
    main()
