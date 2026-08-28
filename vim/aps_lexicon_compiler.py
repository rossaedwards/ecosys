#!/usr/bin/env python3
"""Read-only lexicon harvest for vim nomenclature.

Writes vim/APS_NOMENCLATURE_HARVEST.md.
Does not rewrite aps_nomenclature_map.yaml unless --write-map is passed
(that dump is for machine refresh only; prefer the hand-typed map).
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))

from aps_nomenclature_lib import (
    MAP_PATH,
    REPO_ROOT,
    SYMBOLS_JSON,
    VIM_DIR,
    load_map,
    merge_symbols_json,
)

HARVEST_MD = VIM_DIR / "APS_NOMENCLATURE_HARVEST.md"

RETIRED_SCAN = [
    ("rÆ", "Balance State Vector / x_*"),
    ("rAE", "x / Balance State Vector"),
    ("Bliss manifold", "Equilibrium Manifold"),
    ("Bliss", "Equilibrium Manifold (physics) or protect product names"),
    ("SIC", "SIX"),
    ("SCC", "SCX"),
    ("ICC", "ICX"),
    ("USAIC", "SUXS-IFO"),
    ("Vibe-OKF", "APS-OKF"),
    ("vibe-okf", "APS-OKF"),
    ("Accessibility", "Xessability"),
]


def scan_file(path: Path) -> list[tuple[str, int, str]]:
    hits = []
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return hits
    lines = text.splitlines()
    for i, line in enumerate(lines, 1):
        for token, mapping in RETIRED_SCAN:
            if token in line:
                hits.append((token, i, mapping))
    return hits


def harvest_sources() -> dict:
    sources = [
        REPO_ROOT / ".cursorrules",
        REPO_ROOT / "aurphyx_welcome2tribe.md",
        REPO_ROOT / "welcome2tribe.md",
        REPO_ROOT / "PROJECT_CONTEXT.md",
    ]
    tslca = REPO_ROOT / "tslca"
    if tslca.is_dir():
        sources.extend(sorted(tslca.glob("*.md"))[:40])
    records = []
    for path in sources:
        if not path.exists() or not path.is_file():
            continue
        rel = str(path.relative_to(REPO_ROOT))
        for token, line, mapping in scan_file(path):
            records.append(
                {
                    "path": rel,
                    "line": line,
                    "token": token,
                    "maps_to": mapping,
                }
            )
    return {"count": len(records), "hits": records}


def refresh_map(write_map: bool) -> dict:
    nmap = load_map()
    nmap = merge_symbols_json(nmap)
    nmap.setdefault("meta", {})
    nmap["meta"]["symbols_json"] = str(SYMBOLS_JSON.relative_to(REPO_ROOT)) if SYMBOLS_JSON.exists() else None
    nmap["meta"]["welcome"] = "aurphyx_welcome2tribe.md"
    nmap["meta"]["tslca"] = "tslca/"
    if write_map:
        MAP_PATH.write_text(
            yaml.safe_dump(nmap, sort_keys=False, allow_unicode=True, width=88),
            encoding="utf-8",
        )
    return nmap


def write_harvest(data: dict) -> None:
    lines = [
        "# APS Nomenclature Harvest",
        "",
        "Read-only scan of welcome, `.cursorrules`, PROJECT_CONTEXT, and TSLCA markdown.",
        "This file is a report. It does not rewrite sources.",
        "",
        f"Hits: {data['count']}",
        "",
        "| File | Line | Token | Maps to |",
        "|---|---:|---|---|",
    ]
    for hit in data["hits"][:500]:
        token = hit["token"].replace("|", "\\|")
        maps = hit["maps_to"].replace("|", "\\|")
        lines.append(f"| `{hit['path']}` | {hit['line']} | `{token}` | {maps} |")
    if data["count"] > 500:
        lines.append("")
        lines.append(f"_Truncated; {data['count'] - 500} more hits._")
    lines.append("")
    HARVEST_MD.write_text("\n".join(lines), encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description="Harvest nomenclature sources (read-only).")
    parser.add_argument("--write-map", action="store_true", help="Dump merged map YAML (optional).")
    args = parser.parse_args()
    nmap = refresh_map(args.write_map)
    data = harvest_sources()
    write_harvest(data)
    cores = (nmap.get("tslca") or {}).get("cores")
    if args.write_map:
        print(f"Wrote {MAP_PATH.relative_to(REPO_ROOT)}")
    print(f"TSLCA cores: {cores}")
    print(f"Harvest hits: {data['count']} -> {HARVEST_MD.relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
