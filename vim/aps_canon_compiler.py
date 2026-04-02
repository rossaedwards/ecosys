import os
import json
from pathlib import Path
from typing import Dict, Any, List

REPO_ROOT = Path(__file__).resolve().parent

MASTER_TOC_JSON = REPO_ROOT / "APS_MASTER_TOC.json"
MASTER_TOC_MD = REPO_ROOT / "APS_MASTER_TOC.md"
MASTER_INDEX_JSON = REPO_ROOT / "APS_MASTER_INDEX.json"
MASTER_INDEX_MD = REPO_ROOT / "APS_MASTER_INDEX.md"

VOLUME_MAP_JSON = REPO_ROOT / "aps_volume_map.json"
SYMBOLS_MAP_JSON = REPO_ROOT / "aps_symbols_map.json"

LOG_FILE = REPO_ROOT / "APS_COMPILER_LOG.txt"


def log(msg: str) -> None:
    with LOG_FILE.open("a", encoding="utf-8") as f:
        f.write(msg + "\n")
    print(msg)


def scan_repo() -> List[Dict[str, Any]]:
    """Scan the repo and collect basic file metadata."""
    entries = []
    for root, dirs, files in os.walk(REPO_ROOT):
        root_path = Path(root)
        for name in files:
            path = root_path / name
            rel = path.relative_to(REPO_ROOT)
            ext = path.suffix.lower()
            if ext in {".md", ".tex", ".py", ".ipynb", ".json", ".txt", ".svg"}:
                entries.append({
                    "path": str(rel),
                    "ext": ext,
                    "size": path.stat().st_size,
                    "mtime": path.stat().st_mtime,
                })
    log(f"Scanned {len(entries)} files.")
    return entries


def analyze_file(entry: Dict[str, Any]) -> Dict[str, Any]:
    """Placeholder: analyze a single file (structure, math, terminology)."""
    # TODO: Implement real parsing per extension.
    # For now, just return a stub with the path and extension.
    return {
        "path": entry["path"],
        "ext": entry["ext"],
        "title": None,
        "type": None,
        "domain_tags": [],
        "math_summary": [],
        "terminology": [],
        "candidate_volume": None,
        "confidence": 0.0,
    }


def build_master_toc_and_index(entries: List[Dict[str, Any]]) -> None:
    """Build APS_MASTER_TOC and APS_MASTER_INDEX from scanned files."""
    analyzed = [analyze_file(e) for e in entries]

    # For now, TOC and Index are the same stub; later they diverge.
    toc = analyzed
    index = analyzed

    MASTER_TOC_JSON.write_text(json.dumps(toc, indent=2), encoding="utf-8")
    MASTER_INDEX_JSON.write_text(json.dumps(index, indent=2), encoding="utf-8")

    # Simple Markdown views
    with MASTER_TOC_MD.open("w", encoding="utf-8") as f:
        f.write("# APS Master Table of Contents (Stub)\n\n")
        for item in toc:
            f.write(f"- `{item['path']}` (ext: {item['ext']})\n")

    with MASTER_INDEX_MD.open("w", encoding="utf-8") as f:
        f.write("# APS Master Content Index (Stub)\n\n")
        for item in index:
            f.write(f"- `{item['path']}` (ext: {item['ext']})\n")

    log("Master TOC and Index written.")


def load_or_init_volume_map() -> Dict[str, Any]:
    if VOLUME_MAP_JSON.exists():
        data = json.loads(VOLUME_MAP_JSON.read_text(encoding="utf-8"))
        log("Loaded existing aps_volume_map.json.")
        return data
    else:
        log("aps_volume_map.json not found. Please create it before running classification.")
        return {}


def load_or_init_symbols_map() -> Dict[str, Any]:
    if SYMBOLS_MAP_JSON.exists():
        data = json.loads(SYMBOLS_MAP_JSON.read_text(encoding="utf-8"))
        log("Loaded existing aps_symbols_map.json.")
        return data
    else:
        log("aps_symbols_map.json not found. Please create it before running symbol normalization.")
        return {}


def main():
    log("=== APS Canon Compiler: Phase 1 (Scan + TOC + Index) ===")
    entries = scan_repo()
    build_master_toc_and_index(entries)
    load_or_init_volume_map()
    load_or_init_symbols_map()
    log("Phase 1 complete. Next phases: classification, MD→TeX, volume build, FTQC arXiv prep.")


if __name__ == "__main__":
    main()
