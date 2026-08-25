#!/usr/bin/env python3
"""
tunnel_guard.py — keeps the Memoree daemon + its Cloudflare tunnel alive.

What it checks, every interval:
  1. Is memoree_service.py's process running? (Windows: tasklist)
  2. Does http://127.0.0.1:7042/health respond?
  3. Does https://memoree.aurphyx.org/health respond? (the public tunnel hop)

What it does about it:
  - If the local service is down, it starts it.
  - If the local service is up but the public tunnel is down, it logs and
    alerts (it does NOT try to restart cloudflared automatically by default
    — killing/restarting someone else's tunnel process from a background
    script is the kind of thing that should be opt-in, see --restart-tunnel).
  - Everything is logged to devops/logs/tunnel_guard.log so you have a
    record of outages instead of just a vague "it felt slow yesterday".

This is pure standard library on purpose — no pip install required to run
it. It's written to run on Windows (where Memoree + cloudflared actually
live) but the health checks work anywhere.

Usage:
    python devops/scripts/tunnel_guard.py                 # one check, then exit
    python devops/scripts/tunnel_guard.py --loop           # check every 60s forever
    python devops/scripts/tunnel_guard.py --loop --interval 120
    python devops/scripts/tunnel_guard.py --restart-tunnel  # also allowed to
                                                              # restart cloudflared.exe

Recommended: don't run this by hand forever. Instead, once it's doing what
you want, register it with Windows Task Scheduler so it runs at logon and
restarts itself if it crashes:

    schtasks /create /tn "Memoree Tunnel Guard" ^
      /tr "python C:\\rossaedwards\\ecosys\\devops\\scripts\\tunnel_guard.py --loop" ^
      /sc onlogon /rl highest

(Run that once from an elevated PowerShell / cmd prompt. See
devops/cloudflare/README.md for the full walkthrough.)
"""

from __future__ import annotations

import argparse
import datetime
import platform
import subprocess
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

LOCAL_HEALTH_URL = "http://127.0.0.1:7042/health"
PUBLIC_HEALTH_URL = "https://memoree.aurphyx.org/health"

THIS_FILE = Path(__file__).resolve()
DEVOPS_DIR = THIS_FILE.parent.parent
LOG_DIR = DEVOPS_DIR / "logs"
LOG_FILE = LOG_DIR / "tunnel_guard.log"

# Best-guess location of memoree_service.py relative to this repo layout.
# Override with --memoree-path if your checkout differs.
DEFAULT_MEMOREE_SCRIPT = DEVOPS_DIR.parent / "memoree" / "memoree_service.py"


def log(msg: str) -> None:
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    stamp = datetime.datetime.now().isoformat(timespec="seconds")
    line = f"[{stamp}] {msg}"
    print(line)
    with open(LOG_FILE, "a", encoding="utf-8") as f:
        f.write(line + "\n")


def check_url(url: str, timeout: float = 5.0) -> tuple[bool, str]:
    try:
        with urllib.request.urlopen(url, timeout=timeout) as resp:
            return resp.status < 400, f"HTTP {resp.status}"
    except urllib.error.HTTPError as e:
        return False, f"HTTP {e.code}"
    except Exception as e:
        return False, f"{type(e).__name__}: {e}"


def is_windows() -> bool:
    return platform.system() == "Windows"


def process_running(name_fragment: str) -> bool:
    """Best-effort check via tasklist (Windows) or ps (elsewhere)."""
    try:
        if is_windows():
            out = subprocess.run(
                ["tasklist", "/FI", f"IMAGENAME eq {name_fragment}"],
                capture_output=True, text=True, timeout=10,
            ).stdout
            return name_fragment.lower() in out.lower()
        else:
            out = subprocess.run(["ps", "aux"], capture_output=True, text=True, timeout=10).stdout
            return name_fragment.lower() in out.lower()
    except Exception as e:
        log(f"WARN: could not check process list ({e})")
        return False  # unknown; treat as "not confirmed running"


def start_memoree(memoree_script: Path) -> None:
    if not memoree_script.exists():
        log(f"ERROR: cannot start Memoree, script not found at {memoree_script}")
        return
    log(f"Starting Memoree: python {memoree_script}")
    try:
        if is_windows():
            subprocess.Popen(
                ["python", str(memoree_script)],
                cwd=str(memoree_script.parent),
                creationflags=subprocess.CREATE_NEW_CONSOLE,
            )
        else:
            subprocess.Popen(
                ["python3", str(memoree_script)],
                cwd=str(memoree_script.parent),
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
    except Exception as e:
        log(f"ERROR: failed to start Memoree: {e}")


def restart_cloudflared() -> None:
    if not is_windows():
        log("WARN: --restart-tunnel only implemented for Windows right now.")
        return
    log("Restarting cloudflared.exe (taskkill + you must have it configured to relaunch, "
        "e.g. via `cloudflared service install` or your own launcher).")
    subprocess.run(["taskkill", "/IM", "cloudflared.exe", "/F"], capture_output=True)
    # We deliberately do NOT try to guess how you launch cloudflared (service
    # vs. manual `cloudflared tunnel run <name>`), since restarting it wrong
    # can leave you with zero tunnel instead of a flaky one. If you run it as
    # a Windows service, `cloudflared service install` makes Windows restart
    # it for you and this function only needs the taskkill above.


def run_once(args) -> bool:
    """Returns True if everything is healthy."""
    healthy = True

    local_ok, local_detail = check_url(LOCAL_HEALTH_URL)
    if local_ok:
        log(f"OK: local Memoree ({LOCAL_HEALTH_URL}) -> {local_detail}")
    else:
        healthy = False
        log(f"DOWN: local Memoree ({LOCAL_HEALTH_URL}) -> {local_detail}")
        running = process_running("python.exe") if is_windows() else process_running("memoree_service.py")
        if not running:
            log("Memoree process not detected — attempting to start it.")
            start_memoree(Path(args.memoree_path))
        else:
            log("Memoree process appears to be running but /health isn't responding yet — "
                "giving it time rather than double-launching.")

    public_ok, public_detail = check_url(PUBLIC_HEALTH_URL)
    if public_ok:
        log(f"OK: public tunnel ({PUBLIC_HEALTH_URL}) -> {public_detail}")
    else:
        healthy = False
        log(f"DOWN: public tunnel ({PUBLIC_HEALTH_URL}) -> {public_detail}")
        if local_ok:
            log("Local service is healthy but the tunnel isn't — this points at "
                "cloudflared, not Memoree.")
            if args.restart_tunnel:
                restart_cloudflared()
            else:
                log("Not restarting cloudflared (pass --restart-tunnel to allow it).")

    return healthy


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--loop", action="store_true", help="run forever, checking on an interval")
    parser.add_argument("--interval", type=int, default=60, help="seconds between checks in --loop mode (default 60)")
    parser.add_argument("--restart-tunnel", action="store_true", help="allow restarting cloudflared.exe when the public tunnel is down")
    parser.add_argument("--memoree-path", default=str(DEFAULT_MEMOREE_SCRIPT), help="path to memoree_service.py")
    args = parser.parse_args()

    if not args.loop:
        return 0 if run_once(args) else 1

    log(f"tunnel_guard starting loop, interval={args.interval}s, restart_tunnel={args.restart_tunnel}")
    try:
        while True:
            run_once(args)
            time.sleep(args.interval)
    except KeyboardInterrupt:
        log("tunnel_guard stopped (Ctrl+C).")
        return 0


if __name__ == "__main__":
    sys.exit(main())
