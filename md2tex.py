#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        description="Convert .md files to .tex using Pandoc. Works on a single file or an entire folder."
    )
    p.add_argument("input_path", help="Markdown file or folder to process")
    p.add_argument(
        "-o",
        "--output-dir",
        help="Optional output root for .tex files. Folder structure is preserved when converting a directory.",
    )
    p.add_argument(
        "--pattern",
        default="*.md",
        help="Glob pattern used when input_path is a directory (default: *.md)",
    )
    p.add_argument(
        "--non-recursive",
        action="store_true",
        help="Only scan the top level of the folder",
    )
    p.add_argument(
        "--overwrite",
        action="store_true",
        help="Overwrite existing .tex files",
    )
    p.add_argument(
        "--pandoc",
        default="pandoc",
        help="Pandoc executable to use (default: pandoc)",
    )
    p.add_argument(
        "--standalone",
        action="store_true",
        help="Ask Pandoc to emit a standalone LaTeX document",
    )
    p.add_argument(
        "--pandoc-arg",
        action="append",
        default=[],
        help="Extra Pandoc argument; repeat as needed",
    )
    return p


def find_markdown_files(root: Path, pattern: str, recursive: bool) -> list[Path]:
    iterator = root.rglob(pattern) if recursive else root.glob(pattern)
    return sorted(p for p in iterator if p.is_file())


def destination_for(src: Path, input_root: Path, output_root: Path | None) -> Path:
    if output_root is None:
        return src.with_suffix(".tex")
    if input_root.is_file():
        return output_root / src.with_suffix(".tex").name
    relative = src.relative_to(input_root).with_suffix(".tex")
    return output_root / relative


def convert_one(
    src: Path,
    dst: Path,
    pandoc: str,
    standalone: bool,
    extra_args: list[str],
    overwrite: bool,
) -> tuple[bool, str]:
    if dst.exists() and not overwrite:
        return True, f"SKIP {src} -> {dst} (already exists)"

    dst.parent.mkdir(parents=True, exist_ok=True)

    cmd = [pandoc, str(src), "-f", "markdown", "-t", "latex", "-o", str(dst)]
    if standalone:
        cmd.append("--standalone")
    cmd.extend(extra_args)

    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode == 0:
        return True, f"OK   {src} -> {dst}"

    stderr = result.stderr.strip() or result.stdout.strip() or "Unknown Pandoc error"
    return False, f"FAIL {src} -> {dst}\n{stderr}"


def main() -> int:
    args = build_parser().parse_args()
    input_path = Path(args.input_path).expanduser().resolve()
    output_root = (
        Path(args.output_dir).expanduser().resolve() if args.output_dir else None
    )

    if shutil.which(args.pandoc) is None:
        print(
            f"Error: Pandoc executable '{args.pandoc}' was not found in PATH. Install Pandoc first.",
            file=sys.stderr,
        )
        return 2

    if not input_path.exists():
        print(f"Error: input path does not exist: {input_path}", file=sys.stderr)
        return 2

    if input_path.is_file():
        if input_path.suffix.lower() != ".md":
            print("Error: input file must end in .md", file=sys.stderr)
            return 2
        sources = [input_path]
        input_root = input_path
    else:
        sources = find_markdown_files(input_path, args.pattern, not args.non_recursive)
        input_root = input_path

    if not sources:
        print("No Markdown files found.")
        return 0

    ok_count = 0
    fail_count = 0

    for src in sources:
        dst = destination_for(src, input_root, output_root)
        ok, message = convert_one(
            src=src,
            dst=dst,
            pandoc=args.pandoc,
            standalone=args.standalone,
            extra_args=args.pandoc_arg,
            overwrite=args.overwrite,
        )
        print(message)
        if ok:
            ok_count += 1
        else:
            fail_count += 1

    print(f"\nDone. success={ok_count} failed={fail_count} total={len(sources)}")
    return 1 if fail_count else 0


if __name__ == "__main__":
    raise SystemExit(main())
