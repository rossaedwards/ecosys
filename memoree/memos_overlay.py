"""
Memoree — MemOS FastAPI bridge (optional)
"""

from __future__ import annotations
import os
import httpx
import yaml

# Fixed for flat structure
_CONFIG_PATH = "config.yaml"


def _cfg() -> dict:
    try:
        with open(_CONFIG_PATH, encoding="utf-8") as f:
            return yaml.safe_load(f) or {}
    except Exception:
        return {}


def _enabled() -> bool:
    return _cfg().get("memos", {}).get("enabled", False)


def _url() -> str:
    return _cfg().get("memos", {}).get("api_url", "http://localhost:9090")


def _cube_id() -> str:
    return _cfg().get("memos", {}).get("cube_id", "rossaedwards_main")


def mirror_event(event: dict):
    """Mirror a Memoree event into MemOS cube."""
    if not _enabled():
        return
    try:
        httpx.post(
            f"{_url()}/memory/add",
            json={
                "cube_id": _cube_id(),
                "content": event.get("content", ""),
                "meta": {
                    "llm": event.get("llm"),
                    "type": event.get("type"),
                    "tags": event.get("tags", []),
                    "timestamp": event.get("timestamp"),
                },
            },
            timeout=3,
        )
    except Exception:
        pass


def recall_from_memos(query: str, top_k: int = 5) -> list[dict]:
    """Pull enriched context from MemOS."""
    if not _enabled():
        return []
    try:
        resp = httpx.post(
            f"{_url()}/memory/search",
            json={"cube_id": _cube_id(), "query": query, "top_k": top_k},
            timeout=5,
        )
        resp.raise_for_status()
        return resp.json().get("results", [])
    except Exception:
        return []