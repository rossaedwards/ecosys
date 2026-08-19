#!/usr/bin/env python3
"""
md2tex.py — Markdown → LaTeX via system pandoc (content-conserving).

Usage (PowerShell / bash):
  python md2tex.py
  python md2tex.py --recursive --out-dir .\\tex
  python md2tex.py tslca_section-vi.md --inplace
  python md2tex.py --dry-run -v

Requires: pandoc on PATH (or a common install location on Windows).
Does not rewrite body text, strip OKF front matter, or shift heading levels.
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


def find_pandoc() -> Path | None:
    which = shutil.which("pandoc")
    if which:
        return Path(which)

    # Common Windows install locations (best-effort fallback)
    candidates = [
        Path(os.environ.get("LOCALAPPDATA", "")) / "Pandoc" / "pandoc.exe",
        Path(os.environ.get("PROGRAMFILES", r"C:\Program Files")) / "Pandoc" / "pandoc.exe",
        Path(os.environ.get("PROGRAMFILES(X86)", r"C:\Program Files (x86)")) / "Pandoc" / "pandoc.exe",
        Path.home() / "AppData" / "Local" / "Pandoc" / "pandoc.exe",
    ]
    for path in candidates:
        if path.is_file():
            return path
    return None


def pandoc_version(pandoc: Path) -> str:
    try:
        out = subprocess.run(
            [str(pandoc), "--version"],
            check=True,
            capture_output=True,
            text=True,
        )
        return out.stdout.splitlines()[0].strip() if out.stdout else "unknown"
    except (OSError, subprocess.CalledProcessError) as exc:
        return f"unavailable ({exc})"


def collect_markdown(
    paths: list[Path],
    *,
    recursive: bool,
    cwd: Path,
) -> list[Path]:
    found: list[Path] = []
    if paths:
        for p in paths:
            path = p if p.is_absolute() else (cwd / p)
            path = path.resolve()
            if not path.exists():
                raise FileNotFoundError(f"not found: {path}")
            if path.is_dir():
                pattern = "**/*.md" if recursive else "*.md"
                found.extend(sorted(path.glob(pattern)))
            elif path.suffix.lower() == ".md":
                found.append(path)
            else:
                raise ValueError(f"not a markdown file: {path}")
    else:
        pattern = "**/*.md" if recursive else "*.md"
        found.extend(sorted(cwd.glob(pattern)))

    # De-dupe while preserving order; skip non-files
    seen: set[Path] = set()
    out: list[Path] = []
    for f in found:
        f = f.resolve()
        if not f.is_file() or f.suffix.lower() != ".md":
            continue
        if f in seen:
            continue
        seen.add(f)
        out.append(f)
    return out


def output_path(
    md: Path,
    *,
    cwd: Path,
    out_dir: Path | None,
    inplace: bool,
) -> Path:
    stem_tex = md.with_suffix(".tex").name
    if inplace:
        return md.with_suffix(".tex")
    if out_dir is not None:
        try:
            rel = md.resolve().relative_to(cwd.resolve())
            dest = out_dir / rel.with_suffix(".tex")
        except ValueError:
            dest = out_dir / stem_tex
        return dest
    return (cwd / "tex" / stem_tex).resolve()


def convert_one(
    pandoc: Path,
    md: Path,
    tex: Path,
    *,
    dry_run: bool,
    verbose: bool,
) -> None:
    tex.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        str(pandoc),
        str(md),
        "-f",
        "markdown",
        "-t",
        "latex",
        "--wrap=none",
        # Keep YAML/OKF front matter; do not shift headings or rewrite body.
        "-o",
        str(tex) if dry_run else "",  # placeholder; real write uses temp
    ]

    if dry_run:
        print(f"DRY  {md} → {tex}")
        if verbose:
            print("    ", " ".join(cmd[:-2] + ["-o", str(tex)]))
        return

    # Atomic write: pandoc → temp in same dir, then replace
    fd, tmp_name = tempfile.mkstemp(prefix=tex.stem + ".", suffix=".tex.tmp", dir=str(tex.parent))
    os.close(fd)
    tmp = Path(tmp_name)
    try:
        run_cmd = [
            str(pandoc),
            str(md),
            "-f",
            "markdown",
            "-t",
            "latex",
            "--wrap=none",
            "-o",
            str(tmp),
        ]
        if verbose:
            print("    ", " ".join(run_cmd))
        proc = subprocess.run(
            run_cmd,
            capture_output=True,
            text=True,
            check=False,
        )
        if proc.returncode != 0:
            err = (proc.stderr or proc.stdout or "pandoc failed").strip()
            raise RuntimeError(err)
        tmp.replace(tex)
        print(f"OK   {md} → {tex}")
    except Exception:
        if tmp.exists():
            tmp.unlink(missing_ok=True)
        raise


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Convert Markdown files to LaTeX via pandoc (content-conserving).",
    )
    parser.add_argument(
        "paths",
        nargs="*",
        type=Path,
        help="Optional .md files or directories (default: all *.md in cwd)",
    )
    parser.add_argument(
        "--recursive",
        "-r",
        action="store_true",
        help="Recurse into subdirectories when scanning",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=None,
        help="Write .tex under this directory (mirrors relative paths when possible)",
    )
    parser.add_argument(
        "--inplace",
        action="store_true",
        help="Write each .tex beside its .md",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print planned conversions without writing",
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
        help="Print pandoc version and full commands",
    )
    args = parser.parse_args(argv)

    if args.inplace and args.out_dir is not None:
        print("error: use only one of --inplace or --out-dir", file=sys.stderr)
        return 2

    pandoc = find_pandoc()
    if pandoc is None:
        print(
            "error: pandoc not found on PATH (and no common Windows install location matched)",
            file=sys.stderr,
        )
        return 1

    cwd = Path.cwd()
    if args.verbose:
        print(f"pandoc: {pandoc}")
        print(f"version: {pandoc_version(pandoc)}")
        print(f"cwd: {cwd}")

    try:
        md_files = collect_markdown(list(args.paths), recursive=args.recursive, cwd=cwd)
    except (FileNotFoundError, ValueError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    if not md_files:
        print("No markdown files to convert.")
        return 0

    failures = 0
    for md in md_files:
        tex = output_path(
            md,
            cwd=cwd,
            out_dir=args.out_dir.resolve() if args.out_dir else None,
            inplace=args.inplace,
        )
        try:
            convert_one(
                pandoc,
                md,
                tex,
                dry_run=args.dry_run,
                verbose=args.verbose,
            )
        except Exception as exc:
            failures += 1
            print(f"FAIL {md}: {exc}", file=sys.stderr)

    if failures:
        print(f"{failures} conversion(s) failed.", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
