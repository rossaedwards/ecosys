"""Shared APS nomenclature engine for vim/.

Read-only helpers plus apply_replacements(). Never walks outside a given root.
"""

from __future__ import annotations

import fnmatch
import json
import re
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Tuple

import yaml

VIM_DIR = Path(__file__).resolve().parent
REPO_ROOT = VIM_DIR.parent
MAP_PATH = VIM_DIR / "aps_nomenclature_map.yaml"
SYMBOLS_JSON = VIM_DIR / "aps_symbols_map.json"

SKIP_RAE_CONTEXT = re.compile(
    r"https?://|doi:|arxiv\.org|\\cite\{|\\bibitem|\\url\{|\\href\{",
    re.IGNORECASE,
)

OKF_KEYS = (
    "type",
    "title",
    "description",
    "workspaces",
    "services",
    "domains",
    "nodes",
    "cores",
    "fields",
)

ILLEGAL_OKF_KEYS = {
    "resource",
    "tags",
    "timestamp",
    "implementations",
    "Mythic Naming",
    "Claims",
    "Lore",
}


def load_map(path: Path = MAP_PATH) -> Dict[str, Any]:
    data = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValueError(f"nomenclature map is not a mapping: {path}")
    return data


def _placeholder(index: int, kind: str) -> str:
    return f"@@APS{kind}{index:04d}@@"


def mask_tokens(text: str, tokens: Iterable[str]) -> Tuple[str, Dict[str, str]]:
    mapping: Dict[str, str] = {}
    out = text
    # Longest first so BlissCore is not split by Bliss
    ordered = sorted({t for t in tokens if t}, key=len, reverse=True)
    for i, token in enumerate(ordered):
        if token not in out:
            continue
        ph = _placeholder(i, "P")
        mapping[ph] = token
        out = out.replace(token, ph)
    return out, mapping


def unmask(text: str, mapping: Dict[str, str]) -> str:
    out = text
    for ph, token in mapping.items():
        out = out.replace(ph, token)
    return out


def apply_replacements(
    content: str,
    nmap: Dict[str, Any],
    *,
    rewrite_historical_filenames: bool = False,
    skip_url_rae: bool = True,
) -> str:
    s = content
    protect = list(nmap.get("protect") or [])
    historical = list(nmap.get("historical_protect") or [])
    if not rewrite_historical_filenames:
        protect = protect + historical

    s, masked = mask_tokens(s, protect)

    for rule in nmap.get("replace") or []:
        src = rule.get("from") or ""
        dst = rule.get("to") or ""
        mode = (rule.get("mode") or "literal").lower()
        if not src:
            continue
        if mode == "regex":
            if skip_url_rae and "rAE" in src:
                lines = s.split("\n")
                new_lines = []
                cre = re.compile(src)
                for line in lines:
                    if SKIP_RAE_CONTEXT.search(line):
                        new_lines.append(line)
                    else:
                        new_lines.append(cre.sub(dst, line))
                s = "\n".join(new_lines)
            else:
                s = re.sub(src, dst, s)
        else:
            s = s.replace(src, dst)

    for rule in nmap.get("cleanup") or []:
        s = s.replace(rule.get("from") or "", rule.get("to") or "")

    return unmask(s, masked)


def filename_skipped(name: str, nmap: Dict[str, Any], *, nomenclature: bool = False) -> bool:
    lowered = name.lower()
    if name in set(nmap.get("skip_generated_names") or []):
        return True
    if nomenclature and name in set(nmap.get("skip_nomenclature_names") or []):
        return True
    for glob in nmap.get("skip_filename_globs") or []:
        if fnmatch.fnmatch(lowered, glob.lower()):
            return True
    return False


def iter_vim_markdown(
    vim_dir: Path = VIM_DIR,
    nmap: Optional[Dict[str, Any]] = None,
    *,
    include_codex: bool = False,
    nomenclature: bool = False,
) -> List[Path]:
    nmap = nmap or load_map()
    skip_dirs = set(nmap.get("skip_dirs") or [])
    if include_codex:
        skip_dirs.discard("extracted_math_v32")
    files: List[Path] = []
    for path in vim_dir.rglob("*.md"):
        rel_parts = path.relative_to(vim_dir).parts
        if any(p in skip_dirs for p in rel_parts):
            continue
        if filename_skipped(path.name, nmap, nomenclature=nomenclature):
            continue
        files.append(path)
    return sorted(files)


def diff_hits(original: str, updated: str, rel: str) -> List[Dict[str, Any]]:
    hits: List[Dict[str, Any]] = []
    if original == updated:
        return hits
    orig_lines = original.splitlines()
    new_lines = updated.splitlines()
    n = max(len(orig_lines), len(new_lines))
    for i in range(n):
        a = orig_lines[i] if i < len(orig_lines) else ""
        b = new_lines[i] if i < len(new_lines) else ""
        if a != b:
            hits.append(
                {
                    "path": rel,
                    "line": i + 1,
                    "before": a,
                    "after": b,
                }
            )
    return hits


def merge_symbols_json(nmap: Dict[str, Any], symbols_path: Path = SYMBOLS_JSON) -> Dict[str, Any]:
    """Fill replace list from aps_symbols_map.json if a pair is missing."""
    if not symbols_path.exists():
        return nmap
    data = json.loads(symbols_path.read_text(encoding="utf-8"))
    existing = {(r.get("from"), r.get("to")) for r in (nmap.get("replace") or [])}
    extra: List[Dict[str, str]] = []
    for src, dst in (data.get("prose_replacements") or {}).items():
        if src == "Bliss":
            continue  # map YAML already has protected bare-Bliss regex after phrases
        pair = (src, dst)
        if pair not in existing:
            extra.append({"from": src, "to": dst, "mode": "literal", "source": "aps_symbols_map.json"})
    for src, dst in (data.get("latex_replacements") or {}).items():
        src_u = src.replace("\\\\", "\\")
        dst_u = dst.replace("\\\\", "\\")
        pair = (src_u, dst_u)
        if pair not in existing:
            extra.append({"from": src_u, "to": dst_u, "mode": "literal", "source": "aps_symbols_map.json"})
    for src, dst in (data.get("variable_replacements") or {}).items():
        pair = (src, dst)
        if pair not in existing:
            extra.append({"from": src, "to": dst, "mode": "literal", "source": "aps_symbols_map.json"})
    if extra:
        # Insert before the regex Bliss rule so JSON literals stay first
        nmap.setdefault("replace", [])
        nmap["replace"] = extra + list(nmap["replace"])
    return nmap
