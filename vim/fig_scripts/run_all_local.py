#!/usr/bin/env python3
"""
Run all VIM figure scripts locally.

Usage:
  python run_all_local.py              # run all, output to fig_scripts/
  python run_all_local.py -o ./out    # output to ./out
  python run_all_local.py -n 5        # run first 5 only
  python run_all_local.py --list      # list scripts only
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

# Ensure fig_scripts is on path so vim_common can be imported
SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))


def get_scripts() -> list[Path]:
    """Return fig_*.py scripts, excluding run_all_local and vim_common."""
    scripts = sorted(SCRIPT_DIR.glob("fig_*.py"))
    return [s for s in scripts if s.name not in ("run_all_local.py", "vim_common.py")]


def run_one(script: Path, output_dir: Path) -> tuple[Path, bool, str | None]:
    """Run a single figure script. Returns (script, success, error_msg)."""
    try:
        # Import and call run_simulation
        import importlib.util
        spec = importlib.util.spec_from_file_location(script.stem, script)
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        out = mod.run_simulation(output_dir)
        return (script, True, None)
    except Exception as e:
        return (script, False, str(e))


def main() -> None:
    ap = argparse.ArgumentParser(description="Run all VIM figure scripts locally")
    ap.add_argument("-o", "--output", type=Path, default=SCRIPT_DIR,
                    help="Output directory for figures (default: fig_scripts/)")
    ap.add_argument("-n", "--max", type=int, default=None,
                    help="Max number of scripts to run (for testing)")
    ap.add_argument("--list", action="store_true", help="List scripts and exit")
    args = ap.parse_args()

    scripts = get_scripts()
    if args.list:
        for s in scripts:
            print(s.name)
        print(f"\nTotal: {len(scripts)} scripts")
        return

    args.output.mkdir(parents=True, exist_ok=True)
    print(f"Running {len(scripts)} figure scripts -> {args.output}")
    print("-" * 60)

    ok, fail = 0, 0
    for i, script in enumerate(scripts):
        if args.max and i >= args.max:
            break
        _, success, err = run_one(script, args.output)
        if success:
            ok += 1
            print(f"  OK   {script.name}")
        else:
            fail += 1
            print(f"  FAIL {script.name}: {err}")

    print("-" * 60)
    print(f"Done: {ok} OK, {fail} failed")


if __name__ == "__main__":
    main()
