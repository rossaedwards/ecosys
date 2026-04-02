#!/usr/bin/env python3
"""
Apply global symbol mapping and refactor rules across the workspace.
Run from main/ directory. Order of rules is critical.
"""

import re
from pathlib import Path
import json

REPO_ROOT = Path(__file__).resolve().parent

# File extensions to process
TEXT_EXTENSIONS = {".md", ".tex", ".py", ".json", ".yaml", ".ipynb", ".txt", ".scad"}

# Skip these paths (binary, generated, or external)
SKIP_DIRS = {".git", "node_modules", "__pycache__", ".venv", "venv", ".cursor"}
SKIP_FILES = {"apply_symbol_mapping.py"}  # Don't modify self

# Patterns that indicate URL/citation context - skip standalone rAE replacement
SKIP_RAE_CONTEXT = re.compile(
    r"https?://|doi:|arxiv\.org|\\cite\{|\\bibitem|\\url\{|\\href\{",
    re.IGNORECASE
)


def should_skip_standalone_rAE(line: str) -> bool:
    """Skip standalone rAE replacement in URLs, citations, bibliography."""
    return bool(SKIP_RAE_CONTEXT.search(line))


def apply_replacements(content: str, filepath: Path) -> str:
    """Apply all replacement rules in order. Returns modified content."""
    s = content

    # Pre-rule: rÆt and rÆ-Cell before general rÆ (avoid "Balance State Vectort")
    s = s.replace("rÆt", "x_t")
    s = s.replace("rÆ-Cell", "Balance State Vector-Cell")
    s = s.replace("rÆ-CELL", "Balance State Vector-CELL")
    s = s.replace("rÆL", "x_L")
    s = s.replace("rÆ", "Balance State Vector")

    # Rule 1: CORE CONCEPT RENAMING (PROSE) - order matters for Bliss
    s = s.replace("Bliss manifold", "Equilibrium Manifold")
    s = s.replace("Bliss state", "Equilibrium state")
    s = s.replace("Bliss attractor", "Equilibrium attractor")
    s = s.replace("Bliss nullcline", "Equilibrium nullcline")
    s = s.replace("Bliss surface", "Equilibrium surface")
    # Standalone "Bliss" - use word boundary to avoid partial matches
    s = re.sub(r"\bBliss\b", "Equilibrium Manifold", s)

    # Rule 2: LATEX SYMBOL RENAMING (EXACT MATCHES)
    s = s.replace(r"\mathbf{rAE}", r"\mathbf{x}")
    s = s.replace(r"\mathcal{B}_{\text{Bliss}}", r"\mathcal{E}")

    # Rule 3: COMPONENT-LEVEL rAE_* VARIABLE MAPPING
    rAE_map = [
        (r"rAE_{\text{rHz}}", r"x_{\text{rHz}}"),  # LaTeX subscript form
        (r"rAE_{rHz}", r"x_{rHz}"),  # LaTeX subscript shorthand
        ("rAE_rHz", "x_rHz"),  # Do longer form first
        ("rAE_y", "x_y"),
        ("rAE_Cell", "x_Cell"),
        ("rAEt_flux", "x_t_flux"),  # Compound: current_rAEt_flux, optimal_rAEt_flux
        ("rAEt", "x_t"),  # Code var: rAEt FLUX, rho_rAEt, lambda_rAEL
        ("rAEL", "x_L"),
        ("rAEi", "x_i"),
        ("rAE_t", "x_t"),
        ("rAE_k", "x_k"),
        ("rAE_b", "x_b"),
        ("rAE_d", "x_d"),
        ("rAE_f", "x_f"),
        ("rAE_v", "x_v"),
        ("rAE_i", "x_i"),
        ("rAE_c", "x_c"),
        ("rAE_a", "x_a"),
        ("rAE_e", "x_e"),
        ("rAE_s", "x_s"),
        ("rAE_g", "x_g"),
        ("rAE_p", "x_p"),
        ("rAE_h", "x_h"),
        ("rAE_n", "x_n"),
    ]
    for old, new in rAE_map:
        s = s.replace(old, new)

    # Rule 4: FULL VECTOR AND GENERIC rAE REFERENCES
    s = s.replace(r"\mathbf{rAE}", r"\mathbf{x}")  # Redundant but safe
    s = s.replace("rAE_x", "x_x")
    # Standalone rAE - only when not in URL/citation context
    lines = s.split("\n")
    new_lines = []
    for line in lines:
        if should_skip_standalone_rAE(line):
            new_lines.append(line)
        else:
            # Replace rAE as standalone token (word boundary, not followed by _letter)
            new_lines.append(re.sub(r"\brAE\b", "x", line))
    s = "\n".join(new_lines)

    # Rule 5: MANIFOLD REFERENCES (already covered in 1 and 2)
    # No additional changes needed

    # Cleanup: fix erroneous "Balance State Vectort" and "Balance State VectorL"
    # (caused when rÆ was replaced before rÆt/rÆL in earlier runs)
    s = s.replace("Balance State Vectort", "x_t")
    s = s.replace("Balance State VectorL", "x_L")

    return s


def process_file(filepath: Path) -> bool:
    """Process a single file. Returns True if modified."""
    try:
        content = filepath.read_text(encoding="utf-8", errors="replace")
    except Exception as e:
        print(f"  Skip (read error): {filepath.relative_to(REPO_ROOT)}: {e}")
        return False

    new_content = apply_replacements(content, filepath)
    if new_content != content:
        filepath.write_text(new_content, encoding="utf-8")
        return True
    return False


def process_ipynb_recursive(obj, modified_ref: list) -> None:
    """Recursively process all strings in notebook JSON (sources + outputs)."""
    if isinstance(obj, dict):
        for k, v in obj.items():
            if isinstance(v, str):
                new_v = apply_replacements(v, Path("."))
                if new_v != v:
                    obj[k] = new_v
                    modified_ref[0] = True
            else:
                process_ipynb_recursive(v, modified_ref)
    elif isinstance(obj, list):
        for i, item in enumerate(obj):
            if isinstance(item, str):
                new_item = apply_replacements(item, Path("."))
                if new_item != item:
                    obj[i] = new_item
                    modified_ref[0] = True
            else:
                process_ipynb_recursive(item, modified_ref)


def process_ipynb(filepath: Path) -> bool:
    """Process Jupyter notebook - apply to source cells and outputs."""
    try:
        nb = json.loads(filepath.read_text(encoding="utf-8"))
    except Exception as e:
        print(f"  Skip (read error): {filepath.relative_to(REPO_ROOT)}: {e}")
        return False

    modified_ref = [False]
    process_ipynb_recursive(nb, modified_ref)

    if modified_ref[0]:
        filepath.write_text(json.dumps(nb, indent=1, ensure_ascii=False), encoding="utf-8")
    return modified_ref[0]


def collect_files() -> list[Path]:
    """Collect all text files to process."""
    files = []
    for path in REPO_ROOT.rglob("*"):
        if path.is_file() and path.name not in SKIP_FILES:
            if any(d in path.parts for d in SKIP_DIRS):
                continue
            if path.suffix.lower() in TEXT_EXTENSIONS:
                files.append(path)
    return files


def rename_files() -> list[tuple[Path, Path]]:
    """Rule 6: Filename replacements. Returns list of (old_path, new_path)."""
    renames = []
    for path in REPO_ROOT.rglob("*"):
        if not path.is_file():
            continue
        if any(d in path.parts for d in SKIP_DIRS):
            continue

        name = path.name
        new_name = name

        # rae_ -> balance_state_vector_
        if "rae_" in new_name:
            new_name = new_name.replace("rae_", "balance_state_vector_")
        # rAE_ -> x_
        if "rAE_" in new_name:
            new_name = new_name.replace("rAE_", "x_")
        # bliss_ -> equilibrium_
        if "bliss_" in new_name:
            new_name = new_name.replace("bliss_", "equilibrium_")

        if new_name != name:
            new_path = path.parent / new_name
            renames.append((path, new_path))
    return renames


def update_file_references(old_path: Path, new_path: Path) -> None:
    """Update references to renamed file across workspace."""
    old_name = old_path.name
    new_name = new_path.name
    rel_old = str(old_path.relative_to(REPO_ROOT))
    rel_new = str(new_path.relative_to(REPO_ROOT))

    for path in REPO_ROOT.rglob("*"):
        if not path.is_file() or path == old_path:
            continue
        if any(d in path.parts for d in SKIP_DIRS):
            continue
        if path.suffix.lower() not in TEXT_EXTENSIONS:
            continue

        try:
            content = path.read_text(encoding="utf-8", errors="replace")
        except Exception:
            continue

        new_content = content
        if old_name in new_content:
            new_content = new_content.replace(old_name, new_name)
        if rel_old in new_content:
            new_content = new_content.replace(rel_old, rel_new)

        if new_content != content:
            path.write_text(new_content, encoding="utf-8")
            print(f"  Updated refs: {path.relative_to(REPO_ROOT)}")


def main():
    print("Applying global symbol mapping...")
    print("=" * 60)

    files = collect_files()
    print(f"Processing {len(files)} files...")

    modified_count = 0
    for path in sorted(files):
        rel = path.relative_to(REPO_ROOT)
        if path.suffix.lower() == ".ipynb":
            if process_ipynb(path):
                modified_count += 1
                print(f"  Modified: {rel}")
        else:
            if process_file(path):
                modified_count += 1
                print(f"  Modified: {rel}")

    print(f"\nPhase 1: Modified {modified_count} files")

    # Phase 2: Filename replacements
    renames = rename_files()
    if renames:
        print(f"\nPhase 2: Renaming {len(renames)} files...")
        for old_path, new_path in renames:
            if new_path.exists() and new_path != old_path:
                print(f"  Skip (target exists): {old_path.name} -> {new_path.name}")
                continue
            old_path.rename(new_path)
            print(f"  Renamed: {old_path.relative_to(REPO_ROOT)} -> {new_path.relative_to(REPO_ROOT)}")
            update_file_references(old_path, new_path)
    else:
        print("\nPhase 2: No filenames to rename")

    print("\nDone.")


if __name__ == "__main__":
    main()
