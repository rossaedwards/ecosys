"""Memoree v3 — SuperGrok hook (stub).

Phase 1: Manual POST — paste Grok response text and save.
Phase 2 (future): WebSocket subscribe to SuperGrok session.

Usage (Phase 1):
    from memoree.hooks.supergrok_hook import save_manual
    save_manual("Grok said: ...", tags=["ftqc", "grok"])
"""
from __future__ import annotations
import httpx

_MEMOREE_URL = "http://127.0.0.1:8765"


def save_manual(content: str, tags: list[str] = [], meta: dict = {}):
    """Manually save a SuperGrok response to Memoree."""
    httpx.post(
        f"{_MEMOREE_URL}/write_event",
        json={"llm": "supergrok", "type": "episodic", "content": content, "tags": tags, "meta": meta},
        timeout=5,
    )


# Phase 2 stub — WS subscribe
async def subscribe_ws(ws_url: str):
    """
    TODO: Connect to SuperGrok WebSocket session stream.
    On each message event, call write_event POST.
    """
    raise NotImplementedError("SuperGrok WS bridge not yet implemented — use save_manual() for now.")
