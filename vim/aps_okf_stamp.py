#!/usr/bin/env python3
"""Two-phase APS-OKF stamper for vim/*.md.

Phase A: --plan   -> vim/APS_OKF_STAMP_PLAN.json
Phase B: --stamp  -> prepend-only nine-key YAML. Never rewrites bodies.

Does not combine with nomenclature apply.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))

from aps_nomenclature_lib import (
    ILLEGAL_OKF_KEYS,
    OKF_KEYS,
    REPO_ROOT,
    VIM_DIR,
    iter_vim_markdown,
    load_map,
)

PLAN_PATH = VIM_DIR / "APS_OKF_STAMP_PLAN.json"
FM_RE = re.compile(r"^---\s*\n(.*?)\n---\s*\n?", re.DOTALL)
HEADING_RE = re.compile(r"^#\s+(.+)$", re.MULTILINE)

TYPE_ENUM = {
    "standard-framework-foundational",
    "theory-standard",
    "protocol-spec",
    "standard-section",
    "standard-appendix",
    "implementation-note",
    "overview",
}


def split_frontmatter(text: str) -> Tuple[Optional[Dict[str, Any]], str, bool]:
    if not text.startswith("---"):
        return None, text, False
    m = FM_RE.match(text)
    if not m:
        return None, text, False
    raw = m.group(1)
    body = text[m.end() :]
    try:
        data = yaml.safe_load(raw)
    except yaml.YAMLError:
        return None, text, True
    if not isinstance(data, dict):
        return None, text, True
    return data, body, True


def header_is_valid(data: Dict[str, Any]) -> bool:
    keys = list(data.keys())
    if keys != list(OKF_KEYS):
        # allow same keys in order even if extras missing check below
        if set(keys) != set(OKF_KEYS):
            return False
        if keys != list(OKF_KEYS):
            return False
    for k in OKF_KEYS:
        if k not in data:
            return False
    extra = set(data) - set(OKF_KEYS) - set(ILLEGAL_OKF_KEYS)
    if extra:
        return False
    if any(k in data for k in ILLEGAL_OKF_KEYS):
        return False
    t = data.get("type")
    if t not in TYPE_ENUM:
        return False
    return True


def first_heading(body: str) -> str:
    m = HEADING_RE.search(body)
    if m:
        title = m.group(1).strip()
        if not title.lower().endswith(".md"):
            return title
    for line in body.splitlines():
        s = line.strip()
        if not s or s.lower().endswith(".md"):
            continue
        if s.lower() in {"abstract", "overview"}:
            continue
        return s[:120]
    return "VIM document"


def first_paragraph(body: str) -> str:
    chunks = []
    for line in body.splitlines():
        s = line.strip()
        if s.startswith("#"):
            continue
        if s.lower().endswith(".md"):
            continue
        if s.lower().startswith("author:"):
            continue
        if not s:
            if chunks:
                break
            continue
        chunks.append(s)
        if sum(len(c) for c in chunks) > 220:
            break
    text = " ".join(chunks)
    text = re.sub(r"\s+", " ", text).strip()
    if len(text) > 280:
        text = text[:277].rstrip() + "..."
    return text or "Vacuum Impedance Matching and Theory of Balance document."


def infer_type(name: str) -> str:
    lower = name.lower()
    if lower.startswith("vim_section_") or lower == "section_c.md":
        return "standard-section"
    if lower.startswith("appendix_"):
        return "standard-appendix"
    if lower.startswith("hif_") or lower.startswith("tsl_"):
        return "protocol-spec"
    if "sumz" in lower or lower in {"readme.md", "aurphyx_readme.md", "balance_state_vector_readme.md"}:
        return "overview"
    if lower.startswith("aps_") or "compiler" in lower or "parser" in lower or "guide" in lower:
        return "implementation-note"
    if lower in {"project_context.md", "invariants.md", "physics.md"}:
        return "overview"
    return "implementation-note"


def infer_payload(path: Path, body: str) -> Dict[str, Any]:
    name = path.name
    lower = name.lower()
    text = body.lower()
    title = first_heading(body)
    description = first_paragraph(body)
    doc_type = infer_type(name)

    services: List[str] = ["Audry"]
    if "sages" in text or "governance" in text:
        services.append("SAGES")

    domains: List[str] = ["vim", "systems"]
    if any(k in text for k in ("tsl", "tslca", "lattice", "hif", "six", "scx", "icx")):
        if "cognition" not in domains:
            domains.append("cognition")

    cores: List[str] = ["SCX"]
    nodes: List[str] = ["SCX⊗SCX"]
    if lower.startswith("hif_") or lower.startswith("tsl_") or "three-squared" in text:
        cores = ["SIX", "SCX", "ICX"]
        nodes = ["SCX⊗SCX", "SIX⊗SCX", "ICX⊗SCX"]

    fields: List[str] = ["balance-field"]
    if "impedance" in text or "vim" in lower:
        fields.append("vacuum-impedance")
    if "hif" in text or "harmonic integrity" in text:
        fields.append("harmonic-integrity-field")
    # de-dupe preserve order
    fields = list(dict.fromkeys(fields))
    domains = list(dict.fromkeys(domains))
    services = list(dict.fromkeys(services))

    payload = {
        "type": doc_type,
        "title": title,
        "description": description,
        "workspaces": "rossaedwards/ecosys, aurphyx/ecosys",
        "services": services,
        "domains": domains,
        "nodes": nodes,
        "cores": cores,
        "fields": fields,
    }
    return payload


class IndentDumper(yaml.SafeDumper):
    def increase_indent(self, flow=False, indentless=False):
        return super().increase_indent(flow, False)


def dump_header(data: Dict[str, Any]) -> str:
    ordered = {k: data[k] for k in OKF_KEYS}
    dumped = yaml.dump(
        ordered,
        Dumper=IndentDumper,
        sort_keys=False,
        allow_unicode=True,
        width=88,
        default_flow_style=False,
    )
    return f"---\n{dumped}---\n\n"


KEEP_HAND_HEADERS = {"PROJECT_CONTEXT.md", "SUMZ-SUGGZ.md"}


def plan_file(path: Path, *, force: bool = False) -> Dict[str, Any]:
    text = path.read_text(encoding="utf-8", errors="replace")
    existing, body, had_fm = split_frontmatter(text)
    rel = str(path.relative_to(REPO_ROOT))
    keep = path.name in KEEP_HAND_HEADERS
    if existing and header_is_valid(existing) and (keep or not force):
        return {
            "path": rel,
            "action": "skip_valid",
            "header": {k: existing.get(k) for k in OKF_KEYS},
        }
    action = "repair" if had_fm else "prepend"
    source_body = body if had_fm else text
    header = infer_payload(path, source_body)
    if existing and not force:
        for key in ("title", "description", "type"):
            val = existing.get(key)
            if isinstance(val, str) and val.strip() and (key != "type" or val in TYPE_ENUM):
                header[key] = val.strip()
    return {
        "path": rel,
        "action": action,
        "header": header,
    }


def write_plan(entries: List[Dict[str, Any]]) -> None:
    PLAN_PATH.write_text(json.dumps({"entries": entries}, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def stamp(entries: List[Dict[str, Any]]) -> int:
    n = 0
    for entry in entries:
        if entry["action"] == "skip_valid":
            continue
        path = REPO_ROOT / entry["path"]
        text = path.read_text(encoding="utf-8", errors="replace")
        existing, body, had_fm = split_frontmatter(text)
        source_body = body if had_fm else text
        # Strip a leading extra blank from body; header dump already ends with newline
        source_body = source_body.lstrip("\n")
        new_text = dump_header(entry["header"]) + source_body
        if not source_body.endswith("\n"):
            new_text += "\n"
        path.write_text(new_text, encoding="utf-8")
        n += 1
    return n


def main() -> None:
    parser = argparse.ArgumentParser(description="APS-OKF prepend-only stamper for vim markdown.")
    parser.add_argument("--plan", action="store_true", help="Write stamp plan JSON (default if --stamp absent).")
    parser.add_argument("--stamp", action="store_true", help="Prepend/repair headers from the plan.")
    parser.add_argument("--force", action="store_true", help="Rewrite headers even when a valid 9-key block exists.")
    parser.add_argument("--include-codex", action="store_true")
    args = parser.parse_args()

    nmap = load_map()
    files = iter_vim_markdown(VIM_DIR, nmap, include_codex=args.include_codex)
    entries = [plan_file(p, force=args.force) for p in files]
    write_plan(entries)
    counts = {}
    for e in entries:
        counts[e["action"]] = counts.get(e["action"], 0) + 1
    print(f"Plan written: {PLAN_PATH} ({len(entries)} files) {counts}")
    if args.stamp:
        n = stamp(entries)
        print(f"Stamped/repaired {n} files.")
    else:
        print("Plan only. Re-run with --stamp to prepend headers.")


if __name__ == "__main__":
    main()
