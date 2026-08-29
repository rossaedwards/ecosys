#!/usr/bin/env python3
"""Two-phase nomenclature writer for vim/*.md.

Phase A: --dry-run  -> vim/APS_NOMENCLATURE_DRY_RUN.md
Phase B: --apply    -> in-place rewrite + vim/APS_NOMENCLATURE_APPLY_LOG.txt

Never combines with OKF stamping.
"""

from __future__ import annotations

import argparse
import sys
from datetime import datetime, timezone
from pathlib import Path

# Allow `python vim/aps_nomenclature_apply.py` from repo root
sys.path.insert(0, str(Path(__file__).resolve().parent))

from aps_nomenclature_lib import (
    REPO_ROOT,
    VIM_DIR,
    apply_replacements,
    diff_hits,
    iter_vim_markdown,
    load_map,
)

DRY_RUN_MD = VIM_DIR / "APS_NOMENCLATURE_DRY_RUN.md"
APPLY_LOG = VIM_DIR / "APS_NOMENCLATURE_APPLY_LOG.txt"


def process(
    *,
    apply: bool,
    include_codex: bool,
    rewrite_historical: bool,
) -> dict:
    nmap = load_map()
    files = iter_vim_markdown(
        VIM_DIR, nmap, include_codex=include_codex, nomenclature=True
    )
    all_hits = []
    changed = []
    for path in files:
        original = path.read_text(encoding="utf-8", errors="replace")
        updated = apply_replacements(
            original,
            nmap,
            rewrite_historical_filenames=rewrite_historical,
        )
        rel = str(path.relative_to(REPO_ROOT))
        hits = diff_hits(original, updated, rel)
        if not hits:
            continue
        all_hits.extend(hits)
        changed.append(rel)
        if apply:
            path.write_text(updated, encoding="utf-8")
    return {
        "files_scanned": len(files),
        "files_changed": changed,
        "hits": all_hits,
        "apply": apply,
        "include_codex": include_codex,
        "rewrite_historical": rewrite_historical,
    }


def write_dry_run(result: dict) -> None:
    lines = [
        "# APS Nomenclature Dry Run — vim/",
        "",
        f"Scanned: {result['files_scanned']} markdown files.",
        f"Would change: {len(result['files_changed'])} files.",
        f"Line hits: {len(result['hits'])}.",
        f"include_codex: {result['include_codex']}",
        f"rewrite_historical_filenames: {result['rewrite_historical']}",
        "",
        "Protected: BlissCore, ChaosCore, BlissID, Chaos & Bliss, historical AUX-SIC/SCC/ICC.",
        "Filenames matching `*bliss*` or `*chaos*` are skipped.",
        "",
        "## Files",
        "",
    ]
    for rel in result["files_changed"]:
        lines.append(f"- `{rel}`")
    lines.extend(["", "## Line diffs (truncated to 400)", "", "| File | Line | Before | After |", "|---|---:|---|---|"])
    for hit in result["hits"][:400]:
        before = hit["before"].replace("|", "\\|")[:160]
        after = hit["after"].replace("|", "\\|")[:160]
        lines.append(f"| `{hit['path']}` | {hit['line']} | `{before}` | `{after}` |")
    if len(result["hits"]) > 400:
        lines.append("")
        lines.append(f"_Truncated; {len(result['hits']) - 400} more line hits._")
    lines.append("")
    DRY_RUN_MD.write_text("\n".join(lines), encoding="utf-8")


def write_apply_log(result: dict) -> None:
    stamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    lines = [
        f"APS nomenclature apply {stamp}",
        f"files_changed={len(result['files_changed'])} line_hits={len(result['hits'])}",
        "",
    ]
    lines.extend(result["files_changed"])
    lines.append("")
    APPLY_LOG.write_text("\n".join(lines), encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description="Apply APS nomenclature inside vim/ markdown.")
    parser.add_argument("--dry-run", action="store_true", help="Report only (default if --apply is absent).")
    parser.add_argument("--apply", action="store_true", help="Rewrite vim markdown in place.")
    parser.add_argument("--include-codex", action="store_true", help="Also process extracted_math_v32/.")
    parser.add_argument(
        "--rewrite-historical-filenames",
        action="store_true",
        help="Also rewrite AUX-SIC / AUX-SCC / AUX-ICC tokens.",
    )
    args = parser.parse_args()
    apply = bool(args.apply)
    result = process(
        apply=apply,
        include_codex=args.include_codex,
        rewrite_historical=args.rewrite_historical_filenames,
    )
    write_dry_run(result)
    if apply:
        write_apply_log(result)
        print(f"Applied {len(result['files_changed'])} files. Log: {APPLY_LOG}")
    else:
        print(f"Dry run: {len(result['files_changed'])} files would change. Report: {DRY_RUN_MD}")
    print(f"Line hits: {len(result['hits'])}")


if __name__ == "__main__":
    main()
