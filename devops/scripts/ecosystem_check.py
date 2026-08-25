#!/usr/bin/env python3
"""
ecosystem_check.py — the "does everything still line up" tool for the
Aurphyx ecosystem repo.

Reads devops/config/ecosystem_manifest.yaml (the declared contract for what
"in sync" means) and checks the real repo against it. Nothing about *what*
is required lives in this file — that's all in the YAML. If you need to
change what's checked, edit the manifest, not this script.

Usage (run from anywhere; it finds the repo root itself):
    python devops/scripts/ecosystem_check.py validate
    python devops/scripts/ecosystem_check.py validate --strict
    python devops/scripts/ecosystem_check.py check-locks
    python devops/scripts/ecosystem_check.py regen-tree
    python devops/scripts/ecosystem_check.py all            # validate + check-locks (what CI runs)

Exit codes:
    0  -> all good (or only warnings, and --strict was not passed)
    1  -> at least one WARNING, and --strict was passed
    2  -> at least one ERROR (missing required file, malformed lock, etc.)
          — this always fails, strict or not.

Requires: PyYAML  (pip install pyyaml  -- also see devops/requirements.txt)
"""

from __future__ import annotations

import argparse
import datetime
import json
import subprocess
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    print(
        "ERROR: PyYAML is not installed. Run:\n"
        "    pip install -r devops/requirements.txt\n"
        "or:\n"
        "    pip install pyyaml",
        file=sys.stderr,
    )
    sys.exit(2)

IN_CI = bool(__import__("os").environ.get("GITHUB_ACTIONS"))

WARNINGS: list[str] = []
ERRORS: list[str] = []


def warn(msg: str) -> None:
    WARNINGS.append(msg)
    if IN_CI:
        print(f"::warning::{msg}")
    else:
        print(f"[WARN]  {msg}")


def error(msg: str) -> None:
    ERRORS.append(msg)
    if IN_CI:
        print(f"::error::{msg}")
    else:
        print(f"[ERROR] {msg}")


def ok(msg: str) -> None:
    print(f"[ OK ]  {msg}")


# ─────────────────────────────────────────────────────────────────────────
# Repo / manifest discovery
# ─────────────────────────────────────────────────────────────────────────


def find_repo_root(start: Path) -> Path:
    """Walk upward from `start` until a directory containing .git is found."""
    cur = start.resolve()
    for _ in range(20):
        if (cur / ".git").exists():
            return cur
        if cur.parent == cur:
            break
        cur = cur.parent
    # Fall back to two levels above this script (devops/scripts/ -> repo root)
    return start.resolve().parents[1]


THIS_FILE = Path(__file__).resolve()
REPO_ROOT = find_repo_root(THIS_FILE.parent)
MANIFEST_PATH = THIS_FILE.parent.parent / "config" / "ecosystem_manifest.yaml"


def load_manifest() -> dict:
    if not MANIFEST_PATH.exists():
        error(f"Manifest not found at {MANIFEST_PATH}")
        sys.exit(2)
    with open(MANIFEST_PATH, encoding="utf-8") as f:
        return yaml.safe_load(f)


# ─────────────────────────────────────────────────────────────────────────
# git helpers
# ─────────────────────────────────────────────────────────────────────────


def _git(*args: str) -> str:
    try:
        out = subprocess.run(
            ["git", *args],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            timeout=30,
        )
        return out.stdout.strip()
    except Exception:
        return ""


def changed_files() -> set[str]:
    """
    Best-effort set of changed files (repo-relative, forward slashes),
    covering: uncommitted working-tree changes, staged changes, and (in CI)
    the diff against the PR base / previous commit. Used to check
    change_groups. Never raises — an empty git history just means an empty
    set and change_group checks become a no-op (nothing to warn about yet).
    """
    files: set[str] = set()

    for args in (["diff", "--name-only"], ["diff", "--cached", "--name-only"]):
        out = _git(*args)
        if out:
            files.update(out.splitlines())

    import os

    base_ref = os.environ.get("GITHUB_BASE_REF")
    if base_ref:
        out = _git("diff", "--name-only", f"origin/{base_ref}...HEAD")
        if out:
            files.update(out.splitlines())
    else:
        out = _git("diff", "--name-only", "HEAD~1..HEAD")
        if out:
            files.update(out.splitlines())

    return {f.replace("\\", "/") for f in files if f}


# ─────────────────────────────────────────────────────────────────────────
# validate
# ─────────────────────────────────────────────────────────────────────────


def _check_required_files(base_path: Path, label: str, required: list[str]) -> None:
    for rel in required:
        p = base_path / rel
        if p.exists():
            ok(f"{label}: {rel}")
        else:
            error(f"{label}: required file missing -> {rel}")


def _check_change_groups(base_path: Path, label: str, groups: list[dict], changed: set[str]) -> None:
    for group in groups or []:
        name = group.get("name", "unnamed-group")
        group_files = group.get("files", [])
        reason = group.get("reason", "").strip()

        # Only look at group members that actually exist on disk (a group
        # can reference a file that's aspirational, e.g. the n8n workflow).
        existing = [f for f in group_files if (base_path / f).exists()]
        if len(existing) < 2:
            continue

        rel_prefix = "" if label == "canon" else f"{base_path.relative_to(REPO_ROOT)}/"
        touched = {f for f in existing if f"{rel_prefix}{f}".replace("\\", "/") in changed}

        if touched and touched != set(existing):
            missing = set(existing) - touched
            warn(
                f"{label}/{name}: changed {sorted(touched)} but not "
                f"{sorted(missing)} in the same set of changes. {reason}"
            )


def cmd_validate(strict: bool) -> int:
    manifest = load_manifest()
    changed = changed_files()

    canon = manifest.get("canon", {})
    canon_path = REPO_ROOT / canon.get("path", ".")
    print(f"\n=== canon ({canon_path}) ===")
    _check_required_files(canon_path, "canon", canon.get("required_files", []))
    _check_change_groups(canon_path, "canon", canon.get("change_groups", []), changed)

    for project in manifest.get("projects", []):
        proj_path = REPO_ROOT / project["path"]
        label = project["name"]
        print(f"\n=== project: {label} ({proj_path}) ===")
        if not proj_path.exists():
            error(f"{label}: project path does not exist -> {proj_path}")
            continue
        _check_required_files(proj_path, label, project.get("required_files", []))
        _check_change_groups(proj_path, label, project.get("change_groups", []), changed)

    return _summarize(strict)


# ─────────────────────────────────────────────────────────────────────────
# check-locks  (enforces the convention in root .agent-lock.md)
# ─────────────────────────────────────────────────────────────────────────


def cmd_check_locks(strict: bool) -> int:
    locks_dir = REPO_ROOT / "locks"
    print(f"\n=== agent locks ({locks_dir}) ===")
    if not locks_dir.exists():
        ok("no locks/ directory yet — nothing to check")
        return _summarize(strict)

    now = datetime.datetime.now(datetime.timezone.utc)
    lock_files = sorted(locks_dir.glob("*.lock.json"))
    if not lock_files:
        ok("locks/ exists but is empty")

    for lf in lock_files:
        try:
            data = json.loads(lf.read_text(encoding="utf-8"))
        except Exception as e:
            error(f"{lf.name}: malformed lock file ({e})")
            continue

        expires_at = data.get("expires_at")
        path = data.get("path", "?")
        agent = data.get("agent", "?")
        if not expires_at:
            warn(f"{lf.name}: no expires_at set (path={path}, agent={agent})")
            continue

        try:
            exp = datetime.datetime.fromisoformat(expires_at.replace("Z", "+00:00"))
        except ValueError:
            error(f"{lf.name}: expires_at is not valid ISO-8601: {expires_at!r}")
            continue

        if exp < now:
            age = now - exp
            warn(
                f"{lf.name}: STALE lock on {path!r} held by {agent!r}, "
                f"expired {age} ago. Per .agent-lock.md this may be broken, "
                f"but note it in HANDOFF.md when you do."
            )
        else:
            ok(f"{lf.name}: active lock on {path!r} by {agent!r} (expires {exp.isoformat()})")

    return _summarize(strict)


# ─────────────────────────────────────────────────────────────────────────
# regen-tree  (writes a current, single-file snapshot per project instead
# of accumulating hand-run, date-stamped *_tree.txt files)
# ─────────────────────────────────────────────────────────────────────────

EXCLUDE_DIRS = {
    ".git", "target", "node_modules", ".venv", "__pycache__", ".obsidian",
    ".continue", ".cursor", ".sixth", "embeddings", "qdrant",
}


def _walk_tree(base: Path, max_depth: int = 4) -> list[str]:
    lines: list[str] = []

    def _walk(d: Path, depth: int, prefix: str):
        if depth > max_depth:
            return
        try:
            entries = sorted(d.iterdir(), key=lambda p: (p.is_file(), p.name.lower()))
        except PermissionError:
            return
        for entry in entries:
            if entry.name in EXCLUDE_DIRS:
                continue
            lines.append(f"{prefix}{entry.name}{'/' if entry.is_dir() else ''}")
            if entry.is_dir():
                _walk(entry, depth + 1, prefix + "  ")

    lines.append(f"{base.name}/")
    _walk(base, 1, "  ")
    return lines


def cmd_regen_tree() -> int:
    manifest = load_manifest()
    out_dir = THIS_FILE.parent.parent / "generated"
    out_dir.mkdir(parents=True, exist_ok=True)

    targets = [("canon", REPO_ROOT)] + [
        (p["name"], REPO_ROOT / p["path"]) for p in manifest.get("projects", [])
    ]

    stamp = datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
    for name, path in targets:
        if not path.exists():
            warn(f"regen-tree: skipping {name}, path does not exist ({path})")
            continue
        lines = [f"# {name} — generated by devops/scripts/ecosystem_check.py regen-tree on {stamp}", ""]
        lines += _walk_tree(path)
        out_file = out_dir / f"{name}_tree.CURRENT.txt"
        out_file.write_text("\n".join(lines) + "\n", encoding="utf-8")
        ok(f"wrote {out_file.relative_to(REPO_ROOT)}")

    return 0


# ─────────────────────────────────────────────────────────────────────────
# main
# ─────────────────────────────────────────────────────────────────────────


def _summarize(strict: bool) -> int:
    print(f"\n--- summary: {len(ERRORS)} error(s), {len(WARNINGS)} warning(s) ---")
    if ERRORS:
        return 2
    if WARNINGS and strict:
        return 1
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument(
        "command",
        choices=["validate", "check-locks", "regen-tree", "all"],
        help="what to run",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="treat warnings as failures (exit 1). Off by default so a first "
        "run doesn't go red before you've had a chance to clean anything up.",
    )
    args = parser.parse_args()

    if args.command == "validate":
        return cmd_validate(args.strict)
    if args.command == "check-locks":
        return cmd_check_locks(args.strict)
    if args.command == "regen-tree":
        return cmd_regen_tree()
    if args.command == "all":
        code1 = cmd_validate(args.strict)
        code2 = cmd_check_locks(args.strict)
        return max(code1, code2)
    return 0


if __name__ == "__main__":
    sys.exit(main())
