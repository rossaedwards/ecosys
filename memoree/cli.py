## ** APS-TSLCA-MEMOREE-CLI **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Sovereign CLI Tool Suite
═══════════════════════════════════════════════════════════════════════════════
Command-line interface for Memoree management, auth, lattice inspection,
model test pings, and Rituals/Chains/Links (RCL) orchestration.

Usage:
  python cli.py auth status
  python cli.py auth <provider> [api_key]
  python cli.py auth test [provider]
  python cli.py status
  python cli.py lattice
  python cli.py recall <query> [--project <proj>] [--top-k 5]
  python cli.py cure
  python cli.py rcl list
  python cli.py dashboard
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import argparse
import asyncio
import json
import sys
import webbrowser
from pathlib import Path
from typing import Optional

import httpx

# Ensure UTF-8 stdout encoding for Windows terminals
if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")

# Ensure local imports work
sys.path.insert(0, str(Path(__file__).parent))


from credentials_manager import credentials
from hooks_registry import chat_with_memory_async, get_hook, list_available_hooks
from memory_engine import MemoryEngine
from rcl_engine import rcl_engine


def print_banner():
    print("=" * 68)
    print("  MEMOREE v4.0 - Sovereign Memory Substrate & Multi-Model Console")
    print("  Aurphyx LLC | SAGES Pro-Existence | TSLCA 3x3 Lattice")
    print("=" * 68)


# ── Auth Commands ─────────────────────────────────────────────────────────────


def cmd_auth_status(args):
    print_banner()
    print("\n[+] Configured Model & Platform Hooks:")
    print("-" * 68)
    print(f"{'PROVIDER':<14} | {'STATUS':<12} | {'DEFAULT MODEL':<24} | {'KEY'}")
    print("-" * 68)
    status_map = credentials.list_status()
    for p, info in sorted(status_map.items()):
        stat = "[OK] Ready" if info["configured"] else "[X] Missing"
        print(f"{p:<14} | {stat:<12} | {info['default_model']:<24} | {info['masked_key']}")
    print("-" * 68)
    print("Tip: Run 'python cli.py auth <provider> <key>' to configure a key.")
    print("Tip: Run 'python cli.py auth test <provider>' to test live API connection.\n")


def cmd_auth_set(args):
    provider = args.provider.lower()
    key = args.key
    if not key:
        try:
            key = input(f"Enter API key for {provider}: ").strip()
        except (KeyboardInterrupt, EOFError):
            print("\nAborted.")
            return

    if not key:
        print(f"Error: API key cannot be empty.")
        return

    credentials.set_key(provider, key)
    print(f"\n[OK] Successfully configured API key for '{provider}'.")
    print(f"     Saved securely to ~/.memoree/credentials.json")


def cmd_auth_test(args):
    provider = args.provider.lower() if args.provider else None
    targets = [provider] if provider else ["gemini", "claude", "supergrok", "perplexity", "hermes", "lmstudio", "ollama"]

    print_banner()
    print("\n[+] Dispatching live authentication test probes...")
    print("-" * 68)

    async def _test():
        for p in targets:
            try:
                hook = get_hook(p)
                if not hook.is_configured():
                    print(f"[-] {p:<12} : [SKIPPED] Key not configured")
                    continue

                print(f"[*] Testing {p:<10} ...", end="", flush=True)
                res = await hook.generate_async(
                    prompt="Ping test: return the exact phrase 'AURA_MEMOREE_OK'.",
                    inject_memory=False,
                )
                txt = res.get("text", "").strip()
                lat = res.get("latency_ms", 0.0)
                print(f" [OK] {lat:.1f}ms | Reply: '{txt[:30]}...'")
            except Exception as e:
                print(f" [FAILED] {e}")

    asyncio.run(_test())
    print("-" * 68)


# ── Status Command ────────────────────────────────────────────────────────────


def cmd_status(args):
    print_banner()
    engine = MemoryEngine()
    diag = engine.diagnostics()
    print(f"\n[*] Daemon Version       : {diag.version}")
    print(f"[*] Subsystem Status     : {diag.status.upper()}")
    print(f"[*] Vector DB (Qdrant)   : {'Connected (localhost:6333)' if diag.qdrant_connected else 'In-Memory Fallback'}")
    print(f"[*] 9 TSL Collections    : {', '.join(diag.qdrant_collections or ['(fallback initialized)'])}")
    print(f"[*] Cloudflare Tunnel    : https://memoree.aurphyx.com (5b13dbbe-9a8d-4d0e-b4d3-08ba18fda966)")
    print(f"[*] Active Projects      : {len(engine.projects)} loaded")
    print(f"[*] Registered Dualities : {len(engine.global_dualities)}")
    print(f"[*] Global Invariants    : {len(engine.global_invariants)}")
    print("-" * 68)


# ── Lattice Inspection Command ────────────────────────────────────────────────


def cmd_lattice(args):
    print_banner()
    engine = MemoryEngine()
    snap = engine.get_lattice_snapshot()

    print(f"\n[+] Three-Squared-Lattice Cognitive Architecture (3x3 Field Tensor):")
    print(f"    Timestamp: {snap['timestamp']} | Unified Field Trace: {snap['unified_field_trace']}")
    print("-" * 68)
    print(f"{'CELL':<12} | {'CORES':<10} | {'MEMORY TYPE':<14} | {'COUNT':<8} | {'MEAN HIF'}")
    print("-" * 68)
    for cell_name, data in snap["cells"].items():
        print(
            f"{cell_name:<12} | {data['core_i']}x{data['core_j']:<6} | {data['memory_type']:<14} | {data['count']:<8} | {data['mean_hif']:.2f}"
        )
    print("-" * 68)


# ── Recall Search Command ─────────────────────────────────────────────────────


def cmd_recall(args):
    engine = MemoryEngine()
    from schemas import MemoryQuery, MemoryType

    q = args.query
    proj = args.project
    top_k = args.top_k or 5

    print(f"\n[+] Recalling memories matching: '{q}' (Project: {proj or 'ALL'})...")
    req = MemoryQuery(
        query_text=q,
        project=proj,
        top_k=top_k,
        memory_types=[
            MemoryType.EPISODIC,
            MemoryType.SEMANTIC,
            MemoryType.META,
            MemoryType.PROCEDURAL,
            MemoryType.QUANTUM,
            MemoryType.IDENTITY,
            MemoryType.SENSORY,
            MemoryType.WORKING,
            MemoryType.GOVERNANCE,
        ],
    )
    results = engine.query(req)
    print(f"Found {len(results)} memory records:")
    print("-" * 68)
    for i, r in enumerate(results, 1):
        print(f"[{i}] ({r.memory_type.value.upper()}) [Score: {r.score:.3f}] - {r.project}")
        print(f"    {r.content_preview[:120]}...\n")


# ── Working Memory Curing Command ─────────────────────────────────────────────


def cmd_cure(args):
    engine = MemoryEngine()
    stats = engine.cure_working_buffer()
    print(f"\n[OK] Cured {stats.get('cured_count', 0)} working memories into permanent layers.")
    print(f"     Remaining in-session buffer: {stats.get('remaining_buffer', 0)}")


# ── RCL Command ───────────────────────────────────────────────────────────────


def cmd_rcl(args):
    print_banner()
    manifest = rcl_engine.get_manifest()
    print("\n[+] Registered Links (Level 1):")
    for l in manifest["links"]:
        print(f"  * [{l['id']}] {l['name']} ({l['source_agent']} -> {l['target_agent']})")

    print("\n[+] Registered Chains (Level 2):")
    for c in manifest["chains"]:
        print(f"  * [{c['id']}] {c['name']} ({len(c['steps'])} steps)")

    print("\n[+] Registered Rituals (Level 3):")
    for r in manifest["rituals"]:
        print(f"  * [{r['id']}] {r['name']} (Interval: {r['interval_seconds']}s, Action: {r['action_slug']})")
    print("-" * 68)


# ── Dashboard Command ─────────────────────────────────────────────────────────


def cmd_dashboard(args):
    url = "http://127.0.0.1:7042/dashboard"
    print(f"\n[+] Launching Memoree Hub Dashboard at: {url}")
    print(f"    Qdrant Dashboard available at   : http://localhost:6333/dashboard")
    print(f"    Cloudflare Edge Endpoint at     : https://memoree.aurphyx.com/dashboard")
    webbrowser.open(url)


# ── Main Dispatcher ───────────────────────────────────────────────────────────


def main():
    parser = argparse.ArgumentParser(
        prog="memoree",
        description="Memoree v4.0 Sovereign Memory Substrate & Multi-Model Console CLI",
    )
    subparsers = parser.add_subparsers(dest="command", help="Available subcommands")

    # auth
    auth_parser = subparsers.add_parser("auth", help="Manage model & platform API keys")
    auth_sub = auth_parser.add_subparsers(dest="auth_action")
    auth_sub.add_parser("status", help="Show credentials status table")
    test_sub = auth_sub.add_parser("test", help="Test live API connections")
    test_sub.add_argument("provider", nargs="?", help="Specific provider to test")
    set_sub = auth_sub.add_parser("set", help="Set credentials")
    set_sub.add_argument("provider", help="Provider name (gemini, claude, grok, etc.)")
    set_sub.add_argument("key", nargs="?", help="API key string")

    # status
    subparsers.add_parser("status", help="Show daemon & backend diagnostic status")

    # lattice
    subparsers.add_parser("lattice", help="Inspect 3x3 Cognitive Field Tensor")

    # recall
    rec_parser = subparsers.add_parser("recall", help="Semantic query across 9 memory types")
    rec_parser.add_argument("query", help="Query string")
    rec_parser.add_argument("--project", "-p", help="Filter by project key")
    rec_parser.add_argument("--top-k", "-k", type=int, default=5, help="Max results")

    # cure
    subparsers.add_parser("cure", help="Manually cure working memory buffer")

    # rcl
    rcl_parser = subparsers.add_parser("rcl", help="Inspect Rituals, Chains, Links, & Forkz")
    rcl_parser.add_argument("action", nargs="?", default="list", help="Action (list, run)")
    rcl_parser.add_argument("target", nargs="?", help="Target ID to run")


    # dashboard
    subparsers.add_parser("dashboard", help="Open Memoree Web Dashboard in browser")

    args = parser.parse_args()

    if args.command == "auth":
        if args.auth_action == "status" or not args.auth_action:
            cmd_auth_status(args)
        elif args.auth_action == "test":
            cmd_auth_test(args)
        elif args.auth_action == "set":
            cmd_auth_set(args)
        else:
            # Shortcut: memoree auth gemini <key>
            args.provider = args.auth_action
            args.key = sys.argv[3] if len(sys.argv) > 3 else None
            cmd_auth_set(args)
    elif args.command == "status":
        cmd_status(args)
    elif args.command == "lattice":
        cmd_lattice(args)
    elif args.command == "recall":
        cmd_recall(args)
    elif args.command == "cure":
        cmd_cure(args)
    elif args.command == "rcl":
        cmd_rcl(args)
    elif args.command == "dashboard":
        cmd_dashboard(args)
    else:
        cmd_auth_status(args)


if __name__ == "__main__":
    main()
