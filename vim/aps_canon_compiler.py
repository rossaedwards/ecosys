#!/usr/bin/env python3
"""APS Canon Compiler — read-only scan, parse, TOC, index, classify.

Default root is vim/. Writes reports next to this script. Never rewrites sources.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any, Dict, List

sys.path.insert(0, str(Path(__file__).resolve().parent))

from aps_nomenclature_lib import REPO_ROOT, VIM_DIR, iter_vim_markdown, load_map

VOLUME_MAP_JSON = VIM_DIR / "aps_volume_map.json"
SYMBOLS_MAP_JSON = VIM_DIR / "aps_symbols_map.json"
MASTER_TOC_JSON = VIM_DIR / "APS_MASTER_TOC.json"
MASTER_TOC_MD = VIM_DIR / "APS_MASTER_TOC.md"
MASTER_INDEX_JSON = VIM_DIR / "APS_MASTER_INDEX.json"
MASTER_INDEX_MD = VIM_DIR / "APS_MASTER_INDEX.md"
LOG_FILE = VIM_DIR / "APS_COMPILER_LOG.txt"

HEADING_RE = re.compile(r"^(#{1,6})\s+(.+)$", re.MULTILINE)
DISPLAY_MATH_RE = re.compile(r"\$\$(.+?)\$\$", re.DOTALL)
INLINE_MATH_RE = re.compile(r"(?<!\$)\$(?!\$)(.+?)\$")
ALT_DISPLAY_RE = re.compile(r"\\\[(.+?)\\\]", re.DOTALL)
ALT_INLINE_RE = re.compile(r"\\\((.+?)\\\)")
LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")
SECTION_REF_RE = re.compile(r"(?:see\s+)?(?:Section|Appendix)\s+[A-Z0-9∞ΩIVXLCDM\-]+", re.IGNORECASE)
FIG_RE = re.compile(r"fig_[0-9a-z_]+", re.IGNORECASE)

TERM_BANK = [
    "Balance State Vector",
    "Equilibrium Manifold",
    "Harmonic Integrity Field",
    "HIF",
    "TSLCA",
    "FTQC",
    "TVFD",
    "SAGES",
    "SUXS-IFO",
    "SIX",
    "SCX",
    "ICX",
    "R_24",
    "VIM",
    "beta",
    "β",
]


def log(msg: str) -> None:
    with LOG_FILE.open("a", encoding="utf-8") as f:
        f.write(msg + "\n")
    print(msg)


def infer_type(path: Path) -> str:
    name = path.name.lower()
    if name.startswith("vim_section_") or name == "section_c.md":
        return "section"
    if name.startswith("appendix_"):
        return "appendix"
    if name.startswith("fig_") or "figure" in name:
        return "figure_script" if path.suffix == ".py" else "figure_guide"
    if "terminolog" in name or "nomenclature" in name:
        return "terminology"
    if name.startswith("hif_") or name.startswith("tsl_"):
        return "spec"
    if path.suffix == ".py":
        return "utility"
    return "markdown"


def extract_title(text: str, path: Path) -> str:
    # skip yaml frontmatter
    body = text
    if body.startswith("---"):
        end = body.find("\n---", 3)
        if end != -1:
            body = body[end + 4 :]
    m = HEADING_RE.search(body)
    if m:
        return m.group(2).strip()
    return path.stem.replace("_", " ")


def parse_markdown(path: Path, text: str) -> Dict[str, Any]:
    headings = [{"level": len(m.group(1)), "text": m.group(2).strip()} for m in HEADING_RE.finditer(text)]
    math_blocks = [{"kind": "display", "source": m.group(0), "label": None} for m in DISPLAY_MATH_RE.finditer(text)]
    math_blocks += [{"kind": "display", "source": m.group(0), "label": None} for m in ALT_DISPLAY_RE.finditer(text)]
    inline_math = [{"source": m.group(0)} for m in INLINE_MATH_RE.finditer(text)][:80]
    inline_math += [{"source": m.group(0)} for m in ALT_INLINE_RE.finditer(text)][:40]
    terminology_refs = [t for t in TERM_BANK if t.lower() in text.lower() or t in text]
    figure_refs = sorted(set(FIG_RE.findall(text)))
    cross_refs = sorted(set(m.group(0) for m in SECTION_REF_RE.finditer(text)))[:40]
    domain_tags = []
    lower = text.lower() + " " + path.name.lower()
    for tag, needles in [
        ("FTQC", ("ftqc", "entanglement", "anyonic")),
        ("TSLCA", ("tslca", "three-squared", "activation lattice")),
        ("SAGES", ("sages", "governance")),
        ("TVFD", ("tvfd", "vacuum flux")),
        ("VIM", ("vim", "impedance", "beta", "β")),
        ("Balance Geometry", ("geodesic", "curvature", "balance tensor")),
        ("Continuum", ("continuum", "32-d", "32-dimensional")),
        ("Fuxyez", ("fuxyez", "fuxrt", "yezrt")),
        ("Standards", ("standard", "aps-okf")),
    ]:
        if any(n in lower for n in needles):
            domain_tags.append(tag)
    return {
        "path": str(path.relative_to(REPO_ROOT)),
        "ext": path.suffix.lower(),
        "title": extract_title(text, path),
        "type": infer_type(path),
        "domain_tags": domain_tags,
        "headings": headings[:40],
        "math_blocks": math_blocks[:40],
        "inline_math": inline_math[:40],
        "terminology_refs": terminology_refs,
        "figure_refs": figure_refs[:40],
        "cross_refs": cross_refs,
        "math_summary": [b["source"][:80] for b in math_blocks[:12]],
    }


def classify(record: Dict[str, Any], volume_map: Dict[str, Any], rel: Path) -> Dict[str, Any]:
    scores: Dict[str, float] = {}
    text_blob = " ".join(
        [
            record.get("title") or "",
            " ".join(record.get("domain_tags") or []),
            " ".join(record.get("terminology_refs") or []),
            record.get("path") or "",
        ]
    ).lower()
    parts = Path(record["path"]).parts
    for vol, spec in volume_map.items():
        if vol.startswith("_"):
            continue
        if not isinstance(spec, dict):
            continue
        score = 0.0
        signals = {"keywords": 0.0, "folder": 0.0, "patterns": 0.0, "manual": 0.0}
        for kw in spec.get("auto_keywords") or []:
            if str(kw).lower() in text_blob:
                signals["keywords"] += 0.15
        for pat in spec.get("auto_patterns") or []:
            if Path(record["path"]).name == pat or str(pat).lower() in record["path"].lower():
                signals["patterns"] += 0.4
        for folder in spec.get("auto_folders") or []:
            if folder in parts:
                signals["folder"] += 0.5
        manuals = spec.get("manual_files") or []
        if record["path"] in manuals or Path(record["path"]).name in manuals:
            signals["manual"] = 1.0
            score = 1.0
            scores[vol] = 1.0
            record["candidate_volume"] = vol
            record["confidence"] = 1.0
            record["signals"] = signals
            record["manual_override"] = True
            return record
        score = min(1.0, sum(signals.values()))
        signals["keywords"] = round(signals["keywords"], 3)
        scores[vol] = score
        record.setdefault("_scores", {})[vol] = {"score": round(score, 3), "signals": signals}

    if scores:
        best = max(scores.items(), key=lambda kv: kv[1])
        record["candidate_volume"] = best[0] if best[1] > 0.05 else None
        record["confidence"] = round(float(best[1]), 3)
        record["signals"] = (record.get("_scores") or {}).get(best[0], {}).get("signals", {})
        record["manual_override"] = False
    else:
        record["candidate_volume"] = None
        record["confidence"] = 0.0
        record["signals"] = {}
        record["manual_override"] = False
    record.pop("_scores", None)
    return record


def write_outputs(analyzed: List[Dict[str, Any]]) -> None:
    MASTER_TOC_JSON.write_text(json.dumps(analyzed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    MASTER_INDEX_JSON.write_text(json.dumps(analyzed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    toc_lines = ["# APS Master Table of Contents (vim)", ""]
    for item in analyzed:
        vol = item.get("candidate_volume") or "unassigned"
        toc_lines.append(
            f"- `{item['path']}` — {item.get('title')} ({item.get('type')}; {vol}; conf {item.get('confidence')})"
        )
    MASTER_TOC_MD.write_text("\n".join(toc_lines) + "\n", encoding="utf-8")

    idx_lines = ["# APS Master Content Index (vim)", ""]
    for item in analyzed:
        idx_lines.append(f"## `{item['path']}`")
        idx_lines.append("")
        idx_lines.append(f"- title: {item.get('title')}")
        idx_lines.append(f"- math blocks: {len(item.get('math_blocks') or [])}")
        idx_lines.append(f"- terms: {', '.join(item.get('terminology_refs') or []) or '(none)'}")
        idx_lines.append(f"- figures: {', '.join(item.get('figure_refs') or []) or '(none)'}")
        idx_lines.append(f"- volume: {item.get('candidate_volume')} ({item.get('confidence')})")
        idx_lines.append("")
    MASTER_INDEX_MD.write_text("\n".join(idx_lines), encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description="Read-only APS canon compiler.")
    parser.add_argument("--root", default=str(VIM_DIR), help="Scan root (default: vim/).")
    parser.add_argument("--include-codex", action="store_true")
    args = parser.parse_args()

    if LOG_FILE.exists():
        LOG_FILE.write_text("", encoding="utf-8")
    log("=== APS Canon Compiler (read-only) ===")

    volume_map = {}
    if VOLUME_MAP_JSON.exists():
        volume_map = json.loads(VOLUME_MAP_JSON.read_text(encoding="utf-8"))
        log(f"Loaded {VOLUME_MAP_JSON.name}")
    if SYMBOLS_MAP_JSON.exists():
        log(f"Loaded {SYMBOLS_MAP_JSON.name}")

    root = Path(args.root).resolve()
    nmap = load_map()
    if root == VIM_DIR.resolve():
        md_files = iter_vim_markdown(VIM_DIR, nmap, include_codex=args.include_codex)
    else:
        md_files = sorted(p for p in root.rglob("*.md") if ".git" not in p.parts)

    analyzed: List[Dict[str, Any]] = []
    for path in md_files:
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError as exc:
            log(f"skip {path}: {exc}")
            continue
        rec = parse_markdown(path, text)
        rec = classify(rec, volume_map, path)
        analyzed.append(rec)

    write_outputs(analyzed)
    log(f"Parsed {len(analyzed)} files.")
    log("Master TOC and Index written under vim/.")
    log("Compiler did not modify sources.")


if __name__ == "__main__":
    main()
