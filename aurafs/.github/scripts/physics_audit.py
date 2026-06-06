#!/usr/bin/env python3
"""
AuraFS Physics Invariant CI Auditor

Parses compliance/PHYSICS_INVARIANTS.json ci_test_directives and scans .rs files
for magic number violations (PHYS-001). Excludes matches inside comments.
Produces GitHub Actions annotations on violations.
Exit code 1 if any ERROR/CRITICAL violation found.
"""

import json
import re
import sys
from pathlib import Path


def load_directives(repo_root: Path) -> list[dict]:
    """Load ci_test_directives from PHYSICS_INVARIANTS.json."""
    json_path = repo_root / "compliance" / "PHYSICS_INVARIANTS.json"
    if not json_path.exists():
        print(f"::error file={json_path}::PHYSICS_INVARIANTS.json not found")
        sys.exit(2)

    with open(json_path, encoding="utf-8") as f:
        data = json.load(f)

    directives = data.get("ci_test_directives", {}).get("rules", [])
    # Filter to ERROR/CRITICAL with regex_pattern (source-code scanning)
    return [
        d for d in directives
        if d.get("severity") in ("ERROR", "CRITICAL") and "regex_pattern" in d
    ]


def is_comment_only(line: str) -> bool:
    """True if line is entirely whitespace or a comment."""
    stripped = line.strip()
    return not stripped or stripped.startswith("//") or stripped.startswith("*") or stripped.startswith("/*")


def strip_line_comment(line: str) -> str:
    """Remove // and everything after (naive; doesn't handle strings)."""
    # Find // not inside a string (simplified: first // wins)
    idx = line.find("//")
    if idx >= 0:
        return line[:idx]
    return line


def in_block_comment(lines: list[str], line_idx: int) -> bool:
    """True if line_idx is inside a /* ... */ block."""
    in_block = False
    for i in range(line_idx + 1):
        line = lines[i]
        if "/*" in line:
            in_block = True
        if "*/" in line:
            in_block = False
        if i == line_idx:
            return in_block
    return False


def scan_file(
    file_path: Path,
    patterns: list[tuple[str, str, str]],  # (rule_id, desc, regex)
    repo_root: Path,
) -> list[dict]:
    """
    Scan a single .rs file for magic number violations.
    Returns list of {file, line, col, rule_id, message, match}.
    """
    violations = []
    try:
        content = file_path.read_text(encoding="utf-8", errors="replace")
    except OSError as e:
        return [{"error": str(e), "file": str(file_path)}]

    lines = content.splitlines()
    rel_path = file_path.relative_to(repo_root) if file_path.is_relative_to(repo_root) else file_path

    for line_num, line in enumerate(lines, 1):
        # Skip full-line comments
        if is_comment_only(line):
            continue
        # Skip lines inside block comments
        if in_block_comment(lines, line_num - 1):
            continue

        # Strip // comment for pattern matching (check code part only)
        code_part = strip_line_comment(line)

        for rule_id, desc, pattern in patterns:
            for m in re.finditer(pattern, code_part):
                # Avoid false positives: doc strings and string literals
                if line.strip().startswith("///") or line.strip().startswith("//!"):
                    continue
                # Skip if match is inside a string literal (odd unescaped " before match)
                before = code_part[: m.start()]
                unescaped_quotes = len(re.findall(r'(?<!\\)"', before))
                if unescaped_quotes % 2 == 1:
                    continue
                violations.append({
                    "file": str(rel_path),
                    "line": line_num,
                    "col": m.start() + 1,
                    "rule_id": rule_id,
                    "message": desc,
                    "match": m.group(),
                })

    return violations


def main() -> None:
    repo_root = Path(__file__).resolve().parent.parent.parent
    if (repo_root / "aurafs").exists():
        repo_root = repo_root / "aurafs"  # monorepo layout
    elif (repo_root / "Cargo.toml").exists():
        pass
    else:
        repo_root = Path.cwd()

    directives = load_directives(repo_root)
    if not directives:
        print("No regex-based directives found. Skipping physics audit.")
        sys.exit(0)

    # Build (rule_id, description, compiled_regex) for PHYS-001 style rules
    # We strip comments before matching, so use pattern without lookbehind.
    # JSON regex may have (?<!//.*) which is variable-width and invalid in Python.
    magic_pattern = r"(5\.3[0-9]*|1\.37[0-9]*|0\.21[0-9]*|1\.585[0-9]*)"
    patterns = []
    for d in directives:
        rp = d.get("regex_pattern", magic_pattern)
        try:
            # Strip lookbehind - we exclude comments by preprocessing
            rp = re.sub(r"\(\?<![^)]*\)", "", rp).strip() or magic_pattern
            pat = re.compile(rp)
            patterns.append((d["rule_id"], d.get("description", ""), pat))
        except re.error as e:
            print(f"::error::Invalid regex in {d.get('rule_id')}: {e}")
            sys.exit(2)

    # Exclude paths per aurafs.toml [ci.physics_audit]
    exclude = {"target", "simulations", "docs", "compliance", ".git"}
    rs_files = [
        p for p in repo_root.rglob("*.rs")
        if not any(ex in p.parts for ex in exclude)
    ]

    all_violations = []
    for fp in rs_files:
        vulns = scan_file(fp, patterns, repo_root)
        if vulns and "error" in vulns[0]:
            print(f"::error file={vulns[0]['file']}::{vulns[0]['error']}")
            sys.exit(2)
        all_violations.extend(vulns)

    if not all_violations:
        print("Physics audit: No magic number violations found.")
        sys.exit(0)

    # Emit GitHub Actions annotations (::error file=X,line=Y,col=Z::message)
    for v in all_violations:
        loc = f"file={v['file']},line={v['line']},col={v['col']}"
        msg = f"{v['rule_id']}: {v['message']} (found '{v['match']}')"
        print(f"::error {loc}::{msg}")

    print(f"\nPhysics audit FAILED: {len(all_violations)} violation(s) found.")
    sys.exit(1)


if __name__ == "__main__":
    main()
